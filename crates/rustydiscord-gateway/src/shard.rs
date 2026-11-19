use core::{fmt::Write as _, future::Future, pin::Pin, time::Duration};
use std::sync::{Arc, Mutex, PoisonError};

use futures_util::{SinkExt, StreamExt};
use rustydiscord_model::gateway::{
    API_VERSION, CloseCode, DispatchEvent, GatewayEvent, GatewayEventDeserializer, OpCode, ShardId,
    payload::{
        CommandPayload, GatewayCapabilities, GatewayCommand, Heartbeat, Identify, Resume,
        UpdatePresence,
    },
};
use serde::de::DeserializeSeed;
use tokio::{
    net::TcpStream,
    sync::mpsc,
    time::{Instant, Sleep, sleep, sleep_until},
};
use tokio_websockets::{ClientBuilder, Limits, MaybeTlsStream, Message, WebSocketStream};

use crate::{
    Config, Encoding,
    compression::Decompressor,
    error::{ReceiveError, ReceiveErrorKind, SendError},
    send_limit::SendLimit,
};

#[cfg(feature = "etf")]
use crate::etf;

type Connection = WebSocketStream<MaybeTlsStream<TcpStream>>;

type IdentifyTurn = Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>;

const MAX_BACKOFF: Duration = Duration::from_secs(60);

const CLOSE_TIMEOUT: Duration = Duration::from_secs(2);

const RESUMABLE_CLOSE_CODE: u16 = 4000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ShardState {
    Disconnected,

    Identifying,

    Resuming,

    Active,

    FatallyClosed(CloseCode),

    Closed,
}

pub struct Shard {
    id: ShardId,
    config: Config,
    state: ShardState,
    connection: Option<Connection>,
    decompressor: Option<Decompressor>,
    heartbeat: Option<Heartbeater>,
    hello_deadline: Option<Pin<Box<Sleep>>>,

    identify_turn: Option<IdentifyTurn>,
    session: Option<Session>,
    sequence: Option<u64>,
    latency: Option<Duration>,
    reconnect_attempts: u32,
    connect_after: Option<Instant>,
    commands_tx: mpsc::UnboundedSender<Command>,
    commands_rx: mpsc::UnboundedReceiver<Command>,
    send_limit: SendLimit,
    held: Option<HeldCommand>,

    presence: SharedPresence,
}

type SharedPresence = Arc<Mutex<Option<UpdatePresence>>>;

struct Session {
    id: Box<str>,
    resume_url: Box<str>,
}

struct Heartbeater {
    interval: Duration,
    sleep: Pin<Box<Sleep>>,
    last_sent: Option<Instant>,
    acked: bool,
}

struct HeldCommand {
    command: Command,

    until: Pin<Box<Sleep>>,
}

#[derive(Debug)]
pub(crate) enum Command {
    Text(String),
    #[cfg_attr(
        not(feature = "etf"),
        allow(dead_code, reason = "constructed by the ETF send path")
    )]
    Binary(Vec<u8>),
}

enum Wake {
    Heartbeat,
    HelloTimeout,
    Identify,
    Command(Command),
    HeldCommandAllowed,
    Message(Option<Result<Message, tokio_websockets::Error>>),
}

impl core::fmt::Debug for Shard {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Shard")
            .field("id", &self.id)
            .field("state", &self.state)
            .field(
                "session_id",
                &self.session.as_ref().map(|session| &session.id),
            )
            .field("sequence", &self.sequence)
            .field("latency", &self.latency)
            .finish_non_exhaustive()
    }
}

impl Shard {
    #[must_use]
    pub fn new(
        id: ShardId,
        token: impl Into<String>,
        intents: rustydiscord_model::gateway::Intents,
    ) -> Self {
        Self::with_config(id, Config::new(token, intents))
    }

    #[must_use]
    pub fn with_config(id: ShardId, config: Config) -> Self {
        let (commands_tx, commands_rx) = mpsc::unbounded_channel();
        let presence = Arc::new(Mutex::new(config.presence.clone()));
        Self {
            id,
            config,
            state: ShardState::Disconnected,
            connection: None,
            decompressor: None,
            heartbeat: None,
            hello_deadline: None,
            identify_turn: None,
            session: None,
            sequence: None,
            latency: None,
            reconnect_attempts: 0,
            connect_after: None,
            commands_tx,
            commands_rx,
            send_limit: SendLimit::default(),
            held: None,
            presence,
        }
    }

    #[must_use]
    pub const fn id(&self) -> ShardId {
        self.id
    }

