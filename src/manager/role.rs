use std::{future::Future, sync::Arc};

use crate::cache::{Backend, GuildView};
use crate::collection::Collection;
use crate::model::{
    guild::Role,
    id::{GuildId, RoleId},
    rest::{CreateGuildRole, ModifyGuildRole, ModifyGuildRolePosition},
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct RoleManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for RoleManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for RoleManager<'_, B> {}

impl<'a, B: Backend> RoleManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, RoleId, Role, B>> {
        self.ctx.cache().map(|cache| cache.roles(self.guild_id))
    }

    pub fn fetch(
        self,
        role_id: RoleId,
    ) -> Fetch<Arc<Role>, impl Future<Output = Result<Arc<Role>, Error>> + Send + 'a> {
        let cached = self.ctx.ops().role(self.guild_id, role_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let role = ctx.rest().get_guild_role(guild_id, role_id).await?;
            Ok(ctx.store_role(guild_id, &role))
        })
    }

    pub async fn list(self) -> Result<Collection<RoleId, Arc<Role>>, Error> {
        let roles = self.ctx.rest().get_guild_roles(self.guild_id).await?;
        Ok(roles
            .iter()
            .map(|role| (role.id, self.ctx.store_role(self.guild_id, role)))
            .collect())
    }

    pub async fn create(
        self,
        body: &CreateGuildRole,
        reason: Option<&str>,
    ) -> Result<Arc<Role>, Error> {
        let role = self
            .ctx
            .rest()
            .create_guild_role(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_role(self.guild_id, &role))
    }

    pub async fn edit(
        self,
        role_id: RoleId,
        body: &ModifyGuildRole,
        reason: Option<&str>,
    ) -> Result<Arc<Role>, Error> {
        let role = self
            .ctx
            .rest()
            .modify_guild_role(self.guild_id, role_id, body, reason)
            .await?;
        Ok(self.ctx.store_role(self.guild_id, &role))
    }

    pub async fn delete(self, role_id: RoleId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_role(self.guild_id, role_id, reason)
            .await
    }

    pub async fn set_positions(
        self,
        positions: &[ModifyGuildRolePosition],
        reason: Option<&str>,
    ) -> Result<Collection<RoleId, Arc<Role>>, Error> {
        let roles = self
            .ctx
            .rest()
            .modify_guild_role_positions(self.guild_id, positions, reason)
            .await?;
        Ok(roles
            .iter()
            .map(|role| (role.id, self.ctx.store_role(self.guild_id, role)))
            .collect())
    }
}
