use rustydiscord_model::gateway::CloseCode;

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, thiserror::Error)]
#[error("{kind}")]
pub struct ReceiveError {
    kind: ReceiveErrorKind,
    #[source]
    source: Option<BoxError>,
}

impl ReceiveError {
    pub(crate) fn new(kind: ReceiveErrorKind) -> Self {
        Self { kind, source: None }
    }

    pub(crate) fn with_source(kind: ReceiveErrorKind, source: impl Into<BoxError>) -> Self {
        Self {
            kind,
            source: Some(source.into()),
        }
    }

    #[must_use]
    pub const fn kind(&self) -> &ReceiveErrorKind {
        &self.kind
    }

    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        matches!(self.kind, ReceiveErrorKind::FatallyClosed { .. })
    }

    #[must_use]
    pub fn into_parts(
        self,
    ) -> (
        ReceiveErrorKind,
        Option<Box<dyn std::error::Error + Send + Sync>>,
    ) {
        (self.kind, self.source)
    }
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum ReceiveErrorKind {
    #[error("failed to connect to the gateway")]
    Connect,

    #[error("the gateway connection failed")]
    Connection,

    #[error("failed to decompress a gateway message")]
    Decompress,

    #[error("failed to deserialize a gateway payload")]
    Deserialize,

    #[error("the gateway closed the connection with {code:?}; reconnecting is not allowed")]
    FatallyClosed { code: CloseCode },
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum SendError {
    #[error("failed to serialize the gateway command")]
    Serialize(#[source] serde_json::Error),

    #[cfg(feature = "etf")]
    #[error("failed to encode the gateway command as ETF")]
    Encode(#[source] crate::etf::EtfError),

    #[error("the shard is gone")]
    Closed,
}
