use core::{fmt, future::Future, ops::Range, pin::Pin, time::Duration};
use std::{
    collections::{HashMap, VecDeque},
    net::SocketAddr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};

use crate::gateway::IdentifyQueue;
use crate::model::gateway::ShardId;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::{
    net::TcpStream,
    sync::{mpsc, oneshot},
    task::JoinHandle,
};

use super::{
    ClusterError,
    command::{Commands, Failure, Replies},
    ipc::{TOO_LARGE, ToCluster, ToManager, read_frame, too_large, write_frame},
    lock,
};
use crate::context::Shared;

pub(crate) const CLUSTER_ENV: &str = "DISCORD_RS_CLUSTER";

const HELLO_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterAssignment {
    cluster: u32,
    shards: Range<u32>,
    total: u32,
    gateway_url: String,
    max_concurrency: u32,
    address: SocketAddr,
    token: String,
}

impl ClusterAssignment {
    pub(crate) fn new(
        cluster: u32,
        shards: Range<u32>,
        gateway: &Gateway,
        address: SocketAddr,
        token: String,
    ) -> Self {
        Self {
            cluster,
            shards,
            total: gateway.total,
            gateway_url: gateway.url.clone(),
            max_concurrency: gateway.max_concurrency,
            address,
            token,
        }
    }

    #[must_use]
    pub const fn cluster_id(&self) -> u32 {
        self.cluster
    }

    #[must_use]
    pub fn shards(&self) -> Range<u32> {
        self.shards.clone()
    }

    #[must_use]
    pub const fn total_shards(&self) -> u32 {
        self.total
    }

    pub(crate) fn gateway_url(&self) -> &str {
        &self.gateway_url
    }

    pub(crate) const fn max_concurrency(&self) -> u32 {
        self.max_concurrency
    }

    pub(crate) fn from_env() -> Result<Option<Self>, ClusterError> {
        match std::env::var_os(CLUSTER_ENV) {
            Some(value) => value
                .into_string()
                .map_err(|_| ClusterError::Assignment)
                .and_then(|value| Self::parse(&value))
                .map(Some),
            None => Ok(None),
        }
    }

    fn parse(value: &str) -> Result<Self, ClusterError> {
        let assignment: Self = serde_json::from_str(value).map_err(|_| ClusterError::Assignment)?;
        let valid = assignment.shards.start < assignment.shards.end
            && assignment.shards.end <= assignment.total
            && assignment.max_concurrency > 0
            && assignment.address.ip().is_loopback();
        if valid {
            Ok(assignment)
        } else {
            Err(ClusterError::Assignment)
        }
    }

    pub(crate) fn to_env(&self) -> String {
        serde_json::to_string(self).expect("an assignment always serializes")
    }

    pub(crate) async fn connect(
        &self,
        shared: Arc<Shared>,
        commands: Arc<dyn Commands>,
    ) -> Result<Connection, ClusterError> {
        let stream = TcpStream::connect(self.address)
            .await
            .map_err(ClusterError::Io)?;
        stream.set_nodelay(true).map_err(ClusterError::Io)?;
        let (mut reader, mut writer) = stream.into_split();
        let hello = ToManager::Hello {
            cluster: self.cluster,
            token: self.token.clone(),
        };
        write_frame(&mut writer, &hello)
            .await
            .map_err(ClusterError::Io)?;
        match tokio::time::timeout(HELLO_TIMEOUT, read_frame(&mut reader)).await {
            Ok(Ok(Some(ToCluster::Welcome))) => {}
            Ok(Err(error)) => return Err(ClusterError::Io(error)),
            Ok(Ok(_)) | Err(_) => return Err(ClusterError::Refused),
        }

        let (requests, mut outgoing) = mpsc::unbounded_channel();
        let queue = Arc::new(ManagerQueue {
            requests: requests.clone(),
            waiting: Mutex::default(),
        });
        let link = Arc::new(Link {
            requests,
            next: AtomicU64::new(0),
            pending: Mutex::default(),
        });
        *lock(&shared.link) = Some(Arc::clone(&link));

        let writing = tokio::spawn({
            let link = Arc::clone(&link);
            async move {
                while let Some(message) = outgoing.recv().await {
                    match write_frame(&mut writer, &message).await {
                        Ok(()) => {}

                        Err(error) if too_large(&error) => {
                            let Some(instead) = link.too_large(&message) else {
                                continue;
                            };
                            if write_frame(&mut writer, &instead).await.is_err() {
                                break;
                            }
                        }
                        Err(_) => break,
                    }
                }
            }
        });
        let cluster = self.cluster;
        let granted = Arc::clone(&queue);
        let (connected, reader_link) = (Arc::clone(&shared), Arc::clone(&link));
        let reading = tokio::spawn(async move {
            let (shared, link) = (connected, reader_link);
            loop {
                match read_frame::<_, ToCluster>(&mut reader).await {
                    Ok(Some(ToCluster::Identify { shard })) => granted.grant(shard),
                    Ok(Some(ToCluster::Request { id, command, input })) => {
                        link.run(id, commands.call(&command, input));
                    }
                    Ok(Some(ToCluster::Replies { id, replies })) => link.replied(id, replies),
                    Ok(Some(ToCluster::Shutdown)) => {
                        tracing::info!(cluster, "the cluster manager asked to shut down");
                        break;
                    }
                    Ok(Some(ToCluster::Welcome)) => {}
                    Ok(None) | Err(_) => {
                        tracing::warn!(cluster, "lost the connection to the cluster manager");
                        break;
                    }
                }
            }
            disconnect(&shared, &link);
            shared.shutdown.send_replace(true);
        });
        Ok(Connection {
            queue,
            shared,
            link,
            reading,
            writing,
        })
    }
}

