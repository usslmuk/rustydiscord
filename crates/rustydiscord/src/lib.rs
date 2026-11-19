#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "gateway"), allow(dead_code, unused_imports))]

mod cache_ops;
mod client;
pub mod collector;
mod context;
mod dispatch;
mod error;
pub mod event;
pub mod ext;
mod handler;
#[cfg(feature = "http-interactions")]
#[cfg_attr(docsrs, doc(cfg(feature = "http-interactions")))]
pub mod http_interactions;
pub mod manager;
pub mod router;
pub use rustydiscord_macros::handler;
#[cfg(feature = "sharding")]
#[cfg_attr(docsrs, doc(cfg(feature = "sharding")))]
pub mod sharding;
#[cfg(feature = "voice")]
#[cfg_attr(docsrs, doc(cfg(feature = "voice")))]
pub mod voice;
pub mod webhook;

pub use self::{
    client::{Client, ClientBuilder, EventStream, HandlerMode},
    context::Context,
    error::Error,
    event::Events,
    handler::HandlerError,
    webhook::WebhookClient,
};

#[cfg(feature = "gateway")]
#[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
pub use self::client::Shards;

pub use rustydiscord_model as model;

pub use rustydiscord_rest as rest;

#[cfg(feature = "gateway")]
#[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
pub use rustydiscord_gateway as gateway;

pub use rustydiscord_cache as cache;

pub use rustydiscord_builders as builders;

pub use rustydiscord_brokers as brokers;

pub use rustydiscord_opcodes as opcodes;

pub use rustydiscord_proxy as proxy;

pub use rustydiscord_formatters as formatters;

pub use rustydiscord_collection as collection;

pub use rustydiscord_util as util;

pub mod prelude {
    pub use crate::{
        Client, Context, Events, HandlerError,
        ext::{ChannelExt, GuildExt, InteractionExt, MemberExt, MessageExt, UserExt, WebhookExt},
        model::gateway::Intents,
    };
}
