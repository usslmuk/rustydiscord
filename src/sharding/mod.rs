mod cluster;
mod cluster_client;
mod command;
mod ipc;
mod manager;
pub mod remote;
mod stats;

use std::sync::{Mutex, MutexGuard, PoisonError};

pub use self::remote::{
    CrossHostEndpoint, EvalError, EvalLimits, EvalRequest, RedisIdentifyQueue, ReshardState,
    TlsConfig,
};
pub use self::{
    cluster::ClusterAssignment,
    cluster_client::ClusterClient,
    command::{ClusterCommand, ClusterReply},
    manager::{
        Cluster, ClusterEvent, ClusterEvents, ClusterManager, ClusterManagerBuilder, Restart,
    },
    stats::{ClusterStats, ShardStats, ShardStatus, Stats},
};
pub(crate) use self::{
    cluster::{Connection, Link},
    cluster_client::Table,
    command::Handler,
    stats::{Reporter, ShardSlot},
};

pub fn process() -> Result<Option<ClusterAssignment>, ClusterError> {
    ClusterAssignment::from_env()
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ClusterError {
    #[error("could not fetch the gateway's details")]
    Rest(#[from] crate::rest::Error),

    #[error(
        "not enough session starts left to start {required} shards ({remaining} remaining, \
         reset in {reset_after:?})"
    )]
    SessionStartLimit {
        required: u32,

        remaining: u32,

        reset_after: core::time::Duration,
    },

    #[error("a cluster process or connection failed")]
    Io(#[source] std::io::Error),

    #[error("the cluster assignment in the environment is not valid")]
    Assignment,

    #[error("the cluster manager refused this process")]
    Refused,

    #[error("the cluster manager is already running")]
    AlreadyRunning,

    #[error("the cluster is not connected")]
    NotConnected,

    #[error("the cluster does not handle this command")]
    UnknownCommand,

    #[error("the command failed: {0}")]
    CommandFailed(String),

    #[error("the cluster did not answer in time")]
    Timeout,

    #[error("could not encode or decode a command: {0}")]
    Encoding(String),
}
