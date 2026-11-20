use core::{
    ops::Range,
    pin::Pin,
    sync::atomic::{AtomicBool, AtomicU32, Ordering},
    task::{Context as TaskContext, Poll},
    time::Duration,
};
use std::{
    future::Future,
    sync::{Arc, Mutex, MutexGuard, PoisonError, RwLock},
};

use crate::cache::{Backend, Cache, CacheConfig, DashMapBackend};
#[cfg(feature = "gateway")]
use crate::gateway::{
    Compression, Config, IdentifyQueue, LocalIdentifyQueue, MessageSender, ReceiveError,
    ReceiveErrorKind, SendError, Shard, ShardState, create_shards,
};
use crate::model::gateway::{
    DispatchEvent, GatewayEvent, Intents, ShardId, payload::UpdatePresence,
};
use crate::rest::Rest;
use futures_core::Stream;
use tokio::{
    sync::{mpsc, watch},
    task::JoinSet,
    time::{Instant, MissedTickBehavior},
};

use crate::{
    Context, Error,
    collector::EndReason,
    context::Shared,
    dispatch::{ReadySeen, ShardTracker, convert},
    event::{Dispatch, Event, EventKind, ShardEvent, payload},
    handler::{ErasedHandler, Handler, HandlerError, Registry},
    router::{
        AutocompleteInteraction, CommandInteraction, ComponentInteraction, ModalInteraction, Route,
    },
};

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(crate) fn warn_missing_intents(
    intents: Intents,
    kinds: impl IntoIterator<Item = EventKind>,
) -> Option<String> {
    use core::fmt::Write as _;

    let mut missing = String::new();
    for kind in kinds {
        let needed = kind.intents();
        if !needed.is_empty() && !intents.intersects(needed) {
            let separator = if missing.is_empty() { "" } else { ", " };
            let _ = write!(missing, "{separator}{} ({needed:?})", kind.name());
        }
    }
    if missing.is_empty() {
        return None;
    }
    tracing::warn!(
        events = %missing,
        "these events are never received without one of their intents; add them with \
         ClientBuilder::intents"
    );
    Some(missing)
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HandlerMode {
    #[default]
    Concurrent,

    Sequential,
}

#[cfg(feature = "gateway")]
#[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Shards {
    #[default]
    Auto,

    Count(u32),

    Range {
        range: Range<u32>,

        total: u32,
    },
}

pub struct Client<B: Backend = DashMapBackend> {
    inner: Arc<Inner<B>>,
}

struct Inner<B: Backend> {
    token: Box<str>,
    intents: Intents,
    rest: Rest,
    #[cfg(feature = "gateway")]
    compression: Compression,
    #[cfg(feature = "gateway")]
    gateway_url: Option<Box<str>>,
    #[cfg(feature = "gateway")]
    shards: Shards,
    #[cfg(feature = "gateway")]
    presence: Option<UpdatePresence>,
    mode: HandlerMode,
    cache: Option<Arc<Cache<B>>>,
    registry: RwLock<Arc<Registry<B>>>,
    subscribers: Mutex<Vec<mpsc::UnboundedSender<ShardEvent>>>,
    has_subscribers: AtomicBool,
    running: AtomicBool,

    shard_count: AtomicU32,
    ready_shards: AtomicU32,

    shared: Arc<Shared>,
    compiled: Compiled<B>,

    #[cfg(feature = "sharding")]
    cluster: Option<crate::sharding::ClusterAssignment>,

    #[cfg(feature = "sharding")]
    commands: RwLock<std::collections::HashMap<&'static str, crate::sharding::Handler<Client<B>>>>,

    #[cfg(feature = "sharding")]
    shard_slots: RwLock<Vec<Arc<crate::sharding::ShardSlot>>>,

    #[cfg(feature = "voice")]
    voice: crate::voice::VoiceManager,

    #[cfg(feature = "voice")]
    voice_checked: std::sync::Once,
}

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

struct Compiled<B: Backend> {
    build: fn(ClientBuilder<B>) -> Client<B>,
    #[cfg(feature = "gateway")]
    login: for<'a> fn(&'a Client<B>) -> BoxFuture<'a, Result<(), Error>>,
}

impl<B: Backend> Compiled<B> {
    fn generic() -> Self {
        #[cfg(feature = "gateway")]
        fn login<B: Backend>(client: &Client<B>) -> BoxFuture<'_, Result<(), Error>> {
            Box::pin(client.login_inner())
        }
        Self {
            build: ClientBuilder::build_inner,
            #[cfg(feature = "gateway")]
            login: login::<B>,
        }
    }
}

impl<B: Backend> Clone for Compiled<B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for Compiled<B> {}

impl<B: Backend> core::fmt::Debug for Compiled<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Compiled").finish_non_exhaustive()
    }
}

