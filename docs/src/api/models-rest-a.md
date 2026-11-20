# REST models: A to G

Request and response shapes for the REST endpoints, part one.

## `model.rest.application`

### `EditCurrentApplication`

Request body for the matching REST call.

```rust
pub struct EditCurrentApplication
```

| Field | Type | Description |
| --- | --- | --- |
| `custom_install_url` | `Option<CompactString>` | The URL for the custom install. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `role_connections_verification_url` | `Option<CompactString>` | The URL for the role connections verification. |
| `install_params` | `Option<InstallParams>` | The install params. |
| `integration_types_config` | `Option<IntegrationTypesConfig>` | The integration types config. |
| `flags` | `Option<ApplicationFlags>` | Bitfield of flags attached to the object. |
| `icon` | `Patch<ImageData>` | The icon hash. |
| `cover_image` | `Patch<ImageData>` | The cover image hash. |
| `interactions_endpoint_url` | `Option<CompactString>` | The URL for the interactions endpoint. |
| `tags` | `Option<Vec<CompactString>>` | Links to the bot, integration or boost that created the role. |
| `event_webhooks_url` | `Option<CompactString>` | The URL for the event webhooks. |
| `event_webhooks_status` | `Option<EventWebhookStatus>` | The event webhooks status. |
| `event_webhooks_types` | `Option<Vec<WebhookEventType>>` | The event webhooks types. |

## `model.rest.application_command`

### `CreateApplicationCommand`

Request body for the matching REST call.

```rust
pub struct CreateApplicationCommand
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `options` | `Vec<CommandOption>` | The selectable options or command options. |
| `default_member_permissions` | `Option<Permissions>` | Permissions required by default for members to use the command. |
| `integration_types` | `Vec<IntegrationType>` | The installation contexts where the command is available. |
| `contexts` | `Vec<InteractionContextType>` | The interaction contexts where the command is available. |
| `kind` | `Option<CommandType>` | The object's type discriminator. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `handler` | `Option<EntryPointHandlerType>` | The handler this is attached to. |

#### Methods

##### `new`

Creates a new `CreateApplicationCommand`.

```rust
pub fn new(name: impl Into<CompactString>, kind: CommandType) -> Self;
```

##### `chat_input`

Builds a chat-input command from a name and description.

```rust
pub fn chat_input( name: impl Into<CompactString>, description: impl Into<CompactString>, ) -> Self;
```

##### `user`

Builds a user context-menu command.

```rust
pub fn user(name: impl Into<CompactString>) -> Self;
```

##### `message`

Builds a message context-menu command.

```rust
pub fn message(name: impl Into<CompactString>) -> Self;
```

### `EditApplicationCommand`

Request body for the matching REST call.

```rust
pub struct EditApplicationCommand
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `name_localizations` | `Patch<BTreeMap<Locale, CompactString>>` | Translated names keyed by locale. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `description_localizations` | `Patch<BTreeMap<Locale, CompactString>>` | Translated descriptions keyed by locale. |
| `options` | `Option<Vec<CommandOption>>` | The selectable options or command options. |
| `default_member_permissions` | `Patch<Permissions>` | Permissions required by default for members to use the command. |
| `integration_types` | `Option<Vec<IntegrationType>>` | The installation contexts where the command is available. |
| `contexts` | `Option<Vec<InteractionContextType>>` | The interaction contexts where the command is available. |
| `nsfw` | `Option<bool>` | Whether the channel or content is age-restricted. |
| `handler` | `Option<EntryPointHandlerType>` | The handler this is attached to. |

### `GetApplicationCommands`

Query parameters for the matching REST call.

```rust
pub struct GetApplicationCommands
```

| Field | Type | Description |
| --- | --- | --- |
| `with_localizations` | `Option<bool>` | Localized with values. |

### `EditApplicationCommandPermissions`

Request body for the matching REST call.

```rust
pub struct EditApplicationCommandPermissions
```

| Field | Type | Description |
| --- | --- | --- |
| `permissions` | `Vec<CommandPermission>` | The permissions bitfield. |

#### Methods

##### `new`

Creates a new `EditApplicationCommandPermissions`.

```rust
pub const fn new(permissions: Vec<CommandPermission>) -> Self;
```

## `model.rest.application_identity_profile`

### `UpdateApplicationIdentityProfile`

Request body for the matching REST call.

```rust
pub struct UpdateApplicationIdentityProfile
```

| Field | Type | Description |
| --- | --- | --- |
| `username` | `Option<CompactString>` | The account username. |
| `data` | `Option<ProfileData>` | The data. |

### `GetApplicationIdentitiesByExternalId`

Query parameters for the matching REST call.

```rust
pub struct GetApplicationIdentitiesByExternalId
```

| Field | Type | Description |
| --- | --- | --- |
| `provider_id` | `Option<CompactString>` | The ID of the provider. |

### `DeleteApplicationIdentity`

Request body for the matching REST call.

```rust
pub struct DeleteApplicationIdentity
```

| Field | Type | Description |
| --- | --- | --- |
| `provider_id` | `Option<CompactString>` | The ID of the provider. |

## `model.rest.audit_log`

### `GetGuildAuditLog`

Query parameters for the matching REST call.

```rust
pub struct GetGuildAuditLog
```

| Field | Type | Description |
| --- | --- | --- |
| `user_id` | `Option<UserId>` | The ID of the user. |
| `action_type` | `Option<AuditLogEvent>` | The action type. |
| `before` | `Option<AuditLogEntryId>` | Return items before this ID. |
| `after` | `Option<AuditLogEntryId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |

## `model.rest.auto_moderation`

### `CreateAutoModerationRule`

Request body for the matching REST call.

```rust
pub struct CreateAutoModerationRule
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `event_type` | `EventType` | The type of event. |
| `trigger_type` | `TriggerType` | The trigger type. |
| `trigger_metadata` | `TriggerMetadata` | The trigger metadata. |
| `actions` | `Vec<Action>` | The actions configured on the object. |
| `enabled` | `Option<bool>` | Whether the feature is enabled. |
| `exempt_roles` | `Vec<RoleId>` | The roles exempt from the rule. |
| `exempt_channels` | `Vec<ChannelId>` | The channels exempt from the rule. |

#### Methods

##### `keyword`

Watches for custom keywords.

```rust
pub fn keyword( name: impl Into<CompactString>, keywords: impl IntoIterator<Item = impl Into<CompactString>>, actions: impl IntoIterator<Item = Action>, ) -> Self;
```

##### `spam`

Watches for spam.

```rust
pub fn spam(name: impl Into<CompactString>, actions: impl IntoIterator<Item = Action>) -> Self;
```

##### `keyword_preset`

Watches a Discord keyword preset list.

```rust
pub fn keyword_preset( name: impl Into<CompactString>, presets: impl IntoIterator<Item = KeywordPresetType>, actions: impl IntoIterator<Item = Action>, ) -> Self;
```

##### `mention_spam`

Watches for mention spam.

```rust
pub fn mention_spam( name: impl Into<CompactString>, limit: u8, actions: impl IntoIterator<Item = Action>, ) -> Self;
```

##### `member_profile`

Watches member profile content.

```rust
pub fn member_profile( name: impl Into<CompactString>, keywords: impl IntoIterator<Item = impl Into<CompactString>>, actions: impl IntoIterator<Item = Action>, ) -> Self;
```

### `ModifyAutoModerationRule`

Request body for the matching REST call.

