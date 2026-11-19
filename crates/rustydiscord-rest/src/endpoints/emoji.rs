use rustydiscord_model::{
    emoji::Emoji,
    id::{ApplicationId, EmojiId, GuildId},
    rest::{CreateApplicationEmoji, CreateGuildEmoji, ModifyApplicationEmoji, ModifyGuildEmoji},
};

use super::Items;
use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_guild_emojis(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Emoji>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ListGuildEmojis { guild_id }))
                .await
        })
    }

    pub fn get_guild_emoji(
        &self,
        guild_id: GuildId,
        emoji_id: EmojiId,
    ) -> impl Future<Output = Result<Emoji, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildEmoji { guild_id, emoji_id }))
                .await
        })
    }

    pub fn create_guild_emoji<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildEmoji,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildEmoji { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_emoji<'a>(
        &'a self,
        guild_id: GuildId,
        emoji_id: EmojiId,
        body: &'a ModifyGuildEmoji,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildEmoji { guild_id, emoji_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn delete_guild_emoji<'a>(
        &'a self,
        guild_id: GuildId,
        emoji_id: EmojiId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteGuildEmoji { guild_id, emoji_id }).reason(reason))
                .await
        })
    }
}

impl Rest {
    pub fn list_application_emojis(
        &self,
        application_id: ApplicationId,
    ) -> impl Future<Output = Result<Vec<Emoji>, Error>> + Send + '_ {
        boxed(async move {
            let list: Items<Emoji> = self
                .request(Request::new(Route::ListApplicationEmojis {
                    application_id,
                }))
                .await?;
            Ok(list.items)
        })
    }

    pub fn get_application_emoji(
        &self,
        application_id: ApplicationId,
        emoji_id: EmojiId,
    ) -> impl Future<Output = Result<Emoji, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetApplicationEmoji {
                application_id,
                emoji_id,
            }))
            .await
        })
    }

    pub fn create_application_emoji<'a>(
        &'a self,
        application_id: ApplicationId,
        body: &'a CreateApplicationEmoji,
    ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::CreateApplicationEmoji { application_id }).json(body)?)
                .await
        })
    }

    pub fn modify_application_emoji<'a>(
        &'a self,
        application_id: ApplicationId,
        emoji_id: EmojiId,
        body: &'a ModifyApplicationEmoji,
    ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyApplicationEmoji {
                    application_id,
                    emoji_id,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn delete_application_emoji(
        &self,
        application_id: ApplicationId,
        emoji_id: EmojiId,
    ) -> impl Future<Output = Result<(), Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::DeleteApplicationEmoji {
                application_id,
                emoji_id,
            }))
            .await
        })
    }
}
