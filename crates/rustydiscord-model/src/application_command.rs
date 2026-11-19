use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

use crate::{
    application::IntegrationType,
    channel::ChannelType,
    id::{ApplicationId, CommandId, CommandVersionId, GenericId, GuildId},
    interaction::InteractionContextType,
    macros::int_enum,
    permissions::Permissions,
    util::Locale,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationCommand {
    pub id: CommandId,

    #[serde(rename = "type", default)]
    pub kind: CommandType,

    pub application_id: ApplicationId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    #[serde(default)]
    pub description: CompactString,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub options: Vec<CommandOption>,

    #[serde(default)]
    pub default_member_permissions: Option<Permissions>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dm_permission: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_permission: Option<bool>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub nsfw: bool,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub integration_types: Vec<IntegrationType>,

    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub contexts: Vec<InteractionContextType>,

    pub version: CommandVersionId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub handler: Option<EntryPointHandlerType>,
}

int_enum! {



    #[doc(alias = "ApplicationCommandType")]
    #[derive(Default)]
    pub enum CommandType: u8 {

        #[default]
        ChatInput = 1,

        User = 2,

        Message = 3,

        PrimaryEntryPoint = 4,
    }
}

int_enum! {



    #[doc(alias = "EntryPointCommandHandlerType")]
    pub enum EntryPointHandlerType: u8 {

        AppHandler = 1,

        DiscordLaunchActivity = 2,
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
#[doc(alias = "ApplicationCommandOption")]
pub enum CommandOption {
    SubCommand(SubCommandOption),

    SubCommandGroup(SubCommandOption),

    String(StringOption),

    Integer(IntegerOption),

    Boolean(BasicOption),

    User(BasicOption),

    Channel(ChannelOption),

    Role(BasicOption),

    Mentionable(BasicOption),

    Number(NumberOption),

    Attachment(AttachmentOption),

    Unknown(UnknownOption),
}

#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct OptionInfo<'a> {
    pub name: &'a str,

    pub name_localizations: &'a BTreeMap<Locale, CompactString>,

    pub description: &'a str,

    pub description_localizations: &'a BTreeMap<Locale, CompactString>,

    pub name_localized: Option<&'a str>,

    pub description_localized: Option<&'a str>,
}

macro_rules! common_fields {
    ($self:ident) => {
        match $self {
            Self::SubCommand(option) | Self::SubCommandGroup(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::String(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::Integer(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::Boolean(option)
            | Self::User(option)
            | Self::Role(option)
            | Self::Mentionable(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::Channel(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::Number(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::Attachment(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
            Self::Unknown(option) => OptionInfo {
                name: &option.name,
                name_localizations: &option.name_localizations,
                description: &option.description,
                description_localizations: &option.description_localizations,
                name_localized: option.name_localized.as_deref(),
                description_localized: option.description_localized.as_deref(),
            },
        }
    };
}

impl CommandOption {
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
            Self::Unknown(option) => option.kind,
        }
    }

    #[must_use]
    pub fn common(&self) -> OptionInfo<'_> {
        common_fields!(self)
    }

    #[must_use]
    pub fn name(&self) -> &str {
        self.common().name
    }

    #[must_use]
    pub fn options(&self) -> &[Self] {
        match self {
            Self::SubCommand(option) | Self::SubCommandGroup(option) => &option.options,
            _ => &[],
        }
    }
}

int_enum! {



    #[doc(alias = "ApplicationCommandOptionType")]
    pub enum CommandOptionType: u8 {

        SubCommand = 1,

        SubCommandGroup = 2,

        String = 3,

        Integer = 4,

        Boolean = 5,

        User = 6,

        Channel = 7,

        Role = 8,

        Mentionable = 9,

        Number = 10,

        Attachment = 11,
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct SubCommandOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOption>,
}

impl SubCommandOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            options: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct StringOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<Choice<CompactString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub autocomplete: bool,
}

impl StringOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            required: false,
            choices: Vec::new(),
            min_length: None,
            max_length: None,
            autocomplete: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct IntegerOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<Choice<i64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i64>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub autocomplete: bool,
}

impl IntegerOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            required: false,
            choices: Vec::new(),
            min_value: None,
            max_value: None,
            autocomplete: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct NumberOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub choices: Vec<Choice<f64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_value: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_value: Option<f64>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub autocomplete: bool,
}

impl NumberOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            required: false,
            choices: Vec::new(),
            min_value: None,
            max_value: None,
            autocomplete: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct BasicOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,
}

impl BasicOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            required: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ChannelOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub channel_types: Vec<ChannelType>,
}

impl ChannelOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            required: false,
            channel_types: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct AttachmentOption {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_types: Vec<CompactString>,
}

impl AttachmentOption {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
            name_localized: None,
            description_localized: None,
            required: false,
            file_types: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UnknownOption {
    #[serde(skip)]
    pub kind: CommandOptionType,

    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_localized: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub required: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "ApplicationCommandOptionChoice")]
pub struct Choice<T> {
    pub name: CompactString,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub value: T,
}

impl<T> Choice<T> {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, value: T) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum RawValue {
    String(CompactString),
    Integer(i64),
    Float(f64),
}

impl<'de> Deserialize<'de> for RawValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct RawValueVisitor;

        impl Visitor<'_> for RawValueVisitor {
            type Value = RawValue;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("a string or a number")
            }

            fn visit_i64<E: DeError>(self, value: i64) -> Result<RawValue, E> {
                Ok(RawValue::Integer(value))
            }

            fn visit_u64<E: DeError>(self, value: u64) -> Result<RawValue, E> {
                i64::try_from(value)
                    .map(RawValue::Integer)
                    .map_err(|_| E::invalid_value(Unexpected::Unsigned(value), &self))
            }

            fn visit_f64<E: DeError>(self, value: f64) -> Result<RawValue, E> {
                Ok(RawValue::Float(value))
            }

            fn visit_str<E: DeError>(self, value: &str) -> Result<RawValue, E> {
                Ok(RawValue::String(value.into()))
            }
        }

        deserializer.deserialize_any(RawValueVisitor)
    }
}

