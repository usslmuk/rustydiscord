use core::{
    fmt,
    pin::Pin,
    sync::atomic::{AtomicU64, Ordering},
    task::{Context, Poll},
    time::Duration,
};
use std::{
    collections::{HashMap, VecDeque},
    error::Error,
    sync::{Arc, Mutex, MutexGuard, PoisonError, Weak},
};

use crate::model::{
    gateway::payload::UpdateVoiceState,
    id::{ChannelId, GuildId, UserId},
};
use futures_util::Stream;
use tokio::{
    sync::{
        mpsc::{self, error::TryRecvError},
        oneshot, watch,
    },
    task::JoinHandle,
    time::{Instant, sleep_until},
};

use crate::voice::{
    Input, InputError, JoinError, TrackError, TrackHandle, TrackQueue, VoiceGateway,
    codec::{self, Decoder, Encoder, MAX_PACKET, MAX_PACKET_VALUES},
    connection::{
        self, Command, Config, ConnectError, ConnectionEvent, End, Handle, Info, Receiver, Sender,
    },
    input::{self as reading, Again, Chunk, Frame, History, Source},
    payload::{Event, MICROPHONE},
    queue::{Queue, Shared, Slot},
    receive::{self, ReceiveOptions, Receiving, VoiceStream},
    track::{Track, TrackEnd, TrackEvent, duration},
    udp::Rtp,
};

const DISCONNECTED: u16 = 4014;

const MOVE_GRACE: Duration = Duration::from_secs(5);

const SILENCE_FRAMES: u8 = 5;

const SILENCE: [u8; 3] = [0xF8, 0xFF, 0xFE];

const LATE: Duration = Duration::from_millis(100);

const AHEAD: usize = 50;

const SETTLE: u8 = 2;

type Read = Option<Result<Chunk, InputError>>;

struct Reencode {
    decoder: Decoder,
    fifo: VecDeque<f32>,

    steady: u8,
}

pub(crate) type Calls = Mutex<HashMap<GuildId, Call>>;

pub(crate) fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

#[derive(Clone)]
pub struct Call {
    pub(crate) inner: Arc<Inner>,
}

pub(crate) struct Inner {
    pub(crate) id: u64,
    guild_id: GuildId,
    shard_id: u32,
    gateway: Arc<dyn VoiceGateway>,

    shared: Arc<Shared>,
    subscribers: Arc<Mutex<Subscribers>>,
    channel: watch::Receiver<Option<ChannelId>>,
    tracks: AtomicU64,
}

#[derive(Default)]
struct Subscribers {
    senders: Vec<mpsc::UnboundedSender<CallEvent>>,

    closed: bool,
}

impl fmt::Debug for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Call")
            .field("guild_id", &self.inner.guild_id)
            .field("channel_id", &self.channel_id())
            .finish_non_exhaustive()
    }
}

pub(crate) enum CallCommand {
    Join {
        channel_id: ChannelId,

        bitrate: u32,
        done: oneshot::Sender<Result<(), JoinError>>,
    },

    Abandoned,

    State {
        session_id: String,
        channel_id: Option<ChannelId>,
    },

    Server {
        token: String,
        endpoint: Option<String>,
    },

    Wake,

    Seek {
        id: u64,
        samples: u64,
        done: oneshot::Sender<Result<Duration, TrackError>>,
    },
    Leave {
        done: oneshot::Sender<()>,
    },

    Subscribe {
        user_id: UserId,
        options: ReceiveOptions,
        tx: mpsc::Sender<receive::Item>,
    },
}

pub(crate) struct Spawn {
    pub(crate) id: u64,
    pub(crate) guild_id: GuildId,
    pub(crate) user_id: UserId,
    pub(crate) shard_id: u32,
    pub(crate) gateway: Arc<dyn VoiceGateway>,
    pub(crate) config: Config,
    pub(crate) calls: Weak<Calls>,
}

impl Call {
    pub(crate) fn spawn(spawn: Spawn) -> Self {
        let (commands, commands_rx) = mpsc::unbounded_channel();
        let (channel, channel_rx) = watch::channel(None);
        let subscribers = Arc::new(Mutex::new(Subscribers::default()));
        let shared = Arc::new(Shared {
            commands,
            queue: Mutex::new(Queue::default()),
        });
        let task = Task {
            call_id: spawn.id,
            guild_id: spawn.guild_id,
            user_id: spawn.user_id,
            config: spawn.config,
            calls: spawn.calls,
            subscribers: Arc::clone(&subscribers),
            channel,
            commands: commands_rx,
            voice: VoiceState::default(),
            fresh_server: false,
            live: None,
            connecting: None,
            join: None,
            grace: None,
            removed: false,
            connected_once: false,
            leave: None,
            player: Player::new(Arc::clone(&shared)),
            receiving: Receiving::default(),
        };
        tokio::spawn(task.run());
        Self {
            inner: Arc::new(Inner {
                id: spawn.id,
                guild_id: spawn.guild_id,
                shard_id: spawn.shard_id,
                gateway: spawn.gateway,
                shared,
                subscribers,
                channel: channel_rx,
                tracks: AtomicU64::new(0),
            }),
        }
    }

    pub(crate) fn command(&self, command: CallCommand) -> bool {
        self.inner.shared.commands.send(command).is_ok()
    }

