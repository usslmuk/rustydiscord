use core::{
    fmt,
    sync::atomic::{AtomicU64, Ordering},
};

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
use web_time::{SystemTime, UNIX_EPOCH};

use compact_str::CompactString;
use serde::{Deserialize, Serialize, Serializer};

use super::ForumTagData;
use crate::model::{
    channel::{
        AutoArchiveDuration, Channel, ChannelFlags, ChannelType, DefaultReaction, ForumLayoutType,
        Overwrite, SortOrderType, ThreadMember, VideoQualityMode,
    },
    emoji::PartialEmoji,
    guild::{
        DefaultMessageNotificationLevel, ExplicitContentFilterLevel, Feature, MemberFlags,
        MfaLevel, OnboardingMode, OnboardingPrompt, PromptOption, PromptType, RoleColors,
        SystemChannelFlags, VerificationLevel, WelcomeScreenChannel,
    },
    id::{ChannelId, EmojiId, GenericId, RoleId, UserId},
    permissions::Permissions,
    util::{ImageData, Locale, Patch, Timestamp},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetGuild {
    pub with_counts: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuild {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub verification_level: Patch<VerificationLevel>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub default_message_notifications: Patch<DefaultMessageNotificationLevel>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub explicit_content_filter: Patch<ExplicitContentFilterLevel>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub afk_channel_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub icon: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub splash: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub discovery_splash: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub banner: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub system_channel_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub rules_channel_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub public_updates_channel_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub preferred_locale: Patch<Locale>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<Vec<Feature>>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description: Patch<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub premium_progress_bar_enabled: Option<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub safety_alerts_channel_id: Patch<ChannelId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGuildChannel {
    pub name: CompactString,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChannelType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bitrate: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_limit: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i32>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permission_overwrites: Vec<Overwrite>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub nsfw: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtc_region: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_quality_mode: Option<VideoQualityMode>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_reaction_emoji: Option<DefaultReaction>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available_tags: Vec<ForumTagData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_sort_order: Option<SortOrderType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,
}

impl CreateGuildChannel {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            kind: None,
            topic: None,
            bitrate: None,
            user_limit: None,
            rate_limit_per_user: None,
            position: None,
            permission_overwrites: Vec::new(),
            parent_id: None,
            nsfw: false,
            rtc_region: None,
            video_quality_mode: None,
            default_auto_archive_duration: None,
            default_reaction_emoji: None,
            available_tags: Vec::new(),
            default_sort_order: None,
            default_forum_layout: None,
            default_thread_rate_limit_per_user: None,
            flags: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildChannelPosition {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub position: Patch<i32>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub lock_permissions: Patch<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub parent_id: Patch<ChannelId>,
}

impl ModifyGuildChannelPosition {
    #[must_use]
    pub const fn new(id: ChannelId, position: i32) -> Self {
        Self {
            id,
            position: Patch::Set(position),
            lock_permissions: Patch::Keep,
            parent_id: Patch::Keep,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActiveThreads {
    pub threads: Vec<Channel>,

    pub members: Vec<ThreadMember>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListGuildMembers {
    pub limit: Option<u16>,

    pub after: Option<UserId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct SearchGuildMembers {
    pub query: CompactString,

    pub limit: Option<u16>,
}

impl SearchGuildMembers {
    #[must_use]
    pub fn new(query: impl Into<CompactString>) -> Self {
        Self {
            query: query.into(),
            limit: None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct AddGuildMember {
    pub access_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<CompactString>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,
}

impl AddGuildMember {
    #[must_use]
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            nick: None,
            roles: Vec::new(),
            mute: None,
            deaf: None,
        }
    }
}

impl fmt::Debug for AddGuildMember {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AddGuildMember")
            .field("access_token", &"<redacted>")
            .field("nick", &self.nick)
            .field("roles", &self.roles)
            .field("mute", &self.mute)
            .field("deaf", &self.deaf)
            .finish()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildMember {
    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub nick: Patch<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub roles: Patch<Vec<RoleId>>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub mute: Patch<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub deaf: Patch<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub channel_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub communication_disabled_until: Patch<Timestamp>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub flags: Patch<MemberFlags>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyCurrentMember {
    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub nick: Patch<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub banner: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub avatar: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub bio: Patch<CompactString>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetGuildBans {
    pub limit: Option<u16>,

    pub before: Option<UserId>,

    pub after: Option<UserId>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGuildBan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct BulkGuildBan {
    pub user_ids: Vec<UserId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_message_seconds: Option<u32>,
}

impl BulkGuildBan {
    #[must_use]
    pub const fn new(user_ids: Vec<UserId>) -> Self {
        Self {
            user_ids,
            delete_message_seconds: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct BulkBanResponse {
    pub banned_users: Vec<UserId>,

    pub failed_users: Vec<UserId>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGuildRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<RoleColors>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub hoist: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub unicode_emoji: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub mentionable: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildRolePosition {
    pub id: RoleId,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub position: Patch<i32>,
}

impl ModifyGuildRolePosition {
    #[must_use]
    pub const fn new(id: RoleId, position: i32) -> Self {
        Self {
            id,
            position: Patch::Set(position),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildRole {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub colors: Patch<RoleColors>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hoist: Option<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub icon: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub unicode_emoji: Patch<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mentionable: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetGuildPruneCount {
    pub days: Option<u8>,

    pub include_roles: Vec<RoleId>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct BeginGuildPrune {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_prune_count: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub include_roles: Vec<RoleId>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildWidget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub channel_id: Patch<ChannelId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VanityUrl {
    pub code: Option<CompactString>,

    pub uses: u32,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildWelcomeScreen {
    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub enabled: Patch<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub welcome_channels: Patch<Vec<WelcomeScreenChannel>>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description: Patch<CompactString>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildOnboarding {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<Vec<OnboardingPromptData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_channel_ids: Option<Vec<ChannelId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<OnboardingMode>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct OnboardingPromptData {
    pub id: Option<GenericId>,

    pub kind: PromptType,

    pub options: Vec<PromptOptionData>,

    pub title: CompactString,

    pub single_select: bool,

    pub required: bool,

    pub in_onboarding: bool,
}

impl OnboardingPromptData {
    #[must_use]
    pub fn new(title: impl Into<CompactString>, options: Vec<PromptOptionData>) -> Self {
        Self {
            id: None,
            kind: PromptType::MultipleChoice,
            options,
            title: title.into(),
            single_select: false,
            required: false,
            in_onboarding: true,
        }
    }
}

impl From<OnboardingPrompt> for OnboardingPromptData {
    fn from(prompt: OnboardingPrompt) -> Self {
        let OnboardingPrompt {
            id,
            kind,
            options,
            title,
            single_select,
            required,
            in_onboarding,
            ..
        } = prompt;
        Self {
            id: Some(id),
            kind,
            options: options.into_iter().map(Into::into).collect(),
            title,
            single_select,
            required,
            in_onboarding,
        }
    }
}

impl Serialize for OnboardingPromptData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            id: GenericId,
            #[serde(rename = "type")]
            kind: PromptType,
            options: &'a [PromptOptionData],
            title: &'a str,
            single_select: bool,
            required: bool,
            in_onboarding: bool,
        }

        let Self {
            id,
            kind,
            options,
            title,
            single_select,
            required,
            in_onboarding,
        } = self;
        Wire {
            id: id.unwrap_or_else(generated_id),
            kind: *kind,
            options,
            title,
            single_select: *single_select,
            required: *required,
            in_onboarding: *in_onboarding,
        }
        .serialize(serializer)
    }
}

fn generated_id() -> GenericId {
    static INCREMENT: AtomicU64 = AtomicU64::new(0);
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| {
            u64::try_from(since.as_millis()).unwrap_or(u64::MAX)
        });
    let base = GenericId::from_timestamp_ms(now_ms).map_or(1 << 22, GenericId::get);
    GenericId::new(base | (INCREMENT.fetch_add(1, Ordering::Relaxed) & 0xfff))
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct PromptOptionData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericId>,

    pub channel_ids: Vec<ChannelId>,

    pub role_ids: Vec<RoleId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<EmojiId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_animated: Option<bool>,

    pub title: CompactString,

    pub description: Option<CompactString>,
}

impl PromptOptionData {
    #[must_use]
    pub fn new(title: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            channel_ids: Vec::new(),
            role_ids: Vec::new(),
            emoji_id: None,
            emoji_name: None,
            emoji_animated: None,
            title: title.into(),
            description: None,
        }
    }
}

impl From<PromptOption> for PromptOptionData {
    fn from(option: PromptOption) -> Self {
        let PromptOption {
            id,
            channel_ids,
            role_ids,
            emoji,
            title,
            description,
            ..
        } = option;
        let (emoji_id, emoji_name, emoji_animated) = match emoji {
            Some(PartialEmoji::Custom { id, name, animated }) => (Some(id), name, Some(animated)),
            Some(PartialEmoji::Unicode { name }) => (None, Some(name), None),
            None => (None, None, None),
        };
        Self {
            id: Some(id),
            channel_ids,
            role_ids,
            emoji_id,
            emoji_name,
            emoji_animated,
            title,
            description,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGuild {
    pub name: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_level: Option<VerificationLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_message_notifications: Option<DefaultMessageNotificationLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_content_filter: Option<ExplicitContentFilterLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_channel_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afk_timeout: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_channel_flags: Option<SystemChannelFlags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale: Option<Locale>,
}

impl CreateGuild {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildMfaLevel {
    pub level: MfaLevel,
}

impl ModifyGuildMfaLevel {
    #[must_use]
    pub const fn new(level: MfaLevel) -> Self {
        Self { level }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMembershipScreening {
    pub version: CompactString,

    pub form_fields: Vec<MembershipScreeningField>,

    pub description: Option<CompactString>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MembershipScreeningField {
    pub field_type: MembershipScreeningFieldType,

    pub label: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<CompactString>>,

    pub required: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MembershipScreeningFieldType {
    #[default]
    #[serde(rename = "TERMS")]
    Terms,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildMembershipScreening {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_fields: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description: Patch<CompactString>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildIncidentActions {
    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub invites_disabled_until: Patch<Timestamp>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub dms_disabled_until: Patch<Timestamp>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modify_guild_and_member() {
        let guild = ModifyGuild {
            name: Some("Rustaceans".into()),
            icon: Patch::Clear,
            preferred_locale: Patch::Set(Locale::Polish),
            ..ModifyGuild::default()
        };
        assert_eq!(
            serde_json::to_string(&guild).unwrap(),
            r#"{"name":"Rustaceans","icon":null,"preferred_locale":"pl"}"#
        );

        let member = ModifyGuildMember {
            nick: Patch::Clear,
            roles: Patch::Set(vec![RoleId::new(1)]),
            channel_id: Patch::Clear,
            ..ModifyGuildMember::default()
        };
        assert_eq!(
            serde_json::to_string(&member).unwrap(),
            r#"{"nick":null,"roles":["1"],"channel_id":null}"#
        );

        let add = AddGuildMember::new("super-secret");
        assert!(!format!("{add:?}").contains("super-secret"));
    }

    #[test]
    fn positions_and_roles() {
        let mut moved = ModifyGuildChannelPosition::new(ChannelId::new(2), 0);
        moved.parent_id = Patch::Clear;
        moved.lock_permissions = Patch::Set(true);
        assert_eq!(
            serde_json::to_string(&[moved]).unwrap(),
            r#"[{"id":"2","position":0,"lock_permissions":true,"parent_id":null}]"#
        );

        let role = CreateGuildRole {
            name: Some("mods".into()),
            colors: Some(RoleColors::solid(0x0000_ff00)),
            hoist: true,
            ..CreateGuildRole::default()
        };
        assert_eq!(
            serde_json::to_string(&role).unwrap(),
            concat!(
                r#"{"name":"mods","colors":{"primary_color":65280,"secondary_color":null,"#,
                r#""tertiary_color":null},"hoist":true}"#
            )
        );
        let modify = ModifyGuildRole {
            unicode_emoji: Patch::Clear,
            ..ModifyGuildRole::default()
        };
        assert_eq!(
            serde_json::to_string(&modify).unwrap(),
            r#"{"unicode_emoji":null}"#
        );
    }

    #[test]
    fn new_prompts_get_unique_generated_ids() {
        let mut option = PromptOptionData::new("Rust");
        option.emoji_name = Some("🦀".into());
        let prompt = OnboardingPromptData::new("Languages", vec![option]);
        let first: serde_json::Value = serde_json::to_value(&prompt).unwrap();
        let second: serde_json::Value = serde_json::to_value(&prompt).unwrap();
        assert_ne!(first["id"], second["id"]);
        let id: u64 = first["id"].as_str().unwrap().parse().unwrap();
        let now_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let id_ms = u128::from(GenericId::new(id).timestamp_ms());
        assert!(now_ms.abs_diff(id_ms) < 60_000, "{id_ms} vs {now_ms}");
        assert_eq!(
            first["options"],
            serde_json::json!([{
                "channel_ids": [], "role_ids": [], "emoji_name": "🦀", "title": "Rust",
                "description": null
            }])
        );

        let existing = OnboardingPromptData {
            id: Some(GenericId::new(42)),
            ..prompt
        };
        assert_eq!(serde_json::to_value(&existing).unwrap()["id"], "42");
    }
}