fn disconnect(shared: &Shared, link: &Arc<Link>) {
    let mut current = lock(&shared.link);
    if current
        .as_ref()
        .is_some_and(|current| Arc::ptr_eq(current, link))
    {
        *current = None;
    }
    drop(current);
    lock(&link.pending).clear();
}

impl fmt::Debug for ClusterAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClusterAssignment")
            .field("cluster", &self.cluster)
            .field("shards", &self.shards)
            .field("total", &self.total)
            .field("gateway_url", &self.gateway_url)
            .field("max_concurrency", &self.max_concurrency)
            .field("address", &self.address)
            .finish_non_exhaustive()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Gateway {
    pub(crate) url: String,
    pub(crate) total: u32,
    pub(crate) max_concurrency: u32,
}

#[derive(Debug)]
pub(crate) struct Connection {
    queue: Arc<ManagerQueue>,
    shared: Arc<Shared>,
    link: Arc<Link>,
    reading: JoinHandle<()>,
    writing: JoinHandle<()>,
}

impl Connection {
    pub(crate) fn identify_queue(&self) -> Arc<dyn IdentifyQueue> {
        Arc::clone(&self.queue) as Arc<dyn IdentifyQueue>
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        self.reading.abort();
        self.writing.abort();

        disconnect(&self.shared, &self.link);
    }
}

#[derive(Debug)]
pub(crate) struct Link {
    requests: mpsc::UnboundedSender<ToManager>,
    next: AtomicU64,

    pending: Mutex<HashMap<u64, oneshot::Sender<Result<Replies, ClusterError>>>>,
}

impl Link {
    pub(crate) fn ask<'a>(
        &'a self,
        to: Option<u32>,
        command: &'a str,
        input: Value,
    ) -> super::command::BoxFuture<'a, Result<Replies, ClusterError>> {
        Box::pin(self.request(to, command, input))
    }

    async fn request(
        &self,
        to: Option<u32>,
        command: &str,
        input: Value,
    ) -> Result<Replies, ClusterError> {
        let id = self.next.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = oneshot::channel();
        lock(&self.pending).insert(id, tx);
        let request = ToManager::Request {
            id,
            to,
            command: command.to_owned(),
            input,
        };
        if self.requests.send(request).is_err() {
            lock(&self.pending).remove(&id);
            return Err(ClusterError::NotConnected);
        }

        rx.await.map_err(|_| ClusterError::NotConnected)?
    }

    fn replied(&self, id: u64, replies: Replies) {
        if let Some(waiting) = lock(&self.pending).remove(&id) {
            let _ = waiting.send(Ok(replies));
        }
    }

    fn too_large(&self, message: &ToManager) -> Option<ToManager> {
        match *message {
            ToManager::Request { id, .. } => {
                tracing::warn!("a command is too large to send to the cluster manager");
                if let Some(waiting) = lock(&self.pending).remove(&id) {
                    let _ = waiting.send(Err(ClusterError::Encoding(TOO_LARGE.into())));
                }
                None
            }
            ToManager::Reply { id, .. } => {
                tracing::warn!("the result of a command is too large to send");
                Some(ToManager::Reply {
                    id,
                    result: Err(Failure::Encoding {
                        message: TOO_LARGE.into(),
                    }),
                })
            }
            _ => None,
        }
    }

    fn run(&self, id: u64, handler: super::command::BoxFuture<'static, Result<Value, Failure>>) {
        let requests = self.requests.clone();
        tokio::spawn(async move {
            let result = tokio::spawn(handler).await.unwrap_or_else(|_| {
                Err(Failure::Failed {
                    message: "the command handler panicked".into(),
                })
            });
            let _ = requests.send(ToManager::Reply { id, result });
        });
    }
}

