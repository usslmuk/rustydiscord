use core::{
    future::Future,
    net::{Ipv4Addr, SocketAddr},
    ops::Range,
    pin::Pin,
    task::{Context as TaskContext, Poll},
    time::Duration,
};
use std::{
    collections::HashMap,
    ffi::OsString,
    io,
    path::PathBuf,
    process::{ExitStatus, Stdio},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicU64, Ordering},
    },
};

use futures_core::Stream;
use rustydiscord_gateway::{IdentifyQueue, LocalIdentifyQueue};
use rustydiscord_model::gateway::ShardId;
use rustydiscord_rest::Rest;
use serde_json::Value;
use tokio::{
    net::{TcpListener, TcpStream},
    process::Command,
    sync::{mpsc, oneshot, watch},
    task::JoinSet,
};

use super::{
    ClusterCommand, ClusterError, ClusterReply,
    cluster::{CLUSTER_ENV, ClusterAssignment, Gateway},
    command::{Failure, Replies, decode_one, decode_replies, encode},
    ipc::{
        MAX_HELLO, TOO_LARGE, ToCluster, ToManager, Token, read_frame, read_frame_within,
        too_large, write_frame,
    },
    lock,
};
use crate::Shards;

const HELLO_TIMEOUT: Duration = Duration::from_secs(10);

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(30);

const COMMAND_TIMEOUT: Duration = Duration::from_secs(30);

const RESTART_DELAY: (Duration, Duration) = (Duration::from_secs(1), Duration::from_secs(60));

type Exit = (u32, io::Result<ExitStatus>, Duration);

type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

#[derive(Clone, Debug)]
pub struct ClusterManager {
    inner: Arc<Inner>,
}

#[derive(Debug)]
struct Inner {
    rest: Rest,
    shards: Shards,
    shards_per_process: u32,
    executable: Option<PathBuf>,
    args: Option<Vec<OsString>>,
    envs: Vec<(OsString, OsString)>,
    shutdown_timeout: Duration,
    restart: Restart,
    restart_delay: (Duration, Duration),
    command_timeout: Duration,

    next_request: AtomicU64,
    running: AtomicBool,
    shutdown: watch::Sender<bool>,
    subscribers: Mutex<Vec<mpsc::UnboundedSender<ClusterEvent>>>,
    clusters: Mutex<Vec<State>>,
}

#[derive(Debug)]
struct State {
    shards: Range<u32>,
    pid: Option<u32>,

    sender: Option<mpsc::UnboundedSender<ToCluster>>,

    delay: Duration,

    pending: HashMap<u64, oneshot::Sender<Result<Value, Failure>>>,
}

impl ClusterManager {
    pub fn builder(token: impl Into<String>) -> ClusterManagerBuilder {
        ClusterManagerBuilder {
            token: token.into(),
            rest: None,
            shards: Shards::Auto,
            shards_per_process: 1,
            executable: None,
            args: None,
            envs: Vec::new(),
            shutdown_timeout: SHUTDOWN_TIMEOUT,
            restart: Restart::OnFailure,
            restart_delay: RESTART_DELAY,
            command_timeout: COMMAND_TIMEOUT,
        }
    }

