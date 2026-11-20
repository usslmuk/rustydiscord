use core::{
    fmt,
    pin::Pin,
    task::{Context, Poll, ready},
    time::Duration,
};
use std::collections::{HashMap, VecDeque};

use crate::model::id::UserId;
use futures_util::Stream;
use tokio::{
    sync::mpsc::{self, error::TrySendError},
    time::Instant,
};

use crate::voice::{codec::Decoder, opus, udp::Rtp};

const SILENCE: [u8; 3] = [0xF8, 0xFF, 0xFE];

const REORDER: Duration = Duration::from_millis(60);

const SPEAKING: Duration = Duration::from_millis(100);

const JUMP: u16 = 50;

pub(crate) const QUEUE: usize = 256;

const MAX_SAMPLES: usize = 5760;

const FRAME: u32 = 960;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EndBehavior {
    #[default]
    Manual,

    AfterSilence(Duration),

    AfterInactivity(Duration),
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ReceiveOptions {
    decode: bool,
    end: EndBehavior,
}

impl ReceiveOptions {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            decode: false,
            end: EndBehavior::Manual,
        }
    }

    #[must_use]
    pub const fn decode(mut self, decode: bool) -> Self {
        self.decode = decode;
        self
    }

    #[must_use]
    pub const fn end(mut self, end: EndBehavior) -> Self {
        self.end = end;
        self
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct VoicePacket {
    pub sequence: u16,

    pub timestamp: u32,

    pub audio: VoiceData,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VoiceData {
    Opus(Vec<u8>),

    Pcm(Vec<i16>),

    Lost,
}

pub struct VoiceStream {
    rx: mpsc::Receiver<Item>,

    decoding: Option<Box<Decoding>>,
}

impl VoiceStream {
    pub(crate) fn new(mut rx: mpsc::Receiver<Item>, options: ReceiveOptions) -> Self {
        let decoding = if options.decode {
            let decoding = Decoder::new().map(|decoder| {
                Box::new(Decoding {
                    decoder,
                    pcm: vec![0; MAX_SAMPLES * 2],
                    ahead: None,
                    lost: None,
                })
            });
            if decoding.is_none() {
                rx.close();
            }
            decoding
        } else {
            None
        };
        Self { rx, decoding }
    }

    pub async fn next(&mut self) -> Option<VoicePacket> {
        core::future::poll_fn(|cx| Pin::new(&mut *self).poll_next(cx)).await
    }

    #[cfg(test)]
    pub(crate) fn try_next(&mut self) -> Result<VoicePacket, mpsc::error::TryRecvError> {
        let mut cx = Context::from_waker(std::task::Waker::noop());
        match Pin::new(self).poll_next(&mut cx) {
            Poll::Ready(Some(packet)) => Ok(packet),
            Poll::Ready(None) => Err(mpsc::error::TryRecvError::Disconnected),
            Poll::Pending => Err(mpsc::error::TryRecvError::Empty),
        }
    }
}

impl fmt::Debug for VoiceStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VoiceStream")
            .field("decode", &self.decoding.is_some())
            .finish_non_exhaustive()
    }
}

impl Stream for VoiceStream {
    type Item = VoicePacket;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<VoicePacket>> {
        let Self { rx, decoding } = self.get_mut();
        let Some(decoding) = decoding else {
            return rx.poll_recv(cx).map(|item| {
                item.map(|item| match item {
                    Item::Packet {
                        sequence,
                        timestamp,
                        opus,
                    } => VoicePacket {
                        sequence,
                        timestamp,
                        audio: VoiceData::Opus(opus),
                    },
                    Item::Lost {
                        sequence,
                        timestamp,
                        ..
                    } => VoicePacket {
                        sequence,
                        timestamp,
                        audio: VoiceData::Lost,
                    },
                })
            });
        };
        decoding.poll(rx, cx)
    }
}

struct Decoding {
    decoder: Decoder,

    pcm: Vec<i16>,

    ahead: Option<Item>,

    lost: Option<(u16, u32, u32)>,
}

