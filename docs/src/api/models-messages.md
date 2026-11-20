# Models: messages and components

Messages, attachments, embeds, components and polls.

## `model.message`

### `pub use self::{ attachment::{Attachment, AttachmentRequest, AttachmentRequestId}, embed::{ Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedType, EmbedVideo, }, interaction_metadata::{ CommandMetadata, ComponentMetadata, InteractionMetadata, ModalSubmitMetadata, UnknownMetadata, }, };`

```rust
pub use self::{ attachment::{Attachment, AttachmentRequest, AttachmentRequestId}, embed::{ Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage, EmbedProvider, EmbedType, EmbedVideo, }, interaction_metadata::{ CommandMetadata, ComponentMetadata, InteractionMetadata, ModalSubmitMetadata, UnknownMetadata, }, };
```

### `Message`

A message: author, content, embeds, attachments, components, reactions, poll and reference.

```rust
pub struct Message
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `MessageId` | The object's snowflake ID. |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `author` | `User` | The author. |
| `member` | `Option<Box<PartialMember>>` | The guild member this refers to. |
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

### `Mention`

A user mentioned by a message, with optional member data.

```rust
pub struct Mention
```

| Field | Type | Description |
| --- | --- | --- |
| `user` | `User` | The user this object describes. |
| `member` | `Option<Box<PartialMember>>` | The guild member this refers to. |

#### Methods

##### `new`

Creates a new `Mention`.

```rust
pub const fn new(user: User) -> Self;
```

### `ReferencedMessage`

The message a reply or forward points at.

```rust
pub enum ReferencedMessage
```

| Variant | Payload | Description |
| --- | --- | --- |
| `NotFetched` | `,` | Not Fetched. |
| `Deleted` | `,` | Deleted. |
| `Message` | `Box<Message>` | Message. |

#### Methods

##### `as_message`

Returns the interaction response as a message.

```rust
pub fn as_message(&self) -> Option<&Message>;
```

##### `into_message`

Consumes this and returns the interaction response as a message.

```rust
pub fn into_message(self) -> Option<Box<Message>>;
```

##### `is_not_fetched`

Returns whether not fetched.

```rust
pub const fn is_not_fetched(&self) -> bool;
```

##### `is_deleted`

Returns whether deleted.

```rust
pub const fn is_deleted(&self) -> bool;
```

### `Nonce`

Possible nonce values.

```rust
pub enum Nonce
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Integer` | `i64` | Integer. |
| `String` | `CompactString` | String. |

### `MessageReference`

A reference to another message, by guild, channel and message ID.

```rust
pub struct MessageReference
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `MessageReferenceType` | The object's type discriminator. |
| `message_id` | `Option<MessageId>` | The ID of the message. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `fail_if_not_exists` | `Option<bool>` | The fail if not exists. |

#### Methods

##### `reply`

Builds a reply reference to the given message.

```rust
pub const fn reply(message_id: MessageId) -> Self;
```

### `MessageReferenceType`

Possible message reference type values.

```rust
pub enum MessageReferenceType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Default` | `,` | Default. |
| `Forward` | `,` | Forward. |

### `MessageType`

What kind of message this is: regular, join, boost, pin notice, call and more.

```rust
pub enum MessageType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Default` | `,` | A regular user or bot message. |
| `RecipientAdd` | `,` | A user was added to a group DM. |
| `RecipientRemove` | `,` | A user was removed from a group DM. |
| `Call` | `,` | A group DM call notice. |
| `ChannelNameChange` | `,` | The channel was renamed. |
| `ChannelIconChange` | `,` | The channel icon changed. |
| `ChannelPinnedMessage` | `,` | A message was pinned. |
| `UserJoin` | `,` | User Join. |
| `GuildBoost` | `,` | Guild Boost. |
| `GuildBoostTier1` | `,` | Guild Boost Tier 1. |
| `GuildBoostTier2` | `,` | Guild Boost Tier 2. |
| `GuildBoostTier3` | `,` | Guild Boost Tier 3. |
| `ChannelFollowAdd` | `,` | Channel Follow Add. |
| `GuildDiscoveryDisqualified` | `,` | Guild Discovery Disqualified. |
| `GuildDiscoveryRequalified` | `,` | Guild Discovery Requalified. |
| `GuildDiscoveryGracePeriodInitialWarning` | `,` | Guild Discovery Grace Period Initial Warning. |
| `GuildDiscoveryGracePeriodFinalWarning` | `,` | Guild Discovery Grace Period Final Warning. |
| `ThreadCreated` | `,` | A thread was created. |
| `Reply` | `,` | A reply to another message. |
| `ChatInputCommand` | `,` | A slash command was used. |
| `ThreadStarterMessage` | `,` | Thread Starter Message. |
| `GuildInviteReminder` | `,` | Guild Invite Reminder. |
| `ContextMenuCommand` | `,` | A user or message command was used. |
| `AutoModerationAction` | `,` | Auto Moderation Action. |
| `RoleSubscriptionPurchase` | `,` | Role Subscription Purchase. |
| `InteractionPremiumUpsell` | `,` | Interaction Premium Upsell. |
| `StageStart` | `,` | Stage Start. |
| `StageEnd` | `,` | Stage End. |
| `StageSpeaker` | `,` | Stage Speaker. |
| `StageRaiseHand` | `,` | Stage Raise Hand. |
| `StageTopic` | `,` | Stage Topic. |
| `GuildApplicationPremiumSubscription` | `,` | Guild Application Premium Subscription. |
| `GuildIncidentAlertModeEnabled` | `,` | Guild Incident Alert Mode Enabled. |
| `GuildIncidentAlertModeDisabled` | `,` | Guild Incident Alert Mode Disabled. |
| `GuildIncidentReportRaid` | `,` | Guild Incident Report Raid. |
| `GuildIncidentReportFalseAlarm` | `,` | Guild Incident Report False Alarm. |
| `PurchaseNotification` | `,` | Purchase Notification. |
| `PollResult` | `,` | Poll Result. |

