use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    guild::PartialGuild,
    id::{ApplicationId, ChannelId, GenericId, GuildId, SkuId, UserId},
    macros::{int_enum, int_flags, str_enum},
    oauth2::Scope,
    permissions::Permissions,
    team::Team,
    user::User,
    util::ImageHash,
    webhook_event::WebhookEventType,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Application {
    pub id: ApplicationId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,

    pub description: CompactString,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rpc_origins: Vec<CompactString>,

    pub bot_public: bool,

    pub bot_require_code_grant: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<Box<User>>,

    pub verify_key: CompactString,

    pub team: Option<Box<Team>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<Box<PartialGuild>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_sku_id: Option<SkuId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slug: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ApplicationFlags>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::model::serde_util::option_flags_string"
    )]
    pub flags_new: Option<ApplicationFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_guild_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_user_install_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_user_authorization_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub redirect_uris: Vec<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_webhooks_status: Option<EventWebhookStatus>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub event_webhooks_types: Vec<WebhookEventType>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub install_params: Option<InstallParams>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_types_config: Option<IntegrationTypesConfig>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<CompactString>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PartialApplication {
    pub id: ApplicationId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cover_image: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<ApplicationFlags>,
}

int_enum! {



    #[doc(alias = "ApplicationIntegrationType")]
    pub enum IntegrationType: u8 {

        GuildInstall = 0,

        UserInstall = 1,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IntegrationTypesConfig {
    #[serde(rename = "0", default, skip_serializing_if = "Option::is_none")]
    pub guild_install: Option<IntegrationTypeConfig>,

    #[serde(rename = "1", default, skip_serializing_if = "Option::is_none")]
    pub user_install: Option<IntegrationTypeConfig>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IntegrationTypeConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub oauth2_install_params: Option<InstallParams>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InstallParams {
    pub scopes: Vec<Scope>,

    pub permissions: Permissions,
}

impl InstallParams {
    #[must_use]
    pub fn new(scopes: impl IntoIterator<Item = Scope>, permissions: Permissions) -> Self {
        Self {
            scopes: scopes.into_iter().collect(),
            permissions,
        }
    }
}

int_enum! {



    pub enum EventWebhookStatus: u8 {

        Disabled = 1,

        Enabled = 2,

        DisabledByDiscord = 3,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityInstance {
    pub application_id: ApplicationId,

    pub instance_id: CompactString,

    pub launch_id: GenericId,

    pub location: ActivityLocation,

    pub users: Vec<UserId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ActivityLocation {
    pub id: CompactString,

    pub kind: ActivityLocationKind,

    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,
}

str_enum! {



    pub enum ActivityLocationKind {

        GuildChannel = "gc",

        PrivateChannel = "pc",
    }
}

int_flags! {

















    pub struct ApplicationFlags: u64 {




        const EMBEDDED_RELEASED = 1 << 1;




        const MANAGED_EMOJI = 1 << 2;




        const EMBEDDED_IAP = 1 << 3;




        const GROUP_DM_CREATE = 1 << 4;


        const APPLICATION_AUTO_MODERATION_RULE_CREATE_BADGE = 1 << 6;




        const RPC_HAS_CONNECTED = 1 << 11;

        const GATEWAY_PRESENCE = 1 << 12;

        const GATEWAY_PRESENCE_LIMITED = 1 << 13;

        const GATEWAY_GUILD_MEMBERS = 1 << 14;


        const GATEWAY_GUILD_MEMBERS_LIMITED = 1 << 15;

        const VERIFICATION_PENDING_GUILD_LIMIT = 1 << 16;

        const EMBEDDED = 1 << 17;

        const GATEWAY_MESSAGE_CONTENT = 1 << 18;


        const GATEWAY_MESSAGE_CONTENT_LIMITED = 1 << 19;




        const EMBEDDED_FIRST_PARTY = 1 << 20;


        const APPLICATION_COMMAND_BADGE = 1 << 23;
    }
}
