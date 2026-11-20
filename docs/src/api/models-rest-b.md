# REST models: I to W

Request and response shapes for the REST endpoints, part two.

## `model.rest.interaction`

### `CreateFollowupMessage`

Request body for the matching REST call.

```rust
pub struct CreateFollowupMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `Option<CompactString>` | The message text. |
| `tts` | `bool` | The tts. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `allowed_mentions` | `Option<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `attachments` | `Option<Vec<AttachmentRequest>>` | The files attached to the message. |
| `flags` | `Option<MessageFlags>` | Bitfield of flags attached to the object. |
| `poll` | `Option<CreatePoll>` | The poll attached to the message. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |

## `model.rest.invite`

### `GetInvite`

Query parameters for the matching REST call.

```rust
pub struct GetInvite
```

| Field | Type | Description |
| --- | --- | --- |
| `with_counts` | `Option<bool>` | The with counts. |
| `guild_scheduled_event_id` | `Option<ScheduledEventId>` | The ID of the scheduled event. |
| `with_expiration` | `Option<bool>` | The with expiration. |

### `BulkTargetUsers`

Request body for the matching REST call.

```rust
pub struct BulkTargetUsers
```

| Field | Type | Description |
| --- | --- | --- |
| `user_ids` | `Vec<UserId>` | The IDs of the user. |

#### Methods

##### `new`

Creates a new `BulkTargetUsers`.

```rust
pub fn new(user_ids: impl IntoIterator<Item = UserId>) -> Self;
```

## `model.rest.lobby`

### `LobbyMetadata`

Request or response shape for the matching REST call.

```rust
pub type LobbyMetadata = BTreeMap<CompactString, CompactString>;
```

### `CreateLobby`

Request body for the matching REST call.

```rust
pub struct CreateLobby
```

| Field | Type | Description |
| --- | --- | --- |
| `metadata` | `LobbyMetadata` | Additional structured metadata. |
| `members` | `Vec<LobbyMemberData>` | The members in this collection. |
| `idle_timeout_seconds` | `Option<u32>` | How long to wait for activity before ending. |

### `LobbyMemberData`

Response shape for the matching REST call.

```rust
pub struct LobbyMemberData
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `UserId` | The object's snowflake ID. |
| `metadata` | `Option<LobbyMetadata>` | Additional structured metadata. |
| `flags` | `Option<LobbyMemberFlags>` | Bitfield of flags attached to the object. |
| `additional_name` | `Patch<CompactString>` | The additional name. |

#### Methods

##### `new`

Creates a new `LobbyMemberData`.

```rust
pub const fn new(id: UserId) -> Self;
```

### `CreateOrJoinLobby`

Request body for the matching REST call.

```rust
pub struct CreateOrJoinLobby
```

| Field | Type | Description |
| --- | --- | --- |
| `secret` | `CompactString` | The secret. |
| `idle_timeout_seconds` | `Option<u32>` | How long to wait for activity before ending. |
| `lobby_metadata` | `Option<LobbyMetadata>` | The lobby metadata. |
| `member_metadata` | `Option<LobbyMetadata>` | The member metadata. |

#### Methods

##### `new`

Creates a new `CreateOrJoinLobby`.

```rust
pub fn new(secret: impl Into<CompactString>) -> Self;
```

### `ModifyLobby`

Request body for the matching REST call.

```rust
pub struct ModifyLobby
```

| Field | Type | Description |
| --- | --- | --- |
| `metadata` | `Option<LobbyMetadata>` | Additional structured metadata. |
| `members` | `Option<Vec<LobbyMemberData>>` | The members in this collection. |
| `idle_timeout_seconds` | `Option<u32>` | How long to wait for activity before ending. |

### `AddLobbyMember`

Request body for the matching REST call.

```rust
pub struct AddLobbyMember
```

| Field | Type | Description |
| --- | --- | --- |
| `metadata` | `Option<LobbyMetadata>` | Additional structured metadata. |
| `flags` | `Option<LobbyMemberFlags>` | Bitfield of flags attached to the object. |
| `additional_name` | `Patch<CompactString>` | The additional name. |

### `BulkLobbyMember`

Request body for the matching REST call.

```rust
pub struct BulkLobbyMember
```

| Field | Type | Description |
| --- | --- | --- |
| `member` | `LobbyMemberData` | The guild member this refers to. |
| `remove_member` | `bool` | The remove member. |

#### Methods

##### `upsert`

Adds or updates one member in the bulk request.

```rust
pub const fn upsert(member: LobbyMemberData) -> Self;
```

##### `remove`

Removes a value by key and returns it when present.

```rust
pub const fn remove(id: UserId) -> Self;
```

### `SendLobbyMessage`

Request or response shape for the matching REST call.

```rust
pub struct SendLobbyMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `CompactString` | The message text. |
| `metadata` | `LobbyMetadata` | Additional structured metadata. |
| `flags` | `Option<MessageFlags>` | Bitfield of flags attached to the object. |