```rust
pub struct ModifyAutoModerationRule
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `event_type` | `Option<EventType>` | The type of event. |
| `trigger_metadata` | `Option<TriggerMetadata>` | The trigger metadata. |
| `actions` | `Option<Vec<Action>>` | The actions configured on the object. |
| `enabled` | `Option<bool>` | Whether the feature is enabled. |
| `exempt_roles` | `Option<Vec<RoleId>>` | The roles exempt from the rule. |
| `exempt_channels` | `Option<Vec<ChannelId>>` | The channels exempt from the rule. |

## `model.rest.channel`

### `ModifyGuildChannel`

Request body for the matching REST call.

```rust
pub struct ModifyGuildChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `icon` | `Patch<ImageData>` | The icon hash. |
| `kind` | `Option<ChannelType>` | The object's type discriminator. |
| `position` | `Patch<i32>` | The position among siblings, lowest first. |
| `topic` | `Patch<CompactString>` | The topic. |
| `nsfw` | `Patch<bool>` | Whether the channel or content is age-restricted. |
| `rate_limit_per_user` | `Patch<u16>` | Slowmode delay in seconds between messages per user. |
| `bitrate` | `Patch<u32>` | The voice channel's bitrate in bits per second. |
| `user_limit` | `Patch<u32>` | The maximum number of users allowed in the voice channel. |
| `permission_overwrites` | `Patch<Vec<Overwrite>>` | Channel-specific permission overwrites. |
| `parent_id` | `Patch<ChannelId>` | The ID of the parent channel or category. |
| `rtc_region` | `Patch<CompactString>` | The preferred voice region. |
| `video_quality_mode` | `Patch<VideoQualityMode>` | The channel's video quality mode. |
| `default_auto_archive_duration` | `Patch<AutoArchiveDuration>` | The default auto-archive duration for new threads. |
| `flags` | `Option<ChannelFlags>` | Bitfield of flags attached to the object. |
| `available_tags` | `Option<Vec<ForumTagData>>` | The tags that can be applied to new forum posts. |
| `default_reaction_emoji` | `Patch<DefaultReaction>` | The default reaction emoji for new forum posts. |
| `default_thread_rate_limit_per_user` | `Option<u16>` | The default slowmode for new threads. |
| `default_sort_order` | `Patch<SortOrderType>` | The default forum sort order. |
| `default_forum_layout` | `Option<ForumLayoutType>` | The default forum layout. |

### `ModifyThread`

Request body for the matching REST call.

```rust
pub struct ModifyThread
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `archived` | `Option<bool>` | Whether the thread is archived. |
| `auto_archive_duration` | `Option<AutoArchiveDuration>` | How long before the thread auto-archives, in minutes. |
| `locked` | `Option<bool>` | Whether only moderators can unarchive the thread. |
| `invitable` | `Option<bool>` | Whether non-members can be invited to the thread. |
| `rate_limit_per_user` | `Patch<u16>` | Slowmode delay in seconds between messages per user. |
| `flags` | `Option<ChannelFlags>` | Bitfield of flags attached to the object. |
| `applied_tags` | `Option<Vec<TagId>>` | The forum tags applied to this thread. |

### `ModifyGroupDm`

Request body for the matching REST call.

```rust
pub struct ModifyGroupDm
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `icon` | `Patch<ImageData>` | The icon hash. |

### `ForumTagData`

Response shape for the matching REST call.

```rust
pub struct ForumTagData
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<TagId>` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `moderated` | `bool` | Whether the content is moderated. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |

#### Methods

##### `new`

Creates a new `ForumTagData`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `GetChannelPermissions`

Query parameters for the matching REST call.

```rust
pub struct GetChannelPermissions
```

| Field | Type | Description |
| --- | --- | --- |
| `implicit` | `Option<bool>` | The implicit. |

### `SetVoiceChannelStatus`

Request body for the matching REST call.

```rust
pub struct SetVoiceChannelStatus
```

| Field | Type | Description |
| --- | --- | --- |
| `status` | `Option<CompactString>` | The current status. |

#### Methods

##### `new`

Creates a new `SetVoiceChannelStatus`.

```rust
pub fn new(status: Option<impl Into<CompactString>>) -> Self;
```

### `EditChannelPermissions`

Request body for the matching REST call.

```rust
pub struct EditChannelPermissions
```

| Field | Type | Description |
| --- | --- | --- |
| `allow` | `Option<Permissions>` | The permissions explicitly allowed by this overwrite. |
| `deny` | `Option<Permissions>` | The permissions explicitly denied by this overwrite. |
| `kind` | `OverwriteType` | The object's type discriminator. |

#### Methods

##### `new`

Creates a new `EditChannelPermissions`.

```rust
pub const fn new(kind: OverwriteType) -> Self;
```

### `CreateChannelInvite`

Request body for the matching REST call.

```rust
pub struct CreateChannelInvite
```

| Field | Type | Description |
| --- | --- | --- |
| `max_age` | `Option<u32>` | How long the invite stays valid, in seconds. |
| `max_uses` | `Option<u8>` | How many times the invite can be used. |
| `temporary` | `bool` | The temporary. |
| `unique` | `bool` | The unique. |
| `target_type` | `Option<InviteTargetType>` | The target type. |
| `target_user_id` | `Option<UserId>` | The ID of the target user. |
| `target_application_id` | `Option<ApplicationId>` | The ID of the target application. |
| `target_user_ids` | `Vec<UserId>` | The IDs of the target user. |
| `role_ids` | `Vec<RoleId>` | The IDs of the roles. |
| `target_users_file` | `Option<FileUpload>` | The target users file. |

### `FollowAnnouncementChannel`

Request body for the matching REST call.

```rust
pub struct FollowAnnouncementChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `webhook_channel_id` | `ChannelId` | The ID of the webhook channel. |

#### Methods

##### `new`

Creates a new `FollowAnnouncementChannel`.

```rust
pub const fn new(webhook_channel_id: ChannelId) -> Self;
```

### `GroupDmAddRecipient`

Request or response shape for the matching REST call.

```rust
pub struct GroupDmAddRecipient
```

| Field | Type | Description |
| --- | --- | --- |
| `access_token` | `String` | The OAuth2 access token. |
| `nick` | `Option<CompactString>` | The member's guild nickname, when set. |

#### Methods

##### `new`

Creates a new `GroupDmAddRecipient`.

```rust
pub fn new(access_token: impl Into<String>) -> Self;
```

### `StartThreadFromMessage`

Request body for the matching REST call.

```rust
pub struct StartThreadFromMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `auto_archive_duration` | `Option<AutoArchiveDuration>` | How long before the thread auto-archives, in minutes. |
| `rate_limit_per_user` | `Option<u16>` | Slowmode delay in seconds between messages per user. |

#### Methods

##### `new`

Creates a new `StartThreadFromMessage`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `StartThreadWithoutMessage`

Request body for the matching REST call.

```rust
pub struct StartThreadWithoutMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `auto_archive_duration` | `Option<AutoArchiveDuration>` | How long before the thread auto-archives, in minutes. |
| `kind` | `ChannelType` | The object's type discriminator. |
| `invitable` | `Option<bool>` | Whether non-members can be invited to the thread. |
| `rate_limit_per_user` | `Option<u16>` | Slowmode delay in seconds between messages per user. |

#### Methods

##### `new`

Creates a new `StartThreadWithoutMessage`.

```rust
pub fn new(name: impl Into<CompactString>, kind: ChannelType) -> Self;
```

### `StartThreadInForumOrMediaChannel`

Request body for the matching REST call.

```rust
pub struct StartThreadInForumOrMediaChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `auto_archive_duration` | `Option<AutoArchiveDuration>` | How long before the thread auto-archives, in minutes. |
| `rate_limit_per_user` | `Option<u16>` | Slowmode delay in seconds between messages per user. |
| `message` | `ForumThreadMessage` | The message this refers to. |
| `applied_tags` | `Vec<TagId>` | The forum tags applied to this thread. |

#### Methods

##### `new`

Creates a new `StartThreadInForumOrMediaChannel`.

```rust
pub fn new(name: impl Into<CompactString>, message: ForumThreadMessage) -> Self;
```

### `ForumThreadMessage`

Request or response shape for the matching REST call.

```rust
pub struct ForumThreadMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `content` | `Option<CompactString>` | The message text. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `allowed_mentions` | `Option<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `sticker_ids` | `Vec<StickerId>` | The IDs of the sticker. |
| `attachments` | `Option<Vec<AttachmentRequest>>` | The files attached to the message. |
| `flags` | `Option<MessageFlags>` | Bitfield of flags attached to the object. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |

### `ForumThread`

Request or response shape for the matching REST call.

```rust
pub struct ForumThread
```

| Field | Type | Description |
| --- | --- | --- |
| `channel` | `Channel` | The channel. |
| `message` | `Option<Message>` | The message this refers to. |

### `GetThreadMember`

Query parameters for the matching REST call.

```rust
pub struct GetThreadMember
```

| Field | Type | Description |
| --- | --- | --- |
| `with_member` | `Option<bool>` | The with member. |

### `ListThreadMembers`

Query parameters for the matching REST call.

```rust
pub struct ListThreadMembers
```