#### Methods

##### `is_deletable`

Returns whether deletable.

```rust
pub const fn is_deletable(self) -> bool;
```

### `MessageActivity`

The activity invite attached to a rich-presence message.

```rust
pub struct MessageActivity
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `MessageActivityType` | The object's type discriminator. |
| `party_id` | `Option<CompactString>` | The ID of the party. |

### `MessageActivityType`

Possible message activity type values.

```rust
pub enum MessageActivityType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Join` | `,` | Join. |
| `Spectate` | `,` | Spectate. |
| `Listen` | `,` | Listen. |
| `JoinRequest` | `,` | Join Request. |
| `StreamRequest` | `,` | Stream Request. |

### `MessageCall`

Discord's message call object.

```rust
pub struct MessageCall
```

| Field | Type | Description |
| --- | --- | --- |
| `participants` | `Vec<UserId>` | The participants. |
| `ended_timestamp` | `Option<Timestamp>` | When the ended happened, if known. |

### `MessageSnapshot`

A snapshot of a forwarded message's original content.

```rust
pub struct MessageSnapshot
```

| Field | Type | Description |
| --- | --- | --- |
| `message` | `SnapshotMessage` | The message this refers to. |

### `SnapshotMessage`

Discord's snapshot message object.

```rust
pub struct SnapshotMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `MessageType` | The object's type discriminator. |
| `content` | `CompactString` | The message text. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `attachments` | `Vec<Attachment>` | The files attached to the message. |
| `timestamp` | `Timestamp` | The timestamp. |
| `edited_timestamp` | `Option<Timestamp>` | When the edited happened, if known. |
| `flags` | `MessageFlags` | Bitfield of flags attached to the object. |
| `mentions` | `Vec<User>` | The mentions. |
| `mention_roles` | `Vec<RoleId>` | The role IDs mentioned by the message. |
| `stickers` | `Vec<Sticker>` | The stickers attached to the message. |
| `sticker_items` | `Vec<StickerItem>` | The sticker items. |
| `components` | `Vec<Component>` | The components attached to the message. |

### `ReactionType`

A reaction emoji, either unicode or a custom emoji ID.

```rust
pub enum ReactionType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Normal` | `,` | Normal. |
| `Burst` | `,` | Burst. |

### `Reaction`

One emoji reaction and the users who added it.

```rust
pub struct Reaction
```

| Field | Type | Description |
| --- | --- | --- |
| `count` | `u32` | The count. |
| `count_details` | `ReactionCountDetails` | The count details. |
| `me` | `bool` | The me. |
| `me_burst` | `bool` | The me burst. |
| `emoji` | `PartialEmoji` | The emoji shown with the object. |
| `burst_colors` | `Vec<CompactString>` | The burst colors. |

#### Methods

##### `new`

Creates a new `Reaction`.

```rust
pub fn new(emoji: PartialEmoji) -> Self;
```

### `ReactionCountDetails`

Discord's reaction count details object.

```rust
pub struct ReactionCountDetails
```

| Field | Type | Description |
| --- | --- | --- |
| `burst` | `u32` | The burst color as a 24-bit integer. |
| `normal` | `u32` | The normal. |

### `ChannelMention`

Discord's channel mention object.