    #[must_use]
    pub const fn config(&self) -> &Config {
        &self.config
    }

    #[must_use]
    pub const fn state(&self) -> ShardState {
        self.state
    }

    #[must_use]
    pub const fn latency(&self) -> Option<Duration> {
        self.latency
    }

    #[must_use]
    pub fn session_id(&self) -> Option<&str> {
        self.session.as_ref().map(|session| &*session.id)
    }

    #[must_use]
    pub const fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    #[must_use]
    pub fn sender(&self) -> MessageSender {
        MessageSender {
            tx: self.commands_tx.clone(),
            presence: Arc::clone(&self.presence),
            encoding: self.config.encoding,
        }
    }

    pub async fn next_event(&mut self) -> Option<Result<GatewayEvent, ReceiveError>> {
        loop {
            if matches!(
                self.state,
                ShardState::FatallyClosed(_) | ShardState::Closed
            ) {
                return None;
            }
            if self.connection.is_none() {
                if let Some(at) = self.connect_after.take() {
                    sleep_until(at).await;
                }
                if let Err(error) = self.connect().await {
                    self.schedule_reconnect();
                    return Some(Err(error));
                }
            }
            if let Some(result) = self.step().await {
                return Some(result);
            }
        }
    }

    pub async fn close(&mut self) {
        if let Some(mut connection) = self.connection.take() {
            let frame = Message::close(Some(tokio_websockets::CloseCode::NORMAL_CLOSURE), "");
            let _ = tokio::time::timeout(CLOSE_TIMEOUT, connection.send(frame)).await;
        }
        self.reset_connection();
        self.session = None;
        self.sequence = None;
        self.state = ShardState::Closed;
        tracing::info!(shard = %self.id, "shard closed");
    }

    async fn step(&mut self) -> Option<Result<GatewayEvent, ReceiveError>> {
        let active = self.state == ShardState::Active;

        let receive_commands = active && self.held.is_none();
        let connection = self.connection.as_mut()?;
        let heartbeat = self
            .heartbeat
            .as_mut()
            .map(|heartbeat| heartbeat.sleep.as_mut());
        let hello = self.hello_deadline.as_mut().map(Pin::as_mut);
        let identify = self.identify_turn.as_mut().map(|turn| {
            turn.get_mut()
                .unwrap_or_else(PoisonError::into_inner)
                .as_mut()
        });
        let held = self
            .held
            .as_mut()
            .filter(|_| active)
            .map(|held| held.until.as_mut());

        let wake = tokio::select! {
            biased;
            () = optional(heartbeat) => Wake::Heartbeat,
            () = optional(hello) => Wake::HelloTimeout,
            () = optional(identify) => Wake::Identify,
            () = optional(held) => Wake::HeldCommandAllowed,
            Some(command) = self.commands_rx.recv(), if receive_commands => Wake::Command(command),
            message = connection.next() => Wake::Message(message),
        };

        match wake {
            Wake::Heartbeat => self.heartbeat_due().await.err().map(Err),
            Wake::HelloTimeout => {
                tracing::warn!(shard = %self.id, "no Hello received in time, reconnecting");
                self.disconnect(true).await;
                None
            }
            Wake::Identify => {
                self.identify_turn = None;
                self.identify().await.err().map(Err)
            }
            Wake::Command(command) => self.send_limited(command).await.err().map(Err),
            Wake::HeldCommandAllowed => match self.held.take() {
                Some(held) => self.send_limited(held.command).await.err().map(Err),
                None => None,
            },
            Wake::Message(None) => {
                tracing::info!(shard = %self.id, "connection ended without a close frame, reconnecting");
                self.disconnect(true).await;
                None
            }
            Wake::Message(Some(Err(error))) => {
                self.disconnect(true).await;
                Some(Err(ReceiveError::with_source(
                    ReceiveErrorKind::Connection,
                    error,
                )))
            }
            Wake::Message(Some(Ok(message))) => self.handle_message(&message).await,
        }
    }

