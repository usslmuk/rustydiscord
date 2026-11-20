use core::{
    fmt,
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
use std::sync::Arc;

use crate::model::{guild::WidgetStyle, id::GuildId, rest::FileUpload};
use bytes::Bytes;
use http::{
    HeaderName, HeaderValue, Method, StatusCode, Uri,
    header::{AUTHORIZATION, CONTENT_TYPE, USER_AGENT},
};
use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::Notify;

use crate::rest::{
    cdn::{self, Cdn},
    error::{ApiError, BoxError, Error, ErrorKind, RateLimitedBody},
    metrics::RestMetrics,
    multipart::{Form, PreparedBody},
    query::{Query, QueryWriter},
    ratelimit::{BucketState, InvalidRequests, RateLimitHeaders, RateLimiter},
    route::{self, BucketRoute, Route},
    transport::{Body, Instant, Transport},
};

const AUDIT_LOG_REASON: HeaderName = HeaderName::from_static("x-audit-log-reason");

pub const DEFAULT_API: &str = "https://discord.com/api/v10";

pub(crate) fn boxed<'a, T>(
    future: impl Future<Output = T> + Send + 'a,
) -> Pin<Box<dyn Future<Output = T> + Send + 'a>> {
    Box::pin(future)
}

#[derive(Clone, Debug, Default)]
pub struct AbortHandle {
    state: Arc<AbortState>,
}

#[derive(Debug, Default)]
struct AbortState {
    tripped: AtomicBool,
    notify: Notify,
}

impl AbortHandle {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn abort(&self) {
        self.state.tripped.store(true, Ordering::SeqCst);
        self.state.notify.notify_one();
    }

    #[must_use]
    pub fn is_aborted(&self) -> bool {
        self.state.tripped.load(Ordering::SeqCst)
    }

