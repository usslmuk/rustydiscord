use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};

use crate::{
    id::{InteractionId, MessageId},
    interaction::{AuthorizingIntegrationOwners, InteractionType},
    user::User,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "MessageInteractionMetadata")]
pub enum InteractionMetadata {
    ApplicationCommand(CommandMetadata),

    MessageComponent(ComponentMetadata),

    ModalSubmit(ModalSubmitMetadata),

    Unknown(UnknownMetadata),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[doc(alias = "ApplicationCommandInteractionMetadata")]
pub struct CommandMetadata {
    pub id: InteractionId,

    pub user: User,

    pub authorizing_integration_owners: AuthorizingIntegrationOwners,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user: Option<Box<User>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_message_id: Option<MessageId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[doc(alias = "MessageComponentInteractionMetadata")]
pub struct ComponentMetadata {
    pub id: InteractionId,

    pub user: User,

    pub authorizing_integration_owners: AuthorizingIntegrationOwners,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,

    pub interacted_message_id: MessageId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[doc(alias = "ModalSubmitInteractionMetadata")]
pub struct ModalSubmitMetadata {
    pub id: InteractionId,

    pub user: User,

    pub authorizing_integration_owners: AuthorizingIntegrationOwners,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,

    pub triggering_interaction_metadata: Box<InteractionMetadata>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UnknownMetadata {
    #[serde(skip)]
    pub kind: InteractionType,

    pub id: InteractionId,

    pub user: User,

    pub authorizing_integration_owners: AuthorizingIntegrationOwners,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_response_message_id: Option<MessageId>,
}

impl InteractionMetadata {
    #[must_use]
    pub const fn id(&self) -> InteractionId {
        match self {
            Self::ApplicationCommand(metadata) => metadata.id,
            Self::MessageComponent(metadata) => metadata.id,
            Self::ModalSubmit(metadata) => metadata.id,
            Self::Unknown(metadata) => metadata.id,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> InteractionType {
        match self {
            Self::ApplicationCommand(_) => InteractionType::ApplicationCommand,
            Self::MessageComponent(_) => InteractionType::MessageComponent,
            Self::ModalSubmit(_) => InteractionType::ModalSubmit,
            Self::Unknown(metadata) => metadata.kind,
        }
    }

    #[must_use]
    pub const fn user(&self) -> &User {
        match self {
            Self::ApplicationCommand(metadata) => &metadata.user,
            Self::MessageComponent(metadata) => &metadata.user,
            Self::ModalSubmit(metadata) => &metadata.user,
            Self::Unknown(metadata) => &metadata.user,
        }
    }

    #[must_use]
    pub const fn authorizing_integration_owners(&self) -> &AuthorizingIntegrationOwners {
        match self {
            Self::ApplicationCommand(metadata) => &metadata.authorizing_integration_owners,
            Self::MessageComponent(metadata) => &metadata.authorizing_integration_owners,
            Self::ModalSubmit(metadata) => &metadata.authorizing_integration_owners,
            Self::Unknown(metadata) => &metadata.authorizing_integration_owners,
        }
    }

    #[must_use]
    pub const fn original_response_message_id(&self) -> Option<MessageId> {
        match self {
            Self::ApplicationCommand(metadata) => metadata.original_response_message_id,
            Self::MessageComponent(metadata) => metadata.original_response_message_id,
            Self::ModalSubmit(metadata) => metadata.original_response_message_id,
            Self::Unknown(metadata) => metadata.original_response_message_id,
        }
    }
}

#[derive(Deserialize)]
struct RawMetadata {
    #[serde(rename = "type")]
    kind: InteractionType,
    id: InteractionId,
    user: User,
    authorizing_integration_owners: AuthorizingIntegrationOwners,
    #[serde(default)]
    original_response_message_id: Option<MessageId>,
    #[serde(default)]
    target_user: Option<Box<User>>,
    #[serde(default)]
    target_message_id: Option<MessageId>,
    #[serde(default)]
    interacted_message_id: Option<MessageId>,
    #[serde(default)]
    triggering_interaction_metadata: Option<Box<InteractionMetadata>>,
}

impl<'de> Deserialize<'de> for InteractionMetadata {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawMetadata::deserialize(deserializer)?;
        Ok(match raw.kind {
            InteractionType::ApplicationCommand => Self::ApplicationCommand(CommandMetadata {
                id: raw.id,
                user: raw.user,
                authorizing_integration_owners: raw.authorizing_integration_owners,
                original_response_message_id: raw.original_response_message_id,
                target_user: raw.target_user,
                target_message_id: raw.target_message_id,
            }),
            InteractionType::MessageComponent => Self::MessageComponent(ComponentMetadata {
                id: raw.id,
                user: raw.user,
                authorizing_integration_owners: raw.authorizing_integration_owners,
                original_response_message_id: raw.original_response_message_id,
                interacted_message_id: raw
                    .interacted_message_id
                    .ok_or_else(|| D::Error::missing_field("interacted_message_id"))?,
            }),
            InteractionType::ModalSubmit => Self::ModalSubmit(ModalSubmitMetadata {
                id: raw.id,
                user: raw.user,
                authorizing_integration_owners: raw.authorizing_integration_owners,
                original_response_message_id: raw.original_response_message_id,
                triggering_interaction_metadata: raw
                    .triggering_interaction_metadata
                    .ok_or_else(|| D::Error::missing_field("triggering_interaction_metadata"))?,
            }),
            kind => Self::Unknown(UnknownMetadata {
                kind,
                id: raw.id,
                user: raw.user,
                authorizing_integration_owners: raw.authorizing_integration_owners,
                original_response_message_id: raw.original_response_message_id,
            }),
        })
    }
}

#[derive(Serialize)]
struct Tagged<'a, T> {
    #[serde(rename = "type")]
    kind: InteractionType,
    #[serde(flatten)]
    metadata: &'a T,
}

impl Serialize for InteractionMetadata {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let kind = self.kind();
        match self {
            Self::ApplicationCommand(metadata) => Tagged { kind, metadata }.serialize(serializer),
            Self::MessageComponent(metadata) => Tagged { kind, metadata }.serialize(serializer),
            Self::ModalSubmit(metadata) => Tagged { kind, metadata }.serialize(serializer),
            Self::Unknown(metadata) => Tagged { kind, metadata }.serialize(serializer),
        }
    }
}
