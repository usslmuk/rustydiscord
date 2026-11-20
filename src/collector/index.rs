use core::{
    fmt,
    hash::{BuildHasherDefault, Hasher},
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
    time::Duration,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard, PoisonError},
};

use crate::model::{
    gateway::payload::{GuildMembersChunk, InteractionCreate, MessageCreate, SoundboardSounds},
    id::{ChannelId, GuildId, InteractionId, MessageId, UserId},
    interaction::InteractionData,
};
use tokio::sync::mpsc;

use super::{EndReason, ReactionEvent};
use crate::{
    event::{Dispatch, EventKind},
    router::{ComponentInteraction, ModalInteraction},
};

#[derive(Debug)]
pub enum Signal<T> {
    Item(T),

    End(EndReason),
}

pub(crate) type Tx<T> = mpsc::UnboundedSender<Signal<T>>;

#[derive(Debug)]
pub enum Sink {
    Messages(Tx<Arc<MessageCreate>>),

    Reactions(Tx<ReactionEvent>),

    Components(Tx<ComponentInteraction>),

    Modals(Tx<ModalInteraction>),

    MemberChunks(Tx<MemberReply>),

    SoundboardSounds(Tx<Arc<SoundboardSounds>>),
}

#[derive(Debug)]
pub enum MemberReply {
    Chunk(Arc<GuildMembersChunk>),

    RateLimited(Duration),
}

const DELETIONS: [EventKind; 3] = [
    EventKind::ChannelDelete,
    EventKind::ThreadDelete,
    EventKind::GuildDelete,
];

impl Sink {
    pub(crate) const fn kinds(&self) -> &'static [EventKind] {
        match self {
            Self::Messages(_) => &[
                EventKind::MessageCreate,
                DELETIONS[0],
                DELETIONS[1],
                DELETIONS[2],
            ],
            Self::Reactions(_) => &[
                EventKind::MessageReactionAdd,
                EventKind::MessageReactionRemove,
                EventKind::MessageReactionRemoveAll,
                EventKind::MessageDelete,
                EventKind::MessageBulkDelete,
                DELETIONS[0],
                DELETIONS[1],
                DELETIONS[2],
            ],
            Self::Components(_) => &[
                EventKind::InteractionCreate,
                EventKind::MessageDelete,
                EventKind::MessageBulkDelete,
                DELETIONS[0],
                DELETIONS[1],
                DELETIONS[2],
            ],
            Self::Modals(_) => &[EventKind::InteractionCreate],
            Self::MemberChunks(_) => &[EventKind::GuildMembersChunk, EventKind::RateLimited],
            Self::SoundboardSounds(_) => &[EventKind::SoundboardSounds],
        }
    }

    fn end(&self, reason: EndReason) {
        match self {
            Self::Messages(tx) => drop(tx.send(Signal::End(reason))),
            Self::Reactions(tx) => drop(tx.send(Signal::End(reason))),
            Self::Components(tx) => drop(tx.send(Signal::End(reason))),
            Self::Modals(tx) => drop(tx.send(Signal::End(reason))),
            Self::MemberChunks(tx) => drop(tx.send(Signal::End(reason))),
            Self::SoundboardSounds(tx) => drop(tx.send(Signal::End(reason))),
        }
    }
}

fn send<T>(tx: &Tx<T>, item: T) {
    let _ = tx.send(Signal::Item(item));
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Key {
    Channel(ChannelId),

    Message(MessageId),

    Interaction(InteractionId),

    User(UserId),

    Nonce(u64),

    Sounds(GuildId),
}

#[derive(Debug)]
struct Entry {
    id: u64,

    guild: Option<GuildId>,
    channel: Option<ChannelId>,
    sink: Sink,
}

#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.write_u64(u64::from(byte));
        }
    }

    fn write_u64(&mut self, value: u64) {
        self.0 = (self.0.rotate_left(5) ^ value).wrapping_mul(0x9E37_79B9_7F4A_7C15);
    }
}