impl RawValue {
    pub(crate) fn into_string<E: DeError>(self) -> Result<CompactString, E> {
        match self {
            Self::String(value) => Ok(value),
            Self::Integer(value) => Err(E::invalid_type(Unexpected::Signed(value), &"a string")),
            Self::Float(value) => Err(E::invalid_type(Unexpected::Float(value), &"a string")),
        }
    }

    pub(crate) fn into_integer<E: DeError>(self) -> Result<i64, E> {
        match self {
            Self::Integer(value) => Ok(value),
            Self::String(value) => Err(E::invalid_type(Unexpected::Str(&value), &"an integer")),
            Self::Float(value) => Err(E::invalid_type(Unexpected::Float(value), &"an integer")),
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        reason = "Discord limits command numbers to ±2^53, which f64 represents exactly"
    )]
    pub(crate) fn into_number<E: DeError>(self) -> Result<f64, E> {
        match self {
            Self::Integer(value) => Ok(value as f64),
            Self::Float(value) => Ok(value),
            Self::String(value) => Err(E::invalid_type(Unexpected::Str(&value), &"a number")),
        }
    }
}

pub(crate) fn convert_choices<T, E: DeError>(
    choices: Vec<Choice<RawValue>>,
    convert: impl Fn(RawValue) -> Result<T, E>,
) -> Result<Vec<Choice<T>>, E> {
    choices
        .into_iter()
        .map(|choice| {
            Ok(Choice {
                name: choice.name,
                name_localizations: choice.name_localizations,
                value: convert(choice.value)?,
            })
        })
        .collect()
}

#[derive(Deserialize)]
struct RawOption {
    #[serde(rename = "type")]
    kind: CommandOptionType,
    name: CompactString,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    name_localizations: BTreeMap<Locale, CompactString>,
    #[serde(default)]
    description: CompactString,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    description_localizations: BTreeMap<Locale, CompactString>,
    #[serde(default)]
    name_localized: Option<CompactString>,
    #[serde(default)]
    description_localized: Option<CompactString>,
    #[serde(default)]
    required: bool,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    choices: Vec<Choice<RawValue>>,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    options: Vec<CommandOption>,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    channel_types: Vec<ChannelType>,
    #[serde(default)]
    min_value: Option<RawValue>,
    #[serde(default)]
    max_value: Option<RawValue>,
    #[serde(default)]
    min_length: Option<u16>,
    #[serde(default)]
    max_length: Option<u16>,
    #[serde(default)]
    autocomplete: bool,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    file_types: Vec<CompactString>,
}

