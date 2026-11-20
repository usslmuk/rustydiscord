use std::{future::Future, sync::Arc};

use crate::cache::{Backend, CachedMessage};
use crate::collection::Collection;
use crate::model::{
    id::{ChannelId, GuildId, MessageId, WebhookId},
    rest::{CreateWebhook, EditWebhookMessage, ExecuteWebhook, ModifyWebhook},
    webhook::Webhook,
};
use crate::rest::Error;

use super::Fetch;
use crate::Context;

#[derive(Debug)]
pub struct WebhookManager<'a, B: Backend> {
    ctx: &'a Context<B>,
}

impl<B: Backend> Clone for WebhookManager<'_, B> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<B: Backend> Copy for WebhookManager<'_, B> {}

impl<'a, B: Backend> WebhookManager<'a, B> {
    pub(crate) const fn new(ctx: &'a Context<B>) -> Self {
        Self { ctx }
    }

    pub fn fetch(
        self,
        webhook_id: WebhookId,
    ) -> Fetch<Webhook, impl Future<Output = Result<Webhook, Error>> + Send + 'a> {
        let ctx = self.ctx;
        Fetch::new(
            None,
            async move { ctx.rest().get_webhook(webhook_id).await },
        )
    }

    pub async fn fetch_with_token(
        self,
        webhook_id: WebhookId,
        token: &str,
    ) -> Result<Webhook, Error> {
        self.ctx
            .rest()
            .get_webhook_with_token(webhook_id, token)
            .await
    }

    pub async fn create(
        self,
        channel_id: ChannelId,
        body: &CreateWebhook,
        reason: Option<&str>,
    ) -> Result<Webhook, Error> {
        self.ctx
            .rest()
            .create_webhook(channel_id, body, reason)
            .await
    }

    pub async fn list_channel(
        self,
        channel_id: ChannelId,
    ) -> Result<Collection<WebhookId, Webhook>, Error> {
        let webhooks = self.ctx.rest().get_channel_webhooks(channel_id).await?;
        Ok(by_id(webhooks))
    }

    pub async fn list_guild(
        self,
        guild_id: GuildId,
    ) -> Result<Collection<WebhookId, Webhook>, Error> {
        let webhooks = self.ctx.rest().get_guild_webhooks(guild_id).await?;
        Ok(by_id(webhooks))
    }

    pub async fn edit(
        self,
        webhook_id: WebhookId,
        body: &ModifyWebhook,
        reason: Option<&str>,
    ) -> Result<Webhook, Error> {
        self.ctx
            .rest()
            .modify_webhook(webhook_id, body, reason)
            .await
    }

    pub async fn delete(self, webhook_id: WebhookId, reason: Option<&str>) -> Result<(), Error> {
        self.ctx.rest().delete_webhook(webhook_id, reason).await
    }

    pub fn send<'t>(
        self,
        webhook_id: WebhookId,
        token: &'t str,
        body: impl Into<ExecuteWebhook>,
    ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 't
    where
        'a: 't,
    {
        let body = body.into();
        let ctx = self.ctx;
        async move {
            let message = ctx
                .rest()
                .execute_webhook_and_wait(webhook_id, token, &body)
                .await?;
            Ok(ctx.store_message(&message))
        }
    }

    pub async fn fetch_message(
        self,
        webhook_id: WebhookId,
        token: &str,
        message_id: MessageId,
        thread: Option<ChannelId>,
    ) -> Result<Arc<CachedMessage>, Error> {
        let message = self
            .ctx
            .rest()
            .get_webhook_message(webhook_id, token, message_id, thread)
            .await?;
        Ok(self.ctx.store_message(&message))
    }

    pub async fn edit_message(
        self,
        webhook_id: WebhookId,
        token: &str,
        message_id: MessageId,
        body: &EditWebhookMessage,
    ) -> Result<Arc<CachedMessage>, Error> {
        let message = self
            .ctx
            .rest()
            .edit_webhook_message(webhook_id, token, message_id, body)
            .await?;
        Ok(self.ctx.store_message(&message))
    }

    pub async fn delete_message(
        self,
        webhook_id: WebhookId,
        token: &str,
        message_id: MessageId,
        thread: Option<ChannelId>,
    ) -> Result<(), Error> {
        self.ctx
            .rest()
            .delete_webhook_message(webhook_id, token, message_id, thread)
            .await
    }
}

fn by_id(webhooks: Vec<Webhook>) -> Collection<WebhookId, Webhook> {
    webhooks
        .into_iter()
        .map(|webhook| (webhook.id, webhook))
        .collect()
}
