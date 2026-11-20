use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    application_command::ApplicationCommand,
    auto_moderation::{Rule, TriggerType},
    channel::{Channel, OverwriteType},
    guild::{IntegrationType, PartialIntegration},
    guild_scheduled_event::GuildScheduledEvent,
    id::{ApplicationId, AuditLogEntryId, ChannelId, GenericId, GuildId, MessageId, UserId},
    macros::int_enum,
    user::User,
    util::{JsonValue, JsonValueError},
    webhook::Webhook,
};

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AuditLog {
    #[serde(default)]
    pub application_commands: Vec<ApplicationCommand>,

    #[serde(default)]
    pub audit_log_entries: Vec<AuditLogEntry>,

    #[serde(default)]
    pub auto_moderation_rules: Vec<Rule>,

    #[serde(default)]
    pub guild_scheduled_events: Vec<GuildScheduledEvent>,

    #[serde(default)]
    pub integrations: Vec<PartialIntegration>,

    #[serde(default)]
    pub threads: Vec<Channel>,

    #[serde(default)]
    pub users: Vec<User>,

    #[serde(default)]
    pub webhooks: Vec<Webhook>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AuditLogEntry {
    pub id: AuditLogEntryId,

    pub action_type: AuditLogEvent,

    pub target_id: Option<GenericId>,

    pub user_id: Option<UserId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changes: Vec<AuditLogChange>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Box<AuditLogEntryOptions>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

int_enum! {



    pub enum AuditLogEvent: u8 {

        GuildUpdate = 1,

        ChannelCreate = 10,

        ChannelUpdate = 11,

        ChannelDelete = 12,


        ChannelOverwriteCreate = 13,


        ChannelOverwriteUpdate = 14,


        ChannelOverwriteDelete = 15,

        ChannelPositionUpdate = 16,

        MemberKick = 20,

        MemberPrune = 21,

        MemberBanAdd = 22,

        MemberBanRemove = 23,

        MemberUpdate = 24,


        MemberRoleUpdate = 25,

        MemberMove = 26,

        MemberDisconnect = 27,

        BotAdd = 28,

        RoleCreate = 30,

        RoleUpdate = 31,

        RoleDelete = 32,

        RolePositionUpdate = 33,

        InviteCreate = 40,

        InviteUpdate = 41,

        InviteDelete = 42,

        WebhookCreate = 50,

        WebhookUpdate = 51,

        WebhookDelete = 52,

        EmojiCreate = 60,

        EmojiUpdate = 61,

        EmojiDelete = 62,

        MessageDelete = 72,

        MessageBulkDelete = 73,

        MessagePin = 74,

        MessageUnpin = 75,

        IntegrationCreate = 80,

        IntegrationUpdate = 81,

        IntegrationDelete = 82,

        StageInstanceCreate = 83,

        StageInstanceUpdate = 84,

        StageInstanceDelete = 85,

        StickerCreate = 90,

        StickerUpdate = 91,

        StickerDelete = 92,

        GuildScheduledEventCreate = 100,

        GuildScheduledEventUpdate = 101,

        GuildScheduledEventDelete = 102,

        ThreadCreate = 110,

        ThreadUpdate = 111,

        ThreadDelete = 112,


        ApplicationCommandPermissionUpdate = 121,

        SoundboardSoundCreate = 130,

        SoundboardSoundUpdate = 131,

        SoundboardSoundDelete = 132,

        AutoModerationRuleCreate = 140,

        AutoModerationRuleUpdate = 141,

        AutoModerationRuleDelete = 142,


        AutoModerationBlockMessage = 143,


        AutoModerationFlagToChannel = 144,


        AutoModerationUserCommunicationDisabled = 145,


        AutoModerationQuarantineUser = 146,

        CreatorMonetizationRequestCreated = 150,

        CreatorMonetizationTermsAccepted = 151,

        OnboardingPromptCreate = 163,

        OnboardingPromptUpdate = 164,

        OnboardingPromptDelete = 165,

        OnboardingCreate = 166,

        OnboardingUpdate = 167,

        HomeSettingsCreate = 190,

        HomeSettingsUpdate = 191,

        VoiceChannelStatusCreate = 192,

        VoiceChannelStatusDelete = 193,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "OptionalAuditEntryInfo")]
pub struct AuditLogEntryOptions {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_moderation_rule_name: Option<CompactString>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_string_number"
    )]
    pub auto_moderation_rule_trigger_type: Option<TriggerType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_string_number"
    )]
    pub count: Option<u32>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_string_number"
    )]
    pub delete_member_days: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<GenericId>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "option_string_number"
    )]
    pub members_removed: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_name: Option<CompactString>,

    #[serde(
        default,
        rename = "type",
        skip_serializing_if = "Option::is_none",
        with = "option_string_number"
    )]
    pub kind: Option<OverwriteType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<IntegrationType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CompactString>,
}