impl RawOption {
    fn into_option<E: DeError>(self) -> Result<CommandOption, E> {
        let basic = |raw: Self| BasicOption {
            name: raw.name,
            name_localizations: raw.name_localizations,
            description: raw.description,
            description_localizations: raw.description_localizations,
            name_localized: raw.name_localized,
            description_localized: raw.description_localized,
            required: raw.required,
        };
        Ok(match self.kind {
            kind @ (CommandOptionType::SubCommand | CommandOptionType::SubCommandGroup) => {
                let option = SubCommandOption {
                    name: self.name,
                    name_localizations: self.name_localizations,
                    description: self.description,
                    description_localizations: self.description_localizations,
                    name_localized: self.name_localized,
                    description_localized: self.description_localized,
                    options: self.options,
                };
                if kind == CommandOptionType::SubCommand {
                    CommandOption::SubCommand(option)
                } else {
                    CommandOption::SubCommandGroup(option)
                }
            }
            CommandOptionType::String => CommandOption::String(StringOption {
                choices: convert_choices(self.choices, RawValue::into_string)?,
                name: self.name,
                name_localizations: self.name_localizations,
                description: self.description,
                description_localizations: self.description_localizations,
                name_localized: self.name_localized,
                description_localized: self.description_localized,
                required: self.required,
                min_length: self.min_length,
                max_length: self.max_length,
                autocomplete: self.autocomplete,
            }),
            CommandOptionType::Integer => CommandOption::Integer(IntegerOption {
                choices: convert_choices(self.choices, RawValue::into_integer)?,
                min_value: self.min_value.map(RawValue::into_integer).transpose()?,
                max_value: self.max_value.map(RawValue::into_integer).transpose()?,
                name: self.name,
                name_localizations: self.name_localizations,
                description: self.description,
                description_localizations: self.description_localizations,
                name_localized: self.name_localized,
                description_localized: self.description_localized,
                required: self.required,
                autocomplete: self.autocomplete,
            }),
            CommandOptionType::Number => CommandOption::Number(NumberOption {
                choices: convert_choices(self.choices, RawValue::into_number)?,
                min_value: self.min_value.map(RawValue::into_number).transpose()?,
                max_value: self.max_value.map(RawValue::into_number).transpose()?,
                name: self.name,
                name_localizations: self.name_localizations,
                description: self.description,
                description_localizations: self.description_localizations,
                name_localized: self.name_localized,
                description_localized: self.description_localized,
                required: self.required,
                autocomplete: self.autocomplete,
            }),
            CommandOptionType::Boolean => CommandOption::Boolean(basic(self)),
            CommandOptionType::User => CommandOption::User(basic(self)),
            CommandOptionType::Role => CommandOption::Role(basic(self)),
            CommandOptionType::Mentionable => CommandOption::Mentionable(basic(self)),
            CommandOptionType::Channel => CommandOption::Channel(ChannelOption {
                name: self.name,
                name_localizations: self.name_localizations,
                description: self.description,
                description_localizations: self.description_localizations,
                name_localized: self.name_localized,
                description_localized: self.description_localized,
                required: self.required,
                channel_types: self.channel_types,
            }),
            CommandOptionType::Attachment => CommandOption::Attachment(AttachmentOption {
                name: self.name,
                name_localizations: self.name_localizations,
                description: self.description,
                description_localizations: self.description_localizations,
                name_localized: self.name_localized,
                description_localized: self.description_localized,
                required: self.required,
                file_types: self.file_types,
            }),
            kind @ CommandOptionType::Unknown(_) => CommandOption::Unknown(UnknownOption {
                kind,
                name: self.name,
                name_localizations: self.name_localizations,
                description: self.description,
                description_localizations: self.description_localizations,
                name_localized: self.name_localized,
                description_localized: self.description_localized,
                required: self.required,
            }),
        })
    }
}

impl<'de> Deserialize<'de> for CommandOption {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RawOption::deserialize(deserializer)?.into_option::<D::Error>()
    }
}

#[derive(Serialize)]
struct Tagged<'a, T> {
    #[serde(rename = "type")]
    kind: CommandOptionType,
    #[serde(flatten)]
    option: &'a T,
}

impl Serialize for CommandOption {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let kind = self.kind();
        match self {
            Self::SubCommand(option) | Self::SubCommandGroup(option) => {
                Tagged { kind, option }.serialize(serializer)
            }
            Self::String(option) => Tagged { kind, option }.serialize(serializer),
            Self::Integer(option) => Tagged { kind, option }.serialize(serializer),
            Self::Boolean(option)
            | Self::User(option)
            | Self::Role(option)
            | Self::Mentionable(option) => Tagged { kind, option }.serialize(serializer),
            Self::Channel(option) => Tagged { kind, option }.serialize(serializer),
            Self::Number(option) => Tagged { kind, option }.serialize(serializer),
            Self::Attachment(option) => Tagged { kind, option }.serialize(serializer),
            Self::Unknown(option) => Tagged { kind, option }.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "GuildApplicationCommandPermissions")]
pub struct GuildCommandPermissions {
    pub id: GenericId,

    pub application_id: ApplicationId,

    pub guild_id: GuildId,

    pub permissions: Vec<CommandPermission>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "ApplicationCommandPermissions")]
pub struct CommandPermission {
    pub id: GenericId,

    #[serde(rename = "type")]
    pub kind: CommandPermissionType,

    pub permission: bool,
}

impl CommandPermission {
    #[must_use]
    pub const fn new(id: GenericId, kind: CommandPermissionType, permission: bool) -> Self {
        Self {
            id,
            kind,
            permission,
        }
    }

    #[must_use]
    pub const fn everyone(guild_id: GuildId) -> GenericId {
        guild_id.cast()
    }

    #[must_use]
    pub const fn all_channels(guild_id: GuildId) -> Option<GenericId> {
        GenericId::new_checked(guild_id.get() - 1)
    }
}

int_enum! {



    #[doc(alias = "ApplicationCommandPermissionType")]
    pub enum CommandPermissionType: u8 {

        Role = 1,

        User = 2,

        Channel = 3,
    }
}
