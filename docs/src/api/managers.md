# Managers

Scoped clients for every resource: fetch, list, create, edit, delete.

## `manager`

### `pub use self::{ application_emoji::ApplicationEmojiManager, auto_moderation::AutoModerationRuleManager, ban::BanManager, channel::{ChannelManager, OverwriteTarget}, command::{CommandManager, GuildCommandManager}, emoji::EmojiManager, guild::GuildManager, guild_channel::GuildChannelManager, invite::InviteManager, member::MemberManager, message::MessageManager, role::RoleManager, scheduled_event::ScheduledEventManager, soundboard::SoundboardSoundManager, stage_instance::StageInstanceManager, sticker::StickerManager, thread::{FetchedThreads, ThreadManager, ThreadMemberManager}, user::UserManager, webhook::WebhookManager, };`

```rust
pub use self::{ application_emoji::ApplicationEmojiManager, auto_moderation::AutoModerationRuleManager, ban::BanManager, channel::{ChannelManager, OverwriteTarget}, command::{CommandManager, GuildCommandManager}, emoji::EmojiManager, guild::GuildManager, guild_channel::GuildChannelManager, invite::InviteManager, member::MemberManager, message::MessageManager, role::RoleManager, scheduled_event::ScheduledEventManager, soundboard::SoundboardSoundManager, stage_instance::StageInstanceManager, sticker::StickerManager, thread::{FetchedThreads, ThreadManager, ThreadMemberManager}, user::UserManager, webhook::WebhookManager, };
```

### `pub use self::gateway::{FetchMembers, GatewayRequestError, GuildSoundboardSounds};`

```rust
pub use self::gateway::{FetchMembers, GatewayRequestError, GuildSoundboardSounds};
```

### `Fetch`

A cached-or-remote fetch: returns the cached value when present, otherwise runs the future and caches the result.

```rust
pub struct Fetch<T, F> { /* private fields */ }
```

#### Methods

##### `force`

Skips the cache and always goes remote.

```rust
pub fn force(mut self) -> Self;
```

##### `is_cached`

Whether this fetch was satisfied without a request.

```rust
pub const fn is_cached(&self) -> bool;
```

### `FetchFuture`

```rust
pub struct FetchFuture<T, F>(State<T, F>);
```

### `Paginator`

Pages through a list endpoint, yielding one cached collection per page.

```rust
pub struct Paginator<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `pages`

Returns a paginated stream over the results.

```rust
pub fn pages(self) -> Self;
```

##### `fetch_all`

Collects every page into one map. Honors rate limits between pages.

```rust
pub async fn fetch_all(mut self) -> Result<Collection<MessageId, Arc<CachedMessage>>, Error>;
```

### `Context`

##### `guilds`

Returns the guild manager.

```rust
pub const fn guilds(&self) -> GuildManager<'_, B>;
```

##### `channels`

Opens the guild's channel manager.

```rust
pub const fn channels(&self) -> ChannelManager<'_, B>;
```

##### `guild_channels`

Returns the guild channel manager.

```rust
pub const fn guild_channels(&self, guild_id: GuildId) -> GuildChannelManager<'_, B>;
```

##### `users`

Lists users interested in an event.

```rust
pub const fn users(&self) -> UserManager<'_, B>;
```

##### `members`

Opens the guild's member manager.

```rust
pub const fn members(&self, guild_id: GuildId) -> MemberManager<'_, B>;
```

##### `messages`

Opens the message manager for the channel.

```rust
pub const fn messages(&self, channel_id: ChannelId) -> MessageManager<'_, B>;
```

##### `roles`

Opens the guild's role manager.

```rust
pub const fn roles(&self, guild_id: GuildId) -> RoleManager<'_, B>;
```

##### `bans`

Opens the guild's ban manager.

```rust
pub const fn bans(&self, guild_id: GuildId) -> BanManager<'_, B>;
```

##### `emojis`

Opens the guild's emoji manager.

```rust
pub const fn emojis(&self, guild_id: GuildId) -> EmojiManager<'_, B>;
```

##### `stickers`

Returns the sticker manager.

```rust
pub const fn stickers(&self, guild_id: GuildId) -> StickerManager<'_, B>;
```

##### `scheduled_events`

Opens the guild's scheduled-event manager.

```rust
pub const fn scheduled_events(&self, guild_id: GuildId) -> ScheduledEventManager<'_, B>;
```

##### `stage_instances`

Returns the stage-instance manager.

```rust
pub const fn stage_instances(&self, guild_id: GuildId) -> StageInstanceManager<'_, B>;
```

##### `soundboard_sounds`

Returns the soundboard-sound manager.

```rust
pub const fn soundboard_sounds(&self, guild_id: GuildId) -> SoundboardSoundManager<'_, B>;
```

##### `threads`

Returns the thread manager.

```rust
pub const fn threads(&self, channel_id: ChannelId) -> ThreadManager<'_, B>;
```

##### `thread_members`

Returns the thread-member manager.

```rust
pub const fn thread_members(&self, thread_id: ChannelId) -> ThreadMemberManager<'_, B>;
```

##### `default_soundboard_sounds`

Returns the default soundboard-sound manager.

```rust
pub async fn default_soundboard_sounds( &self, ) -> Result<Collection<SoundboardSoundId, SoundboardSound>, Error>;
```

##### `commands`

Returns the command manager.

```rust
pub fn commands(&self) -> CommandManager<'_, B>;
```

##### `guild_commands`

Returns the guild command manager.

```rust
pub fn guild_commands(&self, guild_id: GuildId) -> GuildCommandManager<'_, B>;
```

##### `application_emojis`

Returns the application emoji manager.

```rust
pub fn application_emojis(&self) -> ApplicationEmojiManager<'_, B>;
```

##### `auto_moderation_rules`

Returns the auto-moderation manager.

```rust
pub const fn auto_moderation_rules( &self, guild_id: GuildId, ) -> AutoModerationRuleManager<'_, B>;
```

##### `sticker`

Returns the sticker manager.

```rust
pub async fn sticker(&self, sticker_id: StickerId) -> Result<Sticker, Error>;
```

##### `sticker_packs`

Returns the sticker-pack manager.

```rust
pub async fn sticker_packs(&self) -> Result<Collection<StickerPackId, StickerPack>, Error>;
```

##### `sticker_pack`

Returns the sticker pack.

```rust
pub async fn sticker_pack(&self, pack_id: StickerPackId) -> Result<StickerPack, Error>;
```

##### `invites`

Lists the channel's invites.

```rust
pub const fn invites(&self) -> InviteManager<'_, B>;
```

##### `webhooks`

Lists the channel's webhooks.

```rust
pub const fn webhooks(&self) -> WebhookManager<'_, B>;
```

## `manager.application_emoji`

### `ApplicationEmojiManager`

Application-owned emoji: fetch, create, edit, delete.

```rust
pub struct ApplicationEmojiManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `fetch`

