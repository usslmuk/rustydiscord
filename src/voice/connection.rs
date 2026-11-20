use core::{fmt, time::Duration};
use std::{
    io,
    net::SocketAddr,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use crate::model::id::{ChannelId, GuildId, UserId};
use futures_util::{SinkExt, StreamExt};
use tokio::{
    net::{TcpStream, UdpSocket},
    sync::{mpsc, watch},
    time::{Instant, sleep_until, timeout, timeout_at},
};
use tokio_websockets::{ClientBuilder, Connector, MaybeTlsStream, Message, WebSocketStream};

use crate::voice::{
    crypto::{Cipher, Mode, OVERHEAD, RTP_HEADER},
    dave::{Dave, Handled, Reply},
    payload::{self, AfterClose, DecodeError, Event, Incoming},
    tls,
    udp::{self, Rtp},
};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

const CLOSE_TIMEOUT: Duration = Duration::from_secs(1);

#[derive(Clone)]
pub(crate) struct Info {
    pub(crate) guild_id: GuildId,

    pub(crate) channel_id: ChannelId,
    pub(crate) user_id: UserId,
    pub(crate) session_id: String,

    pub(crate) endpoint: String,

    pub(crate) token: String,
}

impl fmt::Debug for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Info")
            .field("guild_id", &self.guild_id)
            .field("channel_id", &self.channel_id)
            .field("user_id", &self.user_id)
            .field("session_id", &self.session_id)
            .field("endpoint", &self.endpoint)
            .field("token", &"<redacted>")
            .finish()
    }
}

#[derive(Clone)]
pub(crate) struct Config {
    pub(crate) timeout: Duration,

    pub(crate) retry_delays: Vec<Duration>,

    pub(crate) max_dave_protocol_version: u16,

    pub(crate) keepalive: Duration,

    pub(crate) discovery_retry: Duration,

    pub(crate) tls: Arc<Connector>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            retry_delays: [1, 2, 4, 8, 16].map(Duration::from_secs).to_vec(),
            max_dave_protocol_version: davey::DAVE_PROTOCOL_VERSION,
            keepalive: Duration::from_secs(5),
            discovery_retry: Duration::from_secs(1),
            tls: Arc::new(tls::connector()),
        }
    }
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("timeout", &self.timeout)
            .field("retry_delays", &self.retry_delays)
            .field("max_dave_protocol_version", &self.max_dave_protocol_version)
            .field("keepalive", &self.keepalive)
            .field("discovery_retry", &self.discovery_retry)
            .finish_non_exhaustive()
    }
}

#[derive(Debug)]
pub(crate) struct Udp {
    pub(crate) socket: UdpSocket,
    pub(crate) ssrc: u32,
    pub(crate) cipher: Cipher,
}

#[derive(Debug)]
pub(crate) enum ConnectError {
    Endpoint,

    WebSocket(tokio_websockets::Error),

    Closed(Option<u16>),

    Timeout,

    Io(io::Error),

    Decode(DecodeError),

    NoMode,

    Unexpected(&'static str),
}

impl fmt::Display for ConnectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Endpoint => f.write_str("the voice server endpoint is not a valid URL"),
            Self::WebSocket(error) => write!(f, "the voice gateway connection failed: {error}"),
            Self::Closed(Some(code)) => write!(f, "the voice gateway closed with code {code}"),
            Self::Closed(None) => f.write_str("the voice gateway connection was lost"),
            Self::Timeout => f.write_str("the voice connection timed out"),
            Self::Io(error) => write!(f, "the voice UDP socket failed: {error}"),
            Self::Decode(error) => error.fmt(f),
            Self::NoMode => f.write_str("the voice server offers no supported encryption mode"),
            Self::Unexpected(what) => write!(f, "the voice server misbehaved: {what}"),
        }
    }
}

impl std::error::Error for ConnectError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::WebSocket(error) => Some(error),
            Self::Io(error) => Some(error),
            Self::Decode(error) => Some(error),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) enum ConnectionEvent {
    Gateway(Event),

    Resumed,

    Reconnected,

    Ended(End),
}

#[derive(Debug)]
pub(crate) enum End {
    Closed,

    Stopped(u16),

    Lost(ConnectError),
}