    #[must_use]
    pub fn guild_id(&self) -> GuildId {
        self.inner.guild_id
    }

    #[must_use]
    pub fn channel_id(&self) -> Option<ChannelId> {
        *self.inner.channel.borrow()
    }

    #[allow(
        clippy::must_use_candidate,
        reason = "playing without keeping the handle is fine"
    )]
    pub fn play(&self, input: Input) -> TrackHandle {
        self.add(input, true)
    }

    #[allow(
        clippy::must_use_candidate,
        reason = "queueing without keeping the handle is fine"
    )]
    pub fn enqueue(&self, input: Input) -> TrackHandle {
        self.add(input, false)
    }

    fn add(&self, input: Input, now: bool) -> TrackHandle {
        let id = self.inner.tracks.fetch_add(1, Ordering::Relaxed);
        let track = Track::new(id, input.seekable(), Arc::downgrade(&self.inner.shared));
        let handle = TrackHandle::new(Arc::clone(&track));
        let slot = Slot {
            track,
            input: Some(input),
        };
        let mut queue = lock(&self.inner.shared.queue);
        if queue.closed {
            slot.track.end(TrackEnd::CallEnded);
            return handle;
        }
        if now {
            if let Some(old) = queue.current.replace(slot) {
                old.track.end(TrackEnd::Replaced);
            }
        } else if queue.current.is_none() {
            queue.current = Some(slot);
        } else {
            queue.waiting.push_back(slot);
        }
        drop(queue);
        self.inner.shared.wake();
        handle
    }

    #[must_use]
    pub fn queue(&self) -> TrackQueue {
        TrackQueue {
            shared: Arc::clone(&self.inner.shared),
        }
    }

    #[must_use]
    pub fn subscribe(&self, user_id: UserId) -> VoiceStream {
        self.subscribe_with(user_id, ReceiveOptions::new())
    }

    #[must_use]
    pub fn subscribe_with(&self, user_id: UserId, options: ReceiveOptions) -> VoiceStream {
        let (tx, rx) = mpsc::channel(receive::QUEUE);

        self.command(CallCommand::Subscribe {
            user_id,
            options,
            tx,
        });
        VoiceStream::new(rx, options)
    }

    #[must_use]
    pub fn events(&self) -> CallEvents {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut subscribers = lock(&self.inner.subscribers);
        if !subscribers.closed {
            subscribers.senders.push(tx);
        }
        CallEvents { rx }
    }

    pub async fn wait_for(
        &self,
        timeout: Duration,
        mut matches: impl FnMut(&CallEvent) -> bool,
    ) -> Result<CallEvent, WaitError> {
        let mut events = self.events();
        let found = tokio::time::timeout(timeout, async {
            while let Some(event) = events.next().await {
                if matches(&event) {
                    return Some(event);
                }
            }
            None
        })
        .await;
        match found {
            Ok(Some(event)) => Ok(event),
            Ok(None) => Err(WaitError::Ended),
            Err(_) => Err(WaitError::Timeout),
        }
    }

    pub async fn leave(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let command = UpdateVoiceState::new(self.inner.guild_id, None, false, false);
        let sent = self.inner.gateway.send(self.inner.shard_id, &command);
        let (done, ended) = oneshot::channel();
        if self.command(CallCommand::Leave { done }) {
            let _ = ended.await;
        }
        sent
    }
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CallEvent {
    Resumed,

    Reconnected,

    Disconnected(DisconnectReason),

    UserJoined(UserId),

    UserLeft(UserId),

    #[non_exhaustive]
    Speaking {
        user_id: UserId,

        ssrc: u32,
    },

    SpeakingStarted(UserId),

    SpeakingStopped(UserId),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum DisconnectReason {
    Left,

    #[non_exhaustive]
    Closed {
        code: u16,
    },

    Lost(Arc<dyn Error + Send + Sync>),
}

impl fmt::Display for DisconnectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => f.write_str("the bot left the call"),
            Self::Closed { code } => {
                write!(f, "the voice gateway closed the call with code {code}")
            }
            Self::Lost(_) => f.write_str("the voice connection was lost"),
        }
    }
}

impl Error for DisconnectReason {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Lost(error) => Some(&**error),
            Self::Left | Self::Closed { .. } => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WaitError {
    Timeout,
    Ended,
}

impl fmt::Display for WaitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Timeout => f.write_str("the awaited call event did not arrive in time"),
            Self::Ended => f.write_str("the call ended before the awaited event arrived"),
        }
    }
}

impl Error for WaitError {}

#[derive(Debug)]
pub struct CallEvents {
    rx: mpsc::UnboundedReceiver<CallEvent>,
}

impl CallEvents {
    pub async fn next(&mut self) -> Option<CallEvent> {
        self.rx.recv().await
    }
}

impl Stream for CallEvents {
    type Item = CallEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<CallEvent>> {
        self.rx.poll_recv(cx)
    }
}

#[derive(Default)]
struct VoiceState {
    session_id: Option<String>,
    channel_id: Option<ChannelId>,

    server: Option<(String, String)>,
}

struct Live {
    channel_id: ChannelId,
    handle: Handle,
    sender: Sender,
    receiver: Receiver,
    events: mpsc::UnboundedReceiver<ConnectionEvent>,
}

type Connected = (Handle, Sender, mpsc::UnboundedReceiver<ConnectionEvent>);

