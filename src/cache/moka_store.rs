use std::sync::Arc;

use std::collections::HashMap;

use std::sync::RwLock;

use core::hash::Hash;

use crate::cache::{Backend, Limit, Resource, Store};

#[derive(Debug, Default)]
pub struct MokaBackend {
    max_capacity: u64,
}

impl MokaBackend {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_capacity: 10_000,
        }
    }

    #[must_use]
    pub const fn with_max_capacity(max_capacity: u64) -> Self {
        Self { max_capacity }
    }
}

impl Backend for MokaBackend {
    type Store<K, V>
        = MokaStore<K, V>
    where
        K: Eq + Hash + Copy + Send + Sync + 'static,
        V: Send + Sync + 'static;

    fn store<K, V>(&self, _resource: Resource, _limit: Limit) -> MokaStore<K, V>
    where
        K: Eq + Hash + Copy + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        MokaStore::new(self.max_capacity)
    }
}

#[derive(Debug)]
pub struct MokaStore<K, V> {
    map: RwLock<HashMap<K, Arc<V>>>,

    max_capacity: u64,
}

impl<K, V> MokaStore<K, V> {
    fn new(max_capacity: u64) -> Self {
        Self {
            map: RwLock::new(HashMap::new()),
            max_capacity,
        }
    }
}

impl<K, V> Store<K, V> for MokaStore<K, V>
where
    K: Eq + Hash + Copy + Send + Sync,
    V: Send + Sync,
{
    fn get(&self, key: &K) -> Option<Arc<V>> {
        self.map.read().ok()?.get(key).cloned()
    }

    fn insert(&self, key: K, value: Arc<V>) -> Option<Arc<V>> {
        let mut map = self.map.write().ok()?;

        if map.len() as u64 >= self.max_capacity
            && let Some(first) = map.keys().next().copied()
        {
            map.remove(&first);
        }

        map.insert(key, value)
    }

    fn remove(&self, key: &K) -> Option<Arc<V>> {
        self.map.write().ok()?.remove(key)
    }

    fn len(&self) -> usize {
        self.map.read().map_or(0, |m| m.len())
    }

    fn retain(&self, keep: &mut dyn FnMut(&K, &V) -> bool) {
        if let Ok(mut map) = self.map.write() {
            map.retain(|k, v| keep(k, v));
        }
    }

    fn values(&self) -> Vec<Arc<V>> {
        self.map
            .read()
            .map_or_default(|m| m.values().cloned().collect())
    }

    fn keys(&self) -> Vec<K> {
        self.map
            .read()
            .map_or_default(|m| m.keys().copied().collect())
    }
}
