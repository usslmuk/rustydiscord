# Collectors

Filtered event streams with time, idle and count caps.

## `collector`

### `EndReason`

Why a collector stopped: Finished, Idle, MaxedOut, or the watched object was Deleted.

```rust
pub enum EndReason
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Time` | `,` | Time. |
| `Idle` | `,` | Idle. |
| `Limit` | `,` | Limit. |
| `ProcessedLimit` | `,` | Processed Limit. |
| `UserLimit` | `,` | User Limit. |
| `EmojiLimit` | `,` | Emoji Limit. |
| `MessageDelete` | `,` | Message Delete. |
| `ChannelDelete` | `,` | Channel Delete. |
| `ThreadDelete` | `,` | Thread Delete. |
| `GuildDelete` | `,` | Guild Delete. |
| `Stopped` | `,` | Stopped. |
| `Shutdown` | `,` | Shutdown. |

#### Methods

##### `as_str`

Returns the string form.

```rust
pub const fn as_str(self) -> &'static str;
```

### `CollectorError`

The collector ended before producing the awaited item.

```rust
pub struct CollectorError { /* private fields */ }
```

#### Methods

##### `reason`

Returns the audit-log reason, when set.

```rust
pub const fn reason(self) -> EndReason;
```

### `ReactionEvent`

A reaction add or remove, normalized across cached and uncached messages.

```rust
pub enum ReactionEvent
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Added` | `Arc<MessageReactionAdd>` | Added. |
| `Removed` | `Arc<MessageReactionRemove>` | Removed. |
| `RemovedAll` | `Arc<MessageReactionRemoveAll>` | Removed All. |

#### Methods

##### `user_id`

Who reacted, when known.

```rust
pub fn user_id(&self) -> Option<UserId>;
```

##### `emoji`

The emoji used, as unicode text or a custom emoji reference.

```rust
pub fn emoji(&self) -> Option<&PartialEmoji>;
```

### `CollectedReaction`

Reaction counts grouped by emoji for one message.

```rust
pub struct CollectedReaction
```

| Field | Type | Description |
| --- | --- | --- |
| `emoji` | `PartialEmoji` | The emoji. |
| `users` | `Vec<UserId>` | The users. |

#### Methods

##### `count`

Total reactions counted under this emoji.

```rust
pub fn count(&self) -> usize;
```

### `reaction_key`

Stable grouping key for an emoji: unicode text or custom id.

```rust
pub fn reaction_key(emoji: &PartialEmoji) -> String;
```

### `Collected`

One collected item plus the key it matched on.

```rust
pub struct Collected<K, V> { /* private fields */ }
```

#### Methods

##### `reason`

Returns the audit-log reason, when set.

```rust
pub const fn reason(&self) -> EndReason;
```

##### `into_collection`

Drains everything collected into an insertion-ordered map.

```rust
pub fn into_collection(self) -> Collection<K, V>;
```

### `Collectable`

```rust
pub trait Collectable: sealed::Item;
```


### `CollectorBuilder`

Configures what a collector watches and when it stops.

```rust
pub struct CollectorBuilder<T: Collectable, M = mode::Stream> { /* private fields */ }
```

#### Methods

##### `time`

Stop after this long no matter what.

```rust
pub const fn time(mut self, time: Duration) -> Self;
```

##### `idle`

Stop after this long without a matching item.

```rust
pub const fn idle(mut self, idle: Duration) -> Self;
```

##### `max`

Stop after this many accepted items.

```rust
pub const fn max(mut self, max: usize) -> Self;
```

##### `max_processed`

Stop after processing this many events.

```rust
pub const fn max_processed(mut self, max: usize) -> Self;
```

##### `filter`

Only accept items the predicate approves.

```rust
pub fn filter(mut self, filter: impl FnMut(&T) -> bool + Send + 'static) -> Self;
```

##### `component_type`

Only accept clicks on this component type.

```rust
pub const fn component_type(mut self, kind: ComponentType) -> Self;
```

