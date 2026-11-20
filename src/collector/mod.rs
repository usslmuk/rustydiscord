mod index;

use core::{
    fmt,
    future::{Future, IntoFuture},
    marker::PhantomData,
    ops::{Deref, DerefMut},
    pin::Pin,
    task::{Context as TaskContext, Poll},
    time::Duration,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use crate::cache::Backend;
use crate::collection::Collection;
use crate::model::{
    component::ComponentType,
    emoji::PartialEmoji,
    gateway::Intents,
    gateway::payload::{
        MessageCreate, MessageReactionAdd, MessageReactionRemove, MessageReactionRemoveAll,
    },
    id::{ChannelId, GuildId, MessageId, UserId},
};
use futures_core::Stream;
use tokio::{
    sync::mpsc,
    time::{Instant, Sleep},
};

pub(crate) use self::index::{
    Collectors, Key, MemberReply, NONCE_PREFIX, Registration, Signal, Sink,
};
use crate::{
    Context,
    router::{ComponentInteraction, ModalInteraction},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EndReason {
    Time,

    Idle,

    Limit,

    ProcessedLimit,

    UserLimit,

    EmojiLimit,

    MessageDelete,

    ChannelDelete,

    ThreadDelete,

    GuildDelete,

    Stopped,

    Shutdown,
}

impl EndReason {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Time => "the time elapsed",
            Self::Idle => "nothing was collected for the idle time",
            Self::Limit => "the limit was reached",
            Self::ProcessedLimit => "the processed limit was reached",
            Self::UserLimit => "the user limit was reached",
            Self::EmojiLimit => "the emoji limit was reached",
            Self::MessageDelete => "the message was deleted",
            Self::ChannelDelete => "the channel was deleted",
            Self::ThreadDelete => "the thread was deleted",
            Self::GuildDelete => "the guild was deleted",
            Self::Stopped => "the collector was stopped",
            Self::Shutdown => "the client shut down",
        }
    }
}

impl fmt::Display for EndReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CollectorError {
    reason: EndReason,
}

impl CollectorError {
    #[must_use]
    pub const fn reason(self) -> EndReason {
        self.reason
    }
}

impl fmt::Display for CollectorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "nothing was collected: {}", self.reason)
    }
}

impl std::error::Error for CollectorError {}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum ReactionEvent {
    Added(Arc<MessageReactionAdd>),

    Removed(Arc<MessageReactionRemove>),

    RemovedAll(Arc<MessageReactionRemoveAll>),
}

impl ReactionEvent {
    #[must_use]
    pub fn user_id(&self) -> Option<UserId> {
        match self {
            Self::Added(reaction) => Some(reaction.user_id),
            Self::Removed(reaction) => Some(reaction.user_id),
            Self::RemovedAll(_) => None,
        }
    }

