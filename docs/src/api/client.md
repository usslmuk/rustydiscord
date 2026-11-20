# Client

The bot itself: connecting, handling events, and reaching every subsystem.

## `client`

### `HandlerMode`

How event handlers run. Sequential awaits each handler in registration order; Concurrent spawns every handler as its own task.

```rust
pub enum HandlerMode
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Concurrent` | `,` | Spawn every handler as its own task. |
| `Sequential` | `,` | Await each handler before running the next one. |

### `Shards`

Which shards this process runs. Auto asks Discord for the recommended count; Count runs 0..n; Range pins this process to a slice of the total.

```rust
pub enum Shards
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Auto` | `,` | Ask Discord how many shards the bot needs and run them all here. |
| `Count` | `u32` | Run shards 0 through n minus 1 in this process. |
| `Range` | `range: Range<u32>, total: u32,` | Run only this slice of the total, for multi-process fleets. |

### `Client`

A connected bot: gateway shards, REST, cache, collectors and the event registry behind one handle. Clone it freely; clones share everything.

```rust
pub struct Client<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `builder`

Starts a builder with a bot token. Nothing connects until login.

```rust
pub fn builder(token: impl Into<String>) -> ClientBuilder;
```

##### `on`

Runs the handler every time the event fires. Returns the client so calls chain.

```rust
pub fn on<E, F, Fut>(&self, event: E, handler: F) -> &Self where E: Event, F: Fn(Context<B>, Arc<E::Payload>) -> Fut + Send + Sync + 'static, Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,;
```

##### `command`

Handles a slash command by name, with autocomplete and option helpers on the context.

```rust
pub fn command<F, Fut>(&self, name: &str, handler: F) -> &Self where F: Fn(Context<B>, CommandInteraction) -> Fut + Send + Sync + 'static, Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,;
```

##### `autocomplete`

Answers autocomplete interactions for the named command.

```rust
pub fn autocomplete<F, Fut>(&self, name: &str, handler: F) -> &Self where F: Fn(Context<B>, AutocompleteInteraction) -> Fut + Send + Sync + 'static, Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,;
```

##### `component`

Handles button, select-menu and other component interactions by custom id.

```rust
pub fn component<F, Fut>(&self, name: &str, handler: F) -> &Self where F: Fn(Context<B>, ComponentInteraction) -> Fut + Send + Sync + 'static, Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,;
```

##### `modal`

Handles modal submissions by custom id.

```rust
pub fn modal<F, Fut>(&self, name: &str, handler: F) -> &Self where F: Fn(Context<B>, ModalInteraction) -> Fut + Send + Sync + 'static, Fut: Future<Output = Result<(), HandlerError>> + Send + 'static,;
```

##### `required_intents`

The union of intents every registered handler needs. Compare against your builder intents to catch missing ones.

```rust
pub fn required_intents(&self) -> Intents;
```

##### `events`

A stream of every raw dispatch that arrives, for logging, metrics, or driving your own dispatch.

```rust
pub fn events(&self) -> EventStream;
```

##### `rest`

The REST client this bot sends through.

```rust
pub fn rest(&self) -> &Rest;
```

##### `voice`

The voice manager. Join and leave flow through here.

```rust
pub fn voice(&self) -> &crate::voice::VoiceManager;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(&self) -> Option<&Cache<B>>;
```

##### `cluster`

The sharding cluster client, when this process runs inside a cluster.

```rust
pub fn cluster(&self) -> Option<crate::sharding::ClusterClient<B>>;
```

##### `login`

Connects every shard and runs until shutdown. Resumes dropped sessions automatically.

```rust
pub fn login(&self) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `set_presence`

Sets the presence on all running shards. Fails when nothing is connected yet.

```rust
pub fn set_presence(&self, presence: &UpdatePresence) -> Result<(), SendError>;
```

##### `shutdown`

Stops every shard and makes login return.

```rust
pub fn shutdown(&self);
```

##### `interactions_endpoint`

Builds an HTTP-interactions endpoint that verifies signatures and routes into the registered command handlers. No gateway needed.

```rust
pub fn interactions_endpoint( &self, public_key: &str, ) -> Result< crate::http_interactions::InteractionsEndpoint<B>, crate::http_interactions::InvalidPublicKey, >;
```


### `ClientBuilder`

Configures a Client: token, intents, REST, cache, shards, presence and handler mode.