#[derive(Debug)]
pub(crate) struct ManagerQueue {
    requests: mpsc::UnboundedSender<ToManager>,

    waiting: Mutex<HashMap<u32, VecDeque<oneshot::Sender<()>>>>,
}

impl ManagerQueue {
    fn grant(&self, shard: u32) {
        let mut waiting = lock(&self.waiting);
        let Some(waiters) = waiting.get_mut(&shard) else {
            return;
        };
        while let Some(waiter) = waiters.pop_front() {
            if waiter.send(()).is_ok() {
                break;
            }
        }
        if waiters.is_empty() {
            waiting.remove(&shard);
        }
    }
}

impl IdentifyQueue for ManagerQueue {
    fn wait_turn(&self, shard: ShardId) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let (tx, rx) = oneshot::channel();
        lock(&self.waiting)
            .entry(shard.number())
            .or_default()
            .push_back(tx);
        let _ = self.requests.send(ToManager::Identify {
            shard: shard.number(),
        });
        Box::pin(async move {
            if rx.await.is_err() {
                core::future::pending::<()>().await;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;
    use std::sync::Mutex;

    use crate::gateway::IdentifyQueue;
    use crate::model::gateway::ShardId;
    use tokio::{sync::mpsc, time::timeout};

    use super::{ClusterAssignment, Gateway, ManagerQueue};
    use crate::sharding::{ClusterError, ipc::ToManager, lock};

    #[tokio::test]
    async fn turns_go_to_the_shards_still_waiting() {
        let (requests, mut sent) = mpsc::unbounded_channel();
        let queue = ManagerQueue {
            requests,
            waiting: Mutex::default(),
        };
        let shard = ShardId::new(3, 4);

        drop(queue.wait_turn(shard));
        let mut waiting = queue.wait_turn(shard);
        for _ in 0..2 {
            assert_eq!(sent.recv().await, Some(ToManager::Identify { shard: 3 }));
        }

        assert!(
            timeout(Duration::from_millis(20), &mut waiting)
                .await
                .is_err()
        );

        queue.grant(3);
        timeout(Duration::from_secs(5), waiting).await.unwrap();

        queue.grant(3);
        queue.grant(9);
        assert!(lock(&queue.waiting).is_empty());
    }

    #[test]
    fn assignments_round_trip_through_the_environment() {
        let gateway = Gateway {
            url: "wss://gateway.example".into(),
            total: 8,
            max_concurrency: 16,
        };
        let assignment = ClusterAssignment::new(
            2,
            4..6,
            &gateway,
            "127.0.0.1:4000".parse().unwrap(),
            "t".into(),
        );
        let value = assignment.to_env();
        assert_eq!(
            value,
            r#"{"cluster":2,"shards":{"start":4,"end":6},"total":8,"gateway_url":"wss://gateway.example","max_concurrency":16,"address":"127.0.0.1:4000","token":"t"}"#
        );
        assert_eq!(assignment.gateway_url(), "wss://gateway.example");
        assert_eq!(assignment.max_concurrency(), 16);
        assert_eq!(ClusterAssignment::parse(&value).unwrap(), assignment);
        assert_eq!(assignment.cluster_id(), 2);
        assert_eq!(assignment.shards(), 4..6);
        assert_eq!(assignment.total_shards(), 8);

        assert!(!format!("{assignment:?}").contains("token"));
    }

    #[test]
    fn invalid_assignments_are_rejected() {
        let with = |shards: &str, total: u32, max_concurrency: u32, address: &str| {
            ClusterAssignment::parse(&format!(
                r#"{{"cluster":0,"shards":{shards},"total":{total},"gateway_url":"wss://g","max_concurrency":{max_concurrency},"address":"{address}","token":"t"}}"#
            ))
        };
        assert!(with(r#"{"start":0,"end":1}"#, 1, 1, "127.0.0.1:1").is_ok());
        assert!(with(r#"{"start":0,"end":1}"#, 1, 1, "[::1]:1").is_ok());
        for invalid in [
            with(r#"{"start":1,"end":1}"#, 2, 1, "127.0.0.1:1"),
            with(r#"{"start":0,"end":3}"#, 2, 1, "127.0.0.1:1"),
            with(r#"{"start":0,"end":1}"#, 1, 0, "127.0.0.1:1"),
            with(r#"{"start":0,"end":1}"#, 1, 1, "10.0.0.1:1"),
            ClusterAssignment::parse("not json"),
        ] {
            assert!(
                matches!(invalid, Err(ClusterError::Assignment)),
                "{invalid:?}"
            );
        }
    }
}
