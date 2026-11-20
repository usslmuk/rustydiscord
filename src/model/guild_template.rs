use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    channel::{
        AutoArchiveDuration, ChannelFlags, ChannelType, DefaultReaction, ForumLayoutType,
        OverwriteType, SortOrderType, VideoQualityMode,
    },
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilterLevel, RoleColors,
        SystemChannelFlags, VerificationLevel,
    },
    id::{EmojiId, GuildId, UserId},
    permissions::Permissions,
    user::User,
    util::{ImageHash, Locale, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "Template")]
pub struct GuildTemplate {
    pub code: CompactString,

    pub name: CompactString,

    pub description: Option<CompactString>,

    pub usage_count: u32,

    pub creator_id: UserId,

    pub creator: User,

    pub created_at: Timestamp,

    pub updated_at: Timestamp,

    pub source_guild_id: GuildId,

    pub serialized_source_guild: SerializedSourceGuild,

    pub is_dirty: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SerializedSourceGuild {
    pub name: CompactString,

    #[serde(default)]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub region: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<DefaultMessageNotificationLevel>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,

    pub preferred_locale: Locale,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<TemplateRole>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<TemplateChannel>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::model::serde_util::option_placeholder_id"
    )]
    pub afk_channel_id: Option<u64>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::model::serde_util::option_placeholder_id"
    )]
    pub system_channel_id: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,

    #[serde(default)]
    pub icon_hash: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub premium_progress_bar_enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TemplateRole {
    #[serde(with = "crate::model::serde_util::placeholder_id")]
    pub id: u64,

    pub name: CompactString,

    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub permissions: Permissions,

    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub color: u32,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub colors: Option<RoleColors>,

    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub hoist: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<CompactString>,

    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub mentionable: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TemplateChannel {
    #[serde(with = "crate::model::serde_util::placeholder_id")]
    pub id: u64,

    #[serde(rename = "type")]
    pub kind: ChannelType,

    pub name: CompactString,

    #[serde(default)]
    pub position: i32,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub topic: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub nsfw: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::model::serde_util::option_placeholder_id"
    )]
    pub parent_id: Option<u64>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permission_overwrites: Vec<TemplateOverwrite>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub available_tags: Vec<TemplateForumTag>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<DefaultReaction>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<SortOrderType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TemplateOverwrite {
    #[serde(with = "crate::model::serde_util::placeholder_id")]
    pub id: u64,

    #[serde(rename = "type")]
    pub kind: OverwriteType,

    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub allow: Permissions,

    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub deny: Permissions,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TemplateForumTag {
    pub name: CompactString,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub moderated: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<EmojiId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<CompactString>,
}
