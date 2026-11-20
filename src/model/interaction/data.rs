use std::collections::BTreeMap;

use compact_str::{CompactString, ToCompactString};
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

use super::{InteractionChannel, InteractionMember, InteractionType};
use crate::model::{
    application_command::{CommandOptionType, CommandType},
    component::ComponentType,
    guild::Role,
    id::{
        AttachmentId, ChannelId, CommandId, GenericId, GuildId, Id, MessageId, RoleId, UserId,
        marker::{ChannelMarker, GenericMarker, RoleMarker, UserMarker},
    },
    message::{Attachment, Message},
    user::User,
};

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum InteractionData {
    Ping,

    ApplicationCommand(Box<CommandData>),

    MessageComponent(Box<ComponentData>),

    Autocomplete(Box<CommandData>),

    ModalSubmit(Box<ModalSubmitData>),

    Unknown(InteractionType),
}

impl InteractionData {
    #[must_use]
    pub const fn kind(&self) -> InteractionType {
        match self {
            Self::Ping => InteractionType::Ping,
            Self::ApplicationCommand(_) => InteractionType::ApplicationCommand,
            Self::MessageComponent(_) => InteractionType::MessageComponent,
            Self::Autocomplete(_) => InteractionType::ApplicationCommandAutocomplete,
            Self::ModalSubmit(_) => InteractionType::ModalSubmit,
            Self::Unknown(kind) => *kind,
        }
    }

