# Models: core types

Snowflakes, timestamps, hashes, locales, patches and JSON values.

## `model.id`

### `DISCORD_EPOCH_MS`

```rust
pub const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;
```

### `Id`

A Discord snowflake. Parses from strings or integers and formats as a decimal string.

```rust
pub struct Id<T> { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `Id`.

```rust
pub const fn new(value: u64) -> Self;
```

##### `new_checked`

Creates a new `Id` after validating its inputs.

```rust
pub const fn new_checked(value: u64) -> Option<Self>;
```

##### `from_nonzero`

Builds an ID from a nonzero integer without checking the Discord epoch.

```rust
pub const fn from_nonzero(value: NonZeroU64) -> Self;
```

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub const fn get(self) -> u64;
```

##### `into_nonzero`

Consumes this and returns nonzero.

```rust
pub const fn into_nonzero(self) -> NonZeroU64;
```

##### `cast`

Reinterprets the snowflake under a different marker type.

```rust
pub const fn cast<U>(self) -> Id<U>;
```

##### `timestamp_ms`

Returns the timestamp as Unix milliseconds.

```rust
pub const fn timestamp_ms(self) -> u64;
```

##### `timestamp`

Sets the timestamp shown in the footer.

```rust
pub const fn timestamp(self) -> Timestamp;
```

##### `worker_id`

Returns the ID of the worker.

```rust
pub const fn worker_id(self) -> u8;
```

##### `process_id`

Returns the ID of the process.

```rust
pub const fn process_id(self) -> u8;
```

##### `increment`

Returns the next snowflake after this one.

```rust
pub const fn increment(self) -> u16;
```

##### `from_timestamp_ms`

Builds a timestamp from Unix milliseconds.

```rust
pub const fn from_timestamp_ms(unix_ms: u64) -> Option<Self>;
```

### `ParseIdError`

The value was not a valid snowflake.

```rust
pub struct ParseIdError { /* private fields */ }
```

#### Methods

##### `is_zero`

Returns whether zero.

```rust
pub fn is_zero(&self) -> bool;
```

### `pub use marker::aliases::*;`

```rust
pub use marker::aliases::*;
```

## `model.util.image_data`

### `ImageData`

An inline image payload for stickers and soundboard sounds.

```rust
pub struct ImageData { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ImageData`.

```rust
pub fn new(data: impl Into<Bytes>) -> Result<Self, ImageDataError>;
```

##### `with_content_type`

Sets content type.

```rust
pub fn with_content_type( data: impl Into<Bytes>, content_type: impl Into<CompactString>, ) -> Self;
```

##### `data`

Sets the inline image bytes.

```rust
pub const fn data(&self) -> &Bytes;
```

##### `content_type`

Sets the MIME type of the inline image.

```rust
pub fn content_type(&self) -> &str;
```

### `ImageDataError`

Discord's image data error object.

```rust
pub struct ImageDataError;
```

## `model.util.image_hash`

### `ImageHash`

A Discord image hash with helpers for CDN URLs and animation detection.

```rust
pub struct ImageHash { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ImageHash`.

```rust
pub const fn new(bytes: [u8;
```

##### `bytes`

Returns the raw hash bytes.

```rust
pub const fn bytes(self) -> [u8;
```

##### `is_animated`

Returns whether animated.

```rust
pub const fn is_animated(self) -> bool;
```

##### `parse`

Parses an image hash from its hex string.

```rust
pub const fn parse(input: &str) -> Result<Self, ParseImageHashError>;
```

### `ParseImageHashError`

Discord's parse image hash error object.

```rust
pub struct ParseImageHashError(());
```

## `model.util.json_value`

### `JsonValue`

A JSON value that preserves integers and arbitrary precision.

```rust
pub enum JsonValue
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Null` | `,` | Null. |
| `Bool` | `bool` | Bool. |
| `Unsigned` | `u64` | Unsigned. |
| `Signed` | `i64` | Signed. |
| `Float` | `f64` | Float. |
| `String` | `CompactString` | String. |
| `Array` | `Vec<JsonValue>` | Array. |
| `Object` | `BTreeMap<CompactString, JsonValue>` | Object. |

#### Methods

##### `to_type`

Converts this to type.

```rust
pub fn to_type<'a, T: Deserialize<'a>>(&'a self) -> Result<T, JsonValueError>;
```

##### `as_str`

Returns the string form.

```rust
pub fn as_str(&self) -> Option<&str>;
```

### `JsonValueError`

Discord's json value error object.

```rust
pub struct JsonValueError(CompactString);
```

## `model.util.locale`

### `Locale`

A Discord locale code such as en-US.

```rust
pub enum Locale
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Indonesian` | `,` | Indonesian. |
| `Danish` | `,` | Danish. |
| `German` | `,` | German. |
| `EnglishUk` | `,` | English Uk. |
| `EnglishUs` | `,` | English Us. |
| `Spanish` | `,` | Spanish. |
| `SpanishLatam` | `,` | Spanish Latam. |
| `French` | `,` | French. |
| `Croatian` | `,` | Croatian. |
| `Italian` | `,` | Italian. |
| `Lithuanian` | `,` | Lithuanian. |
| `Hungarian` | `,` | Hungarian. |
| `Dutch` | `,` | Dutch. |
| `Norwegian` | `,` | Norwegian. |
| `Polish` | `,` | Polish. |
| `PortugueseBrazilian` | `,` | Portuguese Brazilian. |
| `Romanian` | `,` | Romanian. |
| `Finnish` | `,` | Finnish. |
| `Swedish` | `,` | Swedish. |
| `Vietnamese` | `,` | Vietnamese. |
| `Turkish` | `,` | Turkish. |
| `Czech` | `,` | Czech. |
| `Greek` | `,` | Greek. |
| `Bulgarian` | `,` | Bulgarian. |
| `Russian` | `,` | Russian. |
| `Ukrainian` | `,` | Ukrainian. |
| `Hindi` | `,` | Hindi. |
| `Thai` | `,` | Thai. |
| `ChineseChina` | `,` | Chinese China. |
| `Japanese` | `,` | Japanese. |
| `ChineseTaiwan` | `,` | Chinese Taiwan. |
| `Korean` | `,` | Korean. |