    async fn handle_message(
        &mut self,
        message: &Message,
    ) -> Option<Result<GatewayEvent, ReceiveError>> {
        if let Some((code, reason)) = message.as_close() {
            let code = u16::from(code);
            let reason = reason.to_owned();
            return self.handle_close(Some(code), &reason);
        }
        if message.is_close() {
            return self.handle_close(None, "");
        }

        let (sequence, parsed) = if let Some(json) = message.as_text() {
            parse(json)
        } else if message.is_binary() {
            let payload = message.as_payload();
            if let Some(decompressor) = self.decompressor.as_mut() {
                match decompressor.decompress(payload) {
                    Ok(None) => return None,
                    Ok(Some(bytes)) => Self::parse_bytes(self.config.encoding, bytes),
                    Err(error) => {
                        let error = ReceiveError::with_source(ReceiveErrorKind::Decompress, error);
                        self.disconnect(true).await;
                        return Some(Err(error));
                    }
                }
            } else {
                match self.config.encoding {
                    Encoding::Json => {
                        tracing::warn!(shard = %self.id, "binary message received without compression, ignoring");
                        return None;
                    }
                    #[cfg(feature = "etf")]
                    Encoding::Etf => etf::parse(payload),
                }
            }
        } else {
            return None;
        };

        if let Some(sequence) = sequence {
            self.sequence = Some(sequence);
        }
        match parsed {
            Ok(event) => match self.process(&event).await {
                Ok(()) => Some(Ok(event)),
                Err(error) => Some(Err(error)),
            },
            Err(error) => {
                tracing::warn!(shard = %self.id, %error, "skipping a gateway payload that failed to parse");
                Some(Err(error))
            }
        }
    }