    #[must_use]
    pub fn command(&self) -> Option<&CommandData> {
        match self {
            Self::ApplicationCommand(data) | Self::Autocomplete(data) => Some(data),
            _ => None,
        }
    }

    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        match self {
            Self::MessageComponent(data) => Some(&data.custom_id),
            Self::ModalSubmit(data) => Some(&data.custom_id),
            _ => None,
        }
    }

    #[must_use]
    pub fn resolved(&self) -> Option<&Resolved> {
        match self {
            Self::ApplicationCommand(data) | Self::Autocomplete(data) => data.resolved.as_deref(),
            Self::MessageComponent(data) => data.resolved.as_deref(),
            Self::ModalSubmit(data) => data.resolved.as_deref(),
            Self::Ping | Self::Unknown(_) => None,
        }
    }

    pub(super) fn as_ref(&self) -> Option<DataRef<'_>> {
        match self {
            Self::ApplicationCommand(data) | Self::Autocomplete(data) => {
                Some(DataRef::Command(data))
            }
            Self::MessageComponent(data) => Some(DataRef::Component(data)),
            Self::ModalSubmit(data) => Some(DataRef::ModalSubmit(data)),
            Self::Ping | Self::Unknown(_) => None,
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub(super) enum DataRef<'a> {
    Command(&'a CommandData),
    Component(&'a ComponentData),
    ModalSubmit(&'a ModalSubmitData),
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CommandData {
    pub id: CommandId,

    pub name: CompactString,

    #[serde(rename = "type")]
    pub kind: CommandType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Box<Resolved>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandDataOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_id: Option<GenericId>,
}

impl CommandData {
    #[must_use]
    pub fn focused(&self) -> Option<&CommandDataOption> {
        fn find(options: &[CommandDataOption]) -> Option<&CommandDataOption> {
            options.iter().find_map(|option| match &option.value {
                CommandOptionValue::Focused { .. } => Some(option),
                CommandOptionValue::SubCommand(options)
                | CommandOptionValue::SubCommandGroup(options) => find(options),
                _ => None,
            })
        }

        find(&self.options)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CommandDataOption {
    pub name: CompactString,

    pub value: CommandOptionValue,
}

impl CommandDataOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, value: CommandOptionValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum CommandOptionValue {
    SubCommand(Vec<CommandDataOption>),

    SubCommandGroup(Vec<CommandDataOption>),

    String(CompactString),

    Integer(i64),

    Boolean(bool),

    User(UserId),

    Channel(ChannelId),

    Role(RoleId),

    Mentionable(GenericId),

    Number(f64),

    Attachment(AttachmentId),

    Focused {
        kind: CommandOptionType,

        value: CompactString,
    },

    Unknown(CommandOptionType),
}

impl CommandOptionValue {
    #[must_use]
    pub const fn kind(&self) -> CommandOptionType {
        match self {
            Self::SubCommand(_) => CommandOptionType::SubCommand,
            Self::SubCommandGroup(_) => CommandOptionType::SubCommandGroup,
            Self::String(_) => CommandOptionType::String,
            Self::Integer(_) => CommandOptionType::Integer,
            Self::Boolean(_) => CommandOptionType::Boolean,
            Self::User(_) => CommandOptionType::User,
            Self::Channel(_) => CommandOptionType::Channel,
            Self::Role(_) => CommandOptionType::Role,
            Self::Mentionable(_) => CommandOptionType::Mentionable,
            Self::Number(_) => CommandOptionType::Number,
            Self::Attachment(_) => CommandOptionType::Attachment,
            Self::Focused { kind, .. } | Self::Unknown(kind) => *kind,
        }
    }
}

enum RawOptionValue {
    String(CompactString),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl<'de> Deserialize<'de> for RawOptionValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RawOptionValueVisitor;

        impl Visitor<'_> for RawOptionValueVisitor {
            type Value = RawOptionValue;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a string, a number or a boolean")
            }

            fn visit_bool<E: DeError>(self, value: bool) -> Result<RawOptionValue, E> {
                Ok(RawOptionValue::Boolean(value))
            }

            fn visit_i64<E: DeError>(self, value: i64) -> Result<RawOptionValue, E> {
                Ok(RawOptionValue::Integer(value))
            }

            fn visit_u64<E: DeError>(self, value: u64) -> Result<RawOptionValue, E> {
                i64::try_from(value)
                    .map(RawOptionValue::Integer)
                    .map_err(|_| E::invalid_value(Unexpected::Unsigned(value), &self))
            }

            fn visit_f64<E: DeError>(self, value: f64) -> Result<RawOptionValue, E> {
                Ok(RawOptionValue::Float(value))
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<RawOptionValue, E> {
                Ok(RawOptionValue::String(value.into()))
            }
        }

        deserializer.deserialize_any(RawOptionValueVisitor)
    }
}

impl RawOptionValue {
    fn unexpected(&self) -> Unexpected<'_> {
        match self {
            Self::String(value) => Unexpected::Str(value),
            Self::Integer(value) => Unexpected::Signed(*value),
            Self::Float(value) => Unexpected::Float(*value),
            Self::Boolean(value) => Unexpected::Bool(*value),
        }
    }

    fn into_text(self) -> CompactString {
        match self {
            Self::String(value) => value,
            Self::Integer(value) => value.to_compact_string(),
            Self::Float(value) => value.to_compact_string(),
            Self::Boolean(value) => value.to_compact_string(),
        }
    }

    fn into_string<E: DeError>(self) -> Result<CompactString, E> {
        match self {
            Self::String(value) => Ok(value),
            other => Err(E::invalid_type(other.unexpected(), &"a string")),
        }
    }

    fn into_integer<E: DeError>(self) -> Result<i64, E> {
        match self {
            Self::Integer(value) => Ok(value),
            Self::String(ref text) => text
                .parse()
                .map_err(|_| E::invalid_type(self.unexpected(), &"an integer")),
            other => Err(E::invalid_type(other.unexpected(), &"an integer")),
        }
    }

    fn into_number<E: DeError>(self) -> Result<f64, E> {
        match self {
            Self::Float(value) => Ok(value),
            #[expect(
                clippy::cast_precision_loss,
                reason = "number options are doubles; Discord limits them to ±2^53"
            )]
            Self::Integer(value) => Ok(value as f64),
            Self::String(ref text) => text
                .parse()
                .map_err(|_| E::invalid_type(self.unexpected(), &"a number")),
            Self::Boolean(_) => Err(E::invalid_type(self.unexpected(), &"a number")),
        }
    }

    fn into_boolean<E: DeError>(self) -> Result<bool, E> {
        match self {
            Self::Boolean(value) => Ok(value),
            other => Err(E::invalid_type(other.unexpected(), &"a boolean")),
        }
    }

    fn into_id<T, E: DeError>(self) -> Result<Id<T>, E> {
        match self {
            Self::String(ref text) => text
                .parse()
                .map_err(|_| E::invalid_value(self.unexpected(), &"a snowflake ID")),
            Self::Integer(value) => u64::try_from(value)
                .ok()
                .and_then(Id::new_checked)
                .ok_or_else(|| E::invalid_value(self.unexpected(), &"a snowflake ID")),
            other => Err(E::invalid_type(other.unexpected(), &"a snowflake ID")),
        }
    }
}

#[derive(Deserialize)]
struct RawDataOption {
    name: CompactString,
    #[serde(rename = "type")]
    kind: CommandOptionType,
    #[serde(default)]
    value: Option<RawOptionValue>,
    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    options: Vec<CommandDataOption>,
    #[serde(default)]
    focused: bool,
}

impl<'de> Deserialize<'de> for CommandDataOption {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawDataOption::deserialize(deserializer)?;
        let kind = raw.kind;
        let value = match kind {
            CommandOptionType::SubCommand => CommandOptionValue::SubCommand(raw.options),
            CommandOptionType::SubCommandGroup => CommandOptionValue::SubCommandGroup(raw.options),
            CommandOptionType::Unknown(_) => CommandOptionValue::Unknown(kind),
            _ => {
                let value = raw.value.ok_or_else(|| D::Error::missing_field("value"))?;
                if raw.focused {
                    CommandOptionValue::Focused {
                        kind,
                        value: value.into_text(),
                    }
                } else {
                    match kind {
                        CommandOptionType::String => {
                            CommandOptionValue::String(value.into_string()?)
                        }
                        CommandOptionType::Integer => {
                            CommandOptionValue::Integer(value.into_integer()?)
                        }
                        CommandOptionType::Boolean => {
                            CommandOptionValue::Boolean(value.into_boolean()?)
                        }
                        CommandOptionType::User => CommandOptionValue::User(value.into_id()?),
                        CommandOptionType::Channel => CommandOptionValue::Channel(value.into_id()?),
                        CommandOptionType::Role => CommandOptionValue::Role(value.into_id()?),
                        CommandOptionType::Mentionable => {
                            CommandOptionValue::Mentionable(value.into_id()?)
                        }
                        CommandOptionType::Number => {
                            CommandOptionValue::Number(value.into_number()?)
                        }
                        _ => CommandOptionValue::Attachment(value.into_id()?),
                    }
                }
            }
        };
        Ok(Self {
            name: raw.name,
            value,
        })
    }
}

