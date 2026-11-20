macro_rules! name_localizations {
    ($field:ident) => {
        pub fn name_localization(
            mut self,
            locale: crate::model::util::Locale,
            name: impl Into<::compact_str::CompactString>,
        ) -> Self {
            self.$field.name_localizations.insert(locale, name.into());
            self
        }

        pub fn name_localizations<N: Into<::compact_str::CompactString>>(
            mut self,
            localizations: impl IntoIterator<Item = (crate::model::util::Locale, N)>,
        ) -> Self {
            self.$field.name_localizations.extend(
                localizations
                    .into_iter()
                    .map(|(locale, name)| (locale, name.into())),
            );
            self
        }
    };
}

macro_rules! description_localizations {
    ($field:ident) => {
        pub fn description_localization(
            mut self,
            locale: crate::model::util::Locale,
            description: impl Into<::compact_str::CompactString>,
        ) -> Self {
            self.$field
                .description_localizations
                .insert(locale, description.into());
            self
        }

        pub fn description_localizations<D: Into<::compact_str::CompactString>>(
            mut self,
            localizations: impl IntoIterator<Item = (crate::model::util::Locale, D)>,
        ) -> Self {
            self.$field.description_localizations.extend(
                localizations
                    .into_iter()
                    .map(|(locale, description)| (locale, description.into())),
            );
            self
        }
    };
}

macro_rules! command_settings {
    () => {
        pub fn default_member_permissions(
            mut self,
            permissions: crate::model::permissions::Permissions,
        ) -> Self {
            self.command.default_member_permissions = Some(permissions);
            self
        }

        pub fn contexts(
            mut self,
            contexts: impl IntoIterator<Item = crate::model::interaction::InteractionContextType>,
        ) -> Self {
            self.command.contexts = contexts.into_iter().collect();
            self
        }

        pub fn integration_types(
            mut self,
            integration_types: impl IntoIterator<Item = crate::model::application::IntegrationType>,
        ) -> Self {
            self.command.integration_types = integration_types.into_iter().collect();
            self
        }

        pub fn nsfw(mut self, nsfw: bool) -> Self {
            self.command.nsfw = nsfw;
            self
        }
    };
}

mod context_menu;
mod option;
mod slash;
mod validate;

pub use self::{
    context_menu::ContextMenuCommandBuilder,
    option::{
        SlashCommandAttachmentOption, SlashCommandBooleanOption, SlashCommandChannelOption,
        SlashCommandIntegerOption, SlashCommandMentionableOption, SlashCommandNumberOption,
        SlashCommandRoleOption, SlashCommandStringOption, SlashCommandUserOption,
    },
    slash::{
        SlashCommandBuilder, SlashCommandSubcommandBuilder, SlashCommandSubcommandGroupBuilder,
    },
};
