# Sharding

Run shards across processes with supervision, broadcasts and live stats.

## `sharding`

### `pub use self::remote::{ CrossHostEndpoint, EvalError, EvalLimits, EvalRequest, RedisIdentifyQueue, ReshardState, TlsConfig, };`

```rust
pub use self::remote::{ CrossHostEndpoint, EvalError, EvalLimits, EvalRequest, RedisIdentifyQueue, ReshardState, TlsConfig, };
```

### `pub use self::{ cluster::ClusterAssignment, cluster_client::ClusterClient, command::{ClusterCommand, ClusterReply}, manager::{ Cluster, ClusterEvent, ClusterEvents, ClusterManager, ClusterManagerBuilder, Restart, }, stats::{ClusterStats, ShardStats, ShardStatus, Stats}, };`

```rust
pub use self::{ cluster::ClusterAssignment, cluster_client::ClusterClient, command::{ClusterCommand, ClusterReply}, manager::{ Cluster, ClusterEvent, ClusterEvents, ClusterManager, ClusterManagerBuilder, Restart, }, stats::{ClusterStats, ShardStats, ShardStatus, Stats}, };
```

### `process`

```rust
pub fn process() -> Result<Option<ClusterAssignment>, ClusterError>;
```

### `ClusterError`

Possible cluster error values.

```rust
pub enum ClusterError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Rest` | `#[from] crate::rest::Error` | Rest. |

## `sharding.cluster`

### `ClusterAssignment`

Which shards this process owns, how to reach the gateway, and how fast it may identify.

```rust
pub struct ClusterAssignment { /* private fields */ }
```

#### Methods

##### `cluster_id`

Returns the ID of the cluster.

```rust
pub const fn cluster_id(&self) -> u32;
```

##### `shards`

Which shards this process runs. Defaults to Auto.

```rust
pub fn shards(&self) -> Range<u32>;
```

##### `total_shards`

Returns the total shard count.

```rust
pub const fn total_shards(&self) -> u32;
```




## `sharding.cluster_client`

### `ClusterClient`

The in-process side of a cluster worker: talk to siblings and answer the manager.

```rust
pub struct ClusterClient<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub fn id(&self) -> u32;
```

##### `shards`

Which shards this process runs. Defaults to Auto.

```rust
pub fn shards(&self) -> Range<u32>;
```

##### `total_shards`

Returns the total shard count.

```rust
pub fn total_shards(&self) -> u32;
```

##### `handle`

Verifies the Ed25519 signature and timestamp, then dispatches. Returns the HTTP response plus the background work as a future.

```rust
pub fn handle<C, F, Fut>(&self, handler: F) -> &Self where C: ClusterCommand, F: Fn(Client<B>, C::Input) -> Fut + Send + Sync + 'static, Fut: Future<Output = Result<C::Output, HandlerError>> + Send + 'static,;
```

##### `broadcast`

Runs the same code on every shard and gathers the answers.

```rust
pub async fn broadcast<C: ClusterCommand>( &self, input: C::Input, ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError>;
```

##### `broadcast_eval`

Evals JavaScript on every shard and gathers the answers.

```rust
pub async fn broadcast_eval<C: ClusterCommand>( &self, input: C::Input, ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError>;
```

##### `send`

Sends a webhook message and returns it.

```rust
pub async fn send<C: ClusterCommand>( &self, cluster: u32, input: C::Input, ) -> Result<C::Output, ClusterError>;
```

##### `stats`

Live per-shard numbers.

```rust
pub async fn stats(&self) -> Result<Vec<ClusterReply<ClusterStats>>, ClusterError>;
```


## `sharding.command`

### `ClusterCommand`

```rust
pub trait ClusterCommand: Send + Sync + 'static;
```

### `ClusterReply`

```rust
pub struct ClusterReply<T>
```

| Field | Type | Description |
| --- | --- | --- |
| `cluster` | `u32` | The cluster. |
| `result` | `Result<T, ClusterError>` | The result. |


## `sharding.ipc`


## `sharding.manager`

### `ClusterManager`

The parent side: spawns worker processes and supervises shards.

```rust
pub struct ClusterManager { /* private fields */ }
```

#### Methods

##### `builder`

Starts a builder with a bot token. Nothing connects until login.

```rust
pub fn builder(token: impl Into<String>) -> ClusterManagerBuilder;
```

##### `spawn`

Starts the configured processes and connects every shard.

```rust
pub fn spawn(&self) -> impl Future<Output = Result<(), ClusterError>> + Send + '_;
```

##### `shutdown`

Stops every shard and makes login return.

