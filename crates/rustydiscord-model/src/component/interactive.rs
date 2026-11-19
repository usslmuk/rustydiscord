use compact_str::CompactString;
use serde::{Serialize, Serializer};

use crate::{
    channel::ChannelType,
    emoji::PartialEmoji,
    id::{GenericId, SkuId},
    macros::{int_enum, str_enum},
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
#[doc(alias = "ButtonComponent")]
pub enum Button {
    Interactive(InteractiveButton),

    Link(LinkButton),

    Premium(PremiumButton),

    Unknown(Box<UnknownButton>),
}

impl Button {
    #[must_use]
    pub const fn id(&self) -> Option<i32> {
        match self {
            Self::Interactive(button) => button.id,
            Self::Link(button) => button.id,
            Self::Premium(button) => button.id,
            Self::Unknown(button) => button.id,
        }
    }

    #[must_use]
    pub const fn style(&self) -> ButtonStyle {
        match self {
            Self::Interactive(button) => button.style,
            Self::Link(_) => ButtonStyle::Link,
            Self::Premium(_) => ButtonStyle::Premium,
            Self::Unknown(button) => button.style,
        }
    }

    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        match self {
            Self::Interactive(button) => Some(&button.custom_id),
            Self::Unknown(button) => button.custom_id.as_deref(),
            Self::Link(_) | Self::Premium(_) => None,
        }
    }

    #[must_use]
    pub const fn disabled(&self) -> bool {
        match self {
            Self::Interactive(button) => button.disabled,
            Self::Link(button) => button.disabled,
            Self::Premium(button) => button.disabled,
            Self::Unknown(button) => button.disabled,
        }
    }
}

#[derive(Serialize)]
struct Styled<'a, T> {
    style: ButtonStyle,
    #[serde(flatten)]
    button: &'a T,
}

impl Serialize for Button {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let style = self.style();
        match self {
            Self::Interactive(button) => Styled { style, button }.serialize(serializer),
            Self::Link(button) => Styled { style, button }.serialize(serializer),
            Self::Premium(button) => Styled { style, button }.serialize(serializer),
            Self::Unknown(button) => Styled { style, button }.serialize(serializer),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct InteractiveButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip)]
    pub style: ButtonStyle,

    pub custom_id: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

impl InteractiveButton {
    #[must_use]
    pub fn new(style: ButtonStyle, custom_id: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            style,
            custom_id: custom_id.into(),
            label: None,
            emoji: None,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct LinkButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub url: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

impl LinkButton {
    #[must_use]
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            url: url.into(),
            label: None,
            emoji: None,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct PremiumButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub sku_id: SkuId,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

impl PremiumButton {
    #[must_use]
    pub const fn new(sku_id: SkuId) -> Self {
        Self {
            id: None,
            sku_id,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UnknownButton {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    #[serde(skip)]
    pub style: ButtonStyle,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku_id: Option<SkuId>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

int_enum! {



    pub enum ButtonStyle: u8 {

        Primary = 1,

        Secondary = 2,

        Success = 3,

        Danger = 4,

        Link = 5,

        Premium = 6,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[doc(alias = "StringSelectMenu")]
pub struct StringSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    pub options: Vec<SelectOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    pub required: bool,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

impl StringSelect {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>, options: Vec<SelectOption>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            options,
            placeholder: None,
            min_values: None,
            max_values: None,
            required: true,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct SelectOption {
    pub label: CompactString,

    pub value: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub default: bool,
}

impl SelectOption {
    #[must_use]
    pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            description: None,
            emoji: None,
            default: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct TextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    pub style: TextInputStyle,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<u16>,

    pub required: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,
}

impl TextInput {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>, style: TextInputStyle) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            style,
            label: None,
            min_length: None,
            max_length: None,
            required: true,
            value: None,
            placeholder: None,
        }
    }
}

int_enum! {



    pub enum TextInputStyle: u8 {

        Short = 1,

        Paragraph = 2,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[doc(
    alias = "UserSelect",
    alias = "RoleSelect",
    alias = "MentionableSelect"
)]
pub struct AutoPopulatedSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_values: Vec<SelectDefaultValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    pub required: bool,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

impl AutoPopulatedSelect {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            placeholder: None,
            default_values: Vec::new(),
            min_values: None,
            max_values: None,
            required: true,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
#[doc(alias = "ChannelSelectMenu")]
pub struct ChannelSelect {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub channel_types: Vec<ChannelType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub default_values: Vec<SelectDefaultValue>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    pub required: bool,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub disabled: bool,
}

impl ChannelSelect {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            channel_types: Vec::new(),
            placeholder: None,
            default_values: Vec::new(),
            min_values: None,
            max_values: None,
            required: true,
            disabled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, serde::Deserialize)]
#[non_exhaustive]
pub struct SelectDefaultValue {
    pub id: GenericId,

    #[serde(rename = "type")]
    pub kind: SelectDefaultValueType,
}

impl SelectDefaultValue {
    #[must_use]
    pub fn new(id: GenericId, kind: SelectDefaultValueType) -> Self {
        Self { id, kind }
    }
}

str_enum! {



    #[doc(alias = "SelectMenuDefaultValueType")]
    pub enum SelectDefaultValueType {

        User = "user",

        Role = "role",

        Channel = "channel",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct FileUpload {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    pub required: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub file_types: Vec<CompactString>,
}

impl FileUpload {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            min_values: None,
            max_values: None,
            required: true,
            file_types: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RadioGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    pub options: Vec<GroupOption>,

    pub required: bool,
}

impl RadioGroup {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>, options: Vec<GroupOption>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            options,
            required: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CheckboxGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    pub options: Vec<GroupOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_values: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_values: Option<u8>,

    pub required: bool,
}

impl CheckboxGroup {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>, options: Vec<GroupOption>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            options,
            min_values: None,
            max_values: None,
            required: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, serde::Deserialize)]
#[non_exhaustive]
#[doc(alias = "RadioGroupOption", alias = "CheckboxGroupOption")]
pub struct GroupOption {
    pub value: CompactString,

    pub label: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub default: bool,
}

impl GroupOption {
    #[must_use]
    pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
            default: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Checkbox {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub custom_id: CompactString,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub default: bool,
}

impl Checkbox {
    #[must_use]
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            custom_id: custom_id.into(),
            default: false,
        }
    }
}