trait StringNumber: Sized + Copy {
    fn from_u64(value: u64) -> Option<Self>;
    fn to_u64(self) -> u64;
}

impl StringNumber for u32 {
    fn from_u64(value: u64) -> Option<Self> {
        Self::try_from(value).ok()
    }

    fn to_u64(self) -> u64 {
        self.into()
    }
}

impl StringNumber for OverwriteType {
    fn from_u64(value: u64) -> Option<Self> {
        u8::try_from(value).ok().map(Self::from)
    }

    fn to_u64(self) -> u64 {
        u8::from(self).into()
    }
}

impl StringNumber for TriggerType {
    fn from_u64(value: u64) -> Option<Self> {
        u8::try_from(value).ok().map(Self::from)
    }

    fn to_u64(self) -> u64 {
        u8::from(self).into()
    }
}

mod option_string_number {
    use core::marker::PhantomData;

    use serde::{
        Deserializer, Serializer,
        de::{Error, Unexpected, Visitor},
    };

    use super::StringNumber;

    #[expect(clippy::ref_option, reason = "signature required by serde(with)")]
    pub(super) fn serialize<T: StringNumber, S: Serializer>(
        value: &Option<T>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        match value {
            Some(value) => serializer.collect_str(&value.to_u64()),
            None => serializer.serialize_none(),
        }
    }

    pub(super) fn deserialize<'de, T: StringNumber, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<T>, D::Error> {
        struct NumberVisitor<T>(PhantomData<T>);

        impl<'de, T: StringNumber> Visitor<'de> for NumberVisitor<T> {
            type Value = Option<T>;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a number as a decimal string")
            }

            fn visit_none<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
                Ok(None)
            }

            fn visit_some<D: Deserializer<'de>>(
                self,
                deserializer: D,
            ) -> Result<Self::Value, D::Error> {
                deserializer.deserialize_any(self)
            }

            fn visit_u64<E: Error>(self, value: u64) -> Result<Self::Value, E> {
                T::from_u64(value)
                    .map(Some)
                    .ok_or_else(|| E::invalid_value(Unexpected::Unsigned(value), &self))
            }

            fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
                value
                    .parse()
                    .ok()
                    .and_then(T::from_u64)
                    .map(Some)
                    .ok_or_else(|| E::invalid_value(Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_option(NumberVisitor(PhantomData))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AuditLogChange {
    pub key: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub old_value: Option<JsonValue>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_value: Option<JsonValue>,
}

impl AuditLogChange {
    pub fn old_value_as<'a, T: Deserialize<'a>>(&'a self) -> Result<Option<T>, JsonValueError> {
        self.old_value.as_ref().map(JsonValue::to_type).transpose()
    }

    pub fn new_value_as<'a, T: Deserialize<'a>>(&'a self) -> Result<Option<T>, JsonValueError> {
        self.new_value.as_ref().map(JsonValue::to_type).transpose()
    }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    #[test]
    fn sizes() {
        assert!(
            size_of::<AuditLogEntry>() <= 128,
            "{}",
            size_of::<AuditLogEntry>()
        );
    }
}