    pub(crate) async fn cancelled(&self) {
        while !self.is_aborted() {
            self.state.notify.notified().await;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RetryPolicy {
    pub max_retries: u8,

    pub base: Duration,

    pub cap: Duration,

    pub jitter: bool,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base: Duration::from_millis(500),
            cap: Duration::from_secs(30),
            jitter: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RateLimited {
    pub route: &'static str,

    pub global: bool,

    pub scope: Option<String>,

    pub retry_after: Duration,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RateLimitAction {
    Wait,
    Abort,
}

fn decode<T: DeserializeOwned>(body: &[u8]) -> Result<T, Error> {
    let body: &[u8] = if body.is_empty() { b"null" } else { body };
    serde_json::from_slice(body)
        .map_err(|source| Error::with_source(ErrorKind::Deserialize, source))
}

#[derive(Clone)]
pub struct Rest {
    inner: Arc<Inner>,
}

pub(crate) type RateLimitHook = Arc<dyn Fn(&RateLimited) -> RateLimitAction + Send + Sync>;

struct Pending {
    bucket: BucketRoute,
    uri: Uri,
    reason: Option<HeaderValue>,
    body: PreparedBody,
    method: Method,
    authenticated: bool,
    abort: Option<AbortHandle>,
}

pub(crate) async fn cancellable<T>(
    abort: Option<&AbortHandle>,
    task: impl Future<Output = T>,
) -> Option<T> {
    match abort {
        None => Some(task.await),
        Some(handle) => tokio::select! {
            biased;
            () = handle.cancelled() => None,
            output = task => Some(output),
        },
    }
}

fn raw_authenticated(path: &str) -> bool {
    let mut segments = path.split('/').filter(|segment| !segment.is_empty());
    !matches!(
        (segments.next(), segments.next(), segments.next()),
        (Some("webhooks"), Some(_), Some(_))
    )
}

struct Inner {
    transport: Arc<dyn Transport>,
    token: Token,

    base: String,
    user_agent: HeaderValue,
    timeout: Duration,
    retry: RetryPolicy,
    ratelimiter: RateLimiter,
    invalid: InvalidRequests,
    metrics: RestMetrics,
    on_rate_limit: Option<RateLimitHook>,

    invalid_request_limit: u32,
    cdn: Cdn,
}

enum Token {
    None,
    Valid(HeaderValue),
    Invalid,
}

impl fmt::Debug for Rest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rest")
            .field("base", &self.inner.base)
            .field(
                "token",
                &match self.inner.token {
                    Token::None => "<none>",
                    Token::Valid(_) => "<redacted>",
                    Token::Invalid => "<invalid>",
                },
            )
            .field("timeout", &self.inner.timeout)
            .field("retry", &self.inner.retry)
            .finish_non_exhaustive()
    }
}

impl Rest {
    #[cfg(feature = "hyper")]
    #[cfg_attr(docsrs, doc(cfg(feature = "hyper")))]
    #[must_use]
    pub fn new(token: impl Into<String>) -> Self {
        Self::builder().token(token).build()
    }

    #[must_use]
    pub fn cdn(&self) -> &Cdn {
        &self.inner.cdn
    }

    #[must_use]
    pub fn guild_widget_image_url(&self, guild_id: GuildId, style: Option<WidgetStyle>) -> String {
        let mut url = format!("{}/guilds/{guild_id}/widget.png", self.inner.base);
        if let Some(style) = style {
            url.push_str("?style=");
            route::write_text(&mut url, style.as_str());
        }
        url
    }

    pub fn builder() -> RestBuilder {
        RestBuilder::default()
    }

    #[must_use]
    pub fn invalid_requests(&self) -> u32 {
        self.inner.invalid.count(Instant::now())
    }

    #[must_use]
    pub fn metrics(&self) -> &RestMetrics {
        &self.inner.metrics
    }

    pub fn request<'a, T: DeserializeOwned + 'a>(
        &'a self,
        request: Request<'a>,
    ) -> impl Future<Output = Result<T, Error>> + Send + 'a {
        let response = boxed(self.execute(request));
        async move {
            let (_, body) = response.await?;
            decode(&body)
        }
    }

    pub(crate) async fn call<T: DeserializeOwned>(&self, request: Request<'_>) -> Result<T, Error> {
        let (_, body) = self.execute(request).await?;
        decode(&body)
    }

    pub(crate) async fn execute(&self, request: Request<'_>) -> Result<(StatusCode, Bytes), Error> {
        let inner = &*self.inner;
        let route = request.route;
        let abort = request.abort.clone();
        let pending = Pending {
            bucket: route.bucket(),
            uri: inner.uri(route, request.query)?,
            reason: request.reason.map(reason_header),
            body: request.prepare_body(),
            method: route.method(),
            authenticated: route.authenticated(),
            abort,
        };
        self.execute_pending(pending).await
    }

    pub async fn request_raw(
        &self,
        method: Method,
        path: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value, Error> {
        if !path.starts_with('/') {
            return Err(Error::with_source(
                ErrorKind::BuildRequest,
                "a raw path starts with `/`, like `/applications/@me`",
            ));
        }
        let inner = &*self.inner;
        let body = body
            .map(|body| serde_json::to_vec(&body))
            .transpose()
            .map_err(|source| Error::with_source(ErrorKind::BuildRequest, source))?
            .map(Bytes::from);
        let mut uri = String::with_capacity(inner.base.len() + path.len());
        uri.push_str(&inner.base);
        uri.push_str(path);
        let uri = Uri::try_from(uri)
            .map_err(|source| Error::with_source(ErrorKind::BuildRequest, source))?;
        let form = Form::default();
        let pending = Pending {
            bucket: BucketRoute::raw(&method),
            uri,
            reason: None,
            body: PreparedBody::new(body, &form),
            method,
            authenticated: raw_authenticated(path),
            abort: None,
        };
        let (_, body) = self.execute_pending(pending).await?;
        decode(&body)
    }

    async fn execute_pending(&self, pending: Pending) -> Result<(StatusCode, Bytes), Error> {
        let inner = &*self.inner;
        let Pending {
            bucket: bucket_route,
            uri,
            reason,
            body,
            method,
            authenticated,
            abort,
        } = pending;
        let limits = bucket_route.limits;
        let template = bucket_route.template;

        let mut attempt: u8 = 0;
        loop {
            inner.check_invalid_requests()?;
            if abort.as_ref().is_some_and(AbortHandle::is_aborted) {
                return Err(Error::new(ErrorKind::Aborted));
            }
            let bucket = inner.ratelimiter.bucket(bucket_route);

            let mut state = if limits.burst {
                None
            } else {
                let mut state = bucket.state.lock().await;
                if state
                    .wait(
                        inner.ratelimiter.offset(),
                        &*inner.transport,
                        abort.as_ref(),
                    )
                    .await
                {
                    return Err(Error::new(ErrorKind::Aborted));
                }
                Some(state)
            };
            if limits.global
                && inner
                    .ratelimiter
                    .acquire_global(&*inner.transport, abort.as_ref())
                    .await
            {
                return Err(Error::new(ErrorKind::Aborted));
            }

            let http_request =
                inner.build_request(method.clone(), authenticated, &uri, reason.as_ref(), &body)?;
            let started = Instant::now();
            inner.metrics.record_request();
            let send = timeout(&*inner.transport, inner.timeout, http_request);
            let response = match cancellable(abort.as_ref(), send).await {
                None => {
                    drop(state);
                    return Err(Error::new(ErrorKind::Aborted));
                }
                Some(Ok(Ok(response))) => response,
                Some(failed) => {
                    drop(state);
                    let error = match failed {
                        Ok(Err(source)) => Error::with_source(ErrorKind::Transport, source),
                        _ => Error::new(ErrorKind::Timeout),
                    };
                    if attempt < inner.retry.max_retries {
                        attempt += 1;
                        inner.metrics.record_retry();
                        tracing::warn!(route = template, attempt, %error, "request failed, retrying");
                        let backoff = backoff_delay(attempt, &inner.retry);
                        if cancellable(abort.as_ref(), inner.transport.sleep(backoff))
                            .await
                            .is_none()
                        {
                            return Err(Error::new(ErrorKind::Aborted));
                        }
                        continue;
                    }
                    return Err(error);
                }
            };

            let now = Instant::now();
            let (parts, body) = response.into_parts();
            match self
                .settle_response(
                    template,
                    bucket_route,
                    &mut state,
                    &mut attempt,
                    started,
                    now,
                    parts.status,
                    &RateLimitHeaders::parse(&parts.headers),
                    body,
                    abort.as_ref(),
                )
                .await
            {
                Settled::Done(outcome) => return outcome,
                Settled::Retry => {}
            }
        }
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "one call site; groups one retry step's inputs"
    )]
    async fn settle_response(
        &self,
        template: &'static str,
        bucket_route: BucketRoute,
        state: &mut Option<tokio::sync::MutexGuard<'_, BucketState>>,
        attempt: &mut u8,
        started: Instant,
        now: Instant,
        status: StatusCode,
        headers: &RateLimitHeaders<'_>,
        body: Bytes,
        abort: Option<&AbortHandle>,
    ) -> Settled {
        let inner = &*self.inner;
        if let Some(hash) = headers.bucket {
            inner.ratelimiter.learn_hash(bucket_route.template, hash);
        }
        if let Some(state) = state {
            state.update(headers, now);
        }
        if is_invalid(status, headers) {
            inner.invalid.record(now);
        }
        inner.metrics.record_status(status.as_u16());
        tracing::debug!(
            route = template,
            status = status.as_u16(),
            elapsed_ms = millis(started.elapsed()),
            remaining = headers.remaining,
            "REST request completed"
        );

        match status {
            status if status.is_success() => Settled::Done(Ok((status, body))),
            StatusCode::TOO_MANY_REQUESTS => {
                let (retry_after, global) = rate_limited(&body, headers);
                tracing::warn!(
                    route = template,
                    global,
                    scope = headers.scope,
                    retry_after_ms = millis(retry_after),
                    "rate limited by Discord, retrying"
                );
                if global {
                    inner.ratelimiter.pause_global(retry_after);
                }
                let waiting = inner
                    .on_rate_limit
                    .as_ref()
                    .map_or(RateLimitAction::Wait, |hook| {
                        hook(&RateLimited {
                            route: template,
                            global,
                            scope: headers.scope.map(str::to_owned),
                            retry_after,
                        })
                    });
                if waiting == RateLimitAction::Abort {
                    return response_error(template, status, &body);
                }
                let exhausted = matches!(state, Some(state) if state.remaining() == Some(0));
                match state {
                    Some(state) if exhausted => state.exhaust_for(retry_after, now),
                    _ => {
                        drop(state.take());
                        if cancellable(
                            abort,
                            inner
                                .transport
                                .sleep(retry_after + inner.ratelimiter.offset()),
                        )
                        .await
                        .is_none()
                        {
                            return Settled::Done(Err(Error::new(ErrorKind::Aborted)));
                        }
                    }
                }
                inner.metrics.record_retry();
                Settled::Retry
            }
            status if status.is_server_error() && *attempt < inner.retry.max_retries => {
                *attempt += 1;
                inner.metrics.record_retry();
                tracing::warn!(
                    route = template,
                    status = status.as_u16(),
                    attempt = *attempt,
                    "server error, retrying"
                );
                let backoff = backoff_delay(*attempt, &inner.retry);
                if cancellable(abort, inner.transport.sleep(backoff))
                    .await
                    .is_none()
                {
                    return Settled::Done(Err(Error::new(ErrorKind::Aborted)));
                }
                Settled::Retry
            }
            status => response_error(template, status, &body),
        }
    }
}

fn response_error(template: &'static str, status: StatusCode, body: &Bytes) -> Settled {
    let error = serde_json::from_slice::<ApiError>(body).ok().map(Box::new);
    let body = (!body.is_empty()).then(|| {
        body.slice(..body.len().min(4096))
            .to_vec()
            .into_boxed_slice()
    });
    Settled::Done(Err(Error::new(ErrorKind::Response {
        status,
        route: template,
        error,
        body,
    })))
}

enum Settled {
    Done(Result<(StatusCode, Bytes), Error>),
    Retry,
}

impl Inner {
    fn check_invalid_requests(&self) -> Result<(), Error> {
        if self.invalid_request_limit == 0 {
            return Ok(());
        }
        let count = self.invalid.count(Instant::now());
        if count >= self.invalid_request_limit {
            return Err(Error::new(ErrorKind::InvalidRequestLimit {
                count,
                limit: self.invalid_request_limit,
            }));
        }
        Ok(())
    }

    fn uri(&self, route: Route<'_>, query: Option<&dyn Query>) -> Result<Uri, Error> {
        let mut uri = String::with_capacity(self.base.len() + 64);
        uri.push_str(&self.base);
        route.write_path(&mut uri);
        if let Some(query) = query {
            query.write(&mut QueryWriter::new(&mut uri));
        }
        Uri::try_from(uri).map_err(|source| Error::with_source(ErrorKind::BuildRequest, source))
    }

    fn build_request(
        &self,
        method: Method,
        authenticated: bool,
        uri: &Uri,
        reason: Option<&HeaderValue>,
        body: &PreparedBody,
    ) -> Result<http::Request<Body>, Error> {
        let mut builder = http::Request::builder()
            .method(method)
            .uri(uri.clone())
            .header(USER_AGENT, self.user_agent.clone());
        match &self.token {
            _ if !authenticated => {}
            Token::None => {}
            Token::Valid(token) => builder = builder.header(AUTHORIZATION, token.clone()),
            Token::Invalid => {
                return Err(Error::with_source(
                    ErrorKind::BuildRequest,
                    "the token contains characters that are not allowed in an HTTP header",
                ));
            }
        }
        if let Some(reason) = reason {
            builder = builder.header(AUDIT_LOG_REASON, reason.clone());
        }
        if let Some(content_type) = body.content_type() {
            builder = builder.header(CONTENT_TYPE, content_type.clone());
        }
        builder
            .body(body.body())
            .map_err(|source| Error::with_source(ErrorKind::BuildRequest, source))
    }
}

fn strip_version(api: &str) -> &str {
    let api = api.trim_end_matches('/');
    match api.rsplit_once("/v") {
        Some((root, version))
            if !root.is_empty() && version.bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            root
        }
        _ => api,
    }
}
fn reason_header(reason: &str) -> HeaderValue {
    let mut encoded = String::with_capacity(reason.len());
    route::write_text(&mut encoded, reason);

    HeaderValue::try_from(encoded).unwrap_or_else(|_| HeaderValue::from_static(""))
}

fn is_invalid(status: StatusCode, headers: &RateLimitHeaders<'_>) -> bool {
    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => true,
        StatusCode::TOO_MANY_REQUESTS => !headers
            .scope
            .is_some_and(|scope| scope.eq_ignore_ascii_case("shared")),
        _ => false,
    }
}

