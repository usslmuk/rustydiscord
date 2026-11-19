use compact_str::CompactString;
use serde::Serialize;

use crate::{
    auto_moderation::{Action, EventType, KeywordPresetType, TriggerMetadata, TriggerType},
    id::{ChannelId, RoleId},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateAutoModerationRule {
    pub name: CompactString,

    pub event_type: EventType,

    pub trigger_type: TriggerType,

    #[serde(skip_serializing_if = "is_empty")]
    pub trigger_metadata: TriggerMetadata,

    pub actions: Vec<Action>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exempt_roles: Vec<RoleId>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub exempt_channels: Vec<ChannelId>,
}

fn is_empty(metadata: &TriggerMetadata) -> bool {
    *metadata == TriggerMetadata::default()
}

impl CreateAutoModerationRule {
    fn with(
        name: impl Into<CompactString>,
        event_type: EventType,
        trigger_type: TriggerType,
        trigger_metadata: TriggerMetadata,
        actions: impl IntoIterator<Item = Action>,
    ) -> Self {
        Self {
            name: name.into(),
            event_type,
            trigger_type,
            trigger_metadata,
            actions: actions.into_iter().collect(),
            enabled: None,
            exempt_roles: Vec::new(),
            exempt_channels: Vec::new(),
        }
    }

    #[must_use]
    pub fn keyword(
        name: impl Into<CompactString>,
        keywords: impl IntoIterator<Item = impl Into<CompactString>>,
        actions: impl IntoIterator<Item = Action>,
    ) -> Self {
        let metadata = TriggerMetadata {
            keyword_filter: keywords.into_iter().map(Into::into).collect(),
            ..TriggerMetadata::default()
        };
        Self::with(
            name,
            EventType::MessageSend,
            TriggerType::Keyword,
            metadata,
            actions,
        )
    }

    #[must_use]
    pub fn spam(name: impl Into<CompactString>, actions: impl IntoIterator<Item = Action>) -> Self {
        Self::with(
            name,
            EventType::MessageSend,
            TriggerType::Spam,
            TriggerMetadata::default(),
            actions,
        )
    }

    #[must_use]
    pub fn keyword_preset(
        name: impl Into<CompactString>,
        presets: impl IntoIterator<Item = KeywordPresetType>,
        actions: impl IntoIterator<Item = Action>,
    ) -> Self {
        let metadata = TriggerMetadata {
            presets: presets.into_iter().collect(),
            ..TriggerMetadata::default()
        };
        Self::with(
            name,
            EventType::MessageSend,
            TriggerType::KeywordPreset,
            metadata,
            actions,
        )
    }

    #[must_use]
    pub fn mention_spam(
        name: impl Into<CompactString>,
        limit: u8,
        actions: impl IntoIterator<Item = Action>,
    ) -> Self {
        let metadata = TriggerMetadata {
            mention_total_limit: Some(limit),
            ..TriggerMetadata::default()
        };
        Self::with(
            name,
            EventType::MessageSend,
            TriggerType::MentionSpam,
            metadata,
            actions,
        )
    }

    #[must_use]
    pub fn member_profile(
        name: impl Into<CompactString>,
        keywords: impl IntoIterator<Item = impl Into<CompactString>>,
        actions: impl IntoIterator<Item = Action>,
    ) -> Self {
        let metadata = TriggerMetadata {
            keyword_filter: keywords.into_iter().map(Into::into).collect(),
            ..TriggerMetadata::default()
        };
        Self::with(
            name,
            EventType::MemberUpdate,
            TriggerType::MemberProfile,
            metadata,
            actions,
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ModifyAutoModerationRule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<EventType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_metadata: Option<TriggerMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<Action>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_roles: Option<Vec<RoleId>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_channels: Option<Vec<ChannelId>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auto_moderation::{Action, KeywordPresetType};

    #[test]
    fn constructors_set_the_trigger() {
        let json = |body: &CreateAutoModerationRule| serde_json::to_string(body).unwrap();
        assert_eq!(
            json(&CreateAutoModerationRule::spam(
                "Spam",
                [Action::block_message()]
            )),
            r#"{"name":"Spam","event_type":1,"trigger_type":3,"actions":[{"type":1}]}"#
        );
        assert_eq!(
            json(&CreateAutoModerationRule::keyword_preset(
                "Slurs",
                [KeywordPresetType::Slurs],
                [Action::block_message()]
            )),
            r#"{"name":"Slurs","event_type":1,"trigger_type":4,"trigger_metadata":{"presets":[3]},"actions":[{"type":1}]}"#
        );
        let mut mentions = CreateAutoModerationRule::mention_spam(
            "Mentions",
            10,
            [Action::block_message(), Action::timeout(60)],
        );
        mentions.trigger_metadata.mention_raid_protection_enabled = true;
        assert_eq!(
            json(&mentions),
            r#"{"name":"Mentions","event_type":1,"trigger_type":5,"trigger_metadata":{"mention_total_limit":10,"mention_raid_protection_enabled":true},"actions":[{"type":1},{"type":3,"metadata":{"duration_seconds":60}}]}"#
        );
        assert_eq!(
            json(&CreateAutoModerationRule::member_profile(
                "Names",
                ["*admin*"],
                [Action::block_member_interaction()]
            )),
            r#"{"name":"Names","event_type":2,"trigger_type":6,"trigger_metadata":{"keyword_filter":["*admin*"]},"actions":[{"type":4}]}"#
        );
    }
}
