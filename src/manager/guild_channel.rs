use std::sync::Arc;

use crate::cache::Backend;
use crate::collection::Collection;
use crate::model::{
    channel::Channel,
    id::{ChannelId, GuildId},
    rest::{CreateGuildChannel, ModifyGuildChannelPosition},
};
use crate::rest::Error;

use super::FetchedThreads;
use crate::Context;

#[derive(Debug)]
pub struct GuildChannelManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for GuildChannelManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for GuildChannelManager<'_, B> {}

impl<'a, B: Backend> GuildChannelManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    pub async fn list(self) -> Result<Collection<ChannelId, Arc<Channel>>, Error> {
        let channels = self.ctx.rest().get_guild_channels(self.guild_id).await?;
        Ok(channels
            .iter()
            .map(|channel| (channel.id(), self.ctx.store_channel(channel)))
            .collect())
    }

    pub async fn create(
        self,
        body: &CreateGuildChannel,
        reason: Option<&str>,
    ) -> Result<Arc<Channel>, Error> {
        let channel = self
            .ctx
            .rest()
            .create_guild_channel(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_channel(&channel))
    }

    pub async fn set_positions(
        self,
        positions: &[ModifyGuildChannelPosition],
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .modify_guild_channel_positions(self.guild_id, positions, reason)
            .await
    }

    pub async fn list_active_threads(self) -> Result<FetchedThreads, Error> {
        let active = self
            .ctx
            .rest()
            .list_active_guild_threads(self.guild_id)
            .await?;
        Ok(self
            .ctx
            .fetched_threads(&active.threads, active.members, false))
    }
}