    #[must_use]
    pub fn emoji(&self) -> Option<&PartialEmoji> {
        match self {
            Self::Added(reaction) => Some(&reaction.emoji),
            Self::Removed(reaction) => Some(&reaction.emoji),
            Self::RemovedAll(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CollectedReaction {
    pub emoji: PartialEmoji,

    pub users: Vec<UserId>,
}

impl CollectedReaction {
    #[must_use]
    pub fn count(&self) -> usize {
        self.users.len()
    }
}

#[must_use]
pub fn reaction_key(emoji: &PartialEmoji) -> String {
    match emoji {
        PartialEmoji::Custom { id, .. } => id.to_string(),
        PartialEmoji::Unicode { name } => name.to_string(),
    }
}

#[derive(Clone, Debug)]
pub struct Collected<K, V> {
    collection: Collection<K, V>,
    reason: EndReason,
}

impl<K, V> Collected<K, V> {
    #[must_use]
    pub const fn reason(&self) -> EndReason {
        self.reason
    }

    #[must_use]
    pub fn into_collection(self) -> Collection<K, V> {
        self.collection
    }
}

impl<K, V> Deref for Collected<K, V> {
    type Target = Collection<K, V>;

    fn deref(&self) -> &Collection<K, V> {
        &self.collection
    }
}

impl<K, V> DerefMut for Collected<K, V> {
    fn deref_mut(&mut self) -> &mut Collection<K, V> {
        &mut self.collection
    }
}

pub mod mode {

    #[derive(Debug)]
    pub struct Stream;

    #[derive(Debug)]
    pub struct First;

    #[derive(Debug)]
    pub struct All;
}

pub trait Collectable: sealed::Item {}

impl Collectable for ComponentInteraction {}
impl Collectable for ModalInteraction {}
impl Collectable for Arc<MessageCreate> {}
impl Collectable for ReactionEvent {}

mod sealed {
    use super::{
        Arc, ComponentInteraction, ComponentType, MessageCreate, ModalInteraction, PartialEmoji,
        ReactionEvent, UserId,
    };
    use crate::collector::{Sink, index::Tx};

    #[derive(Debug)]
    pub enum Action<'a> {
        Collect {
            user: Option<UserId>,
            emoji: Option<&'a PartialEmoji>,
            component: Option<ComponentType>,
        },

        Remove {
            user: UserId,
            emoji: &'a PartialEmoji,
        },

        Clear,
    }

    pub trait Item: Sized + Send + 'static {
        fn sink(tx: Tx<Self>) -> Sink;

        fn action(&self) -> Action<'_>;
    }

    impl Item for ComponentInteraction {
        fn sink(tx: Tx<Self>) -> Sink {
            Sink::Components(tx)
        }

        fn action(&self) -> Action<'_> {
            Action::Collect {
                user: self.interaction().author().map(|user| user.id),
                emoji: None,
                component: Some(self.component_type),
            }
        }
    }

    impl Item for ModalInteraction {
        fn sink(tx: Tx<Self>) -> Sink {
            Sink::Modals(tx)
        }

        fn action(&self) -> Action<'_> {
            Action::Collect {
                user: self.interaction().author().map(|user| user.id),
                emoji: None,
                component: None,
            }
        }
    }

    impl Item for Arc<MessageCreate> {
        fn sink(tx: Tx<Self>) -> Sink {
            Sink::Messages(tx)
        }

        fn action(&self) -> Action<'_> {
            Action::Collect {
                user: Some(self.author.id),
                emoji: None,
                component: None,
            }
        }
    }

    impl Item for ReactionEvent {
        fn sink(tx: Tx<Self>) -> Sink {
            Sink::Reactions(tx)
        }

        fn action(&self) -> Action<'_> {
            match self {
                Self::Added(reaction) => Action::Collect {
                    user: Some(reaction.user_id),
                    emoji: Some(&reaction.emoji),
                    component: None,
                },
                Self::Removed(reaction) => Action::Remove {
                    user: reaction.user_id,
                    emoji: &reaction.emoji,
                },
                Self::RemovedAll(_) => Action::Clear,
            }
        }
    }
}

use sealed::Action;

type Filter<T> = Box<dyn FnMut(&T) -> bool + Send>;

struct Options<T> {
    time: Option<Duration>,
    idle: Option<Duration>,
    max: Option<usize>,
    max_processed: Option<usize>,
    max_users: Option<usize>,
    max_emojis: Option<usize>,
    dispose: bool,
    component_type: Option<ComponentType>,
    filter: Option<Filter<T>>,
}

impl<T> Default for Options<T> {
    fn default() -> Self {
        Self {
            time: None,
            idle: None,
            max: None,
            max_processed: None,
            max_users: None,
            max_emojis: None,
            dispose: false,
            component_type: None,
            filter: None,
        }
    }
}

impl<T> fmt::Debug for Options<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("time", &self.time)
            .field("idle", &self.idle)
            .field("max", &self.max)
            .field("max_processed", &self.max_processed)
            .field("max_users", &self.max_users)
            .field("max_emojis", &self.max_emojis)
            .field("dispose", &self.dispose)
            .field("component_type", &self.component_type)
            .field("filter", &self.filter.is_some())
            .finish()
    }
}