```rust
pub struct ChannelMention
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `kind` | `ChannelType` | The object's type discriminator. |
| `name` | `CompactString` | The object's name. |

### `AllowedMentions`

Which mentions a message is allowed to ping.

```rust
pub struct AllowedMentions
```

| Field | Type | Description |
| --- | --- | --- |
| `parse` | `Vec<AllowedMentionType>` | The parse. |
| `roles` | `Vec<RoleId>` | The roles attached or referenced. |
| `users` | `Vec<UserId>` | The users. |
| `replied_user` | `bool` | The replied user. |

### `AllowedMentionType`

One mention category: everyone, users or roles.

```rust
pub enum AllowedMentionType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Roles` | `,` | Roles. |
| `Users` | `,` | Users. |
| `Everyone` | `,` | Everyone. |

### `RoleSubscriptionData`

Details for a role-subscription purchase or renewal message.

```rust
pub struct RoleSubscriptionData
```

| Field | Type | Description |
| --- | --- | --- |
| `role_subscription_listing_id` | `SkuId` | The ID of the role subscription listing. |
| `tier_name` | `CompactString` | The tier name. |
| `total_months_subscribed` | `u32` | The total months subscribed. |
| `is_renewal` | `bool` | Whether this is renewal. |

### `MessagePin`

Who pinned a message and when.

```rust
pub struct MessagePin
```

| Field | Type | Description |
| --- | --- | --- |
| `pinned_at` | `Timestamp` | When the pinned happened, if known. |
| `message` | `Message` | The message this refers to. |

### `SharedClientTheme`

Discord's shared client theme object.

```rust
pub struct SharedClientTheme
```

| Field | Type | Description |
| --- | --- | --- |
| `colors` | `Vec<CompactString>` | The role's primary, secondary and tertiary colors. |
| `gradient_angle` | `u16` | The gradient angle. |
| `base_mix` | `u8` | The base mix. |
| `base_theme` | `Option<BaseTheme>` | The base theme. |

### `BaseTheme`

Possible base theme values.

```rust
pub enum BaseTheme: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Unset` | `,` | Unset. |
| `Dark` | `,` | Dark. |
| `Light` | `,` | Light. |
| `Darker` | `,` | Darker. |
| `Midnight` | `,` | Midnight. |

## `model.message.attachment`

### `Attachment`

A file attached to a message, with size, dimensions and proxied URL.

```rust
pub struct Attachment
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `AttachmentId` | The object's snowflake ID. |
| `filename` | `CompactString` | The filename. |
| `title` | `Option<CompactString>` | The title. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `content_type` | `Option<CompactString>` | The content type. |
| `size` | `u64` | The size, in pixels for images. |
| `url` | `CompactString` | The URL. |
| `proxy_url` | `CompactString` | The proxied media URL. |
| `height` | `Option<u32>` | The height in pixels. |
| `width` | `Option<u32>` | The width in pixels. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `placeholder_version` | `Option<u32>` | The placeholder version. |
| `ephemeral` | `bool` | The ephemeral. |
| `duration_secs` | `Option<f64>` | The duration in seconds. |
| `waveform` | `Option<CompactString>` | The waveform. |
| `flags` | `AttachmentFlags` | Bitfield of flags attached to the object. |
| `clip_participants` | `Vec<User>` | The clip participants. |
| `clip_created_at` | `Option<Timestamp>` | When the clip created happened, if known. |
| `application` | `Option<Box<PartialApplication>>` | The application this belongs to. |

### `AttachmentRequest`

A file to upload, taken from a path, bytes or a URL.

```rust
pub struct AttachmentRequest
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `AttachmentRequestId` | The object's snowflake ID. |
| `filename` | `Option<CompactString>` | The filename. |
| `title` | `Option<CompactString>` | The title. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `duration_secs` | `Option<f64>` | The duration in seconds. |
| `waveform` | `Option<CompactString>` | The waveform. |
| `is_spoiler` | `Option<bool>` | Whether this is spoiler. |

#### Methods

##### `upload`

Attaches a file upload to the request.

```rust
pub fn upload(index: u32, filename: impl Into<CompactString>) -> Self;
```

##### `existing`

References an already-uploaded attachment by ID.

```rust
pub fn existing(id: AttachmentId) -> Self;
```

### `AttachmentRequestId`

Possible attachment request id values.

```rust
pub enum AttachmentRequestId
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Upload` | `u32` | Upload. |
| `Existing` | `AttachmentId` | Existing. |

## `model.message.embed`

### `Embed`

A rich embed: title, description, fields, images, footer and color.

```rust
pub struct Embed
```

| Field | Type | Description |
| --- | --- | --- |
| `title` | `Option<CompactString>` | The title. |
| `kind` | `Option<EmbedType>` | The object's type discriminator. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `url` | `Option<CompactString>` | The URL. |
| `timestamp` | `Option<Timestamp>` | The timestamp. |
| `color` | `Option<u32>` | The primary role color as a 24-bit integer. |
| `footer` | `Option<Box<EmbedFooter>>` | The footer. |
| `image` | `Option<Box<EmbedImage>>` | The image hash or URL. |
| `thumbnail` | `Option<Box<EmbedImage>>` | The thumbnail. |
| `video` | `Option<Box<EmbedVideo>>` | The video. |
| `provider` | `Option<Box<EmbedProvider>>` | The provider. |
| `author` | `Option<Box<EmbedAuthor>>` | The author. |
| `fields` | `Vec<EmbedField>` | The fields. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `flags` | `Option<EmbedFlags>` | Bitfield of flags attached to the object. |

### `EmbedType`

Whether an embed was written by a bot or generated by Discord.

```rust
pub enum EmbedType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Rich` | `,` | Rich. |
| `Image` | `,` | Image. |
| `Video` | `,` | Video. |
| `Gifv` | `,` | Gifv. |
| `Article` | `,` | Article. |
| `Link` | `,` | Link. |
| `AutoModerationMessage` | `,` | Auto Moderation Message. |
| `PollResult` | `,` | Poll Result. |
| `Components` | `,` | Components. |

### `EmbedFooter`

The footer line and optional icon.

```rust
pub struct EmbedFooter
```

| Field | Type | Description |
| --- | --- | --- |
| `text` | `CompactString` | The text. |
| `icon_url` | `Option<CompactString>` | The URL for the icon. |
| `proxy_icon_url` | `Option<CompactString>` | The URL for the proxy icon. |

#### Methods

##### `new`

Creates a new `EmbedFooter`.

```rust
pub fn new(text: impl Into<CompactString>) -> Self;
```

### `EmbedImage`

An embed image with URL, proxy and dimensions.

```rust
pub struct EmbedImage
```

| Field | Type | Description |
| --- | --- | --- |
| `url` | `CompactString` | The URL. |
| `proxy_url` | `Option<CompactString>` | The proxied media URL. |
| `height` | `Option<u32>` | The height in pixels. |
| `width` | `Option<u32>` | The width in pixels. |
| `content_type` | `Option<CompactString>` | The content type. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `placeholder_version` | `Option<u32>` | The placeholder version. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `flags` | `Option<EmbedMediaFlags>` | Bitfield of flags attached to the object. |

#### Methods

##### `new`

Creates a new `EmbedImage`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

### `EmbedVideo`

An embedded video with URL and dimensions.

```rust
pub struct EmbedVideo
```

| Field | Type | Description |
| --- | --- | --- |
| `url` | `Option<CompactString>` | The URL. |
| `proxy_url` | `Option<CompactString>` | The proxied media URL. |
| `height` | `Option<u32>` | The height in pixels. |
| `width` | `Option<u32>` | The width in pixels. |
| `content_type` | `Option<CompactString>` | The content type. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `placeholder_version` | `Option<u32>` | The placeholder version. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `flags` | `Option<EmbedMediaFlags>` | Bitfield of flags attached to the object. |

### `EmbedProvider`

The provider name and URL shown on an embed.

```rust
pub struct EmbedProvider
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `url` | `Option<CompactString>` | The URL. |

