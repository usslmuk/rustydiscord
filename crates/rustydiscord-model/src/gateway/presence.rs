use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    id::{ApplicationId, EmojiId, GuildId, UserId},
    macros::{int_enum, int_flags, str_enum},
    user::{AvatarDecorationData, Collectibles, PrimaryGuild, UserFlags},
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Presence {
    pub user: PresenceUser,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default = "offline")]
    pub status: Status,

    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    pub activities: Vec<Activity>,

    #[serde(default)]
    pub client_status: ClientStatus,
}

fn offline() -> Status {
    Status::Offline
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PresenceUser {
    pub id: UserId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub username: Option<CompactString>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::user::option_discriminator"
    )]
    pub discriminator: Option<u16>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub global_name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub banner: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_flags: Option<UserFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Box<AvatarDecorationData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<Collectibles>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_guild: Option<Box<PrimaryGuild>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ClientStatus {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub desktop: Option<Status>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mobile: Option<Status>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web: Option<Status>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vr: Option<Status>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Activity {
    pub name: CompactString,

    #[serde(rename = "type")]
    pub kind: ActivityType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_util::option_unix_millis"
    )]
    pub created_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamps: Option<ActivityTimestamps>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_display_type: Option<StatusDisplayType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<ActivityEmoji>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub party: Option<ActivityParty>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assets: Option<Box<ActivityAssets>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secrets: Option<Box<ActivitySecrets>>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub instance: bool,

    #[serde(default, skip_serializing_if = "ActivityFlags::is_empty")]
    pub flags: ActivityFlags,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buttons: Vec<CompactString>,
}

impl Activity {
    #[must_use]
    pub fn new(kind: ActivityType, name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            kind,
            url: None,
            created_at: None,
            timestamps: None,
            application_id: None,
            status_display_type: None,
            details: None,
            details_url: None,
            state: None,
            state_url: None,
            emoji: None,
            party: None,
            assets: None,
            secrets: None,
            instance: false,
            flags: ActivityFlags::empty(),
            buttons: Vec::new(),
        }
    }

    #[must_use]
    pub fn playing(name: impl Into<CompactString>) -> Self {
        Self::new(ActivityType::Playing, name)
    }

    #[must_use]
    pub fn streaming(name: impl Into<CompactString>, url: impl Into<CompactString>) -> Self {
        Self {
            url: Some(url.into()),
            ..Self::new(ActivityType::Streaming, name)
        }
    }

    #[must_use]
    pub fn listening(name: impl Into<CompactString>) -> Self {
        Self::new(ActivityType::Listening, name)
    }

    #[must_use]
    pub fn watching(name: impl Into<CompactString>) -> Self {
        Self::new(ActivityType::Watching, name)
    }

    #[must_use]
    pub fn competing(name: impl Into<CompactString>) -> Self {
        Self::new(ActivityType::Competing, name)
    }

    #[must_use]
    pub fn custom(text: impl Into<CompactString>) -> Self {
        Self {
            state: Some(text.into()),
            ..Self::new(ActivityType::Custom, "Custom Status")
        }
    }
}

int_enum! {



    pub enum ActivityType: u8 {

        Playing = 0,

        Streaming = 1,

        Listening = 2,

        Watching = 3,

        Custom = 4,

        Competing = 5,
    }
}

int_enum! {




    pub enum StatusDisplayType: u8 {

        Name = 0,

        State = 1,

        Details = 2,
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityTimestamps {
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_util::option_unix_millis"
    )]
    pub start: Option<Timestamp>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_util::option_unix_millis"
    )]
    pub end: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityEmoji {
    pub name: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<EmojiId>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub animated: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityParty {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size: Option<(u32, u32)>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityAssets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_image: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_text: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub large_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_image: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_text: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub small_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub invite_cover_image: Option<CompactString>,
}

#[derive(Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivitySecrets {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub join: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spectate: Option<CompactString>,

    #[serde(default, rename = "match", skip_serializing_if = "Option::is_none")]
    pub match_: Option<CompactString>,
}

impl core::fmt::Debug for ActivitySecrets {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let redact = |secret: &Option<CompactString>| secret.as_ref().map(|_| "<redacted>");
        f.debug_struct("ActivitySecrets")
            .field("join", &redact(&self.join))
            .field("spectate", &redact(&self.spectate))
            .field("match", &redact(&self.match_))
            .finish()
    }
}

int_flags! {




    pub struct ActivityFlags: u64 {

        const INSTANCE = 1 << 0;

        const JOIN = 1 << 1;

        const SPECTATE = 1 << 2;

        const JOIN_REQUEST = 1 << 3;

        const SYNC = 1 << 4;

        const PLAY = 1 << 5;

        const PARTY_PRIVACY_FRIENDS = 1 << 6;

        const PARTY_PRIVACY_VOICE_CHANNEL = 1 << 7;

        const EMBEDDED = 1 << 8;
    }
}

str_enum! {




    pub enum Status {

        Online = "online",

        DoNotDisturb = "dnd",

        Idle = "idle",

        Invisible = "invisible",

        Offline = "offline",
    }
}
