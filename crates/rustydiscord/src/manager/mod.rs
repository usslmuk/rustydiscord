mod application_emoji;
mod auto_moderation;
mod ban;
mod channel;
mod command;
mod emoji;
#[cfg(feature = "gateway")]
mod gateway;
mod guild;
mod guild_channel;
mod invite;
mod member;
mod message;
mod role;
mod scheduled_event;
mod soundboard;
mod stage_instance;
mod sticker;
mod thread;
mod user;
mod webhook;

use core::{
    fmt,
    future::{Future, IntoFuture},
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use std::sync::Arc;

use rustydiscord_cache::{Backend, CachedGuild, CachedMember, CachedMessage, CachedUser};
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    channel::Channel,
    emoji::Emoji,
    guild::{Guild, Member, Role},
    guild_scheduled_event::GuildScheduledEvent,
    id::{ChannelId, GuildId, MessageId, SoundboardSoundId, StickerId, StickerPackId},
    message::Message,
    rest::GetChannelMessages,
    soundboard::SoundboardSound,
    stage_instance::StageInstance,
    sticker::{Sticker, StickerPack},
    user::User,
};
use rustydiscord_rest::Error;

pub use self::{
    application_emoji::ApplicationEmojiManager,
    auto_moderation::AutoModerationRuleManager,
    ban::BanManager,
    channel::{ChannelManager, OverwriteTarget},
    command::{CommandManager, GuildCommandManager},
    emoji::EmojiManager,
    guild::GuildManager,
    guild_channel::GuildChannelManager,
    invite::InviteManager,
    member::MemberManager,
    message::MessageManager,
    role::RoleManager,
    scheduled_event::ScheduledEventManager,
    soundboard::SoundboardSoundManager,
    stage_instance::StageInstanceManager,
    sticker::StickerManager,
    thread::{FetchedThreads, ThreadManager, ThreadMemberManager},
    user::UserManager,
    webhook::WebhookManager,
};

#[cfg(feature = "gateway")]
#[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
pub use self::gateway::{FetchMembers, GatewayRequestError, GuildSoundboardSounds};
use crate::Context;

#[must_use = "a fetch does nothing until it is awaited"]
pub struct Fetch<T, F> {
    cached: Option<T>,
    rest: F,
    force: bool,
}

impl<T, F> Fetch<T, F> {
    pub(crate) const fn new(cached: Option<T>, rest: F) -> Self {
        Self {
            cached,
            rest,
            force: false,
        }
    }

    pub fn force(mut self) -> Self {
        self.force = true;
        self
    }

    #[must_use]
    pub const fn is_cached(&self) -> bool {
        self.cached.is_some()
    }
}

impl<T, F> fmt::Debug for Fetch<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Fetch")
            .field("cached", &self.cached.is_some())
            .field("force", &self.force)
            .finish_non_exhaustive()
    }
}

impl<T: Unpin, F: Future<Output = Result<T, Error>>> IntoFuture for Fetch<T, F> {
    type Output = Result<T, Error>;
    type IntoFuture = FetchFuture<T, F>;

    fn into_future(self) -> FetchFuture<T, F> {
        match self.cached {
            Some(cached) if !self.force => FetchFuture(State::Cached(Some(cached))),
            _ => FetchFuture(State::Rest(Box::pin(self.rest))),
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct FetchFuture<T, F>(State<T, F>);

enum State<T, F> {
    Cached(Option<T>),
    Rest(Pin<Box<F>>),
}

impl<T, F> fmt::Debug for FetchFuture<T, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.0 {
            State::Cached(_) => "cached",
            State::Rest(_) => "requesting",
        };
        f.debug_tuple("FetchFuture").field(&state).finish()
    }
}

impl<T: Unpin, F: Future<Output = Result<T, Error>>> Future for FetchFuture<T, F> {
    type Output = Result<T, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Self::Output> {
        match &mut self.get_mut().0 {
            State::Cached(cached) => Poll::Ready(Ok(cached
                .take()
                .expect("`FetchFuture` polled after completion"))),
            State::Rest(rest) => rest.as_mut().poll(cx),
        }
    }
}

#[must_use = "paginators do nothing unless polled"]
pub struct Paginator<'a, B: Backend> {
    ctx: &'a Context<B>,
    channel_id: ChannelId,
    limit: u8,
    before: Option<MessageId>,
    done: bool,
    pending: Option<Pending<'a>>,
}

