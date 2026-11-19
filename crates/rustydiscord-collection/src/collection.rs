use core::{
    borrow::Borrow,
    cmp::Ordering,
    fmt,
    hash::{BuildHasher, Hash},
    ops::Index,
};

use indexmap::IndexMap;

use crate::{
    iter::{IntoIter, IntoKeys, IntoValues, Iter, IterMut, Keys, Values, ValuesMut},
    random,
};

pub type DefaultHashBuilder = foldhash::fast::RandomState;

#[derive(Clone)]
pub struct Collection<K, V, S = DefaultHashBuilder> {
    map: IndexMap<K, V, S>,
}

impl<K, V> Collection<K, V> {
    #[must_use]
    pub fn new() -> Self {
        Self::with_hasher(DefaultHashBuilder::default())
    }

    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, DefaultHashBuilder::default())
    }
}

impl<K, V, S> Collection<K, V, S> {
    #[must_use]
    pub const fn with_hasher(hasher: S) -> Self {
        Self {
            map: IndexMap::with_hasher(hasher),
        }
    }

    #[must_use]
    pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self {
        Self {
            map: IndexMap::with_capacity_and_hasher(capacity, hasher),
        }
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.map.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    #[must_use]
    pub fn capacity(&self) -> usize {
        self.map.capacity()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    #[must_use]
    pub fn hasher(&self) -> &S {
        self.map.hasher()
    }

    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter(self.map.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, K, V> {
        IterMut(self.map.iter_mut())
    }

    pub fn keys(&self) -> Keys<'_, K, V> {
        Keys(self.map.keys())
    }

    pub fn values(&self) -> Values<'_, K, V> {
        Values(self.map.values())
    }

    pub fn values_mut(&mut self) -> ValuesMut<'_, K, V> {
        ValuesMut(self.map.values_mut())
    }

    pub fn into_keys(self) -> IntoKeys<K, V> {
        IntoKeys(self.map.into_keys())
    }

    pub fn into_values(self) -> IntoValues<K, V> {
        IntoValues(self.map.into_values())
    }

    #[must_use]
    pub const fn as_index_map(&self) -> &IndexMap<K, V, S> {
        &self.map
    }

    pub const fn as_index_map_mut(&mut self) -> &mut IndexMap<K, V, S> {
        &mut self.map
    }

    #[must_use]
    pub fn into_index_map(self) -> IndexMap<K, V, S> {
        self.map
    }

    #[must_use]
    pub fn first(&self) -> Option<&V> {
        self.map.first().map(|(_, value)| value)
    }

    #[must_use]
    pub fn first_key(&self) -> Option<&K> {
        self.map.first().map(|(key, _)| key)
    }

    #[must_use]
    pub fn last(&self) -> Option<&V> {
        self.map.last().map(|(_, value)| value)
    }

    #[must_use]
    pub fn last_key(&self) -> Option<&K> {
        self.map.last().map(|(key, _)| key)
    }

    fn position(&self, index: isize) -> Option<usize> {
        if index >= 0 {
            usize::try_from(index).ok()
        } else {
            self.len().checked_sub(index.unsigned_abs())
        }
    }

    #[must_use]
    pub fn at(&self, index: isize) -> Option<&V> {
        let position = self.position(index)?;
        self.map.get_index(position).map(|(_, value)| value)
    }

    #[must_use]
    pub fn key_at(&self, index: isize) -> Option<&K> {
        let position = self.position(index)?;
        self.map.get_index(position).map(|(key, _)| key)
    }

    #[must_use]
    pub fn random(&self) -> Option<&V> {
        self.random_entry().map(|(_, value)| value)
    }

    #[must_use]
    pub fn random_key(&self) -> Option<&K> {
        self.random_entry().map(|(key, _)| key)
    }

    fn random_entry(&self) -> Option<(&K, &V)> {
        if self.is_empty() {
            return None;
        }
        self.map.get_index(random::Rng::new().below(self.len()))
    }

    #[must_use]
    pub fn random_n(&self, count: usize) -> Vec<&V> {
        let count = count.min(self.len());
        random::distinct(self.len(), count)
            .into_iter()
            .filter_map(|index| self.map.get_index(index).map(|(_, value)| value))
            .collect()
    }

