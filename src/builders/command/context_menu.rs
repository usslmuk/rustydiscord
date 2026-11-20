use crate::model::rest::CreateApplicationCommand;
use compact_str::CompactString;

use super::validate;
use crate::builders::ValidationError;

#[derive(Clone, Debug, PartialEq)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct ContextMenuCommandBuilder {
    command: CreateApplicationCommand,
}

impl ContextMenuCommandBuilder {
    pub fn user(name: impl Into<CompactString>) -> Self {
        Self {
            command: CreateApplicationCommand::user(name),
        }
    }

    pub fn message(name: impl Into<CompactString>) -> Self {
        Self {
            command: CreateApplicationCommand::message(name),
        }
    }

    name_localizations!(command);
    command_settings!();

    pub fn build(self) -> Result<CreateApplicationCommand, ValidationError> {
        validate::context_menu_command(&self.command)?;
        Ok(self.command)
    }

    #[must_use]
    pub fn build_unchecked(self) -> CreateApplicationCommand {
        self.command
    }
}
