use rustydiscord_model::id::GuildId;

use crate::{Call, Input, TrackHandle};

#[derive(Clone, Debug, Default)]
pub struct Group {
    calls: Vec<Call>,
}

impl Group {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn add(&mut self, call: Call) -> &mut Self {
        if !self.calls.iter().any(|c| c.guild_id() == call.guild_id()) {
            self.calls.push(call);
        }
        self
    }

    #[must_use]
    pub fn remove(&mut self, guild_id: GuildId) -> bool {
        let len = self.calls.len();
        self.calls.retain(|call| call.guild_id() != guild_id);
        self.calls.len() != len
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.calls.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.calls.is_empty()
    }

    #[must_use]
    pub fn calls(&self) -> Vec<Call> {
        self.calls.clone()
    }

    #[must_use]
    pub fn play(&self, make_input: impl Fn() -> Input) -> Vec<TrackHandle> {
        self.calls
            .iter()
            .map(|call| call.play(make_input()))
            .collect()
    }

    #[must_use]
    pub fn enqueue(&self, make_input: impl Fn() -> Input) -> Vec<TrackHandle> {
        self.calls
            .iter()
            .map(|call| call.enqueue(make_input()))
            .collect()
    }

    #[must_use]
    pub fn pause_all(&self) -> Vec<Result<(), crate::TrackError>> {
        self.calls
            .iter()
            .filter_map(|call| call.queue().current())
            .map(|track| track.pause())
            .collect()
    }

    #[must_use]
    pub fn skip_all(&self) -> usize {
        self.calls.iter().filter(|call| call.queue().skip()).count()
    }

    pub fn clear(&self) {
        for call in &self.calls {
            call.queue().clear();
        }
    }

    #[must_use]
    pub fn current(&self) -> Vec<TrackHandle> {
        self.calls
            .iter()
            .filter_map(|call| call.queue().current())
            .collect()
    }
}
