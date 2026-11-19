use rustydiscord_model::{
    id::{ChannelId, GuildId, MessageId, WebhookId},
    message::Message,
    rest::{CreateWebhook, EditWebhookMessage, ExecuteWebhook, ModifyWebhook},
    webhook::Webhook,
};

use crate::client::boxed;
use crate::{
    Error, Request, Rest, Route,
    query::{Query, QueryWriter},
};

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct WebhookQuery {
    pub(crate) wait: Option<bool>,
    pub(crate) thread_id: Option<ChannelId>,
    pub(crate) with_components: Option<bool>,
}

impl WebhookQuery {
    pub(crate) const fn of_edit(body: &EditWebhookMessage) -> Self {
        Self {
            wait: None,
            thread_id: body.thread_id,
            with_components: body.with_components,
        }
    }

    const fn of_execute(body: &ExecuteWebhook, wait: bool) -> Self {
        Self {
            wait: if wait { Some(true) } else { None },
            thread_id: body.thread_id,
            with_components: body.with_components,
        }
    }

    const fn thread(thread_id: Option<ChannelId>) -> Self {
        Self {
            wait: None,
            thread_id,
            with_components: None,
        }
    }
}

impl Query for WebhookQuery {
    fn write(&self, query: &mut QueryWriter<'_>) {
        query.optional("wait", self.wait.as_ref());
        query.optional("thread_id", self.thread_id.as_ref());
        query.optional("with_components", self.with_components.as_ref());
    }
}

impl Rest {
    pub fn create_webhook<'a>(
        &'a self,
        channel_id: ChannelId,
        body: &'a CreateWebhook,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::CreateWebhook { channel_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn get_channel_webhooks(
        &self,
        channel_id: ChannelId,
    ) -> impl Future<Output = Result<Vec<Webhook>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetChannelWebhooks { channel_id }))
                .await
        })
    }

    pub fn get_guild_webhooks(
        &self,
        guild_id: GuildId,
    ) -> impl Future<Output = Result<Vec<Webhook>, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetGuildWebhooks { guild_id }))
                .await
        })
    }

    pub fn get_webhook(
        &self,
        webhook_id: WebhookId,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send + '_ {
        boxed(async move {
            self.call(Request::new(Route::GetWebhook { webhook_id }))
                .await
        })
    }

    pub fn get_webhook_with_token<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::GetWebhookWithToken {
                webhook_id,
                webhook_token,
            }))
            .await
        })
    }

    pub fn modify_webhook<'a>(
        &'a self,
        webhook_id: WebhookId,
        body: &'a ModifyWebhook,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyWebhook { webhook_id })
                    .json(body)?
                    .reason(reason),
            )
            .await
        })
    }

    pub fn modify_webhook_with_token<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
        body: &'a ModifyWebhook,
    ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a {
        boxed(async move {
            self.call(
                Request::new(Route::ModifyWebhookWithToken {
                    webhook_id,
                    webhook_token,
                })
                .json(body)?,
            )
            .await
        })
    }

    pub fn delete_webhook<'a>(
        &'a self,
        webhook_id: WebhookId,
        reason: Option<&'a str>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteWebhook { webhook_id }).reason(reason))
                .await
        })
    }

    pub fn delete_webhook_with_token<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            self.call(Request::new(Route::DeleteWebhookWithToken {
                webhook_id,
                webhook_token,
            }))
            .await
        })
    }
}

impl Rest {
    pub fn execute_webhook<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
        body: &'a ExecuteWebhook,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::of_execute(body, false);
            self.request_ignoring_body(
                Request::new(Route::ExecuteWebhook {
                    webhook_id,
                    webhook_token,
                })
                .query(&query)
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn execute_webhook_and_wait<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
        body: &'a ExecuteWebhook,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::of_execute(body, true);
            self.call(
                Request::new(Route::ExecuteWebhook {
                    webhook_id,
                    webhook_token,
                })
                .query(&query)
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn get_webhook_message<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
        message_id: MessageId,
        thread_id: Option<ChannelId>,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::thread(thread_id);
            self.call(
                Request::new(Route::GetWebhookMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                })
                .query(&query),
            )
            .await
        })
    }

    pub fn edit_webhook_message<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
        message_id: MessageId,
        body: &'a EditWebhookMessage,
    ) -> impl Future<Output = Result<Message, Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::of_edit(body);
            self.call(
                Request::new(Route::EditWebhookMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                })
                .query(&query)
                .json(body)?
                .files(&body.files),
            )
            .await
        })
    }

    pub fn delete_webhook_message<'a>(
        &'a self,
        webhook_id: WebhookId,
        webhook_token: &'a str,
        message_id: MessageId,
        thread_id: Option<ChannelId>,
    ) -> impl Future<Output = Result<(), Error>> + Send + 'a {
        boxed(async move {
            let query = WebhookQuery::thread(thread_id);
            self.call(
                Request::new(Route::DeleteWebhookMessage {
                    webhook_id,
                    webhook_token,
                    message_id,
                })
                .query(&query),
            )
            .await
        })
    }
}
