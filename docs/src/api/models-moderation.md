# Models: moderation

Audit logs and auto-moderation rules.

## `model.audit_log`

### `AuditLog`

A page of audit-log entries plus the users and integrations they reference.

```rust
pub struct AuditLog
```

| Field | Type | Description |
| --- | --- | --- |
| `application_commands` | `Vec<ApplicationCommand>` | The application commands. |
| `audit_log_entries` | `Vec<AuditLogEntry>` | The audit log entries. |
| `auto_moderation_rules` | `Vec<Rule>` | The auto moderation rules. |
| `guild_scheduled_events` | `Vec<GuildScheduledEvent>` | The guild scheduled events. |
| `integrations` | `Vec<PartialIntegration>` | The integrations. |
| `threads` | `Vec<Channel>` | The threads in this channel. |
| `users` | `Vec<User>` | The users. |
| `webhooks` | `Vec<Webhook>` | The webhooks. |

### `AuditLogEntry`

One audit-log entry: action, actor, target and changes.

```rust
pub struct AuditLogEntry
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `AuditLogEntryId` | The object's snowflake ID. |
| `action_type` | `AuditLogEvent` | The action type. |
| `target_id` | `Option<GenericId>` | The ID of the target. |
| `user_id` | `Option<UserId>` | The ID of the user. |
| `changes` | `Vec<AuditLogChange>` | The changes. |
| `options` | `Option<Box<AuditLogEntryOptions>>` | The selectable options or command options. |
| `reason` | `Option<CompactString>` | The reason. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |

### `AuditLogEvent`

The kind of action recorded.

```rust
pub enum AuditLogEvent: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `GuildUpdate` | `,` | The guild settings changed. |
| `ChannelCreate` | `,` | A channel was created. |
| `ChannelUpdate` | `,` | A channel was edited. |
| `ChannelDelete` | `,` | A channel was deleted. |
| `ChannelOverwriteCreate` | `,` | Channel Overwrite Create. |
| `ChannelOverwriteUpdate` | `,` | Channel Overwrite Update. |
| `ChannelOverwriteDelete` | `,` | Channel Overwrite Delete. |
| `ChannelPositionUpdate` | `,` | Channel Position Update. |
| `MemberKick` | `,` | A member was kicked. |
| `MemberPrune` | `,` | Inactive members were pruned. |
| `MemberBanAdd` | `,` | A member was banned. |
| `MemberBanRemove` | `,` | A ban was lifted. |
| `MemberUpdate` | `,` | A member was edited. |
| `MemberRoleUpdate` | `,` | A member's roles changed. |
| `MemberMove` | `,` | Member Move. |
| `MemberDisconnect` | `,` | Member Disconnect. |
| `BotAdd` | `,` | Bot Add. |
| `RoleCreate` | `,` | A role was created. |
| `RoleUpdate` | `,` | A role was edited. |
| `RoleDelete` | `,` | A role was deleted. |
| `RolePositionUpdate` | `,` | Role Position Update. |
| `InviteCreate` | `,` | An invite was created. |
| `InviteUpdate` | `,` | An invite was edited. |
| `InviteDelete` | `,` | An invite was deleted. |
| `WebhookCreate` | `,` | A webhook was created. |
| `WebhookUpdate` | `,` | A webhook was edited. |
| `WebhookDelete` | `,` | A webhook was deleted. |
| `EmojiCreate` | `,` | Emoji Create. |
| `EmojiUpdate` | `,` | Emoji Update. |
| `EmojiDelete` | `,` | Emoji Delete. |
| `MessageDelete` | `,` | A message was deleted. |
| `MessageBulkDelete` | `,` | Messages were bulk deleted. |
| `MessagePin` | `,` | Message Pin. |
| `MessageUnpin` | `,` | Message Unpin. |
| `IntegrationCreate` | `,` | Integration Create. |
| `IntegrationUpdate` | `,` | Integration Update. |
| `IntegrationDelete` | `,` | Integration Delete. |
| `StageInstanceCreate` | `,` | Stage Instance Create. |
| `StageInstanceUpdate` | `,` | Stage Instance Update. |
| `StageInstanceDelete` | `,` | Stage Instance Delete. |
| `StickerCreate` | `,` | Sticker Create. |
| `StickerUpdate` | `,` | Sticker Update. |
| `StickerDelete` | `,` | Sticker Delete. |
| `GuildScheduledEventCreate` | `,` | Guild Scheduled Event Create. |
| `GuildScheduledEventUpdate` | `,` | Guild Scheduled Event Update. |
| `GuildScheduledEventDelete` | `,` | Guild Scheduled Event Delete. |
| `ThreadCreate` | `,` | Thread Create. |
| `ThreadUpdate` | `,` | Thread Update. |
| `ThreadDelete` | `,` | Thread Delete. |
| `ApplicationCommandPermissionUpdate` | `,` | Application Command Permission Update. |
| `SoundboardSoundCreate` | `,` | Soundboard Sound Create. |
| `SoundboardSoundUpdate` | `,` | Soundboard Sound Update. |
| `SoundboardSoundDelete` | `,` | Soundboard Sound Delete. |
| `AutoModerationRuleCreate` | `,` | Auto Moderation Rule Create. |
| `AutoModerationRuleUpdate` | `,` | Auto Moderation Rule Update. |
| `AutoModerationRuleDelete` | `,` | Auto Moderation Rule Delete. |
| `AutoModerationBlockMessage` | `,` | Auto Moderation Block Message. |
| `AutoModerationFlagToChannel` | `,` | Auto Moderation Flag To Channel. |
| `AutoModerationUserCommunicationDisabled` | `,` | Auto Moderation User Communication Disabled. |
| `AutoModerationQuarantineUser` | `,` | Auto Moderation Quarantine User. |
| `CreatorMonetizationRequestCreated` | `,` | Creator Monetization Request Created. |
| `CreatorMonetizationTermsAccepted` | `,` | Creator Monetization Terms Accepted. |
| `OnboardingPromptCreate` | `,` | Onboarding Prompt Create. |
| `OnboardingPromptUpdate` | `,` | Onboarding Prompt Update. |
| `OnboardingPromptDelete` | `,` | Onboarding Prompt Delete. |
| `OnboardingCreate` | `,` | Onboarding Create. |
| `OnboardingUpdate` | `,` | Onboarding Update. |
| `HomeSettingsCreate` | `,` | Home Settings Create. |
| `HomeSettingsUpdate` | `,` | Home Settings Update. |
| `VoiceChannelStatusCreate` | `,` | Voice Channel Status Create. |
| `VoiceChannelStatusDelete` | `,` | Voice Channel Status Delete. |

