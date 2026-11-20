# Gateway

The WebSocket layer: shards, identify queues, compression, metrics and gateway models.

## `gateway.compression`

### `Compression`

Possible compression values.

```rust
pub enum Compression
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | None. |
| `ZstdStream` | `,` | Zstd Stream. |
| `ZlibStream` | `,` | Zlib Stream. |


## `gateway.config`

### `DEFAULT_GATEWAY_URL`

```rust
pub const DEFAULT_GATEWAY_URL: &str = "wss://gateway.discord.gg";
```

### `Config`

```rust
pub struct Config
```

| Field | Type | Description |
| --- | --- | --- |
| `token` | `Box<str>` | The token. |
| `intents` | `Intents` | The intents. |
| `gateway_url` | `Box<str>` | The URL for the gateway. |
| `compression` | `Compression` | The compression. |
| `encoding` | `Encoding` | The encoding. |
| `properties` | `IdentifyProperties` | The properties. |
| `large_threshold` | `Option<u8>` | The large threshold. |
| `connect_timeout` | `Duration` | The connect timeout. |
| `hello_timeout` | `Duration` | The hello timeout. |
| `tls` | `Arc<Connector>` | The tls. |
| `identify_queue` | `Arc<dyn IdentifyQueue>` | The identify queue. |
| `presence` | `Option<UpdatePresence>` | The member's presence. |

#### Methods

##### `new`

Creates a new `Config`.

```rust
pub fn new(token: impl Into<String>, intents: Intents) -> Self;
```

##### `builder`

Returns a builder for the related request or configuration.

```rust
pub fn builder(token: impl Into<String>, intents: Intents) -> ConfigBuilder;
```

##### `intents`

Sets the gateway intents.

```rust
pub const fn intents(&self) -> Intents;
```

##### `gateway_url`

Returns the gateway URL.

```rust
pub fn gateway_url(&self) -> &str;
```

##### `compression`

Sets the gateway transport compression.

```rust
pub const fn compression(&self) -> Compression;
```

##### `encoding`

Sets the gateway payload encoding.

```rust
pub const fn encoding(&self) -> Encoding;
```

##### `presence`

Sets the presence broadcast at login.

```rust
pub const fn presence(&self) -> Option<&UpdatePresence>;
```

##### `identify_queue`

Sets the queue that serializes identifies.

```rust
pub fn identify_queue(&self) -> &Arc<dyn IdentifyQueue>;
```

### `ConfigBuilder`

Builds config values.

```rust
pub struct ConfigBuilder { /* private fields */ }
```

#### Methods

##### `gateway_url`

Returns the gateway URL.

```rust
pub fn gateway_url(mut self, url: impl Into<String>) -> Self;
```

##### `compression`

Sets the gateway transport compression.

```rust
pub const fn compression(mut self, compression: Compression) -> Self;
```

##### `encoding`

Sets the gateway payload encoding.

```rust
pub const fn encoding(mut self, encoding: Encoding) -> Self;
```

##### `identify_properties`

Sets the client properties sent with identify.

```rust
pub fn identify_properties(mut self, properties: IdentifyProperties) -> Self;
```

##### `large_threshold`

Sets the member-count threshold for large guilds.

```rust
pub const fn large_threshold(mut self, threshold: u8) -> Self;
```

##### `connect_timeout`

Sets how long to wait for a connection.

```rust
pub const fn connect_timeout(mut self, timeout: Duration) -> Self;
```

##### `hello_timeout`

Sets how long to wait for the hello frame.

```rust
pub const fn hello_timeout(mut self, timeout: Duration) -> Self;
```

##### `presence`

Sets the presence broadcast at login.

```rust
pub fn presence(mut self, presence: UpdatePresence) -> Self;
```

##### `identify_queue`

Sets the queue that serializes identifies.

```rust
pub fn identify_queue(mut self, queue: Arc<dyn IdentifyQueue>) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Config;
```

## `gateway.encoding`

### `Encoding`

Possible encoding values.

```rust
pub enum Encoding
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Json` | `,` | Json. |
| `Etf` | `,` | Etf. |

## `gateway.error`

### `ReceiveError`

```rust
pub struct ReceiveError { /* private fields */ }
```

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> &ReceiveErrorKind;
```

##### `is_fatal`

Returns whether fatal.

```rust
pub const fn is_fatal(&self) -> bool;
```

##### `into_parts`

Consumes this and returns parts.

```rust
pub fn into_parts( self, ) -> ( ReceiveErrorKind, Option<Box<dyn std::error::Error + Send + Sync>>, );
```

