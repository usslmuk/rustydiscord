use core::fmt;
use std::sync::{Arc, OnceLock, PoisonError, RwLock};

use crate::cache::{Backend, Cache, DashMapBackend};
#[cfg(feature = "gateway")]
use crate::gateway::MessageSender;
use crate::model::{
    gateway::{Intents, ShardId},
    id::{ApplicationId, GuildId},
};
use crate::rest::Rest;
use tokio::sync::watch;

use crate::{
    cache_ops::{CacheOps, converter},
    collector::Collectors,
};

#[derive(Debug)]
pub(crate) struct Shared {
    pub(crate) application_id: OnceLock<ApplicationId>,

    pub(crate) collectors: Arc<Collectors>,

    pub(crate) sequential: bool,

    pub(crate) intents: Intents,

    #[cfg(feature = "gateway")]
    pub(crate) shards: RwLock<Vec<(u32, MessageSender)>>,

    pub(crate) total_shards: core::sync::atomic::AtomicU32,

    pub(crate) shutdown: watch::Sender<bool>,

    #[cfg(feature = "sharding")]
    pub(crate) link: std::sync::Mutex<Option<Arc<crate::sharding::Link>>>,
}

impl Default for Shared {
    fn default() -> Self {
        Self {
            application_id: OnceLock::new(),
            collectors: Arc::default(),
            sequential: false,
            intents: Intents::empty(),
            #[cfg(feature = "gateway")]
            shards: RwLock::default(),
            total_shards: core::sync::atomic::AtomicU32::new(0),
            shutdown: watch::channel(false).0,
            #[cfg(feature = "sharding")]
            link: std::sync::Mutex::new(None),
        }
    }
}

#[cfg(feature = "gateway")]
impl Shared {
    pub(crate) fn shard(&self, number: u32) -> Option<MessageSender> {
        self.shards
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .iter()
            .find(|(shard, _)| *shard == number)
            .map(|(_, sender)| sender.clone())
    }
}

#[derive(Debug)]
pub struct Context<B: Backend = DashMapBackend> {
    inner: Arc<Inner<B>>,
}

#[derive(Debug)]
struct Inner<B: Backend> {
    core: Core,
    cache: Option<Arc<Cache<B>>>,
}

pub(crate) struct Core {
    rest: Rest,
    shard: ShardId,
    #[cfg(feature = "gateway")]
    sender: MessageSender,
    shared: Arc<Shared>,

    ops: Arc<dyn CacheOps>,

    #[cfg(feature = "http-interactions")]
    inline: Option<crate::http_interactions::Inline>,
}

impl fmt::Debug for Core {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = f.debug_struct("Core");
        debug.field("rest", &self.rest).field("shard", &self.shard);
        #[cfg(feature = "gateway")]
        debug.field("sender", &self.sender);
        debug.field("shared", &self.shared).finish_non_exhaustive()
    }
}

impl Core {
    pub(crate) fn collectors(&self) -> &Arc<Collectors> {
        &self.shared.collectors
    }

    pub(crate) const fn shard_id(&self) -> ShardId {
        self.shard
    }

    pub(crate) fn ops(&self) -> &dyn CacheOps {
        &*self.ops
    }

    #[cfg(feature = "http-interactions")]
    pub(crate) fn take_inline(
        &self,
        interaction: crate::model::id::InteractionId,
    ) -> Option<crate::http_interactions::InitialSender> {
        self.inline.as_ref()?.take(interaction)
    }

    #[cfg(feature = "gateway")]
    pub(crate) fn guild_shard(&self, guild: GuildId) -> Option<MessageSender> {
        let own = self.shard;
        let number = ShardId::for_guild(guild, own.total()).number();
        if number == own.number() {
            Some(self.sender.clone())
        } else {
            self.shared.shard(number)
        }
    }
}

impl<B: Backend> Clone for Context<B> {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<B: Backend> Context<B> {
    pub(crate) fn new(
        rest: Rest,
        shard: ShardId,
        #[cfg(feature = "gateway")] sender: MessageSender,
        cache: Option<Arc<Cache<B>>>,
        state: Arc<Shared>,
    ) -> Self {
        let ops = match &cache {
            Some(cache) => Arc::clone(cache) as Arc<dyn CacheOps>,
            None => converter(),
        };
        Self {
            inner: Arc::new(Inner {
                core: Core {
                    rest,
                    shard,
                    #[cfg(feature = "gateway")]
                    sender,
                    shared: state,
                    ops,
                    #[cfg(feature = "http-interactions")]
                    inline: None,
                },
                cache,
            }),
        }
    }

    #[cfg(feature = "http-interactions")]
    pub(crate) fn http(
        rest: Rest,
        shard: ShardId,
        #[cfg(feature = "gateway")] sender: MessageSender,
        cache: Option<Arc<Cache<B>>>,
        state: Arc<Shared>,
        inline: crate::http_interactions::Inline,
    ) -> Self {
        #[cfg(feature = "gateway")]
        let mut ctx = Self::new(rest, shard, sender, cache, state);
        #[cfg(not(feature = "gateway"))]
        let mut ctx = Self::new(rest, shard, cache, state);
        if let Some(inner) = Arc::get_mut(&mut ctx.inner) {
            inner.core.inline = Some(inline);
        }
        ctx
    }

    pub(crate) fn core(&self) -> &Core {
        &self.inner.core
    }

    pub(crate) fn ops(&self) -> &dyn CacheOps {
        self.inner.core.ops()
    }

    pub(crate) fn collectors(&self) -> &Arc<Collectors> {
        self.inner.core.collectors()
    }

    pub(crate) fn is_sequential(&self) -> bool {
        self.inner.core.shared.sequential
    }

    pub(crate) fn intents(&self) -> Intents {
        self.inner.core.shared.intents
    }

    #[must_use]
    pub fn application_id(&self) -> Option<ApplicationId> {
        self.inner.core.shared.application_id.get().copied()
    }

    #[track_caller]
    pub(crate) fn expect_application_id(&self) -> ApplicationId {
        self.application_id()
            .expect("the application ID is known from the first READY; this handler ran before it")
    }

    #[must_use]
    pub fn rest(&self) -> &Rest {
        &self.inner.core.rest
    }

    #[must_use]
    pub fn cache(&self) -> Option<&Cache<B>> {
        self.inner.cache.as_deref()
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    #[must_use]
    pub fn shard_id(&self) -> ShardId {
        self.inner.core.shard
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    #[must_use]
    pub fn shard(&self) -> &MessageSender {
        &self.inner.core.sender
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub fn shutdown(&self) {
        self.inner.core.shared.shutdown.send_replace(true);
    }
}
