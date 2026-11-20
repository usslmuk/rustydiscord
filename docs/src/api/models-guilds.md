# Models: guilds

Guild, member, role, emoji and template models.

## `model.guild`

### `UnavailableGuild`

A guild the bot is a member of but cannot see, usually during an outage.

```rust
pub struct UnavailableGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `unavailable` | `bool` | Whether the guild is unavailable because of an outage. |

#### Methods

##### `new`

Creates a new `UnavailableGuild`.

```rust
pub const fn new(id: GuildId, unavailable: bool) -> Self;
```

### `Member`

A user's membership in one guild: nickname, roles, join date and timeout state.

```rust
pub struct Member
```

| Field | Type | Description |
| --- | --- | --- |
| `user` | `User` | The user this object describes. |
| `nick` | `Option<CompactString>` | The member's guild nickname, when set. |
| `avatar` | `Option<ImageHash>` | The avatar hash. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `roles` | `Vec<RoleId>` | The roles attached or referenced. |
| `joined_at` | `Option<Timestamp>` | When the member joined the guild. |
| `premium_since` | `Option<Timestamp>` | When the member started boosting the guild. |
| `deaf` | `bool` | Whether the member is server-deafened. |
| `mute` | `bool` | Whether the member is server-muted. |
| `flags` | `MemberFlags` | Bitfield of flags attached to the object. |
| `pending` | `bool` | Whether the member has not finished onboarding. |
| `communication_disabled_until` | `Option<Timestamp>` | When the member's timeout expires, when one is active. |
| `avatar_decoration_data` | `Option<Box<AvatarDecorationData>>` | The avatar decoration the member has equipped. |
| `collectibles` | `Option<Box<Collectibles>>` | Profile collectibles shown for the member. |

#### Methods

##### `display_name`

Returns the name shown in the UI, preferring the guild nickname when set.

```rust
pub fn display_name(&self) -> &str;
```

##### `is_timed_out`

Returns whether the member's communication timeout is still active.

```rust
pub fn is_timed_out(&self, now: Timestamp) -> bool;
```

### `PartialMember`

Membership data attached to interactions and mentions, without the full guild context.

```rust
pub struct PartialMember
```

| Field | Type | Description |
| --- | --- | --- |
| `nick` | `Option<CompactString>` | The member's guild nickname, when set. |
| `avatar` | `Option<ImageHash>` | The avatar hash. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `roles` | `Vec<RoleId>` | The roles attached or referenced. |
| `joined_at` | `Option<Timestamp>` | When the member joined the guild. |
| `premium_since` | `Option<Timestamp>` | When the member started boosting the guild. |
| `deaf` | `bool` | Whether the member is server-deafened. |
| `mute` | `bool` | Whether the member is server-muted. |
| `flags` | `MemberFlags` | Bitfield of flags attached to the object. |
| `pending` | `bool` | Whether the member has not finished onboarding. |
| `communication_disabled_until` | `Option<Timestamp>` | When the member's timeout expires, when one is active. |
| `avatar_decoration_data` | `Option<Box<AvatarDecorationData>>` | The avatar decoration the member has equipped. |
| `collectibles` | `Option<Box<Collectibles>>` | Profile collectibles shown for the member. |

#### Methods

##### `minimal`

Returns the minimal representation.

```rust
pub fn minimal() -> Self;
```

##### `into_member`

Combines this partial member with a user into a full member.

```rust
pub fn into_member(self, user: User) -> Member;
```

### `Role`

A role: name, colors, permissions, hoisting and icon. Everyone has the implicit @everyone role.

```rust
pub struct Role
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `RoleId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `color` | `u32` | The primary role color as a 24-bit integer. |
| `colors` | `RoleColors` | The role's primary, secondary and tertiary colors. |
| `hoist` | `bool` | Whether the role is shown separately in the member list. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `unicode_emoji` | `Option<CompactString>` | The unicode emoji shown as the role icon. |
| `position` | `i32` | The position among siblings, lowest first. |
| `permissions` | `Permissions` | The permissions bitfield. |
| `managed` | `bool` | Whether the role or account is managed by an integration. |
| `mentionable` | `bool` | Whether members can mention the role. |
| `tags` | `Option<Box<RoleTags>>` | Links to the bot, integration or boost that created the role. |
| `flags` | `RoleFlags` | Bitfield of flags attached to the object. |

### `RoleColors`

The three-color role gradient: primary, secondary and tertiary.

```rust
pub struct RoleColors
```

| Field | Type | Description |
| --- | --- | --- |
| `primary_color` | `u32` | The primary color. |
| `secondary_color` | `Option<u32>` | The secondary color. |
| `tertiary_color` | `Option<u32>` | The tertiary color. |

#### Methods

##### `solid`

Builds a solid role color from one 24-bit integer.

```rust
pub const fn solid(color: u32) -> Self;
```

### `RoleTags`

Links between a role and the integration, bot or boost that created it.

```rust
pub struct RoleTags
```

| Field | Type | Description |
| --- | --- | --- |
| `bot_id` | `Option<UserId>` | The ID of the bot. |
| `integration_id` | `Option<IntegrationId>` | The ID of the integration. |
| `premium_subscriber` | `bool` | The premium subscriber. |
| `subscription_listing_id` | `Option<SkuId>` | The ID of the subscription listing. |
| `available_for_purchase` | `bool` | The available for purchase. |
| `guild_connections` | `bool` | The guild connections. |

### `Guild`

A server: channels, roles, members, emoji, boosts and settings. Partial payloads omit most of it; the gateway fills the rest.

```rust
pub struct Guild
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
| `roles` | `Vec<Role>` | The roles attached or referenced. |
| `emojis` | `Vec<Emoji>` | The emojis in the guild. |
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
| `stickers` | `Vec<Sticker>` | The stickers attached to the message. |
| `premium_progress_bar_enabled` | `bool` | Whether the boost progress bar is shown. |
| `safety_alerts_channel_id` | `Option<ChannelId>` | The ID of the channel that receives safety alerts. |
| `incidents_data` | `Option<Box<IncidentsData>>` | Recent incident restrictions on the guild. |