type Pending<'a> = Pin<Box<dyn Future<Output = Result<Vec<Message>, Error>> + Send + 'a>>;

impl<B: Backend> fmt::Debug for Paginator<'_, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Paginator")
            .field("channel_id", &self.channel_id)
            .field("limit", &self.limit)
            .field("before", &self.before)
            .field("done", &self.done)
            .finish_non_exhaustive()
    }
}

impl<'a, B: Backend> Paginator<'a, B> {
    pub(crate) fn new(
        ctx: &'a Context<B>,
        channel_id: ChannelId,
        query: GetChannelMessages,
    ) -> Self {
        Self {
            ctx,
            channel_id,
            limit: Self::effective_limit(&query),
            before: query.before,
            done: false,
            pending: None,
        }
    }

    pub(crate) fn effective_limit(query: &GetChannelMessages) -> u8 {
        query.limit.unwrap_or(50).clamp(1, 100)
    }

    pub(crate) fn next_before(ids: impl IntoIterator<Item = MessageId>) -> Option<MessageId> {
        ids.into_iter().min()
    }

    pub fn pages(self) -> Self {
        self
    }

    pub async fn fetch_all(mut self) -> Result<Collection<MessageId, Arc<CachedMessage>>, Error> {
        let mut all = Collection::new();
        while let Some(page) = self.next_collection().await? {
            all.extend(page);
            if self.done {
                break;
            }
        }
        Ok(all)
    }

    async fn next_collection(
        &mut self,
    ) -> Result<Option<Collection<MessageId, Arc<CachedMessage>>>, Error> {
        if self.done {
            return Ok(None);
        }
        let mut query = GetChannelMessages::default();
        query.before = self.before;
        query.limit = Some(self.limit);
        let messages = self
            .ctx
            .rest()
            .get_channel_messages(self.channel_id, &query)
            .await?;
        if messages.is_empty() {
            self.done = true;
            return Ok(None);
        }
        self.before = Self::next_before(messages.iter().map(|message| message.id));
        if messages.len() < usize::from(self.limit) {
            self.done = true;
        }
        Ok(Some(
            messages
                .iter()
                .map(|message| (message.id, self.ctx.store_message(message)))
                .collect(),
        ))
    }
}

impl<B: Backend> futures_core::Stream for Paginator<'_, B> {
    type Item = Result<Collection<MessageId, Arc<CachedMessage>>, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<Self::Item>> {
        let this = self.as_mut().get_mut();
        if this.done && this.pending.is_none() {
            return Poll::Ready(None);
        }
        if this.pending.is_none() {
            let ctx = this.ctx;
            let channel_id = this.channel_id;
            let before = this.before;
            let limit = this.limit;
            let future = async move {
                let mut query = GetChannelMessages::default();
                query.before = before;
                query.limit = Some(limit);
                ctx.rest().get_channel_messages(channel_id, &query).await
            };
            this.pending = Some(Box::pin(future));
        }
        let output = match this.pending.as_mut() {
            Some(pending) => match pending.as_mut().poll(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(output) => output,
            },
            None => return Poll::Ready(None),
        };
        this.pending = None;
        match output {
            Err(error) => {
                this.done = true;
                Poll::Ready(Some(Err(error)))
            }
            Ok(messages) => {
                if messages.is_empty() {
                    this.done = true;
                    Poll::Ready(None)
                } else {
                    this.before = Self::next_before(messages.iter().map(|message| message.id));
                    if messages.len() < usize::from(this.limit) {
                        this.done = true;
                    }
                    let ctx = this.ctx;
                    let page: Collection<MessageId, Arc<CachedMessage>> = messages
                        .iter()
                        .map(|message| (message.id, ctx.store_message(message)))
                        .collect();
                    Poll::Ready(Some(Ok(page)))
                }
            }
        }
    }
}