struct Connecting {
    channel_id: ChannelId,
    task: JoinHandle<Result<Connected, ConnectError>>,
}

struct PendingJoin {
    channel_id: ChannelId,
    waiters: Vec<oneshot::Sender<Result<(), JoinError>>>,
    deadline: Instant,

    state: bool,

    server: bool,
}

enum Step {
    Command(Option<CallCommand>),
    Connection(Option<ConnectionEvent>),
    Connected(ChannelId, Result<Connected, ConnectError>),
    JoinTimeout,
    GraceOver,
    Chunk(Option<Result<Chunk, InputError>>),
    Frame,

    Packet(Rtp),

    Receive,
}

struct Ending {
    reason: Option<DisconnectReason>,
    join: JoinError,
}

impl Ending {
    const fn new(reason: Option<DisconnectReason>, join: JoinError) -> Self {
        Self { reason, join }
    }
}

struct Task {
    call_id: u64,
    guild_id: GuildId,
    user_id: UserId,
    config: Config,
    calls: Weak<Calls>,
    subscribers: Arc<Mutex<Subscribers>>,
    channel: watch::Sender<Option<ChannelId>>,
    commands: mpsc::UnboundedReceiver<CallCommand>,
    voice: VoiceState,

    fresh_server: bool,
    live: Option<Live>,
    connecting: Option<Connecting>,
    join: Option<PendingJoin>,

    grace: Option<Instant>,

    removed: bool,
    connected_once: bool,

    leave: Option<oneshot::Sender<()>>,
    player: Player,
    receiving: Receiving,
}

impl Task {
    async fn run(mut self) {
        loop {
            let join_deadline = self
                .join
                .as_ref()
                .filter(|_| self.connecting.is_none())
                .map(|join| join.deadline);

            let grace = self.grace.filter(|_| self.connecting.is_none());
            let due = self.player.due();
            let wants_chunk = self.player.wants_chunk();
            let receive = self.receiving.deadline();
            let step = tokio::select! {
                command = self.commands.recv() => Step::Command(command),
                step = live_step(&mut self.live) => step,
                (channel_id, result) = connected(&mut self.connecting) => {
                    Step::Connected(channel_id, result)
                }
                () = wait(join_deadline) => Step::JoinTimeout,
                () = wait(grace) => Step::GraceOver,
                chunk = self.player.recv(), if wants_chunk => Step::Chunk(chunk),
                () = wait(due) => Step::Frame,
                () = wait(receive) => Step::Receive,
            };
            if let Err(ending) = self.step(step).await {
                self.end(ending);
                return;
            }
        }
    }

    async fn step(&mut self, step: Step) -> Result<(), Ending> {
        match step {
            Step::Command(None) => Err(Ending::new(None, JoinError::Cancelled)),
            Step::Command(Some(command)) => self.command(command),
            Step::Connection(event) => self.connection_event(event),
            Step::Connected(channel_id, result) => self.connected(channel_id, result),
            Step::JoinTimeout => {
                let join = self.join.take();
                if let Err(ending) = self.idle(|| JoinError::Timeout) {
                    self.join = join;
                    return Err(ending);
                }
                if let Some(join) = join {
                    answer(join.waiters, || Err(JoinError::Timeout));
                }
                Ok(())
            }
            Step::GraceOver => Err(Ending::new(
                Some(DisconnectReason::Closed { code: DISCONNECTED }),
                JoinError::Cancelled,
            )),
            Step::Chunk(chunk) => {
                self.player.received(chunk);
                Ok(())
            }
            Step::Frame => {
                self.player.frame(self.live.as_mut()).await;
                Ok(())
            }
            Step::Packet(rtp) => {
                self.packet(rtp);
                Ok(())
            }
            Step::Receive => {
                for user_id in self.receiving.tick(Instant::now()) {
                    self.emit(&CallEvent::SpeakingStopped(user_id));
                }
                Ok(())
            }
        }
    }

    fn packet(&mut self, rtp: Rtp) {
        let now = Instant::now();
        let Some(arrival) = self.receiving.arrived(rtp, now) else {
            return;
        };
        if arrival.started {
            self.emit(&CallEvent::SpeakingStarted(arrival.user_id));
        }
        if arrival.wanted
            && let Some(live) = &mut self.live
            && let Some(opus) = live.receiver.open(arrival.user_id.get())
        {
            self.receiving.opened(arrival.user_id, rtp, opus, now);
        }
    }

