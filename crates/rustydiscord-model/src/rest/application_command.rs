use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::Serialize;

use crate::{
    application::IntegrationType,
    application_command::{CommandOption, CommandPermission, CommandType, EntryPointHandlerType},
    interaction::InteractionContextType,
    permissions::Permissions,
    util::{Locale, Patch},
};

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateApplicationCommand {
    pub name: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub description_localizations: BTreeMap<Locale, CompactString>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub options: Vec<CommandOption>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_member_permissions: Option<Permissions>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub integration_types: Vec<IntegrationType>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contexts: Vec<InteractionContextType>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<CommandType>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub nsfw: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<EntryPointHandlerType>,
}

impl CreateApplicationCommand {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, kind: CommandType) -> Self {
        Self {
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: None,
            description_localizations: BTreeMap::new(),
            options: Vec::new(),
            default_member_permissions: None,
            integration_types: Vec::new(),
            contexts: Vec::new(),
            kind: Some(kind),
            nsfw: false,
            handler: None,
        }
    }

    #[must_use]
    pub fn chat_input(
        name: impl Into<CompactString>,
        description: impl Into<CompactString>,
    ) -> Self {
        Self {
            description: Some(description.into()),
            ..Self::new(name, CommandType::ChatInput)
        }
    }

    #[must_use]
    pub fn user(name: impl Into<CompactString>) -> Self {
        Self::new(name, CommandType::User)
    }

    #[must_use]
    pub fn message(name: impl Into<CompactString>) -> Self {
        Self::new(name, CommandType::Message)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct EditApplicationCommand {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub name_localizations: Patch<BTreeMap<Locale, CompactString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description_localizations: Patch<BTreeMap<Locale, CompactString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CommandOption>>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub default_member_permissions: Patch<Permissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types: Option<Vec<IntegrationType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contexts: Option<Vec<InteractionContextType>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nsfw: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub handler: Option<EntryPointHandlerType>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetApplicationCommands {
    pub with_localizations: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct EditApplicationCommandPermissions {
    pub permissions: Vec<CommandPermission>,
}

impl EditApplicationCommandPermissions {
    #[must_use]
    pub const fn new(permissions: Vec<CommandPermission>) -> Self {
        Self { permissions }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_edit() {
        let mut command = CreateApplicationCommand::chat_input("ban", "Bans a member");
        command.default_member_permissions = Some(Permissions::BAN_MEMBERS);
        command.contexts = vec![InteractionContextType::Guild];
        command
            .name_localizations
            .insert(Locale::Polish, "zbanuj".into());
        assert_eq!(
            serde_json::to_string(&command).unwrap(),
            concat!(
                r#"{"name":"ban","name_localizations":{"pl":"zbanuj"},"description":"Bans a member","#,
                r#""default_member_permissions":"4","contexts":[0],"type":1}"#
            )
        );

        let edit = EditApplicationCommand {
            description: Some("Bans someone".into()),
            default_member_permissions: Patch::Clear,
            name_localizations: Patch::Set(BTreeMap::new()),
            ..EditApplicationCommand::default()
        };
        assert_eq!(
            serde_json::to_string(&edit).unwrap(),
            r#"{"name_localizations":{},"description":"Bans someone","default_member_permissions":null}"#
        );
    }
}