### `EmbedAuthor`

The author line with name, URL and icon.

```rust
pub struct EmbedAuthor
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `url` | `Option<CompactString>` | The URL. |
| `icon_url` | `Option<CompactString>` | The URL for the icon. |
| `proxy_icon_url` | `Option<CompactString>` | The URL for the proxy icon. |

#### Methods

##### `new`

Creates a new `EmbedAuthor`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `EmbedField`

One inline or block field inside an embed.

```rust
pub struct EmbedField
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `value` | `CompactString` | The option's value. |
| `inline` | `bool` | The inline. |

#### Methods

##### `new`

Creates a new `EmbedField`.

```rust
pub fn new( name: impl Into<CompactString>, value: impl Into<CompactString>, inline: bool, ) -> Self;
```

## `model.message.interaction_metadata`

### `InteractionMetadata`

Possible interaction metadata values.

```rust
pub enum InteractionMetadata
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ApplicationCommand` | `CommandMetadata` | Application Command. |
| `MessageComponent` | `ComponentMetadata` | Message Component. |
| `ModalSubmit` | `ModalSubmitMetadata` | Modal Submit. |
| `Unknown` | `UnknownMetadata` | A value this build does not recognize. |

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> InteractionId;
```

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> InteractionType;
```

##### `user`

Builds a user context-menu command.

```rust
pub const fn user(&self) -> &User;
```

##### `authorizing_integration_owners`

Returns which installation authorized the interaction.

```rust
pub const fn authorizing_integration_owners(&self) -> &AuthorizingIntegrationOwners;
```

##### `original_response_message_id`

Returns the ID of the original response message.

```rust
pub const fn original_response_message_id(&self) -> Option<MessageId>;
```

### `CommandMetadata`

Discord's command metadata object.

```rust
pub struct CommandMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `InteractionId` | The object's snowflake ID. |
| `user` | `User` | The user this object describes. |
| `authorizing_integration_owners` | `AuthorizingIntegrationOwners` | The authorizing integration owners. |
| `original_response_message_id` | `Option<MessageId>` | The ID of the original response message. |
| `target_user` | `Option<Box<User>>` | The target user. |
| `target_message_id` | `Option<MessageId>` | The ID of the target message. |

### `ComponentMetadata`

Discord's component metadata object.

```rust
pub struct ComponentMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `InteractionId` | The object's snowflake ID. |
| `user` | `User` | The user this object describes. |
| `authorizing_integration_owners` | `AuthorizingIntegrationOwners` | The authorizing integration owners. |
| `original_response_message_id` | `Option<MessageId>` | The ID of the original response message. |
| `interacted_message_id` | `MessageId` | The ID of the interacted message. |

### `ModalSubmitMetadata`

Discord's modal submit metadata object.

```rust
pub struct ModalSubmitMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `InteractionId` | The object's snowflake ID. |
| `user` | `User` | The user this object describes. |
| `authorizing_integration_owners` | `AuthorizingIntegrationOwners` | The authorizing integration owners. |
| `original_response_message_id` | `Option<MessageId>` | The ID of the original response message. |
| `triggering_interaction_metadata` | `Box<InteractionMetadata>` | The triggering interaction metadata. |

### `UnknownMetadata`

Discord's unknown metadata object.

```rust
pub struct UnknownMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `InteractionType` | The object's type discriminator. |
| `id` | `InteractionId` | The object's snowflake ID. |
| `user` | `User` | The user this object describes. |
| `authorizing_integration_owners` | `AuthorizingIntegrationOwners` | The authorizing integration owners. |
| `original_response_message_id` | `Option<MessageId>` | The ID of the original response message. |

