use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    id::{ApplicationId, ChannelId, GuildId, WebhookId},
    macros::int_enum,
    user::User,
    util::ImageHash,
};

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Webhook {
    pub id: WebhookId,

    #[serde(rename = "type")]
    pub kind: WebhookType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,

    pub name: Option<CompactString>,

    pub avatar: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<CompactString>,

    pub application_id: Option<ApplicationId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_guild: Option<SourceGuild>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_channel: Option<SourceChannel>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,
}

impl core::fmt::Debug for Webhook {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Webhook")
            .field("id", &self.id)
            .field("kind", &self.kind)
            .field("guild_id", &self.guild_id)
            .field("channel_id", &self.channel_id)
            .field("user", &self.user)
            .field("name", &self.name)
            .field("avatar", &self.avatar)
            .field("token", &self.token.as_ref().map(|_| "<redacted>"))
            .field("application_id", &self.application_id)
            .field("source_guild", &self.source_guild)
            .field("source_channel", &self.source_channel)
            .field("url", &self.url.as_ref().map(|_| "<redacted>"))
            .finish()
    }
}

int_enum! {



    pub enum WebhookType: u8 {

        Incoming = 1,

        ChannelFollower = 2,

        Application = 3,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SourceGuild {
    pub id: GuildId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SourceChannel {
    pub id: ChannelId,

    pub name: CompactString,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_is_redacted_in_debug() {
        let webhook: Webhook = serde_json::from_str(
            r#"{"id":"1","type":1,"channel_id":"2","name":"hook","avatar":null,
                "token":"secret-token","application_id":null}"#,
        )
        .unwrap();
        let debug = format!("{webhook:?}");
        assert!(!debug.contains("secret-token"));
        assert!(debug.contains("<redacted>"));
        assert_eq!(webhook.kind, WebhookType::Incoming);
    }
}
