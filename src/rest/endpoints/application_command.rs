use crate::model::{
    application_command::{ApplicationCommand, GuildCommandPermissions},
    id::{ApplicationId, CommandId, GuildId},
    rest::{
        CreateApplicationCommand, EditApplicationCommand, EditApplicationCommandPermissions,
        GetApplicationCommands,
    },
};

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

impl Rest {
    pub fn get_global_application_commands<'a>(
        &'a self,
        application_id: ApplicationId,
        query: &'a GetApplicationCommands,
    ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::GetGlobalApplicationCommands { application_id }).query(query),
            )
            .await
        })
    }

    pub fn create_global_application_command<'a>(
        &'a self,
        application_id: ApplicationId,
        body: &'a CreateApplicationCommand,
    ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGlobalApplicationCommand { application_id })
                    .json(body)?,
            )
            .await
        })
    }

    pub fn get_global_application_command(
        &self,
        application_id: ApplicationId,
        command_id: CommandId,
    ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGlobalApplicationCommand {
                application_id,
                command_id,
            }))
            .await
        })
    }

    pub fn edit_global_application_command<'a>(
        &'a self,
        application_id: ApplicationId,
        command_id: CommandId,
        body: &'a EditApplicationCommand,
    ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::EditGlobalApplicationCommand {
                    application_id,
                    command_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn delete_global_application_command(
        &self,
        application_id: ApplicationId,
        command_id: CommandId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteGlobalApplicationCommand {
                application_id,
                command_id,
            }))
            .await
        })
    }

    pub fn bulk_overwrite_global_application_commands<'a>(
        &'a self,
        application_id: ApplicationId,
        commands: &'a [CreateApplicationCommand],
    ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::BulkOverwriteGlobalApplicationCommands { application_id })
                    .json(commands)?,
            )
            .await
        })
    }

    pub fn get_guild_application_commands<'a>(
        &'a self,
        application_id: ApplicationId,
        guild_id: GuildId,
        query: &'a GetApplicationCommands,
    ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::GetGuildApplicationCommands {
                    application_id,
                    guild_id,
                })
                .query(query),
            )
            .await
        })
    }

    pub fn create_guild_application_command<'a>(
        &'a self,
        application_id: ApplicationId,
        guild_id: GuildId,
        body: &'a CreateApplicationCommand,
    ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildApplicationCommand {
                    application_id,
                    guild_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn get_guild_application_command(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildApplicationCommand {
                application_id,
                guild_id,
                command_id,
            }))
            .await
        })
    }

    pub fn edit_guild_application_command<'a>(
        &'a self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
        body: &'a EditApplicationCommand,
    ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::EditGuildApplicationCommand {
                    application_id,
                    guild_id,
                    command_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn delete_guild_application_command(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteGuildApplicationCommand {
                application_id,
                guild_id,
                command_id,
            }))
            .await
        })
    }

    pub fn bulk_overwrite_guild_application_commands<'a>(
        &'a self,
        application_id: ApplicationId,
        guild_id: GuildId,
        commands: &'a [CreateApplicationCommand],
    ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::BulkOverwriteGuildApplicationCommands {
                    application_id,
                    guild_id,
                })
                .json(commands)?,
            )
            .await
        })
    }

    pub fn get_guild_application_command_permissions(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<GuildCommandPermissions>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildApplicationCommandPermissions {
                application_id,
                guild_id,
            }))
            .await
        })
    }

    pub fn get_application_command_permissions(
        &self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
    ) -> impl Future<Output = Result<GuildCommandPermissions, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetApplicationCommandPermissions {
                application_id,
                guild_id,
                command_id,
            }))
            .await
        })
    }

    pub fn edit_application_command_permissions<'a>(
        &'a self,
        application_id: ApplicationId,
        guild_id: GuildId,
        command_id: CommandId,
        body: &'a EditApplicationCommandPermissions,
    ) -> impl Future<Output = Result<GuildCommandPermissions, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::EditApplicationCommandPermissions {
                    application_id,
                    guild_id,
                    command_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn bulk_overwrite_guild_command_permissions<'a>(
        &'a self,
        application_id: ApplicationId,
        guild_id: GuildId,
        body: &'a [GuildCommandPermissions],
    ) -> impl Future<Output = Result<Vec<GuildCommandPermissions>, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::BulkOverwriteGuildCommandPermissions {
                    application_id,
                    guild_id,
                })
                .json(body)?,
            )
            .await
        })
    }
}