## `model.util.patch`

### `Patch`

A partial-update helper that distinguishes missing fields from explicit nulls.

```rust
pub enum Patch<T>
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Keep` | `,` | Keep. |
| `Clear` | `,` | Clear. |
| `Set` | `T` | Set. |

#### Methods

##### `is_keep`

Returns whether keep.

```rust
pub const fn is_keep(&self) -> bool;
```

##### `is_clear`

Returns whether clear.

```rust
pub const fn is_clear(&self) -> bool;
```

##### `is_set`

Returns whether set.

```rust
pub const fn is_set(&self) -> bool;
```

##### `into_set`

Consumes this and returns set.

```rust
pub fn into_set(self) -> Option<T>;
```

##### `as_ref`

Borrows this as ref.

```rust
pub const fn as_ref(&self) -> Patch<&T>;
```

##### `map`

Applies a function to every value.

```rust
pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Patch<U>;
```

## `model.util.sound_data`

### `SoundData`

An inline sound payload for soundboard sounds.

```rust
pub struct SoundData { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `SoundData`.

```rust
pub fn new(data: impl Into<Bytes>) -> Result<Self, SoundDataError>;
```

##### `with_content_type`

Sets content type.

```rust
pub fn with_content_type( data: impl Into<Bytes>, content_type: impl Into<CompactString>, ) -> Self;
```

##### `data`

Sets the inline sound bytes.

```rust
pub const fn data(&self) -> &Bytes;
```

##### `content_type`

Sets the MIME type of the inline sound.

```rust
pub fn content_type(&self) -> &str;
```

### `SoundDataError`

Discord's sound data error object.

```rust
pub struct SoundDataError;
```

## `model.util.timestamp`

### `Timestamp`

An ISO-8601 timestamp wrapper with conversions to and from Unix time.

```rust
pub struct Timestamp { /* private fields */ }
```

#### Methods

##### `from_unix_micros`

Builds a timestamp from Unix microseconds.

```rust
pub const fn from_unix_micros(unix_micros: i64) -> Self;
```

##### `from_unix_millis`

Builds a timestamp from Unix milliseconds.

```rust
pub const fn from_unix_millis(unix_millis: i64) -> Option<Self>;
```

##### `from_unix_secs`

Builds a timestamp from Unix seconds.

```rust
pub const fn from_unix_secs(unix_secs: i64) -> Option<Self>;
```

##### `now`

Returns the current time.

```rust
pub fn now() -> Self;
```

##### `as_unix_micros`

Borrows this as unix micros.

```rust
pub const fn as_unix_micros(self) -> i64;
```

##### `as_unix_millis`

Borrows this as unix millis.

```rust
pub const fn as_unix_millis(self) -> i64;
```

##### `as_unix_secs`

Borrows this as unix secs.

```rust
pub const fn as_unix_secs(self) -> i64;
```

##### `parse`

Parses an ISO-8601 timestamp.

```rust
pub fn parse(input: &str) -> Result<Self, ParseTimestampError>;
```

### `ParseTimestampError`

Discord's parse timestamp error object.

```rust
pub struct ParseTimestampError(());
```

### ID aliases

Every alias is `Id<Marker>`: it parses, compares, hashes and formats like any other snowflake.

| Alias | Marker |
| --- | --- |
| `ApplicationId` | `ApplicationMarker` |
| `AttachmentId` | `AttachmentMarker` |
| `AuditLogEntryId` | `AuditLogEntryMarker` |
| `AutoModerationRuleId` | `AutoModerationRuleMarker` |
| `ChannelId` | `ChannelMarker` |
| `CommandId` | `CommandMarker` |
| `CommandVersionId` | `CommandVersionMarker` |
| `EmojiId` | `EmojiMarker` |
| `EntitlementId` | `EntitlementMarker` |
| `GenericId` | `GenericMarker` |
| `GuildId` | `GuildMarker` |
| `IntegrationId` | `IntegrationMarker` |
| `InteractionId` | `InteractionMarker` |
| `LobbyId` | `LobbyMarker` |
| `MessageId` | `MessageMarker` |
| `RoleId` | `RoleMarker` |
| `ScheduledEventId` | `ScheduledEventMarker` |
| `SkuId` | `SkuMarker` |
| `SoundboardSoundId` | `SoundboardSoundMarker` |
| `StageInstanceId` | `StageInstanceMarker` |
| `StickerId` | `StickerMarker` |
| `StickerPackId` | `StickerPackMarker` |
| `SubscriptionId` | `SubscriptionMarker` |
| `TagId` | `TagMarker` |
| `TeamId` | `TeamMarker` |
| `UserId` | `UserMarker` |
| `WebhookId` | `WebhookMarker` |