    pub fn find(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&V> {
        self.iter()
            .find(|&(key, value)| predicate(value, key))
            .map(|(_, value)| value)
    }

    pub fn find_key(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&K> {
        self.iter()
            .find(|&(key, value)| predicate(value, key))
            .map(|(key, _)| key)
    }

    pub fn find_last(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&V> {
        self.iter()
            .rev()
            .find(|&(key, value)| predicate(value, key))
            .map(|(_, value)| value)
    }

    pub fn find_last_key(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&K> {
        self.iter()
            .rev()
            .find(|&(key, value)| predicate(value, key))
            .map(|(key, _)| key)
    }

    pub fn map<T>(&self, mut f: impl FnMut(&V, &K) -> T) -> Vec<T> {
        self.iter().map(|(key, value)| f(value, key)).collect()
    }

    #[must_use]
    pub fn every(&self, mut f: impl FnMut(&V, &K) -> bool) -> bool {
        self.iter().all(|(key, value)| f(value, key))
    }

    #[must_use]
    pub fn some(&self, mut f: impl FnMut(&V, &K) -> bool) -> bool {
        self.iter().any(|(key, value)| f(value, key))
    }

    pub fn reduce<A>(&self, initial: A, mut f: impl FnMut(A, &V, &K) -> A) -> A {
        self.iter()
            .fold(initial, |acc, (key, value)| f(acc, value, key))
    }

    pub fn reduce_right<A>(&self, initial: A, mut f: impl FnMut(A, &V, &K) -> A) -> A {
        self.iter()
            .rev()
            .fold(initial, |acc, (key, value)| f(acc, value, key))
    }

    #[must_use]
    pub fn equals<S2: BuildHasher>(&self, other: &Collection<K, V, S2>) -> bool
    where
        K: Eq + Hash,
        V: PartialEq,
    {
        self.len() == other.len()
            && self
                .iter()
                .all(|(key, value)| other.get(key).is_some_and(|v| v == value))
    }

    pub fn reverse(&mut self) {
        self.map.reverse();
    }

    #[must_use]
    pub fn to_reversed(mut self) -> Self {
        self.reverse();
        self
    }

    pub fn sort_by(&mut self, mut compare: impl FnMut(&V, &V, &K, &K) -> Ordering) {
        self.map
            .sort_by(|key_a, value_a, key_b, value_b| compare(value_a, value_b, key_a, key_b));
    }

    #[must_use]
    pub fn to_sorted_by(mut self, compare: impl FnMut(&V, &V, &K, &K) -> Ordering) -> Self {
        self.sort_by(compare);
        self
    }

    pub fn sort(&mut self)
    where
        V: Ord,
    {
        self.map.sort_by(|_, a, _, b| a.cmp(b));
    }

    #[must_use]
    pub fn to_sorted(mut self) -> Self
    where
        V: Ord,
    {
        self.sort();
        self
    }
}

impl<K, V, S> Collection<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    #[must_use]
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get(key)
    }

    #[must_use]
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_mut(key)
    }

    #[must_use]
    pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_key_value(key)
    }