### `PartialGuild`

The trimmed guild object Discord returns from invite and template endpoints.

```rust
pub struct PartialGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `splash` | `Option<ImageHash>` | The invite splash hash. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `features` | `Vec<Feature>` | The feature flags enabled for the guild. |
| `verification_level` | `Option<VerificationLevel>` | The guild's verification requirement for members. |
| `vanity_url_code` | `Option<CompactString>` | The guild's vanity invite code. |
| `welcome_screen` | `Option<Box<WelcomeScreen>>` | The guild's welcome screen configuration. |

### `DefaultMessageNotificationLevel`

Who receives notifications for new messages by default.

```rust
pub enum DefaultMessageNotificationLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `AllMessages` | `,` | All Messages. |
| `OnlyMentions` | `,` | Only Mentions. |

### `ExplicitContentFilterLevel`

How aggressively Discord filters explicit media.

```rust
pub enum ExplicitContentFilterLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Disabled` | `,` | Disabled. |
| `MembersWithoutRoles` | `,` | Members Without Roles. |
| `AllMembers` | `,` | All Members. |

### `MfaLevel`

Whether guild moderators must have two-factor authentication enabled.

```rust
pub enum MfaLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | None. |
| `Elevated` | `,` | Elevated. |

### `VerificationLevel`

How hard Discord makes new members prove they are human.

```rust
pub enum VerificationLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | No verification required. |
| `Low` | `,` | The account must be verified by email. |
| `Medium` | `,` | The account must be at least five minutes old. |
| `High` | `,` | The member must have been in the server for ten minutes. |
| `VeryHigh` | `,` | The member must have a verified phone number. |

### `AgeRestrictionLevel`

Possible age restriction level values.

```rust
pub enum AgeRestrictionLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Default` | `,` | Default. |
| `Explicit` | `,` | Explicit. |
| `Safe` | `,` | Safe. |
| `AgeRestricted` | `,` | Age Restricted. |

### `PremiumTier`

The guild's boost tier.