impl Serialize for CommandDataOption {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum Value<'a> {
            String(&'a str),
            Integer(i64),
            Number(f64),
            Boolean(bool),
            Id(GenericId),
        }

        #[derive(Serialize)]
        struct Flat<'a> {
            name: &'a str,
            #[serde(rename = "type")]
            kind: CommandOptionType,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<Value<'a>>,
            #[serde(skip_serializing_if = "Option::is_none")]
            options: Option<&'a [CommandDataOption]>,
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            focused: bool,
        }

        let mut flat = Flat {
            name: &self.name,
            kind: self.value.kind(),
            value: None,
            options: None,
            focused: false,
        };
        match &self.value {
            CommandOptionValue::SubCommand(options)
            | CommandOptionValue::SubCommandGroup(options) => flat.options = Some(options),
            CommandOptionValue::String(value) => flat.value = Some(Value::String(value)),
            CommandOptionValue::Integer(value) => flat.value = Some(Value::Integer(*value)),
            CommandOptionValue::Boolean(value) => flat.value = Some(Value::Boolean(*value)),
            CommandOptionValue::User(id) => flat.value = Some(Value::Id(id.cast())),
            CommandOptionValue::Channel(id) => flat.value = Some(Value::Id(id.cast())),
            CommandOptionValue::Role(id) => flat.value = Some(Value::Id(id.cast())),
            CommandOptionValue::Mentionable(id) => flat.value = Some(Value::Id(*id)),
            CommandOptionValue::Number(value) => flat.value = Some(Value::Number(*value)),
            CommandOptionValue::Attachment(id) => flat.value = Some(Value::Id(id.cast())),
            CommandOptionValue::Focused { value, .. } => {
                flat.value = Some(Value::String(value));
                flat.focused = true;
            }
            CommandOptionValue::Unknown(_) => {}
        }
        flat.serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ComponentData {
    pub custom_id: CompactString,

    pub component_type: ComponentType,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub values: Vec<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Box<Resolved>>,
}