#### Methods

##### `new`

Creates a new `SendLobbyMessage`.

```rust
pub fn new(content: impl Into<CompactString>) -> Self;
```

### `GetLobbyMessages`

Query parameters for the matching REST call.

```rust
pub struct GetLobbyMessages
```

| Field | Type | Description |
| --- | --- | --- |
| `limit` | `Option<u8>` | The maximum number of items to return. |

## `model.rest.message`

### `CreateMessage`

Request body for the matching REST call.

```rust
pub struct CreateMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `Option<CompactString>` | The message text. |
| `nonce` | `Option<Nonce>` | Sets the nonce used for message deduplication. |
| `tts` | `bool` | The tts. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `allowed_mentions` | `Option<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `message_reference` | `Option<MessageReference>` | Sets the message reference for a forward. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `sticker_ids` | `Vec<StickerId>` | The IDs of the sticker. |
| `attachments` | `Option<Vec<AttachmentRequest>>` | The files attached to the message. |
| `flags` | `Option<MessageFlags>` | Bitfield of flags attached to the object. |
| `enforce_nonce` | `bool` | The enforce nonce. |
| `poll` | `Option<CreatePoll>` | The poll attached to the message. |
| `shared_client_theme` | `Option<SharedClientTheme>` | Sets the shared client theme of the message. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |

#### Methods

##### `reply_to`

Replies to the given message.

```rust
pub const fn reply_to(mut self, message_id: MessageId) -> Self;
```

##### `message_reference`

Sets the message reference for a forward.

```rust
pub fn message_reference(mut self, reference: MessageReference) -> Self;
```

##### `sticker`

Returns the sticker manager.

```rust
pub fn sticker(mut self, sticker_id: StickerId) -> Self;
```

##### `stickers`

Returns the sticker manager.

```rust
pub fn stickers(mut self, sticker_ids: impl IntoIterator<Item = StickerId>) -> Self;
```

##### `nonce`

Sets the nonce used for message deduplication.

```rust
pub fn nonce(mut self, nonce: Nonce, enforce: bool) -> Self;
```

##### `shared_client_theme`

Sets the shared client theme of the message.

```rust
pub fn shared_client_theme(mut self, theme: SharedClientTheme) -> Self;
```

### `EditMessage`

Request body for the matching REST call.

```rust
pub struct EditMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `Patch<CompactString>` | The message text. |
| `embeds` | `Patch<Vec<Embed>>` | The rich embeds attached to the message. |
| `flags` | `Patch<MessageFlags>` | Bitfield of flags attached to the object. |
| `allowed_mentions` | `Patch<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `components` | `Patch<Vec<Component>>` | The components attached to the message. |
| `attachments` | `Patch<Vec<AttachmentRequest>>` | The files attached to the message. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |

### `GetChannelMessages`

Query parameters for the matching REST call.

```rust
pub struct GetChannelMessages
```

