mod application;
mod application_command;
mod application_identity_profile;
mod application_role_connection_metadata;
mod audit_log;
mod auto_moderation;
mod channel;
mod emoji;
mod entitlement;
mod guild;
mod guild_scheduled_event;
mod guild_template;
mod interaction;
mod invite;
mod lobby;
mod message;
mod oauth2;
mod poll;
mod sku;
mod soundboard;
mod stage_instance;
mod sticker;
mod subscription;
mod user;
mod voice;
mod webhook;

use crate::model::{
    permissions::Permissions,
    rest::{Gateway, GatewayBot},
};
use serde::Deserialize;

use crate::rest::client::boxed;
use crate::rest::{Error, Request, Rest, Route};

#[derive(Deserialize)]
struct PermissionsResponse {
    permissions: Permissions,
}

#[derive(Deserialize)]
struct Items<T> {
    items: Vec<T>,
}

impl Rest {
    pub fn get_gateway(&self) -> impl Future<Output = Result<Gateway, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::GetGateway)).await })
    }

    pub fn get_gateway_bot(&self) -> impl Future<Output = Result<GatewayBot, Error>> + Send + '_ {
        boxed(async move { self.call(Request::new(Route::GetGatewayBot)).await })
    }

    pub(crate) async fn request_ignoring_body(&self, request: Request<'_>) -> Result<(), Error> {
        let _: serde::de::IgnoredAny = self.call(request).await?;
        Ok(())
    }
}
