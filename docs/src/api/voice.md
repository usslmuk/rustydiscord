# Voice

Join calls, play tracks, receive audio and manage the voice lifecycle.

## `voice.call`

### `Call`

A group DM call notice.

```rust
pub struct Call
```

| Field | Type | Description |
| --- | --- | --- |
| `inner` | `Arc<Inner>` | The inner. |

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub fn guild_id(&self) -> GuildId;
```

##### `channel_id`

Returns the ID of the channel.

```rust
pub fn channel_id(&self) -> Option<ChannelId>;
```

##### `play`

Starts playback.

```rust
pub fn play(&self, input: Input) -> TrackHandle;
```

##### `enqueue`

Queues a track for playback.

```rust
pub fn enqueue(&self, input: Input) -> TrackHandle;
```

##### `queue`

Returns the track queue.

```rust
pub fn queue(&self) -> TrackQueue;
```

##### `subscribe`

Subscribes to incoming audio.

```rust
pub fn subscribe(&self, user_id: UserId) -> VoiceStream;
```

##### `subscribe_with`

Subscribes to incoming audio with options.

```rust
pub fn subscribe_with(&self, user_id: UserId, options: ReceiveOptions) -> VoiceStream;
```

##### `events`

Returns a stream of track events.

```rust
pub fn events(&self) -> CallEvents;
```

##### `wait_for`

Waits for the call to end.

```rust
pub async fn wait_for( &self, timeout: Duration, mut matches: impl FnMut(&CallEvent) -> bool, ) -> Result<CallEvent, WaitError>;
```

##### `leave`

Leaves the channel or group.

```rust
pub async fn leave(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
```

### `CallEvent`

Possible call event values.

```rust
pub enum CallEvent
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Resumed` | `,` | Resumed. |
| `Reconnected` | `,` | Reconnected. |
| `Disconnected` | `DisconnectReason` | Disconnected. |
| `UserJoined` | `UserId` | User Joined. |
| `UserLeft` | `UserId` | User Left. |
| `Speaking` | `user_id: UserId, ssrc: u32,` | Speaking. |
| `SpeakingStarted` | `UserId` | Speaking Started. |
| `SpeakingStopped` | `UserId` | Speaking Stopped. |

### `DisconnectReason`

Possible disconnect reason values.

```rust
pub enum DisconnectReason
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Left` | `,` | Left. |
| `Closed` | `code: u16,` | Closed. |
| `Lost` | `Arc<dyn Error + Send + Sync>` | Lost. |

### `WaitError`

Possible wait error values.

```rust
pub enum WaitError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Timeout` | `,` | Timeout. |
| `Ended` | `,` | Ended. |

### `CallEvents`

```rust
pub struct CallEvents { /* private fields */ }
```

#### Methods

##### `next`

Waits for the next item.

```rust
pub async fn next(&mut self) -> Option<CallEvent>;
```





## `voice.error`

### `JoinError`

Possible join error values.

```rust
pub enum JoinError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Gateway` | `Box<dyn Error + Send + Sync>` | Gateway. |
| `Timeout` | `,` | Timeout. |
| `Connection` | `Box<dyn Error + Send + Sync>` | Connection. |
| `Cancelled` | `,` | Cancelled. |

### `InputError`

Possible input error values.

```rust
pub enum InputError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Io` | `io::Error` | Io. |
| `NotOgg` | `,` | Not Ogg. |
| `NotOpus` | `,` | Not Opus. |
| `FfmpegMissing` | `,` | Ffmpeg Missing. |
| `Ffmpeg` | `code: Option<i32>, message: String,` | Ffmpeg. |

### `TrackError`

Possible track error values.

```rust
pub enum TrackError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Ended` | `,` | Ended. |
| `Unsupported` | `,` | Unsupported. |

## `voice.gateway`

### `VoiceGateway`

```rust
pub trait VoiceGateway: Send + Sync + 'static;
```

#### Methods

##### `send`

Sends the payload.

```rust
fn send( &self, shard_id: u32, command: &UpdateVoiceState, ) -> Result<(), Box<dyn Error + Send + Sync>>;
```

## `voice.groups`

### `Group`

```rust
pub struct Group { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `Group`.

```rust
pub fn new() -> Self;
```

##### `add`

Adds the value.

```rust
pub fn add(&mut self, call: Call) -> &mut Self;
```

##### `remove`

Removes a value by key and returns it when present.

```rust
pub fn remove(&mut self, guild_id: GuildId) -> bool;
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

##### `calls`

Returns the active calls.

```rust
pub fn calls(&self) -> Vec<Call>;
```

##### `play`

Starts playback.

```rust
pub fn play(&self, make_input: impl Fn() -> Input) -> Vec<TrackHandle>;
```

##### `enqueue`

Queues a track for playback.

```rust
pub fn enqueue(&self, make_input: impl Fn() -> Input) -> Vec<TrackHandle>;
```

##### `pause_all`

Pauses every call in the group.

```rust
pub fn pause_all(&self) -> Vec<Result<(), crate::voice::TrackError>>;
```

##### `skip_all`

Skips every call in the group.

```rust
pub fn skip_all(&self) -> usize;
```

##### `clear`

Clears the content.

```rust
pub fn clear(&self);
```

##### `current`

Returns the current track, when playing.

```rust
pub fn current(&self) -> Vec<TrackHandle>;
```

## `voice.input`

### `Input`

```rust
pub struct Input { /* private fields */ }
```

#### Methods

##### `ogg`

Creates an input from OGG data.

```rust
pub fn ogg(reader: impl AsyncRead + Send + Unpin + 'static) -> Self;
```

##### `ogg_file`

Creates an input from an OGG file.

```rust
pub async fn ogg_file(path: impl AsRef<Path>) -> Result<Self, InputError>;
```

##### `pcm`

Creates an input from raw PCM samples.

```rust
pub fn pcm(reader: impl AsyncRead + Send + Unpin + 'static) -> Self;
```

##### `ffmpeg`

Creates an input transcoded by FFmpeg.

```rust
pub fn ffmpeg(source: impl AsRef<OsStr>) -> Self;
```




### `ProbedKind`

Possible probed kind values.

```rust
pub enum ProbedKind
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Ogg` | `,` | Ogg. |
| `Raw` | `,` | Raw. |


### `probe`

```rust
pub async fn probe<R>(reader: R) -> Result<(ProbedKind, Input), InputError> where R: AsyncRead + Send + Unpin + 'static,;
```


## `voice.manager`

### `JoinOptions`

```rust
pub struct JoinOptions { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `JoinOptions`.

```rust
pub const fn new() -> Self;
```

##### `self_mute`

Sets whether to join muted.

```rust
pub const fn self_mute(mut self, self_mute: bool) -> Self;
```

##### `self_deaf`

Sets whether to join deafened.

```rust
pub const fn self_deaf(mut self, self_deaf: bool) -> Self;
```

##### `bitrate`

Sets the audio bitrate.

```rust
pub const fn bitrate(mut self, bits_per_second: u32) -> Self;
```

### `VoiceManager`

```rust
pub struct VoiceManager { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `VoiceManager`.

```rust
pub fn new(bot_user_id: UserId, shard_count: u32, gateway: impl VoiceGateway) -> Self;
```

##### `process`

Processes a voice state update.

```rust
pub fn process(&self, event: &DispatchEvent);
```

##### `join`

Joins the voice channel.

```rust
pub async fn join(&self, guild_id: GuildId, channel_id: ChannelId) -> Result<Call, JoinError>;
```

##### `join_with`

Joins the voice channel with options.

```rust
pub async fn join_with( &self, guild_id: GuildId, channel_id: ChannelId, options: JoinOptions, ) -> Result<Call, JoinError>;
```

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub fn get(&self, guild_id: GuildId) -> Option<Call>;
```

##### `calls`

Returns the active calls.

```rust
pub fn calls(&self) -> Vec<Call>;
```

## `voice.player`

### `AudioPlayerStatus`

Possible audio player status values.

```rust
pub enum AudioPlayerStatus
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Idle` | `,` | Idle. |
| `Playing` | `,` | Playing. |
| `Paused` | `,` | Paused. |

### `AudioResource`

```rust
pub struct AudioResource { /* private fields */ }
```

#### Methods

##### `from_ffmpeg`

Creates a resource transcoded by FFmpeg.

```rust
pub fn from_ffmpeg(source: impl AsRef<OsStr>) -> Self;
```

##### `from_file`

Creates a resource from a file.

```rust
pub async fn from_file(path: impl AsRef<Path>) -> Result<Self, InputError>;
```

##### `from_opus`

Creates a resource from raw Opus packets.

```rust
pub fn from_opus(reader: impl AsyncRead + Send + Unpin + 'static) -> Self;
```

##### `into_input`

Consumes this and returns input.

```rust
pub fn into_input(self) -> Input;
```

### `create_audio_resource`

```rust
pub fn create_audio_resource(input: Input) -> AudioResource;
```

### `AudioPlayer`

```rust
pub struct AudioPlayer { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `AudioPlayer`.

```rust
pub fn new(call: Call) -> Self;
```

##### `call`

Returns the call this player belongs to.

```rust
pub fn call(&self) -> &Call;
```

##### `current`

Returns the current track, when playing.

```rust
pub fn current(&self) -> Option<TrackHandle>;
```

##### `status`

Returns the player status.

```rust
pub fn status(&self) -> AudioPlayerStatus;
```

##### `play`

Starts playback.

```rust
pub fn play(&mut self, resource: AudioResource) -> TrackHandle;
```

##### `pause`

Pauses playback.

```rust
pub fn pause(&mut self) -> bool;
```

##### `unpause`

Resumes playback.

```rust
pub fn unpause(&mut self) -> bool;
```

##### `stop`

Stops early.

```rust
pub fn stop(&mut self) -> bool;
```

## `voice.queue`



### `TrackQueue`

```rust
pub struct TrackQueue
```

| Field | Type | Description |
| --- | --- | --- |
| `shared` | `Arc<Shared>` | The shared. |

#### Methods

##### `skip`

Skips to the next track.

```rust
pub fn skip(&self) -> bool;
```

##### `clear`

Clears the content.

```rust
pub fn clear(&self);
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

##### `current`

Returns the current track, when playing.

```rust
pub fn current(&self) -> Option<TrackHandle>;
```

## `voice.receive`

### `EndBehavior`

Possible end behavior values.

```rust
pub enum EndBehavior
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Manual` | `,` | Manual. |
| `AfterSilence` | `Duration` | After Silence. |
| `AfterInactivity` | `Duration` | After Inactivity. |

### `ReceiveOptions`

```rust
pub struct ReceiveOptions { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ReceiveOptions`.

```rust
pub const fn new() -> Self;
```

##### `decode`

Sets whether to decode incoming audio.

```rust
pub const fn decode(mut self, decode: bool) -> Self;
```

##### `end`

Sets when receiving ends.

```rust
pub const fn end(mut self, end: EndBehavior) -> Self;
```

### `VoicePacket`

```rust
pub struct VoicePacket
```

| Field | Type | Description |
| --- | --- | --- |
| `sequence` | `u16` | The sequence. |
| `timestamp` | `u32` | The timestamp. |
| `audio` | `VoiceData` | The audio. |

### `VoiceData`

Possible voice data values.

```rust
pub enum VoiceData
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Opus` | `Vec<u8>` | Opus. |
| `Pcm` | `Vec<i16>` | Pcm. |
| `Lost` | `,` | Lost. |

### `VoiceStream`

```rust
pub struct VoiceStream { /* private fields */ }
```

#### Methods

##### `next`

Waits for the next item.

```rust
pub async fn next(&mut self) -> Option<VoicePacket>;
```






## `voice.report`

### `DependencyReport`

```rust
pub struct DependencyReport
```

| Field | Type | Description |
| --- | --- | --- |
| `opus` | `Option<String>` | The opus. |
| `ffmpeg` | `Option<String>` | The ffmpeg. |
| `dave` | `bool` | The dave. |

### `generate_dependency_report`

```rust
pub fn generate_dependency_report() -> DependencyReport;
```

## `voice.track`


### `TrackHandle`

```rust
pub struct TrackHandle { /* private fields */ }
```

#### Methods

##### `play`

Starts playback.

```rust
pub fn play(&self) -> Result<(), TrackError>;
```

##### `pause`

Pauses playback.

```rust
pub fn pause(&self) -> Result<(), TrackError>;
```

##### `stop`

Stops early.

```rust
pub fn stop(&self) -> Result<(), TrackError>;
```

##### `set_volume`

Sets volume.

```rust
pub fn set_volume(&self, volume: f32) -> Result<(), TrackError>;
```

##### `set_loop`

Sets loop.

```rust
pub fn set_loop(&self, looping: Loop) -> Result<(), TrackError>;
```

##### `seek`

Moves playback to the given position.

```rust
pub async fn seek(&self, position: Duration) -> Result<Duration, TrackError>;
```

##### `info`

Returns information about the track.

```rust
pub fn info(&self) -> TrackInfo;
```

##### `events`

Returns a stream of track events.

```rust
pub fn events(&self) -> TrackEvents;
```

### `Loop`

Possible loop values.

```rust
pub enum Loop
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Off` | `,` | Off. |
| `Forever` | `,` | Forever. |
| `Times` | `u32` | Times. |

### `PlayState`

Possible play state values.

```rust
pub enum PlayState
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Queued` | `,` | Queued. |
| `Playing` | `,` | Playing. |
| `Paused` | `,` | Paused. |
| `Ended` | `,` | Ended. |

### `TrackInfo`

```rust
pub struct TrackInfo
```

| Field | Type | Description |
| --- | --- | --- |
| `state` | `PlayState` | The state. |
| `position` | `Duration` | The position among siblings, lowest first. |
| `volume` | `f32` | Playback volume. |
| `looping` | `Loop` | The looping. |

### `TrackEvent`

Possible track event values.

```rust
pub enum TrackEvent
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Playing` | `,` | Playing. |
| `Paused` | `,` | Paused. |
| `Looped` | `,` | Looped. |
| `Ended` | `TrackEnd` | Ended. |

### `TrackEnd`

Possible track end values.

```rust
pub enum TrackEnd
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Finished` | `,` | Finished. |
| `Stopped` | `,` | Stopped. |
| `Replaced` | `,` | Replaced. |
| `Skipped` | `,` | Skipped. |
| `Failed` | `Arc<InputError>` | Failed. |
| `CallEnded` | `,` | Call Ended. |

### `TrackEvents`

```rust
pub struct TrackEvents { /* private fields */ }
```

#### Methods

##### `next`

Waits for the next item.

```rust
pub async fn next(&mut self) -> Option<TrackEvent>;
```

## `voice_client`