fn rate_limited(body: &[u8], headers: &RateLimitHeaders<'_>) -> (Duration, bool) {
    let body: Option<RateLimitedBody> = serde_json::from_slice(body).ok();
    let retry_after = body
        .as_ref()
        .map(RateLimitedBody::retry_after)
        .or(headers.retry_after)
        .unwrap_or(Duration::from_secs(1));
    let global = headers.global || body.is_some_and(|body| body.global);
    (retry_after, global)
}

fn millis(duration: Duration) -> u64 {
    u64::try_from(duration.as_millis()).unwrap_or(u64::MAX)
}

fn backoff_delay(attempt: u8, policy: &RetryPolicy) -> Duration {
    let capped = policy
        .base
        .saturating_mul(1 << attempt.min(6))
        .min(policy.cap);
    if !policy.jitter {
        return capped;
    }
    let half_ms = u64::try_from(capped.as_millis() / 2).unwrap_or(u64::MAX);
    capped + Duration::from_millis(jitter_ms(half_ms))
}

fn jitter_ms(ceiling_ms: u64) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(ceiling_ms / 2, |elapsed| {
            u64::from(elapsed.subsec_nanos()) % ceiling_ms.saturating_add(1)
        })
}

async fn timeout(
    transport: &dyn Transport,
    limit: Duration,
    request: http::Request<Body>,
) -> Result<Result<http::Response<Bytes>, BoxError>, ()> {
    let mut send = transport.send(request);
    let mut expired = transport.sleep(limit);
    core::future::poll_fn(|cx| {
        if let core::task::Poll::Ready(response) = send.as_mut().poll(cx) {
            return core::task::Poll::Ready(Ok(response));
        }
        expired.as_mut().poll(cx).map(|()| Err(()))
    })
    .await
}