##### `max_users`

Stop after this many distinct users contributed.

```rust
pub const fn max_users(mut self, max: usize) -> Self;
```

##### `dispose`

Keep tracking removals (un-reacts) too.

```rust
pub const fn dispose(mut self, dispose: bool) -> Self;
```

##### `max_users`

Stop after this many distinct users contributed.

```rust
pub const fn max_users(mut self, max: usize) -> Self;
```

##### `max_emojis`

Stop after this many distinct emoji.

```rust
pub const fn max_emojis(mut self, max: usize) -> Self;
```

##### `build`

Builds the client. Nothing connects until login.

```rust
pub fn build(self) -> Collector<T>;
```


### `Collector`

An open collection: a filtered stream of messages, clicks, reactions or modals with a time, idle, count or user cap.

```rust
pub struct Collector<T: Collectable> { /* private fields */ }
```

#### Methods

##### `next`

Waits for the next collected item, or None when the collector ends.

```rust
pub async fn next(&mut self) -> Option<T>;
```

##### `stop`

Ends the collector early. Buffered items stay available.

```rust
pub fn stop(&mut self);
```

##### `reset_timer`

Restarts the idle countdown without ending the collector.

```rust
pub fn reset_timer(&mut self, time: Option<Duration>, idle: Option<Duration>);
```

##### `end_reason`

Why the collector stopped: finished, idle, maxed out, or the watched message vanished.

```rust
pub const fn end_reason(&self) -> Option<EndReason>;
```

##### `is_ended`

Whether the collector has stopped accepting items.

```rust
pub const fn is_ended(&self) -> bool;
```

##### `collected`

Everything collected so far, in arrival order.

```rust
pub const fn collected(&self) -> usize;
```

##### `processed`

How many events the collector looked at, including rejected ones.

```rust
pub const fn processed(&self) -> usize;
```

### `AwaitFirst`

A future that resolves with the first matching item.

```rust
pub struct AwaitFirst<T: Collectable> { /* private fields */ }
```

### `AwaitAll`

A future that gathers every matching item until the collector ends.

```rust
pub struct AwaitAll<T: Collectable, K, V> { /* private fields */ }
```

### `Context`

##### `message_collector`

Creates a collector for messages.

```rust
pub fn message_collector(&self, channel: ChannelId) -> CollectorBuilder<Arc<MessageCreate>>;
```

##### `await_messages`

Waits for the next matching message.

```rust
pub fn await_messages( &self, channel: ChannelId, ) -> CollectorBuilder<Arc<MessageCreate>, mode::All>;
```

##### `component_collector`

Creates a collector for component interactions.

```rust
pub fn component_collector( &self, channel: ChannelId, ) -> CollectorBuilder<ComponentInteraction>;
```

##### `await_component`

Waits for the next matching component interaction.

```rust
pub fn await_component( &self, channel: ChannelId, ) -> CollectorBuilder<ComponentInteraction, mode::First>;
```

## `collector.index`

### `Signal`

Possible signal values.

```rust
pub enum Signal<T>
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Item` | `T` | Item. |
| `End` | `EndReason` | End. |

### `Sink`

Possible sink values.

```rust
pub enum Sink
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Messages` | `Tx<Arc<MessageCreate>>` | Messages. |
| `Reactions` | `Tx<ReactionEvent>` | Reactions. |
| `Components` | `Tx<ComponentInteraction>` | Components. |
| `Modals` | `Tx<ModalInteraction>` | Modals. |
| `MemberChunks` | `Tx<MemberReply>` | Member Chunks. |
| `SoundboardSounds` | `Tx<Arc<SoundboardSounds>>` | Soundboard Sounds. |

### `MemberReply`

Possible member reply values.

```rust
pub enum MemberReply
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Chunk` | `Arc<GuildMembersChunk>` | Chunk. |
| `RateLimited` | `Duration` | Rate Limited. |





## collector.mode

_Module docs follow._