| Field | Type | Description |
| --- | --- | --- |
| `with_member` | `Option<bool>` | The with member. |
| `after` | `Option<UserId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |

### `ListArchivedThreads`

Query parameters for the matching REST call.

```rust
pub struct ListArchivedThreads
```

| Field | Type | Description |
| --- | --- | --- |
| `before` | `Option<Timestamp>` | Return items before this ID. |
| `limit` | `Option<u16>` | The maximum number of items to return. |

### `ListJoinedPrivateArchivedThreads`

Query parameters for the matching REST call.

```rust
pub struct ListJoinedPrivateArchivedThreads
```

| Field | Type | Description |
| --- | --- | --- |
| `before` | `Option<ChannelId>` | Return items before this ID. |
| `limit` | `Option<u16>` | The maximum number of items to return. |

### `ArchivedThreads`

Request or response shape for the matching REST call.

```rust
pub struct ArchivedThreads
```

| Field | Type | Description |
| --- | --- | --- |
| `threads` | `Vec<Channel>` | The threads in this channel. |
| `members` | `Vec<ThreadMember>` | The members in this collection. |
| `has_more` | `bool` | Whether more results are available. |

## `model.rest.emoji`

### `CreateGuildEmoji`

Request body for the matching REST call.

```rust
pub struct CreateGuildEmoji
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `image` | `ImageData` | The image hash or URL. |
| `roles` | `Vec<RoleId>` | The roles attached or referenced. |

#### Methods

##### `new`

Creates a new `CreateGuildEmoji`.

```rust
pub fn new(name: impl Into<CompactString>, image: ImageData) -> Self;
```

### `ModifyGuildEmoji`

Request body for the matching REST call.

```rust
pub struct ModifyGuildEmoji
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `roles` | `Patch<Vec<RoleId>>` | The roles attached or referenced. |

### `CreateApplicationEmoji`

Request body for the matching REST call.

```rust
pub struct CreateApplicationEmoji
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `image` | `ImageData` | The image hash or URL. |

#### Methods

##### `new`

Creates a new `CreateApplicationEmoji`.

```rust
pub fn new(name: impl Into<CompactString>, image: ImageData) -> Self;
```

### `ModifyApplicationEmoji`

Request body for the matching REST call.

