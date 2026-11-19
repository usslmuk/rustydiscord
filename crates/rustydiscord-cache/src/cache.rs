use core::{fmt, hash::Hash, time::Duration};
use std::sync::{Arc, PoisonError, RwLock};

use dashmap::DashMap;
use rustydiscord_model::{
    channel::Channel,
    emoji::{Emoji, PartialEmoji},
    gateway::{
        DispatchEvent, Presence, Status,
        payload::{AvailableGuild, GuildMemberUpdate, ThreadMembersUpdate},
    },
    guild::{Guild, Member, PartialMember, Role},
    guild_scheduled_event::GuildScheduledEvent,
    id::{
        ChannelId, EmojiId, GuildId, MessageId, RoleId, ScheduledEventId, SoundboardSoundId,
        StageInstanceId, StickerId, UserId,
    },
    interaction::Interaction,
    message::{Message, Reaction},
    soundboard::SoundboardSound,
    stage_instance::StageInstance,
    sticker::Sticker,
    user::{CurrentUser, User},
    util::Timestamp,
    voice::VoiceState,
};

use crate::{
    Backend, CacheConfig, CacheView, CachedGuild, CachedMember, CachedMessage, CachedPresence,
    CachedUser, ChannelView, DashMapBackend, GuildView, Previous, Resource, Store, UpdateCache,
    config::Scope,
    dashmap_store::Hasher,
    limits::{ChannelScopedOf, Global, GlobalOf, Index, Scoped, ScopedOf},
};

pub struct Cache<B: Backend = DashMapBackend> {
    config: CacheConfig,
    backend: B,
    current_user: RwLock<Option<Arc<CurrentUser>>>,
    unavailable_guilds: DashMap<GuildId, (), Hasher>,
    guilds: Option<GlobalOf<GuildId, CachedGuild, B>>,
    channels: Option<GlobalOf<ChannelId, Channel, B>>,

    guild_channels: Index<GuildId, ChannelId>,
    roles: Option<ScopedOf<RoleId, Role, B>>,
    emojis: Option<ScopedOf<EmojiId, Emoji, B>>,
    stickers: Option<ScopedOf<StickerId, Sticker, B>>,
    members: Option<ScopedOf<UserId, CachedMember, B>>,
    users: Option<GlobalOf<UserId, CachedUser, B>>,
    threads: Option<ScopedOf<ChannelId, Channel, B>>,
    messages: Option<ChannelScopedOf<MessageId, CachedMessage, B>>,

    message_channels: Index<GuildId, ChannelId>,
    presences: Option<ScopedOf<UserId, CachedPresence, B>>,
    voice_states: Option<ScopedOf<UserId, VoiceState, B>>,
    scheduled_events: Option<ScopedOf<ScheduledEventId, GuildScheduledEvent, B>>,
    stage_instances: Option<ScopedOf<StageInstanceId, StageInstance, B>>,
    soundboard_sounds: Option<ScopedOf<SoundboardSoundId, SoundboardSound, B>>,
}

const _: () = {
    assert!(matches!(Resource::Guild.scope(), Scope::Cache));
    assert!(matches!(Resource::Channel.scope(), Scope::Cache));
    assert!(matches!(Resource::User.scope(), Scope::Cache));
    assert!(matches!(Resource::Role.scope(), Scope::Guild));
    assert!(matches!(Resource::Emoji.scope(), Scope::Guild));
    assert!(matches!(Resource::Sticker.scope(), Scope::Guild));
    assert!(matches!(Resource::Member.scope(), Scope::Guild));
    assert!(matches!(Resource::Thread.scope(), Scope::Guild));
    assert!(matches!(Resource::Message.scope(), Scope::Channel));
    assert!(matches!(Resource::Presence.scope(), Scope::Guild));
    assert!(matches!(Resource::VoiceState.scope(), Scope::Guild));
    assert!(matches!(Resource::ScheduledEvent.scope(), Scope::Guild));
    assert!(matches!(Resource::StageInstance.scope(), Scope::Guild));
    assert!(matches!(Resource::SoundboardSound.scope(), Scope::Guild));
};

impl Cache {
    #[must_use]
    pub fn new() -> Self {
        Self::with_config(CacheConfig::default())
    }

    #[must_use]
    pub fn with_config(config: CacheConfig) -> Self {
        Self::with_backend(config, DashMapBackend::new())
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}

impl<B: Backend> Cache<B> {
    #[must_use]
    pub fn with_backend(config: CacheConfig, backend: B) -> Self {
        Self {
            current_user: RwLock::new(None),
            unavailable_guilds: DashMap::with_hasher(Hasher::default()),
            guilds: global(&backend, &config, Resource::Guild),
            channels: global(&backend, &config, Resource::Channel),
            guild_channels: Index::new(),
            roles: scoped(&backend, &config, Resource::Role),
            emojis: scoped(&backend, &config, Resource::Emoji),
            stickers: scoped(&backend, &config, Resource::Sticker),
            members: scoped(&backend, &config, Resource::Member),
            users: global(&backend, &config, Resource::User),
            threads: scoped(&backend, &config, Resource::Thread),
            messages: channel_scoped(&backend, &config),
            message_channels: Index::new(),
            presences: scoped(&backend, &config, Resource::Presence),
            voice_states: scoped(&backend, &config, Resource::VoiceState),
            scheduled_events: scoped(&backend, &config, Resource::ScheduledEvent),
            stage_instances: scoped(&backend, &config, Resource::StageInstance),
            soundboard_sounds: scoped(&backend, &config, Resource::SoundboardSound),
            config,
            backend,
        }
    }

