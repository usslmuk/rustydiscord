# Models: users and applications

Users, profiles, applications, OAuth2, SKUs, subscriptions, entitlements and lobbies.

## `model.user`

### `User`

A Discord account: username, discriminator, avatar, flags and optional email.

```rust
pub struct User
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
| `banner` | `Option<ImageHash>` | The banner hash. |
| `accent_color` | `Option<u32>` | The accent color. |
| `public_flags` | `Option<UserFlags>` | The account's public flags bitfield. |
| `avatar_decoration_data` | `Option<Box<AvatarDecorationData>>` | The avatar decoration the member has equipped. |
| `collectibles` | `Option<Box<Collectibles>>` | Profile collectibles shown for the member. |
| `primary_guild` | `Option<Box<PrimaryGuild>>` | The primary guild. |

#### Methods

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

### `CurrentUser`

The bot's own account, including verified and MFA flags.

```rust
pub struct CurrentUser
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
| `mfa_enabled` | `bool` | The mfa enabled. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `accent_color` | `Option<u32>` | The accent color. |
| `locale` | `Option<Locale>` | The locale code. |
| `verified` | `bool` | The verified. |
| `email` | `Option<CompactString>` | The email. |
| `flags` | `Option<UserFlags>` | Bitfield of flags attached to the object. |
| `premium_type` | `Option<PremiumType>` | The premium type. |
| `public_flags` | `Option<UserFlags>` | The account's public flags bitfield. |
| `avatar_decoration_data` | `Option<Box<AvatarDecorationData>>` | The avatar decoration the member has equipped. |
| `collectibles` | `Option<Box<Collectibles>>` | Profile collectibles shown for the member. |
| `primary_guild` | `Option<Box<PrimaryGuild>>` | The primary guild. |

#### Methods

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

### `AvatarDecorationData`

The profile avatar decoration the user has equipped.

```rust
pub struct AvatarDecorationData
```

| Field | Type | Description |
| --- | --- | --- |
| `asset` | `ImageHash` | The asset. |
| `sku_id` | `SkuId` | The ID of the SKU. |

### `Collectibles`

Profile collectibles, including the nameplate.

```rust
pub struct Collectibles
```

| Field | Type | Description |
| --- | --- | --- |
| `nameplate` | `Option<Nameplate>` | The nameplate. |

### `Nameplate`

The nameplate style shown on a user's profile.

```rust
pub struct Nameplate
```

| Field | Type | Description |
| --- | --- | --- |
| `sku_id` | `SkuId` | The ID of the SKU. |
| `asset` | `CompactString` | The asset. |
| `label` | `CompactString` | The human-readable label. |
| `palette` | `NameplatePalette` | The palette. |

### `NameplatePalette`

Possible nameplate palette values.

```rust
pub enum NameplatePalette
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Crimson` | `,` | Crimson. |
| `Berry` | `,` | Berry. |
| `Sky` | `,` | Sky. |
| `Teal` | `,` | Teal. |
| `Forest` | `,` | Forest. |
| `BubbleGum` | `,` | Bubble Gum. |
| `Violet` | `,` | Violet. |
| `Cobalt` | `,` | Cobalt. |
| `Clover` | `,` | Clover. |
| `Lemon` | `,` | Lemon. |
| `White` | `,` | White. |

### `PrimaryGuild`

The guild a user chose to show on their profile.

```rust
pub struct PrimaryGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `identity_guild_id` | `Option<GuildId>` | The ID of the identity guild. |
| `identity_enabled` | `Option<bool>` | The identity enabled. |
| `tag` | `Option<CompactString>` | The tag. |
| `badge` | `Option<ImageHash>` | The badge. |

### `PremiumType`

The user's Nitro tier.

```rust
pub enum PremiumType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | None. |
| `NitroClassic` | `,` | Nitro Classic. |
| `Nitro` | `,` | Nitro. |
| `NitroBasic` | `,` | Nitro Basic. |

### `Connection`

A linked third-party account, with visibility and friend-sync state.

```rust
pub struct Connection
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CompactString` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `kind` | `ConnectionService` | The object's type discriminator. |
| `revoked` | `bool` | The revoked. |
| `integrations` | `Vec<PartialIntegration>` | The integrations. |
| `verified` | `bool` | The verified. |
| `friend_sync` | `bool` | The friend sync. |
| `show_activity` | `bool` | The show activity. |
| `two_way_link` | `bool` | The two way link. |
| `visibility` | `ConnectionVisibility` | The visibility. |