    pub fn spawn(&self) -> impl Future<Output = Result<(), ClusterError>> + Send + '_ {
        boxed(self.run())
    }

    pub fn shutdown(&self) {
        self.inner.shutdown.send_replace(true);
    }

    #[must_use]
    pub fn events(&self) -> ClusterEvents {
        let (tx, rx) = mpsc::unbounded_channel();
        lock(&self.inner.subscribers).push(tx);
        ClusterEvents { rx }
    }

    #[must_use]
    pub fn clusters(&self) -> Vec<Cluster> {
        lock(&self.inner.clusters)
            .iter()
            .enumerate()
            .map(|(id, state)| Cluster {
                id: u32::try_from(id).unwrap_or(u32::MAX),
                shards: state.shards.clone(),
                pid: state.pid,
                connected: state.sender.is_some(),
            })
            .collect()
    }

    pub async fn broadcast<C: ClusterCommand>(
        &self,
        input: C::Input,
    ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError> {
        let input = encode(&input)?;
        Ok(decode_replies(self.ask(None, C::NAME, input).await))
    }

    pub async fn broadcast_eval<C: ClusterCommand>(
        &self,
        input: C::Input,
    ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError> {
        self.broadcast::<C>(input).await
    }

    pub async fn stats(&self) -> Result<Vec<ClusterReply<super::ClusterStats>>, ClusterError> {
        self.broadcast::<super::Stats>(()).await
    }

    pub async fn send<C: ClusterCommand>(
        &self,
        cluster: u32,
        input: C::Input,
    ) -> Result<C::Output, ClusterError> {
        let input = encode(&input)?;
        decode_one(self.ask(Some(cluster), C::NAME, input).await)
    }

    pub(crate) fn ask<'a>(
        &'a self,
        to: Option<u32>,
        command: &'a str,
        input: Value,
    ) -> BoxFuture<'a, Replies> {
        Box::pin(async move {
            let inner = &*self.inner;
            let mut waiting = Vec::new();
            {
                let mut clusters = lock(&inner.clusters);
                let targets = match to {
                    Some(cluster) => cluster..cluster.saturating_add(1),
                    None => 0..u32::try_from(clusters.len()).unwrap_or(u32::MAX),
                };
                for cluster in targets {
                    let state = clusters.get_mut(cluster as usize);
                    let Some((state, sender)) =
                        state.and_then(|state| state.sender.clone().map(|sender| (state, sender)))
                    else {
                        waiting.push((cluster, 0, None));
                        continue;
                    };
                    let id = inner.next_request.fetch_add(1, Ordering::Relaxed);
                    let (tx, rx) = oneshot::channel();
                    state.pending.insert(id, tx);
                    let request = ToCluster::Request {
                        id,
                        command: command.to_owned(),
                        input: input.clone(),
                    };
                    let sent = sender.send(request).is_ok();
                    waiting.push((cluster, id, sent.then_some(rx)));
                }
            }
            let deadline = tokio::time::Instant::now() + inner.command_timeout;
            let mut replies = Vec::with_capacity(waiting.len());
            for (cluster, id, reply) in waiting {
                let result = match reply {
                    None => Err(Failure::NotConnected),
                    Some(reply) => match tokio::time::timeout_at(deadline, reply).await {
                        Ok(Ok(result)) => result,
                        Ok(Err(_)) => Err(Failure::NotConnected),
                        Err(_) => {
                            if let Some(state) = lock(&inner.clusters).get_mut(cluster as usize) {
                                state.pending.remove(&id);
                            }
                            Err(Failure::Timeout)
                        }
                    },
                };
                replies.push((cluster, result));
            }
            replies
        })
    }

    fn too_large(&self, cluster: u32, message: ToCluster) -> Option<ToCluster> {
        let failure = || Failure::Encoding {
            message: TOO_LARGE.into(),
        };
        match message {
            ToCluster::Request { id, .. } => {
                tracing::warn!(cluster, "a command for a cluster is too large to send");
                let waiting = lock(&self.inner.clusters)
                    .get_mut(cluster as usize)
                    .and_then(|state| state.pending.remove(&id));
                if let Some(waiting) = waiting {
                    let _ = waiting.send(Err(failure()));
                }
                None
            }
            ToCluster::Replies { id, replies } => {
                tracing::warn!(cluster, "the results of a command are too large to send");
                let replies = replies
                    .into_iter()
                    .map(|(cluster, _)| (cluster, Err(failure())))
                    .collect();
                Some(ToCluster::Replies { id, replies })
            }
            _ => None,
        }
    }

    fn emit(&self, event: &ClusterEvent) {
        lock(&self.inner.subscribers).retain(|tx| tx.send(event.clone()).is_ok());
    }

    async fn run(&self) -> Result<(), ClusterError> {
        let inner = &*self.inner;
        if inner.running.swap(true, Ordering::AcqRel) {
            return Err(ClusterError::AlreadyRunning);
        }
        let result = self.supervise().await;
        inner.running.store(false, Ordering::Release);
        result
    }

    async fn supervise(&self) -> Result<(), ClusterError> {
        let inner = &*self.inner;
        inner.shutdown.send_replace(false);
        let shutdown = inner.shutdown.subscribe();

        let bot = inner.rest.get_gateway_bot().await?;
        let (range, total) = match &inner.shards {
            Shards::Count(count) => (0..*count, *count),
            Shards::Range { range, total } => (range.clone(), *total),
            Shards::Auto => {
                let total = bot.shards.max(1);
                (0..total, total)
            }
        };
        let limit = &bot.session_start_limit;
        let required = range.end - range.start;
        if limit.remaining < required {
            return Err(ClusterError::SessionStartLimit {
                required,
                remaining: limit.remaining,
                reset_after: Duration::from_millis(limit.reset_after),
            });
        }
        let gateway = Gateway {
            url: bot.url.into(),
            total,
            max_concurrency: limit.max_concurrency.max(1),
        };
        let groups = split(range, inner.shards_per_process);
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
            .await
            .map_err(ClusterError::Io)?;
        let session = Arc::new(Session {
            token: Token::generate(),
            identify: LocalIdentifyQueue::new(gateway.max_concurrency),
            gateway,
        });
        let launch = Launch {
            executable: match &inner.executable {
                Some(path) => path.clone(),
                None => std::env::current_exe().map_err(ClusterError::Io)?,
            },
            args: inner
                .args
                .clone()
                .unwrap_or_else(|| std::env::args_os().skip(1).collect()),
            address: listener.local_addr().map_err(ClusterError::Io)?,
        };
        *lock(&inner.clusters) = groups
            .iter()
            .map(|shards| State {
                shards: shards.clone(),
                pid: None,
                sender: None,
                delay: inner.restart_delay.0,
                pending: HashMap::new(),
            })
            .collect();
        tracing::info!(
            clusters = groups.len(),
            total,
            max_concurrency = session.gateway.max_concurrency,
            "starting the cluster processes"
        );

        let accepting = tokio::spawn(accept(self.clone(), listener, Arc::clone(&session)));
        let mut run = Run {
            manager: self,
            launch,
            session,
            kill: watch::channel(false).0,
            processes: JoinSet::new(),
            restarts: JoinSet::new(),
        };
        let mut started = Ok(());
        for id in (0_u32..).take(groups.len()) {
            if let Err(error) = run.start(id) {
                tracing::error!(cluster = id, %error, "could not start a cluster process");
                inner.shutdown.send_replace(true);
                started = Err(ClusterError::Io(error));
                break;
            }
        }
        let waited = run.wait(shutdown).await;
        accepting.abort();
        started.and(waited)
    }
}

