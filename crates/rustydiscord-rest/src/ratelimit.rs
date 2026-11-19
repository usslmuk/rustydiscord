use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, MutexGuard, PoisonError},
    time::Duration,
};

use crate::{
    error::seconds,
    route::BucketRoute,
    transport::{Instant, Transport, sleep_until},
};
use http::HeaderMap;

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum BucketId {
    Route(&'static str),
    Hash(Arc<str>),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct BucketKey {
    id: BucketId,
    major: u64,
}

#[derive(Debug)]
pub(crate) struct RateLimiter {
    hashes: Mutex<HashMap<&'static str, Arc<str>>>,
    buckets: Mutex<Buckets>,
    global: GlobalLimiter,
    offset: Duration,
}

#[derive(Debug)]
struct Buckets {
    map: HashMap<BucketKey, Arc<Bucket>>,

    sweep_at: usize,
}

const MIN_SWEEP_THRESHOLD: usize = 1_024;

impl RateLimiter {
    pub(crate) fn new(global_per_second: u32, offset: Duration) -> Self {
        Self {
            hashes: Mutex::new(HashMap::new()),
            buckets: Mutex::new(Buckets {
                map: HashMap::new(),
                sweep_at: MIN_SWEEP_THRESHOLD,
            }),
            global: GlobalLimiter::new(global_per_second),
            offset,
        }
    }

    pub(crate) fn bucket(&self, route: BucketRoute) -> Arc<Bucket> {
        let hash = lock(&self.hashes).get(route.template).map(Arc::clone);
        let route_key = BucketKey {
            id: BucketId::Route(route.template),
            major: route.major,
        };

        let mut buckets = lock(&self.buckets);
        if buckets.map.len() >= buckets.sweep_at {
            Self::sweep(&mut buckets);
        }
        let key = match hash {
            None => route_key,
            Some(hash) => {
                let hash_key = BucketKey {
                    id: BucketId::Hash(hash),
                    major: route.major,
                };
                if !buckets.map.contains_key(&hash_key)
                    && let Some(bucket) = buckets.map.remove(&route_key)
                {
                    buckets.map.insert(hash_key.clone(), bucket);
                }
                hash_key
            }
        };
        Arc::clone(
            buckets
                .map
                .entry(key)
                .or_insert_with(|| Arc::new(Bucket::default())),
        )
    }

    fn sweep(buckets: &mut Buckets) {
        let now = Instant::now();
        let before = buckets.map.len();
        buckets.map.retain(|_, bucket| {
            Arc::strong_count(bucket) > 1
                || bucket
                    .state
                    .try_lock()
                    .map_or(true, |state| state.is_limiting(now))
        });
        buckets.sweep_at = (buckets.map.len() * 2).max(MIN_SWEEP_THRESHOLD);
        tracing::debug!(
            removed = before - buckets.map.len(),
            remaining = buckets.map.len(),
            "swept idle rate-limit buckets"
        );
    }

    pub(crate) fn learn_hash(&self, template: &'static str, hash: &str) {
        let mut hashes = lock(&self.hashes);
        if hashes.get(template).is_none_or(|known| &**known != hash) {
            tracing::trace!(template, hash, "learned rate-limit bucket hash");
            hashes.insert(template, Arc::from(hash));
        }
    }

    pub(crate) async fn acquire_global(&self, transport: &dyn Transport) {
        self.global.acquire(self.offset, transport).await;
    }

    pub(crate) fn pause_global(&self, duration: Duration) {
        self.global.pause(duration);
    }

    pub(crate) const fn offset(&self) -> Duration {
        self.offset
    }
}

#[derive(Debug, Default)]
pub(crate) struct Bucket {
    pub(crate) state: tokio::sync::Mutex<BucketState>,
}

#[derive(Debug, Default)]
pub(crate) struct BucketState {
    remaining: Option<u32>,
    reset_at: Option<Instant>,
}

impl BucketState {
    pub(crate) async fn wait(&mut self, offset: Duration, transport: &dyn Transport) {
        if self.remaining == Some(0)
            && let Some(reset_at) = self.reset_at
        {
            if reset_at > Instant::now() {
                tracing::debug!(
                    wait_ms =
                        u64::try_from((reset_at - Instant::now()).as_millis()).unwrap_or(u64::MAX),
                    "rate-limit bucket exhausted, waiting for reset"
                );
                sleep_until(transport, reset_at + offset).await;
            }
            self.remaining = None;
            self.reset_at = None;
        }
    }

    pub(crate) fn update(&mut self, headers: &RateLimitHeaders<'_>, now: Instant) {
        if let Some(remaining) = headers.remaining {
            self.remaining = Some(remaining);
        }
        if let Some(reset_after) = headers.reset_after {
            self.reset_at = Some(now + reset_after);
        }
    }

    pub(crate) fn exhaust_for(&mut self, duration: Duration, now: Instant) {
        self.remaining = Some(0);
        self.reset_at = Some(now + duration);
    }

    fn is_limiting(&self, now: Instant) -> bool {
        self.remaining == Some(0) && self.reset_at.is_some_and(|reset_at| reset_at > now)
    }
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct RateLimitHeaders<'a> {
    pub(crate) bucket: Option<&'a str>,

    pub(crate) remaining: Option<u32>,

    pub(crate) reset_after: Option<Duration>,

    pub(crate) global: bool,

    pub(crate) scope: Option<&'a str>,

    pub(crate) retry_after: Option<Duration>,
}

impl<'a> RateLimitHeaders<'a> {
    pub(crate) fn parse(headers: &'a HeaderMap) -> Self {
        let text = |name: &str| headers.get(name).and_then(|value| value.to_str().ok());
        let number = |name: &str| text(name).and_then(|value| value.parse::<f64>().ok());
        Self {
            bucket: text("x-ratelimit-bucket"),
            remaining: text("x-ratelimit-remaining").and_then(|value| value.parse().ok()),
            reset_after: number("x-ratelimit-reset-after").map(seconds),
            global: text("x-ratelimit-global")
                .is_some_and(|value| value.eq_ignore_ascii_case("true")),
            scope: text("x-ratelimit-scope"),
            retry_after: number("retry-after").map(seconds),
        }
    }
}

pub(crate) const DISCORD_INVALID_REQUEST_LIMIT: u32 = 10_000;

const INVALID_WINDOW: Duration = Duration::from_secs(600);
const INVALID_SLOTS: usize = 60;
const INVALID_SLOT: Duration = Duration::from_secs(10);
const _: () = assert!(INVALID_SLOT.as_secs() * INVALID_SLOTS as u64 == INVALID_WINDOW.as_secs());

const INVALID_WARNING_STEP: u32 = 1_000;

#[derive(Debug)]
pub(crate) struct InvalidRequests {
    state: Mutex<InvalidState>,
}

#[derive(Debug)]
struct InvalidState {
    slots: [u32; INVALID_SLOTS],

    current: usize,
    slot_start: Instant,
    total: u32,
}

impl InvalidState {
    fn advance(&mut self, now: Instant) {
        let steps =
            now.saturating_duration_since(self.slot_start).as_secs() / INVALID_SLOT.as_secs();
        if steps == 0 {
            return;
        }
        if steps >= INVALID_SLOTS as u64 {
            self.slots = [0; INVALID_SLOTS];
            self.total = 0;
            self.slot_start = now;
            return;
        }
        for _ in 0..steps {
            self.current = (self.current + 1) % INVALID_SLOTS;
            self.total -= self.slots[self.current];
            self.slots[self.current] = 0;
        }

        self.slot_start += INVALID_SLOT * u32::try_from(steps).unwrap_or(u32::MAX);
    }
}

impl InvalidRequests {
    pub(crate) fn new() -> Self {
        Self {
            state: Mutex::new(InvalidState {
                slots: [0; INVALID_SLOTS],
                current: 0,
                slot_start: Instant::now(),
                total: 0,
            }),
        }
    }

    pub(crate) fn record(&self, now: Instant) -> u32 {
        let mut state = lock(&self.state);
        state.advance(now);
        let current = state.current;
        state.slots[current] += 1;
        state.total += 1;
        let total = state.total;
        drop(state);
        if total.is_multiple_of(INVALID_WARNING_STEP) {
            tracing::warn!(
                count = total,
                ban_at = DISCORD_INVALID_REQUEST_LIMIT,
                "many invalid requests (401/403/429) in the last 10 minutes; Discord bans the IP \
                 temporarily at 10 000"
            );
        }
        total
    }

    pub(crate) fn count(&self, now: Instant) -> u32 {
        let mut state = lock(&self.state);
        state.advance(now);
        state.total
    }
}

#[derive(Debug)]
struct GlobalLimiter {
    per_second: u32,
    state: Mutex<GlobalState>,
}

#[derive(Debug)]
struct GlobalState {
    sent: VecDeque<Instant>,
    paused_until: Option<Instant>,
}

const WINDOW: Duration = Duration::from_secs(1);

impl GlobalLimiter {
    fn new(per_second: u32) -> Self {
        Self {
            per_second,
            state: Mutex::new(GlobalState {
                sent: VecDeque::new(),
                paused_until: None,
            }),
        }
    }

    async fn acquire(&self, offset: Duration, transport: &dyn Transport) {
        let window = WINDOW + offset;
        loop {
            let wait_until = {
                let mut state = lock(&self.state);
                let now = Instant::now();
                match state.paused_until {
                    Some(until) if until > now => Some(until + offset),
                    _ => {
                        state.paused_until = None;
                        if self.per_second == 0 {
                            return;
                        }
                        while state.sent.front().is_some_and(|&sent| sent + window <= now) {
                            state.sent.pop_front();
                        }
                        if state.sent.len() < self.per_second as usize {
                            state.sent.push_back(now);
                            return;
                        }
                        state.sent.front().map(|&oldest| oldest + window)
                    }
                }
            };
            if let Some(until) = wait_until {
                tracing::debug!("global rate limit reached, waiting");
                sleep_until(transport, until).await;
            }
        }
    }

    fn pause(&self, duration: Duration) {
        let until = Instant::now() + duration;
        let mut state = lock(&self.state);
        if state.paused_until.is_none_or(|current| current < until) {
            state.paused_until = Some(until);
        }
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;

    struct Tokio;

    impl Transport for Tokio {
        fn send(
            &self,
            _: http::Request<crate::Body>,
        ) -> crate::BoxFuture<
            '_,
            Result<http::Response<bytes::Bytes>, Box<dyn std::error::Error + Send + Sync>>,
        > {
            unreachable!("the rate limiter sends nothing")
        }

        fn sleep(&self, duration: Duration) -> crate::BoxFuture<'static, ()> {
            Box::pin(tokio::time::sleep(duration))
        }
    }

    use super::*;
    use crate::route::Limits;

    #[test]
    fn parse_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("x-ratelimit-bucket", HeaderValue::from_static("abcd1234"));
        headers.insert("x-ratelimit-remaining", HeaderValue::from_static("0"));
        headers.insert("x-ratelimit-reset-after", HeaderValue::from_static("64.57"));
        headers.insert("x-ratelimit-global", HeaderValue::from_static("true"));
        headers.insert("x-ratelimit-scope", HeaderValue::from_static("global"));
        headers.insert("retry-after", HeaderValue::from_static("65"));
        let parsed = RateLimitHeaders::parse(&headers);
        assert_eq!(parsed.bucket, Some("abcd1234"));
        assert_eq!(parsed.remaining, Some(0));
        assert_eq!(parsed.reset_after, Some(Duration::from_secs_f64(64.57)));
        assert!(parsed.global);
        assert_eq!(parsed.scope, Some("global"));
        assert_eq!(parsed.retry_after, Some(Duration::from_secs(65)));
        assert_eq!(
            RateLimitHeaders::parse(&HeaderMap::new()),
            RateLimitHeaders::default()
        );
    }

    #[test]
    fn learned_hash_keeps_bucket_state() {
        let limiter = RateLimiter::new(50, Duration::ZERO);
        let route = BucketRoute {
            template: "POST /channels/:id/messages",
            major: 1,
            limits: Limits::NORMAL,
        };
        let before = limiter.bucket(route);
        limiter.learn_hash(route.template, "abc");

        let after = limiter.bucket(route);
        assert!(Arc::ptr_eq(&before, &after));
        assert!(Arc::ptr_eq(&after, &limiter.bucket(route)));

        let other_major = limiter.bucket(BucketRoute { major: 2, ..route });
        assert!(!Arc::ptr_eq(&after, &other_major));

        let sibling = BucketRoute {
            template: "PATCH /channels/:id/messages/:id",
            major: 1,
            limits: Limits::NORMAL,
        };
        let sibling_before = limiter.bucket(sibling);
        assert!(!Arc::ptr_eq(&after, &sibling_before));
        limiter.learn_hash(sibling.template, "abc");
        assert!(Arc::ptr_eq(&after, &limiter.bucket(sibling)));
    }

    #[tokio::test(start_paused = true)]
    async fn global_limiter_allows_a_burst_then_waits_for_the_oldest() {
        let limiter = GlobalLimiter::new(2);
        let start = Instant::now();
        let mut sent = Vec::new();
        for _ in 0..5 {
            limiter.acquire(Duration::from_millis(50), &Tokio).await;
            sent.push(start.elapsed());
        }

        let ms = Duration::from_millis;
        assert_eq!(sent, [ms(0), ms(0), ms(1050), ms(1050), ms(2100)]);
    }

    #[tokio::test(start_paused = true)]
    async fn global_limiter_is_strict_in_every_sliding_second() {
        let limiter = GlobalLimiter::new(50);
        let start = Instant::now();
        let mut sent = Vec::new();

        for round in 0_u64..400 {
            if round % 7 == 0 {
                tokio::time::sleep(Duration::from_millis(round % 230)).await;
            }
            limiter.acquire(Duration::ZERO, &Tokio).await;
            sent.push(start.elapsed());
        }
        let mut first = 0;
        for last in 0..sent.len() {
            while sent[last].saturating_sub(sent[first]) >= WINDOW {
                first += 1;
            }
            assert!(
                last - first < 50,
                "{} requests in one second",
                last - first + 1
            );
        }

        assert!(lock(&limiter.state).sent.len() <= 50);
    }

    #[tokio::test(start_paused = true)]
    async fn global_pause() {
        let limiter = GlobalLimiter::new(0);
        limiter.pause(Duration::from_millis(1500));
        let start = Instant::now();
        limiter.acquire(Duration::from_millis(50), &Tokio).await;
        assert_eq!(start.elapsed(), Duration::from_millis(1550));
    }

    #[tokio::test(start_paused = true)]
    async fn exhausted_bucket_waits_for_reset() {
        let mut state = BucketState::default();
        state.exhaust_for(Duration::from_millis(300), Instant::now());
        let start = Instant::now();
        state.wait(Duration::from_millis(50), &Tokio).await;
        assert_eq!(start.elapsed(), Duration::from_millis(350));

        let start = Instant::now();
        state.wait(Duration::from_millis(50), &Tokio).await;
        assert_eq!(start.elapsed(), Duration::ZERO);
    }

    #[tokio::test(start_paused = true)]
    async fn invalid_requests_slide_out_of_the_window() {
        let invalid = InvalidRequests::new();
        let start = Instant::now();
        for _ in 0..3 {
            invalid.record(start);
        }
        tokio::time::advance(Duration::from_secs(300)).await;
        assert_eq!(invalid.record(Instant::now()), 4);

        tokio::time::advance(Duration::from_secs(299)).await;
        assert_eq!(invalid.count(Instant::now()), 4);
        tokio::time::advance(Duration::from_secs(2)).await;
        assert_eq!(invalid.count(Instant::now()), 1);
        tokio::time::advance(Duration::from_secs(300)).await;
        assert_eq!(invalid.count(Instant::now()), 0);

        invalid.record(Instant::now());
        tokio::time::advance(Duration::from_hours(24)).await;
        assert_eq!(invalid.count(Instant::now()), 0);
        assert_eq!(invalid.record(Instant::now()), 1);
    }

    #[test]
    fn sweep_keeps_active_buckets() {
        let limiter = RateLimiter::new(50, Duration::ZERO);
        let held = limiter.bucket(BucketRoute {
            template: "T",
            major: 0,
            limits: Limits::NORMAL,
        });
        for major in 1..=(MIN_SWEEP_THRESHOLD as u64 + 1) {
            drop(limiter.bucket(BucketRoute {
                template: "T",
                major,
                limits: Limits::NORMAL,
            }));
        }
        let buckets = lock(&limiter.buckets);

        assert!(buckets.map.len() <= 3, "{}", buckets.map.len());
        drop(buckets);
        assert!(Arc::strong_count(&held) >= 2);
    }
}