```rust
pub fn shutdown(&self);
```

##### `events`

A stream of every raw dispatch that arrives, for logging, metrics, or driving your own dispatch.

```rust
pub fn events(&self) -> ClusterEvents;
```

##### `clusters`

The running cluster workers.

```rust
pub fn clusters(&self) -> Vec<Cluster>;
```

##### `broadcast`

Runs the same code on every shard and gathers the answers.

```rust
pub async fn broadcast<C: ClusterCommand>( &self, input: C::Input, ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError>;
```

##### `broadcast_eval`

Evals JavaScript on every shard and gathers the answers.

```rust
pub async fn broadcast_eval<C: ClusterCommand>( &self, input: C::Input, ) -> Result<Vec<ClusterReply<C::Output>>, ClusterError>;
```

##### `stats`

Live per-shard numbers.

```rust
pub async fn stats(&self) -> Result<Vec<ClusterReply<super::ClusterStats>>, ClusterError>;
```

##### `send`

Sends a webhook message and returns it.

```rust
pub async fn send<C: ClusterCommand>( &self, cluster: u32, input: C::Input, ) -> Result<C::Output, ClusterError>;
```


### `ClusterManagerBuilder`

Builds cluster manager values.

```rust
pub struct ClusterManagerBuilder { /* private fields */ }
```

#### Methods

##### `rest`

The REST client this bot sends through.

```rust
pub fn rest(mut self, rest: Rest) -> Self;
```

##### `shards`

Which shards this process runs. Defaults to Auto.

```rust
pub fn shards(mut self, shards: Shards) -> Self;
```

##### `shards_per_process`

Sets how many shards each process runs.

```rust
pub fn shards_per_process(mut self, shards: u32) -> Self;
```

##### `executable`

Sets the worker executable.

```rust
pub fn executable(mut self, path: impl Into<PathBuf>) -> Self;
```

##### `args`

Sets the worker arguments.

```rust
pub fn args<A: Into<OsString>>(mut self, args: impl IntoIterator<Item = A>) -> Self;
```

##### `env`

Sets the worker environment.

```rust
pub fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self;
```

##### `shutdown_timeout`

Sets how long to wait for a graceful shutdown.

```rust
pub const fn shutdown_timeout(mut self, timeout: Duration) -> Self;
```

##### `restart`

Restarts failed or requested shards.

```rust
pub const fn restart(mut self, restart: Restart) -> Self;
```

##### `restart_delay`

Sets the delay before restarting a crashed process.

```rust
pub const fn restart_delay(mut self, first: Duration, max: Duration) -> Self;
```

##### `command_timeout`

Sets how long to wait for a remote command.

```rust
pub const fn command_timeout(mut self, timeout: Duration) -> Self;
```

##### `build`

Builds the client. Nothing connects until login.

```rust
pub fn build(self) -> ClusterManager;
```

### `Restart`

Possible restart values.

```rust
pub enum Restart
```

| Variant | Payload | Description |
| --- | --- | --- |
| `OnFailure` | `,` | On Failure. |
| `Always` | `,` | Always. |
| `Never` | `,` | Never. |

### `Cluster`