#[must_use = "a collector builder collects nothing until it is built or awaited"]
pub struct CollectorBuilder<T: Collectable, M = mode::Stream> {
    collectors: Arc<Collectors>,
    sequential: bool,
    intents: Intents,
    key: Key,
    guild: Option<GuildId>,
    channel: Option<ChannelId>,
    options: Options<T>,
    mode: PhantomData<fn() -> M>,
}

impl<T: Collectable, M> fmt::Debug for CollectorBuilder<T, M> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CollectorBuilder")
            .field("key", &self.key)
            .field("options", &self.options)
            .finish_non_exhaustive()
    }
}

impl<T: Collectable, M> CollectorBuilder<T, M> {
    pub(crate) fn new<B: Backend>(
        ctx: &Context<B>,
        key: Key,
        guild: Option<GuildId>,
        channel: Option<ChannelId>,
    ) -> Self {
        Self {
            collectors: Arc::clone(ctx.collectors()),
            sequential: ctx.is_sequential(),
            intents: ctx.intents(),
            key,
            guild,
            channel,
            options: Options::default(),
            mode: PhantomData,
        }
    }

    pub const fn time(mut self, time: Duration) -> Self {
        self.options.time = Some(time);
        self
    }

    pub const fn idle(mut self, idle: Duration) -> Self {
        self.options.idle = Some(idle);
        self
    }

    pub const fn max(mut self, max: usize) -> Self {
        self.options.max = Some(max);
        self
    }

    pub const fn max_processed(mut self, max: usize) -> Self {
        self.options.max_processed = Some(max);
        self
    }

    pub fn filter(mut self, filter: impl FnMut(&T) -> bool + Send + 'static) -> Self {
        self.options.filter = Some(Box::new(filter));
        self
    }

    fn start(self) -> Collector<T> {
        if self.sequential {
            tracing::warn!(
                "a collector was created in the Sequential handler mode: a handler waiting for \
                 it blocks its shard, so it receives nothing from that shard (D-267)"
            );
        }
        let (tx, rx) = mpsc::unbounded_channel();
        let sink = T::sink(tx);

        crate::client::warn_missing_intents(self.intents, sink.kinds().first().copied());
        let registration = self
            .collectors
            .register(self.key, self.guild, self.channel, sink);
        let now = Instant::now();
        let timer = |duration: Option<Duration>| {
            duration
                .and_then(|duration| now.checked_add(duration))
                .map(|deadline| Box::pin(tokio::time::sleep_until(deadline)))
        };
        Collector {
            time: timer(self.options.time),
            idle: timer(self.options.idle),
            rx,
            registration: Some(registration),
            tally: Tally::new(&self.options),
            options: self.options,
            end: None,
        }
    }
}

impl<M> CollectorBuilder<ComponentInteraction, M> {
    pub const fn component_type(mut self, kind: ComponentType) -> Self {
        self.options.component_type = Some(kind);
        self
    }

    pub const fn max_users(mut self, max: usize) -> Self {
        self.options.max_users = Some(max);
        self
    }
}

impl<M> CollectorBuilder<ReactionEvent, M> {
    pub const fn dispose(mut self, dispose: bool) -> Self {
        self.options.dispose = dispose;
        self
    }

    pub const fn max_users(mut self, max: usize) -> Self {
        self.options.max_users = Some(max);
        self
    }

    pub const fn max_emojis(mut self, max: usize) -> Self {
        self.options.max_emojis = Some(max);
        self
    }
}

impl<T: Collectable> CollectorBuilder<T, mode::Stream> {
    pub fn build(self) -> Collector<T> {
        self.start()
    }
}

impl<T: Collectable> IntoFuture for CollectorBuilder<T, mode::First> {
    type Output = Result<T, CollectorError>;
    type IntoFuture = AwaitFirst<T>;

    fn into_future(mut self) -> AwaitFirst<T> {
        self.options.max = Some(1);
        AwaitFirst {
            collector: self.start(),
        }
    }
}