```rust
pub struct ModifyApplicationEmoji
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |

## `model.rest.entitlement`

### `ListEntitlements`

Query parameters for the matching REST call.

```rust
pub struct ListEntitlements
```

| Field | Type | Description |
| --- | --- | --- |
| `user_id` | `Option<UserId>` | The ID of the user. |
| `sku_ids` | `Vec<SkuId>` | The IDs of the sku. |
| `before` | `Option<EntitlementId>` | Return items before this ID. |
| `after` | `Option<EntitlementId>` | Return items after this ID. |
| `limit` | `Option<u8>` | The maximum number of items to return. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `exclude_ended` | `Option<bool>` | The exclude ended. |
| `exclude_deleted` | `Option<bool>` | The exclude deleted. |

### `CreateTestEntitlement`

Request body for the matching REST call.

```rust
pub struct CreateTestEntitlement
```

| Field | Type | Description |
| --- | --- | --- |
| `sku_id` | `SkuId` | The ID of the SKU. |
| `owner_id` | `GenericId` | The ID of the owner. |
| `owner_type` | `EntitlementOwnerType` | The owner type. |

#### Methods

##### `guild`

Scopes the test entitlement to a guild.

```rust
pub const fn guild(sku_id: SkuId, guild_id: GuildId) -> Self;
```

##### `user`

Builds a user context-menu command.

```rust
pub const fn user(sku_id: SkuId, user_id: UserId) -> Self;
```

## `model.rest.error_code`

### `ErrorCode`

Possible error code values.

```rust
pub enum ErrorCode: u32
```

| Variant | Payload | Description |
| --- | --- | --- |
| `GeneralError` | `,` | General Error. |
| `UnknownAccount` | `,` | A value this build does not recognize. |
| `UnknownApplication` | `,` | A value this build does not recognize. |
| `UnknownChannel` | `,` | A value this build does not recognize. |
| `UnknownGuild` | `,` | A value this build does not recognize. |
| `UnknownIntegration` | `,` | A value this build does not recognize. |
| `UnknownInvite` | `,` | A value this build does not recognize. |
| `UnknownMember` | `,` | A value this build does not recognize. |
| `UnknownMessage` | `,` | A value this build does not recognize. |
| `UnknownPermissionOverwrite` | `,` | A value this build does not recognize. |
| `UnknownProvider` | `,` | A value this build does not recognize. |
| `UnknownRole` | `,` | A value this build does not recognize. |
| `UnknownToken` | `,` | A value this build does not recognize. |
| `UnknownUser` | `,` | A value this build does not recognize. |
| `UnknownEmoji` | `,` | A value this build does not recognize. |
| `UnknownWebhook` | `,` | A value this build does not recognize. |
| `UnknownWebhookService` | `,` | A value this build does not recognize. |
| `UnknownSession` | `,` | A value this build does not recognize. |
| `UnknownAsset` | `,` | A value this build does not recognize. |
| `UnknownBan` | `,` | A value this build does not recognize. |
| `UnknownSku` | `,` | A value this build does not recognize. |
| `UnknownStoreListing` | `,` | A value this build does not recognize. |
| `UnknownEntitlement` | `,` | A value this build does not recognize. |
| `UnknownBuild` | `,` | A value this build does not recognize. |
| `UnknownLobby` | `,` | A value this build does not recognize. |
| `UnknownBranch` | `,` | A value this build does not recognize. |
| `UnknownStoreDirectoryLayout` | `,` | A value this build does not recognize. |
| `UnknownRedistributable` | `,` | A value this build does not recognize. |
| `UnknownGiftCode` | `,` | A value this build does not recognize. |
| `UnknownStream` | `,` | A value this build does not recognize. |
| `UnknownPremiumServerSubscribeCooldown` | `,` | A value this build does not recognize. |
| `UnknownGuildTemplate` | `,` | A value this build does not recognize. |
| `UnknownDiscoverableServerCategory` | `,` | A value this build does not recognize. |
| `UnknownSticker` | `,` | A value this build does not recognize. |
| `UnknownStickerPack` | `,` | A value this build does not recognize. |
| `UnknownInteraction` | `,` | A value this build does not recognize. |
| `UnknownApplicationCommand` | `,` | A value this build does not recognize. |
| `UnknownVoiceState` | `,` | A value this build does not recognize. |
| `UnknownApplicationCommandPermissions` | `,` | A value this build does not recognize. |
| `UnknownStageInstance` | `,` | A value this build does not recognize. |
| `UnknownGuildMemberVerificationForm` | `,` | A value this build does not recognize. |
| `UnknownGuildWelcomeScreen` | `,` | A value this build does not recognize. |
| `UnknownGuildScheduledEvent` | `,` | A value this build does not recognize. |
| `UnknownGuildScheduledEventUser` | `,` | A value this build does not recognize. |
| `UnknownTag` | `,` | A value this build does not recognize. |
| `UnknownSound` | `,` | A value this build does not recognize. |
| `UnknownInviteTargetUsersJob` | `,` | A value this build does not recognize. |
| `UnknownInviteTargetUsers` | `,` | A value this build does not recognize. |
| `BotsCannotUseThisEndpoint` | `,` | Bots Cannot Use This Endpoint. |
| `OnlyBotsCanUseThisEndpoint` | `,` | Only Bots Can Use This Endpoint. |
| `ExplicitContentCannotBeSentToTheDesiredRecipient` | `,` | Explicit Content Cannot Be Sent To The Desired Recipient. |
| `NotAuthorizedToPerformThisActionOnThisApplication` | `,` | Not Authorized To Perform This Action On This Application. |
| `ThisActionRequiresAPremiumSubscription` | `,` | This Action Requires A Premium Subscription. |
| `ActionCannotBePerformedDueToSlowmodeRateLimit` | `,` | Action Cannot Be Performed Due To Slowmode Rate Limit. |
| `OnlyTheOwnerOfThisAccountCanPerformThisAction` | `,` | Only The Owner Of This Account Can Perform This Action. |
| `AnnouncementEditLimitExceeded` | `,` | Announcement Edit Limit Exceeded. |
| `UnderMinimumAge` | `,` | Under Minimum Age. |
| `ChannelWriteRateLimit` | `,` | Channel Write Rate Limit. |
| `ServerWriteRateLimit` | `,` | Server Write Rate Limit. |
| `StageTopicServerNameServerDescriptionOrChannelNamesContainDisallowedWords` | `,` | Stage Topic Server Name Server Description Or Channel Names Contain Disallowed Words. |
| `GuildPremiumSubscriptionLevelTooLow` | `,` | Guild Premium Subscription Level Too Low. |
| `MaximumNumberOfGuildsReached` | `,` | Maximum Number Of Guilds Reached. |
| `MaximumNumberOfFriendsReached` | `,` | Maximum Number Of Friends Reached. |
| `MaximumNumberOfPinsReachedForTheChannel` | `,` | Maximum Number Of Pins Reached For The Channel. |
| `MaximumNumberOfRecipientsReached` | `,` | Maximum Number Of Recipients Reached. |
| `MaximumNumberOfGuildRolesReached` | `,` | Maximum Number Of Guild Roles Reached. |
| `MaximumNumberOfWebhooksReached` | `,` | Maximum Number Of Webhooks Reached. |
| `MaximumNumberOfEmojisReached` | `,` | Maximum Number Of Emojis Reached. |
| `MaximumNumberOfReactionsReached` | `,` | Maximum Number Of Reactions Reached. |
| `MaximumNumberOfGroupDmsReached` | `,` | Maximum Number Of Group Dms Reached. |
| `MaximumNumberOfGuildChannelsReached` | `,` | Maximum Number Of Guild Channels Reached. |
| `MaximumNumberOfAttachmentsInAMessageReached` | `,` | Maximum Number Of Attachments In A Message Reached. |
| `MaximumNumberOfInvitesReached` | `,` | Maximum Number Of Invites Reached. |
| `MaximumNumberOfAnimatedEmojisReached` | `,` | Maximum Number Of Animated Emojis Reached. |
| `MaximumNumberOfServerMembersReached` | `,` | Maximum Number Of Server Members Reached. |
| `MaximumNumberOfServerCategoriesReached` | `,` | Maximum Number Of Server Categories Reached. |
| `GuildAlreadyHasTemplate` | `,` | Guild Already Has Template. |
| `MaximumNumberOfApplicationCommandsReached` | `,` | Maximum Number Of Application Commands Reached. |
| `MaximumThreadParticipantsReached` | `,` | Maximum Thread Participants Reached. |
| `MaximumDailyApplicationCommandCreatesReached` | `,` | Maximum Daily Application Command Creates Reached. |
| `MaximumNumberOfNonGuildMemberBansHasBeenExceeded` | `,` | Maximum Number Of Non Guild Member Bans Has Been Exceeded. |
| `MaximumNumberOfBanFetchesHasBeenReached` | `,` | Maximum Number Of Ban Fetches Has Been Reached. |
| `MaximumNumberOfUncompletedGuildScheduledEventsReached` | `,` | Maximum Number Of Uncompleted Guild Scheduled Events Reached. |
| `MaximumNumberOfStickersReached` | `,` | Maximum Number Of Stickers Reached. |
| `MaximumNumberOfPruneRequestsHasBeenReached` | `,` | Maximum Number Of Prune Requests Has Been Reached. |
| `MaximumNumberOfGuildWidgetSettingsUpdatesHasBeenReached` | `,` | Maximum Number Of Guild Widget Settings Updates Has Been Reached. |
| `MaximumNumberOfSoundboardSoundsReached` | `,` | Maximum Number Of Soundboard Sounds Reached. |
| `MaximumNumberOfEditsToMessagesOlderThanOneHourReached` | `,` | Maximum Number Of Edits To Messages Older Than One Hour Reached. |
| `MaximumNumberOfPinnedThreadsInForumHasBeenReached` | `,` | Maximum Number Of Pinned Threads In Forum Has Been Reached. |
| `MaximumNumberOfTagsInForumHasBeenReached` | `,` | Maximum Number Of Tags In Forum Has Been Reached. |
| `BitrateIsTooHighForChannelOfThisType` | `,` | Bitrate Is Too High For Channel Of This Type. |
| `MaximumNumberOfPremiumEmojisReached` | `,` | Maximum Number Of Premium Emojis Reached. |
| `MaximumNumberOfWebhooksPerGuildReached` | `,` | Maximum Number Of Webhooks Per Guild Reached. |
| `MaximumNumberOfChannelPermissionOverwritesReached` | `,` | Maximum Number Of Channel Permission Overwrites Reached. |
| `TheChannelsForThisGuildAreTooLarge` | `,` | The Channels For This Guild Are Too Large. |
| `InviteTargetUsersLimitReached` | `,` | Invite Target Users Limit Reached. |
| `Unauthorized` | `,` | Unauthorized. |
| `VerifyYourAccount` | `,` | Verify Your Account. |
| `OpeningDirectMessagesTooFast` | `,` | Opening Direct Messages Too Fast. |
| `SendMessagesHasBeenTemporarilyDisabled` | `,` | Send Messages Has Been Temporarily Disabled. |
| `RequestEntityTooLarge` | `,` | Request Entity Too Large. |
| `FeatureTemporarilyDisabledServerSide` | `,` | Feature Temporarily Disabled Server Side. |
| `UserBannedFromThisGuild` | `,` | User Banned From This Guild. |
| `OnlyOneChannelCanHaveAParentIdModifiedAtATime` | `,` | Only One Channel Can Have A Parent ID Modified At A Time. |
| `ConnectionHasBeenRevoked` | `,` | Connection Has Been Revoked. |
| `OnlyConsumableSkusCanBeConsumed` | `,` | Only Consumable Skus Can Be Consumed. |
| `YouCanOnlyDeleteSandboxEntitlements` | `,` | You Can Only Delete Sandbox Entitlements. |
| `TargetUserIsNotConnectedToVoice` | `,` | Target User Is Not Connected To Voice. |
| `ThisMessageWasAlreadyCrossposted` | `,` | This Message Was Already Crossposted. |
| `ApplicationCommandWithThatNameAlreadyExists` | `,` | Application Command With That Name Already Exists. |
| `ApplicationInteractionFailedToSend` | `,` | Application Interaction Failed To Send. |
| `CannotSendAMessageInAForumChannel` | `,` | Cannot Send A Message In A Forum Channel. |
| `InteractionHasAlreadyBeenAcknowledged` | `,` | Interaction Has Already Been Acknowledged. |
| `TagNamesMustBeUnique` | `,` | Tag Names Must Be Unique. |
| `ServiceResourceIsBeingRateLimited` | `,` | Service Resource Is Being Rate Limited. |
| `ThereAreNoTagsAvailableThatCanBeSetByNonModerators` | `,` | There Are No Tags Available That Can Be Set By Non Moderators. |
| `TagRequiredToCreateAForumPostInThisChannel` | `,` | Tag Required To Create A Forum Post In This Channel. |
| `AnEntitlementHasAlreadyBeenGrantedForThisResource` | `,` | An Entitlement Has Already Been Granted For This Resource. |
| `ThisInteractionHasHitTheMaximumNumberOfFollowUpMessages` | `,` | This Interaction Has Hit The Maximum Number Of Follow Up Messages. |
| `InviteTargetUsersAlreadyBeingProcessed` | `,` | Invite Target Users Already Being Processed. |
| `InviteTargetUsersFileNotProcessed` | `,` | Invite Target Users File Not Processed. |
| `CloudflareIsBlockingYourRequest` | `,` | Cloudflare Is Blocking Your Request. |
| `MissingAccess` | `,` | Missing Access. |
| `InvalidAccountType` | `,` | Invalid Account Type. |
| `CannotExecuteActionOnDmChannel` | `,` | Cannot Execute Action On DM Channel. |
| `GuildWidgetDisabled` | `,` | Guild Widget Disabled. |
| `CannotEditMessageAuthoredByAnotherUser` | `,` | Cannot Edit Message Authored By Another User. |
| `CannotSendAnEmptyMessage` | `,` | Cannot Send An Empty Message. |
| `CannotSendMessagesToThisUser` | `,` | Cannot Send Messages To This User. |
| `CannotSendMessagesInNonTextChannel` | `,` | Cannot Send Messages In Non Text Channel. |
| `ChannelVerificationLevelTooHighForYouToGainAccess` | `,` | Channel Verification Level Too High For You To Gain Access. |
| `OAuth2ApplicationDoesNotHaveBot` | `,` | O Auth 2 Application Does Not Have Bot. |
| `OAuth2ApplicationLimitReached` | `,` | O Auth 2 Application Limit Reached. |
| `InvalidOAuth2State` | `,` | Invalid O Auth 2 State. |
| `MissingPermissions` | `,` | Missing Permissions. |
| `InvalidToken` | `,` | Invalid Token. |
| `NoteWasTooLong` | `,` | Note Was Too Long. |
| `ProvidedTooFewOrTooManyMessagesToDelete` | `,` | Provided Too Few Or Too Many Messages To Delete. |
| `InvalidMfaLevel` | `,` | Invalid MFA Level. |
| `MessageCanOnlyBePinnedInTheChannelItWasSentIn` | `,` | Message Can Only Be Pinned In The Channel It Was Sent In. |
| `InviteCodeInvalidOrTaken` | `,` | Invite Code Invalid Or Taken. |
| `CannotExecuteActionOnSystemMessage` | `,` | Cannot Execute Action On System Message. |
| `CannotExecuteActionOnThisChannelType` | `,` | Cannot Execute Action On This Channel Type. |
| `InvalidOAuth2AccessToken` | `,` | Invalid O Auth 2 Access Token. |
| `MissingRequiredOAuth2Scope` | `,` | Missing Required O Auth 2 Scope. |
| `InvalidWebhookToken` | `,` | Invalid Webhook Token. |
| `InvalidRole` | `,` | Invalid Role. |
| `InvalidRecipients` | `,` | Invalid Recipients. |
| `OneOfTheMessagesProvidedWasTooOldForBulkDelete` | `,` | One Of The Messages Provided Was Too Old For Bulk Delete. |
| `InvalidFormBodyOrContentType` | `,` | Invalid Form Body Or Content Type. |
| `InviteAcceptedToGuildWithoutTheBotBeingIn` | `,` | Invite Accepted To Guild Without The Bot Being In. |
| `InvalidActivityAction` | `,` | Invalid Activity Action. |
| `InvalidApiVersion` | `,` | Invalid API Version. |
| `FileUploadedExceedsMaximumSize` | `,` | File Uploaded Exceeds Maximum Size. |
| `InvalidFileUploaded` | `,` | Invalid File Uploaded. |
| `CannotSelfRedeemThisGift` | `,` | Cannot Self Redeem This Gift. |
| `InvalidGuild` | `,` | Invalid Guild. |
| `InvalidSku` | `,` | Invalid SKU. |
| `InvalidRequestOrigin` | `,` | Invalid Request Origin. |
| `InvalidMessageType` | `,` | Invalid Message Type. |
| `PaymentSourceRequiredToRedeemGift` | `,` | Payment Source Required To Redeem Gift. |
| `CannotModifyASystemWebhook` | `,` | Cannot Modify A System Webhook. |
| `CannotDeleteChannelRequiredForCommunityGuilds` | `,` | Cannot Delete Channel Required For Community Guilds. |
| `CannotEditStickersWithinMessage` | `,` | Cannot Edit Stickers Within Message. |
| `InvalidStickerSent` | `,` | Invalid Sticker Sent. |
| `InvalidActionOnArchivedThread` | `,` | Invalid Action On Archived Thread. |
| `InvalidThreadNotificationSettings` | `,` | Invalid Thread Notification Settings. |
| `ParameterEarlierThanCreation` | `,` | Parameter Earlier Than Creation. |
| `CommunityServerChannelsMustBeTextChannels` | `,` | Community Server Channels Must Be Text Channels. |
| `TheEntityTypeOfTheEventIsDifferentFromTheEntityYouAreTryingToStartTheEventFor` | `,` | The Entity Type Of The Event Is Different From The Entity You Are Trying To Start The Event For. |
| `ServerNotAvailableInYourLocation` | `,` | Server Not Available In Your Location. |
| `ServerNeedsMonetizationEnabledToPerformThisAction` | `,` | Server Needs Monetization Enabled To Perform This Action. |
| `ServerNeedsMoreBoostsToPerformThisAction` | `,` | Server Needs More Boosts To Perform This Action. |
| `RequestBodyContainsInvalidJson` | `,` | Request Body Contains Invalid Json. |
| `ProvidedFileIsInvalid` | `,` | Provided File Is Invalid. |
| `ProvidedFileTypeIsInvalid` | `,` | Provided File Type Is Invalid. |
| `ProvidedFileDurationExceedsMaximumLength` | `,` | Provided File Duration Exceeds Maximum Length. |
| `OwnerCannotBePendingMember` | `,` | Owner Cannot Be Pending Member. |
| `OwnershipCannotBeMovedToABotUser` | `,` | Ownership Cannot Be Moved To A Bot User. |
| `FailedToResizeAssetBelowTheMaximumSize` | `,` | Failed To Resize Asset Below The Maximum Size. |
| `CannotMixSubscriptionAndNonSubscriptionRolesForAnEmoji` | `,` | Cannot Mix Subscription And Non Subscription Roles For An Emoji. |
| `CannotConvertBetweenPremiumEmojiAndNormalEmoji` | `,` | Cannot Convert Between Premium Emoji And Normal Emoji. |
| `UploadedFileNotFound` | `,` | Uploaded File Not Found. |
| `SpecifiedEmojiIsInvalid` | `,` | Specified Emoji Is Invalid. |
| `VoiceMessagesDoNotSupportAdditionalContent` | `,` | Voice Messages Do Not Support Additional Content. |
| `VoiceMessagesMustHaveASingleAudioAttachment` | `,` | Voice Messages Must Have A Single Audio Attachment. |
| `VoiceMessagesMustHaveSupportingMetadata` | `,` | Voice Messages Must Have Supporting Metadata. |
| `VoiceMessagesCannotBeEdited` | `,` | Voice Messages Cannot Be Edited. |
| `CannotDeleteGuildSubscriptionIntegration` | `,` | Cannot Delete Guild Subscription Integration. |
| `CannotSendVoiceEffectWhenUserIsServerMutedDeafenedOrSuppressed` | `,` | Cannot Send Voice Effect When User Is Server Muted Deafened Or Suppressed. |
| `YouCannotSendVoiceMessagesInThisChannel` | `,` | You Cannot Send Voice Messages In This Channel. |
| `TheUserAccountMustFirstBeVerified` | `,` | The User Account Must First Be Verified. |
| `ProvidedFileDoesNotHaveAValidDuration` | `,` | Provided File Does Not Have A Valid Duration. |
| `CannotSendMessagesToThisUserDueToHavingNoMutualGuilds` | `,` | Cannot Send Messages To This User Due To Having No Mutual Guilds. |
| `YouDoNotHavePermissionToSendThisSticker` | `,` | You Do Not Have Permission To Send This Sticker. |
| `TwoFactorAuthenticationIsRequired` | `,` | Two Factor Authentication Is Required. |
| `NoUsersWithDiscordTagExist` | `,` | No Users With Discord Tag Exist. |
| `ReactionWasBlocked` | `,` | Reaction Was Blocked. |
| `UserCannotUseBurstReactions` | `,` | User Cannot Use Burst Reactions. |
| `IndexNotYetAvailable` | `,` | Index Not Yet Available. |
| `ApplicationNotYetAvailable` | `,` | Application Not Yet Available. |
| `ApiResourceOverloaded` | `,` | API Resource Overloaded. |
| `TheStageIsAlreadyOpen` | `,` | The Stage Is Already Open. |
| `CannotReplyWithoutPermissionToReadMessageHistory` | `,` | Cannot Reply Without Permission To Read Message History. |
| `ThreadAlreadyCreatedForMessage` | `,` | Thread Already Created For Message. |
| `ThreadLocked` | `,` | Thread Locked. |
| `MaximumActiveThreads` | `,` | Maximum Active Threads. |
| `MaximumActiveAnnouncementThreads` | `,` | Maximum Active Announcement Threads. |
| `CannotReferenceAMessageWithoutPermissionToReadMessageHistory` | `,` | Cannot Reference A Message Without Permission To Read Message History. |
| `NsfwChannelMessageReferenceNotAllowed` | `,` | NSFW Channel Message Reference Not Allowed. |
| `CannotForwardMessageWithUnreadableContent` | `,` | Cannot Forward Message With Unreadable Content. |
| `InvalidJsonForUploadedLottieFile` | `,` | Invalid Json For Uploaded Lottie File. |
| `UploadedLottiesCannotContainRasterizedImages` | `,` | Uploaded Lotties Cannot Contain Rasterized Images. |
| `StickerMaximumFramerateExceeded` | `,` | Sticker Maximum Framerate Exceeded. |
| `StickerFrameCountExceedsMaximumOf1000Frames` | `,` | Sticker Frame Count Exceeds Maximum Of 1000 Frames. |
| `LottieAnimationMaximumDimensionsExceeded` | `,` | Lottie Animation Maximum Dimensions Exceeded. |
| `StickerFramerateIsTooSmallOrTooLarge` | `,` | Sticker Framerate Is Too Small Or Too Large. |
| `StickerAnimationDurationExceedsMaximumOf5Seconds` | `,` | Sticker Animation Duration Exceeds Maximum Of 5 Seconds. |
| `CannotUpdateAFinishedEvent` | `,` | Cannot Update A Finished Event. |
| `FailedToCreateStageNeededForStageEvent` | `,` | Failed To Create Stage Needed For Stage Event. |
| `MessageWasBlockedByAutomaticModeration` | `,` | Message Was Blocked By Automatic Moderation. |
| `TitleWasBlockedByAutomaticModeration` | `,` | Title Was Blocked By Automatic Moderation. |
| `WebhooksPostedToForumChannelsMustHaveAThreadNameOrThreadId` | `,` | Webhooks Posted To Forum Channels Must Have A Thread Name Or Thread ID. |
| `WebhooksPostedToForumChannelsCannotHaveBothAThreadNameAndThreadId` | `,` | Webhooks Posted To Forum Channels Cannot Have Both A Thread Name And Thread ID. |
| `WebhooksCanOnlyCreateThreadsInForumChannels` | `,` | Webhooks Can Only Create Threads In Forum Channels. |
| `WebhookServicesCannotBeUsedInForumChannels` | `,` | Webhook Services Cannot Be Used In Forum Channels. |
| `MessageBlockedByHarmfulLinksFilter` | `,` | Message Blocked By Harmful Links Filter. |
| `AccessToJoiningNewServersHasBeenLimitedForTheUser` | `,` | Access To Joining New Servers Has Been Limited For The User. |
| `CannotEnableOnboardingRequirementsAreNotMet` | `,` | Cannot Enable Onboarding Requirements Are Not Met. |
| `CannotUpdateOnboardingWhileBelowRequirements` | `,` | Cannot Update Onboarding While Below Requirements. |
| `AccessToFileUploadsHasBeenLimitedForThisGuild` | `,` | Access To File Uploads Has Been Limited For This Guild. |
| `FailedToBanUsers` | `,` | Failed To Ban Users. |
| `PollVotingBlocked` | `,` | Poll Voting Blocked. |
| `PollExpired` | `,` | Poll Expired. |
| `InvalidChannelTypeForPollCreation` | `,` | Invalid Channel Type For Poll Creation. |
| `CannotEditAPollMessage` | `,` | Cannot Edit A Poll Message. |
| `CannotUseAnEmojiIncludedWithThePoll` | `,` | Cannot Use An Emoji Included With The Poll. |
| `CannotExpireANonPollMessage` | `,` | Cannot Expire A Non Poll Message. |
| `ProvisionalAccountsPermissionNotGranted` | `,` | Provisional Accounts Permission Not Granted. |
| `IdTokenJwtExpired` | `,` | ID Token Jwt Expired. |
| `IdTokenJwtIssuerMismatch` | `,` | ID Token Jwt Issuer Mismatch. |
| `IdTokenJwtAudienceMismatch` | `,` | ID Token Jwt Audience Mismatch. |
| `IdTokenJwtIssuedTooLongAgo` | `,` | ID Token Jwt Issued Too Long Ago. |
| `FailedToGenerateUniqueUsername` | `,` | Failed To Generate Unique Username. |
| `InvalidClientSecret` | `,` | Invalid Client Secret. |

## `model.rest.guild`

### `GetGuild`

Query parameters for the matching REST call.

```rust
pub struct GetGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `with_counts` | `Option<bool>` | The with counts. |