    fn command(&mut self, command: CallCommand) -> Result<(), Ending> {
        match command {
            CallCommand::Join {
                channel_id,
                bitrate,
                done,
            } => {
                self.player.set_bitrate(bitrate);
                self.start_join(channel_id, done);
            }
            CallCommand::Abandoned => {
                if let Some(join) = &mut self.join {
                    join.waiters.retain(|waiter| !waiter.is_closed());
                    if join.waiters.is_empty() {
                        self.join = None;
                    }
                }
                return self.idle(|| JoinError::Cancelled);
            }
            CallCommand::State {
                session_id,
                channel_id,
            } => {
                self.voice.session_id = Some(session_id);
                self.voice.channel_id = channel_id;
                if let Some(channel_id) = channel_id {
                    self.removed = false;
                    if let Some(join) = &mut self.join
                        && join.channel_id == channel_id
                    {
                        join.state = true;
                    }
                    self.connect();
                } else if self.join.is_none() {
                    self.removed = true;
                    if self.grace.is_some() {
                        return Err(Ending::new(
                            Some(DisconnectReason::Closed { code: DISCONNECTED }),
                            JoinError::Cancelled,
                        ));
                    }
                }
            }
            CallCommand::Server { token, endpoint } => match endpoint {
                None => self.voice.server = None,
                Some(endpoint) => {
                    self.voice.server = Some((token, endpoint));
                    match &mut self.join {
                        Some(join) => join.server = true,
                        None => self.fresh_server = true,
                    }
                    self.connect();
                }
            },
            CallCommand::Wake => self.player.reconcile(),
            CallCommand::Seek { id, samples, done } => self.player.seek(id, samples, done),
            CallCommand::Leave { done } => {
                self.leave = Some(done);
                return Err(Ending::new(
                    Some(DisconnectReason::Left),
                    JoinError::Cancelled,
                ));
            }
            CallCommand::Subscribe {
                user_id,
                options,
                tx,
            } => self.receiving.subscribe(user_id, options, tx),
        }
        Ok(())
    }

    fn start_join(&mut self, channel_id: ChannelId, done: oneshot::Sender<Result<(), JoinError>>) {
        let already_there = self
            .live
            .as_ref()
            .is_some_and(|live| live.channel_id == channel_id);
        if already_there && self.connecting.is_none() && self.join.is_none() {
            let _ = done.send(Ok(()));
            return;
        }
        match &mut self.join {
            Some(join) if join.channel_id == channel_id => join.waiters.push(done),
            _ => {
                if let Some(join) = self.join.take() {
                    answer(join.waiters, || Err(JoinError::Cancelled));
                }
                if let Some(connecting) = self.connecting.take() {
                    connecting.task.abort();
                }
                self.join = Some(PendingJoin {
                    channel_id,
                    waiters: vec![done],
                    deadline: Instant::now() + self.config.timeout,
                    state: false,
                    server: false,
                });
            }
        }
    }

    fn connect(&mut self) {
        let VoiceState {
            session_id: Some(session_id),
            channel_id: Some(channel_id),
            server: Some((token, endpoint)),
        } = &self.voice
        else {
            return;
        };
        let channel_id = *channel_id;
        let timeout = match &self.join {
            Some(join) if join.channel_id == channel_id && join.state && join.server => {
                join.deadline.saturating_duration_since(Instant::now())
            }
            Some(_) => return,
            None => {
                if !core::mem::take(&mut self.fresh_server) {
                    return;
                }
                self.config.timeout
            }
        };
        let info = Info {
            guild_id: self.guild_id,
            channel_id,
            user_id: self.user_id,
            session_id: session_id.clone(),
            endpoint: endpoint.clone(),
            token: token.clone(),
        };
        let config = Config {
            timeout,
            ..self.config.clone()
        };
        if let Some(old) = self.connecting.take() {
            old.task.abort();
        }
        self.connecting = Some(Connecting {
            channel_id,
            task: tokio::spawn(connection::connect(info, config)),
        });
    }

    fn connected(
        &mut self,
        channel_id: ChannelId,
        result: Result<Connected, ConnectError>,
    ) -> Result<(), Ending> {
        match result {
            Ok((handle, sender, events)) => {
                if let Some(old) = self.live.take() {
                    old.handle.send(Command::Close);
                }

                for user_id in self.receiving.reset() {
                    self.emit(&CallEvent::SpeakingStopped(user_id));
                }
                let receiver = sender.receiver();
                self.live = Some(Live {
                    channel_id,
                    handle,
                    sender,
                    receiver,
                    events,
                });
                self.channel.send_replace(Some(channel_id));
                self.grace = None;

                self.player.speaking = false;
                match self.join.take() {
                    Some(join) => answer(join.waiters, || Ok(())),
                    None if self.connected_once => self.emit(&CallEvent::Reconnected),
                    None => {}
                }
                self.connected_once = true;
                Ok(())
            }
            Err(error) => {
                let error = Arc::new(error);
                let join_error = {
                    let error = Arc::clone(&error);
                    move || match &*error {
                        ConnectError::Timeout => JoinError::Timeout,
                        _ => JoinError::Connection(Box::new(Arc::clone(&error))),
                    }
                };

                self.grace = None;
                if self.live.is_some() {
                    if let Some(join) = self.join.take() {
                        answer(join.waiters, || Err(join_error()));
                    }
                    tracing::warn!(guild = %self.guild_id, %error, "connecting the call again failed");
                    return Ok(());
                }

                let reason = self
                    .connected_once
                    .then(|| DisconnectReason::Lost(error as Arc<dyn Error + Send + Sync>));
                Err(Ending::new(reason, join_error()))
            }
        }
    }

