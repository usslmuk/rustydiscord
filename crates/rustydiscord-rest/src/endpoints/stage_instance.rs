use rustydiscord_model::{
    id::ChannelId,
    rest::{CreateStageInstance, ModifyStageInstance},
    stage_instance::StageInstance,
};

use crate::client::boxed;
use crate::{Error, Request, Rest, Route};

impl Rest {
    pub fn create_stage_instance<'a>(
        &'a self,
        body: &'a CreateStageInstance,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<StageInstance, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateStageInstance)
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn get_stage_instance(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<StageInstance, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetStageInstance { channel_id }))
                .await
        })
    }

    pub fn modify_stage_instance<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a ModifyStageInstance,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<StageInstance, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyStageInstance { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn delete_stage_instance<'a>(
        &'a self,
        channel_id: ChannelId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteStageInstance { channel_id }).reason(reason))
                .await
        })
    }
}
