use std::sync::Arc;

use std::collections::HashMap;

use std::sync::RwLock;

use core::hash::Hash;

use crate::{Backend, Limit, Resource, Store};

#[derive(Clone, Debug)]
pub struct RedisConfig {
    pub url: String,

    pub prefix: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            url: "redis://127.0.0.1/".to_string(),
            prefix: "discord:".to_string(),
        }
    }
}

#[derive(Debug, Default)]
pub struct RedisBackend {
    config: RedisConfig,
}

impl RedisBackend {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: RedisConfig {
                url: String::new(),
                prefix: String::new(),
            },
        }
    }

    #[must_use]
    pub fn with_config(config: RedisConfig) -> Self {
        Self { config }
    }
}

impl Backend for RedisBackend {
    type Store<K, V>
        = RedisStore<K, V>
    where
        K: Eq + Hash + Copy + Send + Sync + 'static,
        V: Send + Sync + 'static;

    fn store<K, V>(&self, _resource: Resource, _limit: Limit) -> RedisStore<K, V>
    where
        K: Eq + Hash + Copy + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        RedisStore::new(self.config.clone())
    }
}

#[derive(Debug)]
pub struct RedisStore<K, V> {
    local: RwLock<HashMap<K, Arc<V>>>,

    _config: RedisConfig,
}

impl<K, V> RedisStore<K, V> {
    fn new(config: RedisConfig) -> Self {
        Self {
            local: RwLock::new(HashMap::new()),
            _config: config,
        }
    }
}

impl<K, V> Store<K, V> for RedisStore<K, V>
where
    K: Eq + Hash + Copy + Send + Sync,
    V: Send + Sync,
{
    fn get(&self, key: &K) -> Option<Arc<V>> {
        self.local.read().ok()?.get(key).cloned()
    }

    fn insert(&self, key: K, value: Arc<V>) -> Option<Arc<V>> {
        self.local.write().ok()?.insert(key, value)
    }

    fn remove(&self, key: &K) -> Option<Arc<V>> {
        self.local.write().ok()?.remove(key)
    }

    fn len(&self) -> usize {
        self.local.read().map_or(0, |m| m.len())
    }

    fn retain(&self, keep: &mut dyn FnMut(&K, &V) -> bool) {
        if let Ok(mut map) = self.local.write() {
            map.retain(|k, v| keep(k, v));
        }
    }

    fn values(&self) -> Vec<Arc<V>> {
        self.local
            .read()
            .map_or_default(|m| m.values().cloned().collect())
    }

    fn keys(&self) -> Vec<K> {
        self.local
            .read()
            .map_or_default(|m| m.keys().copied().collect())
    }
}