```rust
pub enum PremiumTier: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | No boosts. |
| `Tier1` | `,` | Level 1, two boosts. |
| `Tier2` | `,` | Level 2, seven boosts. |
| `Tier3` | `,` | Level 3, fourteen boosts. |

### `Feature`

An experiment or capability enabled on a guild.

```rust
pub enum Feature
```

| Variant | Payload | Description |
| --- | --- | --- |
| `AnimatedBanner` | `,` | Animated Banner. |
| `AnimatedIcon` | `,` | Animated Icon. |
| `ApplicationCommandPermissionsV2` | `,` | Application Command Permissions V 2. |
| `AutoModeration` | `,` | Auto Moderation. |
| `Banner` | `,` | Banner. |
| `Community` | `,` | Community. |
| `CreatorMonetizableProvisional` | `,` | Creator Monetizable Provisional. |
| `CreatorStorePage` | `,` | Creator Store Page. |
| `DeveloperSupportServer` | `,` | Developer Support Server. |
| `Discoverable` | `,` | Discoverable. |
| `EnhancedRoleColors` | `,` | Enhanced Role Colors. |
| `Featurable` | `,` | Featurable. |
| `GuestsEnabled` | `,` | Guests Enabled. |
| `GuildTags` | `,` | Guild Tags. |
| `HasDirectoryEntry` | `,` | Has Directory Entry. |
| `Hub` | `,` | Hub. |
| `InvitesDisabled` | `,` | Invites Disabled. |
| `InviteSplash` | `,` | Invite Splash. |
| `LinkedToHub` | `,` | Linked To Hub. |
| `MemberVerificationGateEnabled` | `,` | Member Verification Gate Enabled. |
| `MonetizationEnabled` | `,` | Monetization Enabled. |
| `MoreSoundboard` | `,` | More Soundboard. |
| `MoreStickers` | `,` | More Stickers. |
| `News` | `,` | News. |
| `Partnered` | `,` | Partnered. |
| `PinPermissionMigrationComplete` | `,` | Pin Permission Migration Complete. |
| `PreviewEnabled` | `,` | Preview Enabled. |
| `PrivateThreads` | `,` | Private Threads. |
| `PruneRequiresAdmin` | `,` | Prune Requires Admin. |
| `RaidAlertsDisabled` | `,` | Raid Alerts Disabled. |
| `RelayEnabled` | `,` | Relay Enabled. |
| `RoleIcons` | `,` | Role Icons. |
| `RoleSubscriptionsAvailableForPurchase` | `,` | Role Subscriptions Available For Purchase. |
| `RoleSubscriptionsEnabled` | `,` | Role Subscriptions Enabled. |
| `Soundboard` | `,` | Soundboard. |
| `TicketedEventsEnabled` | `,` | Ticketed Events Enabled. |
| `VanityUrl` | `,` | Vanity URL. |
| `Verified` | `,` | Verified. |
| `VipRegions` | `,` | Vip Regions. |
| `WelcomeScreenEnabled` | `,` | Welcome Screen Enabled. |

### `Preview`

Discord's preview object.

```rust
pub struct Preview
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `splash` | `Option<ImageHash>` | The invite splash hash. |
| `discovery_splash` | `Option<ImageHash>` | The discovery splash hash. |
| `emojis` | `Vec<Emoji>` | The emojis in the guild. |
| `features` | `Vec<Feature>` | The feature flags enabled for the guild. |
| `approximate_member_count` | `u32` | The approximate member count from the widget. |
| `approximate_presence_count` | `u32` | The approximate online count from the widget. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `stickers` | `Vec<Sticker>` | The stickers attached to the message. |

### `WidgetSettings`

Discord's widget settings object.

```rust
pub struct WidgetSettings
```

| Field | Type | Description |
| --- | --- | --- |
| `enabled` | `bool` | Whether the feature is enabled. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |

### `Widget`

The public guild widget data served without authentication.

```rust
pub struct Widget
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `instant_invite` | `Option<CompactString>` | The instant invite. |
| `channels` | `Vec<WidgetChannel>` | The channels. |
| `members` | `Vec<WidgetMember>` | The members in this collection. |
| `presence_count` | `u32` | How many presence there are. |

### `WidgetChannel`

Discord's widget channel object.

```rust
pub struct WidgetChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |

### `WidgetMember`

Discord's widget member object.

```rust
pub struct WidgetMember
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CompactString` | The object's snowflake ID. |
| `username` | `CompactString` | The account username. |
| `discriminator` | `CompactString` | The discriminator. |
| `avatar` | `Option<ImageHash>` | The avatar hash. |
| `status` | `Status` | The current status. |
| `activity` | `Option<WidgetActivity>` | The activity. |
| `avatar_url` | `CompactString` | The URL for the avatar. |

### `WidgetActivity`

Discord's widget activity object.

