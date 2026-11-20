use std::sync::Arc;

use crate::model::{
    channel::Channel,
    emoji::Emoji,
    gateway::{
        DispatchEvent,
        payload::{
            ChannelCreate, ChannelDelete, ChannelPinsUpdate, ChannelUpdate, GuildCreate,
            GuildDelete, GuildEmojisUpdate, GuildMemberAdd, GuildMemberRemove, GuildMemberUpdate,
            GuildMembersChunk, GuildRoleCreate, GuildRoleDelete, GuildRoleUpdate,
            GuildScheduledEventCreate, GuildScheduledEventDelete, GuildScheduledEventUpdate,
            GuildSoundboardSoundCreate, GuildSoundboardSoundDelete, GuildSoundboardSoundUpdate,
            GuildSoundboardSoundsUpdate, GuildStickersUpdate, GuildUpdate, InteractionCreate,
            MessageCreate, MessageDelete, MessageDeleteBulk, MessageReactionAdd,
            MessageReactionRemove, MessageReactionRemoveAll, MessageReactionRemoveEmoji,
            MessageUpdate, PresenceUpdate, Ready, SoundboardSounds, StageInstanceCreate,
            StageInstanceDelete, StageInstanceUpdate, ThreadCreate, ThreadDelete, ThreadListSync,
            ThreadMembersUpdate, ThreadUpdate, TypingStart, UserUpdate, VoiceStateUpdate,
        },
    },
    guild::Role,
    guild_scheduled_event::GuildScheduledEvent,
    soundboard::SoundboardSound,
    stage_instance::StageInstance,
    sticker::Sticker,
    user::CurrentUser,
    voice::VoiceState,
};

use crate::cache::{
    Backend, Cache, CachedGuild, CachedMember, CachedMessage, CachedPresence, state::ReactionChange,
};

mod sealed {
    pub trait Sealed {}
}

pub trait UpdateCache: sealed::Sealed {
    type Previous;