#[derive(Debug)]
pub(crate) enum Command {
    Speaking(u32),

    Close,
}

#[derive(Debug)]
pub(crate) struct Handle {
    commands: mpsc::UnboundedSender<Command>,
    #[cfg(test)]
    dave: Arc<Mutex<Dave>>,
}

impl Handle {
    pub(crate) fn send(&self, command: Command) -> bool {
        self.commands.send(command).is_ok()
    }

    #[cfg(test)]
    pub(crate) fn dave_protocol_version(&self) -> u16 {
        lock(&self.dave).version()
    }

    #[cfg(test)]
    pub(crate) fn dave_ready(&self) -> bool {
        lock(&self.dave).ready()
    }
}

fn lock(dave: &Mutex<Dave>) -> MutexGuard<'_, Dave> {
    dave.lock().unwrap_or_else(PoisonError::into_inner)
}

pub(crate) async fn connect(
    info: Info,
    config: Config,
) -> Result<(Handle, Sender, mpsc::UnboundedReceiver<ConnectionEvent>), ConnectError> {
    let mut session = timeout(config.timeout, identify(&info, &config))
        .await
        .map_err(|_| ConnectError::Timeout)??;
    let (commands, commands_rx) = mpsc::unbounded_channel();
    let (events, events_rx) = mpsc::unbounded_channel();
    let (udp, udp_rx) = watch::channel(Arc::clone(&session.udp));
    let dave = Arc::new(Mutex::new(Dave::new(
        info.user_id.get(),
        info.channel_id.get(),
    )));
    let handle = Handle {
        commands,
        #[cfg(test)]
        dave: Arc::clone(&dave),
    };
    let shared = Shared { events, udp, dave };

    let replies = session
        .dave
        .take()
        .map_or_else(Vec::new, |version| shared.start(version));
    let sender = Sender::new(udp_rx, Arc::clone(&shared.dave));
    tokio::spawn(run(info, config, session, commands_rx, shared, replies));
    Ok((handle, sender, events_rx))
}

struct Shared {
    events: mpsc::UnboundedSender<ConnectionEvent>,

    udp: watch::Sender<Arc<Udp>>,
    dave: Arc<Mutex<Dave>>,
}

impl Shared {
    fn event(&self, event: ConnectionEvent) {
        let _ = self.events.send(event);
    }

    fn start(&self, version: u16) -> Vec<Reply> {
        tracing::debug!(dave_protocol_version = version, "voice session started");
        lock(&self.dave).start(version)
    }

    fn handle(&self, event: Event) -> Vec<Reply> {
        match lock(&self.dave).handle(event) {
            Handled::Replies(replies) => replies,
            Handled::Forward(event) => {
                self.event(ConnectionEvent::Gateway(event));
                Vec::new()
            }
        }
    }
}

struct Session {
    socket: Socket,
    udp: Arc<Udp>,
    heartbeat: Heartbeat,

    seq: Option<u64>,

    dave: Option<u16>,

    pending: Vec<Event>,
}

struct Heartbeat {
    interval: Duration,
    next: Instant,
    nonce: u64,

    unacknowledged: Option<u64>,
}

impl Heartbeat {
    fn new(interval: f64) -> Option<Self> {
        let interval = Duration::try_from_secs_f64(interval / 1000.0).ok()?;
        (!interval.is_zero()).then(|| Self {
            interval,
            next: Instant::now() + interval,
            nonce: 0,
            unacknowledged: None,
        })
    }

    fn acknowledged(&mut self, nonce: u64) {
        if self.unacknowledged == Some(nonce) {
            self.unacknowledged = None;
        }
    }

    async fn beat(&mut self, socket: &mut Socket, seq: Option<u64>) -> Result<(), ConnectError> {
        self.nonce = self.nonce.wrapping_add(1);
        self.unacknowledged = Some(self.nonce);
        self.next += self.interval;
        send(socket, Message::text(payload::heartbeat(self.nonce, seq))).await
    }
}

fn url(endpoint: &str) -> String {
    let base = if endpoint.starts_with("ws://") || endpoint.starts_with("wss://") {
        endpoint.trim_end_matches('/').to_owned()
    } else {
        format!("wss://{}", endpoint.trim_end_matches('/'))
    };
    format!("{base}/?v={}", payload::VERSION)
}