macro_rules! await_all {
    ($item:ty => Collected<$key:ty, $value:ty>, $gather:expr) => {
        impl IntoFuture for CollectorBuilder<$item, mode::All> {
            type Output = Collected<$key, $value>;
            type IntoFuture = AwaitAll<$item, $key, $value>;

            fn into_future(self) -> Self::IntoFuture {
                AwaitAll {
                    collector: self.start(),
                    collection: Some(Collection::new()),
                    gather: $gather,
                }
            }
        }
    };
}

await_all!(
    Arc<MessageCreate> => Collected<MessageId, Arc<MessageCreate>>,
    |collection, message| {
        collection.insert(message.id, message);
    }
);

await_all!(
    ReactionEvent => Collected<String, CollectedReaction>,
    gather_reaction
);

fn gather_reaction(collection: &mut Collection<String, CollectedReaction>, event: ReactionEvent) {
    match event {
        ReactionEvent::Added(reaction) => {
            collection
                .as_index_map_mut()
                .entry(reaction_key(&reaction.emoji))
                .or_insert_with(|| CollectedReaction {
                    emoji: reaction.emoji.clone(),
                    users: Vec::new(),
                })
                .users
                .push(reaction.user_id);
        }
        ReactionEvent::Removed(reaction) => {
            let key = reaction_key(&reaction.emoji);
            if let Some(gathered) = collection.as_index_map_mut().get_mut(&key) {
                gathered.users.retain(|&user| user != reaction.user_id);
                if gathered.users.is_empty() {
                    collection.as_index_map_mut().shift_remove(&key);
                }
            }
        }
        ReactionEvent::RemovedAll(_) => collection.clear(),
    }
}

#[derive(Debug)]
struct Tally {
    collected: usize,
    processed: usize,

    users: Option<HashMap<UserId, usize>>,

    emojis: Option<HashMap<String, usize>>,

    pairs: Option<HashSet<(UserId, String)>>,
}

impl Tally {
    fn new<T>(options: &Options<T>) -> Self {
        Self {
            collected: 0,
            processed: 0,
            users: (options.max_users.is_some() || options.dispose).then(HashMap::new),
            emojis: (options.max_emojis.is_some() || options.dispose).then(HashMap::new),
            pairs: options.dispose.then(HashSet::new),
        }
    }

    fn collect(&mut self, user: Option<UserId>, emoji: Option<&PartialEmoji>) {
        self.collected += 1;
        if let (Some(users), Some(user)) = (&mut self.users, user) {
            *users.entry(user).or_default() += 1;
        }
        if let (Some(emojis), Some(emoji)) = (&mut self.emojis, emoji) {
            *emojis.entry(reaction_key(emoji)).or_default() += 1;
        }
        if let (Some(pairs), Some(user), Some(emoji)) = (&mut self.pairs, user, emoji) {
            pairs.insert((user, reaction_key(emoji)));
        }
    }

    fn remove(&mut self, user: UserId, emoji: &PartialEmoji) -> bool {
        let key = reaction_key(emoji);
        let Some(pairs) = &mut self.pairs else {
            return false;
        };
        let Some((_, key)) = pairs.take(&(user, key)) else {
            return false;
        };
        self.collected = self.collected.saturating_sub(1);
        if let Some(users) = &mut self.users {
            decrement(users, &user);
        }
        if let Some(emojis) = &mut self.emojis {
            decrement(emojis, &key);
        }
        true
    }

    fn clear(&mut self) {
        self.collected = 0;
        if let Some(users) = &mut self.users {
            users.clear();
        }
        if let Some(emojis) = &mut self.emojis {
            emojis.clear();
        }
        if let Some(pairs) = &mut self.pairs {
            pairs.clear();
        }
    }