## `model.component`

### `pub use self::{ content::{ ContentScanMetadata, ContentScanMetadataFlags, File, LoadingState, MediaGallery, MediaGalleryItem, TextDisplay, Thumbnail, UnfurledMediaItem, UnfurledMediaItemFlags, }, interactive::{ AutoPopulatedSelect, Button, ButtonStyle, ChannelSelect, Checkbox, CheckboxGroup, FileUpload, GroupOption, InteractiveButton, LinkButton, PremiumButton, RadioGroup, SelectDefaultValue, SelectDefaultValueType, SelectOption, StringSelect, TextInput, TextInputStyle, UnknownButton, }, layout::{ActionRow, Container, Label, Section, Separator, SeparatorSpacing}, };`

```rust
pub use self::{ content::{ ContentScanMetadata, ContentScanMetadataFlags, File, LoadingState, MediaGallery, MediaGalleryItem, TextDisplay, Thumbnail, UnfurledMediaItem, UnfurledMediaItemFlags, }, interactive::{ AutoPopulatedSelect, Button, ButtonStyle, ChannelSelect, Checkbox, CheckboxGroup, FileUpload, GroupOption, InteractiveButton, LinkButton, PremiumButton, RadioGroup, SelectDefaultValue, SelectDefaultValueType, SelectOption, StringSelect, TextInput, TextInputStyle, UnknownButton, }, layout::{ActionRow, Container, Label, Section, Separator, SeparatorSpacing}, };
```

### `Component`

A message component. Match on the variant to reach buttons, selects, text inputs and layout containers.

```rust
pub enum Component
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ActionRow` | `ActionRow` | Action Row. |
| `Button` | `Button` | Button. |
| `StringSelect` | `StringSelect` | String Select. |
| `TextInput` | `TextInput` | Text Input. |
| `UserSelect` | `AutoPopulatedSelect` | User Select. |
| `RoleSelect` | `AutoPopulatedSelect` | Role Select. |
| `MentionableSelect` | `AutoPopulatedSelect` | Mentionable Select. |
| `ChannelSelect` | `ChannelSelect` | Channel Select. |
| `Section` | `Section` | Section. |
| `TextDisplay` | `TextDisplay` | Text Display. |
| `Thumbnail` | `Thumbnail` | Thumbnail. |
| `MediaGallery` | `MediaGallery` | Media Gallery. |
| `File` | `File` | File. |
| `Separator` | `Separator` | Separator. |
| `Container` | `Container` | Container. |
| `Label` | `Label` | Label. |
| `FileUpload` | `FileUpload` | File Upload. |
| `RadioGroup` | `RadioGroup` | Radio Group. |
| `CheckboxGroup` | `CheckboxGroup` | Checkbox Group. |
| `Checkbox` | `Checkbox` | Checkbox. |
| `Unknown` | `UnknownComponent` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> ComponentType;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> Option<i32>;
```

##### `custom_id`

Returns the component's custom ID.

```rust
pub fn custom_id(&self) -> Option<&str>;
```

##### `children`

Returns the child components of a layout container.

```rust
pub fn children(&self) -> &[Self];
```

### `UnknownComponent`

Discord's unknown component object.

```rust
pub struct UnknownComponent
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `ComponentType` | The object's type discriminator. |
| `id` | `Option<i32>` | The object's snowflake ID. |

### `ComponentType`

The component discriminator used on the wire.

```rust
pub enum ComponentType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ActionRow` | `,` | A row of buttons or one select menu. |
| `Button` | `,` | A button. |
| `StringSelect` | `,` | A fixed-option select menu. |
| `TextInput` | `,` | A text input inside a modal. |
| `UserSelect` | `,` | A user select menu. |
| `RoleSelect` | `,` | A role select menu. |
| `MentionableSelect` | `,` | A user-or-role select menu. |
| `ChannelSelect` | `,` | A channel select menu. |
| `Section` | `,` | A section layout component. |
| `TextDisplay` | `,` | A markdown text block. |
| `Thumbnail` | `,` | A thumbnail image. |
| `MediaGallery` | `,` | A media gallery. |
| `File` | `,` | A file display. |
| `Separator` | `,` | A visual separator. |
| `ContentInventoryEntry` | `,` | Content Inventory Entry. |
| `Container` | `,` | A bordered container. |
| `Label` | `,` | A modal label. |
| `FileUpload` | `,` | A file upload input inside a modal. |
| `RadioGroup` | `,` | A radio group inside a modal. |
| `CheckboxGroup` | `,` | A checkbox group inside a modal. |
| `Checkbox` | `,` | A single checkbox inside a modal. |



## `model.component.content`

### `TextDisplay`

A block of markdown text inside a component layout.

```rust
pub struct TextDisplay
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `content` | `CompactString` | The message text. |

#### Methods

##### `new`

Creates a new `TextDisplay`.

```rust
pub fn new(content: impl Into<CompactString>) -> Self;
```

### `Thumbnail`

A small thumbnail image inside a component layout.

```rust
pub struct Thumbnail
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `media` | `Box<UnfurledMediaItem>` | The media. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `spoiler` | `bool` | Whether the attachment is marked as a spoiler. |