### `ModifyGuild`

Request body for the matching REST call.

```rust
pub struct ModifyGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `verification_level` | `Patch<VerificationLevel>` | The guild's verification requirement for members. |
| `default_message_notifications` | `Patch<DefaultMessageNotificationLevel>` | The default notification level for new members. |
| `explicit_content_filter` | `Patch<ExplicitContentFilterLevel>` | The guild's explicit media filter level. |
| `afk_channel_id` | `Patch<ChannelId>` | The ID of the AFK voice channel. |
| `afk_timeout` | `Option<u32>` | How long before a member is moved to the AFK channel, in seconds. |
| `icon` | `Patch<ImageData>` | The icon hash. |
| `splash` | `Patch<ImageData>` | The invite splash hash. |
| `discovery_splash` | `Patch<ImageData>` | The discovery splash hash. |
| `banner` | `Patch<ImageData>` | The banner hash. |
| `system_channel_id` | `Patch<ChannelId>` | The ID of the channel that receives system messages. |
| `system_channel_flags` | `Option<SystemChannelFlags>` | Which system messages the system channel receives. |
| `rules_channel_id` | `Patch<ChannelId>` | The ID of the rules channel. |
| `public_updates_channel_id` | `Patch<ChannelId>` | The ID of the channel that receives public updates. |
| `preferred_locale` | `Patch<Locale>` | The guild's preferred language. |
| `features` | `Option<Vec<Feature>>` | The feature flags enabled for the guild. |
| `description` | `Patch<CompactString>` | A human-readable description. |
| `premium_progress_bar_enabled` | `Option<bool>` | Whether the boost progress bar is shown. |
| `safety_alerts_channel_id` | `Patch<ChannelId>` | The ID of the channel that receives safety alerts. |
| `region` | `Option<CompactString>` | The voice region. |
| `owner_id` | `Option<UserId>` | The ID of the owner. |

### `CreateGuildChannel`

Request body for the matching REST call.

```rust
pub struct CreateGuildChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `kind` | `Option<ChannelType>` | The object's type discriminator. |
| `topic` | `Option<CompactString>` | The topic. |
| `bitrate` | `Option<u32>` | The voice channel's bitrate in bits per second. |
| `user_limit` | `Option<u32>` | The maximum number of users allowed in the voice channel. |
| `rate_limit_per_user` | `Option<u16>` | Slowmode delay in seconds between messages per user. |
| `position` | `Option<i32>` | The position among siblings, lowest first. |
| `permission_overwrites` | `Vec<Overwrite>` | Channel-specific permission overwrites. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `rtc_region` | `Option<CompactString>` | The preferred voice region. |
| `video_quality_mode` | `Option<VideoQualityMode>` | The channel's video quality mode. |
| `default_auto_archive_duration` | `Option<AutoArchiveDuration>` | The default auto-archive duration for new threads. |
| `default_reaction_emoji` | `Option<DefaultReaction>` | The default reaction emoji for new forum posts. |
| `available_tags` | `Vec<ForumTagData>` | The tags that can be applied to new forum posts. |
| `default_sort_order` | `Option<SortOrderType>` | The default forum sort order. |
| `default_forum_layout` | `Option<ForumLayoutType>` | The default forum layout. |
| `default_thread_rate_limit_per_user` | `Option<u16>` | The default slowmode for new threads. |
| `flags` | `Option<ChannelFlags>` | Bitfield of flags attached to the object. |

