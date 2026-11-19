use core::{fmt, future::Future, ops::Range};
use std::time::Instant;

use rustydiscord_cache::{Backend, DashMapBackend};
use serde_json::Value;

use super::{
    ClusterAssignment, ClusterCommand, ClusterError, ClusterReply, ClusterStats, Stats,
    command::{BoxFuture, Commands, Failure, decode_one, decode_replies, encode, erase},
    stats::resident_memory,
};
use crate::{Client, HandlerError};

pub struct ClusterClient<B: Backend = DashMapBackend> {
    client: Client<B>,
}

impl<B: Backend> ClusterClient<B> {
    pub(crate) const fn new(client: Client<B>) -> Self {
        Self { client }
    }

    fn assignment(&self) -> &ClusterAssignment {
        self.client
            .assignment()
            .expect("a cluster client is only made for a cluster process")
    }

    #[must_use]
    pub fn id(&self) -> u32 {
        self.assignment().cluster_id()
    }

    #[must_use]
    pub fn shards(&self) -> Range<u32> {
        self.assignment().shards()
    }

    #[must_use]
    pub fn total_shards(&self) -> u32 {
        self.assignment().total_shards()
    }

    pub fn handle<C, F, Fut>(&self, handler: F) -> &Self
    where
        C: ClusterCommand,
        F: Fn(Client<B>, C::Input) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<C::Output, HandlerError>> + Send + 'static,
    {
        self.client
            .add_cluster_command(C::NAME, erase::<C, _, _, _>(handler));
        self
    }

    pub async fn broadcast<C: ClusterCommand>(
        &self,
        input: C::Input,
    ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError> {
        let link = self.client.link().ok_or(ClusterError::NotConnected)?;
        let input = encode(&input)?;
        Ok(decode_replies(link.ask(None, C::NAME, input).await?))
    }

    pub async fn broadcast_eval<C: ClusterCommand>(
        &self,
        input: C::Input,
    ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError> {
        self.broadcast::<C>(input).await
    }

    pub async fn send<C: ClusterCommand>(
        &self,
        cluster: u32,
        input: C::Input,
    ) -> Result<C::Output, ClusterError> {
        let link = self.client.link().ok_or(ClusterError::NotConnected)?;
        let input = encode(&input)?;
        decode_one(link.ask(Some(cluster), C::NAME, input).await?)
    }

    pub async fn stats(&self) -> Result<Vec<ClusterReply<ClusterStats>>, ClusterError> {
        self.broadcast::<Stats>(()).await
    }
}

impl<B: Backend> Clone for ClusterClient<B> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
        }
    }
}

impl<B: Backend> fmt::Debug for ClusterClient<B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClusterClient")
            .field("assignment", self.assignment())
            .finish_non_exhaustive()
    }
}

pub(crate) struct Table<B: Backend> {
    pub(crate) client: Client<B>,

    pub(crate) started: Instant,
}

impl<B: Backend> Table<B> {
    fn stats(&self) -> ClusterStats {
        let client = &self.client;
        let cache = client.cache();
        ClusterStats {
            cluster: client.assignment().map_or(0, ClusterAssignment::cluster_id),
            shards: client.shard_stats(),
            guilds: cache.map(|cache| cache.guilds().len() as u64),
            users: cache.map(|cache| cache.users().len() as u64),
            uptime: self.started.elapsed(),
            memory: resident_memory(),
        }
    }
}

impl<B: Backend> Commands for Table<B> {
    fn call(&self, name: &str, input: Value) -> BoxFuture<'static, Result<Value, Failure>> {
        if name == Stats::NAME {
            let stats = serde_json::to_value(self.stats()).map_err(|error| Failure::Encoding {
                message: error.to_string(),
            });
            return Box::pin(async move { stats });
        }
        match self.client.cluster_command(name) {
            Some(handler) => handler(self.client.clone(), input),
            None => Box::pin(async { Err(Failure::UnknownCommand) }),
        }
    }
}