    #[must_use]
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.contains_key(key)
    }

    #[must_use]
    pub fn index_of<Q>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.get_index_of(key)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        self.map.insert(key, value)
    }

    pub fn set(&mut self, key: K, value: V) -> Option<V> {
        self.insert(key, value)
    }

    pub fn delete<Q>(&mut self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.remove(key).is_some()
    }

    #[must_use]
    pub fn has<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.contains_key(key)
    }

    #[must_use]
    pub fn size(&self) -> usize {
        self.len()
    }

    pub fn each(&self, mut f: impl FnMut(&V, &K)) -> &Self {
        for (key, value) in &self.map {
            f(value, key);
        }
        self
    }

    pub fn for_each(&self, f: impl FnMut(&V, &K)) {
        let _ = self.each(f);
    }

    pub fn tap(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self {
        f(self);
        self
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.shift_remove(key)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.shift_remove_entry(key)
    }

    pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        self.map.swap_remove(key)
    }

    pub fn ensure(&mut self, key: K, default: impl FnOnce(&K) -> V) -> &mut V {
        self.map.entry(key).or_insert_with_key(default)
    }

    pub fn has_all<'q, Q>(&self, keys: impl IntoIterator<Item = &'q Q>) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized + 'q,
    {
        keys.into_iter().all(|key| self.contains_key(key))
    }

    pub fn has_any<'q, Q>(&self, keys: impl IntoIterator<Item = &'q Q>) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized + 'q,
    {
        keys.into_iter().any(|key| self.contains_key(key))
    }

    pub fn retain(&mut self, mut keep: impl FnMut(&K, &mut V) -> bool) {
        self.map.retain(|key, value| keep(key, value));
    }

    pub fn sweep(&mut self, mut remove: impl FnMut(&V, &K) -> bool) -> usize {
        let before = self.len();
        self.map.retain(|key, value| !remove(value, key));
        before - self.len()
    }

    #[must_use]
    pub fn filter(mut self, keep: impl FnMut(&V, &K) -> bool) -> Self {
        self.sweep(not(keep));
        self
    }

    #[must_use]
    pub fn partition(self, mut predicate: impl FnMut(&V, &K) -> bool) -> (Self, Self)
    where
        S: Clone,
    {
        let mut failed = self.map;
        let mut passed = IndexMap::with_hasher(failed.hasher().clone());
        passed.extend(failed.extract_if(.., |key, value| predicate(value, key)));
        (Self { map: passed }, Self { map: failed })
    }

    #[must_use]
    pub fn map_values<T>(self, mut f: impl FnMut(V, &K) -> T) -> Collection<K, T, S>
    where
        S: Clone,
    {
        let mut mapped =
            Collection::with_capacity_and_hasher(self.len(), self.map.hasher().clone());
        for (key, value) in self.map {
            let value = f(value, &key);
            mapped.map.insert(key, value);
        }
        mapped
    }

    #[must_use]
    pub fn flat_map<K2, V2, S2>(
        self,
        mut f: impl FnMut(V, K) -> Collection<K2, V2, S2>,
    ) -> Collection<K2, V2, S2>
    where
        K2: Hash + Eq,
        S2: BuildHasher + Default,
    {
        let mut joined = Collection::with_hasher(S2::default());
        for (key, value) in self.map {
            joined.map.extend(f(value, key).map);
        }
        joined
    }

    #[must_use]
    pub fn concat<S2>(mut self, others: impl IntoIterator<Item = Collection<K, V, S2>>) -> Self {
        for other in others {
            self.map.extend(other.map);
        }
        self
    }

    #[must_use]
    pub fn union<S2>(mut self, other: Collection<K, V, S2>) -> Self {
        for (key, value) in other.map {
            self.map.entry(key).or_insert(value);
        }
        self
    }

    #[must_use]
    pub fn intersection<V2, S2: BuildHasher>(mut self, other: &Collection<K, V2, S2>) -> Self {
        self.map.retain(|key, _| other.contains_key(key));
        self
    }

    #[must_use]
    pub fn difference<V2, S2: BuildHasher>(mut self, other: &Collection<K, V2, S2>) -> Self {
        self.map.retain(|key, _| !other.contains_key(key));
        self
    }

    #[must_use]
    pub fn symmetric_difference<S2: BuildHasher>(mut self, other: Collection<K, V, S2>) -> Self {
        let mut other = other.map;
        let mut shared = vec![false; other.len()];
        self.map.retain(|key, _| match other.get_index_of(key) {
            Some(index) => {
                shared[index] = true;
                false
            }
            None => true,
        });
        let mut index = 0;
        other.retain(|_, _| {
            let keep = !shared[index];
            index += 1;
            keep
        });
        self.map.extend(other);
        self
    }

    #[must_use]
    pub fn merge<O, R, S2: BuildHasher>(
        self,
        other: Collection<K, O, S2>,
        mut in_self: impl FnMut(V, &K) -> Option<R>,
        mut in_other: impl FnMut(O, &K) -> Option<R>,
        mut in_both: impl FnMut(V, O, &K) -> Option<R>,
    ) -> Collection<K, R, S>
    where
        S: Clone,
    {
        let matches: Vec<Option<usize>> = self
            .map
            .keys()
            .map(|key| other.map.get_index_of(key))
            .collect();
        let mut others: Vec<Option<(K, O)>> = other.map.into_iter().map(Some).collect();
        let mut merged = Collection::with_capacity_and_hasher(
            self.map.len().max(others.len()),
            self.map.hasher().clone(),
        );
        for ((key, value), matched) in self.map.into_iter().zip(matches) {
            let result = match matched.and_then(|index| others[index].take()) {
                Some((_, other_value)) => in_both(value, other_value, &key),
                None => in_self(value, &key),
            };
            if let Some(result) = result {
                merged.map.insert(key, result);
            }
        }
        for (key, value) in others.into_iter().flatten() {
            if let Some(result) = in_other(value, &key) {
                merged.map.insert(key, result);
            }
        }
        merged
    }

    #[must_use]
    pub fn combine_entries(
        entries: impl IntoIterator<Item = (K, V)>,
        mut combine: impl FnMut(V, V, &K) -> V,
    ) -> Self
    where
        S: Default,
    {
        let mut combined: IndexMap<K, Option<V>, S> = IndexMap::default();
        for (key, value) in entries {
            match combined.entry(key) {
                indexmap::map::Entry::Occupied(mut entry) => {
                    let earlier = entry.get_mut().take();
                    let value = match earlier {
                        Some(earlier) => combine(earlier, value, entry.key()),
                        None => value,
                    };
                    *entry.get_mut() = Some(value);
                }
                indexmap::map::Entry::Vacant(entry) => {
                    entry.insert(Some(value));
                }
            }
        }
        let mut collection = Self::with_capacity_and_hasher(combined.len(), S::default());
        collection.map.extend(
            combined
                .into_iter()
                .filter_map(|(key, value)| Some((key, value?))),
        );
        collection
    }
}