    fn limit<T>(&self, options: &Options<T>) -> Option<EndReason> {
        let reached = |max: Option<usize>, count: usize| max.is_some_and(|max| count >= max);
        if reached(options.max, self.collected) {
            Some(EndReason::Limit)
        } else if reached(options.max_processed, self.processed) {
            Some(EndReason::ProcessedLimit)
        } else if reached(
            options.max_emojis,
            self.emojis.as_ref().map_or(0, HashMap::len),
        ) {
            Some(EndReason::EmojiLimit)
        } else if reached(
            options.max_users,
            self.users.as_ref().map_or(0, HashMap::len),
        ) {
            Some(EndReason::UserLimit)
        } else {
            None
        }
    }
}

fn decrement<K: core::hash::Hash + Eq>(counts: &mut HashMap<K, usize>, key: &K) {
    if let Some(count) = counts.get_mut(key) {
        *count -= 1;
        if *count == 0 {
            counts.remove(key);
        }
    }
}

#[must_use = "a collector collects nothing unless it is polled"]
pub struct Collector<T: Collectable> {
    rx: mpsc::UnboundedReceiver<Signal<T>>,
    registration: Option<Registration>,
    options: Options<T>,
    tally: Tally,
    time: Option<Pin<Box<Sleep>>>,
    idle: Option<Pin<Box<Sleep>>>,
    end: Option<EndReason>,
}

impl<T: Collectable> fmt::Debug for Collector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Collector")
            .field("options", &self.options)
            .field("collected", &self.tally.collected)
            .field("processed", &self.tally.processed)
            .field("end", &self.end)
            .finish_non_exhaustive()
    }
}

impl<T: Collectable> Collector<T> {
    pub async fn next(&mut self) -> Option<T> {
        core::future::poll_fn(|cx| Pin::new(&mut *self).poll_next(cx)).await
    }

    pub fn stop(&mut self) {
        self.finish(EndReason::Stopped);
    }

    pub fn reset_timer(&mut self, time: Option<Duration>, idle: Option<Duration>) {
        if self.end.is_some() {
            return;
        }
        if time.is_some() {
            self.options.time = time;
        }
        if idle.is_some() {
            self.options.idle = idle;
        }
        let now = Instant::now();
        let restart = |timer: &mut Option<Pin<Box<Sleep>>>, duration: Option<Duration>| {
            *timer = duration
                .and_then(|duration| now.checked_add(duration))
                .map(|deadline| Box::pin(tokio::time::sleep_until(deadline)));
        };
        restart(&mut self.time, self.options.time);
        restart(&mut self.idle, self.options.idle);
    }

    #[must_use]
    pub const fn end_reason(&self) -> Option<EndReason> {
        self.end
    }

    #[must_use]
    pub const fn is_ended(&self) -> bool {
        self.end.is_some()
    }

    #[must_use]
    pub const fn collected(&self) -> usize {
        self.tally.collected
    }

    #[must_use]
    pub const fn processed(&self) -> usize {
        self.tally.processed
    }

    fn finish(&mut self, reason: EndReason) {
        if self.end.is_none() {
            self.end = Some(reason);
            self.registration = None;
            self.time = None;
            self.idle = None;
            self.rx.close();
        }
    }

    fn process(&mut self, item: T) -> Option<T> {
        let accepted = match item.action() {
            Action::Collect {
                user,
                emoji,
                component,
            } => {
                if self
                    .options
                    .component_type
                    .is_some_and(|wanted| component != Some(wanted))
                {
                    return None;
                }
                self.tally.processed += 1;
                let accepted = self
                    .options
                    .filter
                    .as_mut()
                    .is_none_or(|filter| filter(&item));
                if accepted {
                    self.tally.collect(user, emoji);
                    if let (Some(idle), Some(timer)) = (self.options.idle, &mut self.idle)
                        && let Some(deadline) = Instant::now().checked_add(idle)
                    {
                        timer.as_mut().reset(deadline);
                    }
                }
                accepted
            }
            Action::Remove { user, emoji } => {
                self.options.dispose && self.tally.remove(user, emoji)
            }
            Action::Clear => {
                self.tally.clear();
                self.options.dispose
            }
        };
        if let Some(reason) = self.tally.limit(&self.options) {
            self.finish(reason);
        }
        accepted.then_some(item)
    }
}

