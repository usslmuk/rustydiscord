# Utilities

Small helpers: sharding math, equality, JSON, ranges.

### `calculate_shard_id`

Computes which shard owns a guild: `(guild_id >> 22) % shard_count`. Returns 0 when the count is 0.

```rust
pub const fn calculate_shard_id(guild_id: u64, shard_count: u32) -> u32;
```

### `lazy`

Builds a lazily-initialized static value.

```rust
pub fn lazy<T, F: FnOnce() -> T>(init: F) -> std::sync::LazyLock<T, F>;
```

### `range`

Counts from 0 up to `end`, exclusive.

```rust
pub fn range(end: usize) -> core::ops::Range<usize>;
```

### `range_stepped`

Counts from `start` to `end` in steps. A step of 0 behaves like 1.

```rust
pub fn range_stepped(start: usize, end: usize, step: usize) -> impl Iterator<Item = usize>;
```

### `Equatable`

Structural equality with a named method, so generic code can compare without spelling out `PartialEq`. Blanket-implemented for everything comparable.

```rust
pub trait Equatable;
```

#### Methods

##### `equals`

Returns whether the two values are equal.

```rust
fn equals(&self, other: &Self) -> bool;
```

### `is_equatable`

Compile-time proof that a type implements `Equatable`. Always returns true.

```rust
pub fn is_equatable<T: Equatable + ?Sized>(_: &T) -> bool;
```

### `JsonEncodable`

Blanket trait that serializes anything serializable to a JSON value, falling back to null.

```rust
pub trait JsonEncodable: serde::Serialize;
```

#### Methods

##### `to_json`

Serializes the value, or returns null when serialization fails.

```rust
fn to_json(&self) -> serde_json::Value;
```

### `is_json_encodable`

Compile-time proof that a type implements `JsonEncodable`. Always returns true.

```rust
pub fn is_json_encodable<T: JsonEncodable + ?Sized>(_: &T) -> bool;
```
