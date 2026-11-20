use core::fmt;
use std::future::Future;

use crate::model::{
    id::{ChannelId, MessageId, WebhookId},
    message::Message,
    rest::{EditWebhookMessage, ExecuteWebhook, ModifyWebhook},
    webhook::Webhook,
};
use crate::rest::{Error, Rest};

#[derive(Clone)]
pub struct WebhookClient {
    rest: Rest,
    id: WebhookId,
    token: String,
}

impl WebhookClient {
    #[must_use]
    pub fn new(id: WebhookId, token: impl Into<String>) -> Self {
        Self {
            rest: Rest::builder().build(),
            id,
            token: token.into(),
        }
    }

    pub fn from_url(url: &str) -> Result<Self, InvalidWebhookUrl> {
        let (id, token) = parse_url(url).ok_or(InvalidWebhookUrl(()))?;
        Ok(Self::new(id, token))
    }

    #[must_use]
    pub fn rest(mut self, rest: Rest) -> Self {
        self.rest = rest;
        self
    }

    #[must_use]
    pub const fn id(&self) -> WebhookId {
        self.id
    }

    pub fn send(
        &self,
        body: impl Into<ExecuteWebhook>,
    ) -> impl Future<Output = Result<Message, Error>> + Send + '_ {
        let body = body.into();
        async move {
            self.rest
                .execute_webhook_and_wait(self.id, &self.token, &body)
                .await
        }
    }

    pub async fn fetch_message(
        &self,
        message_id: MessageId,
        thread: Option<ChannelId>,
    ) -> Result<Message, Error> {
        self.rest
            .get_webhook_message(self.id, &self.token, message_id, thread)
            .await
    }

    pub async fn edit_message(
        &self,
        message_id: MessageId,
        body: &EditWebhookMessage,
    ) -> Result<Message, Error> {
        self.rest
            .edit_webhook_message(self.id, &self.token, message_id, body)
            .await
    }

    pub async fn delete_message(
        &self,
        message_id: MessageId,
        thread: Option<ChannelId>,
    ) -> Result<(), Error> {
        self.rest
            .delete_webhook_message(self.id, &self.token, message_id, thread)
            .await
    }

    pub async fn fetch(&self) -> Result<Webhook, Error> {
        self.rest.get_webhook_with_token(self.id, &self.token).await
    }

    pub async fn edit(&self, body: &ModifyWebhook) -> Result<Webhook, Error> {
        self.rest
            .modify_webhook_with_token(self.id, &self.token, body)
            .await
    }

    pub async fn delete(&self) -> Result<(), Error> {
        self.rest
            .delete_webhook_with_token(self.id, &self.token)
            .await
    }
}

impl fmt::Debug for WebhookClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebhookClient")
            .field("id", &self.id)
            .field("token", &"<redacted>")
            .finish_non_exhaustive()
    }
}

fn parse_url(url: &str) -> Option<(WebhookId, &str)> {
    let url = url.strip_prefix("https://")?;
    let url = url.split(['?', '#']).next()?;
    let (host, path) = url.split_once('/')?;
    let host = host
        .strip_prefix("canary.")
        .or_else(|| host.strip_prefix("ptb."))
        .unwrap_or(host);
    if host != "discord.com" && host != "discordapp.com" {
        return None;
    }
    let path = path.strip_prefix("api/")?;
    let path = match path.split_once('/') {
        Some((version, rest))
            if version.len() > 1
                && version.starts_with('v')
                && version[1..].bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            rest
        }
        _ => path,
    };
    let path = path.strip_prefix("webhooks/")?;
    let path = path.strip_suffix('/').unwrap_or(path);
    let (id, token) = path.split_once('/')?;
    let id = WebhookId::new_checked(id.parse().ok()?)?;
    let valid = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_';
    (!token.is_empty() && token.bytes().all(valid)).then_some((id, token))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("not a Discord webhook URL (https://discord.com/api/webhooks/{{id}}/{{token}})")]
pub struct InvalidWebhookUrl(());

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WebhookError {
    #[error("the webhook has no token")]
    MissingToken,

    #[error("a REST request failed")]
    Rest(#[from] Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn webhook_urls_are_parsed() {
        let id = WebhookId::new(123);
        for url in [
            "https://discord.com/api/webhooks/123/abc-DEF_9",
            "https://discord.com/api/webhooks/123/abc-DEF_9/",
            "https://discordapp.com/api/webhooks/123/abc-DEF_9",
            "https://canary.discord.com/api/webhooks/123/abc-DEF_9",
            "https://ptb.discord.com/api/v10/webhooks/123/abc-DEF_9",
            "https://discord.com/api/webhooks/123/abc-DEF_9?thread_id=5",
        ] {
            assert_eq!(parse_url(url), Some((id, "abc-DEF_9")), "{url}");
        }
        for url in [
            "http://discord.com/api/webhooks/123/abc",
            "https://example.com/api/webhooks/123/abc",
            "https://discord.com.evil.example/api/webhooks/123/abc",
            "https://discord.com/webhooks/123/abc",
            "https://discord.com/api/webhooks/123",
            "https://discord.com/api/webhooks/123/",
            "https://discord.com/api/webhooks/0/abc",
            "https://discord.com/api/webhooks/abc/abc",
            "https://discord.com/api/webhooks/123/ab%2Fc",
            "https://discord.com/api/webhooks/123/abc/extra",
            "https://discord.com/api/vx/webhooks/123/abc",
            "",
        ] {
            assert_eq!(parse_url(url), None, "{url}");
        }
    }

    #[test]
    fn the_token_is_redacted() {
        let client = WebhookClient::new(WebhookId::new(1), "secret-token-value");
        let debug = format!("{client:?}");
        assert!(!debug.contains("secret"), "{debug}");
    }
}
