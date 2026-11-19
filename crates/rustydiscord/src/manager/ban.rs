use std::future::Future;

use rustydiscord_cache::Backend;
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    guild::Ban,
    id::{GuildId, UserId},
    rest::{CreateGuildBan, GetGuildBans},
};
use rustydiscord_rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct BanManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for BanManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for BanManager<'_, B> {}

impl<'a, B: Backend> BanManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    pub fn fetch(
        self,
        user_id: UserId,
    ) -> Fetch<Ban, impl Future<Output = Result<Ban, Error>> + Send + 'a> {
        let Self { ctx, guild_id } = self;
        Fetch::new(None, async move {
            ctx.rest().get_guild_ban(guild_id, user_id).await
        })
    }

    pub async fn list(self, query: &GetGuildBans) -> Result<Collection<UserId, Ban>, Error> {
        let bans = self.ctx.rest().get_guild_bans(self.guild_id, query).await?;
        Ok(bans.into_iter().map(|ban| (ban.user.id, ban)).collect())
    }

    pub async fn create(
        self,
        user_id: UserId,
        body: &CreateGuildBan,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .create_guild_ban(self.guild_id, user_id, body, reason)
            .await
    }

    pub async fn remove(self, user_id: UserId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .remove_guild_ban(self.guild_id, user_id, reason)
            .await
    }
}
