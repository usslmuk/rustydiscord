# Context and dispatch

The context a handler receives, the event trait and shard tracking.

## `context`


### `Context`

```rust
pub struct Context<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `application_id`

Returns the bot's application ID, when known.

```rust
pub fn application_id(&self) -> Option<ApplicationId>;
```

##### `rest`

Returns the REST client shared by every handler.

```rust
pub fn rest(&self) -> &Rest;
```

##### `cache`

Returns the cache, when one is configured.

```rust
pub fn cache(&self) -> Option<&Cache<B>>;
```

##### `shard_id`

Returns the shard that received the event.

```rust
pub fn shard_id(&self) -> ShardId;
```

##### `shard`

Returns the sender for the shard that received the event.

```rust
pub fn shard(&self) -> &MessageSender;
```

##### `shutdown`

Shuts down cleanly.

```rust
pub fn shutdown(&self);
```


## `event`

### `Event`

```rust
pub trait Event: Copy + Send + Sync + 'static + sealed::Sealed;
```

#### Methods

##### `payload`

Returns the typed payload for this event, when the dispatch matches.

```rust
fn payload(dispatch: &Dispatch) -> Option<&Arc<Self::Payload>>;
```

### `ShardEvent`

```rust
pub struct ShardEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `shard` | `ShardId` | Shard information. |
| `event` | `Dispatch` | The event. |

## `dispatch`