struct Run<'a> {
    manager: &'a ClusterManager,
    launch: Launch,
    session: Arc<Session>,

    kill: watch::Sender<bool>,
    processes: JoinSet<Exit>,

    restarts: JoinSet<u32>,
}

impl Run<'_> {
    fn start(&mut self, id: u32) -> io::Result<()> {
        let inner = &*self.manager.inner;
        let Some(shards) = lock(&inner.clusters)
            .get(id as usize)
            .map(|state| state.shards.clone())
        else {
            return Ok(());
        };
        let assignment = ClusterAssignment::new(
            id,
            shards,
            &self.session.gateway,
            self.launch.address,
            self.session.token.as_str().into(),
        );
        let mut command = Command::new(&self.launch.executable);
        command
            .args(&self.launch.args)
            .envs(inner.envs.iter().map(|(key, value)| (key, value)))
            .env(CLUSTER_ENV, assignment.to_env())
            .stdin(Stdio::null())
            .kill_on_drop(true);
        let mut child = command.spawn()?;
        let pid = child.id();
        if let Some(state) = lock(&inner.clusters).get_mut(id as usize) {
            state.pid = pid;
        }
        self.manager
            .emit(&ClusterEvent::Spawned { cluster: id, pid });
        let mut kill = self.kill.subscribe();
        let started = tokio::time::Instant::now();
        self.processes.spawn(async move {
            let status = tokio::select! {
                status = child.wait() => status,
                () = async { let _ = kill.wait_for(|kill| *kill).await; } => {
                    let _ = child.start_kill();
                    child.wait().await
                }
            };
            (id, status, started.elapsed())
        });
        Ok(())
    }

    async fn wait(&mut self, mut shutdown: watch::Receiver<bool>) -> Result<(), ClusterError> {
        let inner = &*self.manager.inner;
        let mut deadline: Option<tokio::time::Instant> = None;
        loop {
            if self.processes.is_empty() && self.restarts.is_empty() {
                return Ok(());
            }
            let grace = async {
                match deadline {
                    Some(deadline) => tokio::time::sleep_until(deadline).await,
                    None => core::future::pending().await,
                }
            };
            tokio::select! {
                changed = shutdown.changed(), if deadline.is_none() => {
                    if changed.is_ok() && !*shutdown.borrow() {
                        continue;
                    }
                    tracing::info!("asking the clusters to shut down");

                    self.restarts = JoinSet::new();
                    for state in lock(&inner.clusters).iter() {
                        if let Some(sender) = &state.sender {
                            let _ = sender.send(ToCluster::Shutdown);
                        }
                    }
                    deadline = Some(tokio::time::Instant::now() + inner.shutdown_timeout);
                }
                () = grace => {
                    tracing::warn!("killing the clusters that did not shut down in time");
                    self.kill.send_replace(true);
                    deadline = None;
                }
                Some(finished) = self.processes.join_next() => {
                    let (id, status, ran) = finished.map_err(|error| {
                        ClusterError::Io(io::Error::other(error))
                    })?;
                    let failed = !matches!(&status, Ok(status) if status.success());
                    self.exited(id, status);
                    let restart = match inner.restart {
                        Restart::OnFailure => failed,
                        Restart::Always => true,
                        Restart::Never => false,
                    };
                    if restart && !*shutdown.borrow() {
                        self.restart_later(id, ran);
                    }
                }
                Some(restart) = self.restarts.join_next() => {
                    let Ok(id) = restart else { continue };
                    if let Err(error) = self.start(id) {
                        tracing::error!(cluster = id, %error, "could not restart a cluster process");
                        self.restart_later(id, Duration::ZERO);
                    }
                }
            }
        }
    }

    fn exited(&self, id: u32, status: io::Result<ExitStatus>) {
        if let Some(state) = lock(&self.manager.inner.clusters).get_mut(id as usize) {
            state.sender = None;
            state.pid = None;

            state.pending.clear();
        }
        match status {
            Ok(status) => {
                if status.success() {
                    tracing::info!(cluster = id, "cluster process exited");
                } else {
                    tracing::warn!(cluster = id, %status, "cluster process failed");
                }
                self.manager.emit(&ClusterEvent::Exited {
                    cluster: id,
                    status,
                });
            }
            Err(error) => {
                tracing::error!(cluster = id, %error, "lost a cluster process");
            }
        }
    }

    fn restart_later(&mut self, id: u32, ran: Duration) {
        let (first, max) = self.manager.inner.restart_delay;
        let delay = {
            let mut clusters = lock(&self.manager.inner.clusters);
            let Some(state) = clusters.get_mut(id as usize) else {
                return;
            };
            let (delay, next) = restart_delay(state.delay, ran, first, max);
            state.delay = next;
            delay
        };
        tracing::info!(cluster = id, ?delay, "restarting a cluster process");
        self.manager
            .emit(&ClusterEvent::Restarting { cluster: id, delay });
        self.restarts.spawn(async move {
            tokio::time::sleep(delay).await;
            id
        });
    }
}