```rust
pub struct ClientBuilder<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `intents`

Which gateway events Discord should send. Missing intents silently disable the events that need them.

```rust
pub const fn intents(mut self, intents: Intents) -> Self;
```

##### `rest`

Use a preconfigured REST client instead of building one from the token.

```rust
pub fn rest(mut self, rest: Rest) -> Self;
```

##### `compression`

Gateway transport compression. Defaults to the best compiled-in option.

```rust
pub const fn compression(mut self, compression: Compression) -> Self;
```

##### `gateway_url`

Override the gateway URL, for proxies and tests.

```rust
pub fn gateway_url(mut self, url: impl Into<String>) -> Self;
```

##### `shards`

Which shards this process runs. Defaults to Auto.

```rust
pub fn shards(mut self, shards: Shards) -> Self;
```

##### `cluster`

Joins a sharding cluster instead of running standalone.

```rust
pub fn cluster(mut self, assignment: crate::sharding::ClusterAssignment) -> Self;
```

##### `presence`

The presence set on every shard at login.

```rust
pub fn presence(mut self, presence: UpdatePresence) -> Self;
```

##### `handler_mode`

Sequential (default) or Concurrent handler execution.

```rust
pub const fn handler_mode(mut self, mode: HandlerMode) -> Self;
```

##### `cache`

Enables the cache with the given config. Off by default.

```rust
pub fn cache(mut self, config: CacheConfig) -> Self;
```

##### `cache_backend`

Swaps the cache storage backend and changes the client to use it.

```rust
pub fn cache_backend<C: Backend>(self, backend: C) -> ClientBuilder<C>;
```

##### `build`

Builds the client. Nothing connects until login.

```rust
pub fn build(self) -> Client<B>;
```

### `EventStream`

The raw event stream from Client::events. Each item says which shard it came from.

```rust
pub struct EventStream { /* private fields */ }
```

#### Methods

##### `next`

Waits for the next raw event, or returns None after shutdown.

```rust
pub async fn next(&mut self) -> Option<ShardEvent>;
```

## `context`


### `Context`

Everything a handler needs: REST, cache, the current shard, and helpers to build managers and collectors.

```rust
pub struct Context<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `application_id`

The bot's application id, known after the first READY.

```rust
pub fn application_id(&self) -> Option<ApplicationId>;
```

##### `rest`

The REST client.

```rust
pub fn rest(&self) -> &Rest;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(&self) -> Option<&Cache<B>>;
```

##### `shard_id`

The shard id this context runs on.

```rust
pub fn shard_id(&self) -> ShardId;
```

##### `shard`

The gateway sender for this context's shard.

```rust
pub fn shard(&self) -> &MessageSender;
```

##### `shutdown`

Stops the whole client after the current handlers finish.

```rust
pub fn shutdown(&self);
```


## `event`

### `Event`

A subscribable event: payload type, name, required intents, and which dispatch carries it.

```rust
pub trait Event: Copy + Send + Sync + 'static + sealed::Sealed;
```

#### Methods

##### `payload`

Extracts this event's payload from a dispatch, or None on mismatch.

```rust
fn payload(dispatch: &Dispatch) -> Option<&Arc<Self::Payload>>;
```

### `ShardEvent`

One dispatch tagged with the shard that produced it.

```rust
pub struct ShardEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `shard` | `ShardId` | The shard the event arrived on. |
| `event` | `Dispatch` | The dispatch itself. |

## `event.payload`

### `GuildMemberAvailable`

A guild member that finished chunking. Derefs to the member.

```rust
pub struct GuildMemberAvailable
```

| Field | Type | Description |
| --- | --- | --- |
| `member` | `Arc<CachedMember>` | The member. |

### `MessageBulkDelete`

Bulk-deleted messages: ids plus the channel they were in.

```rust
pub struct MessageBulkDelete
```

| Field | Type | Description |
| --- | --- | --- |
| `old` | `Vec<Arc<CachedMessage>>` | The old. |
| `new` | `gateway::MessageDeleteBulk` | The new. |

### `ShardResume`

Marks a resumed session and how many events were replayed.

```rust
pub struct ShardResume
```

| Field | Type | Description |
| --- | --- | --- |
| `replayed` | `u64` | The replayed. |

### `ShardReconnecting`

Marks a shard that is reconnecting.

```rust
pub struct ShardReconnecting;
```

### `Invalidated`

Marks a dead session that needs a fresh identify.

```rust
pub struct Invalidated;
```

## `handler`

### `HandlerError`

Anything a handler can fail with. Logged with the event name; the client keeps running.

```rust
pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
```



## `cache_ops`


## `dispatch`


