use core::ops::Deref;
use std::sync::Arc;

use crate::cache::{CachedGuild, CachedMember, CachedMessage, CachedPresence};
use crate::model::{
    channel::Channel, emoji::Emoji, gateway::payload as gateway, guild::Role,
    guild_scheduled_event::GuildScheduledEvent, id::GuildId, soundboard::SoundboardSound,
    stage_instance::StageInstance, sticker::Sticker, user::CurrentUser, voice::VoiceState,
};

macro_rules! with_old {
    ($(
        $(#[$doc:meta])*
        $name:ident($old:ty => $new:ident);
    )*) => {$(
        $(#[$doc])*

        #[doc = concat!(
            "Dereferences to `new`, a [`", stringify!($new), "`](crate::model::gateway::payload::",
            stringify!($new), ")."
        )]
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub struct $name {

            pub old: Option<Arc<$old>>,

            pub new: gateway::$new,
        }

        impl Deref for $name {
            type Target = gateway::$new;

            fn deref(&self) -> &gateway::$new {
                &self.new
            }
        }
    )*};
}

with_old! {

    ChannelUpdate(Channel => ChannelUpdate);

    ChannelDelete(Channel => ChannelDelete);

    ThreadUpdate(Channel => ThreadUpdate);


    ThreadDelete(Channel => ThreadDelete);

    GuildUpdate(CachedGuild => GuildUpdate);


    GuildDelete(CachedGuild => GuildDelete);



    GuildUnavailable(CachedGuild => GuildDelete);

    GuildMemberUpdate(CachedMember => GuildMemberUpdate);


    GuildMemberRemove(CachedMember => GuildMemberRemove);

    GuildRoleUpdate(Role => GuildRoleUpdate);


    GuildRoleDelete(Role => GuildRoleDelete);


    GuildScheduledEventUpdate(GuildScheduledEvent => GuildScheduledEventUpdate);


    GuildScheduledEventDelete(GuildScheduledEvent => GuildScheduledEventDelete);


    GuildSoundboardSoundUpdate(SoundboardSound => GuildSoundboardSoundUpdate);



    GuildSoundboardSoundDelete(SoundboardSound => GuildSoundboardSoundDelete);

    StageInstanceUpdate(StageInstance => StageInstanceUpdate);

    StageInstanceDelete(StageInstance => StageInstanceDelete);

    MessageUpdate(CachedMessage => MessageUpdate);


    MessageDelete(CachedMessage => MessageDelete);

    PresenceUpdate(CachedPresence => PresenceUpdate);

    UserUpdate(CurrentUser => UserUpdate);

    VoiceStateUpdate(VoiceState => VoiceStateUpdate);
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct GuildMemberAvailable {
    pub member: Arc<CachedMember>,
}

impl Deref for GuildMemberAvailable {
    type Target = CachedMember;

    fn deref(&self) -> &CachedMember {
        &self.member
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct MessageBulkDelete {
    pub old: Vec<Arc<CachedMessage>>,

    pub new: gateway::MessageDeleteBulk,
}

impl Deref for MessageBulkDelete {
    type Target = gateway::MessageDeleteBulk;

    fn deref(&self) -> &gateway::MessageDeleteBulk {
        &self.new
    }
}

macro_rules! derived {
    ($item:ident: $field:ident, $create:ident, $update:ident, $delete:ident, $list:literal) => {
        #[doc = concat!(
                    "The payload of [`Events::", stringify!($create), "`](crate::Events::",
                    stringify!($create), "): a new ", $list, ".\n\nDereferences to the [`",
                    stringify!($item), "`]."
                )]
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub struct $create {

            pub guild_id: GuildId,
            #[doc = concat!("The new ", $list, ".")]
            pub $field: $item,
        }

        impl Deref for $create {
            type Target = $item;

            fn deref(&self) -> &$item {
                &self.$field
            }
        }

        #[doc = concat!(
                    "The payload of [`Events::", stringify!($update), "`](crate::Events::",
                    stringify!($update), "): a changed ", $list, ", before and after.\n\n",
                    "Dereferences to `new`."
                )]
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub struct $update {

            pub guild_id: GuildId,
            #[doc = concat!("The ", $list, " as the cache held it.")]
            pub old: Arc<$item>,
            #[doc = concat!("The ", $list, " now.")]
            pub new: $item,
        }

        impl Deref for $update {
            type Target = $item;

            fn deref(&self) -> &$item {
                &self.new
            }
        }

        #[doc = concat!(
                    "The payload of [`Events::", stringify!($delete), "`](crate::Events::",
                    stringify!($delete), "): a deleted ", $list, ", as the cache held it.\n\n",
                    "Dereferences to the [`", stringify!($item), "`]."
                )]
        #[derive(Clone, Debug, PartialEq)]
        #[non_exhaustive]
        pub struct $delete {

            pub guild_id: GuildId,
            #[doc = concat!("The deleted ", $list, ".")]
            pub $field: Arc<$item>,
        }

        impl Deref for $delete {
            type Target = $item;

            fn deref(&self) -> &$item {
                &self.$field
            }
        }
    };
}

derived!(Emoji: emoji, GuildEmojiCreate, GuildEmojiUpdate, GuildEmojiDelete, "emoji");
derived!(Sticker: sticker, GuildStickerCreate, GuildStickerUpdate, GuildStickerDelete, "sticker");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ShardResume {
    pub replayed: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct ShardReconnecting;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Invalidated;