    fn connection_event(&mut self, event: Option<ConnectionEvent>) -> Result<(), Ending> {
        let end = match event {
            Some(ConnectionEvent::Gateway(event)) => {
                match event {
                    Event::ClientsConnect(users) => {
                        for user in users {
                            self.emit(&CallEvent::UserJoined(user));
                        }
                    }
                    Event::ClientDisconnect(user) => {
                        self.receiving.left(user);
                        self.emit(&CallEvent::UserLeft(user));
                    }
                    Event::Speaking(speaking) => {
                        self.receiving
                            .map(speaking.user_id, speaking.ssrc, Instant::now());
                        self.emit(&CallEvent::Speaking {
                            user_id: speaking.user_id,
                            ssrc: speaking.ssrc,
                        });
                    }
                    _ => {}
                }
                return Ok(());
            }
            Some(ConnectionEvent::Resumed) => {
                self.emit(&CallEvent::Resumed);
                return Ok(());
            }
            Some(ConnectionEvent::Reconnected) => {
                self.emit(&CallEvent::Reconnected);
                return Ok(());
            }
            Some(ConnectionEvent::Ended(end)) => end,
            None => End::Lost(ConnectError::Closed(None)),
        };
        self.live = None;
        self.channel.send_replace(None);
        if self.join.is_some() || self.connecting.is_some() {
            return Ok(());
        }
        match end {
            End::Stopped(DISCONNECTED) if !self.removed => {
                self.grace = Some(Instant::now() + MOVE_GRACE);
                Ok(())
            }
            End::Stopped(code) => Err(Ending::new(
                Some(DisconnectReason::Closed { code }),
                JoinError::Cancelled,
            )),
            End::Lost(error) => Err(Ending::new(
                Some(DisconnectReason::Lost(Arc::new(error))),
                JoinError::Cancelled,
            )),
            End::Closed => Err(Ending::new(None, JoinError::Cancelled)),
        }
    }

    fn idle(&self, join: impl FnOnce() -> JoinError) -> Result<(), Ending> {
        if self.live.is_none()
            && self.connecting.is_none()
            && self.join.is_none()
            && self.grace.is_none()
        {
            let reason = self.connected_once.then(|| {
                DisconnectReason::Lost(
                    Arc::new(ConnectError::Timeout) as Arc<dyn Error + Send + Sync>
                )
            });
            return Err(Ending::new(reason, join()));
        }
        Ok(())
    }

    fn emit(&self, event: &CallEvent) {
        lock(&self.subscribers)
            .senders
            .retain(|subscriber| subscriber.send(event.clone()).is_ok());
    }

    fn end(mut self, ending: Ending) {
        if let Some(calls) = self.calls.upgrade() {
            let mut calls = lock(&calls);
            if calls
                .get(&self.guild_id)
                .is_some_and(|call| call.inner.id == self.call_id)
            {
                calls.remove(&self.guild_id);
            }
        }
        if let Some(connecting) = self.connecting.take() {
            connecting.task.abort();
        }
        if let Some(live) = self.live.take() {
            live.handle.send(Command::Close);
        }
        lock(&self.player.shared.queue).close();
        self.player.end();
        self.channel.send_replace(None);
        {
            let mut subscribers = lock(&self.subscribers);
            if let Some(reason) = ending.reason {
                let event = CallEvent::Disconnected(reason);
                for subscriber in &subscribers.senders {
                    let _ = subscriber.send(event.clone());
                }
            }
            subscribers.senders.clear();
            subscribers.closed = true;
        }
        if let Some(join) = self.join.take() {
            let mut join_error = Some(ending.join);
            answer(join.waiters, || {
                Err(join_error.take().unwrap_or(JoinError::Cancelled))
            });
        }
        if let Some(leave) = self.leave.take() {
            let _ = leave.send(());
        }
    }
}

fn answer<T>(waiters: Vec<oneshot::Sender<T>>, mut result: impl FnMut() -> T) {
    for waiter in waiters {
        let _ = waiter.send(result());
    }
}

async fn live_step(live: &mut Option<Live>) -> Step {
    let Some(live) = live else {
        return core::future::pending().await;
    };

    tokio::select! {
        event = live.events.recv() => Step::Connection(event),
        rtp = live.receiver.recv() => Step::Packet(rtp),
    }
}

async fn connected(
    connecting: &mut Option<Connecting>,
) -> (ChannelId, Result<Connected, ConnectError>) {
    let Some(pending) = connecting else {
        return core::future::pending().await;
    };
    let result = (&mut pending.task)
        .await
        .unwrap_or(Err(ConnectError::Unexpected("the connection task failed")));
    let channel_id = pending.channel_id;
    *connecting = None;
    (channel_id, result)
}

async fn wait(deadline: Option<Instant>) {
    match deadline {
        Some(deadline) => sleep_until(deadline).await,
        None => core::future::pending().await,
    }
}

fn spawn_reader(
    source: Source,
    start: u64,
) -> (mpsc::Receiver<Result<Chunk, InputError>>, JoinHandle<()>) {
    let (tx, chunks) = mpsc::channel(AHEAD);
    (chunks, tokio::spawn(reading::read(source, start, tx)))
}

#[expect(
    clippy::struct_excessive_bools,
    reason = "independent facts about the track, each changed on its own"
)]
struct Playing {
    track: Arc<Track>,

    again: Option<Again>,
    chunks: mpsc::Receiver<Result<Chunk, InputError>>,
    reader: JoinHandle<()>,

    reencode: Option<Reencode>,

    history: History,

    continuing: bool,

    celt: bool,

    after: VecDeque<Frame>,
    held: Option<Read>,

    decoded: usize,

    paused: bool,

    announced: bool,

    seeks: Vec<oneshot::Sender<Result<Duration, TrackError>>>,

    gain: f32,

    closing: bool,

    seek_to: Option<u64>,

    restarting: bool,
}