    #[doc(hidden)]
    fn update_cache<B: Backend>(&self, cache: &Cache<B>) -> Self::Previous;
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Previous {
    None,

    CurrentUser(Arc<CurrentUser>),

    Guild(Arc<CachedGuild>),

    Channel(Arc<Channel>),

    Role(Arc<Role>),

    Emojis(Vec<Arc<Emoji>>),

    Stickers(Vec<Arc<Sticker>>),

    Member(Arc<CachedMember>),

    Members(Vec<Arc<CachedMember>>),

    Message(Arc<CachedMessage>),

    Messages(Vec<Arc<CachedMessage>>),

    Presence(Arc<CachedPresence>),

    VoiceState(Arc<VoiceState>),

    Thread(Arc<Channel>),

    Threads(Vec<Arc<Channel>>),

    ScheduledEvent(Arc<GuildScheduledEvent>),

    StageInstance(Arc<StageInstance>),

    SoundboardSound(Arc<SoundboardSound>),

    SoundboardSounds(Vec<Arc<SoundboardSound>>),
}

macro_rules! update_cache {
    ($($payload:ty => $previous:ty, |$event:ident, $cache:ident| $body:expr;)*) => {$(
        impl sealed::Sealed for $payload {}

        impl UpdateCache for $payload {
            type Previous = $previous;

            fn update_cache<B: Backend>(&self, $cache: &Cache<B>) -> $previous {
                let $event = self;
                $body
            }
        }
    )*};
}

update_cache! {
    Ready => Option<Arc<CurrentUser>>, |event, cache| {
        for guild in &event.guilds {
            cache.mark_unavailable(guild.id);
        }
        cache.set_current_user(&event.user)
    };
    UserUpdate => Option<Arc<CurrentUser>>, |event, cache| cache.set_current_user(event);
    GuildCreate => Option<Arc<CachedGuild>>, |event, cache| match event {
        GuildCreate::Available(guild) => cache.insert_guild(guild),
        GuildCreate::Unavailable(guild) => {
            cache.mark_unavailable(guild.id);
            None
        }
    };
    GuildUpdate => Option<Arc<CachedGuild>>, |event, cache| cache.update_guild(event);
    GuildDelete => Option<Arc<CachedGuild>>, |event, cache| {
        cache.remove_guild(event.id, event.unavailable)
    };
    ChannelCreate => Option<Arc<Channel>>, |event, cache| cache.insert_channel(event, None);
    ChannelUpdate => Option<Arc<Channel>>, |event, cache| cache.insert_channel(event, None);
    ChannelDelete => Option<Arc<Channel>>, |event, cache| cache.remove_channel(event);
    ChannelPinsUpdate => Option<Arc<Channel>>, |event, cache| {
        cache.update_pins(event.channel_id, event.last_pin_timestamp)
    };
    GuildRoleCreate => Option<Arc<Role>>, |event, cache| {
        cache.insert_role(event.guild_id, &event.role)
    };
    GuildRoleUpdate => Option<Arc<Role>>, |event, cache| {
        cache.insert_role(event.guild_id, &event.role)
    };
    GuildRoleDelete => Option<Arc<Role>>, |event, cache| {
        cache.remove_role(event.guild_id, event.role_id)
    };
    GuildEmojisUpdate => Vec<Arc<Emoji>>, |event, cache| {
        cache.replace_emojis(event.guild_id, &event.emojis)
    };
    GuildStickersUpdate => Vec<Arc<Sticker>>, |event, cache| {
        cache.replace_stickers(event.guild_id, &event.stickers)
    };
    GuildMemberAdd => Option<Arc<CachedMember>>, |event, cache| {
        let previous = cache.upsert_member(event.guild_id, event);
        if previous.is_none() {
            cache.count_member(event.guild_id, true);
        }
        previous
    };
    GuildMemberUpdate => Option<Arc<CachedMember>>, |event, cache| cache.update_member(event);
    GuildMemberRemove => Option<Arc<CachedMember>>, |event, cache| {
        cache.count_member(event.guild_id, false);
        cache.remove_member(event.guild_id, &event.user)
    };
    GuildMembersChunk => Vec<Arc<CachedMember>>, |event, cache| {
        for presence in &event.presences {
            cache.update_presence(event.guild_id, presence);
        }
        event
            .members
            .iter()
            .filter_map(|member| cache.upsert_member(event.guild_id, member))
            .collect()
    };
    MessageCreate => Option<Arc<CachedMessage>>, |event, cache| cache.insert_message(event);
    MessageUpdate => Option<Arc<CachedMessage>>, |event, cache| cache.update_message(event);
    MessageDelete => Option<Arc<CachedMessage>>, |event, cache| {
        cache.remove_message(event.channel_id, event.id)
    };
    MessageDeleteBulk => Vec<Arc<CachedMessage>>, |event, cache| {
        event
            .ids
            .iter()
            .filter_map(|&id| cache.remove_message(event.channel_id, id))
            .collect()
    };
    MessageReactionAdd => Option<Arc<CachedMessage>>, |event, cache| {
        if let (Some(guild_id), Some(member)) = (event.guild_id, &event.member) {
            cache.upsert_member(guild_id, member);
        }
        cache.react(event.channel_id, event.message_id, ReactionChange::Add {
            emoji: &event.emoji,
            user_id: event.user_id,
            burst: event.burst,
            burst_colors: &event.burst_colors,
        })
    };
    MessageReactionRemove => Option<Arc<CachedMessage>>, |event, cache| {
        cache.react(event.channel_id, event.message_id, ReactionChange::Remove {
            emoji: &event.emoji,
            user_id: event.user_id,
            burst: event.burst,
        })
    };
    MessageReactionRemoveAll => Option<Arc<CachedMessage>>, |event, cache| {
        cache.react(event.channel_id, event.message_id, ReactionChange::RemoveAll)
    };
    MessageReactionRemoveEmoji => Option<Arc<CachedMessage>>, |event, cache| {
        cache.react(event.channel_id, event.message_id, ReactionChange::RemoveEmoji(&event.emoji))
    };
    TypingStart => Option<Arc<CachedMember>>, |event, cache| {
        cache.upsert_member(event.guild_id?, event.member.as_deref()?)
    };
    InteractionCreate => Option<Arc<CachedMember>>, |event, cache| cache.apply_interaction(event);
    PresenceUpdate => Option<Arc<CachedPresence>>, |event, cache| {
        cache.update_presence(event.guild_id?, event)
    };
    VoiceStateUpdate => Option<Arc<VoiceState>>, |event, cache| cache.update_voice_state(event);
    ThreadCreate => Option<Arc<Channel>>, |event, cache| cache.insert_thread(&event.thread);
    ThreadUpdate => Option<Arc<Channel>>, |event, cache| cache.insert_thread(event);
    ThreadDelete => Option<Arc<Channel>>, |event, cache| {
        cache.remove_thread(event.guild_id, event.id)
    };
    ThreadListSync => Vec<Arc<Channel>>, |event, cache| {
        cache.sync_threads(event.guild_id, event.channel_ids.as_deref(), &event.threads)
    };
    ThreadMembersUpdate => Option<Arc<Channel>>, |event, cache| cache.update_thread_members(event);
    GuildScheduledEventCreate => Option<Arc<GuildScheduledEvent>>, |event, cache| {
        cache.put_scheduled_event(event)
    };
    GuildScheduledEventUpdate => Option<Arc<GuildScheduledEvent>>, |event, cache| {
        cache.put_scheduled_event(event)
    };
    GuildScheduledEventDelete => Option<Arc<GuildScheduledEvent>>, |event, cache| {
        cache.remove_scheduled_event(event)
    };
    StageInstanceCreate => Option<Arc<StageInstance>>, |event, cache| {
        cache.put_stage_instance(event)
    };
    StageInstanceUpdate => Option<Arc<StageInstance>>, |event, cache| {
        cache.put_stage_instance(event)
    };
    StageInstanceDelete => Option<Arc<StageInstance>>, |event, cache| {
        cache.remove_stage_instance(event)
    };
    GuildSoundboardSoundCreate => Option<Arc<SoundboardSound>>, |event, cache| {
        cache.put_soundboard_sound(event.guild_id?, event)
    };
    GuildSoundboardSoundUpdate => Option<Arc<SoundboardSound>>, |event, cache| {
        cache.put_soundboard_sound(event.guild_id?, event)
    };
    GuildSoundboardSoundDelete => Option<Arc<SoundboardSound>>, |event, cache| {
        cache.remove_soundboard_sound(event.guild_id, event.sound_id)
    };
    GuildSoundboardSoundsUpdate => Vec<Arc<SoundboardSound>>, |event, cache| {
        event
            .soundboard_sounds
            .iter()
            .filter_map(|sound| cache.put_soundboard_sound(event.guild_id, sound))
            .collect()
    };
    SoundboardSounds => Vec<Arc<SoundboardSound>>, |event, cache| {
        cache.replace_soundboard_sounds(event.guild_id, &event.soundboard_sounds)
    };
}

pub(crate) fn dispatch<B: Backend>(cache: &Cache<B>, event: &DispatchEvent) -> Previous {
    match event {
        DispatchEvent::Ready(event) => cache.update(&**event).map(Previous::CurrentUser).into(),
        DispatchEvent::UserUpdate(event) => {
            cache.update(&**event).map(Previous::CurrentUser).into()
        }
        DispatchEvent::GuildCreate(event) => cache.update(&**event).map(Previous::Guild).into(),
        DispatchEvent::GuildUpdate(event) => cache.update(&**event).map(Previous::Guild).into(),
        DispatchEvent::GuildDelete(event) => cache.update(event).map(Previous::Guild).into(),
        DispatchEvent::ChannelCreate(event) => cache.update(&**event).map(Previous::Channel).into(),
        DispatchEvent::ChannelUpdate(event) => cache.update(&**event).map(Previous::Channel).into(),
        DispatchEvent::ChannelDelete(event) => cache.update(&**event).map(Previous::Channel).into(),
        DispatchEvent::ChannelPinsUpdate(event) => {
            cache.update(&**event).map(Previous::Channel).into()
        }
        DispatchEvent::GuildRoleCreate(event) => cache.update(&**event).map(Previous::Role).into(),
        DispatchEvent::GuildRoleUpdate(event) => cache.update(&**event).map(Previous::Role).into(),
        DispatchEvent::GuildRoleDelete(event) => cache.update(event).map(Previous::Role).into(),
        DispatchEvent::GuildEmojisUpdate(event) => Previous::Emojis(cache.update(&**event)),
        DispatchEvent::GuildStickersUpdate(event) => Previous::Stickers(cache.update(&**event)),
        DispatchEvent::GuildMemberAdd(event) => cache.update(&**event).map(Previous::Member).into(),
        DispatchEvent::GuildMemberUpdate(event) => {
            cache.update(&**event).map(Previous::Member).into()
        }
        DispatchEvent::GuildMemberRemove(event) => {
            cache.update(&**event).map(Previous::Member).into()
        }
        DispatchEvent::GuildMembersChunk(event) => Previous::Members(cache.update(&**event)),
        DispatchEvent::MessageCreate(event) => cache.update(&**event).map(Previous::Message).into(),
        DispatchEvent::MessageUpdate(event) => cache.update(&**event).map(Previous::Message).into(),
        DispatchEvent::MessageDelete(event) => cache.update(&**event).map(Previous::Message).into(),
        DispatchEvent::MessageDeleteBulk(event) => Previous::Messages(cache.update(&**event)),
        DispatchEvent::MessageReactionAdd(event) => {
            cache.update(&**event).map(Previous::Message).into()
        }
        DispatchEvent::MessageReactionRemove(event) => {
            cache.update(&**event).map(Previous::Message).into()
        }
        DispatchEvent::MessageReactionRemoveAll(event) => {
            cache.update(event).map(Previous::Message).into()
        }
        DispatchEvent::MessageReactionRemoveEmoji(event) => {
            cache.update(&**event).map(Previous::Message).into()
        }
        DispatchEvent::PresenceUpdate(event) => {
            cache.update(&**event).map(Previous::Presence).into()
        }
        DispatchEvent::VoiceStateUpdate(event) => {
            cache.update(&**event).map(Previous::VoiceState).into()
        }
        DispatchEvent::ThreadCreate(event) => cache.update(&**event).map(Previous::Thread).into(),
        DispatchEvent::ThreadUpdate(event) => cache.update(&**event).map(Previous::Thread).into(),
        DispatchEvent::ThreadDelete(event) => cache.update(&**event).map(Previous::Thread).into(),
        DispatchEvent::ThreadListSync(event) => Previous::Threads(cache.update(&**event)),
        DispatchEvent::ThreadMembersUpdate(event) => {
            cache.update(&**event).map(Previous::Thread).into()
        }
        DispatchEvent::TypingStart(event) => cache.update(&**event).map(Previous::Member).into(),
        DispatchEvent::InteractionCreate(event) => {
            cache.update(&**event).map(Previous::Member).into()
        }
        DispatchEvent::GuildScheduledEventCreate(event) => {
            cache.update(&**event).map(Previous::ScheduledEvent).into()
        }
        DispatchEvent::GuildScheduledEventUpdate(event) => {
            cache.update(&**event).map(Previous::ScheduledEvent).into()
        }
        DispatchEvent::GuildScheduledEventDelete(event) => {
            cache.update(&**event).map(Previous::ScheduledEvent).into()
        }
        DispatchEvent::StageInstanceCreate(event) => {
            cache.update(&**event).map(Previous::StageInstance).into()
        }
        DispatchEvent::StageInstanceUpdate(event) => {
            cache.update(&**event).map(Previous::StageInstance).into()
        }
        DispatchEvent::StageInstanceDelete(event) => {
            cache.update(&**event).map(Previous::StageInstance).into()
        }
        DispatchEvent::GuildSoundboardSoundCreate(event) => {
            cache.update(&**event).map(Previous::SoundboardSound).into()
        }
        DispatchEvent::GuildSoundboardSoundUpdate(event) => {
            cache.update(&**event).map(Previous::SoundboardSound).into()
        }
        DispatchEvent::GuildSoundboardSoundDelete(event) => {
            cache.update(event).map(Previous::SoundboardSound).into()
        }
        DispatchEvent::GuildSoundboardSoundsUpdate(event) => {
            Previous::SoundboardSounds(cache.update(&**event))
        }
        DispatchEvent::SoundboardSounds(event) => {
            Previous::SoundboardSounds(cache.update(&**event))
        }
        _ => Previous::None,
    }
}

impl From<Option<Self>> for Previous {
    fn from(previous: Option<Self>) -> Self {
        previous.unwrap_or(Self::None)
    }
}