impl ComponentData {
    pub fn ids<T>(&self) -> impl Iterator<Item = Id<T>> + '_ {
        self.values.iter().filter_map(|value| value.parse().ok())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ModalSubmitData {
    pub custom_id: CompactString,

    pub components: Vec<SubmittedComponent>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved: Option<Box<Resolved>>,
}

impl ModalSubmitData {
    #[must_use]
    pub fn get(&self, custom_id: &str) -> Option<&SubmittedComponent> {
        fn find<'a>(
            components: &'a [SubmittedComponent],
            custom_id: &str,
        ) -> Option<&'a SubmittedComponent> {
            components.iter().find_map(|component| match component {
                SubmittedComponent::ActionRow(row) => find(&row.components, custom_id),
                SubmittedComponent::Label(label) => {
                    find(core::slice::from_ref(&*label.component), custom_id)
                }
                other => (other.custom_id() == Some(custom_id)).then_some(other),
            })
        }

        find(&self.components, custom_id)
    }

    #[must_use]
    pub fn text_value(&self, custom_id: &str) -> Option<&str> {
        match self.get(custom_id)? {
            SubmittedComponent::TextInput(input) => Some(&input.value),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "ModalSubmitComponent")]
pub enum SubmittedComponent {
    ActionRow(SubmittedActionRow),

    TextInput(SubmittedTextInput),

    StringSelect(SubmittedSelect),

    UserSelect(SubmittedEntitySelect<UserMarker>),

    RoleSelect(SubmittedEntitySelect<RoleMarker>),

    MentionableSelect(SubmittedEntitySelect<GenericMarker>),

    ChannelSelect(SubmittedEntitySelect<ChannelMarker>),

    TextDisplay(SubmittedTextDisplay),

    Label(SubmittedLabel),

    FileUpload(SubmittedFileUpload),

    RadioGroup(SubmittedRadioGroup),

    CheckboxGroup(SubmittedSelect),

    Checkbox(SubmittedCheckbox),

    Unknown(UnknownSubmittedComponent),
}

impl SubmittedComponent {
    #[must_use]
    pub const fn kind(&self) -> ComponentType {
        match self {
            Self::ActionRow(_) => ComponentType::ActionRow,
            Self::TextInput(_) => ComponentType::TextInput,
            Self::StringSelect(_) => ComponentType::StringSelect,
            Self::UserSelect(_) => ComponentType::UserSelect,
            Self::RoleSelect(_) => ComponentType::RoleSelect,
            Self::MentionableSelect(_) => ComponentType::MentionableSelect,
            Self::ChannelSelect(_) => ComponentType::ChannelSelect,
            Self::TextDisplay(_) => ComponentType::TextDisplay,
            Self::Label(_) => ComponentType::Label,
            Self::FileUpload(_) => ComponentType::FileUpload,
            Self::RadioGroup(_) => ComponentType::RadioGroup,
            Self::CheckboxGroup(_) => ComponentType::CheckboxGroup,
            Self::Checkbox(_) => ComponentType::Checkbox,
            Self::Unknown(component) => component.kind,
        }
    }