| Field | Type | Description |
| --- | --- | --- |
| `around` | `Option<MessageId>` | The around. |
| `before` | `Option<MessageId>` | Return items before this ID. |
| `after` | `Option<MessageId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |

### `GetReactions`

Query parameters for the matching REST call.

```rust
pub struct GetReactions
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `Option<ReactionType>` | The object's type discriminator. |
| `after` | `Option<UserId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |

### `BulkDeleteMessages`

Request body for the matching REST call.

```rust
pub struct BulkDeleteMessages
```

| Field | Type | Description |
| --- | --- | --- |
| `messages` | `Vec<MessageId>` | The messages. |

#### Methods

##### `new`

Creates a new `BulkDeleteMessages`.

```rust
pub const fn new(messages: Vec<MessageId>) -> Self;
```

### `GetChannelPins`

Query parameters for the matching REST call.

```rust
pub struct GetChannelPins
```

| Field | Type | Description |
| --- | --- | --- |
| `before` | `Option<Timestamp>` | Return items before this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |

### `ChannelPins`

Request or response shape for the matching REST call.

```rust
pub struct ChannelPins
```

| Field | Type | Description |
| --- | --- | --- |
| `items` | `Vec<MessagePin>` | The items. |
| `has_more` | `bool` | Whether more results are available. |

### `SearchGuildMessages`

Query parameters for the matching REST call.

```rust
pub struct SearchGuildMessages
```

| Field | Type | Description |
| --- | --- | --- |
| `limit` | `Option<u8>` | The maximum number of items to return. |
| `offset` | `Option<u16>` | The offset. |
| `max_id` | `Option<MessageId>` | The ID of the max. |
| `min_id` | `Option<MessageId>` | The ID of the min. |
| `slop` | `Option<u8>` | The slop. |
| `content` | `Option<CompactString>` | The message text. |
| `channel_id` | `Vec<ChannelId>` | The ID of the channel this belongs to. |
| `author_type` | `Vec<SearchFilter<SearchAuthorType>>` | The author type. |
| `author_id` | `Vec<UserId>` | The ID of the author. |
| `mentions` | `Vec<UserId>` | The mentions. |
| `mentions_role_id` | `Vec<RoleId>` | The ID of the mentions role. |
| `mention_everyone` | `Option<bool>` | Whether the message mentions @everyone. |
| `replied_to_user_id` | `Vec<UserId>` | The ID of the replied to user. |
| `replied_to_message_id` | `Vec<MessageId>` | The ID of the replied to message. |
| `pinned` | `Option<bool>` | Whether the message is pinned. |
| `has` | `Vec<SearchFilter<SearchHasType>>` | The has. |
| `embed_type` | `Vec<SearchEmbedType>` | The embed type. |
| `embed_provider` | `Vec<CompactString>` | The embed provider. |
| `link_hostname` | `Vec<CompactString>` | The link hostname. |
| `attachment_filename` | `Vec<CompactString>` | The attachment filename. |
| `attachment_extension` | `Vec<CompactString>` | The attachment extension. |
| `sort_by` | `Option<SearchSortMode>` | The sort by. |
| `sort_order` | `Option<SearchSortOrder>` | The sort order. |
| `include_nsfw` | `Option<bool>` | The include nsfw. |

### `SearchFilter`

Possible search filter values.

```rust
pub enum SearchFilter<T>
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Include` | `T` | Include. |
| `Exclude` | `T` | Exclude. |

### `SearchAuthorType`

Possible search author type values.

```rust
pub enum SearchAuthorType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `User` | `,` | User. |
| `Bot` | `,` | Bot. |
| `Webhook` | `,` | Webhook. |

### `SearchHasType`

Possible search has type values.

```rust
pub enum SearchHasType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Image` | `,` | Image. |
| `Sound` | `,` | Sound. |
| `Video` | `,` | Video. |
| `File` | `,` | File. |
| `Sticker` | `,` | Sticker. |
| `Embed` | `,` | Embed. |
| `Link` | `,` | Link. |
| `Poll` | `,` | Poll. |
| `Snapshot` | `,` | Snapshot. |

### `SearchEmbedType`

Possible search embed type values.

```rust
pub enum SearchEmbedType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Image` | `,` | Image. |
| `Video` | `,` | Video. |
| `Gif` | `,` | Gif. |
| `Sound` | `,` | Sound. |
| `Article` | `,` | Article. |

### `SearchSortMode`

Possible search sort mode values.

```rust
pub enum SearchSortMode
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Timestamp` | `,` | Timestamp. |
| `Relevance` | `,` | Relevance. |

### `SearchSortOrder`

Possible search sort order values.

```rust
pub enum SearchSortOrder
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Ascending` | `,` | Ascending. |
| `Descending` | `,` | Descending. |

### `MessageSearchResults`

Request or response shape for the matching REST call.

```rust
pub struct MessageSearchResults
```

| Field | Type | Description |
| --- | --- | --- |
| `doing_deep_historical_index` | `bool` | The doing deep historical index. |
| `documents_indexed` | `Option<u64>` | The documents indexed. |
| `total_results` | `u64` | The total results. |
| `messages` | `Vec<Vec<Message>>` | The messages. |
| `threads` | `Vec<Channel>` | The threads in this channel. |
| `members` | `Vec<ThreadMember>` | The members in this collection. |

## `model.rest.poll`

### `GetAnswerVoters`

Query parameters for the matching REST call.

```rust
pub struct GetAnswerVoters
```