fn restart_delay(
    current: Duration,
    ran: Duration,
    first: Duration,
    max: Duration,
) -> (Duration, Duration) {
    let delay = if ran >= max { first } else { current };
    (delay, delay.saturating_mul(2).min(max))
}

struct Launch {
    executable: PathBuf,
    args: Vec<OsString>,
    address: SocketAddr,
}

#[derive(Debug)]
struct Session {
    token: Token,
    gateway: Gateway,

    identify: LocalIdentifyQueue,
}

fn boxed<'a, T>(future: impl Future<Output = T> + Send + 'a) -> BoxFuture<'a, T> {
    Box::pin(future)
}

fn split(range: Range<u32>, size: u32) -> Vec<Range<u32>> {
    let size = size.max(1);
    let mut groups = Vec::new();
    let mut start = range.start;
    while start < range.end {
        let end = start.saturating_add(size).min(range.end);
        groups.push(start..end);
        start = end;
    }
    groups
}

async fn accept(manager: ClusterManager, listener: TcpListener, session: Arc<Session>) {
    loop {
        let Ok((stream, _)) = listener.accept().await else {
            tokio::time::sleep(Duration::from_millis(100)).await;
            continue;
        };
        tokio::spawn(handshake(manager.clone(), stream, Arc::clone(&session)));
    }
}