```rust
pub struct WidgetActivity
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |

### `WidgetStyle`

Possible widget style values.

```rust
pub enum WidgetStyle
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Shield` | `,` | Shield. |
| `Banner1` | `,` | Banner 1. |
| `Banner2` | `,` | Banner 2. |
| `Banner3` | `,` | Banner 3. |
| `Banner4` | `,` | Banner 4. |

### `Integration`

A linked service such as Twitch or YouTube, with its subscription settings.

```rust
pub struct Integration
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `IntegrationId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `kind` | `IntegrationType` | The object's type discriminator. |
| `enabled` | `bool` | Whether the feature is enabled. |
| `syncing` | `bool` | The syncing. |
| `role_id` | `Option<RoleId>` | The ID of the role. |
| `enable_emoticons` | `bool` | The enable emoticons. |
| `expire_behavior` | `Option<IntegrationExpireBehavior>` | The expire behavior. |
| `expire_grace_period` | `Option<u32>` | The expire grace period. |
| `user` | `Option<Box<User>>` | The user this object describes. |
| `account` | `IntegrationAccount` | The account. |
| `synced_at` | `Option<Timestamp>` | When the synced happened, if known. |
| `subscriber_count` | `Option<u32>` | How many users subscribed. |
| `revoked` | `bool` | The revoked. |
| `application` | `Option<Box<IntegrationApplication>>` | The application this belongs to. |
| `scopes` | `Vec<Scope>` | The scopes. |

### `PartialIntegration`

Discord's partial integration object.

```rust
pub struct PartialIntegration
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `IntegrationId` | The object's snowflake ID. |
| `name` | `Option<CompactString>` | The object's name. |
| `kind` | `Option<IntegrationType>` | The object's type discriminator. |
| `enabled` | `bool` | Whether the feature is enabled. |
| `syncing` | `bool` | The syncing. |
| `role_id` | `Option<RoleId>` | The ID of the role. |
| `enable_emoticons` | `bool` | The enable emoticons. |
| `expire_behavior` | `Option<IntegrationExpireBehavior>` | The expire behavior. |
| `expire_grace_period` | `Option<u32>` | The expire grace period. |
| `user` | `Option<Box<User>>` | The user this object describes. |
| `account` | `Option<IntegrationAccount>` | The account. |
| `synced_at` | `Option<Timestamp>` | When the synced happened, if known. |
| `subscriber_count` | `Option<u32>` | How many users subscribed. |
| `revoked` | `bool` | The revoked. |
| `application` | `Option<Box<IntegrationApplication>>` | The application this belongs to. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `scopes` | `Vec<Scope>` | The scopes. |

### `IntegrationType`

Where an application can be installed: guild or user.

```rust
pub enum IntegrationType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Twitch` | `,` | Twitch. |
| `Youtube` | `,` | Youtube. |
| `Discord` | `,` | Discord. |
| `GuildSubscription` | `,` | Guild Subscription. |

### `IntegrationExpireBehavior`

Possible integration expire behavior values.

```rust
pub enum IntegrationExpireBehavior: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `RemoveRole` | `,` | Remove Role. |
| `Kick` | `,` | Kick. |

### `IntegrationAccount`

Discord's integration account object.

```rust
pub struct IntegrationAccount
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CompactString` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |

### `IntegrationApplication`

Discord's integration application object.

```rust
pub struct IntegrationApplication
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ApplicationId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `description` | `CompactString` | A human-readable description. |
| `bot` | `Option<User>` | Whether the account belongs to a bot. |

### `Ban`

One guild ban: the banned user and who issued it.

```rust
pub struct Ban
```

| Field | Type | Description |
| --- | --- | --- |
| `reason` | `Option<CompactString>` | The reason. |
| `user` | `User` | The user this object describes. |

### `WelcomeScreen`

The welcome screen shown before a member joins.

```rust
pub struct WelcomeScreen
```

| Field | Type | Description |
| --- | --- | --- |
| `description` | `Option<CompactString>` | A human-readable description. |
| `welcome_channels` | `Vec<WelcomeScreenChannel>` | The welcome channels. |

### `WelcomeScreenChannel`

Discord's welcome screen channel object.

```rust
pub struct WelcomeScreenChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `description` | `CompactString` | A human-readable description. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |

#### Methods

##### `new`

Creates a new `WelcomeScreenChannel`.

```rust
pub fn new(channel_id: ChannelId, description: impl Into<CompactString>) -> Self;
```

### `Onboarding`

The onboarding flow shown to new members, with prompts and default channels.

```rust
pub struct Onboarding
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `prompts` | `Vec<OnboardingPrompt>` | The prompts. |
| `default_channel_ids` | `Vec<ChannelId>` | The IDs of the default channel. |
| `enabled` | `bool` | Whether the feature is enabled. |
| `mode` | `OnboardingMode` | The mode. |

