use core::fmt;

use compact_str::CompactString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Visitor},
};

use crate::{
    application::{IntegrationType, PartialApplication},
    channel::Channel,
    component::Component,
    entitlement::Entitlement,
    guild::Guild,
    id::{ApplicationId, ChannelId, GenericId, LobbyId, MessageId},
    lobby::LobbyMessage,
    macros::{int_enum, str_enum},
    message::{Attachment, Embed, MessageActivity, MessageFlags, MessageType},
    oauth2::Scope,
    user::User,
    util::{JsonValue, JsonValueError, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhookEventPayload {
    pub version: u8,

    pub application_id: ApplicationId,

    #[serde(rename = "type")]
    pub kind: PayloadType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<WebhookEvent>,
}

int_enum! {



    pub enum PayloadType: u8 {

        Ping = 0,

        Event = 1,
    }
}

str_enum! {




    pub enum WebhookEventType {

        ApplicationAuthorized = "APPLICATION_AUTHORIZED",

        ApplicationDeauthorized = "APPLICATION_DEAUTHORIZED",

        EntitlementCreate = "ENTITLEMENT_CREATE",

        EntitlementUpdate = "ENTITLEMENT_UPDATE",

        EntitlementDelete = "ENTITLEMENT_DELETE",

        QuestUserEnrollment = "QUEST_USER_ENROLLMENT",

        LobbyMessageCreate = "LOBBY_MESSAGE_CREATE",

        LobbyMessageUpdate = "LOBBY_MESSAGE_UPDATE",

        LobbyMessageDelete = "LOBBY_MESSAGE_DELETE",

        GameDirectMessageCreate = "GAME_DIRECT_MESSAGE_CREATE",

        GameDirectMessageUpdate = "GAME_DIRECT_MESSAGE_UPDATE",

        GameDirectMessageDelete = "GAME_DIRECT_MESSAGE_DELETE",
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct WebhookEvent {
    pub timestamp: Timestamp,

    pub data: WebhookEventData,
}

impl WebhookEvent {
    #[must_use]
    pub fn kind(&self) -> WebhookEventType {
        self.data.kind()
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum WebhookEventData {
    ApplicationAuthorized(Box<ApplicationAuthorized>),

    ApplicationDeauthorized(Box<ApplicationDeauthorized>),

    EntitlementCreate(Box<Entitlement>),

    EntitlementUpdate(Box<Entitlement>),

    EntitlementDelete(Box<Entitlement>),

    QuestUserEnrollment,

    LobbyMessageCreate(Box<LobbyMessage>),

    LobbyMessageUpdate(Box<LobbyMessage>),

    LobbyMessageDelete(LobbyMessageDelete),

    GameDirectMessageCreate(Box<GameDirectMessage>),

    GameDirectMessageUpdate(Box<GameDirectMessage>),

    GameDirectMessageDelete(Box<GameDirectMessage>),

    Unknown(WebhookEventType),
}

impl WebhookEventData {
    #[must_use]
    pub fn kind(&self) -> WebhookEventType {
        match self {
            Self::ApplicationAuthorized(_) => WebhookEventType::ApplicationAuthorized,
            Self::ApplicationDeauthorized(_) => WebhookEventType::ApplicationDeauthorized,
            Self::EntitlementCreate(_) => WebhookEventType::EntitlementCreate,
            Self::EntitlementUpdate(_) => WebhookEventType::EntitlementUpdate,
            Self::EntitlementDelete(_) => WebhookEventType::EntitlementDelete,
            Self::QuestUserEnrollment => WebhookEventType::QuestUserEnrollment,
            Self::LobbyMessageCreate(_) => WebhookEventType::LobbyMessageCreate,
            Self::LobbyMessageUpdate(_) => WebhookEventType::LobbyMessageUpdate,
            Self::LobbyMessageDelete(_) => WebhookEventType::LobbyMessageDelete,
            Self::GameDirectMessageCreate(_) => WebhookEventType::GameDirectMessageCreate,
            Self::GameDirectMessageUpdate(_) => WebhookEventType::GameDirectMessageUpdate,
            Self::GameDirectMessageDelete(_) => WebhookEventType::GameDirectMessageDelete,
            Self::Unknown(kind) => kind.clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationAuthorized {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<IntegrationType>,

    pub user: User,

    #[serde(default)]
    pub scopes: Vec<Scope>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<Box<Guild>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationDeauthorized {
    pub user: User,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LobbyMessageDelete {
    pub id: MessageId,

    pub lobby_id: LobbyId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GameDirectMessage {
    pub id: MessageId,

    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<MessageType>,

    pub content: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel: Option<Box<Channel>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lobby_id: Option<LobbyId>,

    pub author: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<GenericId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp>,

    #[serde(default)]
    pub flags: MessageFlags,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<Attachment>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity: Option<MessageActivity>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<PartialApplication>>,
}

fn naive_timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Timestamp, D::Error> {
    struct NaiveVisitor;

    impl Visitor<'_> for NaiveVisitor {
        type Value = Timestamp;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("an ISO 8601 timestamp")
        }

        fn visit_str<E: DeError>(self, value: &str) -> Result<Timestamp, E> {
            Timestamp::parse_assume_utc(value).map_err(E::custom)
        }
    }

    deserializer.deserialize_str(NaiveVisitor)
}

#[derive(Deserialize)]
struct RawEvent {
    #[serde(rename = "type")]
    kind: WebhookEventType,
    #[serde(deserialize_with = "naive_timestamp")]
    timestamp: Timestamp,

    #[serde(default)]
    data: Option<JsonValue>,
}

impl<'de> Deserialize<'de> for WebhookEvent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawEvent::deserialize(deserializer)?;
        let convert = |error: JsonValueError| D::Error::custom(format_args!("data: {error}"));
        let data = || {
            raw.data
                .as_ref()
                .ok_or_else(|| D::Error::missing_field("data"))
        };
        let data = match &raw.kind {
            WebhookEventType::QuestUserEnrollment => WebhookEventData::QuestUserEnrollment,
            WebhookEventType::Unknown(_) => WebhookEventData::Unknown(raw.kind.clone()),
            WebhookEventType::ApplicationAuthorized => {
                WebhookEventData::ApplicationAuthorized(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::ApplicationDeauthorized => {
                WebhookEventData::ApplicationDeauthorized(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::EntitlementCreate => {
                WebhookEventData::EntitlementCreate(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::EntitlementUpdate => {
                WebhookEventData::EntitlementUpdate(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::EntitlementDelete => {
                WebhookEventData::EntitlementDelete(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::LobbyMessageCreate => {
                WebhookEventData::LobbyMessageCreate(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::LobbyMessageUpdate => {
                WebhookEventData::LobbyMessageUpdate(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::LobbyMessageDelete => {
                WebhookEventData::LobbyMessageDelete(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::GameDirectMessageCreate => {
                WebhookEventData::GameDirectMessageCreate(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::GameDirectMessageUpdate => {
                WebhookEventData::GameDirectMessageUpdate(data()?.to_type().map_err(convert)?)
            }
            WebhookEventType::GameDirectMessageDelete => {
                WebhookEventData::GameDirectMessageDelete(data()?.to_type().map_err(convert)?)
            }
        };
        Ok(Self {
            timestamp: raw.timestamp,
            data,
        })
    }
}

impl Serialize for WebhookEvent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum DataRef<'a> {
            Authorized(&'a ApplicationAuthorized),
            Deauthorized(&'a ApplicationDeauthorized),
            Entitlement(&'a Entitlement),
            LobbyMessage(&'a LobbyMessage),
            LobbyMessageDelete(&'a LobbyMessageDelete),
            GameDirectMessage(&'a GameDirectMessage),
        }

        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(rename = "type")]
            kind: WebhookEventType,
            timestamp: Timestamp,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<DataRef<'a>>,
        }

        let data = match &self.data {
            WebhookEventData::ApplicationAuthorized(data) => Some(DataRef::Authorized(data)),
            WebhookEventData::ApplicationDeauthorized(data) => Some(DataRef::Deauthorized(data)),
            WebhookEventData::EntitlementCreate(data)
            | WebhookEventData::EntitlementUpdate(data)
            | WebhookEventData::EntitlementDelete(data) => Some(DataRef::Entitlement(data)),
            WebhookEventData::LobbyMessageCreate(data)
            | WebhookEventData::LobbyMessageUpdate(data) => Some(DataRef::LobbyMessage(data)),
            WebhookEventData::LobbyMessageDelete(data) => Some(DataRef::LobbyMessageDelete(data)),
            WebhookEventData::GameDirectMessageCreate(data)
            | WebhookEventData::GameDirectMessageUpdate(data)
            | WebhookEventData::GameDirectMessageDelete(data) => {
                Some(DataRef::GameDirectMessage(data))
            }
            WebhookEventData::QuestUserEnrollment | WebhookEventData::Unknown(_) => None,
        };
        Flat {
            kind: self.kind(),
            timestamp: self.timestamp,
            data,
        }
        .serialize(serializer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(kind: &str, data: &str) -> Result<WebhookEvent, serde_json::Error> {
        serde_json::from_str(&format!(
            r#"{{"data":{data},"type":"{kind}","timestamp":"2024-10-18T14:42:53.064834"}}"#
        ))
    }

    #[test]
    fn data_before_type_and_timestamp_as_utc() {
        let event = event("LOBBY_MESSAGE_DELETE", r#"{"id":"1","lobby_id":"2"}"#).unwrap();
        assert_eq!(event.kind(), WebhookEventType::LobbyMessageDelete);
        assert_eq!(
            event.timestamp,
            "2024-10-18T14:42:53.064834+00:00".parse().unwrap()
        );
        assert_eq!(
            serde_json::to_string(&event).unwrap(),
            r#"{"type":"LOBBY_MESSAGE_DELETE","timestamp":"2024-10-18T14:42:53.064834+00:00","data":{"id":"1","lobby_id":"2"}}"#
        );
    }

    #[test]
    fn unknown_and_dataless_events() {
        let unknown = event("FUTURE_EVENT", r#"{"anything":[1,2]}"#).unwrap();
        assert_eq!(
            unknown.data,
            WebhookEventData::Unknown(WebhookEventType::Unknown("FUTURE_EVENT".into()))
        );
        let quest = event("QUEST_USER_ENROLLMENT", "null").unwrap();
        assert_eq!(quest.data, WebhookEventData::QuestUserEnrollment);
        assert!(!serde_json::to_string(&quest).unwrap().contains("data"));
    }

    #[test]
    fn missing_or_invalid_data_is_an_error() {
        let missing = event("APPLICATION_DEAUTHORIZED", "null").unwrap_err();
        assert!(
            missing.to_string().contains("missing field `data`"),
            "{missing}"
        );
        let invalid = event("LOBBY_MESSAGE_DELETE", r#"{"id":"1"}"#).unwrap_err();
        assert!(invalid.to_string().starts_with("data: "), "{invalid}");
    }
}