impl Playing {
    fn new(track: Arc<Track>, source: Source, again: Option<Again>) -> Self {
        let (chunks, reader) = spawn_reader(source, track.position());
        Self {
            gain: track.volume(),
            paused: track.paused(),
            track,
            again,
            chunks,
            reader,
            reencode: None,
            history: History::default(),
            continuing: false,
            celt: false,
            after: VecDeque::new(),
            held: None,
            decoded: 0,
            announced: false,
            seeks: Vec::new(),
            closing: false,
            seek_to: None,
            restarting: false,
        }
    }

    fn restart(&mut self, start: u64) {
        let Some(again) = &self.again else {
            return;
        };
        self.reader.abort();
        (self.chunks, self.reader) = spawn_reader(again.source(), start);
        self.continuing = self.reencode.is_some();
        self.reencode = None;
        self.celt = false;
        self.history.clear();
        self.after.clear();
        self.held = None;
        self.decoded = 0;
        self.track.set_position(start);
        self.restarting = true;
    }

    fn pull(&mut self) -> bool {
        if self.held.is_some() {
            return false;
        }
        match self.chunks.try_recv() {
            Ok(Ok(Chunk::Frame(frame))) => {
                self.after.push_back(frame);
                true
            }
            Ok(other) => {
                self.held = Some(Some(other));
                false
            }
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => {
                self.held = Some(None);
                false
            }
        }
    }

    fn answer(&mut self, result: &Result<Duration, TrackError>) {
        for seek in self.seeks.drain(..) {
            let _ = seek.send(*result);
        }
    }
}

impl Drop for Playing {
    fn drop(&mut self) {
        self.reader.abort();
    }
}

struct Player {
    shared: Arc<Shared>,
    track: Option<Playing>,

    next: Option<Frame>,

    at: Option<Instant>,

    silence: u8,

    speaking: bool,
    bitrate: u32,

    encoder: Option<Encoder>,

    pcm: Vec<f32>,

    packet: Vec<u8>,
}

impl Player {
    fn new(shared: Arc<Shared>) -> Self {
        Self {
            shared,
            track: None,
            next: None,
            at: None,
            silence: 0,
            speaking: false,
            bitrate: codec::DEFAULT_BITRATE,
            encoder: None,
            pcm: vec![0.0; MAX_PACKET_VALUES],
            packet: vec![0; MAX_PACKET],
        }
    }

    fn set_bitrate(&mut self, bitrate: u32) {
        self.bitrate = bitrate;
        if let Some(encoder) = &mut self.encoder {
            encoder.set_bitrate(bitrate);
        }
    }

    fn reconcile(&mut self) {
        let mut queue = lock(&self.shared.queue);
        let current = queue.current.as_ref().map(|slot| slot.track.id);
        if current.is_some() && current == self.track.as_ref().map(|playing| playing.track.id) {
            drop(queue);
            self.refresh();
            return;
        }
        let started = queue.current.as_mut().and_then(|slot| {
            let input = slot.input.take()?;
            slot.track.start();
            Some((Arc::clone(&slot.track), input))
        });
        drop(queue);
        let had = self.track.take().is_some();
        self.next = None;
        if let Some((track, input)) = started {
            let (source, again) = input.into_parts();
            self.track = Some(Playing::new(track, source, again));
        } else if had {
            self.quiet();
        }
    }

    fn refresh(&mut self) {
        let Some(playing) = &mut self.track else {
            return;
        };
        let paused = playing.track.paused();
        if paused == playing.paused {
            return;
        }
        playing.paused = paused;
        if !paused {
            playing.closing = playing.seek_to.is_some();
            return;
        }
        if core::mem::take(&mut playing.announced) {
            playing.track.emit(&TrackEvent::Paused);
        }

        playing.closing = true;
    }

    fn seek(&mut self, id: u64, samples: u64, done: oneshot::Sender<Result<Duration, TrackError>>) {
        match &mut self.track {
            Some(playing)
                if playing.track.id == id && !playing.track.ended() && playing.again.is_some() =>
            {
                playing.seeks.push(done);

                let silent = (playing.paused && !playing.closing)
                    || (playing.restarting && self.next.is_none());
                if silent {
                    playing.restart(samples);
                    playing.seek_to = None;
                    playing.gain = 0.0;
                    self.next = None;
                } else {
                    playing.seek_to = Some(samples);
                    playing.closing = true;
                }
            }
            _ => {
                let _ = done.send(Err(TrackError::Ended));
            }
        }
    }

    fn quiet(&mut self) {
        if self.speaking {
            self.silence = SILENCE_FRAMES;
        }
    }

    fn end(&mut self) {
        self.track = None;
        self.next = None;
    }

    fn playing(&self) -> bool {
        self.track.as_ref().is_some_and(|playing| !playing.paused)
    }

    fn wants_chunk(&self) -> bool {
        let reading = self
            .track
            .as_ref()
            .is_some_and(|playing| !playing.seeks.is_empty() || playing.closing);
        (self.next.is_none() || self.ahead()) && self.silence == 0 && (self.playing() || reading)
    }

    fn ahead(&self) -> bool {
        matches!(self.next, Some(Frame::Opus { .. }))
            && self.track.as_ref().is_some_and(|playing| {
                playing.held.is_none()
                    && playing.after.iter().map(Frame::samples).sum::<u32>() < 960
            })
    }