impl<B: Backend> Context<B> {
    pub(crate) fn store_guild(&self, guild: &Guild) -> Arc<CachedGuild> {
        self.ops().store_guild(guild)
    }

    pub(crate) fn store_channel(&self, channel: &Channel) -> Arc<Channel> {
        self.ops().store_channel(channel)
    }

    pub(crate) fn store_user(&self, user: &User) -> Arc<CachedUser> {
        self.ops().store_user(user)
    }

    pub(crate) fn store_member(&self, guild_id: GuildId, member: &Member) -> Arc<CachedMember> {
        self.ops().store_member(guild_id, member)
    }

    pub(crate) fn store_message(&self, message: &Message) -> Arc<CachedMessage> {
        self.ops().store_message(message)
    }

    pub(crate) fn store_role(&self, guild_id: GuildId, role: &Role) -> Arc<Role> {
        self.ops().store_role(guild_id, role)
    }

    pub(crate) fn store_emoji(&self, guild_id: GuildId, emoji: &Emoji) -> Arc<Emoji> {
        self.ops().store_emoji(guild_id, emoji)
    }

    pub(crate) fn store_sticker(&self, guild_id: GuildId, sticker: &Sticker) -> Arc<Sticker> {
        self.ops().store_sticker(guild_id, sticker)
    }

    pub(crate) fn store_scheduled_event(
        &self,
        guild_id: GuildId,
        event: &GuildScheduledEvent,
    ) -> Arc<GuildScheduledEvent> {
        self.ops().store_scheduled_event(guild_id, event)
    }

    pub(crate) fn store_stage_instance(
        &self,
        guild_id: GuildId,
        stage: &StageInstance,
    ) -> Arc<StageInstance> {
        self.ops().store_stage_instance(guild_id, stage)
    }

    pub(crate) fn store_soundboard_sound(
        &self,
        guild_id: GuildId,
        sound: &SoundboardSound,
    ) -> Arc<SoundboardSound> {
        self.ops().store_soundboard_sound(guild_id, sound)
    }