    #[must_use]
    pub const fn id(&self) -> Option<u32> {
        match self {
            Self::ActionRow(component) => component.id,
            Self::TextInput(component) => component.id,
            Self::StringSelect(component) | Self::CheckboxGroup(component) => component.id,
            Self::UserSelect(component) => component.id,
            Self::RoleSelect(component) => component.id,
            Self::MentionableSelect(component) => component.id,
            Self::ChannelSelect(component) => component.id,
            Self::TextDisplay(component) => component.id,
            Self::Label(component) => component.id,
            Self::FileUpload(component) => component.id,
            Self::RadioGroup(component) => component.id,
            Self::Checkbox(component) => component.id,
            Self::Unknown(component) => component.id,
        }
    }

    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        match self {
            Self::TextInput(component) => Some(&component.custom_id),
            Self::StringSelect(component) | Self::CheckboxGroup(component) => {
                Some(&component.custom_id)
            }
            Self::UserSelect(component) => Some(&component.custom_id),
            Self::RoleSelect(component) => Some(&component.custom_id),
            Self::MentionableSelect(component) => Some(&component.custom_id),
            Self::ChannelSelect(component) => Some(&component.custom_id),
            Self::FileUpload(component) => Some(&component.custom_id),
            Self::RadioGroup(component) => Some(&component.custom_id),
            Self::Checkbox(component) => Some(&component.custom_id),
            Self::ActionRow(_) | Self::TextDisplay(_) | Self::Label(_) | Self::Unknown(_) => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedActionRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub components: Vec<SubmittedComponent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedTextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub custom_id: CompactString,

    pub value: CompactString,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub custom_id: CompactString,

    pub values: Vec<CompactString>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(bound = "")]
#[non_exhaustive]
pub struct SubmittedEntitySelect<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub custom_id: CompactString,

    pub values: Vec<Id<T>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedTextDisplay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub component: Box<SubmittedComponent>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedFileUpload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub custom_id: CompactString,

    pub values: Vec<AttachmentId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedRadioGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub custom_id: CompactString,

    pub value: Option<CompactString>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SubmittedCheckbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,

    pub custom_id: CompactString,

    pub value: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UnknownSubmittedComponent {
    #[serde(skip)]
    pub kind: ComponentType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
}

enum RawSubmittedValue {
    Text(CompactString),
    Boolean(bool),
}

impl<'de> Deserialize<'de> for RawSubmittedValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RawSubmittedValueVisitor;

        impl Visitor<'_> for RawSubmittedValueVisitor {
            type Value = RawSubmittedValue;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a string or a boolean")
            }

            fn visit_bool<E: DeError>(self, value: bool) -> Result<RawSubmittedValue, E> {
                Ok(RawSubmittedValue::Boolean(value))
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<RawSubmittedValue, E> {
                Ok(RawSubmittedValue::Text(value.into()))
            }
        }

        deserializer.deserialize_any(RawSubmittedValueVisitor)
    }
}

#[derive(Deserialize)]
struct RawSubmittedComponent {
    #[serde(rename = "type")]
    kind: ComponentType,
    #[serde(default)]
    id: Option<u32>,
    #[serde(default)]
    custom_id: Option<CompactString>,
    #[serde(default)]
    value: Option<RawSubmittedValue>,
    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    values: Vec<CompactString>,
    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    components: Vec<SubmittedComponent>,
    #[serde(default)]
    component: Option<Box<SubmittedComponent>>,
}

impl RawSubmittedComponent {
    fn custom_id<E: DeError>(&mut self) -> Result<CompactString, E> {
        self.custom_id
            .take()
            .ok_or_else(|| E::missing_field("custom_id"))
    }

    fn entity_select<T, E: DeError>(mut self) -> Result<SubmittedEntitySelect<T>, E> {
        Ok(SubmittedEntitySelect {
            id: self.id,
            custom_id: self.custom_id()?,
            values: parse_ids(&self.values)?,
        })
    }
}