### `OnboardingPrompt`

Discord's onboarding prompt object.

```rust
pub struct OnboardingPrompt
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GenericId` | The object's snowflake ID. |
| `kind` | `PromptType` | The object's type discriminator. |
| `options` | `Vec<PromptOption>` | The selectable options or command options. |
| `title` | `CompactString` | The title. |
| `single_select` | `bool` | The single select. |
| `required` | `bool` | Whether the option must be supplied. |
| `in_onboarding` | `bool` | The in onboarding. |

### `PromptOption`

Discord's prompt option object.

```rust
pub struct PromptOption
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GenericId` | The object's snowflake ID. |
| `channel_ids` | `Vec<ChannelId>` | The IDs of the channels. |
| `role_ids` | `Vec<RoleId>` | The IDs of the roles. |
| `emoji` | `Option<PartialEmoji>` | The emoji shown with the object. |
| `title` | `CompactString` | The title. |
| `description` | `Option<CompactString>` | A human-readable description. |

### `OnboardingMode`

Possible onboarding mode values.

```rust
pub enum OnboardingMode: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Default` | `,` | Default. |
| `Advanced` | `,` | Advanced. |

### `PromptType`

Possible prompt type values.

```rust
pub enum PromptType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `MultipleChoice` | `,` | Multiple Choice. |
| `Dropdown` | `,` | Dropdown. |

### `IncidentsData`

Discord's incidents data object.

```rust
pub struct IncidentsData
```

| Field | Type | Description |
| --- | --- | --- |
| `invites_disabled_until` | `Option<Timestamp>` | The invites disabled until. |
| `dms_disabled_until` | `Option<Timestamp>` | The dms disabled until. |
| `dm_spam_detected_at` | `Option<Timestamp>` | When the dm spam detected happened, if known. |
| `raid_detected_at` | `Option<Timestamp>` | When the raid detected happened, if known. |

## `model.guild_template`

### `GuildTemplate`

Discord's guild template object.

```rust
pub struct GuildTemplate
```

| Field | Type | Description |
| --- | --- | --- |
| `code` | `CompactString` | The code value. |
| `name` | `CompactString` | The object's name. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `usage_count` | `u32` | How many usage there are. |
| `creator_id` | `UserId` | The ID of the creator. |
| `creator` | `User` | The user who created it. |
| `created_at` | `Timestamp` | When the created happened, if known. |
| `updated_at` | `Timestamp` | When the updated happened, if known. |
| `source_guild_id` | `GuildId` | The ID of the source guild. |
| `serialized_source_guild` | `SerializedSourceGuild` | The serialized source guild. |
| `is_dirty` | `Option<bool>` | Whether this is dirty. |

### `SerializedSourceGuild`

Discord's serialized source guild object.

```rust
pub struct SerializedSourceGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `region` | `Option<CompactString>` | The voice region. |
| `verification_level` | `Option<VerificationLevel>` | The guild's verification requirement for members. |
| `default_message_notifications` | `Option<DefaultMessageNotificationLevel>` | The default notification level for new members. |
| `explicit_content_filter` | `Option<ExplicitContentFilterLevel>` | The guild's explicit media filter level. |
| `preferred_locale` | `Locale` | The guild's preferred language. |
| `afk_timeout` | `Option<u32>` | How long before a member is moved to the AFK channel, in seconds. |
| `roles` | `Vec<TemplateRole>` | The roles attached or referenced. |
| `channels` | `Vec<TemplateChannel>` | The channels. |
| `afk_channel_id` | `Option<u64>` | The ID of the AFK voice channel. |
| `system_channel_id` | `Option<u64>` | The ID of the channel that receives system messages. |
| `system_channel_flags` | `Option<SystemChannelFlags>` | Which system messages the system channel receives. |
| `icon_hash` | `Option<ImageHash>` | The icon hash. |
| `premium_progress_bar_enabled` | `bool` | Whether the boost progress bar is shown. |

### `TemplateRole`

Discord's template role object.