| Field | Type | Description |
| --- | --- | --- |
| `after` | `Option<UserId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |

## `model.rest.setters`

## `model.rest.soundboard`

### `SendSoundboardSound`

Request or response shape for the matching REST call.

```rust
pub struct SendSoundboardSound
```

| Field | Type | Description |
| --- | --- | --- |
| `sound_id` | `SoundboardSoundId` | The ID of the soundboard sound. |
| `source_guild_id` | `Option<GuildId>` | The ID of the source guild. |

#### Methods

##### `new`

Creates a new `SendSoundboardSound`.

```rust
pub const fn new(sound_id: SoundboardSoundId) -> Self;
```

### `CreateGuildSoundboardSound`

Request body for the matching REST call.

```rust
pub struct CreateGuildSoundboardSound
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `sound` | `SoundData` | The sound. |
| `volume` | `Option<f64>` | Playback volume. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |

#### Methods

##### `new`

Creates a new `CreateGuildSoundboardSound`.

```rust
pub fn new(name: impl Into<CompactString>, sound: SoundData) -> Self;
```

### `ModifyGuildSoundboardSound`

Request body for the matching REST call.

```rust
pub struct ModifyGuildSoundboardSound
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `volume` | `Patch<f64>` | Playback volume. |
| `emoji_id` | `Patch<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Patch<CompactString>` | The emoji name. |

## `model.rest.stage_instance`

### `CreateStageInstance`

Request body for the matching REST call.

```rust
pub struct CreateStageInstance
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `topic` | `CompactString` | The topic. |
| `privacy_level` | `Option<PrivacyLevel>` | Who can see the event. |
| `send_start_notification` | `Option<bool>` | The send start notification. |
| `guild_scheduled_event_id` | `Option<ScheduledEventId>` | The ID of the scheduled event. |

#### Methods

##### `new`

Creates a new `CreateStageInstance`.

```rust
pub fn new(channel_id: ChannelId, topic: impl Into<CompactString>) -> Self;
```

### `ModifyStageInstance`

Request body for the matching REST call.

```rust
pub struct ModifyStageInstance
```

| Field | Type | Description |
| --- | --- | --- |
| `topic` | `Option<CompactString>` | The topic. |
| `privacy_level` | `Option<PrivacyLevel>` | Who can see the event. |

## `model.rest.sticker`

### `CreateGuildSticker`

Request body for the matching REST call.

```rust
pub struct CreateGuildSticker
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `description` | `CompactString` | A human-readable description. |
| `tags` | `CompactString` | Links to the bot, integration or boost that created the role. |
| `file` | `FileUpload` | The file. |

#### Methods

##### `new`

Creates a new `CreateGuildSticker`.

```rust
pub fn new( name: impl Into<CompactString>, tags: impl Into<CompactString>, file: FileUpload, ) -> Self;
```

### `ModifyGuildSticker`

Request body for the matching REST call.

```rust
pub struct ModifyGuildSticker
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `description` | `Patch<CompactString>` | A human-readable description. |
| `tags` | `Option<CompactString>` | Links to the bot, integration or boost that created the role. |

## `model.rest.subscription`

### `ListSkuSubscriptions`

Query parameters for the matching REST call.

```rust
pub struct ListSkuSubscriptions
```

| Field | Type | Description |
| --- | --- | --- |
| `before` | `Option<SubscriptionId>` | Return items before this ID. |
| `after` | `Option<SubscriptionId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |
| `user_id` | `Option<UserId>` | The ID of the user. |

## `model.rest.user`

### `ModifyCurrentUser`

Request body for the matching REST call.

```rust
pub struct ModifyCurrentUser
```

| Field | Type | Description |
| --- | --- | --- |
| `username` | `Option<CompactString>` | The account username. |
| `avatar` | `Patch<ImageData>` | The avatar hash. |
| `banner` | `Patch<ImageData>` | The banner hash. |

### `GetCurrentUserGuilds`

Query parameters for the matching REST call.

```rust
pub struct GetCurrentUserGuilds
```

| Field | Type | Description |
| --- | --- | --- |
| `before` | `Option<GuildId>` | Return items before this ID. |
| `after` | `Option<GuildId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |
| `shard` | `Option<u16>` | Shard information. |
| `with_counts` | `Option<bool>` | The with counts. |

### `CurrentUserGuild`

Request or response shape for the matching REST call.

