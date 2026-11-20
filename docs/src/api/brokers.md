# Message brokers

A typed in-process bus for fanning events out to background tasks.

### `Broker`

A typed in-process message bus. Publishers never wait; subscribers get a Tokio broadcast receiver each.

```rust
pub trait Broker;
```

#### Methods

##### `publish`

Sends one message to every current subscriber. Returns false when nobody is listening, so fire-and-forget loops can count drops.

```rust
fn publish(&self, message: Self::Message) -> bool;
```

##### `subscribe`

Opens a new lagging-buffered subscription to the bus.

```rust
fn subscribe(&self) -> broadcast::Receiver<Self::Message>;
```

### `MemoryBroker`

The bundled broker, backed by a Tokio broadcast channel with a fixed-size buffer.

```rust
pub struct MemoryBroker<T> where T: Clone + Send + Sync + 'static, { /* private fields */ }
```

#### Methods

##### `new`

Creates a bus that keeps the newest `capacity` messages for slow subscribers.

```rust
pub fn new(capacity: usize) -> Self;
```
