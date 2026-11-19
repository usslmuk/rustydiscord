use std::{future::Future, sync::Arc};

use rustydiscord_cache::{Backend, CacheView};
use rustydiscord_model::{
    channel::{Channel, OverwriteType},
    id::{ChannelId, GenericId, RoleId, UserId},
    permissions::Permissions,
    rest::{EditChannelPermissions, ModifyGuildChannel, ModifyThread, SendSoundboardSound},
};
use rustydiscord_rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct ChannelManager<'a, B: Backend> {
    ctx: &'a Context<B>,
}

impl<B: Backend> Clone for ChannelManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for ChannelManager<'_, B> {}

impl<'a, B: Backend> ChannelManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>) -> Self {
        Self { ctx }
    }

    #[must_use]
    pub fn cache(self) -> Option<CacheView<'a, ChannelId, Channel, B>> {
        self.ctx.cache().map(rustydiscord_cache::Cache::channels)
    }

    pub fn fetch(
        self,
        channel_id: ChannelId,
    ) -> Fetch<Arc<Channel>, impl Future<Output = Result<Arc<Channel>, Error>> + Send + 'a> {
        let cached = self.ctx.ops().channel(channel_id);
        let ctx = self.ctx;
        Fetch::new(cached, async move {
            let channel = ctx.rest().get_channel(channel_id).await?;
            Ok(ctx.store_channel(&channel))
        })
    }

    pub async fn send_typing(self, channel_id: ChannelId) -> Result<(), Error> {
        self.ctx.rest().trigger_typing_indicator(channel_id).await
    }

    pub async fn edit(
        self,
        channel_id: ChannelId,
        body: &ModifyGuildChannel,
        reason: Option<&str>,
    ) -> Result<Arc<Channel>, Error> {
        let channel = self
            .ctx
            .rest()
            .modify_channel(channel_id, body, reason)
            .await?;
        Ok(self.ctx.store_channel(&channel))
    }

    pub async fn edit_thread(
        self,
        thread_id: ChannelId,
        body: &ModifyThread,
        reason: Option<&str>,
    ) -> Result<Arc<Channel>, Error> {
        let thread = self
            .ctx
            .rest()
            .modify_thread(thread_id, body, reason)
            .await?;
        Ok(self.ctx.store_channel(&thread))
    }

    pub async fn delete(
        self,
        channel_id: ChannelId,
        reason: Option<&str>,
    ) -> Result<Channel, Error> {
        self.ctx.rest().delete_channel(channel_id, reason).await
    }

    pub async fn send_soundboard_sound(
        self,
        channel_id: ChannelId,
        body: &SendSoundboardSound,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .send_soundboard_sound(channel_id, body)
            .await
    }

    pub fn set_permissions(
        self,
        channel_id: ChannelId,
        target: impl Into<OverwriteTarget>,
        allow: Permissions,
        deny: Permissions,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (id, kind) = target.into().parts();
        let mut body = EditChannelPermissions::new(kind);
        body.allow = Some(allow);
        body.deny = Some(deny);
        let ctx = self.ctx;
        async move {
            ctx.rest()
                .edit_channel_permissions(channel_id, id, &body, reason)
                .await
        }
    }

    pub fn delete_permissions(
        self,
        channel_id: ChannelId,
        target: impl Into<OverwriteTarget>,
        reason: Option<&str>,
    ) -> impl Future<Output = Result<(), Error>> + Send {
        let (id, _) = target.into().parts();
        let ctx = self.ctx;
        async move {
            ctx.rest()
                .delete_channel_permission(channel_id, id, reason)
                .await
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OverwriteTarget {
    Role(RoleId),

    Member(UserId),
}

impl OverwriteTarget {
    const fn parts(self) -> (GenericId, OverwriteType) {
        match self {
            Self::Role(id) => (id.cast(), OverwriteType::Role),
            Self::Member(id) => (id.cast(), OverwriteType::Member),
        }
    }
}

impl From<RoleId> for OverwriteTarget {
    fn from(id: RoleId) -> Self {
        Self::Role(id)
    }
}

impl From<UserId> for OverwriteTarget {
    fn from(id: UserId) -> Self {
        Self::Member(id)
    }
}