    #[must_use]
    pub const fn config(&self) -> &CacheConfig {
        &self.config
    }

    #[must_use]
    pub const fn backend(&self) -> &B {
        &self.backend
    }

    pub fn update<E: UpdateCache>(&self, event: &E) -> E::Previous {
        event.update_cache(self)
    }

    pub fn update_dispatch(&self, event: &DispatchEvent) -> Previous {
        crate::update::dispatch(self, event)
    }

    #[must_use]
    pub fn current_user(&self) -> Option<Arc<CurrentUser>> {
        self.current_user
            .read()
            .unwrap_or_else(PoisonError::into_inner)
            .clone()
    }

    #[must_use]
    pub fn unavailable_guilds(&self) -> Vec<GuildId> {
        self.unavailable_guilds
            .iter()
            .map(|entry| *entry.key())
            .collect()
    }

    #[must_use]
    pub fn guilds(&self) -> CacheView<'_, GuildId, CachedGuild, B> {
        CacheView::new(self.guilds.as_ref(), None)
    }

    #[must_use]
    pub fn channels(&self) -> CacheView<'_, ChannelId, Channel, B> {
        CacheView::new(self.channels.as_ref(), Some(&self.guild_channels))
    }

    #[must_use]
    pub fn users(&self) -> CacheView<'_, UserId, CachedUser, B> {
        CacheView::new(self.users.as_ref(), None)
    }

    #[must_use]
    pub fn roles(&self, guild_id: GuildId) -> GuildView<'_, RoleId, Role, B> {
        GuildView::new(self.roles.as_ref(), guild_id)
    }

    #[must_use]
    pub fn emojis(&self, guild_id: GuildId) -> GuildView<'_, EmojiId, Emoji, B> {
        GuildView::new(self.emojis.as_ref(), guild_id)
    }

    #[must_use]
    pub fn stickers(&self, guild_id: GuildId) -> GuildView<'_, StickerId, Sticker, B> {
        GuildView::new(self.stickers.as_ref(), guild_id)
    }

    #[must_use]
    pub fn members(&self, guild_id: GuildId) -> GuildView<'_, UserId, CachedMember, B> {
        GuildView::new(self.members.as_ref(), guild_id)
    }

    #[must_use]
    pub fn threads(&self, guild_id: GuildId) -> GuildView<'_, ChannelId, Channel, B> {
        GuildView::new(self.threads.as_ref(), guild_id)
    }

    #[must_use]
    pub fn messages(&self, channel_id: ChannelId) -> ChannelView<'_, MessageId, CachedMessage, B> {
        ChannelView::new(self.messages.as_ref(), channel_id)
    }

    #[must_use]
    pub fn presences(&self, guild_id: GuildId) -> GuildView<'_, UserId, CachedPresence, B> {
        GuildView::new(self.presences.as_ref(), guild_id)
    }

    #[must_use]
    pub fn voice_states(&self, guild_id: GuildId) -> GuildView<'_, UserId, VoiceState, B> {
        GuildView::new(self.voice_states.as_ref(), guild_id)
    }

    #[must_use]
    pub fn scheduled_events(
        &self,
        guild_id: GuildId,
    ) -> GuildView<'_, ScheduledEventId, GuildScheduledEvent, B> {
        GuildView::new(self.scheduled_events.as_ref(), guild_id)
    }

    #[must_use]
    pub fn stage_instances(
        &self,
        guild_id: GuildId,
    ) -> GuildView<'_, StageInstanceId, StageInstance, B> {
        GuildView::new(self.stage_instances.as_ref(), guild_id)
    }

    #[must_use]
    pub fn soundboard_sounds(
        &self,
        guild_id: GuildId,
    ) -> GuildView<'_, SoundboardSoundId, SoundboardSound, B> {
        GuildView::new(self.soundboard_sounds.as_ref(), guild_id)
    }
}

impl<B: Backend> Cache<B> {
    pub fn sweep_guilds(&self, mut filter: impl FnMut(&CachedGuild) -> bool) -> usize {
        let Some(guilds) = &self.guilds else { return 0 };
        let mut swept = Vec::new();
        let count = guilds.sweep(&mut |id, guild| {
            let sweep = filter(guild);
            if sweep {
                swept.push(*id);
            }
            sweep
        });
        for id in swept {
            self.unavailable_guilds.remove(&id);
            self.remove_guild_children(id);
        }
        count
    }

    pub fn sweep_channels(&self, mut filter: impl FnMut(&Channel) -> bool) -> usize {
        let Some(channels) = &self.channels else {
            return 0;
        };
        let mut swept = Vec::new();
        let count = channels.sweep(&mut |id, channel| {
            let sweep = filter(channel);
            if sweep {
                swept.push((*id, channel.guild_id()));
            }
            sweep
        });
        for (id, guild_id) in swept {
            if let Some(guild_id) = guild_id {
                self.guild_channels.remove(guild_id, id);
            }
            self.remove_messages_of(guild_id, id);
        }
        count
    }

    pub fn sweep_users(&self, mut filter: impl FnMut(&CachedUser) -> bool) -> usize {
        self.users
            .as_ref()
            .map_or(0, |users| users.sweep(&mut |_, user| filter(user)))
    }

    pub fn sweep_threads(&self, mut filter: impl FnMut(GuildId, &Channel) -> bool) -> usize {
        let Some(threads) = &self.threads else {
            return 0;
        };
        let mut swept = Vec::new();
        let count = threads.sweep(&mut |guild_id, thread| {
            let sweep = filter(guild_id, thread);
            if sweep {
                swept.push((guild_id, thread.id()));
            }
            sweep
        });
        for (guild_id, id) in swept {
            self.remove_messages_of(Some(guild_id), id);
        }
        count
    }