### `ConnectionService`

The third-party service a connection belongs to.

```rust
pub enum ConnectionService
```

| Variant | Payload | Description |
| --- | --- | --- |
| `AmazonMusic` | `,` | Amazon Music. |
| `BattleNet` | `,` | Battle Net. |
| `Bluesky` | `,` | Bluesky. |
| `BungieNet` | `,` | Bungie Net. |
| `Crunchyroll` | `,` | Crunchyroll. |
| `Domain` | `,` | Domain. |
| `Ebay` | `,` | Ebay. |
| `EpicGames` | `,` | Epic Games. |
| `Facebook` | `,` | Facebook. |
| `GitHub` | `,` | Git Hub. |
| `Instagram` | `,` | Instagram. |
| `LeagueOfLegends` | `,` | League Of Legends. |
| `Mastodon` | `,` | Mastodon. |
| `PayPal` | `,` | Pay Pal. |
| `PlayStationNetwork` | `,` | Play Station Network. |
| `Reddit` | `,` | Reddit. |
| `RiotGames` | `,` | Riot Games. |
| `Roblox` | `,` | Roblox. |
| `Spotify` | `,` | Spotify. |
| `Skype` | `,` | Skype. |
| `Steam` | `,` | Steam. |
| `TikTok` | `,` | Tik Tok. |
| `Twitch` | `,` | Twitch. |
| `X` | `,` | X. |
| `Xbox` | `,` | Xbox. |
| `YouTube` | `,` | You Tube. |

### `ConnectionVisibility`

Who can see a linked connection.

```rust
pub enum ConnectionVisibility: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `None` | `,` | None. |
| `Everyone` | `,` | Everyone. |

### `ApplicationRoleConnection`

A user's role-connection metadata for an application.

```rust
pub struct ApplicationRoleConnection
```

| Field | Type | Description |
| --- | --- | --- |
| `platform_name` | `Option<CompactString>` | The platform name. |
| `platform_username` | `Option<CompactString>` | The platform username. |
| `metadata` | `BTreeMap<CompactString, CompactString>` | Additional structured metadata. |

## `model.application`

### `Application`

A Discord application: bot identity, install settings, assets and integration config.

```rust
pub struct Application
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ApplicationId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `description` | `CompactString` | A human-readable description. |
| `rpc_origins` | `Vec<CompactString>` | The rpc origins. |
| `bot_public` | `bool` | The bot public. |
| `bot_require_code_grant` | `bool` | The bot require code grant. |
| `bot` | `Option<Box<User>>` | Whether the account belongs to a bot. |
| `terms_of_service_url` | `Option<CompactString>` | The URL for the terms of service. |
| `privacy_policy_url` | `Option<CompactString>` | The URL for the privacy policy. |
| `owner` | `Option<Box<User>>` | Whether the user owns the guild or application. |
| `verify_key` | `CompactString` | The verify key. |
| `team` | `Option<Box<Team>>` | The team. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `guild` | `Option<Box<PartialGuild>>` | The guild this belongs to, when included. |
| `primary_sku_id` | `Option<SkuId>` | The ID of the primary sku. |
| `slug` | `Option<CompactString>` | The slug. |
| `cover_image` | `Option<ImageHash>` | The cover image hash. |
| `flags` | `Option<ApplicationFlags>` | Bitfield of flags attached to the object. |
| `flags_new` | `Option<ApplicationFlags>` | The flags new. |
| `approximate_guild_count` | `Option<u32>` | How many approximate guild there are. |
| `approximate_user_install_count` | `Option<u32>` | How many approximate user install there are. |
| `approximate_user_authorization_count` | `Option<u32>` | How many approximate user authorization there are. |
| `redirect_uris` | `Vec<CompactString>` | The redirect uris. |
| `interactions_endpoint_url` | `Option<CompactString>` | The URL for the interactions endpoint. |
| `role_connections_verification_url` | `Option<CompactString>` | The URL for the role connections verification. |
| `event_webhooks_url` | `Option<CompactString>` | The URL for the event webhooks. |
| `event_webhooks_status` | `Option<EventWebhookStatus>` | The event webhooks status. |
| `event_webhooks_types` | `Vec<WebhookEventType>` | The event webhooks types. |
| `tags` | `Vec<CompactString>` | Links to the bot, integration or boost that created the role. |
| `install_params` | `Option<InstallParams>` | The install params. |
| `integration_types_config` | `Option<IntegrationTypesConfig>` | The integration types config. |
| `custom_install_url` | `Option<CompactString>` | The URL for the custom install. |

