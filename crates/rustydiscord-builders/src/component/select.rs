use compact_str::CompactString;
use rustydiscord_model::{
    channel::ChannelType,
    component::{
        AutoPopulatedSelect, ChannelSelect, Component, SelectDefaultValue, SelectDefaultValueType,
        SelectOption, StringSelect,
    },
    emoji::PartialEmoji,
    id::{ChannelId, RoleId, UserId},
};

use super::validate;
use crate::{ValidationError, check::Path};

macro_rules! select_menu {
    ($builder:ident => $variant:ident) => {
        impl $builder {
            pub fn placeholder(mut self, placeholder: impl Into<CompactString>) -> Self {
                self.select.placeholder = Some(placeholder.into());
                self
            }

            pub fn min_values(mut self, min_values: u8) -> Self {
                self.select.min_values = Some(min_values);
                self
            }

            pub fn max_values(mut self, max_values: u8) -> Self {
                self.select.max_values = Some(max_values);
                self
            }

            pub fn disabled(mut self, disabled: bool) -> Self {
                self.select.disabled = disabled;
                self
            }

            pub fn required(mut self, required: bool) -> Self {
                self.select.required = required;
                self
            }

            pub fn id(mut self, id: i32) -> Self {
                self.select.id = Some(id);
                self
            }

            pub fn build(self) -> Result<Component, ValidationError> {
                let component = Component::from(self);
                validate::component(&Path::Root, &component)?;
                Ok(component)
            }

            #[must_use]
            pub fn build_unchecked(self) -> Component {
                self.into()
            }
        }

        impl From<$builder> for Component {
            fn from(menu: $builder) -> Self {
                Self::$variant(menu.select)
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct StringSelectMenuBuilder {
    select: StringSelect,
}

impl StringSelectMenuBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            select: StringSelect::new(custom_id, Vec::new()),
        }
    }

    pub fn option(
        mut self,
        label: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.select.options.push(SelectOption::new(label, value));
        self
    }

    pub fn option_with(mut self, option: StringSelectMenuOptionBuilder) -> Self {
        self.select.options.push(option.option);
        self
    }

    pub fn options<O: Into<SelectOption>>(mut self, options: impl IntoIterator<Item = O>) -> Self {
        self.select
            .options
            .extend(options.into_iter().map(Into::into));
        self
    }
}

select_menu!(StringSelectMenuBuilder => StringSelect);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "an option does nothing until it is added to a menu"]
pub struct StringSelectMenuOptionBuilder {
    option: SelectOption,
}

impl StringSelectMenuOptionBuilder {
    pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self {
        Self {
            option: SelectOption::new(label, value),
        }
    }

    pub fn description(mut self, description: impl Into<CompactString>) -> Self {
        self.option.description = Some(description.into());
        self
    }

    pub fn emoji(mut self, emoji: impl Into<PartialEmoji>) -> Self {
        self.option.emoji = Some(emoji.into());
        self
    }

    pub fn default(mut self, default: bool) -> Self {
        self.option.default = default;
        self
    }
}

impl From<StringSelectMenuOptionBuilder> for SelectOption {
    fn from(option: StringSelectMenuOptionBuilder) -> Self {
        option.option
    }
}

macro_rules! default_values {
    ($method:ident, $id:ty, $kind:ident, $what:literal, $discord:literal) => {
        #[doc = concat!("Adds ", $what, " selected when the menu appears; their number must be within")]
        #[doc = concat!("discord.js counterpart: `", $discord, "`.")]
        pub fn $method(mut self, ids: impl IntoIterator<Item = $id>) -> Self {
            self.select.default_values.extend(
                ids.into_iter()
                    .map(|id| SelectDefaultValue::new(id.cast(), SelectDefaultValueType::$kind)),
            );
            self
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct UserSelectMenuBuilder {
    select: AutoPopulatedSelect,
}

impl UserSelectMenuBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            select: AutoPopulatedSelect::new(custom_id),
        }
    }

    default_values!(default_users, UserId, User, "users", "addDefaultUsers");
}

select_menu!(UserSelectMenuBuilder => UserSelect);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct RoleSelectMenuBuilder {
    select: AutoPopulatedSelect,
}

impl RoleSelectMenuBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            select: AutoPopulatedSelect::new(custom_id),
        }
    }

    default_values!(default_roles, RoleId, Role, "roles", "addDefaultRoles");
}

select_menu!(RoleSelectMenuBuilder => RoleSelect);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct MentionableSelectMenuBuilder {
    select: AutoPopulatedSelect,
}

impl MentionableSelectMenuBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            select: AutoPopulatedSelect::new(custom_id),
        }
    }

    default_values!(default_users, UserId, User, "users", "addDefaultUsers");
    default_values!(default_roles, RoleId, Role, "roles", "addDefaultRoles");
}

select_menu!(MentionableSelectMenuBuilder => MentionableSelect);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct ChannelSelectMenuBuilder {
    select: ChannelSelect,
}

impl ChannelSelectMenuBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            select: ChannelSelect::new(custom_id),
        }
    }

    pub fn channel_types(mut self, channel_types: impl IntoIterator<Item = ChannelType>) -> Self {
        self.select.channel_types = channel_types.into_iter().collect();
        self
    }

    default_values!(
        default_channels,
        ChannelId,
        Channel,
        "channels",
        "addDefaultChannels"
    );
}

select_menu!(ChannelSelectMenuBuilder => ChannelSelect);
