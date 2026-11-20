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
#[cfg(feature = "sharding")]
#[cfg_attr(docsrs, doc(cfg(feature = "sharding")))]
pub mod sharding;
#[cfg(feature = "voice")]
#[cfg_attr(docsrs, doc(cfg(feature = "voice")))]
pub mod voice;
#[cfg(feature = "voice")]
mod voice_client;
pub mod webhook;

pub mod brokers;
pub mod builders;
pub mod cache;
pub mod collection;
pub mod formatters;
#[cfg(feature = "gateway")]
#[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
pub mod gateway;
pub mod model;
pub mod opcodes;
pub mod proxy;
pub mod rest;
pub mod util;

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

pub mod prelude {
    pub use crate::{
        Client, Context, Events, HandlerError,
        ext::{ChannelExt, GuildExt, InteractionExt, MemberExt, MessageExt, UserExt, WebhookExt},
        model::gateway::Intents,
    };
}