### `PartialApplication`

The reduced application object embedded in other payloads.

```rust
pub struct PartialApplication
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ApplicationId` | The object's snowflake ID. |
| `name` | `Option<CompactString>` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `cover_image` | `Option<ImageHash>` | The cover image hash. |
| `flags` | `Option<ApplicationFlags>` | Bitfield of flags attached to the object. |

### `IntegrationType`

Where an application can be installed: guild or user.

```rust
pub enum IntegrationType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `GuildInstall` | `,` | Guild Install. |
| `UserInstall` | `,` | User Install. |

### `IntegrationTypesConfig`

Discord's integration types config object.

```rust
pub struct IntegrationTypesConfig
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_install` | `Option<IntegrationTypeConfig>` | The guild install. |
| `user_install` | `Option<IntegrationTypeConfig>` | The user install. |

### `IntegrationTypeConfig`

Discord's integration type config object.

```rust
pub struct IntegrationTypeConfig
```

| Field | Type | Description |
| --- | --- | --- |
| `oauth2_install_params` | `Option<InstallParams>` | The oauth 2 install params. |

### `InstallParams`

Default scopes and permissions requested when installing the application.

```rust
pub struct InstallParams
```

| Field | Type | Description |
| --- | --- | --- |
| `scopes` | `Vec<Scope>` | The scopes. |
| `permissions` | `Permissions` | The permissions bitfield. |

#### Methods

##### `new`

Creates a new `InstallParams`.

```rust
pub fn new(scopes: impl IntoIterator<Item = Scope>, permissions: Permissions) -> Self;
```

### `EventWebhookStatus`

Whether the application's event webhook is enabled.

```rust
pub enum EventWebhookStatus: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Disabled` | `,` | Disabled. |
| `Enabled` | `,` | Enabled. |
| `DisabledByDiscord` | `,` | Disabled By Discord. |

### `ActivityInstance`

An activity instance attached to an interaction.

```rust
pub struct ActivityInstance
```

| Field | Type | Description |
| --- | --- | --- |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `instance_id` | `CompactString` | The ID of the instance. |
| `launch_id` | `GenericId` | The ID of the launch. |
| `location` | `ActivityLocation` | The location. |
| `users` | `Vec<UserId>` | The users. |

### `ActivityLocation`

Where an activity instance lives.

```rust
pub struct ActivityLocation
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CompactString` | The object's snowflake ID. |
| `kind` | `ActivityLocationKind` | The object's type discriminator. |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |

### `ActivityLocationKind`

The kind of location an activity instance uses.

```rust
pub enum ActivityLocationKind
```

| Variant | Payload | Description |
| --- | --- | --- |
| `GuildChannel` | `,` | Guild Channel. |
| `PrivateChannel` | `,` | Private Channel. |

## `model.application_identity_profile`

### `ApplicationIdentity`

Discord's application identity object.

```rust
pub struct ApplicationIdentity
```

| Field | Type | Description |
| --- | --- | --- |
| `user_id` | `Option<UserId>` | The ID of the user. |
| `provider_type` | `CompactString` | The provider type. |
| `provider_id` | `Option<CompactString>` | The ID of the provider. |
| `provider_issued_user_id` | `CompactString` | The ID of the provider issued user. |

### `ApplicationIdentityProfile`

Discord's application identity profile object.

```rust
pub struct ApplicationIdentityProfile
```

| Field | Type | Description |
| --- | --- | --- |
| `username` | `Option<CompactString>` | The account username. |
| `metadata` | `Option<BTreeMap<CompactString, JsonValue>>` | Additional structured metadata. |
| `data` | `Option<ProfileData>` | The data. |

### `ProfileData`

Discord's profile data object.

```rust
pub struct ProfileData
```

| Field | Type | Description |
| --- | --- | --- |
| `primary` | `Option<Box<PrimaryProfileData>>` | The primary. |
| `dynamic` | `Vec<DynamicField>` | The dynamic. |

### `PrimaryProfileData`

Discord's primary profile data object.

```rust
pub struct PrimaryProfileData
```

| Field | Type | Description |
| --- | --- | --- |
| `season` | `Option<CompactString>` | The season. |
| `rank_name` | `Option<CompactString>` | The rank name. |
| `rank_image` | `Option<ProfileMedia>` | The rank image. |
| `highest_rank` | `Option<CompactString>` | The highest rank. |
| `highest_rank_image` | `Option<ProfileMedia>` | The highest rank image. |
| `featured_played_character` | `Option<CompactString>` | The featured played character. |
| `featured_played_character_image` | `Option<ProfileMedia>` | The featured played character image. |
| `playtime_hours` | `Option<f64>` | The playtime hours. |
| `total_wins` | `Option<u64>` | The total wins. |
| `current_period_wins` | `Option<u64>` | The current period wins. |
| `total_games` | `Option<u64>` | The total games. |
| `current_period_games` | `Option<u64>` | The current period games. |
| `total_kills` | `Option<u64>` | The total kills. |
| `current_period_kills` | `Option<u64>` | The current period kills. |
| `total_assists` | `Option<u64>` | The total assists. |
| `current_period_assists` | `Option<u64>` | The current period assists. |
| `total_deaths` | `Option<u64>` | The total deaths. |
| `current_period_deaths` | `Option<u64>` | The current period deaths. |

### `ProfileMedia`

Discord's profile media object.

```rust
pub struct ProfileMedia
```

| Field | Type | Description |
| --- | --- | --- |
| `url` | `CompactString` | The URL. |

#### Methods

##### `new`

Creates a new `ProfileMedia`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

### `DynamicField`

Discord's dynamic field object.

```rust
pub struct DynamicField
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `value` | `DynamicFieldValue` | The option's value. |