### `ReceiveErrorKind`

Possible receive error kind values.

```rust
pub enum ReceiveErrorKind
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Connect` | `,` | Connect. |
| `Connection` | `,` | Connection. |
| `Decompress` | `,` | Decompress. |
| `Deserialize` | `,` | Deserialize. |
| `FatallyClosed` | `code: CloseCode` | Fatally Closed. |

### `SendError`

Possible send error values.

```rust
pub enum SendError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Serialize` | `#[source] serde_json::Error` | Serialize. |
| `Encode` | `#[source] crate::gateway::etf::EtfError` | Encode. |
| `Closed` | `,` | Closed. |

## `gateway.etf`

### `EtfError`

Possible etf error values.

```rust
pub enum EtfError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Decode` | `Box<eetf::DecodeError>` | Decode. |
| `Encode` | `eetf::EncodeError` | Encode. |
| `Json` | `serde_json::Error` | Json. |
| `UnsupportedTerm` | `&'static str` | Unsupported Term. |
| `InvalidUtf8` | `,` | Invalid Utf 8. |
| `IntegerOutOfRange` | `,` | Integer Out Of Range. |
| `TooDeep` | `,` | Too Deep. |
| `MissingField` | `&'static str` | Missing Field. |
| `InvalidOpcode` | `,` | Invalid Opcode. |
| `TrailingBytes` | `,` | Trailing Bytes. |

### `encode_value`

```rust
pub fn encode_value(value: &Value) -> Result<Vec<u8>, EtfError>;
```

## `gateway.metrics`

### `ShardMetrics`

```rust
pub struct ShardMetrics { /* private fields */ }
```

#### Methods

##### `record_event`

Records one event metric.

```rust
pub fn record_event(&self);
```

##### `record_reconnect`

Records one reconnect metric.

```rust
pub fn record_reconnect(&self);
```

##### `record_heartbeat`

Records one heartbeat metric.

```rust
pub fn record_heartbeat(&self);
```

##### `record_send_delay`

Records one send delay metric.

```rust
pub fn record_send_delay(&self);
```

##### `events_per_second`

Returns the recent events-per-second rate.

```rust
pub fn events_per_second(&self) -> f64;
```

##### `render_prometheus`

Renders the metrics in Prometheus exposition format.

```rust
pub fn render_prometheus(&self, shard: u32) -> String;
```

## `gateway.queue`

### `IdentifyQueue`

```rust
pub trait IdentifyQueue: fmt::Debug + Send + Sync;
```

#### Methods

##### `wait_turn`

Waits until this shard may identify.

```rust
fn wait_turn(&self, shard: ShardId) -> Pin<Box<dyn Future<Output = ()> + Send>>;
```

### `LocalIdentifyQueue`

