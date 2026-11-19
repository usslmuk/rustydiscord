use std::{future::Future, sync::Arc};

use rustydiscord_cache::{Backend, CacheView, CachedMessage, CachedUser};
use rustydiscord_model::{
    channel::Channel,
    id::UserId,
    rest::{CreateDm, CreateMessage},
};
use rustydiscord_rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct UserManager<'a, B: Backend> {
    ctx: &'a Context<B>,
}

impl<B: Backend> Clone for UserManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for UserManager<'_, B> {}

impl<'a, B: Backend> UserManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>) -> Self {
        Self { ctx }
    }

    #[must_use]
    pub fn cache(self) -> Option<CacheView<'a, UserId, CachedUser, B>> {
        self.ctx.cache().map(rustydiscord_cache::Cache::users)
    }

    pub fn fetch(
        self,
        user_id: UserId,
    ) -> Fetch<Arc<CachedUser>, impl Future<Output = Result<Arc<CachedUser>, Error>> + Send + 'a>
    {
        let cached = self.ctx.ops().user(user_id);
        let ctx = self.ctx;
        Fetch::new(cached, async move {
            let user = ctx.rest().get_user(user_id).await?;
            Ok(ctx.store_user(&user))
        })
    }

    pub async fn create_dm(self, user_id: UserId) -> Result<Arc<Channel>, Error> {
        let channel = self.ctx.rest().create_dm(&CreateDm::new(user_id)).await?;
        Ok(self.ctx.store_channel(&channel))
    }

    pub fn send(
        self,
        user_id: UserId,
        body: impl Into<CreateMessage>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 'a {
        let body = body.into();
        async move {
            let channel = self.create_dm(user_id).await?;
            self.ctx.messages(channel.id()).send(body).await
        }
    }
}
