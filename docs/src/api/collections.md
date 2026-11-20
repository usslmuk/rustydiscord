# Collections

The insertion-ordered map behind every cache, plus its iterators.

## `collection.map`

### `DefaultHashBuilder`

The hasher collections use unless you bring your own.

```rust
pub type DefaultHashBuilder = foldhash::fast::RandomState;
```

### `Collection`

The map behind every cache. An insertion-ordered hash map: lookups by key, iteration in insertion order.

```rust
pub struct Collection<K, V, S = DefaultHashBuilder> { /* private fields */ }
```

#### Methods

##### `new`

Creates a bus that keeps the newest `capacity` messages for slow subscribers.

```rust
pub fn new() -> Self;
```

##### `with_capacity`

Sets capacity.

```rust
pub fn with_capacity(capacity: usize) -> Self;
```

##### `with_hasher`

Sets hasher.

```rust
pub const fn with_hasher(hasher: S) -> Self;
```

##### `with_capacity_and_hasher`

Sets capacity and hasher.

```rust
pub fn with_capacity_and_hasher(capacity: usize, hasher: S) -> Self;
```

##### `len`

Returns how many there are.

```rust
pub fn len(&self) -> usize;
```

##### `is_empty`

Returns whether there is nothing here.

```rust
pub fn is_empty(&self) -> bool;
```

##### `capacity`

Returns how many entries fit before reallocating.

```rust
pub fn capacity(&self) -> usize;
```

##### `clear`

Clears the content.

```rust
pub fn clear(&mut self);
```

##### `hasher`

Returns the hasher.

```rust
pub fn hasher(&self) -> &S;
```

##### `iter`

Returns an iterator over the entries.

```rust
pub fn iter(&self) -> Iter<'_, K, V>;
```

##### `iter_mut`

Returns a mutable iterator over the entries.

```rust
pub fn iter_mut(&mut self) -> IterMut<'_, K, V>;
```

##### `keys`

Returns an iterator over the keys.

```rust
pub fn keys(&self) -> Keys<'_, K, V>;
```

##### `values`

Returns an iterator over the values.

```rust
pub fn values(&self) -> Values<'_, K, V>;
```

##### `values_mut`

Returns a mutable iterator over the values.

```rust
pub fn values_mut(&mut self) -> ValuesMut<'_, K, V>;
```

##### `into_keys`

Consumes this and returns keys.

```rust
pub fn into_keys(self) -> IntoKeys<K, V>;
```

##### `into_values`

Consumes this and returns values.

```rust
pub fn into_values(self) -> IntoValues<K, V>;
```

##### `as_index_map`

Borrows this as index map.

```rust
pub const fn as_index_map(&self) -> &IndexMap<K, V, S>;
```

##### `as_index_map_mut`

Borrows this as index map mut.

```rust
pub const fn as_index_map_mut(&mut self) -> &mut IndexMap<K, V, S>;
```

##### `into_index_map`

Consumes this and returns index map.

```rust
pub fn into_index_map(self) -> IndexMap<K, V, S>;
```

##### `first`

Returns the first value.

```rust
pub fn first(&self) -> Option<&V>;
```

##### `first_key`

Returns the first key.

```rust
pub fn first_key(&self) -> Option<&K>;
```

##### `last`

Returns the last value.

```rust
pub fn last(&self) -> Option<&V>;
```

##### `last_key`

Returns the last key.

```rust
pub fn last_key(&self) -> Option<&K>;
```

##### `at`

Returns the value at the given index.

```rust
pub fn at(&self, index: isize) -> Option<&V>;
```

##### `key_at`

Returns the key at the given index.

```rust
pub fn key_at(&self, index: isize) -> Option<&K>;
```

##### `random`

Returns a random value.

```rust
pub fn random(&self) -> Option<&V>;
```

##### `random_key`

Returns a random key.

```rust
pub fn random_key(&self) -> Option<&K>;
```

##### `random_n`

Returns several random values.

```rust
pub fn random_n(&self, count: usize) -> Vec<&V>;
```

##### `find`

Returns the first value matching the predicate.

```rust
pub fn find(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&V>;
```

##### `find_key`

Returns the first key matching the predicate.

```rust
pub fn find_key(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&K>;
```