#### Methods

##### `new`

Creates a new `Thumbnail`.

```rust
pub fn new(media: UnfurledMediaItem) -> Self;
```

### `MediaGallery`

A gallery of images or videos inside a component layout.

```rust
pub struct MediaGallery
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `items` | `Vec<MediaGalleryItem>` | The items. |

#### Methods

##### `new`

Creates a new `MediaGallery`.

```rust
pub const fn new(items: Vec<MediaGalleryItem>) -> Self;
```

### `MediaGalleryItem`

One item in a media gallery.

```rust
pub struct MediaGalleryItem
```

| Field | Type | Description |
| --- | --- | --- |
| `media` | `UnfurledMediaItem` | The media. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `spoiler` | `bool` | Whether the attachment is marked as a spoiler. |

#### Methods

##### `new`

Creates a new `MediaGalleryItem`.

```rust
pub const fn new(media: UnfurledMediaItem) -> Self;
```

### `File`

A file displayed inside a component layout.

```rust
pub struct File
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `file` | `Box<UnfurledMediaItem>` | The file. |
| `spoiler` | `bool` | Whether the attachment is marked as a spoiler. |
| `name` | `Option<CompactString>` | The object's name. |
| `size` | `Option<u64>` | The size, in pixels for images. |

#### Methods

##### `new`

Creates a new `File`.

```rust
pub fn new(file: UnfurledMediaItem) -> Self;
```

### `UnfurledMediaItem`

A media reference that Discord unfurls.

```rust
pub struct UnfurledMediaItem
```

| Field | Type | Description |
| --- | --- | --- |
| `url` | `CompactString` | The URL. |
| `proxy_url` | `Option<CompactString>` | The proxied media URL. |
| `height` | `Option<u32>` | The height in pixels. |
| `width` | `Option<u32>` | The width in pixels. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `placeholder_version` | `Option<u32>` | The placeholder version. |
| `content_type` | `Option<CompactString>` | The content type. |
| `loading_state` | `Option<LoadingState>` | The loading state. |
| `flags` | `Option<UnfurledMediaItemFlags>` | Bitfield of flags attached to the object. |
| `content_scan_metadata` | `Option<ContentScanMetadata>` | The content scan metadata. |
| `attachment_id` | `Option<AttachmentId>` | The ID of the attachment. |

#### Methods

##### `new`

Creates a new `UnfurledMediaItem`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

### `LoadingState`

Whether a media item has finished loading.

```rust
pub enum LoadingState: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Unspecified` | `,` | Unspecified. |
| `Loading` | `,` | Loading. |
| `LoadedSuccess` | `,` | Loaded Success. |
| `LoadedNotFound` | `,` | Loaded Not Found. |

### `ContentScanMetadata`

Discord's content scan metadata object.

```rust
pub struct ContentScanMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `flags` | `ContentScanMetadataFlags` | Bitfield of flags attached to the object. |

## `model.component.interactive`

### `Button`

A button: interactive, link or premium.

```rust
pub enum Button
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Interactive` | `InteractiveButton` | Interactive. |
| `Link` | `LinkButton` | Link. |
| `Premium` | `PremiumButton` | Premium. |
| `Unknown` | `Box<UnknownButton>` | A value this build does not recognize. |

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> Option<i32>;
```

##### `style`

Returns the style.

```rust
pub const fn style(&self) -> ButtonStyle;
```

##### `custom_id`

Returns the component's custom ID.

```rust
pub fn custom_id(&self) -> Option<&str>;
```

##### `disabled`

Disables the button.

```rust
pub const fn disabled(&self) -> bool;
```

### `InteractiveButton`

Discord's interactive button object.

```rust
pub struct InteractiveButton
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `style` | `ButtonStyle` | The style. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `label` | `Option<CompactString>` | The human-readable label. |
| `emoji` | `Option<PartialEmoji>` | The emoji shown with the object. |
| `disabled` | `bool` | Whether the component is disabled. |

#### Methods

##### `new`

Creates a new `InteractiveButton`.

```rust
pub fn new(style: ButtonStyle, custom_id: impl Into<CompactString>) -> Self;
```

### `LinkButton`

Discord's link button object.

```rust
pub struct LinkButton
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `url` | `CompactString` | The URL. |
| `label` | `Option<CompactString>` | The human-readable label. |
| `emoji` | `Option<PartialEmoji>` | The emoji shown with the object. |
| `disabled` | `bool` | Whether the component is disabled. |

#### Methods

##### `new`

Creates a new `LinkButton`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

### `PremiumButton`

Discord's premium button object.

```rust
pub struct PremiumButton
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `sku_id` | `SkuId` | The ID of the SKU. |
| `disabled` | `bool` | Whether the component is disabled. |

#### Methods

##### `new`

Creates a new `PremiumButton`.

```rust
pub const fn new(sku_id: SkuId) -> Self;
```

### `UnknownButton`

Discord's unknown button object.

```rust
pub struct UnknownButton
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `style` | `ButtonStyle` | The style. |
| `custom_id` | `Option<CompactString>` | The developer-defined ID used to route component interactions. |
| `label` | `Option<CompactString>` | The human-readable label. |
| `emoji` | `Option<PartialEmoji>` | The emoji shown with the object. |
| `url` | `Option<CompactString>` | The URL. |
| `sku_id` | `Option<SkuId>` | The ID of the SKU. |
| `disabled` | `bool` | Whether the component is disabled. |