async fn open(info: &Info, config: &Config) -> Result<Socket, ConnectError> {
    let (socket, _) = ClientBuilder::new()
        .uri(&url(&info.endpoint))
        .map_err(|_| ConnectError::Endpoint)?
        .connector(&config.tls)
        .connect()
        .await
        .map_err(ConnectError::WebSocket)?;
    Ok(socket)
}

async fn send(socket: &mut Socket, message: Message) -> Result<(), ConnectError> {
    socket.send(message).await.map_err(ConnectError::WebSocket)
}

fn decode(message: &Message) -> Option<Result<Incoming, DecodeError>> {
    if let Some(text) = message.as_text() {
        Some(payload::decode_text(text))
    } else if message.is_binary() {
        Some(payload::decode_binary(message.as_payload()))
    } else {
        None
    }
}

async fn next(
    socket: &mut Socket,
    heartbeat: &mut Option<Heartbeat>,
    seq: &mut Option<u64>,
) -> Result<Event, ConnectError> {
    loop {
        let deadline = heartbeat.as_ref().map(|heartbeat| heartbeat.next);
        let message = tokio::select! {
            message = socket.next() => message,
            () = wait(deadline) => {
                if let Some(heartbeat) = heartbeat {
                    heartbeat.beat(socket, *seq).await?;
                }
                continue;
            }
        };
        let message = match message {
            None => return Err(ConnectError::Closed(None)),
            Some(Err(error)) => return Err(ConnectError::WebSocket(error)),
            Some(Ok(message)) => message,
        };
        if let Some((code, _)) = message.as_close() {
            return Err(ConnectError::Closed(Some(u16::from(code))));
        }
        let Some(incoming) = decode(&message) else {
            continue;
        };
        let Incoming { seq: number, event } = incoming.map_err(ConnectError::Decode)?;
        if number.is_some() {
            *seq = number;
        }
        match (event, heartbeat.as_mut()) {
            (Event::HeartbeatAck(nonce), Some(heartbeat)) => heartbeat.acknowledged(nonce),
            (Event::HeartbeatAck(_), None) => {}
            (event, _) => return Ok(event),
        }
    }
}

fn hello(interval: f64) -> Result<Heartbeat, ConnectError> {
    Heartbeat::new(interval).ok_or(ConnectError::Unexpected(
        "Hello has no usable heartbeat interval",
    ))
}

async fn wait(deadline: Option<Instant>) {
    match deadline {
        Some(deadline) => sleep_until(deadline).await,
        None => core::future::pending().await,
    }
}

async fn identify(info: &Info, config: &Config) -> Result<Session, ConnectError> {
    let mut socket = open(info, config).await?;
    let identify = payload::identify(
        info.guild_id,
        info.user_id,
        &info.session_id,
        &info.token,
        config.max_dave_protocol_version,
    );
    send(&mut socket, Message::text(identify)).await?;
    let mut heartbeat = None;
    let mut ready = None;
    let mut seq = None;
    let mut pending = Vec::new();
    while heartbeat.is_none() || ready.is_none() {
        match next(&mut socket, &mut heartbeat, &mut seq).await? {
            Event::Hello(interval) => heartbeat = Some(hello(interval)?),
            Event::Ready(event) => ready = Some(event),
            event => pending.push(event),
        }
    }
    let ready = ready.ok_or(ConnectError::Unexpected("no Ready"))?;
    let mode = Mode::choose(ready.modes.iter().map(String::as_str)).ok_or(ConnectError::NoMode)?;
    let bind: SocketAddr = if ready.ip.is_ipv4() {
        (core::net::Ipv4Addr::UNSPECIFIED, 0).into()
    } else {
        (core::net::Ipv6Addr::UNSPECIFIED, 0).into()
    };
    let socket_udp = UdpSocket::bind(bind).await.map_err(ConnectError::Io)?;
    socket_udp
        .connect((ready.ip, ready.port))
        .await
        .map_err(ConnectError::Io)?;
    let address = discover(&socket_udp, ready.ssrc, config.discovery_retry).await?;
    let select = payload::select_protocol(address.ip(), address.port(), mode);
    send(&mut socket, Message::text(select)).await?;
    let description = loop {
        match next(&mut socket, &mut heartbeat, &mut seq).await? {
            Event::SessionDescription(description) => break description,
            event => pending.push(event),
        }
    };
    if Mode::from_name(&description.mode) != Some(mode) {
        return Err(ConnectError::Unexpected(
            "the voice server described another encryption mode",
        ));
    }
    let heartbeat = heartbeat.ok_or(ConnectError::Unexpected("no Hello"))?;
    let udp = Udp {
        socket: socket_udp,
        ssrc: ready.ssrc,
        cipher: Cipher::new(mode, &description.secret_key),
    };
    Ok(Session {
        socket,
        udp: Arc::new(udp),
        heartbeat,
        seq,
        dave: Some(description.dave_protocol_version),
        pending,
    })
}