impl Decoding {
    fn poll(
        &mut self,
        rx: &mut mpsc::Receiver<Item>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<VoicePacket>> {
        loop {
            if let Some((sequence, timestamp, samples)) = self.lost {
                let after = ready!(rx.poll_recv(cx));
                self.lost = None;
                let audio = match &after {
                    Some(Item::Packet {
                        sequence: next,
                        opus,
                        ..
                    }) if *next == sequence.wrapping_add(1) => {
                        decode(&mut self.decoder, &mut self.pcm, opus, true, samples)
                    }
                    _ => decode(&mut self.decoder, &mut self.pcm, &[], false, samples),
                };
                self.ahead = after;
                return Poll::Ready(Some(VoicePacket {
                    sequence,
                    timestamp,
                    audio,
                }));
            }
            let item = match self.ahead.take() {
                Some(item) => item,
                None => match ready!(rx.poll_recv(cx)) {
                    Some(item) => item,
                    None => return Poll::Ready(None),
                },
            };
            match item {
                Item::Packet {
                    sequence,
                    timestamp,
                    opus,
                } => {
                    let audio = decode(&mut self.decoder, &mut self.pcm, &opus, false, FRAME);
                    return Poll::Ready(Some(VoicePacket {
                        sequence,
                        timestamp,
                        audio,
                    }));
                }
                Item::Lost {
                    sequence,
                    timestamp,
                    samples,
                } => self.lost = Some((sequence, timestamp, samples)),
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Item {
    Packet {
        sequence: u16,
        timestamp: u32,
        opus: Vec<u8>,
    },
    Lost {
        sequence: u16,
        timestamp: u32,

        samples: u32,
    },
}

#[derive(Debug)]
struct Held {
    timestamp: u32,
    opus: Vec<u8>,
}

#[derive(Debug, Default)]
struct Reorder {
    next: Option<u16>,

    timestamp: u32,
    samples: u32,

    held: VecDeque<Option<Held>>,

    since: Option<Instant>,
}

impl Reorder {
    fn push(
        &mut self,
        sequence: u16,
        timestamp: u32,
        opus: &[u8],
        now: Instant,
        out: &mut Vec<Item>,
    ) {
        let Some(next) = self.next else {
            self.pass(sequence, timestamp, opus.to_vec(), out);
            return;
        };
        let ahead = sequence.wrapping_sub(next);
        let behind = next.wrapping_sub(sequence);
        if ahead > JUMP {
            if behind != 0 && behind <= JUMP {
                return;
            }

            self.flush(out);
            self.pass(sequence, timestamp, opus.to_vec(), out);
            return;
        }
        let index = usize::from(ahead);
        if index == 0 && self.held.is_empty() {
            self.pass(sequence, timestamp, opus.to_vec(), out);
            return;
        }
        if self.held.len() <= index {
            self.held.resize_with(index + 1, || None);
        }
        if self.held[index].is_some() {
            return;
        }
        self.held[index] = Some(Held {
            timestamp,
            opus: opus.to_vec(),
        });
        self.since.get_or_insert(now);
        self.drain(out);
    }

    fn drain(&mut self, out: &mut Vec<Item>) {
        while let Some(Some(_)) = self.held.front() {
            let Some(Some(held)) = self.held.pop_front() else {
                break;
            };
            let sequence = self.next.unwrap_or_default();
            self.pass(sequence, held.timestamp, held.opus, out);
        }
        if self.held.is_empty() {
            self.since = None;
        }
    }

    fn flush(&mut self, out: &mut Vec<Item>) {
        while let Some(slot) = self.held.pop_front() {
            let sequence = self.next.unwrap_or_default();
            if let Some(held) = slot {
                self.pass(sequence, held.timestamp, held.opus, out);
            } else {
                let timestamp = self.timestamp.wrapping_add(self.samples);
                out.push(Item::Lost {
                    sequence,
                    timestamp,
                    samples: self.samples,
                });
                self.timestamp = timestamp;
                self.next = Some(sequence.wrapping_add(1));
            }
        }
        self.since = None;
    }

    fn pass(&mut self, sequence: u16, timestamp: u32, opus: Vec<u8>, out: &mut Vec<Item>) {
        self.next = Some(sequence.wrapping_add(1));
        self.timestamp = timestamp;
        self.samples = opus::packet_samples(&opus).unwrap_or(FRAME);
        out.push(Item::Packet {
            sequence,
            timestamp,
            opus,
        });
    }

    fn deadline(&self) -> Option<Instant> {
        self.since.map(|since| since + REORDER)
    }
}

struct Subscription {
    tx: mpsc::Sender<Item>,
    end: EndBehavior,

    end_at: Option<Instant>,
}

impl Subscription {
    fn send(&mut self, item: &Item, now: Instant) -> bool {
        if let Item::Packet { opus, .. } = item {
            match self.end {
                EndBehavior::Manual => {}
                EndBehavior::AfterSilence(wait) => {
                    if opus[..] != SILENCE || self.end_at.is_none() {
                        self.end_at = Some(now + wait);
                    }
                }
                EndBehavior::AfterInactivity(wait) => self.end_at = Some(now + wait),
            }
        }
        match self.tx.try_send(item.clone()) {
            Ok(()) => true,
            Err(TrySendError::Full(_)) => {
                tracing::trace!("dropping a received packet: its stream's reader is behind");
                true
            }
            Err(TrySendError::Closed(_)) => false,
        }
    }

    fn ended(&self, now: Instant) -> bool {
        self.end_at.is_some_and(|at| at <= now) || self.tx.is_closed()
    }
}

fn decode(
    decoder: &mut Decoder,
    pcm: &mut [i16],
    packet: &[u8],
    fec: bool,
    samples: u32,
) -> VoiceData {
    let lost = usize::try_from(samples).map_or(MAX_SAMPLES, |samples| samples.min(MAX_SAMPLES));
    let written = if packet.is_empty() || fec {
        decoder.decode_i16(packet, fec, &mut pcm[..lost * 2])
    } else {
        decoder
            .decode_i16(packet, false, pcm)
            .or_else(|| decoder.decode_i16(&[], false, &mut pcm[..lost * 2]))
    };
    match written {
        Some(samples) => VoiceData::Pcm(pcm[..samples * 2].to_vec()),
        None => VoiceData::Lost,
    }
}

#[derive(Default)]
struct User {
    last: Option<Instant>,
    order: Reorder,
    subscriptions: Vec<Subscription>,
}

impl User {
    fn send(&mut self, items: &[Item], now: Instant) {
        for item in items {
            self.subscriptions
                .retain_mut(|subscription| subscription.send(item, now));
        }
        if self.subscriptions.is_empty() {
            self.order = Reorder::default();
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct Arrival {
    pub(crate) user_id: UserId,

    pub(crate) started: bool,

    pub(crate) wanted: bool,
}

#[derive(Default)]
pub(crate) struct Receiving {
    ssrcs: HashMap<u32, UserId>,
    users: HashMap<UserId, User>,

    items: Vec<Item>,
}

impl Receiving {
    pub(crate) fn subscribe(
        &mut self,
        user_id: UserId,
        options: ReceiveOptions,
        tx: mpsc::Sender<Item>,
    ) {
        self.users
            .entry(user_id)
            .or_default()
            .subscriptions
            .push(Subscription {
                tx,
                end: options.end,
                end_at: None,
            });
    }

    pub(crate) fn map(&mut self, user_id: UserId, ssrc: u32, now: Instant) {
        if self.ssrcs.get(&ssrc) == Some(&user_id) {
            return;
        }
        let had = self.ssrcs.values().any(|user| *user == user_id);
        self.ssrcs.retain(|_, user| *user != user_id);
        self.ssrcs.insert(ssrc, user_id);
        if had && let Some(user) = self.users.get_mut(&user_id) {
            let mut items = core::mem::take(&mut self.items);
            user.order.flush(&mut items);
            user.send(&items, now);
            user.order = Reorder::default();
            items.clear();
            self.items = items;
        }
    }

    pub(crate) fn left(&mut self, user_id: UserId) {
        self.ssrcs.retain(|_, user| *user != user_id);
    }

    pub(crate) fn reset(&mut self) -> Vec<UserId> {
        self.ssrcs.clear();
        let mut stopped = Vec::new();
        for (user_id, user) in &mut self.users {
            if user.last.take().is_some() {
                stopped.push(*user_id);
            }
            user.order = Reorder::default();
        }
        self.users.retain(|_, user| !user.subscriptions.is_empty());
        stopped
    }

    pub(crate) fn arrived(&mut self, rtp: Rtp, now: Instant) -> Option<Arrival> {
        let user_id = *self.ssrcs.get(&rtp.ssrc)?;
        let user = self.users.entry(user_id).or_default();
        let started = user.last.replace(now).is_none();
        Some(Arrival {
            user_id,
            started,
            wanted: !user.subscriptions.is_empty(),
        })
    }

    pub(crate) fn opened(&mut self, user_id: UserId, rtp: Rtp, opus: &[u8], now: Instant) {
        let Some(user) = self.users.get_mut(&user_id) else {
            return;
        };
        let mut items = core::mem::take(&mut self.items);
        user.order
            .push(rtp.sequence, rtp.timestamp, opus, now, &mut items);
        user.send(&items, now);
        items.clear();
        self.items = items;
    }

    pub(crate) fn deadline(&self) -> Option<Instant> {
        self.users
            .values()
            .flat_map(|user| {
                let ends = user
                    .subscriptions
                    .iter()
                    .filter_map(|subscription| subscription.end_at)
                    .min();
                [
                    user.last.map(|last| last + SPEAKING),
                    user.order.deadline(),
                    ends,
                ]
            })
            .flatten()
            .min()
    }

    pub(crate) fn tick(&mut self, now: Instant) -> Vec<UserId> {
        let mut stopped = Vec::new();
        let mut items = core::mem::take(&mut self.items);
        for (user_id, user) in &mut self.users {
            if user.last.is_some_and(|last| last + SPEAKING <= now) {
                user.last = None;
                stopped.push(*user_id);
            }
            if user.order.deadline().is_some_and(|at| at <= now) {
                user.order.flush(&mut items);
                user.send(&items, now);
                items.clear();
            }

            user.subscriptions
                .retain(|subscription| !subscription.ended(now));
            if user.subscriptions.is_empty() {
                user.order = Reorder::default();
            }
        }
        self.items = items;
        self.users
            .retain(|_, user| user.last.is_some() || !user.subscriptions.is_empty());
        stopped
    }
}

#[cfg(test)]
mod tests;