#### Methods

##### `new`

Creates a new `CreateGuildChannel`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `ModifyGuildChannelPosition`

Request body for the matching REST call.

```rust
pub struct ModifyGuildChannelPosition
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `position` | `Patch<i32>` | The position among siblings, lowest first. |
| `lock_permissions` | `Patch<bool>` | The lock permissions. |
| `parent_id` | `Patch<ChannelId>` | The ID of the parent channel or category. |

#### Methods

##### `new`

Creates a new `ModifyGuildChannelPosition`.

```rust
pub const fn new(id: ChannelId, position: i32) -> Self;
```

### `ActiveThreads`

Request or response shape for the matching REST call.

```rust
pub struct ActiveThreads
```

| Field | Type | Description |
| --- | --- | --- |
| `threads` | `Vec<Channel>` | The threads in this channel. |
| `members` | `Vec<ThreadMember>` | The members in this collection. |

### `ListGuildMembers`

Query parameters for the matching REST call.

```rust
pub struct ListGuildMembers
```

| Field | Type | Description |
| --- | --- | --- |
| `limit` | `Option<u16>` | The maximum number of items to return. |
| `after` | `Option<UserId>` | Return items after this ID. |

### `SearchGuildMembers`

Query parameters for the matching REST call.

```rust
pub struct SearchGuildMembers
```

| Field | Type | Description |
| --- | --- | --- |
| `query` | `CompactString` | The query. |
| `limit` | `Option<u16>` | The maximum number of items to return. |

#### Methods

##### `new`

Creates a new `SearchGuildMembers`.

```rust
pub fn new(query: impl Into<CompactString>) -> Self;
```

### `AddGuildMember`

Request body for the matching REST call.

```rust
pub struct AddGuildMember
```

| Field | Type | Description |
| --- | --- | --- |
| `access_token` | `String` | The OAuth2 access token. |
| `nick` | `Option<CompactString>` | The member's guild nickname, when set. |
| `roles` | `Vec<RoleId>` | The roles attached or referenced. |
| `mute` | `Option<bool>` | Whether the member is server-muted. |
| `deaf` | `Option<bool>` | Whether the member is server-deafened. |

#### Methods

##### `new`

Creates a new `AddGuildMember`.

```rust
pub fn new(access_token: impl Into<String>) -> Self;
```

### `ModifyGuildMember`

Request body for the matching REST call.

```rust
pub struct ModifyGuildMember
```

| Field | Type | Description |
| --- | --- | --- |
| `nick` | `Patch<CompactString>` | The member's guild nickname, when set. |
| `roles` | `Patch<Vec<RoleId>>` | The roles attached or referenced. |
| `mute` | `Patch<bool>` | Whether the member is server-muted. |
| `deaf` | `Patch<bool>` | Whether the member is server-deafened. |
| `channel_id` | `Patch<ChannelId>` | The ID of the channel this belongs to. |
| `communication_disabled_until` | `Patch<Timestamp>` | When the member's timeout expires, when one is active. |
| `flags` | `Patch<MemberFlags>` | Bitfield of flags attached to the object. |

### `ModifyCurrentMember`

Request body for the matching REST call.

```rust
pub struct ModifyCurrentMember
```

| Field | Type | Description |
| --- | --- | --- |
| `nick` | `Patch<CompactString>` | The member's guild nickname, when set. |
| `banner` | `Patch<ImageData>` | The banner hash. |
| `avatar` | `Patch<ImageData>` | The avatar hash. |
| `bio` | `Patch<CompactString>` | The bio. |

### `ModifyCurrentMemberNickname`

Request body for the matching REST call.

```rust
pub struct ModifyCurrentMemberNickname
```

| Field | Type | Description |
| --- | --- | --- |
| `nick` | `Option<CompactString>` | The member's guild nickname, when set. |

### `GetGuildBans`

Query parameters for the matching REST call.

```rust
pub struct GetGuildBans
```

| Field | Type | Description |
| --- | --- | --- |
| `limit` | `Option<u16>` | The maximum number of items to return. |
| `before` | `Option<UserId>` | Return items before this ID. |
| `after` | `Option<UserId>` | Return items after this ID. |

### `CreateGuildBan`

Request body for the matching REST call.

```rust
pub struct CreateGuildBan
```

| Field | Type | Description |
| --- | --- | --- |
| `delete_message_seconds` | `Option<u32>` | The delete message seconds. |

### `BulkGuildBan`

Request body for the matching REST call.

```rust
pub struct BulkGuildBan
```

| Field | Type | Description |
| --- | --- | --- |
| `user_ids` | `Vec<UserId>` | The IDs of the user. |
| `delete_message_seconds` | `Option<u32>` | The delete message seconds. |

#### Methods

##### `new`

Creates a new `BulkGuildBan`.

```rust
pub const fn new(user_ids: Vec<UserId>) -> Self;
```

### `BulkBanResponse`

Request body for the matching REST call.

```rust
pub struct BulkBanResponse
```

| Field | Type | Description |
| --- | --- | --- |
| `banned_users` | `Vec<UserId>` | The banned users. |
| `failed_users` | `Vec<UserId>` | The failed users. |

### `CreateGuildRole`

Request body for the matching REST call.

```rust
pub struct CreateGuildRole
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `permissions` | `Option<Permissions>` | The permissions bitfield. |
| `color` | `Option<u32>` | The primary role color as a 24-bit integer. |
| `colors` | `Option<RoleColors>` | The role's primary, secondary and tertiary colors. |
| `hoist` | `bool` | Whether the role is shown separately in the member list. |
| `icon` | `Option<ImageData>` | The icon hash. |
| `unicode_emoji` | `Option<CompactString>` | The unicode emoji shown as the role icon. |
| `mentionable` | `bool` | Whether members can mention the role. |

