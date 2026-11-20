use std::future::Future;

use crate::cache::Backend;
use crate::collection::Collection;
use crate::model::{
    application_command::{ApplicationCommand, GuildCommandPermissions},
    id::{ApplicationId, CommandId, GenericId, GuildId},
    rest::{CreateApplicationCommand, EditApplicationCommand, GetApplicationCommands},
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

fn by_id(commands: Vec<ApplicationCommand>) -> Collection<CommandId, ApplicationCommand> {
    commands
        .into_iter()
        .map(|command| (command.id, command))
        .collect()
}

#[derive(Debug)]
pub struct CommandManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    application_id: ApplicationId,
}

impl<B: Backend> Clone for CommandManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for CommandManager<'_, B> {}

impl<'a, B: Backend> CommandManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, application_id: ApplicationId) -> Self {
        Self {
            ctx,
            application_id,
        }
    }

    #[must_use]
    pub const fn application_id(self) -> ApplicationId {
        self.application_id
    }

    pub fn fetch(
        self,
        command_id: CommandId,
    ) -> Fetch<
        ApplicationCommand,
        impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a,
    > {
        let Self {
            ctx,
            application_id,
        } = self;
        Fetch::new(None, async move {
            ctx.rest()
                .get_global_application_command(application_id, command_id)
                .await
        })
    }

    pub async fn list(
        self,
        query: &GetApplicationCommands,
    ) -> Result<Collection<CommandId, ApplicationCommand>, Error> {
        let commands = self
            .ctx
            .rest()
            .get_global_application_commands(self.application_id, query)
            .await?;
        Ok(by_id(commands))
    }

    pub async fn create(
        self,
        body: &CreateApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        self.ctx
            .rest()
            .create_global_application_command(self.application_id, body)
            .await
    }

    pub async fn edit(
        self,
        command_id: CommandId,
        body: &EditApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        self.ctx
            .rest()
            .edit_global_application_command(self.application_id, command_id, body)
            .await
    }

    pub async fn delete(self, command_id: CommandId) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_global_application_command(self.application_id, command_id)
            .await
    }

    pub async fn set(
        self,
        commands: &[CreateApplicationCommand],
    ) -> Result<Collection<CommandId, ApplicationCommand>, Error> {
        let commands = self
            .ctx
            .rest()
            .bulk_overwrite_global_application_commands(self.application_id, commands)
            .await?;
        Ok(by_id(commands))
    }
}

#[derive(Debug)]
pub struct GuildCommandManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    application_id: ApplicationId,
    guild_id: GuildId,
}

impl<B: Backend> Clone for GuildCommandManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for GuildCommandManager<'_, B> {}

impl<'a, B: Backend> GuildCommandManager<'a, B> {
    pub(crate) const fn new(
        ctx: &'a Context<B>,
        application_id: ApplicationId,
        guild_id: GuildId,
    ) -> Self {
        Self {
            ctx,
            application_id,
            guild_id,
        }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    pub fn fetch(
        self,
        command_id: CommandId,
    ) -> Fetch<
        ApplicationCommand,
        impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a,
    > {
        let Self {
            ctx,
            application_id,
            guild_id,
        } = self;
        Fetch::new(None, async move {
            ctx.rest()
                .get_guild_application_command(application_id, guild_id, command_id)
                .await
        })
    }

    pub async fn list(
        self,
        query: &GetApplicationCommands,
    ) -> Result<Collection<CommandId, ApplicationCommand>, Error> {
        let commands = self
            .ctx
            .rest()
            .get_guild_application_commands(self.application_id, self.guild_id, query)
            .await?;
        Ok(by_id(commands))
    }

    pub async fn create(
        self,
        body: &CreateApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        self.ctx
            .rest()
            .create_guild_application_command(self.application_id, self.guild_id, body)
            .await
    }

    pub async fn edit(
        self,
        command_id: CommandId,
        body: &EditApplicationCommand,
    ) -> Result<ApplicationCommand, Error> {
        self.ctx
            .rest()
            .edit_guild_application_command(self.application_id, self.guild_id, command_id, body)
            .await
    }

    pub async fn delete(self, command_id: CommandId) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_application_command(self.application_id, self.guild_id, command_id)
            .await
    }

    pub async fn set(
        self,
        commands: &[CreateApplicationCommand],
    ) -> Result<Collection<CommandId, ApplicationCommand>, Error> {
        let commands = self
            .ctx
            .rest()
            .bulk_overwrite_guild_application_commands(self.application_id, self.guild_id, commands)
            .await?;
        Ok(by_id(commands))
    }

    pub async fn permissions(
        self,
    ) -> Result<Collection<GenericId, GuildCommandPermissions>, Error> {
        let permissions = self
            .ctx
            .rest()
            .get_guild_application_command_permissions(self.application_id, self.guild_id)
            .await?;
        Ok(permissions
            .into_iter()
            .map(|permissions| (permissions.id, permissions))
            .collect())
    }

    pub async fn command_permissions(
        self,
        command_id: CommandId,
    ) -> Result<GuildCommandPermissions, Error> {
        self.ctx
            .rest()
            .get_application_command_permissions(self.application_id, self.guild_id, command_id)
            .await
    }
}