    pub fn sweep_archived_threads(&self, lifetime: Duration) -> usize {
        let cutoff = cutoff(lifetime);
        self.sweep_threads(|_, thread| {
            thread.as_thread().is_some_and(|thread| {
                thread.thread_metadata.archived
                    && thread.thread_metadata.archive_timestamp.as_unix_micros() < cutoff
            })
        })
    }

    pub fn sweep_messages(
        &self,
        mut filter: impl FnMut(ChannelId, &CachedMessage) -> bool,
    ) -> usize {
        self.messages.as_ref().map_or(0, |messages| {
            messages.sweep(&mut |channel_id, message| filter(channel_id, message))
        })
    }

    pub fn sweep_messages_older_than(&self, lifetime: Duration) -> usize {
        let cutoff = cutoff(lifetime);
        self.sweep_messages(|_, message| {
            message
                .edited_timestamp
                .unwrap_or(message.timestamp)
                .as_unix_micros()
                < cutoff
        })
    }
}

macro_rules! sweep_per_guild {
    ($($(#[$doc:meta])* $method:ident: $field:ident => $value:ty;)*) => {
        impl<B: Backend> Cache<B> {$(
            $(#[$doc])*
            pub fn $method(&self, mut filter: impl FnMut(GuildId, &$value) -> bool) -> usize {
                self.$field.as_ref().map_or(0, |entries| {
                    entries.sweep(&mut |guild_id, value| filter(guild_id, value))
                })
            }
        )*}
    };
}

sweep_per_guild! {

    sweep_roles: roles => Role;

    sweep_emojis: emojis => Emoji;

    sweep_stickers: stickers => Sticker;

    sweep_members: members => CachedMember;

    sweep_presences: presences => CachedPresence;

    sweep_voice_states: voice_states => VoiceState;

    sweep_scheduled_events: scheduled_events => GuildScheduledEvent;

    sweep_stage_instances: stage_instances => StageInstance;

    sweep_soundboard_sounds: soundboard_sounds => SoundboardSound;
}

fn cutoff(lifetime: Duration) -> i64 {
    let lifetime = i64::try_from(lifetime.as_micros()).unwrap_or(i64::MAX);
    Timestamp::now().as_unix_micros().saturating_sub(lifetime)
}

impl<B: Backend> Cache<B> {
    pub(crate) fn set_current_user(&self, user: &CurrentUser) -> Option<Arc<CurrentUser>> {
        self.current_user
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .replace(Arc::new(user.clone()))
    }

    pub(crate) fn mark_unavailable(&self, guild_id: GuildId) {
        self.unavailable_guilds.insert(guild_id, ());
    }

    pub(crate) fn insert_guild(&self, guild: &AvailableGuild) -> Option<Arc<CachedGuild>> {
        let id = guild.guild.id;
        self.unavailable_guilds.remove(&id);
        for channel in &guild.channels {
            self.insert_channel(channel, Some(id));
        }
        replace(self.roles.as_ref(), id, &guild.guild.roles, |role| role.id);
        replace(self.emojis.as_ref(), id, &guild.guild.emojis, |emoji| {
            emoji.id
        });
        replace(
            self.stickers.as_ref(),
            id,
            &guild.guild.stickers,
            |sticker| sticker.id,
        );
        for member in &guild.members {
            self.upsert_member(id, member);
        }
        if let Some(threads) = &self.threads {
            for thread in &guild.threads {
                let mut thread = thread.clone();
                set_guild_id(&mut thread, id);
                threads.insert(id, thread.id(), Arc::new(thread));
            }
        }
        for presence in &guild.presences {
            self.update_presence(id, presence);
        }
        if let Some(voice_states) = &self.voice_states {
            for state in guild
                .voice_states
                .iter()
                .filter(|state| state.channel_id.is_some())
            {
                voice_states.insert(id, state.user_id, Arc::new(stripped_voice_state(state, id)));
            }
        }
        replace(
            self.stage_instances.as_ref(),
            id,
            &guild.stage_instances,
            |stage| stage.id,
        );
        replace(
            self.scheduled_events.as_ref(),
            id,
            &guild.guild_scheduled_events,
            |event| event.id,
        );
        replace(
            self.soundboard_sounds.as_ref(),
            id,
            &guild.soundboard_sounds,
            |sound| sound.sound_id,
        );
        self.put_guild(Arc::new(CachedGuild::from_available(guild)))
    }

    pub(crate) fn update_guild(&self, guild: &Guild) -> Option<Arc<CachedGuild>> {
        if !guild.roles.is_empty() {
            replace(self.roles.as_ref(), guild.id, &guild.roles, |role| role.id);
        }
        if !guild.emojis.is_empty() {
            replace(self.emojis.as_ref(), guild.id, &guild.emojis, |emoji| {
                emoji.id
            });
        }
        if !guild.stickers.is_empty() {
            replace(
                self.stickers.as_ref(),
                guild.id,
                &guild.stickers,
                |sticker| sticker.id,
            );
        }
        let guilds = self.guilds.as_ref()?;
        let previous = guilds.get(&guild.id)?;
        let updated = CachedGuild::updated(guild, &previous);
        guilds.insert(guild.id, Arc::new(updated)).previous
    }

    pub(crate) fn remove_guild(&self, id: GuildId, unavailable: bool) -> Option<Arc<CachedGuild>> {
        if unavailable {
            self.mark_unavailable(id);
            return None;
        }
        self.unavailable_guilds.remove(&id);
        self.remove_guild_children(id);
        self.guilds.as_ref()?.remove(&id)
    }

    fn put_guild(&self, guild: Arc<CachedGuild>) -> Option<Arc<CachedGuild>> {
        let guilds = self.guilds.as_ref()?;
        let inserted = guilds.insert(guild.id, guild);
        if let Some((evicted, _)) = inserted.evicted {
            self.remove_guild_children(evicted);
        }
        inserted.previous
    }

    fn remove_guild_children(&self, id: GuildId) {
        if let Some(channels) = &self.channels {
            for channel in self.guild_channels.remove_parent(id) {
                channels.remove(&channel);
            }
        }
        remove_all(self.roles.as_ref(), id);
        remove_all(self.emojis.as_ref(), id);
        remove_all(self.stickers.as_ref(), id);
        remove_all(self.members.as_ref(), id);
        remove_all(self.threads.as_ref(), id);
        remove_all(self.presences.as_ref(), id);
        remove_all(self.voice_states.as_ref(), id);
        remove_all(self.scheduled_events.as_ref(), id);
        remove_all(self.stage_instances.as_ref(), id);
        remove_all(self.soundboard_sounds.as_ref(), id);
        if let Some(messages) = &self.messages {
            for channel in self.message_channels.remove_parent(id) {
                messages.remove_parent(channel);
            }
        }
    }

    pub(crate) fn count_member(&self, guild_id: GuildId, joined: bool) {
        let Some(guilds) = &self.guilds else { return };
        let Some(guild) = guilds.get(&guild_id) else {
            return;
        };
        let mut updated = CachedGuild::clone(&guild);
        updated.member_count = if joined {
            updated.member_count.saturating_add(1)
        } else {
            updated.member_count.saturating_sub(1)
        };
        guilds.insert(guild_id, Arc::new(updated));
    }

    pub(crate) fn insert_channel(
        &self,
        channel: &Channel,
        guild_id: Option<GuildId>,
    ) -> Option<Arc<Channel>> {
        if channel.as_thread().is_some() || self.channels.is_none() {
            return None;
        }
        let mut channel = channel.clone();
        if let Some(guild_id) = guild_id {
            set_guild_id(&mut channel, guild_id);
        }
        self.put_channel(Arc::new(channel))
    }

    fn put_channel(&self, channel: Arc<Channel>) -> Option<Arc<Channel>> {
        let channels = self.channels.as_ref()?;
        if let Some(guild_id) = channel.guild_id() {
            self.guild_channels.insert(guild_id, channel.id());
        }
        let inserted = channels.insert(channel.id(), channel);
        if let Some((id, evicted)) = &inserted.evicted
            && let Some(guild_id) = evicted.guild_id()
        {
            self.guild_channels.remove(guild_id, *id);
        }
        inserted.previous
    }

    pub(crate) fn remove_channel(&self, channel: &Channel) -> Option<Arc<Channel>> {
        self.remove_messages_of(channel.guild_id(), channel.id());
        let removed = self.channels.as_ref()?.remove(&channel.id());
        let guild_id = channel
            .guild_id()
            .or_else(|| removed.as_ref().and_then(|removed| removed.guild_id()));
        if let Some(guild_id) = guild_id {
            self.guild_channels.remove(guild_id, channel.id());
        }
        removed
    }

    pub(crate) fn update_pins(
        &self,
        channel_id: ChannelId,
        last_pin_timestamp: Option<Timestamp>,
    ) -> Option<Arc<Channel>> {
        let channels = self.channels.as_ref()?;
        let mut channel = Channel::clone(&*channels.get(&channel_id)?);
        if !set_last_pin_timestamp(&mut channel, last_pin_timestamp) {
            return None;
        }
        channels.insert(channel_id, Arc::new(channel)).previous
    }

    pub(crate) fn insert_role(&self, guild_id: GuildId, role: &Role) -> Option<Arc<Role>> {
        self.roles
            .as_ref()?
            .insert(guild_id, role.id, Arc::new(role.clone()))
            .previous
    }

    pub(crate) fn remove_role(&self, guild_id: GuildId, role_id: RoleId) -> Option<Arc<Role>> {
        self.roles.as_ref()?.remove(guild_id, role_id)
    }

    pub(crate) fn replace_emojis(&self, guild_id: GuildId, emojis: &[Emoji]) -> Vec<Arc<Emoji>> {
        replace(self.emojis.as_ref(), guild_id, emojis, |emoji| emoji.id)
    }

    pub(crate) fn replace_stickers(
        &self,
        guild_id: GuildId,
        stickers: &[Sticker],
    ) -> Vec<Arc<Sticker>> {
        replace(self.stickers.as_ref(), guild_id, stickers, |sticker| {
            sticker.id
        })
    }

    pub(crate) fn upsert_member(
        &self,
        guild_id: GuildId,
        member: &Member,
    ) -> Option<Arc<CachedMember>> {
        let Some(members) = &self.members else {
            self.cache_user(&member.user);
            return None;
        };
        let user = self.share_user(&member.user);
        let id = user.id;
        members
            .insert(
                guild_id,
                id,
                Arc::new(CachedMember::new(guild_id, member, user)),
            )
            .previous
    }

    pub(crate) fn update_member(&self, update: &GuildMemberUpdate) -> Option<Arc<CachedMember>> {
        let Some(members) = &self.members else {
            self.cache_user(&update.user);
            return None;
        };
        let user = self.share_user(&update.user);
        let id = user.id;
        let previous = members.get(update.guild_id, id);
        let updated = CachedMember::updated(update, user, previous.as_deref());
        members
            .insert(update.guild_id, id, Arc::new(updated))
            .previous
    }

    pub(crate) fn remove_member(
        &self,
        guild_id: GuildId,
        user: &User,
    ) -> Option<Arc<CachedMember>> {
        self.cache_user(user);
        self.members.as_ref()?.remove(guild_id, user.id)
    }

    pub(crate) fn insert_thread(&self, thread: &Channel) -> Option<Arc<Channel>> {
        let threads = self.threads.as_ref()?;
        let guild_id = thread.guild_id()?;
        threads
            .insert(guild_id, thread.id(), Arc::new(thread.clone()))
            .previous
    }

    pub(crate) fn update_thread_members(
        &self,
        update: &ThreadMembersUpdate,
    ) -> Option<Arc<Channel>> {
        for added in &update.added_members {
            if let Some(member) = &added.member.member {
                self.upsert_member(update.guild_id, member);
            }
        }
        let threads = self.threads.as_ref()?;
        let mut thread = Channel::clone(&*threads.get(update.guild_id, update.id)?);
        if !set_member_count(&mut thread, update.member_count) {
            return None;
        }
        threads
            .insert(update.guild_id, update.id, Arc::new(thread))
            .previous
    }

    pub(crate) fn remove_thread(&self, guild_id: GuildId, id: ChannelId) -> Option<Arc<Channel>> {
        self.remove_messages_of(Some(guild_id), id);
        self.threads.as_ref()?.remove(guild_id, id)
    }

    pub(crate) fn sync_threads(
        &self,
        guild_id: GuildId,
        parents: Option<&[ChannelId]>,
        synced: &[Channel],
    ) -> Vec<Arc<Channel>> {
        let Some(threads) = &self.threads else {
            return Vec::new();
        };
        let mut previous = Vec::new();
        for thread in threads.values_of(guild_id) {
            let in_sync = parents.is_none_or(|parents| {
                thread
                    .parent_id()
                    .is_some_and(|parent| parents.contains(&parent))
            });
            let active = thread
                .as_thread()
                .is_some_and(|thread| !thread.thread_metadata.archived);
            if in_sync && active {
                previous.extend(threads.remove(guild_id, thread.id()));
            }
        }
        for thread in synced {
            threads.insert(guild_id, thread.id(), Arc::new(thread.clone()));
        }
        previous
    }

    pub(crate) fn insert_message(&self, message: &Message) -> Option<Arc<CachedMessage>> {
        let author = if message.webhook_id.is_some() {
            Arc::new(CachedUser::new(&message.author))
        } else if self.messages.is_some() || self.members.is_some() {
            self.share_user(&message.author)
        } else {
            self.cache_user(&message.author);
            return None;
        };
        if let (Some(guild_id), Some(member), Some(members)) =
            (message.guild_id, &message.member, &self.members)
        {
            put_partial_member(members, guild_id, member, Arc::clone(&author));
        }
        let messages = self.messages.as_ref()?;
        if let Some(guild_id) = message.guild_id {
            self.message_channels.insert(guild_id, message.channel_id);
        }
        let cached = CachedMessage::new(message, author);
        messages
            .insert(message.channel_id, message.id, Arc::new(cached))
            .previous
    }

    pub(crate) fn update_message(&self, message: &Message) -> Option<Arc<CachedMessage>> {
        let cached = self
            .messages
            .as_ref()
            .is_some_and(|messages| messages.get(message.channel_id, message.id).is_some());
        if cached {
            self.insert_message(message)
        } else {
            if message.webhook_id.is_none() {
                self.cache_user(&message.author);
            }
            None
        }
    }

    pub(crate) fn remove_message(
        &self,
        channel_id: ChannelId,
        id: MessageId,
    ) -> Option<Arc<CachedMessage>> {
        self.messages.as_ref()?.remove(channel_id, id)
    }

    fn remove_messages_of(&self, guild_id: Option<GuildId>, channel_id: ChannelId) {
        if let Some(messages) = &self.messages {
            messages.remove_parent(channel_id);
            if let Some(guild_id) = guild_id {
                self.message_channels.remove(guild_id, channel_id);
            }
        }
    }

    pub(crate) fn react(
        &self,
        channel_id: ChannelId,
        message_id: MessageId,
        change: ReactionChange<'_>,
    ) -> Option<Arc<CachedMessage>> {
        let messages = self.messages.as_ref()?;
        let mut message = CachedMessage::clone(&*messages.get(channel_id, message_id)?);
        let me = self.current_user().map(|user| user.id);
        change.apply(&mut message.reactions, me);
        messages
            .insert(channel_id, message_id, Arc::new(message))
            .previous
    }

    pub(crate) fn apply_interaction(&self, interaction: &Interaction) -> Option<Arc<CachedMember>> {
        if let Some(resolved) = interaction.data.resolved() {
            for (id, user) in &resolved.users {
                match (interaction.guild_id, resolved.members.get(id)) {
                    (Some(guild_id), Some(member)) => {
                        self.upsert_partial_member(guild_id, &member.member, user, true);
                    }
                    _ => self.cache_user(user),
                }
            }
        }
        if let (Some(guild_id), Some(member)) = (interaction.guild_id, &interaction.member) {
            self.upsert_partial_member(guild_id, &member.member, member.user.as_ref()?, false)
        } else {
            if let Some(user) = &interaction.user {
                self.cache_user(user);
            }
            None
        }
    }

    fn upsert_partial_member(
        &self,
        guild_id: GuildId,
        member: &PartialMember,
        user: &User,
        keep_voice: bool,
    ) -> Option<Arc<CachedMember>> {
        let Some(members) = &self.members else {
            self.cache_user(user);
            return None;
        };
        let user = self.share_user(user);
        let id = user.id;
        let mut cached = CachedMember::from_partial(guild_id, member, user);
        if keep_voice && let Some(previous) = members.get(guild_id, id) {
            cached.deaf = previous.deaf;
            cached.mute = previous.mute;
        }
        members.insert(guild_id, id, Arc::new(cached)).previous
    }

    pub(crate) fn update_presence(
        &self,
        guild_id: GuildId,
        presence: &Presence,
    ) -> Option<Arc<CachedPresence>> {
        let presences = self.presences.as_ref()?;
        if presence.status == Status::Offline {
            return presences.remove(guild_id, presence.user.id);
        }
        presences
            .insert(
                guild_id,
                presence.user.id,
                Arc::new(CachedPresence::new(guild_id, presence)),
            )
            .previous
    }

    pub fn ensure_presence_member(
        &self,
        guild_id: GuildId,
        presence: &Presence,
    ) -> Option<Arc<CachedMember>> {
        if presence.status == Status::Offline {
            return None;
        }
        let members = self.members.as_ref()?;
        let id = presence.user.id;
        if members.get(guild_id, id).is_some() {
            return None;
        }
        self.guilds.as_ref()?.get(&guild_id)?;
        let user = self.users.as_ref()?.get(&id)?;
        let member = Arc::new(CachedMember::from_partial(
            guild_id,
            &PartialMember::minimal(),
            user,
        ));
        members.insert(guild_id, id, Arc::clone(&member));
        Some(member)
    }

    pub(crate) fn update_voice_state(&self, state: &VoiceState) -> Option<Arc<VoiceState>> {
        let guild_id = state.guild_id?;
        if let Some(member) = &state.member {
            self.upsert_member(guild_id, member);
        }
        let voice_states = self.voice_states.as_ref()?;
        if state.channel_id.is_none() {
            return voice_states.remove(guild_id, state.user_id);
        }
        voice_states
            .insert(
                guild_id,
                state.user_id,
                Arc::new(stripped_voice_state(state, guild_id)),
            )
            .previous
    }

    pub(crate) fn put_scheduled_event(
        &self,
        event: &GuildScheduledEvent,
    ) -> Option<Arc<GuildScheduledEvent>> {
        put(
            self.scheduled_events.as_ref(),
            event.guild_id,
            event.id,
            event,
        )
    }

    pub(crate) fn remove_scheduled_event(
        &self,
        event: &GuildScheduledEvent,
    ) -> Option<Arc<GuildScheduledEvent>> {
        self.scheduled_events
            .as_ref()?
            .remove(event.guild_id, event.id)
    }

    pub(crate) fn put_stage_instance(&self, stage: &StageInstance) -> Option<Arc<StageInstance>> {
        put(
            self.stage_instances.as_ref(),
            stage.guild_id,
            stage.id,
            stage,
        )
    }

    pub(crate) fn remove_stage_instance(
        &self,
        stage: &StageInstance,
    ) -> Option<Arc<StageInstance>> {
        self.stage_instances
            .as_ref()?
            .remove(stage.guild_id, stage.id)
    }

    pub(crate) fn put_soundboard_sound(
        &self,
        guild_id: GuildId,
        sound: &SoundboardSound,
    ) -> Option<Arc<SoundboardSound>> {
        put(
            self.soundboard_sounds.as_ref(),
            guild_id,
            sound.sound_id,
            sound,
        )
    }

    pub(crate) fn remove_soundboard_sound(
        &self,
        guild_id: GuildId,
        id: SoundboardSoundId,
    ) -> Option<Arc<SoundboardSound>> {
        self.soundboard_sounds.as_ref()?.remove(guild_id, id)
    }

    pub(crate) fn replace_soundboard_sounds(
        &self,
        guild_id: GuildId,
        sounds: &[SoundboardSound],
    ) -> Vec<Arc<SoundboardSound>> {
        replace(self.soundboard_sounds.as_ref(), guild_id, sounds, |sound| {
            sound.sound_id
        })
    }

    fn cache_user(&self, user: &User) {
        if self.users.is_some() {
            self.share_user(user);
        }
    }

    fn share_user(&self, user: &User) -> Arc<CachedUser> {
        let Some(users) = &self.users else {
            return Arc::new(CachedUser::new(user));
        };
        if let Some(cached) = users.get(&user.id)
            && cached.is(user)
        {
            return cached;
        }
        let shared = Arc::new(CachedUser::new(user));
        users.insert(user.id, Arc::clone(&shared));
        shared
    }
}

impl<B: Backend> Cache<B> {
    pub fn store_user(&self, user: &User) -> Arc<CachedUser> {
        self.share_user(user)
    }

    pub fn store_member(&self, guild_id: GuildId, member: &Member) -> Arc<CachedMember> {
        let user = self.share_user(&member.user);
        let id = user.id;
        let cached = Arc::new(CachedMember::new(guild_id, member, user));
        if let Some(members) = &self.members {
            members.insert(guild_id, id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_message(&self, message: &Message) -> Arc<CachedMessage> {
        let author = if message.webhook_id.is_some() {
            Arc::new(CachedUser::new(&message.author))
        } else {
            self.share_user(&message.author)
        };
        if let (Some(guild_id), Some(member), Some(members)) =
            (message.guild_id, &message.member, &self.members)
        {
            put_partial_member(members, guild_id, member, Arc::clone(&author));
        }
        let cached = Arc::new(CachedMessage::new(message, author));
        if let Some(messages) = &self.messages {
            if let Some(guild_id) = message.guild_id {
                self.message_channels.insert(guild_id, message.channel_id);
            }
            messages.insert(message.channel_id, message.id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_channel(&self, channel: &Channel) -> Arc<Channel> {
        let cached = Arc::new(channel.clone());
        if cached.as_thread().is_some() {
            if let (Some(threads), Some(guild_id)) = (&self.threads, cached.guild_id()) {
                threads.insert(guild_id, cached.id(), Arc::clone(&cached));
            }
        } else {
            self.put_channel(Arc::clone(&cached));
        }
        cached
    }

    pub fn store_role(&self, guild_id: GuildId, role: &Role) -> Arc<Role> {
        let cached = Arc::new(role.clone());
        if let Some(roles) = &self.roles {
            roles.insert(guild_id, role.id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_emoji(&self, guild_id: GuildId, emoji: &Emoji) -> Arc<Emoji> {
        let cached = Arc::new(emoji.clone());
        if let Some(emojis) = &self.emojis {
            emojis.insert(guild_id, emoji.id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_sticker(&self, guild_id: GuildId, sticker: &Sticker) -> Arc<Sticker> {
        let cached = Arc::new(sticker.clone());
        if let Some(stickers) = &self.stickers {
            stickers.insert(guild_id, sticker.id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_scheduled_event(
        &self,
        guild_id: GuildId,
        event: &GuildScheduledEvent,
    ) -> Arc<GuildScheduledEvent> {
        let cached = Arc::new(event.clone());
        if let Some(events) = &self.scheduled_events {
            events.insert(guild_id, event.id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_stage_instance(
        &self,
        guild_id: GuildId,
        stage: &StageInstance,
    ) -> Arc<StageInstance> {
        let cached = Arc::new(stage.clone());
        if let Some(stages) = &self.stage_instances {
            stages.insert(guild_id, stage.id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_soundboard_sound(
        &self,
        guild_id: GuildId,
        sound: &SoundboardSound,
    ) -> Arc<SoundboardSound> {
        let cached = Arc::new(sound.clone());
        if let Some(sounds) = &self.soundboard_sounds {
            sounds.insert(guild_id, sound.sound_id, Arc::clone(&cached));
        }
        cached
    }

    pub fn store_guild(&self, guild: &Guild) -> Arc<CachedGuild> {
        let previous = self
            .guilds
            .as_ref()
            .and_then(|guilds| guilds.get(&guild.id));
        let cached = match previous {
            Some(previous) => CachedGuild::updated(guild, &previous),
            None => CachedGuild::new(
                guild,
                None,
                false,
                guild.approximate_member_count.unwrap_or(0),
            ),
        };
        let cached = Arc::new(cached);
        if self.guilds.is_some() {
            if !guild.roles.is_empty() {
                replace(self.roles.as_ref(), guild.id, &guild.roles, |role| role.id);
            }
            if !guild.emojis.is_empty() {
                replace(self.emojis.as_ref(), guild.id, &guild.emojis, |emoji| {
                    emoji.id
                });
            }
            if !guild.stickers.is_empty() {
                replace(
                    self.stickers.as_ref(),
                    guild.id,
                    &guild.stickers,
                    |sticker| sticker.id,
                );
            }
            self.put_guild(Arc::clone(&cached));
        }
        cached
    }
}

impl<B: Backend> fmt::Debug for Cache<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cache")
            .field("config", &self.config)
            .field("backend", &self.backend)
            .field("guilds", &self.guilds().len())
            .field("channels", &self.channels().len())
            .field("users", &self.users().len())
            .field("members", &self.members.as_ref().map_or(0, Scoped::len))
            .field("messages", &self.messages.as_ref().map_or(0, Scoped::len))
            .finish_non_exhaustive()
    }
}

fn global<K, V, B>(
    backend: &B,
    config: &CacheConfig,
    resource: Resource,
) -> Option<GlobalOf<K, V, B>>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    let limit = config.limit(resource);
    limit
        .is_enabled()
        .then(|| Global::new(backend.store(resource, limit), limit))
}

fn channel_scoped<B: Backend>(
    backend: &B,
    config: &CacheConfig,
) -> Option<ChannelScopedOf<MessageId, CachedMessage, B>> {
    let limit = config.limit(Resource::Message);
    limit
        .is_enabled()
        .then(|| Scoped::new(backend.store(Resource::Message, limit), limit))
}

fn put<K, V, S>(
    entries: Option<&Scoped<GuildId, K, V, S>>,
    guild_id: GuildId,
    id: K,
    value: &V,
) -> Option<Arc<V>>
where
    K: Eq + Hash + Copy,
    V: Clone,
    S: Store<(GuildId, K), V>,
{
    entries?
        .insert(guild_id, id, Arc::new(value.clone()))
        .previous
}

fn put_partial_member<S>(
    members: &Scoped<GuildId, UserId, CachedMember, S>,
    guild_id: GuildId,
    member: &PartialMember,
    user: Arc<CachedUser>,
) where
    S: Store<(GuildId, UserId), CachedMember>,
{
    let id = user.id;
    let cached = CachedMember::from_partial(guild_id, member, user);
    members.insert(guild_id, id, Arc::new(cached));
}

fn stripped_voice_state(state: &VoiceState, guild_id: GuildId) -> VoiceState {
    let mut state = state.clone();
    state.guild_id = Some(guild_id);
    state.member = None;
    state
}

pub(crate) enum ReactionChange<'a> {
    Add {
        emoji: &'a PartialEmoji,
        user_id: UserId,
        burst: bool,
        burst_colors: &'a [compact_str::CompactString],
    },

    Remove {
        emoji: &'a PartialEmoji,
        user_id: UserId,
        burst: bool,
    },

    RemoveAll,

    RemoveEmoji(&'a PartialEmoji),
}

impl ReactionChange<'_> {
    fn apply(self, reactions: &mut Vec<Reaction>, me: Option<UserId>) {
        match self {
            Self::Add {
                emoji,
                user_id,
                burst,
                burst_colors,
            } => {
                let index = if let Some(index) = reactions
                    .iter()
                    .position(|reaction| same_emoji(&reaction.emoji, emoji))
                {
                    index
                } else {
                    reactions.push(Reaction::new(emoji.clone()));
                    reactions.len() - 1
                };
                let reaction = &mut reactions[index];
                let mine = me == Some(user_id);
                reaction.count += 1;
                if burst {
                    reaction.count_details.burst += 1;
                    reaction.me_burst |= mine;
                    if reaction.burst_colors.is_empty() {
                        reaction.burst_colors = burst_colors.to_vec();
                    }
                } else {
                    reaction.count_details.normal += 1;
                    reaction.me |= mine;
                }
            }
            Self::Remove {
                emoji,
                user_id,
                burst,
            } => {
                let Some(index) = reactions
                    .iter()
                    .position(|reaction| same_emoji(&reaction.emoji, emoji))
                else {
                    return;
                };
                let reaction = &mut reactions[index];
                let mine = me == Some(user_id);
                reaction.count = reaction.count.saturating_sub(1);
                if burst {
                    reaction.count_details.burst = reaction.count_details.burst.saturating_sub(1);
                    reaction.me_burst &= !mine;
                } else {
                    reaction.count_details.normal = reaction.count_details.normal.saturating_sub(1);
                    reaction.me &= !mine;
                }
                if reaction.count == 0 {
                    reactions.remove(index);
                }
            }
            Self::RemoveAll => reactions.clear(),
            Self::RemoveEmoji(emoji) => {
                reactions.retain(|reaction| !same_emoji(&reaction.emoji, emoji));
            }
        }
    }
}

fn same_emoji(a: &PartialEmoji, b: &PartialEmoji) -> bool {
    match (a.id(), b.id()) {
        (Some(a), Some(b)) => a == b,
        (None, None) => a == b,
        _ => false,
    }
}

fn scoped<K, V, B>(
    backend: &B,
    config: &CacheConfig,
    resource: Resource,
) -> Option<ScopedOf<K, V, B>>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    let limit = config.limit(resource);
    limit
        .is_enabled()
        .then(|| Scoped::new(backend.store(resource, limit), limit))
}

fn replace<K, V, S>(
    entries: Option<&Scoped<GuildId, K, V, S>>,
    guild_id: GuildId,
    items: &[V],
    id: impl Fn(&V) -> K,
) -> Vec<Arc<V>>
where
    K: Eq + Hash + Copy,
    V: Clone,
    S: Store<(GuildId, K), V>,
{
    let Some(entries) = entries else {
        return Vec::new();
    };
    let previous = entries.remove_parent(guild_id);
    for item in items {
        entries.insert(guild_id, id(item), Arc::new(item.clone()));
    }
    previous
}

fn remove_all<K, V, S>(entries: Option<&Scoped<GuildId, K, V, S>>, guild_id: GuildId)
where
    K: Eq + Hash + Copy,
    S: Store<(GuildId, K), V>,
{
    if let Some(entries) = entries {
        entries.remove_parent(guild_id);
    }
}

const fn set_guild_id(channel: &mut Channel, guild_id: GuildId) {
    let slot = match channel {
        Channel::Text(channel) | Channel::Announcement(channel) => &mut channel.guild_id,
        Channel::Voice(channel) | Channel::Stage(channel) => &mut channel.guild_id,
        Channel::Category(channel) => &mut channel.guild_id,
        Channel::AnnouncementThread(channel)
        | Channel::PublicThread(channel)
        | Channel::PrivateThread(channel) => &mut channel.guild_id,
        Channel::Directory(channel) => &mut channel.guild_id,
        Channel::Forum(channel) | Channel::Media(channel) => &mut channel.guild_id,
        Channel::Unknown(channel) => &mut channel.guild_id,
        _ => return,
    };
    *slot = Some(guild_id);
}

const fn set_member_count(channel: &mut Channel, count: u32) -> bool {
    match channel {
        Channel::AnnouncementThread(thread)
        | Channel::PublicThread(thread)
        | Channel::PrivateThread(thread) => {
            thread.member_count = Some(count);
            true
        }
        _ => false,
    }
}

const fn set_last_pin_timestamp(channel: &mut Channel, timestamp: Option<Timestamp>) -> bool {
    let slot = match channel {
        Channel::Text(channel) | Channel::Announcement(channel) => &mut channel.last_pin_timestamp,
        Channel::Dm(channel) => &mut channel.last_pin_timestamp,
        Channel::GroupDm(channel) => &mut channel.last_pin_timestamp,
        Channel::AnnouncementThread(channel)
        | Channel::PublicThread(channel)
        | Channel::PrivateThread(channel) => &mut channel.last_pin_timestamp,
        _ => return false,
    };
    *slot = timestamp;
    true
}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Cache>();
};