#### Methods

##### `new`

Creates a new `DynamicField`.

```rust
pub fn new(name: impl Into<CompactString>, value: DynamicFieldValue) -> Self;
```

### `DynamicFieldValue`

Possible dynamic field value values.

```rust
pub enum DynamicFieldValue
```

| Variant | Payload | Description |
| --- | --- | --- |
| `String` | `CompactString` | String. |
| `Number` | `f64` | Number. |
| `Media` | `ProfileMedia` | Media. |
| `Unknown` | `DynamicFieldType` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> DynamicFieldType;
```

### `DynamicFieldType`

Possible dynamic field type values.

```rust
pub enum DynamicFieldType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `String` | `,` | String. |
| `Number` | `,` | Number. |
| `Media` | `,` | Media. |

## `model.application_role_connection_metadata`

### `ApplicationRoleConnectionMetadata`

Discord's application role connection metadata object.

```rust
pub struct ApplicationRoleConnectionMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `MetadataType` | The object's type discriminator. |
| `key` | `CompactString` | The key. |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |

#### Methods

##### `new`

Creates a new `ApplicationRoleConnectionMetadata`.

```rust
pub fn new( kind: MetadataType, key: impl Into<CompactString>, name: impl Into<CompactString>, description: impl Into<CompactString>, ) -> Self;
```

### `MetadataType`

Possible metadata type values.

```rust
pub enum MetadataType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `IntegerLessThanOrEqual` | `,` | Integer Less Than Or Equal. |
| `IntegerGreaterThanOrEqual` | `,` | Integer Greater Than Or Equal. |
| `IntegerEqual` | `,` | Integer Equal. |
| `IntegerNotEqual` | `,` | Integer Not Equal. |
| `DatetimeLessThanOrEqual` | `,` | Datetime Less Than Or Equal. |
| `DatetimeGreaterThanOrEqual` | `,` | Datetime Greater Than Or Equal. |
| `BooleanEqual` | `,` | Boolean Equal. |
| `BooleanNotEqual` | `,` | Boolean Not Equal. |

## `model.oauth2`

### `Scope`

An OAuth2 scope the application can request.

