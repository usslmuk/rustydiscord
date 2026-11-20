#![cfg_attr(docsrs, feature(doc_cfg))]

mod backend;
mod config;
mod dashmap_store;
mod limits;
mod metrics;
mod model;
mod moka_store;
mod permissions;
mod redis_store;
mod state;
mod update;
mod view;

pub use backend::{Backend, Store};
pub use config::{CacheConfig, CacheConfigBuilder, Limit, Resource, Sweep, SweepConfig};
pub use dashmap_store::{DashMapBackend, DashMapStore};
pub use metrics::CacheMetrics;
pub use model::{CachedGuild, CachedMember, CachedMessage, CachedPresence, CachedUser};
pub use moka_store::{MokaBackend, MokaStore};
pub use redis_store::{RedisBackend, RedisConfig, RedisStore};
pub use state::Cache;
pub use update::{Previous, UpdateCache};
pub use view::{CacheView, ChannelView, GuildView};