fn not<V, K>(mut predicate: impl FnMut(&V, &K) -> bool) -> impl FnMut(&V, &K) -> bool {
    move |value, key| !predicate(value, key)
}

impl<K, V, S: Default> Default for Collection<K, V, S> {
    fn default() -> Self {
        Self::with_hasher(S::default())
    }
}

impl<K: fmt::Debug, V: fmt::Debug, S> fmt::Debug for Collection<K, V, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.iter()).finish()
    }
}

impl<K, V1, S1, V2, S2> PartialEq<Collection<K, V2, S2>> for Collection<K, V1, S1>
where
    K: Hash + Eq,
    V1: PartialEq<V2>,
    S1: BuildHasher,
    S2: BuildHasher,
{
    fn eq(&self, other: &Collection<K, V2, S2>) -> bool {
        self.map == other.map
    }
}

impl<K: Hash + Eq, V: Eq, S: BuildHasher> Eq for Collection<K, V, S> {}

impl<K, V, Q, S> Index<&Q> for Collection<K, V, S>
where
    K: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
    S: BuildHasher,
{
    type Output = V;

    fn index(&self, key: &Q) -> &V {
        self.get(key)
            .expect("the collection has no entry for this key")
    }
}

impl<K: Hash + Eq, V, S: BuildHasher + Default> FromIterator<(K, V)> for Collection<K, V, S> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(entries: I) -> Self {
        Self {
            map: IndexMap::from_iter(entries),
        }
    }
}

impl<K: Hash + Eq, V, S: BuildHasher> Extend<(K, V)> for Collection<K, V, S> {
    fn extend<I: IntoIterator<Item = (K, V)>>(&mut self, entries: I) {
        self.map.extend(entries);
    }
}

impl<K: Hash + Eq, V, const N: usize> From<[(K, V); N]> for Collection<K, V> {
    fn from(entries: [(K, V); N]) -> Self {
        entries.into_iter().collect()
    }
}

impl<K, V, S> From<IndexMap<K, V, S>> for Collection<K, V, S> {
    fn from(map: IndexMap<K, V, S>) -> Self {
        Self { map }
    }
}

impl<K, V, S> From<Collection<K, V, S>> for IndexMap<K, V, S> {
    fn from(collection: Collection<K, V, S>) -> Self {
        collection.map
    }
}

impl<K, V, S> IntoIterator for Collection<K, V, S> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self.map.into_iter())
    }
}

impl<'a, K, V, S> IntoIterator for &'a Collection<K, V, S> {
    type Item = (&'a K, &'a V);
    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, K, V, S> IntoIterator for &'a mut Collection<K, V, S> {
    type Item = (&'a K, &'a mut V);
    type IntoIter = IterMut<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
