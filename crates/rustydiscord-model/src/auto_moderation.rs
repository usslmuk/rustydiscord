use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    id::{AutoModerationRuleId, ChannelId, GuildId, RoleId, UserId},
    macros::int_enum,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "AutoModerationRule")]
pub struct Rule {
    pub id: AutoModerationRuleId,

    pub guild_id: GuildId,

    pub name: CompactString,

    pub creator_id: UserId,

    pub event_type: EventType,

    pub trigger_type: TriggerType,

    pub trigger_metadata: TriggerMetadata,

    pub actions: Vec<Action>,

    pub enabled: bool,

    pub exempt_roles: Vec<RoleId>,

    pub exempt_channels: Vec<ChannelId>,
}

int_enum! {



    #[doc(alias = "AutoModerationRuleTriggerType")]
    pub enum TriggerType: u8 {

        Keyword = 1,

        Spam = 3,

        KeywordPreset = 4,

        MentionSpam = 5,

        MemberProfile = 6,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct TriggerMetadata {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keyword_filter: Vec<CompactString>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub regex_patterns: Vec<CompactString>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presets: Vec<KeywordPresetType>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allow_list: Vec<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mention_total_limit: Option<u8>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub mention_raid_protection_enabled: bool,
}

int_enum! {



    pub enum KeywordPresetType: u8 {

        Profanity = 1,

        SexualContent = 2,

        Slurs = 3,
    }
}

int_enum! {



    pub enum EventType: u8 {

        MessageSend = 1,

        MemberUpdate = 2,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Action {
    #[serde(rename = "type")]
    pub kind: ActionType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ActionMetadata>,
}

impl Action {
    #[must_use]
    pub const fn block_message() -> Self {
        Self {
            kind: ActionType::BlockMessage,
            metadata: None,
        }
    }

    #[must_use]
    pub fn block_message_with(custom_message: impl Into<CompactString>) -> Self {
        Self {
            kind: ActionType::BlockMessage,
            metadata: Some(ActionMetadata {
                custom_message: Some(custom_message.into()),
                ..ActionMetadata::default()
            }),
        }
    }

    #[must_use]
    pub const fn send_alert_message(channel_id: ChannelId) -> Self {
        Self {
            kind: ActionType::SendAlertMessage,
            metadata: Some(ActionMetadata {
                channel_id: Some(channel_id),
                duration_seconds: None,
                custom_message: None,
            }),
        }
    }

    #[must_use]
    pub const fn timeout(duration_seconds: u32) -> Self {
        Self {
            kind: ActionType::Timeout,
            metadata: Some(ActionMetadata {
                channel_id: None,
                duration_seconds: Some(duration_seconds),
                custom_message: None,
            }),
        }
    }

    #[must_use]
    pub const fn block_member_interaction() -> Self {
        Self {
            kind: ActionType::BlockMemberInteraction,
            metadata: None,
        }
    }
}

int_enum! {



    pub enum ActionType: u8 {

        BlockMessage = 1,

        SendAlertMessage = 2,


        Timeout = 3,

        BlockMemberInteraction = 4,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActionMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_message: Option<CompactString>,
}