#[derive(Clone)]
pub struct Request<'a> {
    route: Route<'a>,
    reason: Option<&'a str>,
    query: Option<&'a dyn Query>,
    body: Option<Bytes>,

    urlencoded: bool,
    files: &'a [FileUpload],

    form_file: Option<(&'static str, &'a FileUpload)>,

    form_text: &'a [(&'static str, &'a str)],

    abort: Option<AbortHandle>,
}

impl<'a> Request<'a> {
    #[must_use]
    pub const fn new(route: Route<'a>) -> Self {
        Self {
            route,
            reason: None,
            query: None,
            body: None,
            urlencoded: false,
            files: &[],
            form_file: None,
            form_text: &[],
            abort: None,
        }
    }

    #[must_use]
    pub const fn reason(mut self, reason: Option<&'a str>) -> Self {
        self.reason = reason;
        self
    }

    #[must_use]
    pub const fn files(mut self, files: &'a [FileUpload]) -> Self {
        self.files = files;
        self
    }

    #[must_use]
    pub fn abort_on(mut self, handle: AbortHandle) -> Self {
        self.abort = Some(handle);
        self
    }

    #[must_use]
    pub const fn uploads(&self) -> &'a [FileUpload] {
        self.files
    }

    pub(crate) const fn form_file(mut self, name: &'static str, file: &'a FileUpload) -> Self {
        self.form_file = Some((name, file));
        self
    }

    pub(crate) fn form_urlencoded(mut self, body: String) -> Self {
        self.body = Some(Bytes::from(body));
        self.urlencoded = true;
        self
    }

    fn prepare_body(self) -> PreparedBody {
        if self.urlencoded {
            return PreparedBody::urlencoded(self.body.unwrap_or_default());
        }
        let form = Form {
            files: self.files,
            file: self.form_file,
            text: self.form_text,
        };
        PreparedBody::new(self.body, &form)
    }

    pub(crate) const fn form_text(mut self, fields: &'a [(&'static str, &'a str)]) -> Self {
        self.form_text = fields;
        self
    }

    pub(crate) const fn query(mut self, query: &'a dyn Query) -> Self {
        self.query = Some(query);
        self
    }

    #[must_use]
    pub const fn audit_log_reason(&self) -> Option<&'a str> {
        self.reason
    }

    pub fn json<B: Serialize + ?Sized>(mut self, body: &B) -> Result<Self, Error> {
        let bytes = serde_json::to_vec(body)
            .map_err(|source| Error::with_source(ErrorKind::BuildRequest, source))?;
        self.body = Some(Bytes::from(bytes));
        Ok(self)
    }

    #[must_use]
    pub const fn route(&self) -> Route<'a> {
        self.route
    }
}

