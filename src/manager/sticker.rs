use std::{future::Future, sync::Arc};

use crate::cache::{Backend, GuildView};
use crate::collection::Collection;
use crate::model::{
    id::{GuildId, StickerId},
    rest::{CreateGuildSticker, ModifyGuildSticker},
    sticker::Sticker,
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct StickerManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for StickerManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for StickerManager<'_, B> {}

impl<'a, B: Backend> StickerManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, StickerId, Sticker, B>> {
        self.ctx.cache().map(|cache| cache.stickers(self.guild_id))
    }

    pub fn fetch(
        self,
        sticker_id: StickerId,
    ) -> Fetch<Arc<Sticker>, impl Future<Output = Result<Arc<Sticker>, Error>> + Send + 'a> {
        let cached = self.ctx.ops().sticker(self.guild_id, sticker_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let sticker = ctx.rest().get_guild_sticker(guild_id, sticker_id).await?;
            Ok(ctx.store_sticker(guild_id, &sticker))
        })
    }

    pub async fn list(self) -> Result<Collection<StickerId, Arc<Sticker>>, Error> {
        let stickers = self.ctx.rest().list_guild_stickers(self.guild_id).await?;
        Ok(stickers
            .iter()
            .map(|sticker| (sticker.id, self.ctx.store_sticker(self.guild_id, sticker)))
            .collect())
    }

    pub async fn create(
        self,
        body: &CreateGuildSticker,
        reason: Option<&str>,
    ) -> Result<Arc<Sticker>, Error> {
        let sticker = self
            .ctx
            .rest()
            .create_guild_sticker(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_sticker(self.guild_id, &sticker))
    }

    pub async fn edit(
        self,
        sticker_id: StickerId,
        body: &ModifyGuildSticker,
        reason: Option<&str>,
    ) -> Result<Arc<Sticker>, Error> {
        let sticker = self
            .ctx
            .rest()
            .modify_guild_sticker(self.guild_id, sticker_id, body, reason)
            .await?;
        Ok(self.ctx.store_sticker(self.guild_id, &sticker))
    }

    pub async fn delete(self, sticker_id: StickerId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_sticker(self.guild_id, sticker_id, reason)
            .await
    }
}
