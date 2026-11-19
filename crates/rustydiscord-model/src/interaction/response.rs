use compact_str::CompactString;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

use super::InteractionType;
use crate::{
    application_command::{Choice, RawValue, convert_choices},
    component::Component,
    id::{InteractionId, MessageId},
    macros::int_enum,
    message::{AllowedMentions, AttachmentRequest, Embed, Message, MessageFlags},
    poll::CreatePoll,
    rest::{Attachments, FileUpload, setters},
};

int_enum! {



    #[doc(alias = "InteractionResponseType")]
    pub enum InteractionCallbackType: u8 {

        Pong = 1,

        ChannelMessageWithSource = 4,

        DeferredChannelMessageWithSource = 5,

        DeferredUpdateMessage = 6,

        UpdateMessage = 7,

        ApplicationCommandAutocompleteResult = 8,

        Modal = 9,

        PremiumRequired = 10,

        LaunchActivity = 12,
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum InteractionResponse {
    Pong,

    ChannelMessageWithSource(Box<MessageCallbackData>),

    DeferredChannelMessageWithSource(DeferredCallbackData),

    DeferredUpdateMessage,

    UpdateMessage(Box<MessageCallbackData>),

    AutocompleteResult(AutocompleteCallbackData),

    Modal(Box<ModalCallbackData>),

    PremiumRequired,

    LaunchActivity,

    Unknown(InteractionCallbackType),
}

impl InteractionResponse {
    #[must_use]
    pub fn files(&self) -> &[FileUpload] {
        match self {
            Self::ChannelMessageWithSource(data) | Self::UpdateMessage(data) => &data.files,
            _ => &[],
        }
    }

    #[must_use]
    pub const fn kind(&self) -> InteractionCallbackType {
        match self {
            Self::Pong => InteractionCallbackType::Pong,
            Self::ChannelMessageWithSource(_) => InteractionCallbackType::ChannelMessageWithSource,
            Self::DeferredChannelMessageWithSource(_) => {
                InteractionCallbackType::DeferredChannelMessageWithSource
            }
            Self::DeferredUpdateMessage => InteractionCallbackType::DeferredUpdateMessage,
            Self::UpdateMessage(_) => InteractionCallbackType::UpdateMessage,
            Self::AutocompleteResult(_) => {
                InteractionCallbackType::ApplicationCommandAutocompleteResult
            }
            Self::Modal(_) => InteractionCallbackType::Modal,
            Self::PremiumRequired => InteractionCallbackType::PremiumRequired,
            Self::LaunchActivity => InteractionCallbackType::LaunchActivity,
            Self::Unknown(kind) => *kind,
        }
    }
}

impl Serialize for InteractionResponse {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum Data<'a> {
            Message(&'a MessageCallbackData),
            Deferred(&'a DeferredCallbackData),
            Autocomplete(&'a AutocompleteCallbackData),
            Modal(&'a ModalCallbackData),
        }

        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(rename = "type")]
            kind: InteractionCallbackType,
            #[serde(skip_serializing_if = "Option::is_none")]
            data: Option<Data<'a>>,
        }

        let data = match self {
            Self::ChannelMessageWithSource(data) | Self::UpdateMessage(data) => {
                Some(Data::Message(data))
            }
            Self::DeferredChannelMessageWithSource(data) if !data.flags.is_empty() => {
                Some(Data::Deferred(data))
            }
            Self::AutocompleteResult(data) => Some(Data::Autocomplete(data)),
            Self::Modal(data) => Some(Data::Modal(data)),
            _ => None,
        };
        Flat {
            kind: self.kind(),
            data,
        }
        .serialize(serializer)
    }
}

#[derive(Deserialize)]
struct RawCallbackData {
    #[serde(default)]
    tts: bool,
    #[serde(default)]
    content: Option<CompactString>,
    #[serde(default)]
    embeds: Option<Vec<Embed>>,
    #[serde(default)]
    allowed_mentions: Option<AllowedMentions>,
    #[serde(default)]
    flags: Option<MessageFlags>,
    #[serde(default)]
    components: Option<Vec<Component>>,
    #[serde(default)]
    attachments: Option<Vec<AttachmentRequest>>,
    #[serde(default)]
    poll: Option<CreatePoll>,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    choices: Vec<Choice<RawValue>>,
    #[serde(default)]
    custom_id: Option<CompactString>,
    #[serde(default)]
    title: Option<CompactString>,
}

impl RawCallbackData {
    fn into_message(self) -> MessageCallbackData {
        MessageCallbackData {
            tts: self.tts,
            content: self.content,
            embeds: self.embeds,
            allowed_mentions: self.allowed_mentions,
            flags: self.flags,
            components: self.components,
            attachments: self.attachments,
            poll: self.poll,
            files: Vec::new(),
        }
    }
}

#[derive(Deserialize)]
struct RawResponse {
    #[serde(rename = "type")]
    kind: InteractionCallbackType,
    #[serde(default)]
    data: Option<RawCallbackData>,
}

impl<'de> Deserialize<'de> for InteractionResponse {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawResponse::deserialize(deserializer)?;
        let data = raw.data;
        let missing = || D::Error::missing_field("data");
        Ok(match raw.kind {
            InteractionCallbackType::Pong => Self::Pong,
            InteractionCallbackType::ChannelMessageWithSource => {
                Self::ChannelMessageWithSource(Box::new(data.ok_or_else(missing)?.into_message()))
            }
            InteractionCallbackType::DeferredChannelMessageWithSource => {
                Self::DeferredChannelMessageWithSource(DeferredCallbackData {
                    flags: data.and_then(|data| data.flags).unwrap_or_default(),
                })
            }
            InteractionCallbackType::DeferredUpdateMessage => Self::DeferredUpdateMessage,
            InteractionCallbackType::UpdateMessage => {
                Self::UpdateMessage(Box::new(data.map_or_default(RawCallbackData::into_message)))
            }
            InteractionCallbackType::ApplicationCommandAutocompleteResult => {
                let data = data.ok_or_else(missing)?;
                Self::AutocompleteResult(AutocompleteCallbackData {
                    choices: AutocompleteChoices::from_raw(data.choices)?,
                })
            }
            InteractionCallbackType::Modal => {
                let data = data.ok_or_else(missing)?;
                Self::Modal(Box::new(ModalCallbackData {
                    custom_id: data
                        .custom_id
                        .ok_or_else(|| D::Error::missing_field("custom_id"))?,
                    title: data.title.ok_or_else(|| D::Error::missing_field("title"))?,
                    components: data.components.unwrap_or_default(),
                }))
            }
            InteractionCallbackType::PremiumRequired => Self::PremiumRequired,
            InteractionCallbackType::LaunchActivity => Self::LaunchActivity,
            kind @ InteractionCallbackType::Unknown(_) => Self::Unknown(kind),
        })
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct MessageCallbackData {
    pub tts: bool,

    pub content: Option<CompactString>,

    pub embeds: Option<Vec<Embed>>,

    pub allowed_mentions: Option<AllowedMentions>,

    pub flags: Option<MessageFlags>,

    pub components: Option<Vec<Component>>,

    pub attachments: Option<Vec<AttachmentRequest>>,

    pub poll: Option<CreatePoll>,

    pub files: Vec<FileUpload>,
}

impl Serialize for MessageCallbackData {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            tts: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "Option::is_none")]
            embeds: Option<&'a [Embed]>,
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_mentions: Option<&'a AllowedMentions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            flags: Option<MessageFlags>,
            #[serde(skip_serializing_if = "Option::is_none")]
            components: Option<&'a [Component]>,
            #[serde(skip_serializing_if = "Attachments::is_absent")]
            attachments: Attachments<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            poll: Option<&'a CreatePoll>,
        }

        let Self {
            tts,
            content,
            embeds,
            allowed_mentions,
            flags,
            components,
            attachments,
            poll,
            files,
        } = self;
        Wire {
            tts: *tts,
            content: content.as_ref(),
            embeds: embeds.as_deref(),
            allowed_mentions: allowed_mentions.as_ref(),
            flags: *flags,
            components: components.as_deref(),
            attachments: Attachments::new(attachments.as_deref(), files),
            poll: poll.as_ref(),
        }
        .serialize(serializer)
    }
}