    async fn recv(&mut self) -> Read {
        let Some(playing) = &mut self.track else {
            return core::future::pending().await;
        };

        if let Some(frame) = playing.after.pop_front() {
            return Some(Ok(Chunk::Frame(frame)));
        }
        if let Some(read) = playing.held.take() {
            return read;
        }
        playing.chunks.recv().await
    }

    fn received(&mut self, chunk: Option<Result<Chunk, InputError>>) {
        let Some(playing) = &mut self.track else {
            return;
        };
        match chunk {
            Some(Ok(Chunk::Frame(frame))) if self.next.is_some() => playing.after.push_back(frame),
            chunk if self.next.is_some() => playing.held = Some(chunk),
            Some(Ok(Chunk::Preroll(packets))) => playing.history = packets,
            Some(Ok(Chunk::Start(at))) => {
                playing.track.set_position(at);
                playing.answer(&Ok(duration(at)));
            }
            Some(Ok(Chunk::Frame(frame))) => {
                if playing.seek_to.is_none() {
                    if !playing.seeks.is_empty() {
                        playing.answer(&Ok(duration(playing.track.position())));
                    }
                    playing.restarting = false;
                }
                self.next = Some(frame);
            }
            Some(Err(error)) => self.finished(Some(error)),
            None => self.finished(None),
        }
    }

    fn finished(&mut self, error: Option<InputError>) {
        let Some(playing) = &mut self.track else {
            return;
        };
        if let Some(samples) = playing.seek_to.take() {
            playing.closing = false;
            playing.restart(samples);
            playing.gain = 0.0;
            return;
        }
        if error.is_none()
            && playing.again.is_some()
            && !playing.track.ended()
            && playing.track.repeat()
        {
            playing.restart(0);
            playing.track.emit(&TrackEvent::Looped);
            return;
        }
        let reason = match error {
            None => TrackEnd::Finished,
            Some(error) => {
                tracing::warn!(%error, "a track's input failed");
                TrackEnd::Failed(Arc::new(error))
            }
        };
        {
            let mut queue = lock(&self.shared.queue);
            playing.track.end(reason);
            if queue
                .current
                .as_ref()
                .is_some_and(|slot| slot.track.id == playing.track.id)
            {
                queue.advance();
            }
        }

        self.reconcile();
    }

    fn filling(&self) -> bool {
        self.speaking
            && self.next.is_none()
            && self
                .track
                .as_ref()
                .is_some_and(|playing| playing.restarting && !playing.paused)
    }

    fn due(&self) -> Option<Instant> {
        let closing = self.track.as_ref().is_some_and(|playing| playing.closing);
        let ready = self.silence > 0
            || (self.next.is_some() && !self.ahead() && (self.playing() || closing))
            || self.filling();
        ready.then(|| self.at.unwrap_or_else(Instant::now))
    }

    async fn frame(&mut self, live: Option<&mut Live>) {
        let now = Instant::now();
        let at = match self.at {
            Some(at) if at + LATE >= now => at,

            _ => now,
        };

        let mut filler = false;
        let frame = if self.silence > 0 {
            self.silence -= 1;
            None
        } else if let Some(frame) = self.next.take() {
            Some(frame)
        } else if self.filling() {
            filler = true;
            None
        } else {
            return;
        };
        let samples = frame.as_ref().map_or(960, Frame::samples);
        self.at = (frame.is_some() || filler || self.silence > 0)
            .then(|| at + duration(u64::from(samples)));

        let mut gains = (1.0, 1.0);
        let mut restart = None;
        if frame.is_some() {
            let Some(playing) = &mut self.track else {
                return;
            };
            if playing.track.ended() {
                return;
            }
            let to = if playing.closing {
                0.0
            } else {
                playing.track.volume()
            };
            gains = (playing.gain, to);
            playing.gain = to;
            playing.track.advance(samples);
            let closing = core::mem::take(&mut playing.closing);

            restart = playing.seek_to.take().filter(|_| closing);
            let pausing = closing && playing.paused;
            if !pausing && !core::mem::replace(&mut playing.announced, true) {
                playing.track.emit(&TrackEvent::Playing);
            }
            if pausing {
                self.quiet();
            }
        }
        self.send(live, frame.as_ref(), gains, filler, samples)
            .await;
        if let Some(playing) = &mut self.track {
            if let Some(Frame::Opus { data, samples }) = frame {
                playing.history.push(data, samples);
            }
            if let Some(samples) = restart {
                playing.restart(samples);
            }
        }
    }

    async fn send(
        &mut self,
        live: Option<&mut Live>,
        frame: Option<&Frame>,
        gains: (f32, f32),
        filler: bool,
        samples: u32,
    ) {
        let Some(live) = live else {
            if self.silence == 0 && frame.is_none() {
                self.speaking = false;
            }
            if let Some(encoder) = &mut self.encoder {
                encoder.independent_next();
            }
            return;
        };

        let (payload, encoded) = match frame {
            Some(frame) => {
                if !self.speaking {
                    live.handle.send(Command::Speaking(MICROPHONE));
                    self.speaking = true;
                }
                match (self.encode(frame, gains), frame) {
                    (Some(len), _) => (&self.packet[..len], true),
                    (None, Frame::Opus { data, .. }) => (&data[..], false),

                    (None, Frame::Pcm(_)) => return,
                }
            }

            None if filler => match self.silence_packet() {
                Some(len) => (&self.packet[..len], true),
                None => (&SILENCE[..], false),
            },
            None => (&SILENCE[..], false),
        };
        if let Err(error) = live.sender.send(payload, samples).await {
            tracing::debug!(%error, "a voice packet could not be sent");
        }
        if !encoded && let Some(encoder) = &mut self.encoder {
            encoder.independent_next();
        }
        if frame.is_none() && !filler && self.silence == 0 {
            live.handle.send(Command::Speaking(0));
            self.speaking = false;
        }
    }