type Map = HashMap<u64, Vec<Entry>, BuildHasherDefault<IdHasher>>;

#[derive(Debug, Default)]
struct Index {
    next_id: u64,
    channels: Map,
    messages: Map,
    interactions: Map,
    users: Map,
    nonces: Map,
    sounds: Map,
}

impl Index {
    fn map(&mut self, key: Key) -> (&mut Map, u64) {
        match key {
            Key::Channel(id) => (&mut self.channels, id.get()),
            Key::Message(id) => (&mut self.messages, id.get()),
            Key::Interaction(id) => (&mut self.interactions, id.get()),
            Key::User(id) => (&mut self.users, id.get()),
            Key::Nonce(nonce) => (&mut self.nonces, nonce),
            Key::Sounds(id) => (&mut self.sounds, id.get()),
        }
    }

    fn maps(&mut self) -> [&mut Map; 6] {
        [
            &mut self.channels,
            &mut self.messages,
            &mut self.interactions,
            &mut self.users,
            &mut self.nonces,
            &mut self.sounds,
        ]
    }
}

pub(crate) struct Collectors {
    count: AtomicUsize,

    wanted: [AtomicU32; EventKind::COUNT],
    index: Mutex<Index>,
}

impl fmt::Debug for Collectors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Collectors")
            .field("count", &self.count.load(Ordering::Relaxed))
            .finish_non_exhaustive()
    }
}

impl Default for Collectors {
    fn default() -> Self {
        Self {
            count: AtomicUsize::new(0),
            wanted: core::array::from_fn(|_| AtomicU32::new(0)),
            index: Mutex::default(),
        }
    }
}

impl Collectors {
    fn lock(&self) -> MutexGuard<'_, Index> {
        self.index.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(crate) fn wants(&self, kind: EventKind) -> bool {
        self.count.load(Ordering::Acquire) != 0
            && self.wanted[kind as usize].load(Ordering::Acquire) != 0
    }

    pub(crate) fn register(
        self: &Arc<Self>,
        key: Key,
        guild: Option<GuildId>,
        channel: Option<ChannelId>,
        sink: Sink,
    ) -> Registration {
        for &kind in sink.kinds() {
            self.wanted[kind as usize].fetch_add(1, Ordering::AcqRel);
        }
        let mut index = self.lock();
        index.next_id += 1;
        let id = index.next_id;
        let (map, raw) = index.map(key);
        map.entry(raw).or_default().push(Entry {
            id,
            guild,
            channel,
            sink,
        });
        self.count.fetch_add(1, Ordering::AcqRel);
        drop(index);
        Registration {
            collectors: Arc::clone(self),
            key,
            id,
        }
    }

    pub(crate) fn nonce(&self) -> u64 {
        let mut index = self.lock();
        index.next_id += 1;
        index.next_id
    }

    fn forget(&self, entry: &Entry) {
        self.count.fetch_sub(1, Ordering::AcqRel);
        for &kind in entry.sink.kinds() {
            self.wanted[kind as usize].fetch_sub(1, Ordering::AcqRel);
        }
    }

    fn unregister(&self, key: Key, id: u64) {
        let mut index = self.lock();
        let (map, raw) = index.map(key);
        let Some(entries) = map.get_mut(&raw) else {
            return;
        };
        let Some(position) = entries.iter().position(|entry| entry.id == id) else {
            return;
        };
        let entry = entries.swap_remove(position);
        if entries.is_empty() {
            map.remove(&raw);
        }
        drop(index);
        self.forget(&entry);
    }

    fn each(&self, key: Key, mut each: impl FnMut(&Sink)) {
        let mut index = self.lock();
        let (map, raw) = index.map(key);
        if let Some(entries) = map.get(&raw) {
            for entry in entries {
                each(&entry.sink);
            }
        }
    }