```rust
pub struct CurrentUserGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `owner` | `bool` | Whether the user owns the guild or application. |
| `permissions` | `Permissions` | The permissions bitfield. |
| `features` | `Vec<Feature>` | The feature flags enabled for the guild. |
| `approximate_member_count` | `Option<u32>` | The approximate member count from the widget. |
| `approximate_presence_count` | `Option<u32>` | The approximate online count from the widget. |

### `CreateDm`

Request body for the matching REST call.

```rust
pub struct CreateDm
```

| Field | Type | Description |
| --- | --- | --- |
| `recipient_id` | `UserId` | The ID of the recipient. |

#### Methods

##### `new`

Creates a new `CreateDm`.

```rust
pub const fn new(recipient_id: UserId) -> Self;
```

### `CreateGroupDm`

Request body for the matching REST call.

```rust
pub struct CreateGroupDm
```

| Field | Type | Description |
| --- | --- | --- |
| `access_tokens` | `Vec<String>` | The access tokens. |
| `nicks` | `BTreeMap<UserId, CompactString>` | The nicks. |

#### Methods

##### `new`

Creates a new `CreateGroupDm`.

```rust
pub fn new(access_tokens: Vec<String>) -> Self;
```

## `model.rest.voice`

### `ModifyCurrentUserVoiceState`

Request body for the matching REST call.

```rust
pub struct ModifyCurrentUserVoiceState
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `suppress` | `Option<bool>` | The suppress. |
| `request_to_speak_timestamp` | `Patch<Timestamp>` | When the request to speak happened, if known. |

### `ModifyUserVoiceState`

Request body for the matching REST call.

```rust
pub struct ModifyUserVoiceState
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `suppress` | `Option<bool>` | The suppress. |

## `model.rest.webhook`

### `CreateWebhook`

Request body for the matching REST call.

```rust
pub struct CreateWebhook
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `avatar` | `Option<ImageData>` | The avatar hash. |

#### Methods

##### `new`

Creates a new `CreateWebhook`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `ModifyWebhook`

Request body for the matching REST call.

```rust
pub struct ModifyWebhook
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `avatar` | `Patch<ImageData>` | The avatar hash. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |

### `ExecuteWebhook`

Request or response shape for the matching REST call.

```rust
pub struct ExecuteWebhook
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `Option<CompactString>` | The message text. |
| `username` | `Option<CompactString>` | Overrides the webhook's username for this message. |
| `avatar_url` | `Option<CompactString>` | The URL for the avatar. |
| `tts` | `bool` | The tts. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `allowed_mentions` | `Option<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `attachments` | `Option<Vec<AttachmentRequest>>` | The files attached to the message. |
| `flags` | `Option<MessageFlags>` | Bitfield of flags attached to the object. |
| `thread_name` | `Option<CompactString>` | Names the thread created for a forum-channel webhook post. |
| `applied_tags` | `Vec<TagId>` | Applies forum tags to the thread created for the webhook post. |
| `poll` | `Option<CreatePoll>` | The poll attached to the message. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |
| `thread_id` | `Option<ChannelId>` | The ID of the thread. |
| `with_components` | `Option<bool>` | The with components. |

#### Methods

##### `username`

Overrides the webhook's username for this message.

```rust
pub fn username(mut self, username: impl Into<CompactString>) -> Self;
```

##### `avatar_url`

Returns the avatar URL.

```rust
pub fn avatar_url(mut self, url: impl Into<CompactString>) -> Self;
```

##### `thread_name`

Names the thread created for a forum-channel webhook post.

```rust
pub fn thread_name(mut self, name: impl Into<CompactString>) -> Self;
```

##### `applied_tags`

Applies forum tags to the thread created for the webhook post.

```rust
pub fn applied_tags(mut self, tags: impl IntoIterator<Item = TagId>) -> Self;
```

##### `thread_id`

Returns the ID of the thread.

```rust
pub const fn thread_id(mut self, thread_id: ChannelId) -> Self;
```

##### `with_components`

Sets components.

```rust
pub const fn with_components(mut self, with_components: bool) -> Self;
```

### `EditWebhookMessage`

Request body for the matching REST call.

```rust
pub struct EditWebhookMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `Patch<CompactString>` | The message text. |
| `embeds` | `Patch<Vec<Embed>>` | The rich embeds attached to the message. |
| `flags` | `Patch<MessageFlags>` | Bitfield of flags attached to the object. |
| `allowed_mentions` | `Patch<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `components` | `Patch<Vec<Component>>` | The components attached to the message. |
| `attachments` | `Patch<Vec<AttachmentRequest>>` | The files attached to the message. |
| `poll` | `Patch<CreatePoll>` | The poll attached to the message. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |
| `thread_id` | `Option<ChannelId>` | The ID of the thread. |
| `with_components` | `Option<bool>` | The with components. |

#### Methods

##### `thread_id`

Returns the ID of the thread.

```rust
pub const fn thread_id(mut self, thread_id: ChannelId) -> Self;
```

##### `with_components`

Sets components.

```rust
pub const fn with_components(mut self, with_components: bool) -> Self;
```