```rust
pub enum Scope
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ActivitiesRead` | `,` | Activities Read. |
| `ActivitiesWrite` | `,` | Activities Write. |
| `ApplicationsBuildsRead` | `,` | Applications Builds Read. |
| `ApplicationsBuildsUpload` | `,` | Applications Builds Upload. |
| `ApplicationsCommands` | `,` | Applications Commands. |
| `ApplicationsCommandsUpdate` | `,` | Applications Commands Update. |
| `ApplicationsCommandsPermissionsUpdate` | `,` | Applications Commands Permissions Update. |
| `ApplicationsEntitlements` | `,` | Applications Entitlements. |
| `ApplicationsStoreUpdate` | `,` | Applications Store Update. |
| `Bot` | `,` | Bot. |
| `Connections` | `,` | Connections. |
| `DmChannelsRead` | `,` | DM Channels Read. |
| `Email` | `,` | Email. |
| `GdmJoin` | `,` | Gdm Join. |
| `Guilds` | `,` | Guilds. |
| `GuildsJoin` | `,` | Guilds Join. |
| `GuildsMembersRead` | `,` | Guilds Members Read. |
| `Identify` | `,` | Identify. |
| `IdentifyPremium` | `,` | Identify Premium. |
| `MessagesRead` | `,` | Messages Read. |
| `RelationshipsRead` | `,` | Relationships Read. |
| `RoleConnectionsWrite` | `,` | Role Connections Write. |
| `Rpc` | `,` | Rpc. |
| `RpcActivitiesWrite` | `,` | Rpc Activities Write. |
| `RpcNotificationsRead` | `,` | Rpc Notifications Read. |
| `RpcVoiceRead` | `,` | Rpc Voice Read. |
| `RpcVoiceWrite` | `,` | Rpc Voice Write. |
| `Voice` | `,` | Voice. |
| `WebhookIncoming` | `,` | Webhook Incoming. |

### `AccessTokenResponse`

The token payload returned by the OAuth2 token endpoints.

```rust
pub struct AccessTokenResponse
```

| Field | Type | Description |
| --- | --- | --- |
| `access_token` | `String` | The OAuth2 access token. |
| `token_type` | `CompactString` | The token type. |
| `expires_in` | `u64` | The expires in. |
| `refresh_token` | `Option<String>` | The refresh token. |
| `scope` | `CompactString` | The scope. |
| `guild` | `Option<Box<Guild>>` | The guild this belongs to, when included. |
| `webhook` | `Option<Box<Webhook>>` | The webhook. |

#### Methods

##### `scopes`

Sets the requested OAuth2 scopes.

```rust
pub fn scopes(&self) -> impl Iterator<Item = Scope> + '_;
```

### `CurrentAuthorizationInformation`

The application's current authorization: scopes, expiry and user.

```rust
pub struct CurrentAuthorizationInformation
```

| Field | Type | Description |
| --- | --- | --- |
| `application` | `PartialApplication` | The application this belongs to. |
| `scopes` | `Vec<Scope>` | The scopes. |
| `expires` | `Timestamp` | The expires. |
| `user` | `Option<Box<User>>` | The user this object describes. |

## `model.sku`

### `Sku`

A purchasable SKU: type, application, name, features and price.

```rust
pub struct Sku
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `SkuId` | The object's snowflake ID. |
| `kind` | `SkuType` | The object's type discriminator. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `name` | `CompactString` | The object's name. |
| `slug` | `CompactString` | The slug. |
| `flags` | `SkuFlags` | Bitfield of flags attached to the object. |

### `SkuType`

Whether the SKU is a subscription, a one-time purchase, or consumable.

```rust
pub enum SkuType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Durable` | `,` | A one-time durable purchase. |
| `Consumable` | `,` | A one-time consumable purchase. |
| `Subscription` | `,` | A recurring subscription. |
| `SubscriptionGroup` | `,` | A group of subscriptions sold together. |

## `model.subscription`

### `Subscription`

A subscription: SKU, user, status and renewal window.

```rust
pub struct Subscription
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `SubscriptionId` | The object's snowflake ID. |
| `user_id` | `UserId` | The ID of the user. |
| `sku_ids` | `Vec<SkuId>` | The IDs of the sku. |
| `entitlement_ids` | `Vec<EntitlementId>` | The IDs of the entitlement. |
| `renewal_sku_ids` | `Option<Vec<SkuId>>` | The IDs of the renewal sku. |
| `current_period_start` | `Timestamp` | The current period start. |
| `current_period_end` | `Timestamp` | The current period end. |
| `status` | `SubscriptionStatus` | The current status. |
| `canceled_at` | `Option<Timestamp>` | When the canceled happened, if known. |
| `country` | `Option<CompactString>` | The country. |