##### `find_last`

Returns the last value matching the predicate.

```rust
pub fn find_last(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&V>;
```

##### `find_last_key`

Returns the last key matching the predicate.

```rust
pub fn find_last_key(&self, mut predicate: impl FnMut(&V, &K) -> bool) -> Option<&K>;
```

##### `map`

Applies a function to every value.

```rust
pub fn map<T>(&self, mut f: impl FnMut(&V, &K) -> T) -> Vec<T>;
```

##### `every`

Returns whether every entry matches the predicate.

```rust
pub fn every(&self, mut f: impl FnMut(&V, &K) -> bool) -> bool;
```

##### `some`

Returns whether any entry matches the predicate.

```rust
pub fn some(&self, mut f: impl FnMut(&V, &K) -> bool) -> bool;
```

##### `reduce`

Folds the entries from left to right.

```rust
pub fn reduce<A>(&self, initial: A, mut f: impl FnMut(A, &V, &K) -> A) -> A;
```

##### `reduce_right`

Folds the entries from right to left.

```rust
pub fn reduce_right<A>(&self, initial: A, mut f: impl FnMut(A, &V, &K) -> A) -> A;
```

##### `equals`

Returns whether the two values are equal.

```rust
pub fn equals<S2: BuildHasher>(&self, other: &Collection<K, V, S2>) -> bool where K: Eq + Hash, V: PartialEq,;
```

##### `reverse`

Reverses insertion order.

```rust
pub fn reverse(&mut self);
```

##### `to_reversed`

Converts this to reversed.

```rust
pub fn to_reversed(mut self) -> Self;
```

##### `sort_by`

Sorts the entries with a comparison function.

```rust
pub fn sort_by(&mut self, mut compare: impl FnMut(&V, &V, &K, &K) -> Ordering);
```

##### `to_sorted_by`

Converts this to sorted by.

```rust
pub fn to_sorted_by(mut self, compare: impl FnMut(&V, &V, &K, &K) -> Ordering) -> Self;
```

##### `sort`

Sorts the entries by key.

```rust
pub fn sort(&mut self) where V: Ord,;
```

##### `to_sorted`

Converts this to sorted.

```rust
pub fn to_sorted(mut self) -> Self where V: Ord,;
```

##### `get`

Looks up a value by key.

```rust
pub fn get<Q>(&self, key: &Q) -> Option<&V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `get_mut`

Returns mut.

```rust
pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `get_key_value`

Returns key value.

```rust
pub fn get_key_value<Q>(&self, key: &Q) -> Option<(&K, &V)> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `contains_key`

Returns whether the key is present.

```rust
pub fn contains_key<Q>(&self, key: &Q) -> bool where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `index_of`

Returns the index of the key.

```rust
pub fn index_of<Q>(&self, key: &Q) -> Option<usize> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `insert`

Stores a value, returning the previous one when present.

```rust
pub fn insert(&mut self, key: K, value: V) -> Option<V>;
```

##### `set`

Replaces the current value.

```rust
pub fn set(&mut self, key: K, value: V) -> Option<V>;
```

##### `delete`

Deletes the object through Discord.

```rust
pub fn delete<Q>(&mut self, key: &Q) -> bool where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `has`

Returns whether the value is present.

```rust
pub fn has<Q>(&self, key: &Q) -> bool where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `size`

Returns the number of entries.

```rust
pub fn size(&self) -> usize;
```

##### `each`

Runs a function for every entry.

```rust
pub fn each(&self, mut f: impl FnMut(&V, &K)) -> &Self;
```

##### `for_each`

Runs a function for every entry.

```rust
pub fn for_each(&self, f: impl FnMut(&V, &K));
```

##### `tap`

Runs a function and returns the collection.

```rust
pub fn tap(&mut self, f: impl FnOnce(&mut Self)) -> &mut Self;
```

##### `remove`

Removes the value.

```rust
pub fn remove<Q>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `remove_entry`

Removes entry.