```rust
pub struct TemplateRole
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `u64` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `permissions` | `Permissions` | The permissions bitfield. |
| `color` | `u32` | The primary role color as a 24-bit integer. |
| `colors` | `Option<RoleColors>` | The role's primary, secondary and tertiary colors. |
| `hoist` | `bool` | Whether the role is shown separately in the member list. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `unicode_emoji` | `Option<CompactString>` | The unicode emoji shown as the role icon. |
| `mentionable` | `bool` | Whether members can mention the role. |

### `TemplateChannel`

Discord's template channel object.

```rust
pub struct TemplateChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `u64` | The object's snowflake ID. |
| `kind` | `ChannelType` | The object's type discriminator. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `topic` | `Option<CompactString>` | The topic. |
| `bitrate` | `Option<u32>` | The voice channel's bitrate in bits per second. |
| `user_limit` | `Option<u32>` | The maximum number of users allowed in the voice channel. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `rate_limit_per_user` | `Option<u16>` | Slowmode delay in seconds between messages per user. |
| `parent_id` | `Option<u64>` | The ID of the parent channel or category. |
| `permission_overwrites` | `Vec<TemplateOverwrite>` | Channel-specific permission overwrites. |
| `rtc_region` | `Option<CompactString>` | The preferred voice region. |
| `video_quality_mode` | `Option<VideoQualityMode>` | The channel's video quality mode. |
| `flags` | `Option<ChannelFlags>` | Bitfield of flags attached to the object. |
| `available_tags` | `Vec<TemplateForumTag>` | The tags that can be applied to new forum posts. |
| `default_auto_archive_duration` | `Option<AutoArchiveDuration>` | The default auto-archive duration for new threads. |
| `default_reaction_emoji` | `Option<DefaultReaction>` | The default reaction emoji for new forum posts. |
| `default_thread_rate_limit_per_user` | `Option<u16>` | The default slowmode for new threads. |
| `default_sort_order` | `Option<SortOrderType>` | The default forum sort order. |
| `default_forum_layout` | `Option<ForumLayoutType>` | The default forum layout. |

### `TemplateOverwrite`

Discord's template overwrite object.

```rust
pub struct TemplateOverwrite
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `u64` | The object's snowflake ID. |
| `kind` | `OverwriteType` | The object's type discriminator. |
| `allow` | `Permissions` | The permissions explicitly allowed by this overwrite. |
| `deny` | `Permissions` | The permissions explicitly denied by this overwrite. |

### `TemplateForumTag`

Discord's template forum tag object.

```rust
pub struct TemplateForumTag
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `moderated` | `bool` | Whether the content is moderated. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |

## `model.emoji`

### `Emoji`

A guild emoji, with name, image, roles and availability.

```rust
pub struct Emoji
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `EmojiId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `roles` | `Vec<RoleId>` | The roles attached or referenced. |
| `user` | `Option<Box<User>>` | The user this object describes. |
| `require_colons` | `bool` | The require colons. |
| `managed` | `bool` | Whether the role or account is managed by an integration. |
| `animated` | `bool` | The animated. |
| `available` | `bool` | Whether the entity is available. |

### `PartialEmoji`

An emoji reference that may be unicode or custom.

```rust
pub enum PartialEmoji
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Custom` | `id: EmojiId, name: Option<CompactString>, animated: bool,` | Custom. |
| `Unicode` | `name: CompactString,` | Unicode. |

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> Option<EmojiId>;
```

##### `name`

Returns the object's name.

```rust
pub fn name(&self) -> Option<&str>;
```

## `model.team`

### `Team`

A team of applications, with members and their roles.

```rust
pub struct Team
```

| Field | Type | Description |
| --- | --- | --- |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `id` | `TeamId` | The object's snowflake ID. |
| `members` | `Vec<Member>` | The members in this collection. |
| `name` | `CompactString` | The object's name. |
| `owner_user_id` | `UserId` | The ID of the owner user. |

### `Member`

A user's membership in one guild: nickname, roles, join date and timeout state.

```rust
pub struct Member
```

| Field | Type | Description |
| --- | --- | --- |
| `membership_state` | `MembershipState` | The membership state. |
| `team_id` | `TeamId` | The ID of the team. |
| `user` | `User` | The user this object describes. |
| `role` | `MemberRole` | The role. |

### `MembershipState`

Whether a team invite is active, pending or accepted.

```rust
pub enum MembershipState: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Invited` | `,` | Invited. |
| `Accepted` | `,` | Accepted. |

### `MemberRole`

The role a user has on a team.

```rust
pub enum MemberRole
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Admin` | `,` | Admin. |
| `Developer` | `,` | Developer. |
| `ReadOnly` | `,` | Read Only. |