    #[must_use]
    pub const fn guilds(&self) -> GuildManager<'_, B> {
        GuildManager::new(self)
    }

    #[must_use]
    pub const fn channels(&self) -> ChannelManager<'_, B> {
        ChannelManager::new(self)
    }

    #[must_use]
    pub const fn guild_channels(&self, guild_id: GuildId) -> GuildChannelManager<'_, B> {
        GuildChannelManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn users(&self) -> UserManager<'_, B> {
        UserManager::new(self)
    }

    #[must_use]
    pub const fn members(&self, guild_id: GuildId) -> MemberManager<'_, B> {
        MemberManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn messages(&self, channel_id: ChannelId) -> MessageManager<'_, B> {
        MessageManager::new(self, channel_id)
    }

    #[must_use]
    pub const fn roles(&self, guild_id: GuildId) -> RoleManager<'_, B> {
        RoleManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn bans(&self, guild_id: GuildId) -> BanManager<'_, B> {
        BanManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn emojis(&self, guild_id: GuildId) -> EmojiManager<'_, B> {
        EmojiManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn stickers(&self, guild_id: GuildId) -> StickerManager<'_, B> {
        StickerManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn scheduled_events(&self, guild_id: GuildId) -> ScheduledEventManager<'_, B> {
        ScheduledEventManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn stage_instances(&self, guild_id: GuildId) -> StageInstanceManager<'_, B> {
        StageInstanceManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn soundboard_sounds(&self, guild_id: GuildId) -> SoundboardSoundManager<'_, B> {
        SoundboardSoundManager::new(self, guild_id)
    }

    #[must_use]
    pub const fn threads(&self, channel_id: ChannelId) -> ThreadManager<'_, B> {
        ThreadManager::new(self, channel_id)
    }

    #[must_use]
    pub const fn thread_members(&self, thread_id: ChannelId) -> ThreadMemberManager<'_, B> {
        ThreadMemberManager::new(self, thread_id)
    }

    pub async fn default_soundboard_sounds(
        &self,
    ) -> Result<Collection<SoundboardSoundId, SoundboardSound>, Error> {
        let sounds = self.rest().list_default_soundboard_sounds().await?;
        Ok(sounds
            .into_iter()
            .map(|sound| (sound.sound_id, sound))
            .collect())
    }

    #[must_use]
    #[track_caller]
    pub fn commands(&self) -> CommandManager<'_, B> {
        CommandManager::new(self, self.expect_application_id())
    }

    #[must_use]
    #[track_caller]
    pub fn guild_commands(&self, guild_id: GuildId) -> GuildCommandManager<'_, B> {
        GuildCommandManager::new(self, self.expect_application_id(), guild_id)
    }

    #[must_use]
    #[track_caller]
    pub fn application_emojis(&self) -> ApplicationEmojiManager<'_, B> {
        ApplicationEmojiManager::new(self, self.expect_application_id())
    }

    #[must_use]
    pub const fn auto_moderation_rules(
        &self,
        guild_id: GuildId,
    ) -> AutoModerationRuleManager<'_, B> {
        AutoModerationRuleManager::new(self, guild_id)
    }

    pub async fn sticker(&self, sticker_id: StickerId) -> Result<Sticker, Error> {
        self.rest().get_sticker(sticker_id).await
    }

    pub async fn sticker_packs(&self) -> Result<Collection<StickerPackId, StickerPack>, Error> {
        let packs = self.rest().list_sticker_packs().await?;
        Ok(packs.into_iter().map(|pack| (pack.id, pack)).collect())
    }

    pub async fn sticker_pack(&self, pack_id: StickerPackId) -> Result<StickerPack, Error> {
        self.rest().get_sticker_pack(pack_id).await
    }

    #[must_use]
    pub const fn invites(&self) -> InviteManager<'_, B> {
        InviteManager::new(self)
    }

    #[must_use]
    pub const fn webhooks(&self) -> WebhookManager<'_, B> {
        WebhookManager::new(self)
    }
}

#[cfg(test)]
mod tests {
    use rustydiscord_cache::DashMapBackend;
    use rustydiscord_model::id::MessageId;
    use rustydiscord_model::rest::GetChannelMessages;

    use super::Paginator;

    #[test]
    fn effective_limit_is_capped() {
        let query = GetChannelMessages::default();
        assert_eq!(Paginator::<DashMapBackend>::effective_limit(&query), 50);
        let mut query = GetChannelMessages::default();
        query.limit = Some(200);
        assert_eq!(Paginator::<DashMapBackend>::effective_limit(&query), 100);
        let mut query = GetChannelMessages::default();
        query.limit = Some(0);
        assert_eq!(Paginator::<DashMapBackend>::effective_limit(&query), 1);
        let mut query = GetChannelMessages::default();
        query.limit = Some(25);
        assert_eq!(Paginator::<DashMapBackend>::effective_limit(&query), 25);
    }

    #[test]
    fn next_before_uses_smallest_id() {
        let ids = [MessageId::new(9), MessageId::new(3), MessageId::new(7)];
        assert_eq!(
            Paginator::<DashMapBackend>::next_before(ids),
            Some(MessageId::new(3))
        );
        assert_eq!(
            Paginator::<DashMapBackend>::next_before(Vec::<MessageId>::new()),
            None
        );
    }
}