impl<B: Backend> Clone for Client<B> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<B: Backend> core::fmt::Debug for Client<B> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Client")
            .field("intents", &self.inner.intents)
            .field("mode", &self.inner.mode)
            .field("cache", &self.inner.cache)
            .field("running", &self.inner.running.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

impl Client {
    pub fn builder(token: impl Into<String>) -> ClientBuilder {
        Self::builder_with(token.into())
    }

    #[inline(never)]
    fn builder_with(token: String) -> ClientBuilder {
        ClientBuilder {
            token,
            intents: Intents::empty(),
            rest: None,
            #[cfg(feature = "gateway")]
            compression: Compression::default(),
            #[cfg(feature = "gateway")]
            gateway_url: None,
            #[cfg(feature = "gateway")]
            shards: Shards::default(),
            #[cfg(feature = "gateway")]
            presence: None,
            mode: HandlerMode::default(),
            cache: None,
            backend: DashMapBackend::new(),
            compiled: Compiled::generic(),
            #[cfg(feature = "sharding")]
            cluster: None,
        }
    }
}

impl<B: Backend> Client<B> {
    pub fn on<E, F, Fut>(&self, event: E, handler: F) -> &Self
    where
        E: Event,
        F: Fn(Context<B>, Arc<E::Payload>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
    {
        let _ = event;
        if self.inner.running.load(Ordering::Acquire) {
            let _ = warn_missing_intents(self.inner.intents, [E::KIND]);
        }
        let handler: Arc<dyn ErasedHandler<B>> = Arc::new(Handler::<E, F>::new(handler));
        let mut registry = self
            .inner
            .registry
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        *registry = Arc::new(registry.with(E::KIND, handler));
        self
    }

    fn change_registry(&self, change: impl FnOnce(&mut Registry<B>)) -> &Self {
        let mut registry = self
            .inner
            .registry
            .write()
            .unwrap_or_else(PoisonError::into_inner);
        let mut next = Registry::clone(&registry);
        change(&mut next);
        *registry = Arc::new(next);
        self
    }

    pub fn command<F, Fut>(&self, name: &str, handler: F) -> &Self
    where
        F: Fn(Context<B>, CommandInteraction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
    {
        let route = Arc::new(Route::new(name, handler));
        self.change_registry(|registry| {
            registry.routes.commands.insert(name.into(), route);
        })
    }

    pub fn autocomplete<F, Fut>(&self, name: &str, handler: F) -> &Self
    where
        F: Fn(Context<B>, AutocompleteInteraction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
    {
        let route = Arc::new(Route::new(name, handler));
        self.change_registry(|registry| {
            registry.routes.autocomplete.insert(name.into(), route);
        })
    }

    #[track_caller]
    pub fn component<F, Fut>(&self, name: &str, handler: F) -> &Self
    where
        F: Fn(Context<B>, ComponentInteraction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
    {
        assert!(
            !name.contains(':'),
            "a component route name cannot contain `:`: {name:?}"
        );
        let route = Arc::new(Route::new(name, handler));
        self.change_registry(|registry| {
            registry.routes.components.insert(name.into(), route);
        })
    }

    #[track_caller]
    pub fn modal<F, Fut>(&self, name: &str, handler: F) -> &Self
    where
        F: Fn(Context<B>, ModalInteraction) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,
    {
        assert!(
            !name.contains(':'),
            "a modal route name cannot contain `:`: {name:?}"
        );
        let route = Arc::new(Route::new(name, handler));
        self.change_registry(|registry| {
            registry.routes.modals.insert(name.into(), route);
        })
    }

    #[must_use]
    pub fn required_intents(&self) -> Intents {
        self.listened()
            .fold(Intents::empty(), |intents, kind| intents | kind.intents())
    }

    fn listened(&self) -> impl Iterator<Item = EventKind> {
        let registry = self.registry();
        let collectors = Arc::clone(&self.inner.shared.collectors);
        EventKind::ALL.into_iter().filter(move |&kind| {
            !registry.get(kind).is_empty()
                || (kind == EventKind::InteractionCreate && !registry.routes.is_empty())
                || collectors.wants(kind)
        })
    }

    #[must_use]
    pub fn events(&self) -> EventStream {
        let (tx, rx) = mpsc::unbounded_channel();
        lock(&self.inner.subscribers).push(tx);
        self.inner.has_subscribers.store(true, Ordering::Release);
        EventStream { rx }
    }

    #[must_use]
    pub fn rest(&self) -> &Rest {
        &self.inner.rest
    }

    #[cfg(feature = "voice")]
    #[cfg_attr(docsrs, doc(cfg(feature = "voice")))]
    #[must_use]
    pub fn voice(&self) -> &crate::voice::VoiceManager {
        self.inner.voice_checked.call_once(|| {
            if !self.inner.intents.contains(Intents::GUILD_VOICE_STATES) {
                tracing::warn!(
                    "voice needs the GUILD_VOICE_STATES intent: without it the bot never gets \
                     its voice state and joins time out; add it with ClientBuilder::intents"
                );
            }
            if crate::voice_client::bot_id(&self.inner.token).is_none() {
                tracing::warn!(
                    "the token does not start with the bot's ID, so voice cannot recognize \
                     the bot's voice state and joins time out"
                );
            }
        });
        &self.inner.voice
    }

    #[cfg(feature = "voice")]
    async fn leave_calls(&self, number: u32) {
        let total = self.inner.shared.total_shards.load(Ordering::Acquire);
        let mut leaving = tokio::task::JoinSet::new();
        for call in self.inner.voice.calls() {
            if crate::voice_client::shard_of(call.guild_id(), total) == number {
                leaving.spawn(async move {
                    let _ = call.leave().await;
                });
            }
        }

        let all = async { while leaving.join_next().await.is_some() {} };
        let _ = tokio::time::timeout(core::time::Duration::from_secs(2), all).await;
    }

    #[must_use]
    pub fn cache(&self) -> Option<&Cache<B>> {
        self.inner.cache.as_deref()
    }

    #[cfg(feature = "sharding")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sharding")))]
    #[must_use]
    pub fn cluster(&self) -> Option<crate::sharding::ClusterClient<B>> {
        self.inner
            .cluster
            .is_some()
            .then(|| crate::sharding::ClusterClient::new(self.clone()))
    }

    #[cfg(feature = "sharding")]
    pub(crate) fn assignment(&self) -> Option<&crate::sharding::ClusterAssignment> {
        self.inner.cluster.as_ref()
    }

    #[cfg(feature = "sharding")]
    pub(crate) fn link(&self) -> Option<Arc<crate::sharding::Link>> {
        self.inner
            .shared
            .link
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    #[cfg(feature = "sharding")]
    pub(crate) fn add_cluster_command(
        &self,
        name: &'static str,
        handler: crate::sharding::Handler<Client<B>>,
    ) {
        self.inner
            .commands
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(name, handler);
    }

    #[cfg(feature = "sharding")]
    fn shard_slot(&self, number: u32) -> Option<Arc<crate::sharding::ShardSlot>> {
        self.inner
            .shard_slots
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .find(|slot| slot.number() == number)
            .cloned()
    }

    #[cfg(feature = "sharding")]
    pub(crate) fn shard_stats(&self) -> Vec<crate::sharding::ShardStats> {
        self.inner
            .shard_slots
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .map(|slot| slot.stats())
            .collect()
    }

    #[cfg(feature = "sharding")]
    pub(crate) fn cluster_command(
        &self,
        name: &str,
    ) -> Option<crate::sharding::Handler<Client<B>>> {
        self.inner
            .commands
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .get(name)
            .cloned()
    }
}

#[cfg(feature = "gateway")]
#[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
impl<B: Backend> Client<B> {
    pub fn login(&self) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        (self.inner.compiled.login)(self)
    }

    async fn login_inner(&self) -> Result<(), Error> {
        if self.inner.running.swap(true, Ordering::AcqRel) {
            return Err(Error::AlreadyRunning);
        }
        let result = self.run().await;
        self.inner.running.store(false, Ordering::Release);
        result
    }

    pub fn set_presence(&self, presence: &UpdatePresence) -> Result<(), SendError> {
        let mut result = Ok(());
        let shards = self
            .inner
            .shared
            .shards
            .read()
            .unwrap_or_else(PoisonError::into_inner);
        for (_, sender) in shards.iter() {
            if let Err(error) = sender.update_presence(presence) {
                result = result.and(Err(error));
            }
        }
        result
    }

    pub fn shutdown(&self) {
        self.inner.shared.shutdown.send_replace(true);
    }

    async fn run(&self) -> Result<(), Error> {
        let inner = &*self.inner;
        let missing = warn_missing_intents(inner.intents, self.listened());
        inner.shared.shutdown.send_replace(false);

        let shutdown = inner.shared.shutdown.subscribe();

        #[cfg(feature = "sharding")]
        let manager = match &inner.cluster {
            Some(assignment) => {
                let commands = Arc::new(crate::sharding::Table {
                    client: self.clone(),
                    started: std::time::Instant::now(),
                });
                Some(
                    assignment
                        .connect(Arc::clone(&inner.shared), commands)
                        .await?,
                )
            }
            None => None,
        };

        let plan = self.plan().await?;
        #[cfg(feature = "sharding")]
        if inner.cluster.is_some() {
            *inner
                .shard_slots
                .write()
                .unwrap_or_else(PoisonError::into_inner) = plan
                .range
                .clone()
                .map(|number| Arc::new(crate::sharding::ShardSlot::new(number)))
                .collect();
        }
        tracing::info!(
            shards = ?plan.range,
            total = plan.total,
            max_concurrency = plan.max_concurrency,
            "starting shards"
        );
        self.announce_startup(missing, &plan).await;
        let identify_queue: Arc<dyn IdentifyQueue> =
            Arc::new(LocalIdentifyQueue::new(plan.max_concurrency));

        #[cfg(feature = "sharding")]
        let identify_queue = manager
            .as_ref()
            .map_or(identify_queue, crate::sharding::Connection::identify_queue);
        let mut config = Config::builder(&*inner.token, inner.intents)
            .gateway_url(plan.url)
            .compression(inner.compression)
            .identify_queue(identify_queue);
        if let Some(presence) = &inner.presence {
            config = config.presence(presence.clone());
        }
        let config = config.build();

        inner
            .shard_count
            .store(plan.range.end - plan.range.start, Ordering::Release);
        inner
            .shared
            .total_shards
            .store(plan.total, Ordering::Release);
        inner.ready_shards.store(0, Ordering::Release);

        let mut sweepers = self.spawn_sweepers();
        let mut tasks = JoinSet::new();
        for shard in create_shards(plan.range, plan.total, config) {
            inner
                .shared
                .shards
                .write()
                .unwrap_or_else(PoisonError::into_inner)
                .push((shard.id().number(), shard.sender()));
            let (client, shutdown) = (self.clone(), shutdown.clone());
            tasks.spawn(async move { client.run_shard(shard, shutdown).await });
        }

        let mut result = Ok(());
        while let Some(finished) = tasks.join_next().await {
            match finished {
                Ok(Ok(())) => {}
                Ok(Err(error)) => {
                    if result.is_ok() {
                        inner.shared.shutdown.send_replace(true);
                        result = Err(error);
                    }
                }

                Err(error) => {
                    if let Ok(panic) = error.try_into_panic() {
                        std::panic::resume_unwind(panic);
                    }
                }
            }
        }
        inner
            .shared
            .shards
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .clear();

        inner.shared.collectors.end_all(EndReason::Shutdown);
        sweepers.shutdown().await;
        result
    }

    fn spawn_sweepers(&self) -> JoinSet<()> {
        let mut tasks = JoinSet::new();
        let Some(cache) = self.inner.cache.clone() else {
            return tasks;
        };
        let sweepers = cache.config().sweepers().clone();
        if let Some(sweep) = sweepers.messages {
            let (client, ctx, cache) = (self.clone(), self.manager_ctx(1), Arc::clone(&cache));
            let lifetime = sweep.lifetime;
            tasks.spawn(async move {
                sweep_every(
                    client,
                    ctx,
                    sweep.interval,
                    "messages",
                    move || cache.sweep_messages_older_than(lifetime),
                    move |swept| format!("Swept {swept} messages older than {lifetime:?}."),
                )
                .await;
            });
        }
        if let Some(sweep) = sweepers.threads {
            let (client, ctx, cache) = (self.clone(), self.manager_ctx(1), Arc::clone(&cache));
            let lifetime = sweep.lifetime;
            tasks.spawn(async move {
                sweep_every(
                    client,
                    ctx,
                    sweep.interval,
                    "threads",
                    move || cache.sweep_archived_threads(lifetime),
                    move |swept| format!("Swept {swept} archived threads."),
                )
                .await;
            });
        }
        tasks
    }

    fn on_ready(
        &self,
        seen: ReadySeen,
        counted_ready: &mut bool,
        registry: &Registry<B>,
        out: &mut Vec<Dispatch>,
    ) {
        let _ = self.inner.shared.application_id.set(seen.application_id);
        if *counted_ready {
            return;
        }
        *counted_ready = true;
        let ready_shards = self.inner.ready_shards.fetch_add(1, Ordering::AcqRel) + 1;
        if ready_shards == self.inner.shard_count.load(Ordering::Acquire)
            && let Some(ready) = seen.ready
            && self.wants(registry, EventKind::ClientReady)
        {
            out.push(Dispatch::ClientReady(ready));
        }
    }

    async fn run_shard(
        &self,
        mut shard: Shard,
        mut shutdown: watch::Receiver<bool>,
    ) -> Result<(), Error> {
        let ctx = Context::new(
            self.inner.rest.clone(),
            shard.id(),
            shard.sender(),
            self.inner.cache.clone(),
            Arc::clone(&self.inner.shared),
        );
        let mut tracker = ShardTracker::default();
        let mut out = Vec::new();

        let mut active = false;

        let mut replayed = 0;

        let mut counted_ready = false;

        #[cfg(feature = "sharding")]
        let mut reporter = crate::sharding::Reporter::new(self.shard_slot(shard.id().number()));
        loop {
            let event = tokio::select! {
                biased;
                changed = shutdown.changed() => {
                    if changed.is_err() || *shutdown.borrow() {
                        #[cfg(feature = "voice")]
                        self.leave_calls(shard.id().number()).await;
                        shard.close().await;
                        return Ok(());
                    }
                    continue;
                }
                event = shard.next_event() => event,
            };
            let registry = self.registry();
            match event {
                Some(Ok(GatewayEvent::Dispatch(_, event))) => {
                    if shard.state() == ShardState::Resuming {
                        replayed += 1;
                    }
                    let resumed = matches!(event, DispatchEvent::Resumed);
                    #[cfg(feature = "voice")]
                    self.inner.voice.process(&event);
                    convert(
                        event,
                        self.inner.cache.as_deref(),
                        &mut tracker,
                        |kind| self.wants(&registry, kind),
                        &mut out,
                    );
                    if resumed {
                        let replay = core::mem::take(&mut replayed);
                        self.resumed_dispatch(&registry, &shard, replay, &mut out);
                    }
                    if let Some(seen) = tracker.take_ready() {
                        self.on_ready(seen, &mut counted_ready, &registry, &mut out);
                    }
                }
                Some(Ok(GatewayEvent::InvalidSession(false))) => {
                    if self.wants(&registry, EventKind::Invalidated) {
                        out.push(Dispatch::Invalidated(Arc::new(payload::Invalidated)));
                    }
                }
                Some(Ok(GatewayEvent::HeartbeatAck)) => {
                    #[cfg(feature = "sharding")]
                    reporter.latency(shard.latency());
                    self.heartbeat_dispatch(&registry, &shard, &mut out);
                }
                Some(Ok(_)) => {}
                Some(Err(error)) if error.is_fatal() => {
                    self.fail_shard(&ctx, &registry, &shard, &error).await;
                    return Err(error.into());
                }
                Some(Err(error)) => {
                    tracing::warn!(shard = %shard.id(), %error, "gateway error, recovering");
                    if self.wants(&registry, EventKind::Warn) {
                        out.push(Dispatch::Warn(Arc::new(format!(
                            "[Shard {}] gateway error, recovering: {error}",
                            shard.id()
                        ))));
                    }
                    if self.wants(&registry, EventKind::ShardError) {
                        out.push(Dispatch::ShardError(Arc::new(error)));
                    }
                }
                None => return Ok(()),
            }

            self.track_state(&registry, &mut active, &shard, &mut out);
            #[cfg(feature = "sharding")]
            reporter.state(shard.state());
            for dispatch in out.drain(..) {
                self.emit(&ctx, &registry, dispatch).await;
            }
        }
    }

    fn track_state(
        &self,
        registry: &Registry<B>,
        active: &mut bool,
        shard: &Shard,
        out: &mut Vec<Dispatch>,
    ) {
        let state = shard.state();
        if *active
            && !matches!(
                state,
                ShardState::Active | ShardState::Closed | ShardState::FatallyClosed(_)
            )
            && self.wants(registry, EventKind::ShardReconnecting)
        {
            out.push(Dispatch::ShardReconnecting(Arc::new(
                payload::ShardReconnecting,
            )));
        }
        *active = state == ShardState::Active;
    }

    async fn plan(&self) -> Result<Plan, Error> {
        let inner = &*self.inner;

        #[cfg(feature = "sharding")]
        if let Some(assignment) = &inner.cluster {
            return Ok(Plan {
                url: inner
                    .gateway_url
                    .as_deref()
                    .unwrap_or(assignment.gateway_url())
                    .to_owned(),
                range: assignment.shards(),
                total: assignment.total_shards(),
                max_concurrency: assignment.max_concurrency(),
            });
        }
        let fixed = match &inner.shards {
            Shards::Auto => None,
            Shards::Count(count) => Some((0..*count, *count)),
            Shards::Range { range, total } => Some((range.clone(), *total)),
        };
        if let (Some(url), Some((range, total))) = (&inner.gateway_url, fixed.clone()) {
            return Ok(Plan {
                url: url.to_string(),
                range,
                total,
                max_concurrency: 1,
            });
        }

        let gateway = inner.rest.get_gateway_bot().await?;
        let (range, total) = fixed.unwrap_or_else(|| {
            let total = gateway.shards.max(1);
            (0..total, total)
        });
        let limit = &gateway.session_start_limit;
        let required = range.end - range.start;
        if limit.remaining < required {
            return Err(Error::SessionStartLimit {
                required,
                remaining: limit.remaining,
                reset_after: Duration::from_millis(limit.reset_after),
            });
        }
        let url = match &inner.gateway_url {
            Some(url) => url.to_string(),
            None => gateway.url.into(),
        };
        Ok(Plan {
            url,
            range,
            total,
            max_concurrency: limit.max_concurrency,
        })
    }
}

impl<B: Backend> Client<B> {
    fn registry(&self) -> Arc<Registry<B>> {
        Arc::clone(
            &self
                .inner
                .registry
                .read()
                .unwrap_or_else(PoisonError::into_inner),
        )
    }

    #[cfg(feature = "gateway")]
    fn manager_ctx(&self, total: u32) -> Context<B> {
        Context::new(
            self.inner.rest.clone(),
            ShardId::new(0, total.max(1)),
            MessageSender::detached(),
            self.inner.cache.clone(),
            Arc::clone(&self.inner.shared),
        )
    }

    #[cfg(feature = "gateway")]
    async fn announce_startup(&self, missing: Option<String>, plan: &Plan) {
        let registry = self.registry();
        let ctx = self.manager_ctx(plan.total);
        let mut out = Vec::new();
        if let Some(missing) = missing.filter(|_| self.wants(&registry, EventKind::Warn)) {
            out.push(Dispatch::Warn(Arc::new(format!(
                "these events are never received without one of their intents: {missing}"
            ))));
        }
        if self.wants(&registry, EventKind::Debug) {
            out.push(Dispatch::Debug(Arc::new(format!(
                "starting shards {:?} of {} (max concurrency {})",
                plan.range, plan.total, plan.max_concurrency
            ))));
        }
        for dispatch in out {
            self.emit(&ctx, &registry, dispatch).await;
        }
    }

    #[cfg(feature = "gateway")]
    fn resumed_dispatch(
        &self,
        registry: &Registry<B>,
        shard: &Shard,
        replayed: u64,
        out: &mut Vec<Dispatch>,
    ) {
        if self.wants(registry, EventKind::ShardResume) {
            out.push(Dispatch::ShardResume(Arc::new(payload::ShardResume {
                replayed,
            })));
        }
        if self.wants(registry, EventKind::Debug) {
            out.push(Dispatch::Debug(Arc::new(format!(
                "[Shard {}] resumed, replaying {replayed} events",
                shard.id()
            ))));
        }
    }

    #[cfg(feature = "gateway")]
    fn heartbeat_dispatch(&self, registry: &Registry<B>, shard: &Shard, out: &mut Vec<Dispatch>) {
        if self.wants(registry, EventKind::Debug)
            && let Some(latency) = shard.latency()
        {
            out.push(Dispatch::Debug(Arc::new(format!(
                "[Shard {}] heartbeat acknowledged, latency {}ms",
                shard.id(),
                latency.as_millis()
            ))));
        }
    }

    #[cfg(feature = "gateway")]
    async fn fail_shard(
        &self,
        ctx: &Context<B>,
        registry: &Registry<B>,
        shard: &Shard,
        error: &ReceiveError,
    ) {
        if let ReceiveErrorKind::FatallyClosed { code } = error.kind() {
            let disconnect = Dispatch::ShardDisconnect(Arc::new(*code));
            self.emit(ctx, registry, disconnect).await;
        }
        if self.wants(registry, EventKind::Error) {
            self.emit(
                ctx,
                registry,
                Dispatch::Error(Arc::new(format!("[Shard {}] {error}", shard.id()))),
            )
            .await;
        }
    }

    fn wants(&self, registry: &Registry<B>, kind: EventKind) -> bool {
        !registry.get(kind).is_empty()
            || (kind == EventKind::InteractionCreate && !registry.routes.is_empty())
            || self.inner.shared.collectors.wants(kind)
            || (kind != EventKind::Raw && self.inner.has_subscribers.load(Ordering::Acquire))
    }

    async fn emit(&self, ctx: &Context<B>, registry: &Registry<B>, dispatch: Dispatch) {
        self.emit_as(ctx, registry, dispatch, self.inner.mode).await;
    }

    async fn emit_as(
        &self,
        ctx: &Context<B>,
        registry: &Registry<B>,
        dispatch: Dispatch,
        mode: HandlerMode,
    ) {
        let inner = &*self.inner;
        if !matches!(dispatch, Dispatch::Raw(_)) && inner.has_subscribers.load(Ordering::Acquire) {
            let mut subscribers = lock(&inner.subscribers);
            subscribers.retain(|subscriber| {
                subscriber
                    .send(ShardEvent {
                        shard: ctx.core().shard_id(),
                        event: dispatch.clone(),
                    })
                    .is_ok()
            });
            inner
                .has_subscribers
                .store(!subscribers.is_empty(), Ordering::Release);
        }

        let Some(kind) = dispatch.kind() else {
            return;
        };

        inner.shared.collectors.deliver(kind, &dispatch);
        let handlers = registry.get(kind);

        let routed = match &dispatch {
            Dispatch::InteractionCreate(event) if !registry.routes.is_empty() => Some(event),
            _ => None,
        };
        match mode {
            HandlerMode::Concurrent => {
                for handler in handlers {
                    handler.spawn(ctx, &dispatch);
                }
                if let Some(event) = routed {
                    registry.routes.spawn(ctx, event);
                }
            }
            HandlerMode::Sequential => {
                for handler in handlers {
                    if let Some(future) = handler.call(ctx, &dispatch) {
                        future.await;
                    }
                }
                if let Some(future) = routed.and_then(|event| registry.routes.call(ctx, event)) {
                    future.await;
                }
            }
        }
    }
}

#[cfg(feature = "http-interactions")]
impl<B: Backend> Client<B> {
    pub fn interactions_endpoint(
        &self,
        public_key: &str,
    ) -> Result<
        crate::http_interactions::InteractionsEndpoint<B>,
        crate::http_interactions::InvalidPublicKey,
    > {
        crate::http_interactions::InteractionsEndpoint::new(self.clone(), public_key)
    }

    pub(crate) fn dispatch_http(
        &self,
        event: crate::model::gateway::payload::InteractionCreate,
    ) -> (
        crate::http_interactions::InitialReceiver,
        BoxFuture<'static, ()>,
    ) {
        let inner = &*self.inner;
        let state = &inner.shared;

        let _ = state.application_id.set(event.application_id);
        let (inline, initial) = crate::http_interactions::Inline::new(event.id);

        #[cfg(feature = "gateway")]
        let ctx = {
            let total = state.total_shards.load(Ordering::Acquire);
            let (shard, sender) = if total == 0 {
                (
                    crate::model::gateway::ShardId::ONE,
                    crate::gateway::MessageSender::detached(),
                )
            } else {
                let number = event.guild_id.map_or(0, |guild| {
                    crate::model::gateway::ShardId::for_guild(guild, total).number()
                });
                let sender = state
                    .shard(number)
                    .unwrap_or_else(crate::gateway::MessageSender::detached);
                (crate::model::gateway::ShardId::new(number, total), sender)
            };
            let (rest, cache) = (inner.rest.clone(), inner.cache.clone());
            Context::http(rest, shard, sender, cache, Arc::clone(state), inline)
        };
        #[cfg(not(feature = "gateway"))]
        let ctx = Context::http(
            inner.rest.clone(),
            crate::model::gateway::ShardId::ONE,
            inner.cache.clone(),
            Arc::clone(state),
            inline,
        );
        let mut out = Vec::with_capacity(1);
        convert(
            DispatchEvent::InteractionCreate(Box::new(event)),
            inner.cache.as_deref(),
            &mut ShardTracker::default(),
            |kind| kind == EventKind::InteractionCreate,
            &mut out,
        );
        let client = self.clone();
        let registry = self.registry();
        let work = async move {
            for dispatch in out {
                client
                    .emit_as(&ctx, &registry, dispatch, HandlerMode::Sequential)
                    .await;
            }
        };
        (initial, Box::pin(work))
    }
}

#[cfg(feature = "gateway")]
async fn sweep_every<B: Backend>(
    client: Client<B>,
    ctx: Context<B>,
    period: Duration,
    resource: &'static str,
    sweep: impl Fn() -> usize,
    message: impl Fn(usize) -> String,
) {
    let Some(mut interval) = sweep_start(period) else {
        tracing::warn!(resource, "a cache sweep with a zero interval is disabled");
        return;
    };
    loop {
        interval.tick().await;
        let swept = sweep();
        tracing::debug!(resource, swept, "swept the cache");
        let registry = client.registry();
        if client.wants(&registry, EventKind::CacheSweep) {
            client
                .emit(
                    &ctx,
                    &registry,
                    Dispatch::CacheSweep(Arc::new(message(swept))),
                )
                .await;
        }
    }
}

#[cfg(feature = "gateway")]
fn sweep_start(period: Duration) -> Option<tokio::time::Interval> {
    if period.is_zero() {
        return None;
    }
    let start = Instant::now().checked_add(period)?;
    let mut interval = tokio::time::interval_at(start, period);
    interval.set_missed_tick_behavior(MissedTickBehavior::Delay);
    Some(interval)
}

#[cfg(feature = "gateway")]
struct Plan {
    url: String,
    range: Range<u32>,
    total: u32,
    max_concurrency: u32,
}

#[derive(Debug)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct ClientBuilder<B: Backend = DashMapBackend> {
    token: String,
    intents: Intents,
    rest: Option<Rest>,
    #[cfg(feature = "gateway")]
    compression: Compression,
    #[cfg(feature = "gateway")]
    gateway_url: Option<Box<str>>,
    #[cfg(feature = "gateway")]
    shards: Shards,
    #[cfg(feature = "gateway")]
    presence: Option<UpdatePresence>,
    mode: HandlerMode,
    cache: Option<CacheConfig>,
    backend: B,
    compiled: Compiled<B>,
    #[cfg(feature = "sharding")]
    cluster: Option<crate::sharding::ClusterAssignment>,
}

impl<B: Backend> ClientBuilder<B> {
    pub const fn intents(mut self, intents: Intents) -> Self {
        self.intents = intents;
        self
    }

    pub fn rest(mut self, rest: Rest) -> Self {
        self.rest = Some(rest);
        self
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub const fn compression(mut self, compression: Compression) -> Self {
        self.compression = compression;
        self
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub fn gateway_url(mut self, url: impl Into<String>) -> Self {
        self.gateway_url = Some(url.into().into_boxed_str());
        self
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    #[track_caller]
    pub fn shards(mut self, shards: Shards) -> Self {
        match &shards {
            Shards::Auto => {}
            Shards::Count(count) => assert!(*count > 0, "the shard count must not be zero"),
            Shards::Range { range, total } => assert!(
                range.start < range.end && range.end <= *total,
                "shard range {range:?} must be non-empty and within the total of {total}"
            ),
        }
        self.shards = shards;
        self
    }

    #[cfg(feature = "sharding")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sharding")))]
    pub fn cluster(mut self, assignment: crate::sharding::ClusterAssignment) -> Self {
        self.shards = Shards::Range {
            range: assignment.shards(),
            total: assignment.total_shards(),
        };
        self.cluster = Some(assignment);
        self
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub fn presence(mut self, presence: UpdatePresence) -> Self {
        self.presence = Some(presence);
        self
    }

    pub const fn handler_mode(mut self, mode: HandlerMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn cache(mut self, config: CacheConfig) -> Self {
        self.cache = Some(config);
        self
    }

    pub fn cache_backend<C: Backend>(self, backend: C) -> ClientBuilder<C> {
        ClientBuilder {
            token: self.token,
            intents: self.intents,
            rest: self.rest,
            #[cfg(feature = "gateway")]
            compression: self.compression,
            #[cfg(feature = "gateway")]
            gateway_url: self.gateway_url,
            #[cfg(feature = "gateway")]
            shards: self.shards,
            #[cfg(feature = "gateway")]
            presence: self.presence,
            mode: self.mode,
            cache: Some(self.cache.unwrap_or_default()),
            backend,
            compiled: Compiled::generic(),
            #[cfg(feature = "sharding")]
            cluster: self.cluster,
        }
    }

    #[must_use]
    pub fn build(self) -> Client<B> {
        (self.compiled.build)(self)
    }

    fn build_inner(self) -> Client<B> {
        let compiled = self.compiled;

        let rest = self
            .rest
            .unwrap_or_else(|| Rest::builder().token(self.token.clone()).build());
        let backend = self.backend;
        let cache = self
            .cache
            .map(|config| Arc::new(Cache::with_backend(config, backend)));
        let shared = Arc::new(Shared {
            sequential: self.mode == HandlerMode::Sequential,
            intents: self.intents,
            ..Shared::default()
        });

        #[cfg(feature = "voice")]
        let voice = crate::voice::VoiceManager::new(
            crate::voice_client::bot_id(&self.token)
                .unwrap_or(crate::model::id::UserId::new(u64::MAX)),
            1,
            crate::voice_client::ClientGateway {
                shared: Arc::clone(&shared),
            },
        );
        Client {
            inner: Arc::new(Inner {
                token: self.token.into_boxed_str(),
                intents: self.intents,
                rest,
                #[cfg(feature = "gateway")]
                compression: self.compression,
                #[cfg(feature = "gateway")]
                gateway_url: self.gateway_url,
                #[cfg(feature = "gateway")]
                shards: self.shards,
                #[cfg(feature = "gateway")]
                presence: self.presence,
                mode: self.mode,
                cache,
                registry: RwLock::new(Arc::new(Registry::default())),
                subscribers: Mutex::new(Vec::new()),
                has_subscribers: AtomicBool::new(false),
                running: AtomicBool::new(false),
                shard_count: AtomicU32::new(0),
                ready_shards: AtomicU32::new(0),
                shared,
                compiled,
                #[cfg(feature = "sharding")]
                cluster: self.cluster,
                #[cfg(feature = "sharding")]
                commands: RwLock::default(),
                #[cfg(feature = "sharding")]
                shard_slots: RwLock::default(),
                #[cfg(feature = "voice")]
                voice,
                #[cfg(feature = "voice")]
                voice_checked: std::sync::Once::new(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct EventStream {
    rx: mpsc::UnboundedReceiver<ShardEvent>,
}

impl EventStream {
    pub async fn next(&mut self) -> Option<ShardEvent> {
        self.rx.recv().await
    }
}

impl Stream for EventStream {
    type Item = ShardEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<ShardEvent>> {
        self.rx.poll_recv(cx)
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::{
        io,
        sync::{Arc, Mutex},
    };

    use crate::model::gateway::Intents;

    #[cfg(feature = "gateway")]
    use super::sweep_start;
    use super::warn_missing_intents;
    use crate::event::EventKind;

    #[derive(Clone, Default)]
    struct Logs(Arc<Mutex<Vec<u8>>>);

    impl io::Write for Logs {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn events_without_their_intents_are_warned_about() {
        let logs = Logs::default();
        let writer = logs.clone();
        let subscriber = tracing_subscriber::fmt()
            .with_writer(move || writer.clone())
            .with_ansi(false)
            .finish();
        tracing::subscriber::with_default(subscriber, || {
            let missing = warn_missing_intents(
                Intents::GUILDS | Intents::DIRECT_MESSAGES,
                [
                    EventKind::MessageReactionAdd,
                    EventKind::GuildCreate,
                    EventKind::InteractionCreate,
                    EventKind::MessageCreate,
                    EventKind::PresenceUpdate,
                ],
            );
            assert!(missing.is_some());
            let missing = warn_missing_intents(Intents::GUILDS, [EventKind::GuildCreate]);
            assert!(missing.is_none());
        });
        let logs = String::from_utf8(logs.0.lock().unwrap().clone()).unwrap();
        assert_eq!(logs.lines().count(), 1, "{logs}");
        assert!(logs.contains("messageReactionAdd"), "{logs}");
        assert!(logs.contains("presenceUpdate"), "{logs}");
        assert!(!logs.contains("guildCreate"), "{logs}");
        assert!(!logs.contains("messageCreate"), "{logs}");
    }

    #[cfg(feature = "gateway")]
    #[tokio::test]
    async fn sweeps_that_cannot_be_scheduled_return_at_once() {
        for period in [Duration::ZERO, Duration::MAX] {
            assert!(sweep_start(period).is_none());
        }
        assert!(sweep_start(Duration::from_secs(60)).is_some());
    }
}