Fetches one application emoji.

```rust
pub fn fetch( self, emoji_id: EmojiId, ) -> Fetch<Emoji, impl Future<Output = Result<Emoji, Error>> + Send + 'a>;
```

##### `list`

Lists messages matching the query.

```rust
pub async fn list(self) -> Result<Collection<EmojiId, Emoji>, Error>;
```

##### `create`

Uploads an application emoji.

```rust
pub async fn create(self, body: &CreateApplicationEmoji) -> Result<Emoji, Error>;
```

##### `edit`

Renames an application emoji.

```rust
pub async fn edit( self, emoji_id: EmojiId, body: &ModifyApplicationEmoji, ) -> Result<Emoji, Error>;
```

##### `delete`

Deletes an application emoji.

```rust
pub async fn delete(self, emoji_id: EmojiId) -> Result<(), Error>;
```

## `manager.auto_moderation`

### `AutoModerationRuleManager`

AutoMod rules: fetch, list, create, edit, delete.

```rust
pub struct AutoModerationRuleManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `fetch`

Fetches one rule by id.

```rust
pub fn fetch( self, rule_id: AutoModerationRuleId, ) -> Fetch<Rule, impl Future<Output = Result<Rule, Error>> + Send + 'a>;
```

##### `list`

Lists the guild's rules.

```rust
pub async fn list(self) -> Result<Collection<AutoModerationRuleId, Rule>, Error>;
```

##### `create`

Creates a rule from a builder-style body.

```rust
pub async fn create( self, body: &CreateAutoModerationRule, reason: Option<&str>, ) -> Result<Rule, Error>;
```

##### `edit`

Edits a rule.

```rust
pub async fn edit( self, rule_id: AutoModerationRuleId, body: &ModifyAutoModerationRule, reason: Option<&str>, ) -> Result<Rule, Error>;
```

##### `delete`

Deletes a rule.

```rust
pub async fn delete( self, rule_id: AutoModerationRuleId, reason: Option<&str>, ) -> Result<(), Error>;
```

## `manager.ban`

### `BanManager`

Bans: fetch one, list all, ban, unban.

```rust
pub struct BanManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `fetch`

Fetches one ban entry.

```rust
pub fn fetch( self, user_id: UserId, ) -> Fetch<Ban, impl Future<Output = Result<Ban, Error>> + Send + 'a>;
```

##### `list`

Lists banned users.

```rust
pub async fn list(self, query: &GetGuildBans) -> Result<Collection<UserId, Ban>, Error>;
```

##### `create`

Bans a user with optional message pruning.

```rust
pub async fn create( self, user_id: UserId, body: &CreateGuildBan, reason: Option<&str>, ) -> Result<(), Error>;
```

##### `remove`

Removes a user from the thread.

```rust
pub async fn remove(self, user_id: UserId, reason: Option<&str>) -> Result<(), Error>;
```

## `manager.channel`

### `ChannelManager`

Channel-scoped work: fetch, edit, delete, voice state and permissions.