```rust
pub struct Cluster
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `u32` | The object's snowflake ID. |
| `shards` | `Range<u32>` | The shards. |
| `pid` | `Option<u32>` | The pid. |
| `connected` | `bool` | The connected. |

### `ClusterEvent`

Something happened in the cluster: a shard changed state, a worker exited, stats ticked.

```rust
pub enum ClusterEvent
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Spawned` | `cluster: u32, pid: Option<u32>` | Spawned. |
| `Connected` | `cluster: u32` | Connected. |
| `Exited` | `cluster: u32, status: ExitStatus` | Exited. |
| `Restarting` | `cluster: u32, delay: Duration` | Restarting. |

### `ClusterEvents`

```rust
pub struct ClusterEvents { /* private fields */ }
```

#### Methods

##### `next`

Waits for the next raw event, or returns None after shutdown.

```rust
pub async fn next(&mut self) -> Option<ClusterEvent>;
```

## `sharding.remote`

### `TlsConfig`

```rust
pub struct TlsConfig
```

| Field | Type | Description |
| --- | --- | --- |
| `server_name` | `String` | The server name. |

#### Methods

##### `new`

Starts a builder for the given key and channel.

```rust
pub fn new(server_name: impl Into<String>) -> Self;
```

### `CrossHostEndpoint`

```rust
pub struct CrossHostEndpoint
```

| Field | Type | Description |
| --- | --- | --- |
| `host` | `String` | The host. |
| `port` | `u16` | The port. |
| `tls` | `Option<TlsConfig>` | The tls. |

#### Methods

##### `new`

Starts a builder for the given key and channel.

```rust
pub fn new(host: impl Into<String>, port: u16) -> Self;
```

##### `with_tls`

Sets tls.

```rust
pub fn with_tls(mut self, tls: TlsConfig) -> Self;
```

##### `addr`

Returns the endpoint address.

```rust
pub fn addr(&self) -> String;
```

##### `uses_tls`

Returns whether the endpoint uses TLS.

```rust
pub fn uses_tls(&self) -> bool;
```

### `RedisIdentifyQueue`

```rust
pub struct RedisIdentifyQueue { /* private fields */ }
```

#### Methods

##### `new`

Starts a builder for the given key and channel.

```rust
pub fn new() -> Self;
```

##### `with_url`

Sets url.

```rust
pub fn with_url(url: impl Into<String>) -> Self;
```

##### `url`

Returns the URL.

```rust
pub fn url(&self) -> &str;
```

##### `interval`

Returns the interval.

```rust
pub fn interval(&self) -> Duration;
```

##### `bucket_key`

Returns the Redis bucket key.

```rust
pub fn bucket_key(shard: u32, max_concurrency: u32) -> u32;
```

### `EvalLimits`

Caps on cross-shard eval: timeout and payload size.

```rust
pub struct EvalLimits
```

| Field | Type | Description |
| --- | --- | --- |
| `max_input_bytes` | `usize` | The max input bytes. |
| `timeout` | `Duration` | The timeout. |
| `allowlist_only` | `bool` | The allowlist only. |

### `EvalRequest`

```rust
pub struct EvalRequest
```

| Field | Type | Description |
| --- | --- | --- |
| `command` | `String` | The command. |
| `input` | `Vec<u8>` | The input. |
| `limits` | `EvalLimits` | The limits. |

#### Methods

##### `new`

Starts a builder for the given key and channel.

```rust
pub fn new(command: impl Into<String>, input: Vec<u8>) -> Self;
```

##### `validate`

Validates the request.

```rust
pub fn validate(&self, allowlist: &[&str]) -> Result<(), EvalError>;
```

### `EvalError`

A broadcast_eval that failed: timed out, too large, or threw.

```rust
pub enum EvalError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `TooLarge` | `,` | Too Large. |
| `UnknownCommand` | `,` | A value this build does not recognize. |
| `NotAllowlisted` | `,` | Not Allowlisted. |
| `Timeout` | `,` | Timeout. |

### `split_range`

```rust
pub fn split_range(total: u32, parts: u32) -> Vec<std::ops::Range<u32>>;
```

### `reshard_plan`

```rust
pub fn reshard_plan(old_total: u32, new_total: u32) -> Vec<(u32, u32)>;
```

### `ReshardState`

Tracks a live reshard: which shards move where.

```rust
pub struct ReshardState
```

| Field | Type | Description |
| --- | --- | --- |
| `old_total` | `u32` | The old total. |
| `new_total` | `u32` | The new total. |
| `started` | `Option<Instant>` | The started. |

#### Methods

##### `new`

Starts a builder for the given key and channel.

```rust
pub fn new(old_total: u32, new_total: u32) -> Self;
```

##### `moves_shard`

Returns whether the reshard plan moves the shard.

```rust
pub fn moves_shard(&self, shard: u32) -> bool;
```

### `Allowlist`

```rust
pub type Allowlist = Arc<Vec<String>>;
```

## `sharding.stats`

### `Stats`

```rust
pub struct Stats;
```

### `ClusterStats`

```rust
pub struct ClusterStats
```

| Field | Type | Description |
| --- | --- | --- |
| `cluster` | `u32` | The cluster. |
| `shards` | `Vec<ShardStats>` | The shards. |
| `guilds` | `Option<u64>` | The guilds. |
| `users` | `Option<u64>` | The users. |
| `uptime` | `Duration` | The uptime. |
| `memory` | `Option<u64>` | The memory. |

### `ShardStats`

One shard's latency, state and guild count.

```rust
pub struct ShardStats
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `u32` | The object's snowflake ID. |
| `status` | `ShardStatus` | The status. |
| `latency` | `Option<Duration>` | The latency. |

### `ShardStatus`

What a shard is doing right now: connecting, ready, resumed, or dead and why.

```rust
pub enum ShardStatus
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Disconnected` | `,` | Disconnected. |
| `Identifying` | `,` | Identifying. |
| `Resuming` | `,` | Resuming. |
| `Active` | `,` | Active. |
| `Closed` | `,` | Closed. |