async fn discover(
    socket: &UdpSocket,
    ssrc: u32,
    retry: Duration,
) -> Result<SocketAddr, ConnectError> {
    let request = udp::discovery_request(ssrc);
    let mut buffer = [0; 1500];
    loop {
        socket.send(&request).await.map_err(ConnectError::Io)?;
        let deadline = Instant::now() + retry;
        while let Ok(received) = timeout_at(deadline, socket.recv(&mut buffer)).await {
            let len = received.map_err(ConnectError::Io)?;
            if let Some(address) = udp::discovery_response(&buffer[..len], ssrc) {
                return Ok(address);
            }
        }
    }
}

async fn resume(
    info: &Info,
    config: &Config,
    udp: Arc<Udp>,
    seq: Option<u64>,
) -> Result<Session, ConnectError> {
    let mut socket = open(info, config).await?;
    let resume = payload::resume(info.guild_id, &info.session_id, &info.token, seq);
    send(&mut socket, Message::text(resume)).await?;
    let mut heartbeat = None;
    let mut seq = seq;
    let mut pending = Vec::new();
    let mut resumed = false;
    while heartbeat.is_none() || !resumed {
        match next(&mut socket, &mut heartbeat, &mut seq).await? {
            Event::Hello(interval) => heartbeat = Some(hello(interval)?),
            Event::Resumed => resumed = true,
            event => pending.push(event),
        }
    }
    let heartbeat = heartbeat.ok_or(ConnectError::Unexpected("no Hello"))?;
    Ok(Session {
        socket,
        udp,
        heartbeat,
        seq,
        dave: None,
        pending,
    })
}

enum Drive {
    Closed,

    Lost(Option<u16>),
}

async fn run(
    info: Info,
    config: Config,
    mut session: Session,
    mut commands: mpsc::UnboundedReceiver<Command>,
    shared: Shared,
    mut replies: Vec<Reply>,
) {
    let mut speaking = None;
    loop {
        let _ = send_replies(&mut session.socket, core::mem::take(&mut replies)).await;
        for event in core::mem::take(&mut session.pending) {
            let replies = shared.handle(event);
            let _ = send_replies(&mut session.socket, replies).await;
        }
        let code = match drive(&mut session, &mut commands, &shared, &mut speaking, &config).await {
            Drive::Closed => {
                shared.event(ConnectionEvent::Ended(End::Closed));

                let _ = timeout(CLOSE_TIMEOUT, session.socket.close()).await;
                return;
            }
            Drive::Lost(code) => code,
        };
        tracing::debug!(guild = %info.guild_id, ?code, "voice connection lost");
        match reconnect(&info, &config, &session, code, &mut commands, &mut speaking).await {
            Ok((mut new, resumed)) => {
                if !resumed {
                    if let Some(version) = new.dave.take() {
                        replies = shared.start(version);
                    }
                    shared.udp.send_replace(Arc::clone(&new.udp));
                    if let Some(flags) = speaking {
                        let message = Message::text(payload::speaking(flags, new.udp.ssrc));

                        let _ = send(&mut new.socket, message).await;
                    }
                }
                session = new;
                shared.event(if resumed {
                    ConnectionEvent::Resumed
                } else {
                    ConnectionEvent::Reconnected
                });
            }
            Err(end) => {
                shared.event(ConnectionEvent::Ended(end));
                return;
            }
        }
    }
}