### `ButtonStyle`

The visual style of a button.

```rust
pub enum ButtonStyle: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Primary` | `,` | A blurple button, the main action. |
| `Secondary` | `,` | A grey button. |
| `Success` | `,` | A green button. |
| `Danger` | `,` | A red button. |
| `Link` | `,` | A button that opens a URL. |
| `Premium` | `,` | A SKU purchase button. |

### `StringSelect`

A select menu of fixed string options.

```rust
pub struct StringSelect
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `options` | `Vec<SelectOption>` | The selectable options or command options. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `min_values` | `Option<u8>` | The min values. |
| `max_values` | `Option<u8>` | The max values. |
| `required` | `bool` | Whether the option must be supplied. |
| `disabled` | `bool` | Whether the component is disabled. |

#### Methods

##### `new`

Creates a new `StringSelect`.

```rust
pub fn new(custom_id: impl Into<CompactString>, options: Vec<SelectOption>) -> Self;
```

### `SelectOption`

One option in a string select menu.

```rust
pub struct SelectOption
```

| Field | Type | Description |
| --- | --- | --- |
| `label` | `CompactString` | The human-readable label. |
| `value` | `CompactString` | The option's value. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `emoji` | `Option<PartialEmoji>` | The emoji shown with the object. |
| `default` | `bool` | The default value. |

#### Methods

##### `new`

Creates a new `SelectOption`.

```rust
pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self;
```

### `TextInput`

A short or paragraph text input inside a modal.

```rust
pub struct TextInput
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `style` | `TextInputStyle` | The style. |
| `label` | `Option<CompactString>` | The human-readable label. |
| `min_length` | `Option<u16>` | The min length. |
| `max_length` | `Option<u16>` | The max length. |
| `required` | `bool` | Whether the option must be supplied. |
| `value` | `Option<CompactString>` | The option's value. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |

#### Methods

##### `new`

Creates a new `TextInput`.

```rust
pub fn new(custom_id: impl Into<CompactString>, style: TextInputStyle) -> Self;
```

### `TextInputStyle`

Whether a text input is one line or a paragraph.

```rust
pub enum TextInputStyle: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Short` | `,` | A single-line text input. |
| `Paragraph` | `,` | A multi-line text input. |

### `AutoPopulatedSelect`

A select menu populated by Discord: users, roles or mentionables.

```rust
pub struct AutoPopulatedSelect
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `default_values` | `Vec<SelectDefaultValue>` | The default values. |
| `min_values` | `Option<u8>` | The min values. |
| `max_values` | `Option<u8>` | The max values. |
| `required` | `bool` | Whether the option must be supplied. |
| `disabled` | `bool` | Whether the component is disabled. |

#### Methods

##### `new`

Creates a new `AutoPopulatedSelect`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

### `ChannelSelect`

A select menu of channels, optionally filtered by type.

```rust
pub struct ChannelSelect
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `channel_types` | `Vec<ChannelType>` | The channel types. |
| `placeholder` | `Option<CompactString>` | Placeholder text shown before a value is chosen. |
| `default_values` | `Vec<SelectDefaultValue>` | The default values. |
| `min_values` | `Option<u8>` | The min values. |
| `max_values` | `Option<u8>` | The max values. |
| `required` | `bool` | Whether the option must be supplied. |
| `disabled` | `bool` | Whether the component is disabled. |

#### Methods

##### `new`

Creates a new `ChannelSelect`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

### `SelectDefaultValue`

A value selected by default in a select menu.

```rust
pub struct SelectDefaultValue
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GenericId` | The object's snowflake ID. |
| `kind` | `SelectDefaultValueType` | The object's type discriminator. |

#### Methods

##### `new`

Creates a new `SelectDefaultValue`.

```rust
pub fn new(id: GenericId, kind: SelectDefaultValueType) -> Self;
```

### `SelectDefaultValueType`

Possible select default value type values.

```rust
pub enum SelectDefaultValueType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `User` | `,` | User. |
| `Role` | `,` | Role. |
| `Channel` | `,` | Channel. |

### `FileUpload`

A file upload component inside a modal.

```rust
pub struct FileUpload
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `min_values` | `Option<u8>` | The min values. |
| `max_values` | `Option<u8>` | The max values. |
| `required` | `bool` | Whether the option must be supplied. |
| `file_types` | `Vec<CompactString>` | The file types. |

#### Methods

##### `new`

Creates a new `FileUpload`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

### `RadioGroup`

A radio group component inside a modal.

```rust
pub struct RadioGroup
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `options` | `Vec<GroupOption>` | The selectable options or command options. |
| `required` | `bool` | Whether the option must be supplied. |

#### Methods

##### `new`

Creates a new `RadioGroup`.

```rust
pub fn new(custom_id: impl Into<CompactString>, options: Vec<GroupOption>) -> Self;
```

### `CheckboxGroup`

A checkbox group component inside a modal.

```rust
pub struct CheckboxGroup
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `options` | `Vec<GroupOption>` | The selectable options or command options. |
| `min_values` | `Option<u8>` | The min values. |
| `max_values` | `Option<u8>` | The max values. |
| `required` | `bool` | Whether the option must be supplied. |

