use core::{
    fmt,
    pin::Pin,
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
    task::{Context, Poll},
    time::Duration,
};
use std::sync::{Arc, Mutex, Weak};

use futures_util::Stream;
use tokio::sync::{mpsc, oneshot};

use crate::voice::{
    InputError, TrackError,
    call::{CallCommand, lock},
    queue::Shared,
};

pub(crate) fn duration(samples: u64) -> Duration {
    Duration::from_nanos(samples.saturating_mul(62_500) / 3)
}

pub(crate) fn samples(duration: Duration) -> u64 {
    u64::try_from(duration.as_nanos() * 3 / 62_500).unwrap_or(u64::MAX)
}

pub(crate) struct Track {
    pub(crate) id: u64,

    seekable: bool,

    started: AtomicBool,
    paused: AtomicBool,
    ended: AtomicBool,

    position: AtomicU64,

    volume: AtomicU32,
    looping: Mutex<Loop>,
    subscribers: Mutex<Subscribers>,
    call: Weak<Shared>,
}

#[derive(Default)]
struct Subscribers {
    senders: Vec<mpsc::UnboundedSender<TrackEvent>>,

    closed: bool,
}

impl Track {
    pub(crate) fn new(id: u64, seekable: bool, call: Weak<Shared>) -> Arc<Self> {
        Arc::new(Self {
            id,
            seekable,
            started: AtomicBool::new(false),
            paused: AtomicBool::new(false),
            ended: AtomicBool::new(false),
            position: AtomicU64::new(0),
            volume: AtomicU32::new(1.0_f32.to_bits()),
            looping: Mutex::new(Loop::Off),
            subscribers: Mutex::new(Subscribers::default()),
            call,
        })
    }

    pub(crate) fn end(&self, reason: TrackEnd) -> bool {
        let mut subscribers = lock(&self.subscribers);
        if subscribers.closed {
            return false;
        }
        self.ended.store(true, Ordering::Release);
        let event = TrackEvent::Ended(reason);
        for subscriber in &subscribers.senders {
            let _ = subscriber.send(event.clone());
        }
        subscribers.senders.clear();
        subscribers.closed = true;
        true
    }

    pub(crate) fn emit(&self, event: &TrackEvent) {
        lock(&self.subscribers)
            .senders
            .retain(|subscriber| subscriber.send(event.clone()).is_ok());
    }

    pub(crate) fn ended(&self) -> bool {
        self.ended.load(Ordering::Acquire)
    }

    pub(crate) fn started(&self) -> bool {
        self.started.load(Ordering::Acquire)
    }

    pub(crate) fn start(&self) {
        self.started.store(true, Ordering::Release);
    }

    pub(crate) fn paused(&self) -> bool {
        self.paused.load(Ordering::Acquire)
    }

    pub(crate) fn position(&self) -> u64 {
        self.position.load(Ordering::Acquire)
    }

    pub(crate) fn set_position(&self, samples: u64) {
        self.position.store(samples, Ordering::Release);
    }

    pub(crate) fn advance(&self, samples: u32) {
        self.position
            .fetch_add(u64::from(samples), Ordering::AcqRel);
    }

    pub(crate) fn volume(&self) -> f32 {
        f32::from_bits(self.volume.load(Ordering::Acquire))
    }

    pub(crate) fn repeat(&self) -> bool {
        let mut looping = lock(&self.looping);
        match *looping {
            Loop::Forever => true,
            Loop::Times(left @ 1..) => {
                *looping = Loop::Times(left - 1);
                true
            }
            Loop::Times(0) | Loop::Off => false,
        }
    }

    fn info(&self) -> TrackInfo {
        let state = if self.ended() {
            PlayState::Ended
        } else if !self.started() {
            PlayState::Queued
        } else if self.paused() {
            PlayState::Paused
        } else {
            PlayState::Playing
        };
        TrackInfo {
            state,
            position: duration(self.position()),
            volume: self.volume(),
            looping: *lock(&self.looping),
        }
    }
}

#[derive(Clone)]
pub struct TrackHandle {
    track: Arc<Track>,
}

impl fmt::Debug for TrackHandle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrackHandle")
            .field("id", &self.track.id)
            .field("info", &self.track.info())
            .finish()
    }
}

impl PartialEq for TrackHandle {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.track, &other.track)
    }
}

impl Eq for TrackHandle {}

impl TrackHandle {
    pub(crate) fn new(track: Arc<Track>) -> Self {
        Self { track }
    }

