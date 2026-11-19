use std::{future::Future, sync::Arc, time::Duration};

use rustydiscord_cache::{Backend, CacheView, CachedGuild};
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    audit_log::AuditLog,
    guild::Integration,
    id::{GuildId, IntegrationId},
    rest::{GetGuild, GetGuildAuditLog, ModifyGuild},
};
use rustydiscord_rest::Error;

use super::Fetch;
#[cfg(feature = "gateway")]
use super::{GatewayRequestError, GuildSoundboardSounds};
use crate::Context;

#[derive(Debug)]
pub struct GuildManager<'a, B: Backend> {
    ctx: &'a Context<B>,
}

impl<B: Backend> Clone for GuildManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for GuildManager<'_, B> {}

impl<'a, B: Backend> GuildManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>) -> Self {
        Self { ctx }
    }

    #[must_use]
    pub fn cache(self) -> Option<CacheView<'a, GuildId, CachedGuild, B>> {
        self.ctx.cache().map(rustydiscord_cache::Cache::guilds)
    }

    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    pub async fn fetch_soundboard_sounds(
        self,
        guild_ids: impl IntoIterator<Item = GuildId>,
        time: Duration,
    ) -> Result<GuildSoundboardSounds, GatewayRequestError> {
        super::gateway::fetch_soundboard_sounds(
            self.ctx.core(),
            guild_ids.into_iter().collect(),
            time,
        )
        .await
    }

    pub fn fetch(
        self,
        guild_id: GuildId,
    ) -> Fetch<Arc<CachedGuild>, impl Future<Output = Result<Arc<CachedGuild>, Error>> + Send + 'a>
    {
        let cached = self.ctx.ops().guild(guild_id);
        let ctx = self.ctx;
        Fetch::new(cached, async move {
            let mut query = GetGuild::default();
            query.with_counts = Some(true);
            let guild = ctx.rest().get_guild(guild_id, &query).await?;
            Ok(ctx.store_guild(&guild))
        })
    }

    pub async fn edit(
        self,
        guild_id: GuildId,
        body: &ModifyGuild,
        reason: Option<&str>,
    ) -> Result<Arc<CachedGuild>, Error> {
        let guild = self.ctx.rest().modify_guild(guild_id, body, reason).await?;
        Ok(self.ctx.store_guild(&guild))
    }

    pub async fn leave(self, guild_id: GuildId) -> Result<(), Error> {
        self.ctx.rest().leave_guild(guild_id).await
    }

    pub async fn audit_log(
        self,
        guild_id: GuildId,
        query: &GetGuildAuditLog,
    ) -> Result<AuditLog, Error> {
        self.ctx.rest().get_guild_audit_log(guild_id, query).await
    }

    pub async fn integrations(
        self,
        guild_id: GuildId,
    ) -> Result<Collection<IntegrationId, Integration>, Error> {
        let integrations = self.ctx.rest().get_guild_integrations(guild_id).await?;
        Ok(integrations
            .into_iter()
            .map(|integration| (integration.id, integration))
            .collect())
    }

    pub async fn delete_integration(
        self,
        guild_id: GuildId,
        integration_id: IntegrationId,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_integration(guild_id, integration_id, reason)
            .await
    }
}
