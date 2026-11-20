#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum Error {
    #[error("a REST request failed")]
    Rest(#[from] crate::rest::Error),

    #[error("the gateway connection failed")]
    #[cfg(feature = "gateway")]
    #[cfg_attr(docsrs, doc(cfg(feature = "gateway")))]
    Gateway(#[from] crate::gateway::ReceiveError),

    #[error(
        "not enough session starts left to start {required} shards ({remaining} remaining, \
         reset in {reset_after:?})"
    )]
    SessionStartLimit {
        required: u32,

        remaining: u32,

        reset_after: core::time::Duration,
    },

    #[error("the client is already logged in")]
    AlreadyRunning,

    #[cfg(feature = "sharding")]
    #[cfg_attr(docsrs, doc(cfg(feature = "sharding")))]
    #[error("the connection to the cluster manager failed")]
    Cluster(#[from] crate::sharding::ClusterError),
}