    fn end_key(&self, key: Key, reason: EndReason) {
        let mut index = self.lock();
        let (map, raw) = index.map(key);
        if let Some(entries) = map.remove(&raw) {
            drop(index);
            for entry in &entries {
                entry.sink.end(reason);
                self.forget(entry);
            }
        }
    }

    fn end_where(&self, reason: EndReason, mut ends: impl FnMut(&Entry) -> bool) {
        let mut index = self.lock();
        for map in index.maps() {
            map.retain(|_, entries| {
                entries.retain(|entry| {
                    let end = ends(entry);
                    if end {
                        entry.sink.end(reason);
                        self.forget(entry);
                    }
                    !end
                });
                !entries.is_empty()
            });
        }
    }

    pub(crate) fn end_all(&self, reason: EndReason) {
        if self.count.load(Ordering::Acquire) != 0 {
            self.end_where(reason, |_| true);
        }
    }

    pub(crate) fn deliver(&self, kind: EventKind, dispatch: &Dispatch) {
        if !self.wants(kind) {
            return;
        }
        match dispatch {
            Dispatch::MessageCreate(message) => {
                self.each(Key::Channel(message.channel_id), |sink| {
                    if let Sink::Messages(tx) = sink {
                        send(tx, Arc::clone(message));
                    }
                });
            }
            Dispatch::MessageReactionAdd(reaction) => {
                self.each(Key::Message(reaction.message_id), |sink| {
                    if let Sink::Reactions(tx) = sink {
                        send(tx, ReactionEvent::Added(Arc::clone(reaction)));
                    }
                });
            }
            Dispatch::MessageReactionRemove(reaction) => {
                self.each(Key::Message(reaction.message_id), |sink| {
                    if let Sink::Reactions(tx) = sink {
                        send(tx, ReactionEvent::Removed(Arc::clone(reaction)));
                    }
                });
            }
            Dispatch::MessageReactionRemoveAll(reactions) => {
                self.each(Key::Message(reactions.message_id), |sink| {
                    if let Sink::Reactions(tx) = sink {
                        send(tx, ReactionEvent::RemovedAll(Arc::clone(reactions)));
                    }
                });
            }
            Dispatch::InteractionCreate(event) => self.deliver_interaction(event),
            Dispatch::MessageDelete(deleted) => {
                self.end_key(Key::Message(deleted.id), EndReason::MessageDelete);
            }
            Dispatch::MessageBulkDelete(deleted) => {
                for &id in &deleted.ids {
                    self.end_key(Key::Message(id), EndReason::MessageDelete);
                }
            }
            Dispatch::ChannelDelete(deleted) => {
                let id = deleted.id();
                self.end_where(EndReason::ChannelDelete, |entry| entry.channel == Some(id));
            }
            Dispatch::ThreadDelete(deleted) => {
                let id = deleted.id;
                self.end_where(EndReason::ThreadDelete, |entry| entry.channel == Some(id));
            }
            Dispatch::GuildDelete(deleted) => {
                let id = deleted.id;
                self.end_where(EndReason::GuildDelete, |entry| entry.guild == Some(id));
            }
            Dispatch::GuildMembersChunk(chunk) => {
                if let Some(nonce) = chunk.nonce.as_deref().and_then(parse_nonce) {
                    self.each(Key::Nonce(nonce), |sink| {
                        if let Sink::MemberChunks(tx) = sink {
                            send(tx, MemberReply::Chunk(Arc::clone(chunk)));
                        }
                    });
                }
            }
            Dispatch::RateLimited(limited) => {
                let nonce = limited
                    .meta
                    .as_ref()
                    .and_then(|meta| meta.nonce.as_deref())
                    .and_then(parse_nonce);
                if let Some(nonce) = nonce {
                    let retry_after =
                        Duration::try_from_secs_f64(limited.retry_after).unwrap_or_default();
                    self.each(Key::Nonce(nonce), |sink| {
                        if let Sink::MemberChunks(tx) = sink {
                            send(tx, MemberReply::RateLimited(retry_after));
                        }
                    });
                }
            }
            Dispatch::SoundboardSounds(sounds) => {
                self.each(Key::Sounds(sounds.guild_id), |sink| {
                    if let Sink::SoundboardSounds(tx) = sink {
                        send(tx, Arc::clone(sounds));
                    }
                });
            }
            _ => {}
        }
    }
}