impl<T: Collectable> Stream for Collector<T> {
    type Item = T;

    fn poll_next(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<T>> {
        let this = self.get_mut();
        loop {
            if this.end.is_some() {
                return Poll::Ready(None);
            }
            if this
                .time
                .as_mut()
                .is_some_and(|timer| timer.as_mut().poll(cx).is_ready())
            {
                this.finish(EndReason::Time);
                continue;
            }
            if this
                .idle
                .as_mut()
                .is_some_and(|timer| timer.as_mut().poll(cx).is_ready())
            {
                this.finish(EndReason::Idle);
                continue;
            }
            match this.rx.poll_recv(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Some(Signal::Item(item))) => {
                    if let Some(item) = this.process(item) {
                        return Poll::Ready(Some(item));
                    }
                }
                Poll::Ready(Some(Signal::End(reason))) => this.finish(reason),
                Poll::Ready(None) => this.finish(EndReason::Shutdown),
            }
        }
    }
}

#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub struct AwaitFirst<T: Collectable> {
    collector: Collector<T>,
}

impl<T: Collectable> Future for AwaitFirst<T> {
    type Output = Result<T, CollectorError>;

    fn poll(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        let collector = &mut self.get_mut().collector;
        match Pin::new(&mut *collector).poll_next(cx) {
            Poll::Ready(Some(item)) => Poll::Ready(Ok(item)),
            Poll::Ready(None) => Poll::Ready(Err(CollectorError {
                reason: collector.end.unwrap_or(EndReason::Shutdown),
            })),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct AwaitAll<T: Collectable, K, V> {
    collector: Collector<T>,
    collection: Option<Collection<K, V>>,
    gather: fn(&mut Collection<K, V>, T),
}

impl<T: Collectable, K, V> fmt::Debug for AwaitAll<T, K, V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AwaitAll")
            .field("collector", &self.collector)
            .finish_non_exhaustive()
    }
}

impl<T: Collectable, K, V> Unpin for AwaitAll<T, K, V> {}

impl<T: Collectable, K, V> Future for AwaitAll<T, K, V> {
    type Output = Collected<K, V>;

    fn poll(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        let this = self.get_mut();
        loop {
            match Pin::new(&mut this.collector).poll_next(cx) {
                Poll::Ready(Some(item)) => {
                    if let Some(collection) = &mut this.collection {
                        (this.gather)(collection, item);
                    }
                }
                Poll::Ready(None) => {
                    return Poll::Ready(Collected {
                        collection: this.collection.take().unwrap_or_default(),
                        reason: this.collector.end.unwrap_or(EndReason::Shutdown),
                    });
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

impl<B: Backend> Context<B> {
    fn channel_guild(&self, channel: ChannelId) -> Option<GuildId> {
        self.ops().channel(channel)?.guild_id()
    }

    pub fn message_collector(&self, channel: ChannelId) -> CollectorBuilder<Arc<MessageCreate>> {
        let guild = self.channel_guild(channel);
        CollectorBuilder::new(self, Key::Channel(channel), guild, Some(channel))
    }

    pub fn await_messages(
        &self,
        channel: ChannelId,
    ) -> CollectorBuilder<Arc<MessageCreate>, mode::All> {
        let guild = self.channel_guild(channel);
        CollectorBuilder::new(self, Key::Channel(channel), guild, Some(channel))
    }

    pub fn component_collector(
        &self,
        channel: ChannelId,
    ) -> CollectorBuilder<ComponentInteraction> {
        let guild = self.channel_guild(channel);
        CollectorBuilder::new(self, Key::Channel(channel), guild, Some(channel))
    }

    pub fn await_component(
        &self,
        channel: ChannelId,
    ) -> CollectorBuilder<ComponentInteraction, mode::First> {
        let guild = self.channel_guild(channel);
        CollectorBuilder::new(self, Key::Channel(channel), guild, Some(channel))
    }
}