fn parse_ids<T, E: DeError>(values: &[CompactString]) -> Result<Vec<Id<T>>, E> {
    values
        .iter()
        .map(|value| {
            value
                .parse()
                .map_err(|_| E::invalid_value(Unexpected::Str(value), &"a snowflake ID"))
        })
        .collect()
}

impl<'de> Deserialize<'de> for SubmittedComponent {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut raw = RawSubmittedComponent::deserialize(deserializer)?;
        let id = raw.id;
        Ok(match raw.kind {
            ComponentType::ActionRow => Self::ActionRow(SubmittedActionRow {
                id,
                components: raw.components,
            }),
            ComponentType::TextInput => Self::TextInput(SubmittedTextInput {
                id,
                custom_id: raw.custom_id()?,
                value: match raw.value {
                    Some(RawSubmittedValue::Text(value)) => value,
                    Some(RawSubmittedValue::Boolean(value)) => {
                        return Err(D::Error::invalid_type(Unexpected::Bool(value), &"a string"));
                    }
                    None => return Err(D::Error::missing_field("value")),
                },
            }),
            kind @ (ComponentType::StringSelect | ComponentType::CheckboxGroup) => {
                let select = SubmittedSelect {
                    id,
                    custom_id: raw.custom_id()?,
                    values: raw.values,
                };
                if kind == ComponentType::StringSelect {
                    Self::StringSelect(select)
                } else {
                    Self::CheckboxGroup(select)
                }
            }
            ComponentType::UserSelect => Self::UserSelect(raw.entity_select()?),
            ComponentType::RoleSelect => Self::RoleSelect(raw.entity_select()?),
            ComponentType::MentionableSelect => Self::MentionableSelect(raw.entity_select()?),
            ComponentType::ChannelSelect => Self::ChannelSelect(raw.entity_select()?),
            ComponentType::TextDisplay => Self::TextDisplay(SubmittedTextDisplay { id }),
            ComponentType::Label => Self::Label(SubmittedLabel {
                id,
                component: raw
                    .component
                    .ok_or_else(|| D::Error::missing_field("component"))?,
            }),
            ComponentType::FileUpload => Self::FileUpload(SubmittedFileUpload {
                id,
                custom_id: raw.custom_id()?,
                values: parse_ids(&raw.values)?,
            }),
            ComponentType::RadioGroup => Self::RadioGroup(SubmittedRadioGroup {
                id,
                custom_id: raw.custom_id()?,
                value: match raw.value {
                    Some(RawSubmittedValue::Text(value)) => Some(value),
                    Some(RawSubmittedValue::Boolean(value)) => {
                        return Err(D::Error::invalid_type(Unexpected::Bool(value), &"a string"));
                    }
                    None => None,
                },
            }),
            ComponentType::Checkbox => Self::Checkbox(SubmittedCheckbox {
                id,
                custom_id: raw.custom_id()?,
                value: match raw.value {
                    Some(RawSubmittedValue::Boolean(value)) => value,
                    Some(RawSubmittedValue::Text(value)) => {
                        return Err(D::Error::invalid_type(
                            Unexpected::Str(&value),
                            &"a boolean",
                        ));
                    }
                    None => false,
                },
            }),
            kind => Self::Unknown(UnknownSubmittedComponent { kind, id }),
        })
    }
}

#[derive(Serialize)]
struct Tagged<'a, T> {
    #[serde(rename = "type")]
    kind: ComponentType,
    #[serde(flatten)]
    component: &'a T,
}