```rust
pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `swap_remove`

Removes an entry by swapping in the last one.

```rust
pub fn swap_remove<Q>(&mut self, key: &Q) -> Option<V> where K: Borrow<Q>, Q: Hash + Eq + ?Sized,;
```

##### `ensure`

Returns the existing value or inserts the default.

```rust
pub fn ensure(&mut self, key: K, default: impl FnOnce(&K) -> V) -> &mut V;
```

##### `has_all`

Returns whether all.

```rust
pub fn has_all<'q, Q>(&self, keys: impl IntoIterator<Item = &'q Q>) -> bool where K: Borrow<Q>, Q: Hash + Eq + ?Sized + 'q,;
```

##### `has_any`

Returns whether any.

```rust
pub fn has_any<'q, Q>(&self, keys: impl IntoIterator<Item = &'q Q>) -> bool where K: Borrow<Q>, Q: Hash + Eq + ?Sized + 'q,;
```

##### `retain`

Keeps only the entries the predicate accepts.

```rust
pub fn retain(&mut self, mut keep: impl FnMut(&K, &mut V) -> bool);
```

##### `sweep`

Removes stale entries from the cache.

```rust
pub fn sweep(&mut self, mut remove: impl FnMut(&V, &K) -> bool) -> usize;
```

##### `filter`

Returns a new collection with the matching entries.

```rust
pub fn filter(mut self, keep: impl FnMut(&V, &K) -> bool) -> Self;
```

##### `partition`

Splits the entries into matching and non-matching collections.

```rust
pub fn partition(self, mut predicate: impl FnMut(&V, &K) -> bool) -> (Self, Self) where S: Clone,;
```

##### `map_values`

Transforms every value.

```rust
pub fn map_values<T>(self, mut f: impl FnMut(V, &K) -> T) -> Collection<K, T, S> where S: Clone,;
```

##### `flat_map`

Maps and flattens every value.

```rust
pub fn flat_map<K2, V2, S2>( self, mut f: impl FnMut(V, K) -> Collection<K2, V2, S2>, ) -> Collection<K2, V2, S2> where K2: Hash + Eq, S2: BuildHasher + Default,;
```

##### `concat`

Concatenates another collection into this one.

```rust
pub fn concat<S2>(mut self, others: impl IntoIterator<Item = Collection<K, V, S2>>) -> Self;
```

##### `union`

Returns the entries present in either collection.

```rust
pub fn union<S2>(mut self, other: Collection<K, V, S2>) -> Self;
```

##### `intersection`

Returns the entries present in both collections.

```rust
pub fn intersection<V2, S2: BuildHasher>(mut self, other: &Collection<K, V2, S2>) -> Self;
```

##### `difference`

Returns the entries present in one collection but not the other.

```rust
pub fn difference<V2, S2: BuildHasher>(mut self, other: &Collection<K, V2, S2>) -> Self;
```

##### `symmetric_difference`

Returns the entries present in exactly one collection.

```rust
pub fn symmetric_difference<S2: BuildHasher>(mut self, other: Collection<K, V, S2>) -> Self;
```

##### `merge`

Merges another collection into this one.

```rust
pub fn merge<O, R, S2: BuildHasher>( self, other: Collection<K, O, S2>, mut in_self: impl FnMut(V, &K) -> Option<R>, mut in_other: impl FnMut(O, &K) -> Option<R>, mut in_both: impl FnMut(V, O, &K) -> Option<R>, ) -> Collection<K, R, S> where S: Clone,;
```

##### `combine_entries`

Combines entries with a function when keys collide.

```rust
pub fn combine_entries( entries: impl IntoIterator<Item = (K, V)>, mut combine: impl FnMut(V, V, &K) -> V, ) -> Self where S: Default,;
```


## `collection.iter`

### `Iter`

Borrowing iterator over a collection's key-value pairs.

```rust
pub struct Iter<'a, K, V>(/* private fields */);
```

### `IterMut`

Mutable iterator over a collection's key-value pairs.

```rust
pub struct IterMut<'a, K, V>(/* private fields */);
```

### `Keys`

Borrowing iterator over a collection's keys.

```rust
pub struct Keys<'a, K, V>(/* private fields */);
```

### `Values`

Borrowing iterator over a collection's values.

```rust
pub struct Values<'a, K, V>(/* private fields */);
```

### `ValuesMut`

Mutable iterator over a collection's values.

```rust
pub struct ValuesMut<'a, K, V>(/* private fields */);
```