async fn send_replies(socket: &mut Socket, replies: Vec<Reply>) -> Result<(), ConnectError> {
    for reply in replies {
        let message = match reply {
            Reply::Text(text) => Message::text(text),
            Reply::Binary(bytes) => Message::binary(bytes),
        };
        send(socket, message).await?;
    }
    Ok(())
}

async fn drive(
    session: &mut Session,
    commands: &mut mpsc::UnboundedReceiver<Command>,
    shared: &Shared,
    speaking: &mut Option<u32>,
    config: &Config,
) -> Drive {
    let mut keepalive = Instant::now() + config.keepalive;
    loop {
        tokio::select! {
            message = session.socket.next() => {
                let Some(Ok(message)) = message else {
                    return Drive::Lost(None);
                };
                if let Some((code, _)) = message.as_close() {
                    return Drive::Lost(Some(u16::from(code)));
                }
                match decode(&message) {
                    None => {}
                    Some(Err(error)) => {
                        tracing::warn!(%error, "ignoring a voice gateway message");
                    }
                    Some(Ok(Incoming { seq, event })) => {
                        if seq.is_some() {
                            session.seq = seq;
                        }
                        match event {
                            Event::HeartbeatAck(nonce) => session.heartbeat.acknowledged(nonce),
                            Event::Hello(interval) => {
                                if let Some(heartbeat) = Heartbeat::new(interval) {
                                    session.heartbeat = heartbeat;
                                }
                            }
                            event => {
                                let replies = shared.handle(event);
                                if send_replies(&mut session.socket, replies).await.is_err() {
                                    return Drive::Lost(None);
                                }
                            }
                        }
                    }
                }
            }
            () = sleep_until(session.heartbeat.next) => {
                if session.heartbeat.unacknowledged.is_some() {
                    tracing::warn!("a voice heartbeat was not acknowledged; reconnecting");
                    return Drive::Lost(None);
                }
                if session.heartbeat.beat(&mut session.socket, session.seq).await.is_err() {
                    return Drive::Lost(None);
                }
            }
            () = sleep_until(keepalive) => {

                let _ = session.udp.socket.try_send(&udp::keepalive(session.udp.ssrc));
                keepalive += config.keepalive;
            }
            command = commands.recv() => {
                let message = match command {
                    None | Some(Command::Close) => return Drive::Closed,
                    Some(Command::Speaking(flags)) => {
                        *speaking = Some(flags);
                        Message::text(payload::speaking(flags, session.udp.ssrc))
                    }
                };
                if send(&mut session.socket, message).await.is_err() {
                    return Drive::Lost(None);
                }
            }
        }
    }
}

async fn reconnect(
    info: &Info,
    config: &Config,
    session: &Session,
    code: Option<u16>,
    commands: &mut mpsc::UnboundedReceiver<Command>,
    speaking: &mut Option<u32>,
) -> Result<(Session, bool), End> {
    match payload::after_close(code) {
        AfterClose::Stop => return Err(End::Stopped(code.unwrap_or_default())),
        AfterClose::Identify => return renew(info, config).await.map(|session| (session, false)),
        AfterClose::Resume => {}
    }
    let mut last = ConnectError::Closed(code);
    for delay in core::iter::once(Duration::ZERO).chain(config.retry_delays.iter().copied()) {
        pause(jitter(delay), commands, speaking).await?;
        let attempt = timeout(
            config.timeout,
            resume(info, config, Arc::clone(&session.udp), session.seq),
        )
        .await
        .unwrap_or(Err(ConnectError::Timeout));
        match attempt {
            Ok(new) => return Ok((new, true)),
            Err(ConnectError::Closed(code)) => match payload::after_close(code) {
                AfterClose::Resume => last = ConnectError::Closed(code),
                AfterClose::Identify => {
                    return renew(info, config).await.map(|session| (session, false));
                }
                AfterClose::Stop => return Err(End::Stopped(code.unwrap_or_default())),
            },
            Err(error) => last = error,
        }
        tracing::debug!(guild = %info.guild_id, error = %last, "resuming a voice connection failed");
    }
    Err(End::Lost(last))
}

async fn renew(info: &Info, config: &Config) -> Result<Session, End> {
    timeout(config.timeout, identify(info, config))
        .await
        .unwrap_or(Err(ConnectError::Timeout))
        .map_err(End::Lost)
}