impl Collectors {
    fn deliver_interaction(&self, event: &Arc<InteractionCreate>) {
        match &event.data {
            InteractionData::MessageComponent(_) => {
                let Some(component) = ComponentInteraction::new(event) else {
                    return;
                };
                let mut components = |sink: &Sink| {
                    if let Sink::Components(tx) = sink {
                        send(tx, component.clone());
                    }
                };
                if let Some(message) = &event.message {
                    self.each(Key::Message(message.id), &mut components);
                    if let Some(metadata) = &message.interaction_metadata {
                        self.each(Key::Interaction(metadata.id()), &mut components);
                    }
                }
                let channel = event
                    .channel_id
                    .or_else(|| event.channel.as_ref().map(|channel| channel.id));
                if let Some(channel) = channel {
                    self.each(Key::Channel(channel), &mut components);
                }
            }
            InteractionData::ModalSubmit(_) => {
                let (Some(user), Some(modal)) = (event.author(), ModalInteraction::new(event))
                else {
                    return;
                };
                self.each(Key::User(user.id), |sink| {
                    if let Sink::Modals(tx) = sink {
                        send(tx, modal.clone());
                    }
                });
            }
            _ => {}
        }
    }
}

pub(crate) const NONCE_PREFIX: &str = "rustydiscord:";

fn parse_nonce(nonce: &str) -> Option<u64> {
    nonce.strip_prefix(NONCE_PREFIX)?.parse().ok()
}

#[derive(Debug)]
pub(crate) struct Registration {
    collectors: Arc<Collectors>,
    key: Key,
    id: u64,
}

impl Drop for Registration {
    fn drop(&mut self) {
        self.collectors.unregister(self.key, self.id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonces_round_trip() {
        assert_eq!(parse_nonce(&format!("{NONCE_PREFIX}42")), Some(42));
        assert_eq!(parse_nonce("someone-else:42"), None);
        assert_eq!(parse_nonce(NONCE_PREFIX), None);
    }

    #[test]
    fn registrations_count_and_unregister() {
        let collectors = Arc::new(Collectors::default());
        assert!(!collectors.wants(EventKind::MessageCreate));
        let (tx, _rx) = mpsc::unbounded_channel();
        let registration = collectors.register(
            Key::Channel(ChannelId::new(1)),
            None,
            Some(ChannelId::new(1)),
            Sink::Messages(tx),
        );
        assert!(collectors.wants(EventKind::MessageCreate));
        assert!(collectors.wants(EventKind::ChannelDelete));
        assert!(!collectors.wants(EventKind::InteractionCreate));
        drop(registration);
        assert!(!collectors.wants(EventKind::MessageCreate));
        assert_eq!(collectors.count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn an_ended_entry_is_forgotten_once() {
        let collectors = Arc::new(Collectors::default());
        let (tx, mut rx) = mpsc::unbounded_channel();
        let registration = collectors.register(
            Key::Message(MessageId::new(7)),
            Some(GuildId::new(3)),
            Some(ChannelId::new(5)),
            Sink::Reactions(tx),
        );
        collectors.end_where(EndReason::GuildDelete, |entry| {
            entry.guild == Some(GuildId::new(3))
        });
        assert!(matches!(
            rx.try_recv(),
            Ok(Signal::End(EndReason::GuildDelete))
        ));
        assert_eq!(collectors.count.load(Ordering::Relaxed), 0);

        drop(registration);
        assert_eq!(collectors.count.load(Ordering::Relaxed), 0);
        assert!(!collectors.wants(EventKind::MessageReactionAdd));
    }
}
