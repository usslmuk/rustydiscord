use compact_str::CompactString;
use serde::Serialize;

use crate::{
    application::{ApplicationFlags, EventWebhookStatus, InstallParams, IntegrationTypesConfig},
    util::{ImageData, Patch},
    webhook_event::WebhookEventType,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct EditCurrentApplication {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_install_url: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_connections_verification_url: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_params: Option<InstallParams>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_types_config: Option<IntegrationTypesConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ApplicationFlags>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub icon: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub cover_image: Patch<ImageData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactions_endpoint_url: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<CompactString>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_url: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_status: Option<EventWebhookStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_webhooks_types: Option<Vec<WebhookEventType>>,
}
