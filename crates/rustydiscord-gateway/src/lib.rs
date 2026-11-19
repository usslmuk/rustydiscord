#![cfg_attr(docsrs, feature(doc_cfg))]

mod compression;
mod config;
mod encoding;
mod error;
#[cfg(feature = "etf")]
mod etf;
mod metrics;
mod queue;
mod send_limit;
mod shard;
mod sharding;
mod tls;

#[cfg(feature = "etf")]
pub use self::etf::{EtfError, encode_value};
pub use self::{
    compression::Compression,
    config::{Config, ConfigBuilder, DEFAULT_GATEWAY_URL},
    encoding::Encoding,
    error::{ReceiveError, ReceiveErrorKind, SendError},
    queue::{IdentifyQueue, LocalIdentifyQueue},
    shard::{MessageSender, Shard, ShardState},
    sharding::{
        create_shards, decode_dispatch_borrowed, overlapping_shards, reshard_plan,
        spawn_shard_threads,
    },
};
pub use metrics::ShardMetrics;
