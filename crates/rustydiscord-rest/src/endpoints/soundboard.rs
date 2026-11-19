use rustydiscord_model::{
    id::{ChannelId, GuildId, SoundboardSoundId},
    rest::{CreateGuildSoundboardSound, ModifyGuildSoundboardSound, SendSoundboardSound},
    soundboard::SoundboardSound,
};

use super::Items;
use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn send_soundboard_sound<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a SendSoundboardSound,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::SendSoundboardSound { channel_id }).json(body)?,
            )
            .await
        })
    }

    pub fn list_default_soundboard_sounds(
        &self,
    ) -> impl Future<Output = Result<Vec<SoundboardSound>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::ListDefaultSoundboardSounds))
                .await
        })
    }
}

impl Rest {
    pub fn list_guild_soundboard_sounds(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<SoundboardSound>, Error>> + Send + '_ {
        boxed(async move {
            let list: Items<SoundboardSound> = self
                .request(Request::new(Route::ListGuildSoundboardSounds { guild_id }))
                .await?;
            Ok(list.items)
        })
    }

    pub fn get_guild_soundboard_sound(
        &self,
        guild_id: GuildId,
        sound_id: SoundboardSoundId,
    ) -> impl Future<Output = Result<SoundboardSound, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildSoundboardSound {
                guild_id,
                sound_id,
            }))
            .await
        })
    }

    pub fn create_guild_soundboard_sound<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a CreateGuildSoundboardSound,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<SoundboardSound, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateGuildSoundboardSound { guild_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_guild_soundboard_sound<'a>(
        &'a self,
        guild_id: GuildId,
        sound_id: SoundboardSoundId,
        body: &'a ModifyGuildSoundboardSound,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<SoundboardSound, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyGuildSoundboardSound { guild_id, sound_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn delete_guild_soundboard_sound<'a>(
        &'a self,
        guild_id: GuildId,
        sound_id: SoundboardSoundId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::DeleteGuildSoundboardSound { guild_id, sound_id })
                    .reason(reason),
            )
            .await
        })
    }
}
