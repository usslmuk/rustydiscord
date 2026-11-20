#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod cdn;
mod client;
mod endpoints;
mod error;
mod metrics;
mod multipart;
pub mod oauth2;
mod query;
mod ratelimit;
mod route;
#[cfg(feature = "hyper")]
mod tls;
mod transport;

pub use self::{
    client::{
        AbortHandle, DEFAULT_API, RateLimitAction, RateLimited, Request, Rest, RestBuilder,
        RetryPolicy,
    },
    error::{ApiError, Error, ErrorKind, FieldError},
    route::{ReactionEmoji, Route},
    transport::{Body, BoxFuture, Transport, TransportKind, transport_kind},
};
pub use metrics::RestMetrics;
