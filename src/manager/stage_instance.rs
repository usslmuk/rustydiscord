use std::{future::Future, sync::Arc};

use crate::cache::{Backend, GuildView};
use crate::model::{
    id::{ChannelId, GuildId, StageInstanceId},
    rest::{CreateStageInstance, ModifyStageInstance},
    stage_instance::StageInstance,
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct StageInstanceManager<'a, B: Backend> {
    ctx: &'a Context<B>,
    guild_id: GuildId,
}

impl<B: Backend> Clone for StageInstanceManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for StageInstanceManager<'_, B> {}

impl<'a, B: Backend> StageInstanceManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>, guild_id: GuildId) -> Self {
        Self { ctx, guild_id }
    }

    #[must_use]
    pub const fn guild_id(self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub fn cache(self) -> Option<GuildView<'a, StageInstanceId, StageInstance, B>> {
        self.ctx
            .cache()
            .map(|cache| cache.stage_instances(self.guild_id))
    }

    pub fn fetch(
        self,
        channel_id: ChannelId,
    ) -> Fetch<
        Arc<StageInstance>,
        impl Future<Output = Result<Arc<StageInstance>, Error>> + Send + 'a,
    > {
        let cached = self.ctx.ops().stage_instance_of(self.guild_id, channel_id);
        let Self { ctx, guild_id } = self;
        Fetch::new(cached, async move {
            let stage = ctx.rest().get_stage_instance(channel_id).await?;
            Ok(ctx.store_stage_instance(guild_id, &stage))
        })
    }

    pub async fn create(
        self,
        body: &CreateStageInstance,
        reason: Option<&str>,
    ) -> Result<Arc<StageInstance>, Error> {
        let stage = self.ctx.rest().create_stage_instance(body, reason).await?;
        Ok(self.ctx.store_stage_instance(self.guild_id, &stage))
    }

    pub async fn edit(
        self,
        channel_id: ChannelId,
        body: &ModifyStageInstance,
        reason: Option<&str>,
    ) -> Result<Arc<StageInstance>, Error> {
        let stage = self
            .ctx
            .rest()
            .modify_stage_instance(channel_id, body, reason)
            .await?;
        Ok(self.ctx.store_stage_instance(self.guild_id, &stage))
    }

    pub async fn delete(self, channel_id: ChannelId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_stage_instance(channel_id, reason)
            .await
    }
}