### `AuditLogEntryOptions`

The type-specific options attached to an audit-log entry.

```rust
pub struct AuditLogEntryOptions
```

| Field | Type | Description |
| --- | --- | --- |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `auto_moderation_rule_name` | `Option<CompactString>` | The auto moderation rule name. |
| `auto_moderation_rule_trigger_type` | `Option<TriggerType>` | The auto moderation rule trigger type. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `count` | `Option<u32>` | The count. |
| `delete_member_days` | `Option<u32>` | The delete member days. |
| `id` | `Option<GenericId>` | The object's snowflake ID. |
| `members_removed` | `Option<u32>` | The members removed. |
| `message_id` | `Option<MessageId>` | The ID of the message. |
| `role_name` | `Option<CompactString>` | The role name. |
| `kind` | `Option<OverwriteType>` | The object's type discriminator. |
| `integration_type` | `Option<IntegrationType>` | The integration type. |
| `status` | `Option<CompactString>` | The current status. |




### `AuditLogChange`

One before/after field change on an audit-log target.

```rust
pub struct AuditLogChange
```

| Field | Type | Description |
| --- | --- | --- |
| `key` | `CompactString` | The key. |
| `old_value` | `Option<JsonValue>` | The old value. |
| `new_value` | `Option<JsonValue>` | The new value. |

#### Methods

##### `old_value_as`

Deserializes the previous value into the requested type.