impl Serialize for SubmittedComponent {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let kind = self.kind();
        match self {
            Self::ActionRow(component) => Tagged { kind, component }.serialize(serializer),
            Self::TextInput(component) => Tagged { kind, component }.serialize(serializer),
            Self::StringSelect(component) | Self::CheckboxGroup(component) => {
                Tagged { kind, component }.serialize(serializer)
            }
            Self::UserSelect(component) => Tagged { kind, component }.serialize(serializer),
            Self::RoleSelect(component) => Tagged { kind, component }.serialize(serializer),
            Self::MentionableSelect(component) => Tagged { kind, component }.serialize(serializer),
            Self::ChannelSelect(component) => Tagged { kind, component }.serialize(serializer),
            Self::TextDisplay(component) => Tagged { kind, component }.serialize(serializer),
            Self::Label(component) => Tagged { kind, component }.serialize(serializer),
            Self::FileUpload(component) => Tagged { kind, component }.serialize(serializer),
            Self::RadioGroup(component) => Tagged { kind, component }.serialize(serializer),
            Self::Checkbox(component) => Tagged { kind, component }.serialize(serializer),
            Self::Unknown(component) => Tagged { kind, component }.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Resolved {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub users: BTreeMap<UserId, Box<User>>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub members: BTreeMap<UserId, Box<InteractionMember>>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub roles: BTreeMap<RoleId, Box<Role>>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub channels: BTreeMap<ChannelId, Box<InteractionChannel>>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub messages: BTreeMap<MessageId, Box<Message>>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub attachments: BTreeMap<AttachmentId, Box<Attachment>>,
}

#[derive(Deserialize)]
pub(super) struct RawData {
    #[serde(default)]
    id: Option<CommandId>,
    #[serde(default)]
    name: Option<CompactString>,
    #[serde(default, rename = "type")]
    kind: Option<CommandType>,
    #[serde(default)]
    resolved: Option<Box<Resolved>>,
    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    options: Vec<CommandDataOption>,
    #[serde(default)]
    guild_id: Option<GuildId>,
    #[serde(default)]
    target_id: Option<GenericId>,
    #[serde(default)]
    custom_id: Option<CompactString>,
    #[serde(default)]
    component_type: Option<ComponentType>,
    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    values: Vec<CompactString>,
    #[serde(
        default,
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    components: Vec<SubmittedComponent>,
}

impl RawData {
    pub(super) fn into_data<E: DeError>(
        data: Option<Self>,
        kind: InteractionType,
    ) -> Result<InteractionData, E> {
        let missing = || E::missing_field("data");
        Ok(match kind {
            InteractionType::Ping => InteractionData::Ping,
            InteractionType::ApplicationCommand => {
                InteractionData::ApplicationCommand(Box::new(data.ok_or_else(missing)?.command()?))
            }
            InteractionType::ApplicationCommandAutocomplete => {
                InteractionData::Autocomplete(Box::new(data.ok_or_else(missing)?.command()?))
            }
            InteractionType::MessageComponent => {
                let data = data.ok_or_else(missing)?;
                InteractionData::MessageComponent(Box::new(ComponentData {
                    custom_id: data
                        .custom_id
                        .ok_or_else(|| E::missing_field("custom_id"))?,
                    component_type: data
                        .component_type
                        .ok_or_else(|| E::missing_field("component_type"))?,
                    values: data.values,
                    resolved: data.resolved,
                }))
            }
            InteractionType::ModalSubmit => {
                let data = data.ok_or_else(missing)?;
                InteractionData::ModalSubmit(Box::new(ModalSubmitData {
                    custom_id: data
                        .custom_id
                        .ok_or_else(|| E::missing_field("custom_id"))?,
                    components: data.components,
                    resolved: data.resolved,
                }))
            }
            InteractionType::Unknown(_) => InteractionData::Unknown(kind),
        })
    }

    fn command<E: DeError>(self) -> Result<CommandData, E> {
        Ok(CommandData {
            id: self.id.ok_or_else(|| E::missing_field("id"))?,
            name: self.name.ok_or_else(|| E::missing_field("name"))?,
            kind: self.kind.unwrap_or_default(),
            resolved: self.resolved,
            options: self.options,
            guild_id: self.guild_id,
            target_id: self.target_id,
        })
    }
}
