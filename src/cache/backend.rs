use core::{fmt, hash::Hash};
use std::sync::Arc;

use crate::cache::{Limit, Resource};

pub trait Store<K, V>: Send + Sync {
    fn get(&self, key: &K) -> Option<Arc<V>>;

    fn insert(&self, key: K, value: Arc<V>) -> Option<Arc<V>>;

    fn remove(&self, key: &K) -> Option<Arc<V>>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    fn retain(&self, keep: &mut dyn FnMut(&K, &V) -> bool);

    fn values(&self) -> Vec<Arc<V>>;

    fn keys(&self) -> Vec<K>;
}

pub trait Backend: fmt::Debug + Send + Sync + 'static {
    type Store<K, V>: Store<K, V>
    where
        K: Eq + Hash + Copy + Send + Sync + 'static,
        V: Send + Sync + 'static;

    fn store<K, V>(&self, resource: Resource, limit: Limit) -> Self::Store<K, V>
    where
        K: Eq + Hash + Copy + Send + Sync + 'static,
        V: Send + Sync + 'static;
}