impl fmt::Debug for Request<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct Size(usize);
        impl fmt::Debug for Size {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "<{} bytes>", self.0)
            }
        }
        let files: Vec<&str> = self.files.iter().map(|file| file.name.as_str()).collect();
        let text: Vec<&str> = self.form_text.iter().map(|(name, _)| *name).collect();
        f.debug_struct("Request")
            .field("route", &self.route)
            .field("reason", &self.reason)
            .field("query", &self.query)
            .field("body", &self.body.as_ref().map(|body| Size(body.len())))
            .field("urlencoded", &self.urlencoded)
            .field("files", &files)
            .field("form_file", &self.form_file.map(|(name, _)| name))
            .field("form_text", &text)
            .finish()
    }
}

#[must_use = "a builder does nothing until `build` is called"]
pub struct RestBuilder {
    token: Option<(AuthScheme, String)>,
    api: String,
    version: u8,
    timeout: Duration,
    retry: RetryPolicy,
    global_requests_per_second: u32,
    offset: Duration,
    user_agent_appendix: Option<String>,
    invalid_request_limit: u32,
    cdn: String,
    media_proxy: String,
    transport: Option<Arc<dyn Transport>>,
    on_rate_limit: Option<RateLimitHook>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuthScheme {
    Bot,
    Bearer,
}

impl AuthScheme {
    const fn prefix(self) -> &'static str {
        match self {
            Self::Bot => "Bot",
            Self::Bearer => "Bearer",
        }
    }
}