async fn handshake(manager: ClusterManager, stream: TcpStream, session: Arc<Session>) {
    let _ = stream.set_nodelay(true);
    let (mut reader, mut writer) = stream.into_split();
    let hello = tokio::time::timeout(
        HELLO_TIMEOUT,
        read_frame_within::<_, ToManager>(&mut reader, MAX_HELLO),
    )
    .await;
    let Ok(Ok(Some(ToManager::Hello {
        cluster,
        token: presented,
    }))) = hello
    else {
        tracing::warn!("refused a connection that did not introduce itself");
        return;
    };
    if !session.token.matches(&presented) {
        tracing::warn!(cluster, "refused a connection with a wrong token");
        return;
    }
    let (tx, mut rx) = mpsc::unbounded_channel();

    let _ = tx.send(ToCluster::Welcome);
    let shards = {
        let mut clusters = lock(&manager.inner.clusters);
        let Some(state) = clusters.get_mut(cluster as usize) else {
            tracing::warn!(cluster, "refused a connection for an unknown cluster");
            return;
        };
        if state.sender.is_some() {
            tracing::warn!(cluster, "refused a second connection for a cluster");
            return;
        }
        state.sender = Some(tx.clone());
        state.shards.clone()
    };
    manager.emit(&ClusterEvent::Connected { cluster });

    if *manager.inner.shutdown.borrow() {
        let _ = tx.send(ToCluster::Shutdown);
    }
    let writing = tokio::spawn({
        let manager = manager.clone();
        async move {
            while let Some(message) = rx.recv().await {
                match write_frame(&mut writer, &message).await {
                    Ok(()) => {}

                    Err(error) if too_large(&error) => {
                        let Some(instead) = manager.too_large(cluster, message) else {
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
    serve(&manager, cluster, &shards, reader, &tx, &session).await;
    writing.abort();
    let mut clusters = lock(&manager.inner.clusters);
    if let Some(state) = clusters.get_mut(cluster as usize)
        && state
            .sender
            .as_ref()
            .is_some_and(|sender| sender.same_channel(&tx))
    {
        state.sender = None;
        state.pending.clear();
    }
}

async fn serve(
    manager: &ClusterManager,
    cluster: u32,
    shards: &Range<u32>,
    mut reader: tokio::net::tcp::OwnedReadHalf,
    tx: &mpsc::UnboundedSender<ToCluster>,
    session: &Session,
) {
    let (frames_tx, mut frames) = mpsc::unbounded_channel();
    let reading = tokio::spawn(async move {
        while let Ok(Some(frame)) = read_frame::<_, ToManager>(&mut reader).await {
            if frames_tx.send(frame).is_err() {
                break;
            }
        }
    });
    let mut tasks = JoinSet::new();
    loop {
        tokio::select! {
            frame = frames.recv() => match frame {
                Some(ToManager::Identify { shard }) if shards.contains(&shard) => {
                    let turn = session
                        .identify
                        .wait_turn(ShardId::new(shard, session.gateway.total));
                    let tx = tx.clone();
                    tasks.spawn(async move {
                        turn.await;
                        let _ = tx.send(ToCluster::Identify { shard });
                    });
                }
                Some(ToManager::Request { id, to, command, input }) => {
                    let (manager, tx) = (manager.clone(), tx.clone());
                    tasks.spawn(async move {
                        let replies = manager.ask(to, &command, input).await;
                        let _ = tx.send(ToCluster::Replies { id, replies });
                    });
                }
                Some(ToManager::Reply { id, result }) => {
                    let waiting = lock(&manager.inner.clusters)
                        .get_mut(cluster as usize)
                        .and_then(|state| state.pending.remove(&id));
                    if let Some(waiting) = waiting {
                        let _ = waiting.send(result);
                    }
                }
                Some(ToManager::Identify { shard }) => {
                    tracing::warn!(cluster, shard, "ignored a turn asked for another cluster's shard");
                }
                Some(ToManager::Hello { .. }) => {}
                None => break,
            },
            Some(_) = tasks.join_next(), if !tasks.is_empty() => {}
        }
    }
    reading.abort();
}

#[derive(Debug)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct ClusterManagerBuilder {
    token: String,
    rest: Option<Rest>,
    shards: Shards,
    shards_per_process: u32,
    executable: Option<PathBuf>,
    args: Option<Vec<OsString>>,
    envs: Vec<(OsString, OsString)>,
    shutdown_timeout: Duration,
    restart: Restart,
    restart_delay: (Duration, Duration),
    command_timeout: Duration,
}

impl ClusterManagerBuilder {
    pub fn rest(mut self, rest: Rest) -> Self {
        self.rest = Some(rest);
        self
    }

    #[track_caller]
    pub fn shards(mut self, shards: Shards) -> Self {
        match &shards {
            Shards::Auto => {}
            Shards::Count(count) => assert!(*count > 0, "the shard count must not be zero"),
            Shards::Range { range, total } => assert!(
                range.start < range.end && range.end <= *total,
                "shard range {range:?} must be non-empty and within the total of {total}"
            ),
        }
        self.shards = shards;
        self
    }

    #[track_caller]
    pub fn shards_per_process(mut self, shards: u32) -> Self {
        assert!(shards > 0, "a process must run at least one shard");
        self.shards_per_process = shards;
        self
    }

    pub fn executable(mut self, path: impl Into<PathBuf>) -> Self {
        self.executable = Some(path.into());
        self
    }

    pub fn args<A: Into<OsString>>(mut self, args: impl IntoIterator<Item = A>) -> Self {
        self.args = Some(args.into_iter().map(Into::into).collect());
        self
    }

    pub fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
        self.envs.push((key.into(), value.into()));
        self
    }

    pub const fn shutdown_timeout(mut self, timeout: Duration) -> Self {
        self.shutdown_timeout = timeout;
        self
    }

    pub const fn restart(mut self, restart: Restart) -> Self {
        self.restart = restart;
        self
    }

    #[track_caller]
    pub const fn restart_delay(mut self, first: Duration, max: Duration) -> Self {
        assert!(!first.is_zero(), "the first restart delay must not be zero");
        assert!(
            first.as_nanos() <= max.as_nanos(),
            "the first restart delay must not be longer than the largest"
        );
        self.restart_delay = (first, max);
        self
    }

    pub const fn command_timeout(mut self, timeout: Duration) -> Self {
        self.command_timeout = timeout;
        self
    }

    #[must_use]
    pub fn build(self) -> ClusterManager {
        ClusterManager {
            inner: Arc::new(Inner {
                rest: self
                    .rest
                    .unwrap_or_else(|| Rest::builder().token(self.token).build()),
                shards: self.shards,
                shards_per_process: self.shards_per_process,
                executable: self.executable,
                args: self.args,
                envs: self.envs,
                shutdown_timeout: self.shutdown_timeout,
                restart: self.restart,
                restart_delay: self.restart_delay,
                command_timeout: self.command_timeout,
                next_request: AtomicU64::new(0),
                running: AtomicBool::new(false),
                shutdown: watch::channel(false).0,
                subscribers: Mutex::new(Vec::new()),
                clusters: Mutex::new(Vec::new()),
            }),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum Restart {
    #[default]
    OnFailure,

    Always,

    Never,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct Cluster {
    pub id: u32,

    pub shards: Range<u32>,

    pub pid: Option<u32>,

    pub connected: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClusterEvent {
    Spawned { cluster: u32, pid: Option<u32> },

    Connected { cluster: u32 },

    Exited { cluster: u32, status: ExitStatus },

    Restarting { cluster: u32, delay: Duration },
}

#[derive(Debug)]
pub struct ClusterEvents {
    rx: mpsc::UnboundedReceiver<ClusterEvent>,
}

impl ClusterEvents {
    pub async fn next(&mut self) -> Option<ClusterEvent> {
        self.rx.recv().await
    }
}

impl Stream for ClusterEvents {
    type Item = ClusterEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut TaskContext<'_>) -> Poll<Option<ClusterEvent>> {
        self.rx.poll_recv(cx)
    }
}

#[cfg(test)]
mod tests {
    use core::{ops::Range, time::Duration};
    use std::{net::SocketAddr, sync::Arc};

    use tokio::net::TcpListener;

    use rustydiscord_gateway::LocalIdentifyQueue;
    use rustydiscord_model::gateway::ShardId;

    use std::collections::HashMap;

    use serde_json::{Value, json};

    use super::{
        ClusterEvent, ClusterManager, Session, State, Token, accept, lock, restart_delay, split,
    };
    use crate::{
        Shards,
        context::Shared,
        sharding::{
            ClusterAssignment, ClusterError,
            cluster::Gateway,
            command::{BoxFuture, Commands, Failure},
            ipc::{ToCluster, ToManager, read_frame, write_frame},
        },
    };

    const STEP: Duration = Duration::from_secs(5);

    #[test]
    fn shards_are_split_into_processes() {
        assert_eq!(split(0..4, 1), vec![0..1, 1..2, 2..3, 3..4]);
        assert_eq!(split(0..10, 4), vec![0..4, 4..8, 8..10]);
        assert_eq!(split(8..16, 8), vec![8..16]);
        assert_eq!(split(0..3, 10), vec![0..3]);
    }

    struct Echo;

    impl Commands for Echo {
        fn call(&self, name: &str, input: Value) -> BoxFuture<'static, Result<Value, Failure>> {
            match name {
                "echo" => Box::pin(async move { Ok(input) }),
                "hang" => Box::pin(core::future::pending()),
                _ => Box::pin(async { Err(Failure::UnknownCommand) }),
            }
        }
    }

    fn commands() -> Arc<dyn Commands> {
        Arc::new(Echo)
    }

    fn gateway() -> Gateway {
        Gateway {
            url: "ws://127.0.0.1:9".into(),
            total: 2,
            max_concurrency: 1,
        }
    }

    #[test]
    fn restart_delays_grow_until_a_process_runs_long_enough() {
        let (first, max) = (Duration::from_secs(1), Duration::from_secs(8));
        let quick = Duration::from_millis(10);
        let mut current = first;
        let mut delays = Vec::new();
        for _ in 0..6 {
            let (delay, next) = restart_delay(current, quick, first, max);
            delays.push(delay.as_secs());
            current = next;
        }
        assert_eq!(delays, [1, 2, 4, 8, 8, 8]);

        assert_eq!(restart_delay(current, max, first, max), (first, first * 2));
    }

    async fn listening() -> (ClusterManager, SocketAddr, Token) {
        let manager = ClusterManager::builder("tok")
            .shards(Shards::Count(2))
            .command_timeout(Duration::from_millis(200))
            .build();
        *lock(&manager.inner.clusters) = [0..1, 1..2]
            .into_iter()
            .map(|shards: Range<u32>| State {
                shards,
                pid: None,
                sender: None,
                delay: Duration::from_secs(1),
                pending: HashMap::new(),
            })
            .collect();
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let token = Token::generate();
        let session = Arc::new(Session {
            token: token.clone(),
            gateway: gateway(),
            identify: LocalIdentifyQueue::new(1),
        });
        tokio::spawn(accept(manager.clone(), listener, session));
        (manager, address, token)
    }

    fn assignment(cluster: u32, address: SocketAddr, token: &str) -> ClusterAssignment {
        ClusterAssignment::new(
            cluster,
            cluster..cluster + 1,
            &gateway(),
            address,
            token.into(),
        )
    }

    #[tokio::test]
    async fn only_the_processes_the_manager_started_connect() {
        let (manager, address, token) = listening().await;
        let mut events = manager.events();
        let shared = Arc::new(Shared::default());

        let refused = assignment(0, address, "not-the-token")
            .connect(Arc::clone(&shared), commands())
            .await;
        assert!(matches!(refused, Err(ClusterError::Refused)), "{refused:?}");
        let unknown = assignment(7, address, token.as_str())
            .connect(Arc::clone(&shared), commands())
            .await;
        assert!(matches!(unknown, Err(ClusterError::Refused)), "{unknown:?}");
        assert!(manager.clusters().iter().all(|cluster| !cluster.connected));

        let connection = assignment(0, address, token.as_str())
            .connect(Arc::clone(&shared), commands())
            .await
            .unwrap();
        assert_eq!(
            events.next().await,
            Some(ClusterEvent::Connected { cluster: 0 })
        );
        let clusters = manager.clusters();
        assert!(clusters[0].connected && !clusters[1].connected);
        let second = assignment(0, address, token.as_str())
            .connect(Arc::new(Shared::default()), commands())
            .await;
        assert!(matches!(second, Err(ClusterError::Refused)), "{second:?}");
        assert!(!*shared.shutdown.borrow());

        let sender = lock(&manager.inner.clusters)[0].sender.clone().unwrap();
        sender.send(ToCluster::Shutdown).unwrap();
        let mut shutdown = shared.shutdown.subscribe();
        tokio::time::timeout(STEP, shutdown.wait_for(|stop| *stop))
            .await
            .unwrap()
            .unwrap();

        drop(connection);
        tokio::time::timeout(STEP, async {
            while manager.clusters()[0].connected {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .unwrap();
        let again = assignment(0, address, token.as_str())
            .connect(Arc::new(Shared::default()), commands())
            .await;
        assert!(again.is_ok(), "{again:?}");
    }

    #[tokio::test]
    async fn turns_are_granted_for_the_clusters_own_shards() {
        let (_manager, address, token) = listening().await;
        let connection = assignment(1, address, token.as_str())
            .connect(Arc::new(Shared::default()), commands())
            .await
            .unwrap();
        let queue = connection.identify_queue();

        let other = queue.wait_turn(ShardId::new(0, 2));
        assert!(
            tokio::time::timeout(Duration::from_millis(100), other)
                .await
                .is_err()
        );

        tokio::time::timeout(STEP, queue.wait_turn(ShardId::new(1, 2)))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn commands_reach_every_connected_cluster() {
        let (manager, address, token) = listening().await;
        let shared = Arc::new(Shared::default());
        let _connection = assignment(0, address, token.as_str())
            .connect(Arc::clone(&shared), commands())
            .await
            .unwrap();

        assert_eq!(
            manager.ask(None, "echo", json!({"n": 1})).await,
            [(0, Ok(json!({"n": 1}))), (1, Err(Failure::NotConnected))]
        );
        assert_eq!(
            manager.ask(Some(0), "nope", Value::Null).await,
            [(0, Err(Failure::UnknownCommand))]
        );
        assert_eq!(
            manager.ask(Some(9), "echo", Value::Null).await,
            [(9, Err(Failure::NotConnected))]
        );

        assert_eq!(
            manager.ask(Some(0), "hang", Value::Null).await,
            [(0, Err(Failure::Timeout))]
        );
        assert!(lock(&manager.inner.clusters)[0].pending.is_empty());

        let link = lock(&shared.link).clone().unwrap();
        assert_eq!(
            link.ask(None, "echo", json!(2)).await.unwrap(),
            [(0, Ok(json!(2))), (1, Err(Failure::NotConnected))]
        );
        assert_eq!(
            link.ask(Some(0), "echo", json!("one")).await.unwrap(),
            [(0, Ok(json!("one")))]
        );
    }

    #[tokio::test]
    async fn losing_the_manager_shuts_the_client_down() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let manager = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let hello = read_frame::<_, ToManager>(&mut stream).await.unwrap();
            assert_eq!(
                hello,
                Some(ToManager::Hello {
                    cluster: 1,
                    token: "t".into()
                })
            );
            write_frame(&mut stream, &ToCluster::Welcome).await.unwrap();
        });
        let shared = Arc::new(Shared::default());
        let _connection = assignment(1, address, "t")
            .connect(Arc::clone(&shared), commands())
            .await
            .unwrap();
        manager.await.unwrap();
        let mut shutdown = shared.shutdown.subscribe();
        tokio::time::timeout(STEP, shutdown.wait_for(|stop| *stop))
            .await
            .unwrap()
            .unwrap();
    }
}