```rust
pub struct LocalIdentifyQueue { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `LocalIdentifyQueue`.

```rust
pub fn new(max_concurrency: u32) -> Self;
```

##### `max_concurrency`

Returns the maximum identify concurrency.

```rust
pub fn max_concurrency(&self) -> u32;
```

## `gateway.shard`

### `ShardState`

Possible shard state values.

```rust
pub enum ShardState
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Disconnected` | `,` | Disconnected. |
| `Identifying` | `,` | Identifying. |
| `Resuming` | `,` | Resuming. |
| `Active` | `,` | Active. |
| `FatallyClosed` | `CloseCode` | Fatally Closed. |
| `Closed` | `,` | Closed. |

### `Shard`

```rust
pub struct Shard { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `Shard`.

```rust
pub fn new( id: ShardId, token: impl Into<String>, intents: crate::model::gateway::Intents, ) -> Self;
```

##### `with_config`

Sets config.

```rust
pub fn with_config(id: ShardId, config: Config) -> Self;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> ShardId;
```

##### `config`

Returns the configuration.

```rust
pub const fn config(&self) -> &Config;
```

##### `state`

Returns the current state.

```rust
pub const fn state(&self) -> ShardState;
```

##### `latency`

Returns the last heartbeat latency, when known.

```rust
pub const fn latency(&self) -> Option<Duration>;
```

##### `session_id`

Returns the ID of the session.

```rust
pub fn session_id(&self) -> Option<&str>;
```

##### `sequence`

Returns the last sequence number, when known.

```rust
pub const fn sequence(&self) -> Option<u64>;
```

##### `sender`

Returns the sender for driving this shard.

```rust
pub fn sender(&self) -> MessageSender;
```

##### `next_event`

Waits for the next event from this shard.

```rust
pub async fn next_event(&mut self) -> Option<Result<GatewayEvent, ReceiveError>>;
```

##### `close`

Closes the connection.

```rust
pub async fn close(&mut self);
```

### `MessageSender`

```rust
pub struct MessageSender { /* private fields */ }
```

#### Methods

##### `detached`

Returns whether the sender is detached from its shard.

```rust
pub fn detached() -> Self;
```

##### `command`

Sends a command to the shard.

```rust
pub fn command<T: GatewayCommand>(&self, command: &T) -> Result<(), SendError>;
```

##### `update_presence`

Edits presence.

```rust
pub fn update_presence(&self, presence: &UpdatePresence) -> Result<(), SendError>;
```

##### `is_closed`

Returns whether closed.

```rust
pub fn is_closed(&self) -> bool;
```

## `gateway.sharding`

### `create_shards`

```rust
pub fn create_shards( range: Range<u32>, total: u32, config: Config, ) -> impl ExactSizeIterator<Item = Shard> + DoubleEndedIterator;
```

### `reshard_plan`

```rust
pub fn reshard_plan(_old_total: u32, new_total: u32) -> Vec<(u32, u32)>;
```

### `overlapping_shards`

```rust
pub fn overlapping_shards(old_total: u32, new_total: u32, range: Range<u32>) -> Vec<u32>;
```

### `decode_dispatch_borrowed`

```rust
pub fn decode_dispatch_borrowed(bytes: &[u8]) -> Result<Option<&str>, serde_json::Error>;
```

### `spawn_shard_threads`

```rust
pub fn spawn_shard_threads( range: Range<u32>, total: u32, config: &Config, threads: u32, on_event: impl Fn(u32, GatewayEvent) + Send + Sync + 'static, ) -> std::io::Result<Vec<std::thread::JoinHandle<()>>>;
```

## `model.gateway`

### `pub use self::{ close_code::CloseCode, event::{DispatchEvent, GatewayEvent, GatewayEventDeserializer}, intents::Intents, opcode::OpCode, presence::{ Activity, ActivityAssets, ActivityEmoji, ActivityFlags, ActivityParty, ActivitySecrets, ActivityTimestamps, ActivityType, ClientStatus, Presence, PresenceUser, Status, StatusDisplayType, }, shard_id::ShardId, };`

```rust
pub use self::{ close_code::CloseCode, event::{DispatchEvent, GatewayEvent, GatewayEventDeserializer}, intents::Intents, opcode::OpCode, presence::{ Activity, ActivityAssets, ActivityEmoji, ActivityFlags, ActivityParty, ActivitySecrets, ActivityTimestamps, ActivityType, ClientStatus, Presence, PresenceUser, Status, StatusDisplayType, }, shard_id::ShardId, };
```

### `API_VERSION`

```rust
pub const API_VERSION: u8 = 10;
```

## `model.gateway.close_code`

### `CloseCode`

The reason a gateway connection closed.

```rust
pub enum CloseCode: u16
```

| Variant | Payload | Description |
| --- | --- | --- |
| `UnknownError` | `,` | A value this build does not recognize. |
| `UnknownOpcode` | `,` | A value this build does not recognize. |
| `DecodeError` | `,` | Decode Error. |
| `NotAuthenticated` | `,` | Not Authenticated. |
| `AuthenticationFailed` | `,` | Authentication Failed. |
| `AlreadyAuthenticated` | `,` | Already Authenticated. |
| `InvalidSequence` | `,` | Invalid Sequence. |
| `RateLimited` | `,` | Rate Limited. |
| `SessionTimedOut` | `,` | Session Timed Out. |
| `InvalidShard` | `,` | Invalid Shard. |
| `ShardingRequired` | `,` | Sharding Required. |
| `InvalidApiVersion` | `,` | Invalid API Version. |
| `InvalidIntents` | `,` | Invalid Intents. |
| `DisallowedIntents` | `,` | Disallowed Intents. |

#### Methods

##### `can_reconnect`

Returns whether it can reconnect.

```rust
pub const fn can_reconnect(self) -> bool;
```

## `model.gateway.event`

### `GatewayEvent`

The typed name of a dispatch event.

```rust
pub enum GatewayEvent
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Dispatch` | `u64, DispatchEvent` | Dispatch. |
| `Heartbeat` | `,` | Heartbeat. |
| `Reconnect` | `,` | Reconnect. |
| `InvalidSession` | `bool` | Invalid Session. |
| `Hello` | `Hello` | Hello. |
| `HeartbeatAck` | `,` | Heartbeat Ack. |
| `Unknown` | `OpCode` | A value this build does not recognize. |

### `GatewayEventDeserializer`

Discord's gateway event deserializer object.

```rust
pub struct GatewayEventDeserializer<'a> { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `GatewayEventDeserializer`.

```rust
pub const fn new(op: OpCode, sequence: Option<u64>, event_type: Option<&'a str>) -> Self;
```

##### `from_json`

Parses the value from JSON.

```rust
pub fn from_json(json: &'a str) -> Option<Self>;
```

##### `op`

Returns the opcode.

```rust
pub const fn op(&self) -> OpCode;
```

##### `sequence`

Returns the last sequence number, when known.

```rust
pub const fn sequence(&self) -> Option<u64>;
```

##### `event_type`

Returns the event type.

```rust
pub const fn event_type(&self) -> Option<&'a str>;
```


## `model.gateway.intents`

## `model.gateway.opcode`

### `OpCode`

A gateway opcode.

```rust
pub enum OpCode: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Dispatch` | `,` | Dispatch. |
| `Heartbeat` | `,` | Heartbeat. |
| `Identify` | `,` | Identify. |
| `PresenceUpdate` | `,` | Presence Update. |
| `VoiceStateUpdate` | `,` | Voice State Update. |
| `Resume` | `,` | Resume. |
| `Reconnect` | `,` | Reconnect. |
| `RequestGuildMembers` | `,` | Request Guild Members. |
| `InvalidSession` | `,` | Invalid Session. |
| `Hello` | `,` | Hello. |
| `HeartbeatAck` | `,` | Heartbeat Ack. |
| `RequestSoundboardSounds` | `,` | Request Soundboard Sounds. |
| `RequestChannelInfo` | `,` | Request Channel Info. |

## `model.gateway.presence`

### `Presence`

A user's presence: status, activities and client platform statuses.

```rust
pub struct Presence
```

| Field | Type | Description |
| --- | --- | --- |
| `user` | `PresenceUser` | The user this object describes. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `status` | `Status` | The current status. |
| `activities` | `Vec<Activity>` | The user's current activities. |
| `client_status` | `ClientStatus` | The client status. |

### `PresenceUser`

Discord's presence user object.

```rust
pub struct PresenceUser
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `UserId` | The object's snowflake ID. |
| `username` | `Option<CompactString>` | The account username. |
| `discriminator` | `Option<u16>` | The discriminator. |
| `global_name` | `Option<CompactString>` | The global name. |
| `avatar` | `Option<ImageHash>` | The avatar hash. |
| `bot` | `Option<bool>` | Whether the account belongs to a bot. |
| `system` | `Option<bool>` | Whether this is a system message or system user. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `accent_color` | `Option<u32>` | The accent color. |
| `public_flags` | `Option<UserFlags>` | The account's public flags bitfield. |
| `avatar_decoration_data` | `Option<Box<AvatarDecorationData>>` | The avatar decoration the member has equipped. |
| `collectibles` | `Option<Box<Collectibles>>` | Profile collectibles shown for the member. |
| `primary_guild` | `Option<Box<PrimaryGuild>>` | The primary guild. |

### `ClientStatus`

The per-platform status of a user.

```rust
pub struct ClientStatus
```

| Field | Type | Description |
| --- | --- | --- |
| `desktop` | `Option<Status>` | The desktop. |
| `mobile` | `Option<Status>` | The mobile. |
| `web` | `Option<Status>` | The web. |
| `vr` | `Option<Status>` | The vr. |

### `Activity`

One activity: name, type, timestamps, assets and party.

```rust
pub struct Activity
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `kind` | `ActivityType` | The object's type discriminator. |
| `url` | `Option<CompactString>` | The URL. |
| `created_at` | `Option<Timestamp>` | When the created happened, if known. |
| `timestamps` | `Option<ActivityTimestamps>` | The timestamps. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `status_display_type` | `Option<StatusDisplayType>` | The status display type. |
| `details` | `Option<CompactString>` | The details. |
| `details_url` | `Option<CompactString>` | The URL for the details. |
| `state` | `Option<CompactString>` | The state. |
| `state_url` | `Option<CompactString>` | The URL for the state. |
| `emoji` | `Option<ActivityEmoji>` | The emoji shown with the object. |
| `party` | `Option<ActivityParty>` | The party. |
| `assets` | `Option<Box<ActivityAssets>>` | The assets. |
| `secrets` | `Option<Box<ActivitySecrets>>` | The secrets. |
| `instance` | `bool` | The instance. |
| `flags` | `ActivityFlags` | Bitfield of flags attached to the object. |
| `buttons` | `Vec<CompactString>` | The buttons. |

#### Methods

##### `new`

Creates a new `Activity`.

```rust
pub fn new(kind: ActivityType, name: impl Into<CompactString>) -> Self;
```

##### `playing`

Sets a playing activity.

```rust
pub fn playing(name: impl Into<CompactString>) -> Self;
```

##### `streaming`

Sets a streaming activity.

```rust
pub fn streaming(name: impl Into<CompactString>, url: impl Into<CompactString>) -> Self;
```

##### `listening`

Sets a listening activity.

```rust
pub fn listening(name: impl Into<CompactString>) -> Self;
```

##### `watching`

Sets a watching activity.

```rust
pub fn watching(name: impl Into<CompactString>) -> Self;
```

##### `competing`

Sets a competing activity.

```rust
pub fn competing(name: impl Into<CompactString>) -> Self;
```

##### `custom`

Sets a custom activity.

```rust
pub fn custom(text: impl Into<CompactString>) -> Self;
```

### `ActivityType`

What the activity is doing: playing, streaming, listening, watching or competing.

```rust
pub enum ActivityType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Playing` | `,` | Playing. |
| `Streaming` | `,` | Streaming. |
| `Listening` | `,` | Listening. |
| `Watching` | `,` | Watching. |
| `Custom` | `,` | Custom. |
| `Competing` | `,` | Competing. |

### `StatusDisplayType`

Possible status display type values.

```rust
pub enum StatusDisplayType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Name` | `,` | Name. |
| `State` | `,` | State. |
| `Details` | `,` | Details. |

### `ActivityTimestamps`

Discord's activity timestamps object.

```rust
pub struct ActivityTimestamps
```

| Field | Type | Description |
| --- | --- | --- |
| `start` | `Option<Timestamp>` | The start. |
| `end` | `Option<Timestamp>` | The end. |

### `ActivityEmoji`

Discord's activity emoji object.

```rust
pub struct ActivityEmoji
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `id` | `Option<EmojiId>` | The object's snowflake ID. |
| `animated` | `bool` | The animated. |

### `ActivityParty`

Discord's activity party object.

```rust
pub struct ActivityParty
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<CompactString>` | The object's snowflake ID. |
| `size` | `Option<(u32, u32)>` | The size, in pixels for images. |

### `ActivityAssets`

Discord's activity assets object.

```rust
pub struct ActivityAssets
```

| Field | Type | Description |
| --- | --- | --- |
| `large_image` | `Option<CompactString>` | The large image. |
| `large_text` | `Option<CompactString>` | The large text. |
| `large_url` | `Option<CompactString>` | The URL for the large. |
| `small_image` | `Option<CompactString>` | The small image. |
| `small_text` | `Option<CompactString>` | The small text. |
| `small_url` | `Option<CompactString>` | The URL for the small. |
| `invite_cover_image` | `Option<CompactString>` | The invite cover image. |

### `ActivitySecrets`

Discord's activity secrets object.

```rust
pub struct ActivitySecrets
```

| Field | Type | Description |
| --- | --- | --- |
| `join` | `Option<CompactString>` | The join. |
| `spectate` | `Option<CompactString>` | The spectate. |
| `match_` | `Option<CompactString>` | The match. |

### `Status`

The online, idle, do-not-disturb or offline status.

```rust
pub enum Status
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Online` | `,` | The user is online. |
| `DoNotDisturb` | `,` | The user is online but does not want notifications. |
| `Idle` | `,` | The user is idle. |
| `Invisible` | `,` | Invisible. |
| `Offline` | `,` | The user is offline or invisible. |

## `model.gateway.shard_id`

### `ShardId`

A shard index plus the total shard count.

```rust
pub struct ShardId { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ShardId`.

```rust
pub const fn new(number: u32, total: u32) -> Self;
```

##### `new_checked`

Creates a new `ShardId` after validating its inputs.

```rust
pub const fn new_checked(number: u32, total: u32) -> Option<Self>;
```

##### `for_guild`

Returns the shard that owns the guild.

```rust
pub const fn for_guild(guild: crate::model::id::GuildId, total: u32) -> Self;
```

##### `number`

Returns the shard number.

```rust
pub const fn number(self) -> u32;
```

##### `total`

Returns the total count.

```rust
pub const fn total(self) -> u32;
```
