use rustydiscord_model::{
    id::{GuildId, UserId},
    rest::{ModifyCurrentUserVoiceState, ModifyUserVoiceState},
    voice::{VoiceRegion, VoiceState},
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn list_voice_regions(
        &self,
    ) -> impl Future<Output = Result<Vec<VoiceRegion>, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::ListVoiceRegions)).await })
    }

    pub fn get_current_user_voice_state(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<VoiceState, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetCurrentUserVoiceState { guild_id }))
                .await
        })
    }

    pub fn get_user_voice_state(
        &self,
        guild_id: GuildId,
        user_id: UserId,
    ) -> impl Future<Output = Result<VoiceState, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetUserVoiceState { guild_id, user_id }))
                .await
        })
    }

    pub fn modify_current_user_voice_state<'a>(
        &'a self,
        guild_id: GuildId,
        body: &'a ModifyCurrentUserVoiceState,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::ModifyCurrentUserVoiceState { guild_id }).json(body)?,
            )
            .await
        })
    }

    pub fn modify_user_voice_state<'a>(
        &'a self,
        guild_id: GuildId,
        user_id: UserId,
        body: &'a ModifyUserVoiceState,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.request_ignoring_body(
                Request::new(Route::ModifyUserVoiceState { guild_id, user_id }).json(body)?,
            )
            .await
        })
    }
}