impl fmt::Debug for RestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RestBuilder")
            .field(
                "token",
                &match self.token {
                    None => "<none>",
                    Some((AuthScheme::Bot, _)) => "Bot <redacted>",
                    Some((AuthScheme::Bearer, _)) => "Bearer <redacted>",
                },
            )
            .field("api", &self.api)
            .field("version", &self.version)
            .field("timeout", &self.timeout)
            .field("retry", &self.retry)
            .field(
                "global_requests_per_second",
                &self.global_requests_per_second,
            )
            .field("offset", &self.offset)
            .field("user_agent_appendix", &self.user_agent_appendix)
            .field("invalid_request_limit", &self.invalid_request_limit)
            .field("cdn", &self.cdn)
            .field("media_proxy", &self.media_proxy)
            .field("transport", &self.transport.as_ref().map(|_| "<custom>"))
            .field(
                "on_rate_limit",
                &self.on_rate_limit.as_ref().map(|_| "<hook>"),
            )
            .finish()
    }
}

impl Default for RestBuilder {
    fn default() -> Self {
        Self {
            token: None,
            api: DEFAULT_API.to_owned(),
            version: crate::model::gateway::API_VERSION,
            timeout: Duration::from_secs(15),
            retry: RetryPolicy::default(),
            global_requests_per_second: 50,
            offset: Duration::from_millis(50),
            user_agent_appendix: None,
            invalid_request_limit: 0,
            cdn: cdn::DEFAULT_CDN.to_owned(),
            media_proxy: cdn::DEFAULT_MEDIA_PROXY.to_owned(),
            transport: None,
            on_rate_limit: None,
        }
    }
}

