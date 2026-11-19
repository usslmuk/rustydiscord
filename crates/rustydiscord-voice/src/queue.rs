use core::fmt;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use tokio::sync::mpsc;

use crate::{
    Input, TrackHandle,
    call::{CallCommand, lock},
    track::{Track, TrackEnd},
};

pub(crate) struct Shared {
    pub(crate) commands: mpsc::UnboundedSender<CallCommand>,
    pub(crate) queue: Mutex<Queue>,
}

impl Shared {
    pub(crate) fn wake(&self) {
        let _ = self.commands.send(CallCommand::Wake);
    }
}

pub(crate) struct Slot {
    pub(crate) track: Arc<Track>,
    pub(crate) input: Option<Input>,
}

#[derive(Default)]
pub(crate) struct Queue {
    pub(crate) current: Option<Slot>,
    pub(crate) waiting: VecDeque<Slot>,

    pub(crate) closed: bool,
}

impl Queue {
    pub(crate) fn advance(&mut self) {
        self.current = self.waiting.pop_front();
    }

    pub(crate) fn remove(&mut self, id: u64, reason: TrackEnd) -> bool {
        if self
            .current
            .as_ref()
            .is_some_and(|slot| slot.track.id == id)
        {
            if let Some(slot) = self.current.take() {
                slot.track.end(reason);
            }
            self.advance();
            return true;
        }
        if let Some(index) = self.waiting.iter().position(|slot| slot.track.id == id) {
            if let Some(slot) = self.waiting.remove(index) {
                slot.track.end(reason);
            }
            return true;
        }
        false
    }

    pub(crate) fn close(&mut self) {
        self.closed = true;
        for slot in self
            .current
            .take()
            .into_iter()
            .chain(self.waiting.drain(..))
        {
            slot.track.end(TrackEnd::CallEnded);
        }
    }
}

#[derive(Clone)]
pub struct TrackQueue {
    pub(crate) shared: Arc<Shared>,
}

impl fmt::Debug for TrackQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TrackQueue")
            .field("len", &self.len())
            .finish()
    }
}

impl TrackQueue {
    #[allow(
        clippy::must_use_candidate,
        reason = "skipping without looking at the result is fine"
    )]
    pub fn skip(&self) -> bool {
        let mut queue = lock(&self.shared.queue);
        let Some(id) = queue.current.as_ref().map(|slot| slot.track.id) else {
            return false;
        };
        queue.remove(id, TrackEnd::Skipped);
        drop(queue);
        self.shared.wake();
        true
    }

    pub fn clear(&self) {
        let waiting: Vec<Slot> = lock(&self.shared.queue).waiting.drain(..).collect();
        for slot in waiting {
            slot.track.end(TrackEnd::Stopped);
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        let queue = lock(&self.shared.queue);
        usize::from(queue.current.is_some()) + queue.waiting.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn current(&self) -> Option<TrackHandle> {
        lock(&self.shared.queue)
            .current
            .as_ref()
            .map(|slot| TrackHandle::new(Arc::clone(&slot.track)))
    }
}
