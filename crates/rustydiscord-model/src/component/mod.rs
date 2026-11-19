mod content;
mod interactive;
mod layout;

use compact_str::CompactString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub use self::{
    content::{
        ContentScanMetadata, ContentScanMetadataFlags, File, LoadingState, MediaGallery,
        MediaGalleryItem, TextDisplay, Thumbnail, UnfurledMediaItem, UnfurledMediaItemFlags,
    },
    interactive::{
        AutoPopulatedSelect, Button, ButtonStyle, ChannelSelect, Checkbox, CheckboxGroup,
        FileUpload, GroupOption, InteractiveButton, LinkButton, PremiumButton, RadioGroup,
        SelectDefaultValue, SelectDefaultValueType, SelectOption, StringSelect, TextInput,
        TextInputStyle, UnknownButton,
    },
    layout::{ActionRow, Container, Label, Section, Separator, SeparatorSpacing},
};
use crate::{channel::ChannelType, emoji::PartialEmoji, id::SkuId, macros::int_enum};

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Component {
    ActionRow(ActionRow),

    Button(Button),

    StringSelect(StringSelect),

    TextInput(TextInput),

    UserSelect(AutoPopulatedSelect),

    RoleSelect(AutoPopulatedSelect),

    MentionableSelect(AutoPopulatedSelect),

    ChannelSelect(ChannelSelect),

    Section(Section),

    TextDisplay(TextDisplay),

    Thumbnail(Thumbnail),

    MediaGallery(MediaGallery),

    File(File),

    Separator(Separator),

    Container(Container),

    Label(Label),

    FileUpload(FileUpload),

    RadioGroup(RadioGroup),

    CheckboxGroup(CheckboxGroup),

    Checkbox(Checkbox),

    Unknown(UnknownComponent),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct UnknownComponent {
    #[serde(skip)]
    pub kind: ComponentType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
}

int_enum! {



    pub enum ComponentType: u8 {

        ActionRow = 1,

        Button = 2,

        StringSelect = 3,

        TextInput = 4,

        UserSelect = 5,

        RoleSelect = 6,

        MentionableSelect = 7,

        ChannelSelect = 8,

        Section = 9,

        TextDisplay = 10,

        Thumbnail = 11,

        MediaGallery = 12,

        File = 13,

        Separator = 14,




        ContentInventoryEntry = 16,

        Container = 17,

        Label = 18,

        FileUpload = 19,

        RadioGroup = 21,

        CheckboxGroup = 22,

        Checkbox = 23,
    }
}

impl Component {
    #[must_use]
    pub const fn kind(&self) -> ComponentType {
        match self {
            Self::ActionRow(_) => ComponentType::ActionRow,
            Self::Button(_) => ComponentType::Button,
            Self::StringSelect(_) => ComponentType::StringSelect,
            Self::TextInput(_) => ComponentType::TextInput,
            Self::UserSelect(_) => ComponentType::UserSelect,
            Self::RoleSelect(_) => ComponentType::RoleSelect,
            Self::MentionableSelect(_) => ComponentType::MentionableSelect,
            Self::ChannelSelect(_) => ComponentType::ChannelSelect,
            Self::Section(_) => ComponentType::Section,
            Self::TextDisplay(_) => ComponentType::TextDisplay,
            Self::Thumbnail(_) => ComponentType::Thumbnail,
            Self::MediaGallery(_) => ComponentType::MediaGallery,
            Self::File(_) => ComponentType::File,
            Self::Separator(_) => ComponentType::Separator,
            Self::Container(_) => ComponentType::Container,
            Self::Label(_) => ComponentType::Label,
            Self::FileUpload(_) => ComponentType::FileUpload,
            Self::RadioGroup(_) => ComponentType::RadioGroup,
            Self::CheckboxGroup(_) => ComponentType::CheckboxGroup,
            Self::Checkbox(_) => ComponentType::Checkbox,
            Self::Unknown(component) => component.kind,
        }
    }

    #[must_use]
    pub const fn id(&self) -> Option<i32> {
        match self {
            Self::ActionRow(component) => component.id,
            Self::Button(component) => component.id(),
            Self::StringSelect(component) => component.id,
            Self::TextInput(component) => component.id,
            Self::UserSelect(component)
            | Self::RoleSelect(component)
            | Self::MentionableSelect(component) => component.id,
            Self::ChannelSelect(component) => component.id,
            Self::Section(component) => component.id,
            Self::TextDisplay(component) => component.id,
            Self::Thumbnail(component) => component.id,
            Self::MediaGallery(component) => component.id,
            Self::File(component) => component.id,
            Self::Separator(component) => component.id,
            Self::Container(component) => component.id,
            Self::Label(component) => component.id,
            Self::FileUpload(component) => component.id,
            Self::RadioGroup(component) => component.id,
            Self::CheckboxGroup(component) => component.id,
            Self::Checkbox(component) => component.id,
            Self::Unknown(component) => component.id,
        }
    }

    #[must_use]
    pub fn custom_id(&self) -> Option<&str> {
        match self {
            Self::Button(component) => component.custom_id(),
            Self::StringSelect(component) => Some(&component.custom_id),
            Self::TextInput(component) => Some(&component.custom_id),
            Self::UserSelect(component)
            | Self::RoleSelect(component)
            | Self::MentionableSelect(component) => Some(&component.custom_id),
            Self::ChannelSelect(component) => Some(&component.custom_id),
            Self::FileUpload(component) => Some(&component.custom_id),
            Self::RadioGroup(component) => Some(&component.custom_id),
            Self::CheckboxGroup(component) => Some(&component.custom_id),
            Self::Checkbox(component) => Some(&component.custom_id),
            Self::ActionRow(_)
            | Self::Section(_)
            | Self::TextDisplay(_)
            | Self::Thumbnail(_)
            | Self::MediaGallery(_)
            | Self::File(_)
            | Self::Separator(_)
            | Self::Container(_)
            | Self::Label(_)
            | Self::Unknown(_) => None,
        }
    }

    #[must_use]
    pub fn children(&self) -> &[Self] {
        match self {
            Self::ActionRow(component) => &component.components,
            Self::Section(component) => &component.components,
            Self::Container(component) => &component.components,
            _ => &[],
        }
    }
}

macro_rules! from_struct {
    ($($ty:ident => $variant:ident),* $(,)?) => {
        $(
            impl From<$ty> for Component {
                fn from(component: $ty) -> Self {
                    Self::$variant(component)
                }
            }
        )*
    };
}

from_struct! {
    ActionRow => ActionRow,
    Button => Button,
    StringSelect => StringSelect,
    TextInput => TextInput,
    ChannelSelect => ChannelSelect,
    Section => Section,
    TextDisplay => TextDisplay,
    Thumbnail => Thumbnail,
    MediaGallery => MediaGallery,
    File => File,
    Separator => Separator,
    Container => Container,
    Label => Label,
    FileUpload => FileUpload,
    RadioGroup => RadioGroup,
    CheckboxGroup => CheckboxGroup,
    Checkbox => Checkbox,
}

macro_rules! from_button {
    ($($ty:ident => $variant:ident),* $(,)?) => {
        $(
            impl From<$ty> for Button {
                fn from(button: $ty) -> Self {
                    Self::$variant(button)
                }
            }

            impl From<$ty> for Component {
                fn from(button: $ty) -> Self {
                    Self::Button(Button::$variant(button))
                }
            }
        )*
    };
}

from_button! {
    InteractiveButton => Interactive,
    LinkButton => Link,
    PremiumButton => Premium,
}

#[derive(Deserialize)]
struct RawComponent {
    #[serde(rename = "type")]
    kind: ComponentType,
    #[serde(default)]
    id: Option<i32>,
    #[serde(default)]
    custom_id: Option<CompactString>,
    #[serde(default)]
    components: Vec<Component>,
    #[serde(default)]
    component: Option<Box<Component>>,
    #[serde(default)]
    accessory: Option<Box<Component>>,
    #[serde(default)]
    style: Option<u8>,
    #[serde(default)]
    label: Option<CompactString>,
    #[serde(default)]
    emoji: Option<PartialEmoji>,
    #[serde(default)]
    url: Option<CompactString>,
    #[serde(default)]
    sku_id: Option<SkuId>,
    #[serde(default)]
    disabled: bool,
    #[serde(default)]
    options: Vec<SelectOption>,
    #[serde(default)]
    placeholder: Option<CompactString>,
    #[serde(default)]
    min_values: Option<u8>,
    #[serde(default)]
    max_values: Option<u8>,
    #[serde(default)]
    required: Option<bool>,
    #[serde(default)]
    min_length: Option<u16>,
    #[serde(default)]
    max_length: Option<u16>,
    #[serde(default)]
    value: Option<CompactString>,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    default_values: Vec<SelectDefaultValue>,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    channel_types: Vec<ChannelType>,
    #[serde(default)]
    content: Option<CompactString>,
    #[serde(default)]
    media: Option<Box<UnfurledMediaItem>>,
    #[serde(default)]
    description: Option<CompactString>,
    #[serde(default)]
    spoiler: bool,
    #[serde(default)]
    items: Vec<MediaGalleryItem>,
    #[serde(default)]
    file: Option<Box<UnfurledMediaItem>>,
    #[serde(default)]
    name: Option<CompactString>,
    #[serde(default)]
    size: Option<u64>,
    #[serde(default)]
    divider: Option<bool>,
    #[serde(default)]
    spacing: Option<SeparatorSpacing>,
    #[serde(default)]
    accent_color: Option<u32>,
    #[serde(default, deserialize_with = "crate::serde_util::null_as_default")]
    file_types: Vec<CompactString>,
    #[serde(default)]
    default: bool,
}

impl RawComponent {
    fn required<T, E: serde::de::Error>(value: Option<T>, field: &'static str) -> Result<T, E> {
        value.ok_or_else(|| E::missing_field(field))
    }

    fn into_component<E: serde::de::Error>(self) -> Result<Component, E> {
        let id = self.id;
        Ok(match self.kind {
            ComponentType::ActionRow => Component::ActionRow(ActionRow {
                id,
                components: self.components,
            }),
            ComponentType::Button => Component::Button(self.into_button()?),
            ComponentType::StringSelect => Component::StringSelect(StringSelect {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                options: self.options,
                placeholder: self.placeholder,
                min_values: self.min_values,
                max_values: self.max_values,
                required: self.required.unwrap_or(true),
                disabled: self.disabled,
            }),
            ComponentType::TextInput => Component::TextInput(TextInput {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                style: TextInputStyle::from(Self::required(self.style, "style")?),
                label: self.label,
                min_length: self.min_length,
                max_length: self.max_length,
                required: self.required.unwrap_or(true),
                value: self.value,
                placeholder: self.placeholder,
            }),
            kind @ (ComponentType::UserSelect
            | ComponentType::RoleSelect
            | ComponentType::MentionableSelect) => {
                let select = AutoPopulatedSelect {
                    id,
                    custom_id: Self::required(self.custom_id, "custom_id")?,
                    placeholder: self.placeholder,
                    default_values: self.default_values,
                    min_values: self.min_values,
                    max_values: self.max_values,
                    required: self.required.unwrap_or(true),
                    disabled: self.disabled,
                };
                match kind {
                    ComponentType::UserSelect => Component::UserSelect(select),
                    ComponentType::RoleSelect => Component::RoleSelect(select),
                    _ => Component::MentionableSelect(select),
                }
            }
            ComponentType::ChannelSelect => Component::ChannelSelect(ChannelSelect {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                channel_types: self.channel_types,
                placeholder: self.placeholder,
                default_values: self.default_values,
                min_values: self.min_values,
                max_values: self.max_values,
                required: self.required.unwrap_or(true),
                disabled: self.disabled,
            }),
            kind => self.into_v2_component(kind)?,
        })
    }

    fn into_v2_component<E: serde::de::Error>(self, kind: ComponentType) -> Result<Component, E> {
        let id = self.id;
        Ok(match kind {
            ComponentType::Section => Component::Section(Section {
                id,
                components: self.components,
                accessory: Self::required(self.accessory, "accessory")?,
            }),
            ComponentType::TextDisplay => Component::TextDisplay(TextDisplay {
                id,
                content: Self::required(self.content, "content")?,
            }),
            ComponentType::Thumbnail => Component::Thumbnail(Thumbnail {
                id,
                media: Self::required(self.media, "media")?,
                description: self.description,
                spoiler: self.spoiler,
            }),
            ComponentType::MediaGallery => Component::MediaGallery(MediaGallery {
                id,
                items: self.items,
            }),
            ComponentType::File => Component::File(File {
                id,
                file: Self::required(self.file, "file")?,
                spoiler: self.spoiler,
                name: self.name,
                size: self.size,
            }),
            ComponentType::Separator => Component::Separator(Separator {
                id,
                divider: self.divider.unwrap_or(true),
                spacing: self.spacing,
            }),
            ComponentType::Container => Component::Container(Container {
                id,
                components: self.components,
                accent_color: self.accent_color,
                spoiler: self.spoiler,
            }),
            ComponentType::Label => Component::Label(Label {
                id,
                label: Self::required(self.label, "label")?,
                description: self.description,
                component: Self::required(self.component, "component")?,
            }),
            ComponentType::FileUpload => Component::FileUpload(FileUpload {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                min_values: self.min_values,
                max_values: self.max_values,
                required: self.required.unwrap_or(true),
                file_types: self.file_types,
            }),
            ComponentType::RadioGroup => Component::RadioGroup(RadioGroup {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                options: self.options.into_iter().map(GroupOption::from).collect(),
                required: self.required.unwrap_or(true),
            }),
            ComponentType::CheckboxGroup => Component::CheckboxGroup(CheckboxGroup {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                options: self.options.into_iter().map(GroupOption::from).collect(),
                min_values: self.min_values,
                max_values: self.max_values,
                required: self.required.unwrap_or(true),
            }),
            ComponentType::Checkbox => Component::Checkbox(Checkbox {
                id,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                default: self.default,
            }),
            kind => Component::Unknown(UnknownComponent { kind, id }),
        })
    }

    fn into_button<E: serde::de::Error>(self) -> Result<Button, E> {
        let id = self.id;
        let style = ButtonStyle::from(Self::required(self.style, "style")?);
        Ok(match style {
            ButtonStyle::Primary
            | ButtonStyle::Secondary
            | ButtonStyle::Success
            | ButtonStyle::Danger => Button::Interactive(InteractiveButton {
                id,
                style,
                custom_id: Self::required(self.custom_id, "custom_id")?,
                label: self.label,
                emoji: self.emoji,
                disabled: self.disabled,
            }),
            ButtonStyle::Link => Button::Link(LinkButton {
                id,
                url: Self::required(self.url, "url")?,
                label: self.label,
                emoji: self.emoji,
                disabled: self.disabled,
            }),
            ButtonStyle::Premium => Button::Premium(PremiumButton {
                id,
                sku_id: Self::required(self.sku_id, "sku_id")?,
                disabled: self.disabled,
            }),
            ButtonStyle::Unknown(_) => Button::Unknown(Box::new(UnknownButton {
                id,
                style,
                custom_id: self.custom_id,
                label: self.label,
                emoji: self.emoji,
                url: self.url,
                sku_id: self.sku_id,
                disabled: self.disabled,
            })),
        })
    }
}

impl From<SelectOption> for GroupOption {
    fn from(option: SelectOption) -> Self {
        Self {
            value: option.value,
            label: option.label,
            description: option.description,
            default: option.default,
        }
    }
}

impl<'de> Deserialize<'de> for Component {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RawComponent::deserialize(deserializer)?.into_component::<D::Error>()
    }
}