setters::message_body!(MessageCallbackData {
    content,
    embeds,
    components,
    allowed_mentions,
    flags,
    ephemeral,
    suppress_notifications,
    files,
    attachments,
    poll,
    tts,
});

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct DeferredCallbackData {
    pub flags: MessageFlags,
}

impl DeferredCallbackData {
    #[must_use]
    pub const fn new(flags: MessageFlags) -> Self {
        Self { flags }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct AutocompleteCallbackData {
    pub choices: AutocompleteChoices,
}

impl AutocompleteCallbackData {
    #[must_use]
    pub const fn new(choices: AutocompleteChoices) -> Self {
        Self { choices }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AutocompleteChoices {
    String(Vec<Choice<CompactString>>),

    Integer(Vec<Choice<i64>>),

    Number(Vec<Choice<f64>>),
}

impl AutocompleteChoices {
    fn from_raw<E: DeError>(choices: Vec<Choice<RawValue>>) -> Result<Self, E> {
        Ok(match choices.first().map(|choice| &choice.value) {
            None | Some(RawValue::String(_)) => {
                Self::String(convert_choices(choices, RawValue::into_string)?)
            }
            Some(_)
                if choices
                    .iter()
                    .any(|choice| matches!(choice.value, RawValue::Float(_))) =>
            {
                Self::Number(convert_choices(choices, RawValue::into_number)?)
            }
            Some(_) => Self::Integer(convert_choices(choices, RawValue::into_integer)?),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ModalCallbackData {
    pub custom_id: CompactString,

    pub title: CompactString,

    pub components: Vec<Component>,
}

impl ModalCallbackData {
    #[must_use]
    pub fn new(
        custom_id: impl Into<CompactString>,
        title: impl Into<CompactString>,
        components: Vec<Component>,
    ) -> Self {
        Self {
            custom_id: custom_id.into(),
            title: title.into(),
            components,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InteractionCallbackResponse {
    pub interaction: InteractionCallback,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resource: Option<CallbackResource>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InteractionCallback {
    pub id: InteractionId,

    #[serde(rename = "type")]
    pub kind: InteractionType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_instance_id: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_message_id: Option<MessageId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_message_loading: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_message_ephemeral: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CallbackResource {
    #[serde(rename = "type")]
    pub kind: InteractionCallbackType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activity_instance: Option<ActivityInstanceResource>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<Box<Message>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityInstanceResource {
    pub id: CompactString,
}

impl From<crate::message::Embed> for MessageCallbackData {
    fn from(embed: crate::message::Embed) -> Self {
        Self::default().embed(embed)
    }
}

impl From<Vec<crate::message::Embed>> for MessageCallbackData {
    fn from(embeds: Vec<crate::message::Embed>) -> Self {
        Self::default().embeds(embeds)
    }
}
