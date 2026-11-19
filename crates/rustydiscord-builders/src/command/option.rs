use compact_str::CompactString;
use rustydiscord_model::{
    application_command::{
        AttachmentOption, BasicOption, ChannelOption, Choice, CommandOption, IntegerOption,
        NumberOption, StringOption,
    },
    channel::ChannelType,
};

use super::validate;
use crate::ValidationError;

macro_rules! option_type {
    (
        $(#[$meta:meta])*
        $name:ident($model:ident) => $variant:ident
    ) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq)]
        #[must_use = "an option does nothing until it is added to a command or built"]
        pub struct $name {
            option: $model,
        }

        impl $name {




            pub fn new(
                name: impl Into<CompactString>,
                description: impl Into<CompactString>,
            ) -> Self {
                Self {
                    option: $model::new(name, description),
                }
            }





            pub fn required(mut self, required: bool) -> Self {
                self.option.required = required;
                self
            }

            name_localizations!(option);
            description_localizations!(option);












            pub fn build(self) -> Result<CommandOption, ValidationError> {
                let option = CommandOption::from(self);
                validate::option_alone(&option)?;
                Ok(option)
            }




            #[must_use]
            pub fn build_unchecked(self) -> CommandOption {
                self.into()
            }
        }

        impl From<$name> for CommandOption {
            fn from(option: $name) -> Self {
                Self::$variant(option.option)
            }
        }
    };
}

option_type! {
























    SlashCommandStringOption(StringOption) => String
}

impl SlashCommandStringOption {
    pub fn choice(
        mut self,
        name: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.option.choices.push(Choice::new(name, value.into()));
        self
    }

    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice<CompactString>>) -> Self {
        self.option.choices.extend(choices);
        self
    }

    pub fn min_length(mut self, min_length: u16) -> Self {
        self.option.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: u16) -> Self {
        self.option.max_length = Some(max_length);
        self
    }

    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.option.autocomplete = autocomplete;
        self
    }
}

option_type! {


















    SlashCommandIntegerOption(IntegerOption) => Integer
}

impl SlashCommandIntegerOption {
    pub fn choice(mut self, name: impl Into<CompactString>, value: i64) -> Self {
        self.option.choices.push(Choice::new(name, value));
        self
    }

    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice<i64>>) -> Self {
        self.option.choices.extend(choices);
        self
    }

    pub fn min_value(mut self, min_value: i64) -> Self {
        self.option.min_value = Some(min_value);
        self
    }

    pub fn max_value(mut self, max_value: i64) -> Self {
        self.option.max_value = Some(max_value);
        self
    }

    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.option.autocomplete = autocomplete;
        self
    }
}

option_type! {






    SlashCommandNumberOption(NumberOption) => Number
}

impl SlashCommandNumberOption {
    pub fn choice(mut self, name: impl Into<CompactString>, value: f64) -> Self {
        self.option.choices.push(Choice::new(name, value));
        self
    }

    pub fn choices(mut self, choices: impl IntoIterator<Item = Choice<f64>>) -> Self {
        self.option.choices.extend(choices);
        self
    }

    pub fn min_value(mut self, min_value: f64) -> Self {
        self.option.min_value = Some(min_value);
        self
    }

    pub fn max_value(mut self, max_value: f64) -> Self {
        self.option.max_value = Some(max_value);
        self
    }

    pub fn autocomplete(mut self, autocomplete: bool) -> Self {
        self.option.autocomplete = autocomplete;
        self
    }
}

option_type! {




    SlashCommandBooleanOption(BasicOption) => Boolean
}

option_type! {




    SlashCommandUserOption(BasicOption) => User
}

option_type! {




    SlashCommandRoleOption(BasicOption) => Role
}

option_type! {




    SlashCommandMentionableOption(BasicOption) => Mentionable
}

option_type! {


















    SlashCommandChannelOption(ChannelOption) => Channel
}

impl SlashCommandChannelOption {
    pub fn channel_types(mut self, channel_types: impl IntoIterator<Item = ChannelType>) -> Self {
        self.option.channel_types = channel_types.into_iter().collect();
        self
    }
}

option_type! {




    SlashCommandAttachmentOption(AttachmentOption) => Attachment
}

impl SlashCommandAttachmentOption {
    pub fn file_types<T: Into<CompactString>>(
        mut self,
        file_types: impl IntoIterator<Item = T>,
    ) -> Self {
        self.option.file_types = file_types.into_iter().map(Into::into).collect();
        self
    }
}
