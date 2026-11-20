use crate::model::{
    id::{GuildId, StickerId, StickerPackId},
    rest::{CreateGuildSticker, ModifyGuildSticker},
    sticker::{Sticker, StickerPack},
};
use serde::Deserialize;

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

#[derive(Deserialize)]
struct StickerPacks {
    sticker_packs: Vec<StickerPack>,
}

impl Rest {
    pub fn get_sticker(
        &self,
        sticker_id: StickerId,
    ) -> impl Future<Output = Result<Sticker, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetSticker { sticker_id }))
                .await
        })
    }

    pub fn list_sticker_packs(
        &self,
    ) -> impl Future<Output = Result<Vec<StickerPack>, Error>> + Send + '_ {
        boxed(async move {
            let list: StickerPacks = self.call(Request::new(Route::ListStickerPacks)).await?;
            Ok(list.sticker_packs)
        })
    }

    pub fn get_sticker_pack(
        &self,
        pack_id: StickerPackId,
    ) -> impl Future<Output = Result<StickerPack, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetStickerPack { pack_id }))
                .await
        })
    }
}

impl Rest {
    pub fn list_guild_stickers(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Sticker>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ListGuildStickers { guild_id }))
                .await
        })
    }

    pub fn get_guild_sticker(
        &self,
        guild_id: GuildId,
        sticker_id: StickerId,
    ) -> impl Future<Output = Result<Sticker, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildSticker {
                guild_id,
                sticker_id,
            }))
            .await
        })
    }

    pub fn create_guild_sticker<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildSticker,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Sticker, Error>> + Send + 'a {
        boxed(async move {
            let fields = [
                ("name", body.name.as_str()),
                ("description", body.description.as_str()),
                ("tags", body.tags.as_str()),
            ];
            self.call(
                Request::new(Route::CreateGuildSticker { guild_id })
                    .form_file("file", &body.file)
                    .form_text(&fields)
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_sticker<'a>(
        &'a self,
        guild_id: GuildId,
        sticker_id: StickerId,
        body: &'a ModifyGuildSticker,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Sticker, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildSticker {
                    guild_id,
                    sticker_id,
                })
                .json(body)?
                .reason(reason),
            )
            .await
        })
    }

    pub fn delete_guild_sticker<'a>(
        &'a self,
        guild_id: GuildId,
        sticker_id: StickerId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::DeleteGuildSticker {
                    guild_id,
                    sticker_id,
                })
                .reason(reason),
            )
            .await
        })
    }
}