### `ModifyGuildRolePosition`

Request body for the matching REST call.

```rust
pub struct ModifyGuildRolePosition
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `RoleId` | The object's snowflake ID. |
| `position` | `Patch<i32>` | The position among siblings, lowest first. |

#### Methods

##### `new`

Creates a new `ModifyGuildRolePosition`.

```rust
pub const fn new(id: RoleId, position: i32) -> Self;
```

### `ModifyGuildRole`

Request body for the matching REST call.

```rust
pub struct ModifyGuildRole
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `permissions` | `Option<Permissions>` | The permissions bitfield. |
| `color` | `Option<u32>` | The primary role color as a 24-bit integer. |
| `colors` | `Patch<RoleColors>` | The role's primary, secondary and tertiary colors. |
| `hoist` | `Option<bool>` | Whether the role is shown separately in the member list. |
| `icon` | `Patch<ImageData>` | The icon hash. |
| `unicode_emoji` | `Patch<CompactString>` | The unicode emoji shown as the role icon. |
| `mentionable` | `Option<bool>` | Whether members can mention the role. |

### `GetGuildPruneCount`

Query parameters for the matching REST call.

```rust
pub struct GetGuildPruneCount
```

| Field | Type | Description |
| --- | --- | --- |
| `days` | `Option<u8>` | The days. |
| `include_roles` | `Vec<RoleId>` | The include roles. |

### `BeginGuildPrune`

Request body for the matching REST call.

```rust
pub struct BeginGuildPrune
```

| Field | Type | Description |
| --- | --- | --- |
| `days` | `Option<u8>` | The days. |
| `compute_prune_count` | `Option<bool>` | How many compute prune there are. |
| `include_roles` | `Vec<RoleId>` | The include roles. |

### `ModifyGuildWidget`

Request body for the matching REST call.

```rust
pub struct ModifyGuildWidget
```

| Field | Type | Description |
| --- | --- | --- |
| `enabled` | `Option<bool>` | Whether the feature is enabled. |
| `channel_id` | `Patch<ChannelId>` | The ID of the channel this belongs to. |

### `VanityUrl`

Request or response shape for the matching REST call.

```rust
pub struct VanityUrl
```

| Field | Type | Description |
| --- | --- | --- |
| `code` | `Option<CompactString>` | The code value. |
| `uses` | `u32` | The uses. |

### `ModifyGuildWelcomeScreen`

Request body for the matching REST call.

```rust
pub struct ModifyGuildWelcomeScreen
```

| Field | Type | Description |
| --- | --- | --- |
| `enabled` | `Patch<bool>` | Whether the feature is enabled. |
| `welcome_channels` | `Patch<Vec<WelcomeScreenChannel>>` | The welcome channels. |
| `description` | `Patch<CompactString>` | A human-readable description. |

### `ModifyGuildOnboarding`

Request body for the matching REST call.

```rust
pub struct ModifyGuildOnboarding
```

| Field | Type | Description |
| --- | --- | --- |
| `prompts` | `Option<Vec<OnboardingPromptData>>` | The prompts. |
| `default_channel_ids` | `Option<Vec<ChannelId>>` | The IDs of the default channel. |
| `enabled` | `Option<bool>` | Whether the feature is enabled. |
| `mode` | `Option<OnboardingMode>` | The mode. |

### `OnboardingPromptData`

Response shape for the matching REST call.

```rust
pub struct OnboardingPromptData
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<GenericId>` | The object's snowflake ID. |
| `kind` | `PromptType` | The object's type discriminator. |
| `options` | `Vec<PromptOptionData>` | The selectable options or command options. |
| `title` | `CompactString` | The title. |
| `single_select` | `bool` | The single select. |
| `required` | `bool` | Whether the option must be supplied. |
| `in_onboarding` | `bool` | The in onboarding. |

#### Methods

##### `new`

Creates a new `OnboardingPromptData`.

```rust
pub fn new(title: impl Into<CompactString>, options: Vec<PromptOptionData>) -> Self;
```

### `PromptOptionData`

Response shape for the matching REST call.

```rust
pub struct PromptOptionData
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<GenericId>` | The object's snowflake ID. |
| `channel_ids` | `Vec<ChannelId>` | The IDs of the channels. |
| `role_ids` | `Vec<RoleId>` | The IDs of the roles. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |
| `emoji_animated` | `Option<bool>` | The emoji animated. |
| `title` | `CompactString` | The title. |
| `description` | `Option<CompactString>` | A human-readable description. |

#### Methods

##### `new`

Creates a new `PromptOptionData`.

```rust
pub fn new(title: impl Into<CompactString>) -> Self;
```

### `CreateGuild`

Request body for the matching REST call.