    fn silence_packet(&mut self) -> Option<usize> {
        let encoder = match &mut self.encoder {
            Some(encoder) => encoder,
            None => self.encoder.insert(Encoder::new(self.bitrate)?),
        };
        let pcm = &mut self.pcm[..codec::FRAME_VALUES];
        pcm.fill(0.0);
        encoder.encode(pcm, 1.0, 1.0, &mut self.packet)
    }

    fn encode(&mut self, frame: &Frame, (from, to): (f32, f32)) -> Option<usize> {
        let encoder = match &mut self.encoder {
            Some(encoder) => encoder,
            None => self.encoder.insert(Encoder::new(self.bitrate)?),
        };
        let playing = self.track.as_mut()?;
        let (data, samples) = match frame {
            Frame::Pcm(bytes) => {
                let pcm = &mut self.pcm[..codec::FRAME_VALUES];
                codec::from_s16le(bytes, pcm);
                return encoder.encode(pcm, from, to, &mut self.packet);
            }
            Frame::Opus { data, samples } => (data, usize::try_from(*samples).ok()?),
        };
        let unity = (from - 1.0).abs() <= f32::EPSILON && (to - 1.0).abs() <= f32::EPSILON;
        let celt = codec::celt_only(data);
        let celt_before = core::mem::replace(&mut playing.celt, celt);
        let as_they_are = match &playing.reencode {
            None => !playing.continuing,

            Some(reencode) => reencode.steady >= SETTLE && celt && celt_before,
        };
        if unity && as_they_are {
            playing.reencode = None;
            playing.decoded = 0;
            return None;
        }
        let lookahead = encoder.lookahead();
        if playing.reencode.is_none() {
            let mut decoder = Decoder::new()?;
            let mut fifo = VecDeque::with_capacity(MAX_PACKET_VALUES * 2);

            let before = 960 - lookahead;

            let mut previous = 0;
            for packet in playing.history.packets() {
                previous = decoder.decode(packet, &mut self.pcm).unwrap_or(0);
            }
            let continuing = core::mem::take(&mut playing.continuing);
            if !continuing {
                let kept = previous.min(before);
                fifo.extend(core::iter::repeat_n(0.0, (before - kept) * 2));
                fifo.extend(&self.pcm[(previous - kept) * 2..previous * 2]);
            }
            let current = decoder.decode(data, &mut self.pcm)?;
            fifo.extend(&self.pcm[..current * 2]);
            playing.decoded = 1;
            let mut reencode = Reencode {
                decoder,
                fifo,
                steady: 0,
            };
            if continuing {
                fill(playing, &mut reencode, &mut self.pcm, lookahead * 2);
                reencode.fifo.drain(..lookahead * 2);
            } else {
                fill(playing, &mut reencode, &mut self.pcm, codec::FRAME_VALUES);

                let mut prime: Vec<f32> = reencode
                    .fifo
                    .drain(..codec::FRAME_VALUES)
                    .map(|value| value * from)
                    .collect();
                let _ = encoder.encode(&mut prime, 1.0, 1.0, &mut self.packet);

                encoder.independent_next();
            }
            playing.reencode = Some(reencode);
        } else if playing.decoded == 0 {
            let reencode = playing.reencode.as_mut()?;
            let decoded = reencode.decoder.decode(data, &mut self.pcm)?;
            reencode.fifo.extend(&self.pcm[..decoded * 2]);
            playing.decoded = 1;
        }
        let mut reencode = playing.reencode.take()?;
        reencode.steady = if unity {
            reencode.steady.saturating_add(1)
        } else {
            0
        };
        let values = samples * 2;
        fill(playing, &mut reencode, &mut self.pcm, values);
        for (out, value) in self.pcm[..values]
            .iter_mut()
            .zip(reencode.fifo.drain(..values.min(reencode.fifo.len())))
        {
            *out = value;
        }
        playing.decoded = playing.decoded.saturating_sub(1);
        playing.reencode = Some(reencode);
        encoder.encode(&mut self.pcm[..values], from, to, &mut self.packet)
    }
}

fn fill(playing: &mut Playing, reencode: &mut Reencode, pcm: &mut [f32], values: usize) {
    while reencode.fifo.len() < values {
        let index = playing.decoded.saturating_sub(1);
        if index >= playing.after.len() && !playing.pull() {
            break;
        }
        let Some(Frame::Opus { data, .. }) = playing.after.get(index) else {
            break;
        };
        let Some(decoded) = reencode.decoder.decode(data, pcm) else {
            break;
        };
        reencode.fifo.extend(&pcm[..decoded * 2]);
        playing.decoded += 1;
    }
    let missing = values.saturating_sub(reencode.fifo.len());
    reencode.fifo.extend(core::iter::repeat_n(0.0, missing));
}