#[derive(Serialize)]
struct Tagged<'a, T> {
    #[serde(rename = "type")]
    kind: ComponentType,
    #[serde(flatten)]
    component: &'a T,
}

impl Serialize for Component {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let kind = self.kind();
        match self {
            Self::ActionRow(component) => Tagged { kind, component }.serialize(serializer),
            Self::Button(component) => Tagged { kind, component }.serialize(serializer),
            Self::StringSelect(component) => Tagged { kind, component }.serialize(serializer),
            Self::TextInput(component) => Tagged { kind, component }.serialize(serializer),
            Self::UserSelect(component)
            | Self::RoleSelect(component)
            | Self::MentionableSelect(component) => {
                Tagged { kind, component }.serialize(serializer)
            }
            Self::ChannelSelect(component) => Tagged { kind, component }.serialize(serializer),
            Self::Section(component) => Tagged { kind, component }.serialize(serializer),
            Self::TextDisplay(component) => Tagged { kind, component }.serialize(serializer),
            Self::Thumbnail(component) => Tagged { kind, component }.serialize(serializer),
            Self::MediaGallery(component) => Tagged { kind, component }.serialize(serializer),
            Self::File(component) => Tagged { kind, component }.serialize(serializer),
            Self::Separator(component) => Tagged { kind, component }.serialize(serializer),
            Self::Container(component) => Tagged { kind, component }.serialize(serializer),
            Self::Label(component) => Tagged { kind, component }.serialize(serializer),
            Self::FileUpload(component) => Tagged { kind, component }.serialize(serializer),
            Self::RadioGroup(component) => Tagged { kind, component }.serialize(serializer),
            Self::CheckboxGroup(component) => Tagged { kind, component }.serialize(serializer),
            Self::Checkbox(component) => Tagged { kind, component }.serialize(serializer),
            Self::Unknown(component) => Tagged { kind, component }.serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sizes() {
        assert!(size_of::<Component>() <= 120, "{}", size_of::<Component>());
    }
}