### `SubscriptionStatus`

Whether the subscription is active, ending or ended.

```rust
pub enum SubscriptionStatus: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Active` | `,` | The subscription is active. |
| `Inactive` | `,` | Inactive. |
| `Ending` | `,` | The subscription will end at the current period's close. |

## `model.entitlement`

### `Entitlement`

A purchased or granted entitlement: SKU, user, guild and time window.

```rust
pub struct Entitlement
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `EntitlementId` | The object's snowflake ID. |
| `sku_id` | `SkuId` | The ID of the SKU. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `user_id` | `Option<UserId>` | The ID of the user. |
| `kind` | `EntitlementType` | The object's type discriminator. |
| `deleted` | `bool` | The deleted. |
| `starts_at` | `Option<Timestamp>` | When the starts happened, if known. |
| `ends_at` | `Option<Timestamp>` | When the ends happened, if known. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `consumed` | `bool` | The consumed. |

#### Methods

##### `is_active`

Returns whether the object is currently active.

```rust
pub fn is_active(&self, now: Timestamp) -> bool;
```

### `EntitlementType`

Whether the entitlement was purchased, granted or gifted.

```rust
pub enum EntitlementType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Purchase` | `,` | The entitlement was purchased. |
| `PremiumSubscription` | `,` | The entitlement comes from a Nitro subscription. |
| `DeveloperGift` | `,` | The entitlement was granted by the developer. |
| `TestModePurchase` | `,` | The entitlement was purchased in test mode. |
| `FreePurchase` | `,` | The entitlement was granted for free. |
| `UserGift` | `,` | The entitlement was gifted by another user. |
| `PremiumPurchase` | `,` | The entitlement was purchased with Nitro. |
| `ApplicationSubscription` | `,` | The entitlement is an application subscription. |

### `EntitlementOwnerType`

Whether the entitlement belongs to a guild or a user.

```rust
pub enum EntitlementOwnerType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Guild` | `,` | Guild. |
| `User` | `,` | User. |

## `model.lobby`

### `Lobby`

A social-layer lobby: members, channels and metadata.

```rust
pub struct Lobby
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `LobbyId` | The object's snowflake ID. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `metadata` | `BTreeMap<CompactString, CompactString>` | Additional structured metadata. |
| `members` | `Vec<LobbyMember>` | The members in this collection. |
| `linked_channel` | `Option<Box<Channel>>` | The linked channel. |

### `LobbyMember`

A member in a lobby.

```rust
pub struct LobbyMember
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `UserId` | The object's snowflake ID. |
| `metadata` | `BTreeMap<CompactString, CompactString>` | Additional structured metadata. |
| `flags` | `LobbyMemberFlags` | Bitfield of flags attached to the object. |
| `additional_name` | `Option<CompactString>` | The additional name. |

### `LobbyMessage`

A message sent inside a lobby.

```rust
pub struct LobbyMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `MessageId` | The object's snowflake ID. |
| `kind` | `MessageType` | The object's type discriminator. |
| `content` | `CompactString` | The message text. |
| `lobby_id` | `LobbyId` | The ID of the lobby. |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `author` | `User` | The author. |
| `lobby_member` | `Option<LobbyMessageMember>` | The lobby member. |
| `metadata` | `BTreeMap<CompactString, CompactString>` | Additional structured metadata. |
| `moderation_metadata` | `BTreeMap<CompactString, CompactString>` | The moderation metadata. |
| `flags` | `MessageFlags` | Bitfield of flags attached to the object. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `timestamp` | `Option<Timestamp>` | The timestamp. |
| `edited_timestamp` | `Option<Timestamp>` | When the edited happened, if known. |

### `LobbyMessageMember`

Discord's lobby message member object.

```rust
pub struct LobbyMessageMember
```

| Field | Type | Description |
| --- | --- | --- |
| `additional_name` | `CompactString` | The additional name. |

### `LobbyInvite`

An invite to a lobby.

```rust
pub struct LobbyInvite
```

| Field | Type | Description |
| --- | --- | --- |
| `code` | `CompactString` | The code value. |
