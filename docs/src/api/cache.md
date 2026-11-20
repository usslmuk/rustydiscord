# Cache

The opt-in cache: configuration, limits, storage backends, metrics and the read views.

## `cache.backend`

### `Store`

The storage contract behind the cache. Implement it to put guilds, channels and messages somewhere other than process memory.

```rust
pub trait Store<K, V>: Send + Sync;
```

#### Methods

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
fn get(&self, key: &K) -> Option<Arc<V>>;
```

##### `insert`

Stores a value, returning the previous one when the key was already present.

```rust
fn insert(&self, key: K, value: Arc<V>) -> Option<Arc<V>>;
```

##### `remove`

Removes a value by key and returns it when present.

```rust
fn remove(&self, key: &K) -> Option<Arc<V>>;
```

##### `len`

Returns how many there are.

```rust
fn len(&self) -> usize;
```

##### `is_empty`

Returns whether there is nothing here.

```rust
fn is_empty(&self) -> bool;
```

##### `contains_key`

Returns whether the key is present.

```rust
fn contains_key(&self, key: &K) -> bool;
```

##### `retain`

Keeps only the entries the predicate accepts.

```rust
fn retain(&self, keep: &mut dyn FnMut(&K, &V) -> bool);
```

##### `values`

Returns every value as a shared handle.

```rust
fn values(&self) -> Vec<Arc<V>>;
```

##### `keys`

Returns every key.

```rust
fn keys(&self) -> Vec<K>;
```

### `Backend`

Marker trait tying a store type to the cache. Implement `Store` for your type, then implement `Backend` to name it.

```rust
pub trait Backend: fmt::Debug + Send + Sync + 'static;
```

#### Methods

##### `store`

Returns the storage backend.

```rust
fn store<K, V>(&self, resource: Resource, limit: Limit) -> Self::Store<K, V> where K: Eq + Hash + Copy + Send + Sync + 'static, V: Send + Sync + 'static;
```

## `cache.config`

### `Resource`

Possible resource values.

```rust
pub enum Resource
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Guild` | `,` | Guild. |
| `Channel` | `,` | Channel. |
| `Thread` | `,` | Thread. |
| `Role` | `,` | Role. |
| `Emoji` | `,` | Emoji. |
| `Sticker` | `,` | Sticker. |
| `Member` | `,` | Member. |
| `User` | `,` | User. |
| `Message` | `,` | Message. |
| `Presence` | `,` | Presence. |
| `VoiceState` | `,` | Voice State. |
| `ScheduledEvent` | `,` | Scheduled Event. |
| `StageInstance` | `,` | Stage Instance. |
| `SoundboardSound` | `,` | Soundboard Sound. |

### `Limit`

Possible limit values.

```rust
pub enum Limit
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Disabled` | `,` | Disabled. |
| `Max` | `usize` | Max. |
| `Unlimited` | `,` | Unlimited. |

#### Methods

##### `is_enabled`

Returns whether enabled.

```rust
pub const fn is_enabled(self) -> bool;
```

##### `max`

Returns the maximum, when one is set.

```rust
pub const fn max(self) -> Option<usize>;
```

### `CacheConfig`

Cache support for cache config.

```rust
pub struct CacheConfig { /* private fields */ }
```

#### Methods

##### `builder`

Returns a builder for the related request or configuration.

```rust
pub fn builder() -> CacheConfigBuilder;
```

##### `large`

A cache preset tuned for large bots with many guilds.

```rust
pub fn large() -> Self;
```

##### `rustydiscord`

The cache preset this library recommends.

```rust
pub fn rustydiscord() -> Self;
```

##### `limit`

Returns the configured limit, when one is set.

```rust
pub const fn limit(&self, resource: Resource) -> Limit;
```

##### `is_enabled`

Returns whether enabled.

```rust
pub const fn is_enabled(&self, resource: Resource) -> bool;
```

##### `sweepers`

Returns the sweeper configuration.

```rust
pub const fn sweepers(&self) -> &SweepConfig;
```

### `CacheConfigBuilder`

Builds cache config values.

```rust
pub struct CacheConfigBuilder { /* private fields */ }
```

#### Methods

##### `limit`

Returns the configured limit, when one is set.

```rust
pub const fn limit(mut self, resource: Resource, limit: Limit) -> Self;
```

##### `guilds`

Returns the guild manager.

```rust
pub const fn guilds(self, limit: Limit) -> Self;
```

##### `channels`

Returns the channel manager.

```rust
pub const fn channels(self, limit: Limit) -> Self;
```

##### `threads`

Returns the thread manager.

```rust
pub const fn threads(self, limit: Limit) -> Self;
```

##### `roles`

Returns the role manager.

```rust
pub const fn roles(self, limit: Limit) -> Self;
```

##### `emojis`

Returns the emoji manager.

```rust
pub const fn emojis(self, limit: Limit) -> Self;
```

##### `stickers`

Returns the sticker manager.

```rust
pub const fn stickers(self, limit: Limit) -> Self;
```

##### `members`

Returns the member manager.

```rust
pub const fn members(self, limit: Limit) -> Self;
```

##### `users`

Returns the users.

```rust
pub const fn users(self, limit: Limit) -> Self;
```

##### `messages`

Returns the message manager.

```rust
pub const fn messages(self, limit: Limit) -> Self;
```

##### `presences`

Returns the presence data.

```rust
pub const fn presences(self, limit: Limit) -> Self;
```

##### `voice_states`

Sets the voice-state cache limit.

```rust
pub const fn voice_states(self, limit: Limit) -> Self;
```

##### `scheduled_events`

Returns the scheduled-event manager.

```rust
pub const fn scheduled_events(self, limit: Limit) -> Self;
```

##### `stage_instances`

Returns the stage-instance manager.

```rust
pub const fn stage_instances(self, limit: Limit) -> Self;
```

##### `soundboard_sounds`

Returns the soundboard-sound manager.

```rust
pub const fn soundboard_sounds(self, limit: Limit) -> Self;
```

##### `sweep_messages`

Removes stale messages from the cache.

```rust
pub const fn sweep_messages(mut self, sweep: Option<Sweep>) -> Self;
```

##### `sweep_threads`

Removes stale threads from the cache.

```rust
pub const fn sweep_threads(mut self, sweep: Option<Sweep>) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> CacheConfig;
```

### `Sweep`

Cache support for sweep.

```rust
pub struct Sweep
```

| Field | Type | Description |
| --- | --- | --- |
| `interval` | `Duration` | The interval between runs. |
| `lifetime` | `Duration` | The lifetime. |

#### Methods

##### `new`

Creates a new `Sweep`.

```rust
pub const fn new(interval: Duration, lifetime: Duration) -> Self;
```

### `SweepConfig`

Cache support for sweep config.

```rust
pub struct SweepConfig
```

| Field | Type | Description |
| --- | --- | --- |
| `messages` | `Option<Sweep>` | The messages. |
| `threads` | `Option<Sweep>` | The threads in this channel. |

## `cache.dashmap_store`

### `DashMapBackend`

Cache support for dash map backend.

```rust
pub struct DashMapBackend;
```

#### Methods

##### `new`

Creates a new `DashMapBackend`.

```rust
pub const fn new() -> Self;
```

### `DashMapStore`

Cache support for dash map store.

```rust
pub struct DashMapStore<K, V> { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `DashMapStore`.

```rust
pub fn new() -> Self;
```

## `cache.metrics`

### `CacheMetrics`

Cache support for cache metrics.

```rust
pub struct CacheMetrics { /* private fields */ }
```

#### Methods

##### `record_hit`

Records one hit metric.

```rust
pub fn record_hit(&self);
```

##### `record_miss`

Records one miss metric.

```rust
pub fn record_miss(&self);
```

##### `record_insert`

Records one insert metric.

```rust
pub fn record_insert(&self);
```

##### `record_eviction`

Records one eviction metric.

```rust
pub fn record_eviction(&self);
```

##### `hit_rate`

Returns the cache hit rate as a fraction.

```rust
pub fn hit_rate(&self) -> f64;
```

##### `render_prometheus`

Renders the metrics in Prometheus exposition format.

```rust
pub fn render_prometheus(&self) -> String;
```

## `cache.model`

### `CachedGuild`

Cache support for cached guild.

```rust
pub struct CachedGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `icon_hash` | `Option<ImageHash>` | The icon hash. |
| `splash` | `Option<ImageHash>` | The invite splash hash. |
| `discovery_splash` | `Option<ImageHash>` | The discovery splash hash. |
| `owner` | `bool` | Whether the user owns the guild or application. |
| `owner_id` | `UserId` | The ID of the owner. |
| `permissions` | `Option<Permissions>` | The permissions bitfield. |
| `region` | `Option<CompactString>` | The voice region. |
| `afk_channel_id` | `Option<ChannelId>` | The ID of the AFK voice channel. |
| `afk_timeout` | `u32` | How long before a member is moved to the AFK channel, in seconds. |
| `widget_enabled` | `bool` | Whether the guild widget is enabled. |
| `widget_channel_id` | `Option<ChannelId>` | The ID of the channel the widget invites to. |
| `verification_level` | `VerificationLevel` | The guild's verification requirement for members. |
| `default_message_notifications` | `DefaultMessageNotificationLevel` | The default notification level for new members. |
| `explicit_content_filter` | `ExplicitContentFilterLevel` | The guild's explicit media filter level. |
| `features` | `Vec<Feature>` | The feature flags enabled for the guild. |
| `mfa_level` | `MfaLevel` | Whether moderators need two-factor authentication. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `system_channel_id` | `Option<ChannelId>` | The ID of the channel that receives system messages. |
| `system_channel_flags` | `SystemChannelFlags` | Which system messages the system channel receives. |
| `rules_channel_id` | `Option<ChannelId>` | The ID of the rules channel. |
| `max_presences` | `Option<u32>` | The maximum number of presences the guild allows. |
| `max_members` | `Option<u32>` | The maximum number of members the guild allows. |
| `vanity_url_code` | `Option<CompactString>` | The guild's vanity invite code. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `premium_tier` | `PremiumTier` | The guild's boost tier. |
| `premium_subscription_count` | `Option<u32>` | How many boosts the guild has. |
| `preferred_locale` | `Locale` | The guild's preferred language. |
| `public_updates_channel_id` | `Option<ChannelId>` | The ID of the channel that receives public updates. |
| `max_video_channel_users` | `Option<u32>` | The maximum number of users in a video channel. |
| `max_stage_video_channel_users` | `Option<u32>` | The max stage video channel users. |
| `approximate_member_count` | `Option<u32>` | The approximate member count from the widget. |
| `approximate_presence_count` | `Option<u32>` | The approximate online count from the widget. |
| `welcome_screen` | `Option<Box<WelcomeScreen>>` | The guild's welcome screen configuration. |
| `nsfw_level` | `AgeRestrictionLevel` | The guild's NSFW level. |
| `premium_progress_bar_enabled` | `bool` | Whether the boost progress bar is shown. |
| `safety_alerts_channel_id` | `Option<ChannelId>` | The ID of the channel that receives safety alerts. |
| `incidents_data` | `Option<Box<IncidentsData>>` | Recent incident restrictions on the guild. |
| `joined_at` | `Option<Timestamp>` | When the member joined the guild. |
| `large` | `bool` | The large. |
| `member_count` | `u32` | How many members are in the thread. |

### `CachedUser`

Cache support for cached user.

```rust
pub struct CachedUser
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `UserId` | The object's snowflake ID. |
| `username` | `CompactString` | The account username. |
| `discriminator` | `u16` | The discriminator. |
| `global_name` | `Option<CompactString>` | The global name. |
| `avatar` | `Option<ImageHash>` | The avatar hash. |
| `bot` | `bool` | Whether the account belongs to a bot. |
| `system` | `bool` | Whether this is a system message or system user. |
| `public_flags` | `Option<UserFlags>` | The account's public flags bitfield. |

#### Methods

##### `banner`

Returns the banner hash, when present.

```rust
pub fn banner(&self) -> Option<ImageHash>;
```

##### `accent_color`

Sets the accent color from a 24-bit integer.

```rust
pub fn accent_color(&self) -> Option<u32>;
```

##### `avatar_decoration_data`

Returns the avatar decoration, when equipped.

```rust
pub fn avatar_decoration_data(&self) -> Option<&AvatarDecorationData>;
```

##### `collectibles`

Returns the profile collectibles, when present.

```rust
pub fn collectibles(&self) -> Option<&Collectibles>;
```

##### `primary_guild`

Returns the profile's primary guild, when set.

```rust
pub fn primary_guild(&self) -> Option<&PrimaryGuild>;
```

##### `display_name`

Returns the name shown in the UI, preferring the guild nickname when set.

```rust
pub fn display_name(&self) -> &str;
```

##### `tag`

Returns the discriminator-bearing username tag, when the account still has one.

```rust
pub fn tag(&self) -> CompactString;
```

### `CachedMember`

Cache support for cached member.

```rust
pub struct CachedMember
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `user` | `Arc<CachedUser>` | The user this object describes. |
| `nick` | `Option<CompactString>` | The member's guild nickname, when set. |
| `roles` | `Box<[RoleId]>` | The roles attached or referenced. |
| `joined_at` | `Option<Timestamp>` | When the member joined the guild. |
| `deaf` | `bool` | Whether the member is server-deafened. |
| `mute` | `bool` | Whether the member is server-muted. |
| `pending` | `bool` | Whether the member has not finished onboarding. |
| `flags` | `MemberFlags` | Bitfield of flags attached to the object. |

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub fn id(&self) -> UserId;
```

##### `display_name`

Returns the name shown in the UI, preferring the guild nickname when set.

```rust
pub fn display_name(&self) -> &str;
```

##### `avatar`

Returns the member avatar hash, when set.

```rust
pub fn avatar(&self) -> Option<ImageHash>;
```

##### `banner`

Returns the member banner hash, when set.

```rust
pub fn banner(&self) -> Option<ImageHash>;
```

##### `premium_since`

Returns when the member started boosting, when applicable.

```rust
pub fn premium_since(&self) -> Option<Timestamp>;
```

##### `communication_disabled_until`

Returns when the timeout expires, when one is active.

```rust
pub fn communication_disabled_until(&self) -> Option<Timestamp>;
```

##### `avatar_decoration_data`

Returns the avatar decoration, when equipped.

```rust
pub fn avatar_decoration_data(&self) -> Option<&AvatarDecorationData>;
```

##### `collectibles`

Returns the profile collectibles, when present.

```rust
pub fn collectibles(&self) -> Option<&Collectibles>;
```


### `CachedMessage`

Cache support for cached message.

```rust
pub struct CachedMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `MessageId` | The object's snowflake ID. |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `author` | `Arc<CachedUser>` | The author. |
| `content` | `CompactString` | The message text. |
| `timestamp` | `Timestamp` | The timestamp. |
| `edited_timestamp` | `Option<Timestamp>` | When the edited happened, if known. |
| `tts` | `bool` | The tts. |
| `mention_everyone` | `bool` | Whether the message mentions @everyone. |
| `mentions` | `Vec<Mention>` | The mentions. |
| `mention_roles` | `Vec<RoleId>` | The role IDs mentioned by the message. |
| `mention_channels` | `Vec<ChannelMention>` | The mention channels. |
| `attachments` | `Vec<Attachment>` | The files attached to the message. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `reactions` | `Vec<Reaction>` | The reactions. |
| `nonce` | `Option<Nonce>` | The nonce used for message deduplication. |
| `pinned` | `bool` | Whether the message is pinned. |
| `webhook_id` | `Option<WebhookId>` | The ID of the webhook. |
| `kind` | `MessageType` | The object's type discriminator. |
| `activity` | `Option<Box<MessageActivity>>` | The activity. |
| `application` | `Option<Box<PartialApplication>>` | The application this belongs to. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `flags` | `MessageFlags` | Bitfield of flags attached to the object. |
| `message_reference` | `Option<MessageReference>` | The message being replied to or forwarded. |
| `message_snapshots` | `Vec<MessageSnapshot>` | The message snapshots. |
| `referenced_message` | `ReferencedMessage` | The referenced message. |
| `interaction_metadata` | `Option<Box<InteractionMetadata>>` | The interaction metadata. |
| `interaction` | `Option<Box<MessageInteraction>>` | The interaction this refers to. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `thread` | `Option<Box<Channel>>` | The thread. |
| `resolved` | `Option<Box<Resolved>>` | The resolved. |
| `sticker_items` | `Vec<StickerItem>` | The sticker items. |
| `stickers` | `Vec<Sticker>` | The stickers attached to the message. |
| `position` | `Option<u32>` | The position among siblings, lowest first. |
| `role_subscription_data` | `Option<Box<RoleSubscriptionData>>` | The role subscription data. |
| `poll` | `Option<Box<Poll>>` | The poll attached to the message. |
| `call` | `Option<Box<MessageCall>>` | The call. |
| `shared_client_theme` | `Option<Box<SharedClientTheme>>` | The shared client theme. |
| `channel_type` | `Option<ChannelType>` | The channel type. |

### `CachedPresence`

Cache support for cached presence.

```rust
pub struct CachedPresence
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `user_id` | `UserId` | The ID of the user. |
| `status` | `Status` | The current status. |
| `client_status` | `ClientStatus` | The client status. |
| `activities` | `Box<[Activity]>` | The user's current activities. |

## `cache.moka_store`

### `MokaBackend`

Cache support for moka backend.

```rust
pub struct MokaBackend { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `MokaBackend`.

```rust
pub const fn new() -> Self;
```

##### `with_max_capacity`

Sets max capacity.

```rust
pub const fn with_max_capacity(max_capacity: u64) -> Self;
```

### `MokaStore`

Cache support for moka store.

```rust
pub struct MokaStore<K, V> { /* private fields */ }
```

## `cache.redis_store`

### `RedisConfig`

Cache support for redis config.

```rust
pub struct RedisConfig
```

| Field | Type | Description |
| --- | --- | --- |
| `url` | `String` | The URL. |
| `prefix` | `String` | The prefix. |

### `RedisBackend`

Cache support for redis backend.

```rust
pub struct RedisBackend { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `RedisBackend`.

```rust
pub const fn new() -> Self;
```

##### `with_config`

Sets config.

```rust
pub fn with_config(config: RedisConfig) -> Self;
```

### `RedisStore`

Cache support for redis store.

```rust
pub struct RedisStore<K, V> { /* private fields */ }
```

## `cache.state`

### `Cache`

Cache support for cache.

```rust
pub struct Cache<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `Cache`.

```rust
pub fn new() -> Self;
```

##### `with_config`

Sets config.

```rust
pub fn with_config(config: CacheConfig) -> Self;
```

##### `with_backend`

Sets backend.

```rust
pub fn with_backend(config: CacheConfig, backend: B) -> Self;
```

##### `config`

Returns the configuration.

```rust
pub const fn config(&self) -> &CacheConfig;
```

##### `backend`

Returns the storage backend.

```rust
pub const fn backend(&self) -> &B;
```

##### `update`

Applies the latest update.

```rust
pub fn update<E: UpdateCache>(&self, event: &E) -> E::Previous;
```

##### `update_dispatch`

Edits dispatch.

```rust
pub fn update_dispatch(&self, event: &DispatchEvent) -> Previous;
```

##### `current_user`

Returns the cached current user.

```rust
pub fn current_user(&self) -> Option<Arc<CurrentUser>>;
```

##### `unavailable_guilds`

Returns the IDs of guilds that are currently unavailable.

```rust
pub fn unavailable_guilds(&self) -> Vec<GuildId>;
```

##### `guilds`

Returns the guild manager.

```rust
pub fn guilds(&self) -> CacheView<'_, GuildId, CachedGuild, B>;
```

##### `channels`

Returns the channel manager.

```rust
pub fn channels(&self) -> CacheView<'_, ChannelId, Channel, B>;
```

##### `users`

Returns the users.

```rust
pub fn users(&self) -> CacheView<'_, UserId, CachedUser, B>;
```

##### `roles`

Returns the role manager.

```rust
pub fn roles(&self, guild_id: GuildId) -> GuildView<'_, RoleId, Role, B>;
```

##### `emojis`

Returns the emoji manager.

```rust
pub fn emojis(&self, guild_id: GuildId) -> GuildView<'_, EmojiId, Emoji, B>;
```

##### `stickers`

Returns the sticker manager.

```rust
pub fn stickers(&self, guild_id: GuildId) -> GuildView<'_, StickerId, Sticker, B>;
```

##### `members`

Returns the member manager.

```rust
pub fn members(&self, guild_id: GuildId) -> GuildView<'_, UserId, CachedMember, B>;
```

##### `threads`

Returns the thread manager.

```rust
pub fn threads(&self, guild_id: GuildId) -> GuildView<'_, ChannelId, Channel, B>;
```

##### `messages`

Returns the message manager.

```rust
pub fn messages(&self, channel_id: ChannelId) -> ChannelView<'_, MessageId, CachedMessage, B>;
```

##### `presences`

Returns the presence data.

```rust
pub fn presences(&self, guild_id: GuildId) -> GuildView<'_, UserId, CachedPresence, B>;
```

##### `voice_states`

Returns the cached voice states.

```rust
pub fn voice_states(&self, guild_id: GuildId) -> GuildView<'_, UserId, VoiceState, B>;
```

##### `scheduled_events`

Returns the scheduled-event manager.

```rust
pub fn scheduled_events( &self, guild_id: GuildId, ) -> GuildView<'_, ScheduledEventId, GuildScheduledEvent, B>;
```

##### `stage_instances`

Returns the stage-instance manager.

```rust
pub fn stage_instances( &self, guild_id: GuildId, ) -> GuildView<'_, StageInstanceId, StageInstance, B>;
```

##### `soundboard_sounds`

Returns the soundboard-sound manager.

```rust
pub fn soundboard_sounds( &self, guild_id: GuildId, ) -> GuildView<'_, SoundboardSoundId, SoundboardSound, B>;
```

##### `sweep_guilds`

Removes stale guilds from the cache.

```rust
pub fn sweep_guilds(&self, mut filter: impl FnMut(&CachedGuild) -> bool) -> usize;
```

##### `sweep_channels`

Removes stale channels from the cache.

```rust
pub fn sweep_channels(&self, mut filter: impl FnMut(&Channel) -> bool) -> usize;
```

##### `sweep_users`

Removes stale users from the cache.

```rust
pub fn sweep_users(&self, mut filter: impl FnMut(&CachedUser) -> bool) -> usize;
```

##### `sweep_threads`

Removes stale threads from the cache.

```rust
pub fn sweep_threads(&self, mut filter: impl FnMut(GuildId, &Channel) -> bool) -> usize;
```

##### `sweep_archived_threads`

Removes stale archived threads from the cache.

```rust
pub fn sweep_archived_threads(&self, lifetime: Duration) -> usize;
```

##### `sweep_messages`

Removes stale messages from the cache.

```rust
pub fn sweep_messages( &self, mut filter: impl FnMut(ChannelId, &CachedMessage) -> bool, ) -> usize;
```

##### `sweep_messages_older_than`

Removes stale messages older than from the cache.

```rust
pub fn sweep_messages_older_than(&self, lifetime: Duration) -> usize;
```

##### `ensure_presence_member`

Ensures the presence's member is cached, fetching it when missing.

```rust
pub fn ensure_presence_member( &self, guild_id: GuildId, presence: &Presence, ) -> Option<Arc<CachedMember>>;
```

##### `store_user`

Stores user in the cache.

```rust
pub fn store_user(&self, user: &User) -> Arc<CachedUser>;
```

##### `store_member`

Stores member in the cache.

```rust
pub fn store_member(&self, guild_id: GuildId, member: &Member) -> Arc<CachedMember>;
```

##### `store_message`

Stores message in the cache.

```rust
pub fn store_message(&self, message: &Message) -> Arc<CachedMessage>;
```

##### `store_channel`

Stores channel in the cache.

```rust
pub fn store_channel(&self, channel: &Channel) -> Arc<Channel>;
```

##### `store_role`

Stores role in the cache.

```rust
pub fn store_role(&self, guild_id: GuildId, role: &Role) -> Arc<Role>;
```

##### `store_emoji`

Stores emoji in the cache.

```rust
pub fn store_emoji(&self, guild_id: GuildId, emoji: &Emoji) -> Arc<Emoji>;
```

##### `store_sticker`

Stores sticker in the cache.

```rust
pub fn store_sticker(&self, guild_id: GuildId, sticker: &Sticker) -> Arc<Sticker>;
```

##### `store_scheduled_event`

Stores scheduled event in the cache.

```rust
pub fn store_scheduled_event( &self, guild_id: GuildId, event: &GuildScheduledEvent, ) -> Arc<GuildScheduledEvent>;
```

##### `store_stage_instance`

Stores stage instance in the cache.

```rust
pub fn store_stage_instance( &self, guild_id: GuildId, stage: &StageInstance, ) -> Arc<StageInstance>;
```

##### `store_soundboard_sound`

Stores soundboard sound in the cache.

```rust
pub fn store_soundboard_sound( &self, guild_id: GuildId, sound: &SoundboardSound, ) -> Arc<SoundboardSound>;
```

##### `store_guild`

Stores guild in the cache.

```rust
pub fn store_guild(&self, guild: &Guild) -> Arc<CachedGuild>;
```


## `cache.update`

### `UpdateCache`

```rust
pub trait UpdateCache: sealed::Sealed;
```

#### Methods

##### `update_cache`

Edits cache.

```rust
fn update_cache<B: Backend>(&self, cache: &Cache<B>) -> Self::Previous;
```

### `Previous`

Possible previous values.

```rust
pub enum Previous
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | None. |
| `CurrentUser` | `Arc<CurrentUser>` | Current User. |
| `Guild` | `Arc<CachedGuild>` | Guild. |
| `Channel` | `Arc<Channel>` | Channel. |
| `Role` | `Arc<Role>` | Role. |
| `Emojis` | `Vec<Arc<Emoji>>` | Emojis. |
| `Stickers` | `Vec<Arc<Sticker>>` | Stickers. |
| `Member` | `Arc<CachedMember>` | Member. |
| `Members` | `Vec<Arc<CachedMember>>` | Members. |
| `Message` | `Arc<CachedMessage>` | Message. |
| `Messages` | `Vec<Arc<CachedMessage>>` | Messages. |
| `Presence` | `Arc<CachedPresence>` | Presence. |
| `VoiceState` | `Arc<VoiceState>` | Voice State. |
| `Thread` | `Arc<Channel>` | Thread. |
| `Threads` | `Vec<Arc<Channel>>` | Threads. |
| `ScheduledEvent` | `Arc<GuildScheduledEvent>` | Scheduled Event. |
| `StageInstance` | `Arc<StageInstance>` | Stage Instance. |
| `SoundboardSound` | `Arc<SoundboardSound>` | Soundboard Sound. |
| `SoundboardSounds` | `Vec<Arc<SoundboardSound>>` | Soundboard Sounds. |

## `cache.view`

### `CacheView`

Cache support for cache view.

```rust
pub struct CacheView<'a, K, V, B> where K: Eq + Hash + Copy + Send + Sync + 'static, V: Send + Sync + 'static, B: Backend, { /* private fields */ }
```

#### Methods

##### `is_enabled`

Returns whether enabled.

```rust
pub const fn is_enabled(&self) -> bool;
```

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub fn get(&self, id: K) -> Option<Arc<V>>;
```

##### `contains`

Returns whether the value is present.

```rust
pub fn contains(&self, id: K) -> bool;
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

##### `keys`

Returns every key.

```rust
pub fn keys(&self) -> Vec<K>;
```

##### `values`

Returns every value as a shared handle.

```rust
pub fn values(&self) -> Vec<Arc<V>>;
```

##### `in_guild`

Returns whether the object belongs to the guild.

```rust
pub fn in_guild(&self, guild_id: GuildId) -> Vec<ChannelId>;
```

### `GuildView`

Cache support for guild view.

```rust
pub struct GuildView<'a, K, V, B> where K: Eq + Hash + Copy + Send + Sync + 'static, V: Send + Sync + 'static, B: Backend, { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(&self) -> GuildId;
```

##### `is_enabled`

Returns whether enabled.

```rust
pub const fn is_enabled(&self) -> bool;
```

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub fn get(&self, id: K) -> Option<Arc<V>>;
```

##### `contains`

Returns whether the value is present.

```rust
pub fn contains(&self, id: K) -> bool;
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

##### `keys`

Returns every key.

```rust
pub fn keys(&self) -> Vec<K>;
```

##### `values`

Returns every value as a shared handle.

```rust
pub fn values(&self) -> Vec<Arc<V>>;
```

### `ChannelView`

Cache support for channel view.

```rust
pub struct ChannelView<'a, K, V, B> where K: Eq + Hash + Copy + Send + Sync + 'static, V: Send + Sync + 'static, B: Backend, { /* private fields */ }
```

#### Methods

##### `channel_id`

Returns the ID of the channel.

```rust
pub const fn channel_id(&self) -> ChannelId;
```

##### `is_enabled`

Returns whether enabled.

```rust
pub const fn is_enabled(&self) -> bool;
```

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub fn get(&self, id: K) -> Option<Arc<V>>;
```

##### `contains`

Returns whether the value is present.

```rust
pub fn contains(&self, id: K) -> bool;
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

##### `keys`

Returns every key.

```rust
pub fn keys(&self) -> Vec<K>;
```

##### `values`

Returns every value as a shared handle.

```rust
pub fn values(&self) -> Vec<Arc<V>>;
```