    async fn process(&mut self, event: &GatewayEvent) -> Result<(), ReceiveError> {
        match event {
            GatewayEvent::Hello(hello) => {
                self.hello_deadline = None;
                let interval = Duration::from_millis(hello.heartbeat_interval);

                let first = interval.mul_f64(fastrand::f64());
                self.heartbeat = Some(Heartbeater {
                    interval,
                    sleep: Box::pin(sleep(first)),
                    last_sent: None,
                    acked: true,
                });
                if self.session.is_some() && self.sequence.is_some() {
                    self.resume().await?;
                } else {
                    self.schedule_identify();
                }
            }
            GatewayEvent::HeartbeatAck => {
                if let Some(heartbeat) = self.heartbeat.as_mut() {
                    heartbeat.acked = true;
                    if let Some(sent) = heartbeat.last_sent {
                        self.latency = Some(sent.elapsed());
                    }
                }
            }
            GatewayEvent::Heartbeat => self.send_heartbeat().await?,
            GatewayEvent::Reconnect => {
                tracing::info!(shard = %self.id, "Discord requested a reconnect");
                self.disconnect(true).await;
            }
            GatewayEvent::InvalidSession(resumable) => {
                tracing::info!(shard = %self.id, resumable, "session invalidated");
                self.disconnect(*resumable).await;

                self.connect_after =
                    Some(Instant::now() + Duration::from_millis(fastrand::u64(1_000..=5_000)));
            }
            GatewayEvent::Dispatch(_, DispatchEvent::Ready(ready)) => {
                self.session = Some(Session {
                    id: ready.session_id.as_str().into(),
                    resume_url: ready.resume_gateway_url.as_str().into(),
                });
                self.state = ShardState::Active;
                self.reconnect_attempts = 0;
                tracing::info!(shard = %self.id, user = %ready.user.username, guilds = ready.guilds.len(), "session ready");
            }
            GatewayEvent::Dispatch(_, DispatchEvent::Resumed) => {
                self.state = ShardState::Active;
                self.reconnect_attempts = 0;
                tracing::info!(shard = %self.id, "session resumed");
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_close(
        &mut self,
        code: Option<u16>,
        reason: &str,
    ) -> Option<Result<GatewayEvent, ReceiveError>> {
        tracing::info!(shard = %self.id, code, reason, "gateway closed the connection");
        self.connection = None;
        self.reset_connection();

        if let Some(code) = code.filter(|code| (4000..5000).contains(code)) {
            let code = CloseCode::from(code);
            if !code.can_reconnect() {
                tracing::error!(shard = %self.id, ?code, "fatal close code, the shard will stop");
                self.state = ShardState::FatallyClosed(code);
                return Some(Err(ReceiveError::new(ReceiveErrorKind::FatallyClosed {
                    code,
                })));
            }
            if matches!(
                code,
                CloseCode::InvalidSequence | CloseCode::SessionTimedOut
            ) {
                self.session = None;
                self.sequence = None;
            }
        }
        None
    }

    async fn connect(&mut self) -> Result<(), ReceiveError> {
        let base = match (&self.session, self.sequence) {
            (Some(session), Some(_)) => &*session.resume_url,
            _ => &*self.config.gateway_url,
        };
        let mut url = String::with_capacity(base.len() + 48);
        url.push_str(base.trim_end_matches('/'));
        let _ = write!(
            url,
            "/?v={API_VERSION}&encoding={}",
            self.config.encoding.query_value()
        );
        if let Some(compression) = self.config.compression.query_value() {
            let _ = write!(url, "&compress={compression}");
        }

        tracing::debug!(shard = %self.id, %url, "connecting to the gateway");
        let builder = ClientBuilder::new()
            .uri(&url)
            .map_err(|error| ReceiveError::with_source(ReceiveErrorKind::Connect, error))?
            .connector(&self.config.tls)
            .limits(Limits::unlimited());
        let (connection, _) = tokio::time::timeout(self.config.connect_timeout, builder.connect())
            .await
            .map_err(|_| {
                ReceiveError::with_source(ReceiveErrorKind::Connect, "connection timed out")
            })?
            .map_err(|error| ReceiveError::with_source(ReceiveErrorKind::Connect, error))?;

        self.decompressor = Decompressor::new(self.config.compression)
            .map_err(|error| ReceiveError::with_source(ReceiveErrorKind::Connect, error))?;
        self.connection = Some(connection);
        self.hello_deadline = Some(Box::pin(sleep(self.config.hello_timeout)));
        tracing::info!(shard = %self.id, "connected to the gateway");
        Ok(())
    }

    fn schedule_reconnect(&mut self) {
        self.reconnect_attempts = self.reconnect_attempts.saturating_add(1);
        let exponent = self.reconnect_attempts.saturating_sub(1).min(6);
        let delay = Duration::from_secs(1 << exponent).min(MAX_BACKOFF);
        let delay = delay.mul_f64(0.5 + fastrand::f64() / 2.0);
        tracing::warn!(
            shard = %self.id,
            attempt = self.reconnect_attempts,
            delay_ms = u64::try_from(delay.as_millis()).unwrap_or(u64::MAX),
            "connecting failed, retrying after a delay"
        );
        self.connect_after = Some(Instant::now() + delay);
    }

    fn schedule_identify(&mut self) {
        self.identify_turn = Some(Mutex::new(self.config.identify_queue.wait_turn(self.id)));
        self.state = ShardState::Identifying;
    }

    async fn identify(&mut self) -> Result<(), ReceiveError> {
        let identify = Identify {
            token: (*self.config.token).into(),
            properties: self.config.properties.clone(),
            large_threshold: self.config.large_threshold,
            shard: Some(self.id),
            intents: self.config.intents,
            presence: self
                .presence
                .lock()
                .unwrap_or_else(PoisonError::into_inner)
                .clone(),
            capabilities: GatewayCapabilities::empty(),
        };
        self.state = ShardState::Identifying;
        tracing::debug!(shard = %self.id, "identifying");
        self.send_command(&identify).await
    }

    async fn resume(&mut self) -> Result<(), ReceiveError> {
        let (Some(session), Some(seq)) = (&self.session, self.sequence) else {
            return Ok(());
        };
        let resume = Resume {
            token: (*self.config.token).into(),
            session_id: (*session.id).into(),
            seq,
        };
        self.state = ShardState::Resuming;
        tracing::debug!(shard = %self.id, seq, "resuming");
        self.send_command(&resume).await
    }

    async fn heartbeat_due(&mut self) -> Result<(), ReceiveError> {
        if self
            .heartbeat
            .as_ref()
            .is_some_and(|heartbeat| !heartbeat.acked)
        {
            tracing::warn!(shard = %self.id, "no heartbeat ACK received (zombied connection), reconnecting");
            self.disconnect(true).await;
            return Ok(());
        }
        self.send_heartbeat().await
    }

    async fn send_heartbeat(&mut self) -> Result<(), ReceiveError> {
        let sequence = self.sequence;
        if let Some(heartbeat) = self.heartbeat.as_mut() {
            let now = Instant::now();
            heartbeat.last_sent = Some(now);
            heartbeat.acked = false;
            heartbeat.sleep.as_mut().reset(now + heartbeat.interval);
        }
        self.send_command(&Heartbeat(sequence)).await
    }

    async fn send_command<T: GatewayCommand>(&mut self, command: &T) -> Result<(), ReceiveError> {
        let payload = CommandPayload::new(command);
        let command = match self.config.encoding {
            Encoding::Json => {
                let json =
                    serde_json::to_string(&payload).expect("gateway commands always serialize");
                Command::Text(json)
            }
            #[cfg(feature = "etf")]
            Encoding::Etf => {
                Command::Binary(etf::encode(&payload).expect("gateway commands always serialize"))
            }
        };
        self.send(command).await
    }

    async fn send_limited(&mut self, command: Command) -> Result<(), ReceiveError> {
        match self.send_limit.try_acquire(Instant::now()) {
            Ok(()) => self.send(command).await,
            Err(at) => {
                tracing::debug!(shard = %self.id, "gateway send limit reached, holding queued commands");
                self.held = Some(HeldCommand {
                    command,
                    until: Box::pin(sleep_until(at)),
                });
                Ok(())
            }
        }
    }

    async fn send(&mut self, command: Command) -> Result<(), ReceiveError> {
        let Some(connection) = self.connection.as_mut() else {
            return Ok(());
        };
        let message = match command {
            Command::Text(json) => Message::text(json),
            Command::Binary(bytes) => Message::binary(bytes),
        };
        if let Err(error) = connection.send(message).await {
            self.disconnect(true).await;
            return Err(ReceiveError::with_source(
                ReceiveErrorKind::Connection,
                error,
            ));
        }
        Ok(())
    }

    async fn disconnect(&mut self, resume: bool) {
        if let Some(mut connection) = self.connection.take() {
            let code = tokio_websockets::CloseCode::try_from(RESUMABLE_CLOSE_CODE).ok();
            let _ = tokio::time::timeout(CLOSE_TIMEOUT, connection.send(Message::close(code, "")))
                .await;
        }
        self.reset_connection();
        if !resume {
            self.session = None;
            self.sequence = None;
        }
    }

    fn reset_connection(&mut self) {
        self.connection = None;
        self.decompressor = None;
        self.heartbeat = None;
        self.hello_deadline = None;

        self.identify_turn = None;
        self.state = ShardState::Disconnected;

        self.send_limit.clear();
        if let Some(held) = self.held.as_mut() {
            held.until.as_mut().reset(Instant::now());
        }
    }
}

async fn optional<F: Future<Output = ()> + ?Sized>(future: Option<Pin<&mut F>>) {
    match future {
        Some(future) => future.await,
        None => core::future::pending().await,
    }
}

fn parse(json: &str) -> (Option<u64>, Result<GatewayEvent, ReceiveError>) {
    let Some(seed) = GatewayEventDeserializer::from_json(json) else {
        return (
            None,
            Err(ReceiveError::with_source(
                ReceiveErrorKind::Deserialize,
                "payload has no valid `op` field",
            )),
        );
    };
    let sequence = (seed.op() == OpCode::Dispatch)
        .then(|| seed.sequence())
        .flatten();
    let mut deserializer = serde_json::Deserializer::from_str(json);
    let result = seed
        .deserialize(&mut deserializer)
        .map_err(|error| ReceiveError::with_source(ReceiveErrorKind::Deserialize, error));
    (sequence, result)
}

impl Shard {
    fn parse_bytes(
        encoding: Encoding,
        bytes: &[u8],
    ) -> (Option<u64>, Result<GatewayEvent, ReceiveError>) {
        match encoding {
            Encoding::Json => match core::str::from_utf8(bytes) {
                Ok(json) => parse(json),
                Err(error) => (
                    None,
                    Err(ReceiveError::with_source(
                        ReceiveErrorKind::Deserialize,
                        error,
                    )),
                ),
            },
            #[cfg(feature = "etf")]
            Encoding::Etf => etf::parse(bytes),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MessageSender {
    tx: mpsc::UnboundedSender<Command>,
    presence: SharedPresence,
    encoding: Encoding,
}

impl MessageSender {
    #[must_use]
    pub fn detached() -> Self {
        Self {
            tx: mpsc::unbounded_channel().0,
            presence: SharedPresence::default(),
            encoding: Encoding::Json,
        }
    }

    pub fn command<T: GatewayCommand>(&self, command: &T) -> Result<(), SendError> {
        let payload = CommandPayload::new(command);
        let command = match self.encoding {
            Encoding::Json => {
                Command::Text(serde_json::to_string(&payload).map_err(SendError::Serialize)?)
            }
            #[cfg(feature = "etf")]
            Encoding::Etf => {
                Command::Binary(crate::etf::encode(&payload).map_err(SendError::Encode)?)
            }
        };
        self.tx.send(command).map_err(|_| SendError::Closed)
    }

    pub fn update_presence(&self, presence: &UpdatePresence) -> Result<(), SendError> {
        self.command(presence)?;
        *self.presence.lock().unwrap_or_else(PoisonError::into_inner) = Some(presence.clone());
        Ok(())
    }

    #[must_use]
    pub fn is_closed(&self) -> bool {
        self.tx.is_closed()
    }
}

const _: () = {
    const fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<Shard>();
    assert_send_sync::<MessageSender>();
    assert_send_sync::<crate::Config>();
};