#### Methods

##### `new`

Creates a new `CheckboxGroup`.

```rust
pub fn new(custom_id: impl Into<CompactString>, options: Vec<GroupOption>) -> Self;
```

### `GroupOption`

One option inside a radio or checkbox group.

```rust
pub struct GroupOption
```

| Field | Type | Description |
| --- | --- | --- |
| `value` | `CompactString` | The option's value. |
| `label` | `CompactString` | The human-readable label. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `default` | `bool` | The default value. |

#### Methods

##### `new`

Creates a new `GroupOption`.

```rust
pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self;
```

### `Checkbox`

A single checkbox inside a modal.

```rust
pub struct Checkbox
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `default` | `bool` | The default value. |

#### Methods

##### `new`

Creates a new `Checkbox`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

## `model.component.layout`

### `ActionRow`

A horizontal row of buttons or one select menu.

```rust
pub struct ActionRow
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `components` | `Vec<Component>` | The components attached to the message. |

#### Methods

##### `new`

Creates a new `ActionRow`.

```rust
pub const fn new(components: Vec<Component>) -> Self;
```

### `Section`

A section that pairs text with an accessory component.

```rust
pub struct Section
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `accessory` | `Box<Component>` | The accessory. |

#### Methods

##### `new`

Creates a new `Section`.

```rust
pub fn new(components: Vec<Component>, accessory: Component) -> Self;
```

### `Container`

A bordered container that groups components.

```rust
pub struct Container
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `accent_color` | `Option<u32>` | The accent color. |
| `spoiler` | `bool` | Whether the attachment is marked as a spoiler. |

#### Methods

##### `new`

Creates a new `Container`.

```rust
pub const fn new(components: Vec<Component>) -> Self;
```

### `Separator`

A visual divider with spacing size.

```rust
pub struct Separator
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `divider` | `bool` | The divider. |
| `spacing` | `Option<SeparatorSpacing>` | The spacing. |

#### Methods

##### `new`

Creates a new `Separator`.

```rust
pub const fn new() -> Self;
```

### `SeparatorSpacing`

Possible separator spacing values.

```rust
pub enum SeparatorSpacing: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Small` | `,` | Small. |
| `Large` | `,` | Large. |

### `Label`

A label wrapping a modal input.

```rust
pub struct Label
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<i32>` | The object's snowflake ID. |
| `label` | `CompactString` | The human-readable label. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `component` | `Box<Component>` | The component. |

#### Methods

##### `new`

Creates a new `Label`.

```rust
pub fn new(label: impl Into<CompactString>, component: Component) -> Self;
```

## `model.poll`

### `Poll`

A poll attached to a message, with answers, expiry and results.

```rust
pub struct Poll
```

| Field | Type | Description |
| --- | --- | --- |
| `question` | `Media` | The question. |
| `answers` | `Vec<Answer>` | The answers. |
| `expiry` | `Option<Timestamp>` | The expiry. |
| `allow_multiselect` | `bool` | The allow multiselect. |
| `layout_type` | `LayoutType` | The layout type. |
| `results` | `Option<Results>` | The results. |

### `LayoutType`

The default answer layout: one column or two.

```rust
pub enum LayoutType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Default` | `,` | Default. |

### `Media`

An image or video attached to a poll.

```rust
pub struct Media
```

| Field | Type | Description |
| --- | --- | --- |
| `text` | `Option<CompactString>` | The text. |
| `emoji` | `Option<PartialEmoji>` | The emoji shown with the object. |

#### Methods

##### `new`

Creates a new `Media`.

```rust
pub fn new(text: impl Into<CompactString>) -> Self;
```

### `Answer`

One answer in a poll.

```rust
pub struct Answer
```

| Field | Type | Description |
| --- | --- | --- |
| `answer_id` | `u32` | The ID of the poll answer. |
| `poll_media` | `Media` | The poll media. |

### `Results`

The current vote counts and whether the poll is final.

```rust
pub struct Results
```

| Field | Type | Description |
| --- | --- | --- |
| `is_finalized` | `bool` | Whether this is finalized. |
| `answer_counts` | `Vec<AnswerCount>` | The answer counts. |

### `AnswerCount`

Discord's answer count object.

```rust
pub struct AnswerCount
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `u32` | The object's snowflake ID. |
| `count` | `u32` | The count. |
| `me_voted` | `bool` | The me voted. |

### `CreatePoll`

The poll to attach when sending a message.

```rust
pub struct CreatePoll
```

| Field | Type | Description |
| --- | --- | --- |
| `question` | `Media` | The question. |
| `answers` | `Vec<Media>` | The answers. |
| `duration` | `Option<u32>` | The duration. |
| `allow_multiselect` | `Option<bool>` | The allow multiselect. |
| `layout_type` | `Option<LayoutType>` | The layout type. |

#### Methods

##### `new`

Creates a new `CreatePoll`.

```rust
pub fn new( question: impl Into<CompactString>, answers: impl IntoIterator<Item = Media>, ) -> Self;
```