    pub fn play(&self) -> Result<(), TrackError> {
        self.set_paused(false)
    }

    pub fn pause(&self) -> Result<(), TrackError> {
        self.set_paused(true)
    }

    fn set_paused(&self, paused: bool) -> Result<(), TrackError> {
        self.alive()?;
        self.track.paused.store(paused, Ordering::Release);
        self.wake();
        Ok(())
    }

    pub fn stop(&self) -> Result<(), TrackError> {
        self.alive()?;
        if let Some(call) = self.track.call.upgrade() {
            lock(&call.queue).remove(self.track.id, TrackEnd::Stopped);
            call.wake();
        }

        self.track.end(TrackEnd::Stopped);
        Ok(())
    }

    pub fn set_volume(&self, volume: f32) -> Result<(), TrackError> {
        self.alive()?;
        let volume = if volume.is_nan() {
            0.0
        } else {
            volume.clamp(0.0, 2.0)
        };
        self.track.volume.store(volume.to_bits(), Ordering::Release);
        Ok(())
    }

    pub fn set_loop(&self, looping: Loop) -> Result<(), TrackError> {
        self.alive()?;
        if !self.track.seekable {
            return Err(TrackError::Unsupported);
        }
        *lock(&self.track.looping) = looping;
        Ok(())
    }

    pub async fn seek(&self, position: Duration) -> Result<Duration, TrackError> {
        self.alive()?;
        if !self.track.seekable {
            return Err(TrackError::Unsupported);
        }
        let target = samples(position);
        let call = self.track.call.upgrade().ok_or(TrackError::Ended)?;
        let (done, result) = oneshot::channel();
        {
            let queue = lock(&call.queue);
            if !self.track.started() {
                self.track.set_position(target);
                return Ok(duration(target));
            }
            drop(queue);
        }
        let command = CallCommand::Seek {
            id: self.track.id,
            samples: target,
            done,
        };
        if call.commands.send(command).is_err() {
            return Err(TrackError::Ended);
        }
        drop(call);
        result.await.unwrap_or(Err(TrackError::Ended))
    }

    #[must_use]
    pub fn info(&self) -> TrackInfo {
        self.track.info()
    }

    #[must_use]
    pub fn events(&self) -> TrackEvents {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut subscribers = lock(&self.track.subscribers);
        if !subscribers.closed {
            subscribers.senders.push(tx);
        }
        TrackEvents { rx }
    }

    fn alive(&self) -> Result<(), TrackError> {
        if self.track.ended() {
            Err(TrackError::Ended)
        } else {
            Ok(())
        }
    }

    fn wake(&self) {
        if let Some(call) = self.track.call.upgrade() {
            call.wake();
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Loop {
    #[default]
    Off,

    Forever,

    Times(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum PlayState {
    Queued,

    Playing,

    Paused,

    Ended,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct TrackInfo {
    pub state: PlayState,

    pub position: Duration,

    pub volume: f32,

    pub looping: Loop,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TrackEvent {
    Playing,

    Paused,

    Looped,

    Ended(TrackEnd),
}

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum TrackEnd {
    Finished,

    Stopped,

    Replaced,

    Skipped,

    Failed(Arc<InputError>),

    CallEnded,
}

impl fmt::Display for TrackEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Finished => f.write_str("the track played to its end"),
            Self::Stopped => f.write_str("the track was stopped"),
            Self::Replaced => f.write_str("another track replaced it"),
            Self::Skipped => f.write_str("the track was skipped"),
            Self::Failed(error) => write!(f, "the track's input failed: {error}"),
            Self::CallEnded => f.write_str("its call ended"),
        }
    }
}

#[derive(Debug)]
pub struct TrackEvents {
    rx: mpsc::UnboundedReceiver<TrackEvent>,
}

impl TrackEvents {
    pub async fn next(&mut self) -> Option<TrackEvent> {
        self.rx.recv().await
    }
}

impl Stream for TrackEvents {
    type Item = TrackEvent;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<TrackEvent>> {
        self.rx.poll_recv(cx)
    }
}

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use super::{duration, samples};

    #[test]
    fn samples_and_durations_convert() {
        assert_eq!(duration(48_000), Duration::from_secs(1));
        assert_eq!(duration(960), Duration::from_millis(20));
        assert_eq!(samples(Duration::from_millis(500)), 24_000);
        assert_eq!(samples(Duration::from_secs(90)), 4_320_000);
        assert_eq!(samples(duration(123_456)), 123_456);
        assert_eq!(samples(Duration::MAX), u64::MAX);
    }
}
