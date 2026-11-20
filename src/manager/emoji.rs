use std::{future::Future, sync::Arc};

use crate::cache::{Backend, GuildView};
use crate::collection::Collection;
use crate::model::{
    emoji::Emoji,
    id::{EmojiId, GuildId},
    rest::{CreateGuildEmoji, ModifyGuildEmoji},
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct EmojiManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for EmojiManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for EmojiManager<'_, B> {}

impl<'a, B: Backend> EmojiManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, EmojiId, Emoji, B>> {
        self.ctx.cache().map(|cache| cache.emojis(self.guild_id))
    }

    pub fn fetch(
        self,
        emoji_id: EmojiId,
    ) -> Fetch<Arc<Emoji>, impl Future<Output = Result<Arc<Emoji>, Error>> + Send + 'a> {
        let cached = self.ctx.ops().emoji(self.guild_id, emoji_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let emoji = ctx.rest().get_guild_emoji(guild_id, emoji_id).await?;
            Ok(ctx.store_emoji(guild_id, &emoji))
        })
    }

    pub async fn list(self) -> Result<Collection<EmojiId, Arc<Emoji>>, Error> {
        let emojis = self.ctx.rest().list_guild_emojis(self.guild_id).await?;
        Ok(emojis
            .iter()
            .map(|emoji| (emoji.id, self.ctx.store_emoji(self.guild_id, emoji)))
            .collect())
    }

    pub async fn create(
        self,
        body: &CreateGuildEmoji,
        reason: Option<&str>,
    ) -> Result<Arc<Emoji>, Error> {
        let emoji = self
            .ctx
            .rest()
            .create_guild_emoji(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_emoji(self.guild_id, &emoji))
    }

    pub async fn edit(
        self,
        emoji_id: EmojiId,
        body: &ModifyGuildEmoji,
        reason: Option<&str>,
    ) -> Result<Arc<Emoji>, Error> {
        let emoji = self
            .ctx
            .rest()
            .modify_guild_emoji(self.guild_id, emoji_id, body, reason)
            .await?;
        Ok(self.ctx.store_emoji(self.guild_id, &emoji))
    }

    pub async fn delete(self, emoji_id: EmojiId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_emoji(self.guild_id, emoji_id, reason)
            .await
    }
}