```rust
pub fn old_value_as<'a, T: Deserialize<'a>>(&'a self) -> Result<Option<T>, JsonValueError>;
```

##### `new_value_as`

Deserializes the new value into the requested type.

```rust
pub fn new_value_as<'a, T: Deserialize<'a>>(&'a self) -> Result<Option<T>, JsonValueError>;
```

## `model.auto_moderation`

### `Rule`

An auto-moderation rule: trigger, actions, exemptions and status.

```rust
pub struct Rule
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `AutoModerationRuleId` | The object's snowflake ID. |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `creator_id` | `UserId` | The ID of the creator. |
| `event_type` | `EventType` | The type of event. |
| `trigger_type` | `TriggerType` | The trigger type. |
| `trigger_metadata` | `TriggerMetadata` | The trigger metadata. |
| `actions` | `Vec<Action>` | The actions configured on the object. |
| `enabled` | `bool` | Whether the feature is enabled. |
| `exempt_roles` | `Vec<RoleId>` | The roles exempt from the rule. |
| `exempt_channels` | `Vec<ChannelId>` | The channels exempt from the rule. |

### `TriggerType`

What the rule watches: keywords, links, spam or mentions.

```rust
pub enum TriggerType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Keyword` | `,` | Custom keywords trigger the rule. |
| `Spam` | `,` | Spam triggers the rule. |
| `KeywordPreset` | `,` | A Discord preset keyword list triggers the rule. |
| `MentionSpam` | `,` | Mention spam triggers the rule. |
| `MemberProfile` | `,` | Profile content triggers the rule. |

### `TriggerMetadata`

The keyword, link and mention lists that trigger the rule.

```rust
pub struct TriggerMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `keyword_filter` | `Vec<CompactString>` | The keyword filter. |
| `regex_patterns` | `Vec<CompactString>` | The regex patterns. |
| `presets` | `Vec<KeywordPresetType>` | The presets. |
| `allow_list` | `Vec<CompactString>` | The allow list. |
| `mention_total_limit` | `Option<u8>` | The mention total limit. |
| `mention_raid_protection_enabled` | `bool` | The mention raid protection enabled. |

### `KeywordPresetType`

A Discord-provided keyword list.

```rust
pub enum KeywordPresetType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Profanity` | `,` | Profanity. |
| `SexualContent` | `,` | Sexual Content. |
| `Slurs` | `,` | Slurs. |

### `EventType`

When the rule fires: message send or member join.

```rust
pub enum EventType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `MessageSend` | `,` | The rule fires when a message is sent. |
| `MemberUpdate` | `,` | The rule fires when a member profile changes. |

### `Action`

One action the rule takes, plus its metadata.

```rust
pub struct Action
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `ActionType` | The object's type discriminator. |
| `metadata` | `Option<ActionMetadata>` | Additional structured metadata. |

#### Methods

##### `block_message`

Blocks the message and optionally tells the user why.

```rust
pub const fn block_message() -> Self;
```

##### `block_message_with`

Blocks the message and explains why with custom text.

```rust
pub fn block_message_with(custom_message: impl Into<CompactString>) -> Self;
```

##### `send_alert_message`

Sends an alert to the configured log channel.

```rust
pub const fn send_alert_message(channel_id: ChannelId) -> Self;
```

##### `timeout`

Times the member out for the given duration.

```rust
pub const fn timeout(duration_seconds: u32) -> Self;
```

##### `block_member_interaction`

Blocks the member from interacting until the timeout expires.

```rust
pub const fn block_member_interaction() -> Self;
```

### `ActionType`

What the rule does: block, alert or timeout.

```rust
pub enum ActionType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `BlockMessage` | `,` | The message is blocked. |
| `SendAlertMessage` | `,` | An alert is posted to a channel. |
| `Timeout` | `,` | The member is timed out. |
| `BlockMemberInteraction` | `,` | Block Member Interaction. |

### `ActionMetadata`

Discord's action metadata object.

```rust
pub struct ActionMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `duration_seconds` | `Option<u32>` | The duration seconds. |
| `custom_message` | `Option<CompactString>` | The custom message. |
