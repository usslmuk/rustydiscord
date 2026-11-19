use core::{fmt, hash::Hash};
use std::sync::Arc;

use rustydiscord_model::{
    channel::Channel,
    id::{ChannelId, GuildId},
};

use crate::{
    Backend,
    limits::{ChannelScopedOf, Global, GlobalOf, Index, ScopedOf},
};

pub struct CacheView<'a, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    entries: Option<&'a GlobalOf<K, V, B>>,
    guild_index: Option<&'a Index<GuildId, K>>,
}

impl<'a, K, V, B> CacheView<'a, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    pub(crate) const fn new(
        entries: Option<&'a GlobalOf<K, V, B>>,
        guild_index: Option<&'a Index<GuildId, K>>,
    ) -> Self {
        Self {
            entries,
            guild_index,
        }
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.entries.is_some()
    }

    #[must_use]
    pub fn get(&self, id: K) -> Option<Arc<V>> {
        self.entries?.get(&id)
    }

    #[must_use]
    pub fn contains(&self, id: K) -> bool {
        self.entries.is_some_and(|entries| entries.contains(&id))
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries.map_or(0, Global::len)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn keys(&self) -> Vec<K> {
        self.entries.map_or_default(Global::keys)
    }

    #[must_use]
    pub fn values(&self) -> Vec<Arc<V>> {
        self.entries.map_or_default(Global::values)
    }
}

impl<B: Backend> CacheView<'_, ChannelId, Channel, B> {
    #[must_use]
    pub fn in_guild(&self, guild_id: GuildId) -> Vec<ChannelId> {
        self.guild_index
            .map_or_default(|index| index.keys_of(guild_id))
    }
}

impl<K, V, B> fmt::Debug for CacheView<'_, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CacheView")
            .field("enabled", &self.is_enabled())
            .field("len", &self.len())
            .finish()
    }
}

pub struct GuildView<'a, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    entries: Option<&'a ScopedOf<K, V, B>>,
    guild_id: GuildId,
}

impl<'a, K, V, B> GuildView<'a, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    pub(crate) const fn new(entries: Option<&'a ScopedOf<K, V, B>>, guild_id: GuildId) -> Self {
        Self { entries, guild_id }
    }

    #[must_use]
    pub const fn guild_id(&self) -> GuildId {
        self.guild_id
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.entries.is_some()
    }

    #[must_use]
    pub fn get(&self, id: K) -> Option<Arc<V>> {
        self.entries?.get(self.guild_id, id)
    }

    #[must_use]
    pub fn contains(&self, id: K) -> bool {
        self.get(id).is_some()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries
            .map_or(0, |entries| entries.len_of(self.guild_id))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn keys(&self) -> Vec<K> {
        self.entries
            .map_or_default(|entries| entries.keys_of(self.guild_id))
    }

    #[must_use]
    pub fn values(&self) -> Vec<Arc<V>> {
        self.entries
            .map_or_default(|entries| entries.values_of(self.guild_id))
    }
}

impl<K, V, B> fmt::Debug for GuildView<'_, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GuildView")
            .field("guild_id", &self.guild_id)
            .field("enabled", &self.is_enabled())
            .field("len", &self.len())
            .finish()
    }
}

pub struct ChannelView<'a, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    entries: Option<&'a ChannelScopedOf<K, V, B>>,
    channel_id: ChannelId,
}

impl<'a, K, V, B> ChannelView<'a, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    pub(crate) const fn new(
        entries: Option<&'a ChannelScopedOf<K, V, B>>,
        channel_id: ChannelId,
    ) -> Self {
        Self {
            entries,
            channel_id,
        }
    }

    #[must_use]
    pub const fn channel_id(&self) -> ChannelId {
        self.channel_id
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.entries.is_some()
    }

    #[must_use]
    pub fn get(&self, id: K) -> Option<Arc<V>> {
        self.entries?.get(self.channel_id, id)
    }

    #[must_use]
    pub fn contains(&self, id: K) -> bool {
        self.get(id).is_some()
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.entries
            .map_or(0, |entries| entries.len_of(self.channel_id))
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[must_use]
    pub fn keys(&self) -> Vec<K> {
        self.entries
            .map_or_default(|entries| entries.keys_of(self.channel_id))
    }

    #[must_use]
    pub fn values(&self) -> Vec<Arc<V>> {
        self.entries
            .map_or_default(|entries| entries.values_of(self.channel_id))
    }
}

impl<K, V, B> fmt::Debug for ChannelView<'_, K, V, B>
where
    K: Eq + Hash + Copy + Send + Sync + 'static,
    V: Send + Sync + 'static,
    B: Backend,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ChannelView")
            .field("channel_id", &self.channel_id)
            .field("enabled", &self.is_enabled())
            .field("len", &self.len())
            .finish()
    }
}
