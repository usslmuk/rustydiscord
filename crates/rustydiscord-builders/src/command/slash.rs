use compact_str::CompactString;
use rustydiscord_model::{
    application_command::{CommandOption, SubCommandOption},
    rest::CreateApplicationCommand,
};

use super::validate;
use crate::ValidationError;

#[derive(Clone, Debug, PartialEq)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct SlashCommandBuilder {
    command: CreateApplicationCommand,
}

impl SlashCommandBuilder {
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            command: CreateApplicationCommand::chat_input(name, description),
        }
    }

    name_localizations!(command);
    description_localizations!(command);
    command_settings!();

    pub fn option(mut self, option: impl Into<CommandOption>) -> Self {
        self.command.options.push(option.into());
        self
    }

    pub fn subcommand(self, subcommand: SlashCommandSubcommandBuilder) -> Self {
        self.option(subcommand)
    }

    pub fn subcommand_group(self, group: SlashCommandSubcommandGroupBuilder) -> Self {
        self.option(group)
    }

    pub fn build(self) -> Result<CreateApplicationCommand, ValidationError> {
        validate::slash_command(&self.command)?;
        Ok(self.command)
    }

    #[must_use]
    pub fn build_unchecked(self) -> CreateApplicationCommand {
        self.command
    }
}

#[derive(Clone, Debug, PartialEq)]
#[must_use = "a builder does nothing until it is added to a command or built"]
pub struct SlashCommandSubcommandBuilder {
    option: SubCommandOption,
}

impl SlashCommandSubcommandBuilder {
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            option: SubCommandOption::new(name, description),
        }
    }

    name_localizations!(option);
    description_localizations!(option);

    pub fn option(mut self, option: impl Into<CommandOption>) -> Self {
        self.option.options.push(option.into());
        self
    }

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

impl From<SlashCommandSubcommandBuilder> for CommandOption {
    fn from(subcommand: SlashCommandSubcommandBuilder) -> Self {
        Self::SubCommand(subcommand.option)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[must_use = "a builder does nothing until it is added to a command or built"]
pub struct SlashCommandSubcommandGroupBuilder {
    option: SubCommandOption,
}

impl SlashCommandSubcommandGroupBuilder {
    pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self {
        Self {
            option: SubCommandOption::new(name, description),
        }
    }

    name_localizations!(option);
    description_localizations!(option);

    pub fn subcommand(mut self, subcommand: SlashCommandSubcommandBuilder) -> Self {
        self.option.options.push(subcommand.into());
        self
    }

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

impl From<SlashCommandSubcommandGroupBuilder> for CommandOption {
    fn from(group: SlashCommandSubcommandGroupBuilder) -> Self {
        Self::SubCommandGroup(group.option)
    }
}