```rust
pub struct ChannelManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `cache`

Reads the cached channel without a request.

```rust
pub fn cache(self) -> Option<CacheView<'a, ChannelId, Channel, B>>;
```

##### `fetch`

Fetches a channel by id, cached first.

```rust
pub fn fetch( self, channel_id: ChannelId, ) -> Fetch<Arc<Channel>, impl Future<Output = Result<Arc<Channel>, Error>> + Send + 'a>;
```

##### `send_typing`

Broadcasts a typing indicator.

```rust
pub async fn send_typing(self, channel_id: ChannelId) -> Result<(), Error>;
```

##### `edit`

Edits a channel (name, topic, rate limits, forum layout).

```rust
pub async fn edit( self, channel_id: ChannelId, body: &ModifyGuildChannel, reason: Option<&str>, ) -> Result<Arc<Channel>, Error>;
```

##### `edit_thread`

Edits thread.

```rust
pub async fn edit_thread( self, thread_id: ChannelId, body: &ModifyThread, reason: Option<&str>, ) -> Result<Arc<Channel>, Error>;
```

##### `delete`

Deletes the channel, with an optional audit reason.

```rust
pub async fn delete( self, channel_id: ChannelId, reason: Option<&str>, ) -> Result<Channel, Error>;
```

##### `send_soundboard_sound`

Plays a soundboard sound in a voice channel.

```rust
pub async fn send_soundboard_sound( self, channel_id: ChannelId, body: &SendSoundboardSound, ) -> Result<(), Error>;
```

##### `set_permissions`

Sets a permission overwrite for a role or member.

```rust
pub fn set_permissions( self, channel_id: ChannelId, target: impl Into<OverwriteTarget>, allow: Permissions, deny: Permissions, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `delete_permissions`

Removes a permission overwrite by id.

```rust
pub fn delete_permissions( self, channel_id: ChannelId, target: impl Into<OverwriteTarget>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

### `OverwriteTarget`

Possible overwrite target values.

```rust
pub enum OverwriteTarget
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Role` | `RoleId` | Role. |
| `Member` | `UserId` | Member. |

## `manager.command`

### `CommandManager`

Global application commands: fetch, create, edit, delete, bulk overwrite.

```rust
pub struct CommandManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `application_id`

The bot's application id, known after the first READY.

```rust
pub const fn application_id(self) -> ApplicationId;
```

##### `fetch`

Fetches one global command.

```rust
pub fn fetch( self, command_id: CommandId, ) -> Fetch< ApplicationCommand, impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a, >;
```

##### `list`

Lists global commands.

```rust
pub async fn list( self, query: &GetApplicationCommands, ) -> Result<Collection<CommandId, ApplicationCommand>, Error>;
```

##### `create`

Creates a global command from a builder.

```rust
pub async fn create( self, body: &CreateApplicationCommand, ) -> Result<ApplicationCommand, Error>;
```

##### `edit`

Edits a global command.

```rust
pub async fn edit( self, command_id: CommandId, body: &EditApplicationCommand, ) -> Result<ApplicationCommand, Error>;
```

##### `delete`

Deletes a global command.

```rust
pub async fn delete(self, command_id: CommandId) -> Result<(), Error>;
```

##### `set`

Bulk-overwrites every global command. Unlisted commands are deleted.

```rust
pub async fn set( self, commands: &[CreateApplicationCommand], ) -> Result<Collection<CommandId, ApplicationCommand>, Error>;
```

### `GuildCommandManager`

Per-guild commands plus their permission overwrites.

```rust
pub struct GuildCommandManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `fetch`

Fetches one guild command.

```rust
pub fn fetch( self, command_id: CommandId, ) -> Fetch< ApplicationCommand, impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a, >;
```

##### `list`

Lists the guild's commands.

```rust
pub async fn list( self, query: &GetApplicationCommands, ) -> Result<Collection<CommandId, ApplicationCommand>, Error>;
```

##### `create`

Creates a guild command from a builder.

```rust
pub async fn create( self, body: &CreateApplicationCommand, ) -> Result<ApplicationCommand, Error>;
```

##### `edit`

Edits a guild command.

```rust
pub async fn edit( self, command_id: CommandId, body: &EditApplicationCommand, ) -> Result<ApplicationCommand, Error>;
```

##### `delete`

Deletes a guild command.

```rust
pub async fn delete(self, command_id: CommandId) -> Result<(), Error>;
```

##### `set`

Bulk-overwrites the guild's commands.

```rust
pub async fn set( self, commands: &[CreateApplicationCommand], ) -> Result<Collection<CommandId, ApplicationCommand>, Error>;
```

##### `permissions`

The member's guild-wide permissions.

```rust
pub async fn permissions( self, ) -> Result<Collection<GenericId, GuildCommandPermissions>, Error>;
```

##### `command_permissions`

Reads who may use a command.

```rust
pub async fn command_permissions( self, command_id: CommandId, ) -> Result<GuildCommandPermissions, Error>;
```

## `manager.emoji`

### `EmojiManager`

Guild emoji: fetch, list, create, edit, delete.

```rust
pub struct EmojiManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, EmojiId, Emoji, B>>;
```

##### `fetch`

Fetches one emoji by id, cached first.

```rust
pub fn fetch( self, emoji_id: EmojiId, ) -> Fetch<Arc<Emoji>, impl Future<Output = Result<Arc<Emoji>, Error>> + Send + 'a>;
```

##### `list`

Lists the guild's emoji.

```rust
pub async fn list(self) -> Result<Collection<EmojiId, Arc<Emoji>>, Error>;
```

##### `create`

Uploads a new emoji from image data.

```rust
pub async fn create( self, body: &CreateGuildEmoji, reason: Option<&str>, ) -> Result<Arc<Emoji>, Error>;
```

##### `edit`

Renames an emoji.

```rust
pub async fn edit( self, emoji_id: EmojiId, body: &ModifyGuildEmoji, reason: Option<&str>, ) -> Result<Arc<Emoji>, Error>;
```

##### `delete`

Deletes an emoji.

```rust
pub async fn delete(self, emoji_id: EmojiId, reason: Option<&str>) -> Result<(), Error>;
```

## `manager.gateway`

### `GatewayRequestError`

A member chunk request that failed: invalid query, timeout, or disconnect.

```rust
pub enum GatewayRequestError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ShardNotRunning` | `GuildId` | Shard Not Running. |
| `Send` | `#[from] SendError` | Send. |
| `Timeout` | `,` | Timeout. |
| `RateLimited` | `retry_after: Duration` | Rate Limited. |
| `Ended` | `EndReason` | Ended. |

### `FetchMembers`

A configurable guild-member chunk request: everyone, a query, or a user list.

```rust
pub struct FetchMembers<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `limit`

Caps how many members come back.

```rust
pub const fn limit(mut self, limit: u32) -> Self;
```

##### `presences`

Includes presences in the response.

```rust
pub const fn presences(mut self, presences: bool) -> Self;
```

##### `time`

Gives up after this long instead of waiting forever.

```rust
pub const fn time(mut self, time: Duration) -> Self;
```

### `GuildSoundboardSounds`

```rust
pub type GuildSoundboardSounds = Collection<GuildId, Collection<SoundboardSoundId, Arc<SoundboardSound>>>;
```

## `manager.guild`

### `GuildManager`

Guild-scoped work: fetch, edit, leave, audit log, integrations.

```rust
pub struct GuildManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `cache`

Reads the cached guild without a request.

```rust
pub fn cache(self) -> Option<CacheView<'a, GuildId, CachedGuild, B>>;
```

##### `fetch_soundboard_sounds`

Lists the guild's cached-and-remote soundboard sounds.

```rust
pub async fn fetch_soundboard_sounds( self, guild_ids: impl IntoIterator<Item = GuildId>, time: Duration, ) -> Result<GuildSoundboardSounds, GatewayRequestError>;
```

##### `fetch`

Fetches a guild by id, cached first.

```rust
pub fn fetch( self, guild_id: GuildId, ) -> Fetch<Arc<CachedGuild>, impl Future<Output = Result<Arc<CachedGuild>, Error>> + Send + 'a>;
```

##### `edit`

Edits the guild's settings.

```rust
pub async fn edit( self, guild_id: GuildId, body: &ModifyGuild, reason: Option<&str>, ) -> Result<Arc<CachedGuild>, Error>;
```

##### `leave`

Leaves the guild.

```rust
pub async fn leave(self, guild_id: GuildId) -> Result<(), Error>;
```

##### `audit_log`

Reads the audit log with filtering.

```rust
pub async fn audit_log( self, guild_id: GuildId, query: &GetGuildAuditLog, ) -> Result<AuditLog, Error>;
```

##### `integrations`

Lists the guild's integrations.

```rust
pub async fn integrations( self, guild_id: GuildId, ) -> Result<Collection<IntegrationId, Integration>, Error>;
```

##### `delete_integration`

Deletes an integration by id.

```rust
pub async fn delete_integration( self, guild_id: GuildId, integration_id: IntegrationId, reason: Option<&str>, ) -> Result<(), Error>;
```

## `manager.guild_channel`

### `GuildChannelManager`

Guild-level channel work: list, create, reposition, active threads.

```rust
pub struct GuildChannelManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `list`

Lists the guild's channels.

```rust
pub async fn list(self) -> Result<Collection<ChannelId, Arc<Channel>>, Error>;
```

##### `create`

Creates a channel from a builder.

```rust
pub async fn create( self, body: &CreateGuildChannel, reason: Option<&str>, ) -> Result<Arc<Channel>, Error>;
```

##### `set_positions`

Bulk-moves channels to new positions.

```rust
pub async fn set_positions( self, positions: &[ModifyGuildChannelPosition], reason: Option<&str>, ) -> Result<(), Error>;
```

##### `list_active_threads`

Lists the guild's currently active threads.

```rust
pub async fn list_active_threads(self) -> Result<FetchedThreads, Error>;
```

## `manager.invite`

### `InviteManager`

Invites: fetch, create, list per channel or guild, delete.

```rust
pub struct InviteManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `fetch`

Fetches an invite by code, cached first.

```rust
pub fn fetch<'r>( self, code: &'r str, query: &'r GetInvite, ) -> Fetch<Invite, impl Future<Output = Result<Invite, Error>> + Send + 'r> where 'a: 'r,;
```

##### `delete`

Deletes an invite by code.

```rust
pub async fn delete(self, code: &str, reason: Option<&str>) -> Result<Invite, Error>;
```

##### `create`

Creates an invite for a channel.

```rust
pub async fn create( self, channel_id: ChannelId, body: &CreateChannelInvite, reason: Option<&str>, ) -> Result<InviteWithMetadata, Error>;
```

##### `list_channel`

Lists a channel's invites.

```rust
pub async fn list_channel( self, channel_id: ChannelId, ) -> Result<Collection<String, InviteWithMetadata>, Error>;
```

##### `list_guild`

Lists a guild's invites.

```rust
pub async fn list_guild( self, guild_id: GuildId, ) -> Result<Collection<String, InviteWithMetadata>, Error>;
```

## `manager.member`

### `MemberManager`

Guild members: fetch one, fetch many, edit, timeout, kick-adjacent work.

```rust
pub struct MemberManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, UserId, CachedMember, B>>;
```

##### `fetch`

Fetches one member by id, cached first.

```rust
pub fn fetch( self, user_id: UserId, ) -> Fetch<Arc<CachedMember>, impl Future<Output = Result<Arc<CachedMember>, Error>> + Send + 'a>;
```

##### `fetch_all`

Pages through every member in the guild.

```rust
pub fn fetch_all(self) -> FetchMembers<'a, B>;
```

##### `fetch_by_query`

Searches members by username prefix.

```rust
pub fn fetch_by_query(self, query: impl Into<String>) -> FetchMembers<'a, B>;
```

##### `fetch_many`

Fetches a page of members with list options.

```rust
pub fn fetch_many(self, user_ids: impl IntoIterator<Item = UserId>) -> FetchMembers<'a, B>;
```

##### `list`

Lists members with pagination options.

```rust
pub async fn list( self, query: &ListGuildMembers, ) -> Result<Collection<UserId, Arc<CachedMember>>, Error>;
```

##### `edit`

Edits a member (nick, roles, mute, timeout).

```rust
pub async fn edit( self, user_id: UserId, body: &ModifyGuildMember, reason: Option<&str>, ) -> Result<Arc<CachedMember>, Error>;
```

##### `timeout`

Times a member out until the given timestamp, or clears it.

```rust
pub async fn timeout( self, user_id: UserId, until: Option<Timestamp>, reason: Option<&str>, ) -> Result<Arc<CachedMember>, Error>;
```

##### `set_nickname`

Sets a member's nickname, or clears it with None.

```rust
pub async fn set_nickname( self, user_id: UserId, nickname: Option<&str>, reason: Option<&str>, ) -> Result<Arc<CachedMember>, Error>;
```

##### `add_role`

Adds a role to the member.

```rust
pub async fn add_role( self, user_id: UserId, role_id: RoleId, reason: Option<&str>, ) -> Result<(), Error>;
```

##### `remove_role`

Removes a role from the member.

```rust
pub async fn remove_role( self, user_id: UserId, role_id: RoleId, reason: Option<&str>, ) -> Result<(), Error>;
```

##### `kick`

Kicks the member with an optional audit reason.

```rust
pub async fn kick(self, user_id: UserId, reason: Option<&str>) -> Result<(), Error>;
```

##### `ban`

Bans the member with optional pruning and reason.

```rust
pub async fn ban( self, user_id: UserId, body: &CreateGuildBan, reason: Option<&str>, ) -> Result<(), Error>;
```

##### `unban`

Lifts the ban.

```rust
pub async fn unban(self, user_id: UserId, reason: Option<&str>) -> Result<(), Error>;
```

##### `search`

Searches for matching objects.

```rust
pub async fn search( self, query: &SearchGuildMembers, ) -> Result<Collection<UserId, Arc<CachedMember>>, Error>;
```

##### `edit_me`

Edits me.

```rust
pub async fn edit_me( self, body: &ModifyCurrentMember, reason: Option<&str>, ) -> Result<Arc<CachedMember>, Error>;
```

##### `prune_count`

Counts members who would be pruned.

```rust
pub async fn prune_count(self, query: &GetGuildPruneCount) -> Result<u32, Error>;
```

##### `prune`

Kicks members who have been inactive for the given days.

```rust
pub async fn prune( self, body: &BeginGuildPrune, reason: Option<&str>, ) -> Result<Option<u32>, Error>;
```

## `manager.message`

### `MessageManager`

Everything about one channel's messages: send, fetch, edit, delete, react, pin, thread.

```rust
pub struct MessageManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `channel_id`

Returns the ID of the channel.

```rust
pub const fn channel_id(self) -> ChannelId;
```

##### `cache`

Reads the cached messages for this channel without a request.

```rust
pub fn cache(self) -> Option<ChannelView<'a, MessageId, CachedMessage, B>>;
```

##### `fetch`

Fetches one message by id, cached first.

```rust
pub fn fetch( self, message_id: MessageId, ) -> Fetch< Arc<CachedMessage>, impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 'a, >;
```

##### `list`

Lists messages matching the query.

```rust
pub async fn list( self, query: &GetChannelMessages, ) -> Result<Collection<MessageId, Arc<CachedMessage>>, Error>;
```

##### `paginate`

Streams the channel history page by page.

```rust
pub fn paginate(self, query: GetChannelMessages) -> Paginator<'a, B>;
```

##### `send`

Sends a message to the channel.

```rust
pub fn send( self, body: impl Into<CreateMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 'a;
```

##### `edit`

Edits a message by id.

```rust
pub async fn edit( self, message_id: MessageId, body: &EditMessage, ) -> Result<Arc<CachedMessage>, Error>;
```

##### `delete`

Deletes a message by id.

```rust
pub async fn delete(self, message_id: MessageId) -> Result<(), Error>;
```

##### `bulk_delete`

Bulk-deletes up to 100 messages younger than two weeks.

```rust
pub async fn bulk_delete( self, message_ids: Vec<MessageId>, reason: Option<&str>, ) -> Result<(), Error>;
```

##### `react`

Adds a reaction to a message.

```rust
pub fn react<'e>( self, message_id: MessageId, emoji: impl Into<ReactionEmoji<'e>>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `pin`

Pins a message.

```rust
pub async fn pin(self, message_id: MessageId, reason: Option<&str>) -> Result<(), Error>;
```

##### `unpin`

Unpins a message.

```rust
pub async fn unpin(self, message_id: MessageId, reason: Option<&str>) -> Result<(), Error>;
```

##### `start_thread`

Starts a thread on a message.

```rust
pub async fn start_thread( self, message_id: MessageId, body: &StartThreadFromMessage, reason: Option<&str>, ) -> Result<Arc<Channel>, Error>;
```

##### `crosspost`

Publishes a news-channel message to its followers.

```rust
pub async fn crosspost(self, message_id: MessageId) -> Result<Arc<CachedMessage>, Error>;
```

##### `reaction_users`

Lists who reacted with an emoji, paginated.

```rust
pub async fn reaction_users( self, message_id: MessageId, emoji: ReactionEmoji<'_>, query: &GetReactions, ) -> Result<Collection<UserId, Arc<CachedUser>>, Error>;
```

##### `remove_reaction`

Removes one user's reaction, or the bot's own.

```rust
pub fn remove_reaction<'e>( self, message_id: MessageId, emoji: impl Into<ReactionEmoji<'e>>, user: Option<UserId>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `remove_emoji_reactions`

Removes emoji reactions.

```rust
pub fn remove_emoji_reactions<'e>( self, message_id: MessageId, emoji: impl Into<ReactionEmoji<'e>>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `remove_all_reactions`

Clears every reaction from a message.

```rust
pub async fn remove_all_reactions(self, message_id: MessageId) -> Result<(), Error>;
```

## `manager.role`

### `RoleManager`

Guild roles: fetch, create, edit, delete, reposition.

```rust
pub struct RoleManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, RoleId, Role, B>>;
```

##### `fetch`

Fetches one role by id, cached first.

```rust
pub fn fetch( self, role_id: RoleId, ) -> Fetch<Arc<Role>, impl Future<Output = Result<Arc<Role>, Error>> + Send + 'a>;
```

##### `list`

Lists the guild's roles.

```rust
pub async fn list(self) -> Result<Collection<RoleId, Arc<Role>>, Error>;
```

##### `create`

Creates a role.

```rust
pub async fn create( self, body: &CreateGuildRole, reason: Option<&str>, ) -> Result<Arc<Role>, Error>;
```

##### `edit`

Edits a role (name, color, permissions, icon).

```rust
pub async fn edit( self, role_id: RoleId, body: &ModifyGuildRole, reason: Option<&str>, ) -> Result<Arc<Role>, Error>;
```

##### `delete`

Deletes a role by id.

```rust
pub async fn delete(self, role_id: RoleId, reason: Option<&str>) -> Result<(), Error>;
```

##### `set_positions`

Bulk-moves roles to new positions.

```rust
pub async fn set_positions( self, positions: &[ModifyGuildRolePosition], reason: Option<&str>, ) -> Result<Collection<RoleId, Arc<Role>>, Error>;
```

## `manager.scheduled_event`

### `ScheduledEventManager`

Scheduled events: fetch, list, create, edit, delete, subscribers.

```rust
pub struct ScheduledEventManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, ScheduledEventId, GuildScheduledEvent, B>>;
```

##### `fetch`

Fetches one scheduled event, cached first.

```rust
pub fn fetch( self, event_id: ScheduledEventId, ) -> Fetch< Arc<GuildScheduledEvent>, impl Future<Output = Result<Arc<GuildScheduledEvent>, Error>> + Send + 'a, >;
```

##### `list`

Lists the guild's scheduled events.

```rust
pub async fn list( self, ) -> Result<Collection<ScheduledEventId, Arc<GuildScheduledEvent>>, Error>;
```

##### `create`

Creates a scheduled event.

```rust
pub async fn create( self, body: &CreateGuildScheduledEvent, reason: Option<&str>, ) -> Result<Arc<GuildScheduledEvent>, Error>;
```

##### `edit`

Edits a scheduled event.

```rust
pub async fn edit( self, event_id: ScheduledEventId, body: &ModifyGuildScheduledEvent, reason: Option<&str>, ) -> Result<Arc<GuildScheduledEvent>, Error>;
```

##### `delete`

Deletes a scheduled event.

```rust
pub async fn delete(self, event_id: ScheduledEventId) -> Result<(), Error>;
```

##### `users`

Lists users interested in an event.

```rust
pub async fn users( self, event_id: ScheduledEventId, query: &GetGuildScheduledEventUsers, ) -> Result<Collection<UserId, GuildScheduledEventUser>, Error>;
```

## `manager.soundboard`

### `SoundboardSoundManager`

Guild soundboard sounds: fetch, list, create, edit, delete.

```rust
pub struct SoundboardSoundManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, SoundboardSoundId, SoundboardSound, B>>;
```

##### `fetch`

Fetches one sound by id, cached first.

```rust
pub fn fetch( self, sound_id: SoundboardSoundId, ) -> Fetch< Arc<SoundboardSound>, impl Future<Output = Result<Arc<SoundboardSound>, Error>> + Send + 'a, >;
```

##### `list`

Lists the guild's soundboard sounds.

```rust
pub async fn list(self) -> Result<Collection<SoundboardSoundId, Arc<SoundboardSound>>, Error>;
```

##### `create`

Uploads a soundboard sound.

```rust
pub async fn create( self, body: &CreateGuildSoundboardSound, reason: Option<&str>, ) -> Result<Arc<SoundboardSound>, Error>;
```

##### `edit`

Renames a soundboard sound.

```rust
pub async fn edit( self, sound_id: SoundboardSoundId, body: &ModifyGuildSoundboardSound, reason: Option<&str>, ) -> Result<Arc<SoundboardSound>, Error>;
```

##### `delete`

Deletes a soundboard sound.

```rust
pub async fn delete( self, sound_id: SoundboardSoundId, reason: Option<&str>, ) -> Result<(), Error>;
```

## `manager.stage_instance`

### `StageInstanceManager`

Stage instances: fetch, create, edit, delete.

```rust
pub struct StageInstanceManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, StageInstanceId, StageInstance, B>>;
```

##### `fetch`

Fetches the stage instance of a channel.

```rust
pub fn fetch( self, channel_id: ChannelId, ) -> Fetch< Arc<StageInstance>, impl Future<Output = Result<Arc<StageInstance>, Error>> + Send + 'a, >;
```

##### `create`

Opens a stage instance on a stage channel.

```rust
pub async fn create( self, body: &CreateStageInstance, reason: Option<&str>, ) -> Result<Arc<StageInstance>, Error>;
```

##### `edit`

Edits the topic or privacy of a stage.

```rust
pub async fn edit( self, channel_id: ChannelId, body: &ModifyStageInstance, reason: Option<&str>, ) -> Result<Arc<StageInstance>, Error>;
```

##### `delete`

Closes the stage instance.

```rust
pub async fn delete(self, channel_id: ChannelId, reason: Option<&str>) -> Result<(), Error>;
```

## `manager.sticker`

### `StickerManager`

Guild stickers: fetch, list, create, edit, delete.

```rust
pub struct StickerManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> GuildId;
```

##### `cache`

The cache, when one is configured.

```rust
pub fn cache(self) -> Option<GuildView<'a, StickerId, Sticker, B>>;
```

##### `fetch`

Fetches one sticker by id, cached first.

```rust
pub fn fetch( self, sticker_id: StickerId, ) -> Fetch<Arc<Sticker>, impl Future<Output = Result<Arc<Sticker>, Error>> + Send + 'a>;
```

##### `list`

Lists the guild's stickers.

```rust
pub async fn list(self) -> Result<Collection<StickerId, Arc<Sticker>>, Error>;
```

##### `create`

Uploads a sticker from a file.

```rust
pub async fn create( self, body: &CreateGuildSticker, reason: Option<&str>, ) -> Result<Arc<Sticker>, Error>;
```

##### `edit`

Edits a sticker's name, description and tags.

```rust
pub async fn edit( self, sticker_id: StickerId, body: &ModifyGuildSticker, reason: Option<&str>, ) -> Result<Arc<Sticker>, Error>;
```

##### `delete`

Deletes a sticker.

```rust
pub async fn delete(self, sticker_id: StickerId, reason: Option<&str>) -> Result<(), Error>;
```

## `manager.thread`

### `FetchedThreads`

One page of archived threads plus whether more pages exist.

```rust
pub struct FetchedThreads
```

| Field | Type | Description |
| --- | --- | --- |
| `threads` | `Collection<ChannelId, Arc<Channel>>` | The threads. |
| `members` | `Vec<ThreadMember>` | The members. |
| `has_more` | `bool` | Whether there is more. |


### `ThreadManager`

Threads of one channel: create, list archived, manage members.

```rust
pub struct ThreadManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `channel_id`

Returns the ID of the channel.

```rust
pub const fn channel_id(self) -> ChannelId;
```

##### `create`

Starts a thread without an existing message.

```rust
pub async fn create( self, body: &StartThreadWithoutMessage, reason: Option<&str>, ) -> Result<Arc<Channel>, Error>;
```

##### `create_in_forum`

Starts a forum or media thread with an opening message.

```rust
pub async fn create_in_forum( self, body: &StartThreadInForumOrMediaChannel, reason: Option<&str>, ) -> Result<Arc<Channel>, Error>;
```

##### `list_public_archived`

Lists public archived threads.

```rust
pub async fn list_public_archived( self, query: &ListArchivedThreads, ) -> Result<FetchedThreads, Error>;
```

##### `list_private_archived`

Lists private archived threads the bot can see.

```rust
pub async fn list_private_archived( self, query: &ListArchivedThreads, ) -> Result<FetchedThreads, Error>;
```

##### `list_joined_private_archived`

Lists private archived threads the bot joined.

```rust
pub async fn list_joined_private_archived( self, query: &ListJoinedPrivateArchivedThreads, ) -> Result<FetchedThreads, Error>;
```

### `ThreadMemberManager`

Who is in a thread: fetch, list, add, remove, join, leave.

```rust
pub struct ThreadMemberManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `thread_id`

The thread this manager is scoped to.

```rust
pub const fn thread_id(self) -> ChannelId;
```

##### `fetch`

Fetches one thread member, cached first.

```rust
pub fn fetch<'r>( self, user_id: UserId, query: &'r GetThreadMember, ) -> Fetch<ThreadMember, impl Future<Output = Result<ThreadMember, Error>> + Send + 'r> where 'a: 'r,;
```

##### `list`

Lists the thread's members, optionally with presences.

```rust
pub async fn list( self, query: &ListThreadMembers, ) -> Result<Collection<UserId, ThreadMember>, Error>;
```

##### `add`

Adds a user to the thread.

```rust
pub async fn add(self, user_id: UserId) -> Result<(), Error>;
```

##### `remove`

Removes a user from the thread.

```rust
pub async fn remove(self, user_id: UserId) -> Result<(), Error>;
```

##### `join`

Joins the thread as the bot.

```rust
pub async fn join(self) -> Result<(), Error>;
```

##### `leave`

Leaves the thread.

```rust
pub async fn leave(self) -> Result<(), Error>;
```

## `manager.user`

### `UserManager`

Users and DMs: fetch, open DM channels, send without a guild.

```rust
pub struct UserManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `cache`

Reads the cached user without a request.

```rust
pub fn cache(self) -> Option<CacheView<'a, UserId, CachedUser, B>>;
```

##### `fetch`

Fetches a user by id, cached first.

```rust
pub fn fetch( self, user_id: UserId, ) -> Fetch<Arc<CachedUser>, impl Future<Output = Result<Arc<CachedUser>, Error>> + Send + 'a>;
```

##### `create_dm`

Opens (or reuses) a DM channel with a user.

```rust
pub async fn create_dm(self, user_id: UserId) -> Result<Arc<Channel>, Error>;
```

##### `send`

Sends a DM to a user, creating the channel if needed.

```rust
pub fn send( self, user_id: UserId, body: impl Into<CreateMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 'a;
```

## `manager.webhook`

### `WebhookManager`

Webhooks: fetch, create, list, send, edit and delete messages through them.

```rust
pub struct WebhookManager<'a, B: Backend> { /* private fields */ }
```

#### Methods

##### `fetch`

Fetches a webhook by id.

```rust
pub fn fetch( self, webhook_id: WebhookId, ) -> Fetch<Webhook, impl Future<Output = Result<Webhook, Error>> + Send + 'a>;
```

##### `fetch_with_token`

Fetches a webhook by id and token, no bot token needed.

```rust
pub async fn fetch_with_token( self, webhook_id: WebhookId, token: &str, ) -> Result<Webhook, Error>;
```

##### `create`

Creates a webhook in a channel.

```rust
pub async fn create( self, channel_id: ChannelId, body: &CreateWebhook, reason: Option<&str>, ) -> Result<Webhook, Error>;
```

##### `list_channel`

Lists a channel's webhooks.

```rust
pub async fn list_channel( self, channel_id: ChannelId, ) -> Result<Collection<WebhookId, Webhook>, Error>;
```

##### `list_guild`

Lists a guild's webhooks.

```rust
pub async fn list_guild( self, guild_id: GuildId, ) -> Result<Collection<WebhookId, Webhook>, Error>;
```

##### `edit`

Edits the webhook (name, avatar, channel).

```rust
pub async fn edit( self, webhook_id: WebhookId, body: &ModifyWebhook, reason: Option<&str>, ) -> Result<Webhook, Error>;
```

##### `delete`

Deletes the webhook.

```rust
pub async fn delete(self, webhook_id: WebhookId, reason: Option<&str>) -> Result<(), Error>;
```

##### `send`

Sends through the webhook without a token-authenticated client.

```rust
pub fn send<'t>( self, webhook_id: WebhookId, token: &'t str, body: impl Into<ExecuteWebhook>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send + 't where 'a: 't,;
```

##### `fetch_message`

Fetches a message previously sent through the webhook.

```rust
pub async fn fetch_message( self, webhook_id: WebhookId, token: &str, message_id: MessageId, thread: Option<ChannelId>, ) -> Result<Arc<CachedMessage>, Error>;
```

##### `edit_message`

Edits a message previously sent through the webhook.

```rust
pub async fn edit_message( self, webhook_id: WebhookId, token: &str, message_id: MessageId, body: &EditWebhookMessage, ) -> Result<Arc<CachedMessage>, Error>;
```

##### `delete_message`

Deletes a previously sent webhook message.

```rust
pub async fn delete_message( self, webhook_id: WebhookId, token: &str, message_id: MessageId, thread: Option<ChannelId>, ) -> Result<(), Error>;
```

