use std::{future::Future, sync::Arc};

use rustydiscord_cache::{Backend, GuildView};
use rustydiscord_collection::Collection;
use rustydiscord_model::{
    id::{GuildId, SoundboardSoundId},
    rest::{CreateGuildSoundboardSound, ModifyGuildSoundboardSound},
    soundboard::SoundboardSound,
};
use rustydiscord_rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct SoundboardSoundManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for SoundboardSoundManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for SoundboardSoundManager<'_, B> {}

impl<'a, B: Backend> SoundboardSoundManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, SoundboardSoundId, SoundboardSound, B>> {
        self.ctx
            .cache()
            .map(|cache| cache.soundboard_sounds(self.guild_id))
    }

    pub fn fetch(
        self,
        sound_id: SoundboardSoundId,
    ) -> Fetch<
        Arc<SoundboardSound>,
        impl Future<Output = Result<Arc<SoundboardSound>, Error>> + Send + 'a,
    > {
        let cached = self.ctx.ops().soundboard_sound(self.guild_id, sound_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let sound = ctx
                .rest()
                .get_guild_soundboard_sound(guild_id, sound_id)
                .await?;
            Ok(ctx.store_soundboard_sound(guild_id, &sound))
        })
    }

    pub async fn list(self) -> Result<Collection<SoundboardSoundId, Arc<SoundboardSound>>, Error> {
        let sounds = self
            .ctx
            .rest()
            .list_guild_soundboard_sounds(self.guild_id)
            .await?;
        Ok(sounds
            .iter()
            .map(|sound| {
                (
                    sound.sound_id,
                    self.ctx.store_soundboard_sound(self.guild_id, sound),
                )
            })
            .collect())
    }

    pub async fn create(
        self,
        body: &CreateGuildSoundboardSound,
        reason: Option<&str>,
    ) -> Result<Arc<SoundboardSound>, Error> {
        let sound = self
            .ctx
            .rest()
            .create_guild_soundboard_sound(self.guild_id, body, reason)
            .await?;
        Ok(self.ctx.store_soundboard_sound(self.guild_id, &sound))
    }

    pub async fn edit(
        self,
        sound_id: SoundboardSoundId,
        body: &ModifyGuildSoundboardSound,
        reason: Option<&str>,
    ) -> Result<Arc<SoundboardSound>, Error> {
        let sound = self
            .ctx
            .rest()
            .modify_guild_soundboard_sound(self.guild_id, sound_id, body, reason)
            .await?;
        Ok(self.ctx.store_soundboard_sound(self.guild_id, &sound))
    }

    pub async fn delete(
        self,
        sound_id: SoundboardSoundId,
        reason: Option<&str>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_guild_soundboard_sound(self.guild_id, sound_id, reason)
            .await
    }
}
