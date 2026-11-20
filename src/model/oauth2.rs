use core::fmt;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    application::PartialApplication, guild::Guild, macros::str_enum, user::User, util::Timestamp,
    webhook::Webhook,
};

str_enum! {













    pub enum Scope {

        ActivitiesRead = "activities.read",

        ActivitiesWrite = "activities.write",

        ApplicationsBuildsRead = "applications.builds.read",

        ApplicationsBuildsUpload = "applications.builds.upload",

        ApplicationsCommands = "applications.commands",

        ApplicationsCommandsUpdate = "applications.commands.update",

        ApplicationsCommandsPermissionsUpdate = "applications.commands.permissions.update",

        ApplicationsEntitlements = "applications.entitlements",

        ApplicationsStoreUpdate = "applications.store.update",

        Bot = "bot",

        Connections = "connections",

        DmChannelsRead = "dm_channels.read",

        Email = "email",

        GdmJoin = "gdm.join",

        Guilds = "guilds",

        GuildsJoin = "guilds.join",

        GuildsMembersRead = "guilds.members.read",

        Identify = "identify",

        IdentifyPremium = "identify.premium",

        MessagesRead = "messages.read",

        RelationshipsRead = "relationships.read",

        RoleConnectionsWrite = "role_connections.write",

        Rpc = "rpc",

        RpcActivitiesWrite = "rpc.activities.write",

        RpcNotificationsRead = "rpc.notifications.read",

        RpcVoiceRead = "rpc.voice.read",

        RpcVoiceWrite = "rpc.voice.write",

        Voice = "voice",

        WebhookIncoming = "webhook.incoming",
    }
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessTokenResponse {
    pub access_token: String,

    pub token_type: CompactString,

    pub expires_in: u64,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,

    pub scope: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<Box<Guild>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Box<Webhook>>,
}

impl AccessTokenResponse {
    pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_ {
        self.scope.split_ascii_whitespace().map(Scope::from)
    }
}

impl fmt::Debug for AccessTokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AccessTokenResponse")
            .field("access_token", &"<redacted>")
            .field("token_type", &self.token_type)
            .field("expires_in", &self.expires_in)
            .field(
                "refresh_token",
                &self.refresh_token.as_ref().map(|_| "<redacted>"),
            )
            .field("scope", &self.scope)
            .field("guild", &self.guild)
            .field("webhook", &self.webhook)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CurrentAuthorizationInformation {
    pub application: PartialApplication,

    pub scopes: Vec<Scope>,

    pub expires: Timestamp,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<Box<User>>,
}
