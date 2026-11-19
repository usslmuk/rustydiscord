use std::future::Future;

use rustydiscord_cache::Backend;
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    emoji::Emoji,
    id::{ApplicationId, EmojiId},
    rest::{CreateApplicationEmoji, ModifyApplicationEmoji},
};
use rustydiscord_rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct ApplicationEmojiManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    application_id: ApplicationId,
}

impl<B: Backend> Clone for ApplicationEmojiManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for ApplicationEmojiManager<'_, B> {}

impl<'a, B: Backend> ApplicationEmojiManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, application_id: ApplicationId) -> Self {
        Self {
            ctx,
            application_id,
        }
    }

    pub fn fetch(
        self,
        emoji_id: EmojiId,
    ) -> Fetch<Emoji, impl Future<Output = Result<Emoji, Error>> + Send + 'a> {
        let Self {
            ctx,
            application_id,
        } = self;
        Fetch::new(None, async move {
            ctx.rest()
                .get_application_emoji(application_id, emoji_id)
                .await
        })
    }

    pub async fn list(self) -> Result<Collection<EmojiId, Emoji>, Error> {
        let emojis = self
            .ctx
            .rest()
            .list_application_emojis(self.application_id)
            .await?;
        Ok(emojis.into_iter().map(|emoji| (emoji.id, emoji)).collect())
    }

    pub async fn create(self, body: &CreateApplicationEmoji) -> Result<Emoji, Error> {
        self.ctx
            .rest()
            .create_application_emoji(self.application_id, body)
            .await
    }

    pub async fn edit(
        self,
        emoji_id: EmojiId,
        body: &ModifyApplicationEmoji,
    ) -> Result<Emoji, Error> {
        self.ctx
            .rest()
            .modify_application_emoji(self.application_id, emoji_id, body)
            .await
    }

    pub async fn delete(self, emoji_id: EmojiId) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_application_emoji(self.application_id, emoji_id)
            .await
    }
}