```rust
pub struct CreateGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageData>` | The icon hash. |
| `verification_level` | `Option<VerificationLevel>` | The guild's verification requirement for members. |
| `default_message_notifications` | `Option<DefaultMessageNotificationLevel>` | The default notification level for new members. |
| `explicit_content_filter` | `Option<ExplicitContentFilterLevel>` | The guild's explicit media filter level. |
| `afk_channel_id` | `Option<ChannelId>` | The ID of the AFK voice channel. |
| `afk_timeout` | `Option<u32>` | How long before a member is moved to the AFK channel, in seconds. |
| `system_channel_id` | `Option<ChannelId>` | The ID of the channel that receives system messages. |
| `system_channel_flags` | `Option<SystemChannelFlags>` | Which system messages the system channel receives. |
| `preferred_locale` | `Option<Locale>` | The guild's preferred language. |
| `region` | `Option<CompactString>` | The voice region. |
| `roles` | `Vec<CreateGuildRole>` | The roles attached or referenced. |

#### Methods

##### `new`

Creates a new `CreateGuild`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `ModifyGuildMfaLevel`

Request body for the matching REST call.

```rust
pub struct ModifyGuildMfaLevel
```

| Field | Type | Description |
| --- | --- | --- |
| `level` | `MfaLevel` | The level. |

#### Methods

##### `new`

Creates a new `ModifyGuildMfaLevel`.

```rust
pub const fn new(level: MfaLevel) -> Self;
```

### `GuildMembershipScreening`

Request or response shape for the matching REST call.

```rust
pub struct GuildMembershipScreening
```

| Field | Type | Description |
| --- | --- | --- |
| `version` | `CompactString` | The version. |
| `form_fields` | `Vec<MembershipScreeningField>` | The form fields. |
| `description` | `Option<CompactString>` | A human-readable description. |

### `MembershipScreeningField`

Request or response shape for the matching REST call.

```rust
pub struct MembershipScreeningField
```

| Field | Type | Description |
| --- | --- | --- |
| `field_type` | `MembershipScreeningFieldType` | The field type. |
| `label` | `CompactString` | The human-readable label. |
| `values` | `Option<Vec<CompactString>>` | The values. |
| `required` | `bool` | Whether the option must be supplied. |

### `MembershipScreeningFieldType`

Possible membership screening field type values.

```rust
pub enum MembershipScreeningFieldType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Terms` | `,` | Terms. |

### `ModifyGuildMembershipScreening`

Request body for the matching REST call.

```rust
pub struct ModifyGuildMembershipScreening
```

| Field | Type | Description |
| --- | --- | --- |
| `enabled` | `Option<bool>` | Whether the feature is enabled. |
| `form_fields` | `Option<CompactString>` | The form fields. |
| `description` | `Patch<CompactString>` | A human-readable description. |

### `ModifyGuildIncidentActions`

Request body for the matching REST call.

```rust
pub struct ModifyGuildIncidentActions
```

| Field | Type | Description |
| --- | --- | --- |
| `invites_disabled_until` | `Patch<Timestamp>` | The invites disabled until. |
| `dms_disabled_until` | `Patch<Timestamp>` | The dms disabled until. |

## `model.rest.guild_scheduled_event`

### `ListScheduledEventsForGuild`

Query parameters for the matching REST call.

```rust
pub struct ListScheduledEventsForGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `with_user_count` | `Option<bool>` | How many with user there are. |

### `GetGuildScheduledEvent`

Query parameters for the matching REST call.

```rust
pub struct GetGuildScheduledEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `with_user_count` | `Option<bool>` | How many with user there are. |

### `GetGuildScheduledEventUsers`

Query parameters for the matching REST call.

```rust
pub struct GetGuildScheduledEventUsers
```

| Field | Type | Description |
| --- | --- | --- |
| `limit` | `Option<u8>` | The maximum number of items to return. |
| `with_member` | `Option<bool>` | The with member. |
| `before` | `Option<UserId>` | Return items before this ID. |
| `after` | `Option<UserId>` | Return items after this ID. |

### `RecurrenceRuleData`

Response shape for the matching REST call.

```rust
pub struct RecurrenceRuleData
```

| Field | Type | Description |
| --- | --- | --- |
| `start` | `Timestamp` | The start. |
| `frequency` | `RecurrenceFrequency` | The frequency. |
| `interval` | `u16` | The interval between runs. |
| `by_weekday` | `Vec<RecurrenceWeekday>` | The by weekday. |
| `by_n_weekday` | `Vec<RecurrenceNWeekday>` | The by n weekday. |
| `by_month` | `Vec<RecurrenceMonth>` | The by month. |
| `by_month_day` | `Vec<u8>` | The by month day. |

#### Methods

##### `daily`

Repeats every day.

```rust
pub const fn daily(start: Timestamp) -> Self;
```

##### `daily_on`

Repeats every day of the given month.

```rust
pub fn daily_on(start: Timestamp, days: impl IntoIterator<Item = RecurrenceWeekday>) -> Self;
```

##### `weekly`

Repeats every week on the given days.

```rust
pub fn weekly(start: Timestamp, day: RecurrenceWeekday) -> Self;
```

##### `every_other_week`

Repeats every other week.

```rust
pub fn every_other_week(start: Timestamp, day: RecurrenceWeekday) -> Self;
```

##### `monthly`

Repeats every month.

```rust
pub fn monthly(start: Timestamp, n: u8, day: RecurrenceWeekday) -> Self;
```

##### `yearly`

Repeats every year.

```rust
pub fn yearly(start: Timestamp, month: RecurrenceMonth, day: u8) -> Self;
```

### `CreateGuildScheduledEvent`

Request body for the matching REST call.

```rust
pub struct CreateGuildScheduledEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `entity_metadata` | `Option<EntityMetadata>` | Metadata for the scheduled event's location or entity. |
| `name` | `CompactString` | The object's name. |
| `privacy_level` | `PrivacyLevel` | Who can see the event. |
| `scheduled_start_time` | `Timestamp` | When the event starts. |
| `scheduled_end_time` | `Option<Timestamp>` | When the event ends. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `entity_type` | `EntityType` | The kind of event entity. |
| `image` | `Option<ImageData>` | The image hash or URL. |
| `recurrence_rule` | `Option<RecurrenceRuleData>` | The recurrence rule. |

#### Methods

##### `stage`

Holds the event in a stage channel.

```rust
pub fn stage(channel_id: ChannelId, name: impl Into<CompactString>, start: Timestamp) -> Self;
```

##### `voice`

Holds the event in a voice channel.

```rust
pub fn voice(channel_id: ChannelId, name: impl Into<CompactString>, start: Timestamp) -> Self;
```

##### `external`

Holds the event at an external location.

```rust
pub fn external( location: impl Into<CompactString>, name: impl Into<CompactString>, start: Timestamp, end: Timestamp, ) -> Self;
```

### `ModifyGuildScheduledEvent`

Request body for the matching REST call.

```rust
pub struct ModifyGuildScheduledEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `Patch<ChannelId>` | The ID of the channel this belongs to. |
| `entity_metadata` | `Patch<EntityMetadata>` | Metadata for the scheduled event's location or entity. |
| `name` | `Option<CompactString>` | The object's name. |
| `privacy_level` | `Option<PrivacyLevel>` | Who can see the event. |
| `scheduled_start_time` | `Option<Timestamp>` | When the event starts. |
| `scheduled_end_time` | `Option<Timestamp>` | When the event ends. |
| `description` | `Patch<CompactString>` | A human-readable description. |
| `entity_type` | `Option<EntityType>` | The kind of event entity. |
| `status` | `Option<Status>` | The current status. |
| `image` | `Patch<ImageData>` | The image hash or URL. |
| `recurrence_rule` | `Patch<RecurrenceRuleData>` | The recurrence rule. |

## `model.rest.guild_template`

### `CreateGuildTemplate`

Request body for the matching REST call.

```rust
pub struct CreateGuildTemplate
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `description` | `Option<CompactString>` | A human-readable description. |

#### Methods

##### `new`

Creates a new `CreateGuildTemplate`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `CreateGuildFromTemplate`

Request body for the matching REST call.

```rust
pub struct CreateGuildFromTemplate
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageData>` | The icon hash. |

#### Methods

##### `new`

Creates a new `CreateGuildFromTemplate`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

### `ModifyGuildTemplate`

Request body for the matching REST call.

```rust
pub struct ModifyGuildTemplate
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `Option<CompactString>` | The object's name. |
| `description` | `Patch<CompactString>` | A human-readable description. |