impl RestBuilder {
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some((AuthScheme::Bot, token.into()));
        self
    }

    pub fn bearer_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some((AuthScheme::Bearer, token.into()));
        self
    }

    pub fn cdn(mut self, cdn: impl Into<String>) -> Self {
        self.cdn = cdn.into();
        self
    }

    pub fn media_proxy(mut self, media_proxy: impl Into<String>) -> Self {
        self.media_proxy = media_proxy.into();
        self
    }

    pub fn api(mut self, api: impl Into<String>) -> Self {
        self.api = api.into();
        self
    }

    pub const fn version(mut self, version: u8) -> Self {
        self.version = version;
        self
    }

    pub const fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub const fn retries(mut self, retries: u8) -> Self {
        self.retry.max_retries = retries;
        self
    }

    pub fn retry_policy(mut self, policy: RetryPolicy) -> Self {
        self.retry = policy;
        self
    }

    pub fn on_rate_limit(
        mut self,
        hook: impl Fn(&RateLimited) -> RateLimitAction + Send + Sync + 'static,
    ) -> Self {
        self.on_rate_limit = Some(Arc::new(hook));
        self
    }

    pub const fn global_requests_per_second(mut self, limit: u32) -> Self {
        self.global_requests_per_second = limit;
        self
    }

    pub const fn offset(mut self, offset: Duration) -> Self {
        self.offset = offset;
        self
    }

    pub const fn invalid_request_limit(mut self, limit: u32) -> Self {
        self.invalid_request_limit = limit;
        self
    }

    pub fn user_agent_appendix(mut self, appendix: impl Into<String>) -> Self {
        self.user_agent_appendix = Some(appendix.into());
        self
    }

    pub fn transport(mut self, transport: impl Transport) -> Self {
        self.transport = Some(Arc::new(transport));
        self
    }

    #[must_use]
    pub fn build(self) -> Rest {
        #[cfg(feature = "hyper")]
        let transport = self
            .transport
            .unwrap_or_else(|| Arc::new(crate::rest::transport::hyper::HyperTransport::new()));
        #[cfg(not(feature = "hyper"))]
        let transport = self.transport.expect(
            "discord-rest is built without the `hyper` feature: set a transport with \
             `RestBuilder::transport`",
        );

        let token = match self.token {
            None => Token::None,
            Some((scheme, token)) => {
                match HeaderValue::try_from(format!("{} {token}", scheme.prefix())) {
                    Ok(mut value) => {
                        value.set_sensitive(true);
                        Token::Valid(value)
                    }
                    Err(_) => Token::Invalid,
                }
            }
        };

        let mut user_agent = format!(
            "DiscordBot (https://github.com/usslmuk/rustydiscord, {})",
            env!("CARGO_PKG_VERSION")
        );
        if let Some(appendix) = &self.user_agent_appendix {
            user_agent.push(' ');
            user_agent.push_str(appendix);
        }

        Rest {
            inner: Arc::new(Inner {
                transport,
                token,
                base: format!("{}/v{}", strip_version(&self.api), self.version),
                user_agent: HeaderValue::try_from(user_agent)
                    .expect("the user agent appendix contains invalid header characters"),
                timeout: self.timeout,
                retry: self.retry,
                ratelimiter: RateLimiter::new(self.global_requests_per_second, self.offset),
                invalid: InvalidRequests::new(),
                metrics: RestMetrics::default(),
                on_rate_limit: self.on_rate_limit,
                invalid_request_limit: self.invalid_request_limit,
                cdn: Cdn::new(self.cdn, self.media_proxy),
            }),
        }
    }
}