fn jitter(delay: Duration) -> Duration {
    delay.mul_f64(0.5 + fastrand::f64() / 2.0)
}

async fn pause(
    delay: Duration,
    commands: &mut mpsc::UnboundedReceiver<Command>,
    speaking: &mut Option<u32>,
) -> Result<(), End> {
    let deadline = Instant::now() + delay;
    loop {
        tokio::select! {
            () = sleep_until(deadline) => return Ok(()),
            command = commands.recv() => match command {
                None | Some(Command::Close) => return Err(End::Closed),
                Some(Command::Speaking(flags)) => *speaking = Some(flags),
            },
        }
    }
}

#[derive(Debug)]
pub(crate) struct Sender {
    udp: watch::Receiver<Arc<Udp>>,
    dave: Arc<Mutex<Dave>>,
    sequence: u16,
    timestamp: u32,
    nonce: u32,

    buffer: Vec<u8>,
}

impl Sender {
    fn new(udp: watch::Receiver<Arc<Udp>>, dave: Arc<Mutex<Dave>>) -> Self {
        Self {
            udp,
            dave,

            sequence: fastrand::u16(..),
            timestamp: fastrand::u32(..),
            nonce: 0,
            buffer: Vec::with_capacity(RTP_HEADER + 1275 + OVERHEAD),
        }
    }

    pub(crate) fn receiver(&self) -> Receiver {
        Receiver {
            udp: self.udp.clone(),
            dave: Arc::clone(&self.dave),
            packet: vec![0; DATAGRAM],
            len: 0,
            frame: Vec::with_capacity(DATAGRAM),
            ended: false,
        }
    }

    pub(crate) async fn send(&mut self, opus: &[u8], samples: u32) -> io::Result<bool> {
        let udp = Arc::clone(&*self.udp.borrow());
        let timestamp = self.timestamp;
        self.timestamp = timestamp.wrapping_add(samples);
        let len = {
            let mut dave = lock(&self.dave);
            let Some(frame) = dave.encrypt(opus) else {
                return Ok(false);
            };
            self.buffer.clear();
            self.buffer
                .extend_from_slice(&udp::rtp_header(self.sequence, timestamp, udp.ssrc));
            self.buffer.extend_from_slice(frame);
            frame.len()
        };
        self.buffer.resize(RTP_HEADER + len + OVERHEAD, 0);
        let sealed = udp
            .cipher
            .seal(&mut self.buffer, RTP_HEADER, len, self.nonce)
            .ok_or_else(|| io::Error::other("the packet could not be encrypted"))?;
        self.nonce = self.nonce.wrapping_add(1);
        self.sequence = self.sequence.wrapping_add(1);
        udp.socket.send(&self.buffer[..sealed]).await?;
        Ok(true)
    }
}

const DATAGRAM: usize = 2048;

#[derive(Debug)]
pub(crate) struct Receiver {
    udp: watch::Receiver<Arc<Udp>>,
    dave: Arc<Mutex<Dave>>,

    packet: Vec<u8>,
    len: usize,

    frame: Vec<u8>,

    ended: bool,
}

impl Receiver {
    pub(crate) async fn recv(&mut self) -> Rtp {
        loop {
            if self.ended {
                return core::future::pending().await;
            }
            let udp = Arc::clone(&*self.udp.borrow_and_update());
            tokio::select! {
                received = udp.socket.recv(&mut self.packet) => match received {
                    Ok(len) => {
                        if let Some(rtp) = udp::rtp(&self.packet[..len]) {
                            self.len = len;
                            return rtp;
                        }
                    }

                    Err(error) => tracing::trace!(%error, "a voice datagram could not be received"),
                },
                changed = self.udp.changed() => self.ended = changed.is_err(),
            }
        }
    }

    pub(crate) fn open(&mut self, user_id: u64) -> Option<&[u8]> {
        let range = self
            .udp
            .borrow()
            .cipher
            .open(&mut self.packet[..self.len])?;

        self.len = 0;
        lock(&self.dave).decrypt(user_id, &self.packet[range], &mut self.frame)
    }
}

#[cfg(test)]
pub(crate) mod tests;
