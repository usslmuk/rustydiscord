# Models: channels and community

Channels, voice states, events, stickers, soundboard, webhooks and invites.

## `model.channel`

### `Channel`

A channel of any type. Match on the variant to reach type-specific fields.

```rust
pub enum Channel
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Text` | `TextChannel` | Text. |
| `Dm` | `DmChannel` | DM. |
| `Voice` | `VoiceChannel` | Voice. |
| `GroupDm` | `GroupDmChannel` | Group DM. |
| `Category` | `CategoryChannel` | Category. |
| `Announcement` | `TextChannel` | Announcement. |
| `AnnouncementThread` | `ThreadChannel` | Announcement Thread. |
| `PublicThread` | `ThreadChannel` | Public Thread. |
| `PrivateThread` | `ThreadChannel` | Private Thread. |
| `Stage` | `VoiceChannel` | Stage. |
| `Directory` | `DirectoryChannel` | Directory. |
| `Forum` | `ForumChannel` | Forum. |
| `Media` | `ForumChannel` | Media. |
| `Unknown` | `UnknownChannel` | A value this build does not recognize. |

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> ChannelId;
```

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> ChannelType;
```

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(&self) -> Option<GuildId>;
```

##### `name`

Returns the object's name.

```rust
pub fn name(&self) -> Option<&str>;
```

##### `parent_id`

Returns the ID of the parent.

```rust
pub const fn parent_id(&self) -> Option<ChannelId>;
```

##### `flags`

Returns the channel flags bitfield.

```rust
pub const fn flags(&self) -> ChannelFlags;
```

##### `permission_overwrites`

Returns the permission overwrites for roles and members.

```rust
pub fn permission_overwrites(&self) -> &[Overwrite];
```

##### `as_thread`

Returns the thread view of this channel, when it is a thread.

```rust
pub const fn as_thread(&self) -> Option<&ThreadChannel>;
```

### `ChannelType`

What a channel is for: text, voice, category, forum, thread or DM.

```rust
pub enum ChannelType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `GuildText` | `,` | A text channel in a guild. |
| `Dm` | `,` | A one-to-one direct message channel. |
| `GuildVoice` | `,` | A voice channel in a guild. |
| `GroupDm` | `,` | A direct message channel with several users. |
| `GuildCategory` | `,` | A category that groups channels. |
| `GuildAnnouncement` | `,` | An announcement channel whose posts can be followed. |
| `AnnouncementThread` | `,` | A thread created from an announcement channel post. |
| `PublicThread` | `,` | A public thread under a text, forum or media channel. |
| `PrivateThread` | `,` | A private thread visible only to invited members. |
| `GuildStageVoice` | `,` | A stage channel. |
| `GuildDirectory` | `,` | A directory channel used by student hubs. |
| `GuildForum` | `,` | A forum channel whose posts are threads. |
| `GuildMedia` | `,` | A media channel whose posts are threads. |

#### Methods

##### `is_thread`

Returns whether this channel is a thread.

```rust
pub const fn is_thread(self) -> bool;
```

### `TextChannel`

A standard guild text channel, including announcements and media channels.

```rust
pub struct TextChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `permission_overwrites` | `Vec<Overwrite>` | Channel-specific permission overwrites. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `topic` | `Option<CompactString>` | The topic. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `rate_limit_per_user` | `u16` | Slowmode delay in seconds between messages per user. |
| `last_pin_timestamp` | `Option<Timestamp>` | When the newest pinned message was pinned. |
| `default_auto_archive_duration` | `Option<AutoArchiveDuration>` | The default auto-archive duration for new threads. |
| `default_thread_rate_limit_per_user` | `Option<u16>` | The default slowmode for new threads. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `VoiceChannel`

A guild voice or stage channel, with bitrate and user limit.

```rust
pub struct VoiceChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `permission_overwrites` | `Vec<Overwrite>` | Channel-specific permission overwrites. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `topic` | `Option<CompactString>` | The topic. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `rate_limit_per_user` | `u16` | Slowmode delay in seconds between messages per user. |
| `bitrate` | `u32` | The voice channel's bitrate in bits per second. |
| `user_limit` | `u32` | The maximum number of users allowed in the voice channel. |
| `rtc_region` | `Option<CompactString>` | The preferred voice region. |
| `video_quality_mode` | `VideoQualityMode` | The channel's video quality mode. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `CategoryChannel`

A category that groups channels and carries shared permission overwrites.

```rust
pub struct CategoryChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `permission_overwrites` | `Vec<Overwrite>` | Channel-specific permission overwrites. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `ForumChannel`

A forum: posts are threads, with tags, layout and default reaction settings.

```rust
pub struct ForumChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `permission_overwrites` | `Vec<Overwrite>` | Channel-specific permission overwrites. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `topic` | `Option<CompactString>` | The topic. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `rate_limit_per_user` | `u16` | Slowmode delay in seconds between messages per user. |
| `default_auto_archive_duration` | `Option<AutoArchiveDuration>` | The default auto-archive duration for new threads. |
| `available_tags` | `Vec<ForumTag>` | The tags that can be applied to new forum posts. |
| `default_thread_rate_limit_per_user` | `Option<u16>` | The default slowmode for new threads. |
| `default_reaction_emoji` | `Option<DefaultReaction>` | The default reaction emoji for new forum posts. |
| `default_sort_order` | `Option<SortOrderType>` | The default forum sort order. |
| `default_forum_layout` | `ForumLayoutType` | The default forum layout. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `ThreadChannel`

A thread under a text, forum or media channel.

```rust
pub struct ThreadChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `owner_id` | `Option<UserId>` | The ID of the owner. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `rate_limit_per_user` | `u16` | Slowmode delay in seconds between messages per user. |
| `last_pin_timestamp` | `Option<Timestamp>` | When the newest pinned message was pinned. |
| `message_count` | `Option<u32>` | How many messages are in the thread. |
| `member_count` | `Option<u32>` | How many members are in the thread. |
| `total_message_sent` | `Option<u32>` | How many messages the thread has ever received. |
| `thread_metadata` | `ThreadMetadata` | The thread metadata. |
| `member` | `Option<Box<ThreadMember>>` | The guild member this refers to. |
| `applied_tags` | `Vec<TagId>` | The forum tags applied to this thread. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `DmChannel`

A one-to-one direct message channel.

```rust
pub struct DmChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `recipients` | `Vec<User>` | The users in the group DM. |
| `last_pin_timestamp` | `Option<Timestamp>` | When the newest pinned message was pinned. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `GroupDmChannel`

A direct message channel shared by several users.

```rust
pub struct GroupDmChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `name` | `Option<CompactString>` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `recipients` | `Vec<User>` | The users in the group DM. |
| `owner_id` | `UserId` | The ID of the owner. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `managed` | `bool` | Whether the role or account is managed by an integration. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `last_pin_timestamp` | `Option<Timestamp>` | When the newest pinned message was pinned. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `DirectoryChannel`

A directory channel used by student hubs.

```rust
pub struct DirectoryChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `permission_overwrites` | `Vec<Overwrite>` | Channel-specific permission overwrites. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |

### `UnknownChannel`

Discord's unknown channel object.

```rust
pub struct UnknownChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `kind` | `ChannelType` | The object's type discriminator. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `Option<CompactString>` | The object's name. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |


### `Overwrite`

One permission overwrite for a role or member.

```rust
pub struct Overwrite
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GenericId` | The object's snowflake ID. |
| `kind` | `OverwriteType` | The object's type discriminator. |
| `allow` | `Permissions` | The permissions explicitly allowed by this overwrite. |
| `deny` | `Permissions` | The permissions explicitly denied by this overwrite. |

#### Methods

##### `new`

Creates a new `Overwrite`.

```rust
pub const fn new( id: GenericId, kind: OverwriteType, allow: Permissions, deny: Permissions, ) -> Self;
```

##### `role`

Builds an overwrite that allows and denies permissions for a role.

```rust
pub const fn role(role_id: RoleId, allow: Permissions, deny: Permissions) -> Self;
```

##### `member`

Builds an overwrite that allows and denies permissions for one member.

```rust
pub const fn member(user_id: UserId, allow: Permissions, deny: Permissions) -> Self;
```

##### `role_id`

Returns the ID of the role.

```rust
pub fn role_id(&self) -> Option<RoleId>;
```

##### `user_id`

Returns the ID of the user.

```rust
pub fn user_id(&self) -> Option<UserId>;
```

### `OverwriteType`

Whether an overwrite targets a role or a member.

```rust
pub enum OverwriteType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Role` | `,` | The overwrite applies to a role. |
| `Member` | `,` | The overwrite applies to one member. |

### `VideoQualityMode`

Possible video quality mode values.

```rust
pub enum VideoQualityMode: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Auto` | `,` | Auto. |
| `Full` | `,` | Full. |

### `SortOrderType`

Possible sort order type values.

```rust
pub enum SortOrderType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `LatestActivity` | `,` | Latest Activity. |
| `CreationDate` | `,` | Creation Date. |

### `ForumLayoutType`

Possible forum layout type values.

```rust
pub enum ForumLayoutType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `NotSet` | `,` | Not Set. |
| `ListView` | `,` | List View. |
| `GalleryView` | `,` | Gallery View. |

### `AutoArchiveDuration`

Possible auto archive duration values.

```rust
pub enum AutoArchiveDuration: u16
```

| Variant | Payload | Description |
| --- | --- | --- |
| `OneHour` | `,` | One Hour. |
| `OneDay` | `,` | One Day. |
| `ThreeDays` | `,` | Three Days. |
| `OneWeek` | `,` | One Week. |

### `ThreadMetadata`

Thread state: archive duration, lock, auto-archive and creation time.

```rust
pub struct ThreadMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `archived` | `bool` | Whether the thread is archived. |
| `auto_archive_duration` | `AutoArchiveDuration` | How long before the thread auto-archives, in minutes. |
| `archive_timestamp` | `Timestamp` | When the thread was archived. |
| `locked` | `bool` | Whether only moderators can unarchive the thread. |
| `invitable` | `bool` | Whether non-members can be invited to the thread. |
| `create_timestamp` | `Option<Timestamp>` | When the create happened, if known. |

### `ThreadMember`

A user's membership in a thread, with join time and notification flags.

```rust
pub struct ThreadMember
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<ChannelId>` | The object's snowflake ID. |
| `user_id` | `Option<UserId>` | The ID of the user. |
| `join_timestamp` | `Timestamp` | When the join happened, if known. |
| `flags` | `ThreadMemberFlags` | Bitfield of flags attached to the object. |
| `member` | `Option<Box<Member>>` | The guild member this refers to. |

### `DefaultReaction`

The emoji a forum uses for new posts by default.

```rust
pub struct DefaultReaction
```

| Field | Type | Description |
| --- | --- | --- |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |

#### Methods

##### `custom`

Sets a custom activity.

```rust
pub const fn custom(emoji_id: EmojiId) -> Self;
```

##### `unicode`

Uses a unicode emoji as the default forum reaction.

```rust
pub fn unicode(emoji: impl Into<CompactString>) -> Self;
```

### `ForumTag`

One selectable tag on a forum channel.

```rust
pub struct ForumTag
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `TagId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `moderated` | `bool` | Whether the content is moderated. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |

### `FollowedChannel`

An announcement channel this channel follows.

```rust
pub struct FollowedChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `webhook_id` | `WebhookId` | The ID of the webhook. |

## `model.voice`

### `VoiceState`

A user's voice state: channel, session, mute, deaf, stream and video flags.

```rust
pub struct VoiceState
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `user_id` | `UserId` | The ID of the user. |
| `member` | `Option<Box<Member>>` | The guild member this refers to. |
| `session_id` | `CompactString` | The ID of the session. |
| `deaf` | `bool` | Whether the member is server-deafened. |
| `mute` | `bool` | Whether the member is server-muted. |
| `self_deaf` | `bool` | The self deaf. |
| `self_mute` | `bool` | The self mute. |
| `self_stream` | `bool` | The self stream. |
| `self_video` | `bool` | The self video. |
| `suppress` | `bool` | The suppress. |
| `request_to_speak_timestamp` | `Option<Timestamp>` | When the request to speak happened, if known. |

### `VoiceRegion`

A voice server region with latency and custom flags.

```rust
pub struct VoiceRegion
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CompactString` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `optimal` | `bool` | The optimal. |
| `deprecated` | `bool` | The deprecated. |
| `custom` | `bool` | The custom. |

## `model.guild_scheduled_event`

### `GuildScheduledEvent`

A scheduled event: name, time, location, status and subscriber count.

```rust
pub struct GuildScheduledEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ScheduledEventId` | The object's snowflake ID. |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `creator_id` | `Option<UserId>` | The ID of the creator. |
| `name` | `CompactString` | The object's name. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `scheduled_start_time` | `Timestamp` | When the event starts. |
| `scheduled_end_time` | `Option<Timestamp>` | When the event ends. |
| `privacy_level` | `PrivacyLevel` | Who can see the event. |
| `status` | `Status` | The current status. |
| `entity_type` | `EntityType` | The kind of event entity. |
| `entity_id` | `Option<GenericId>` | The ID of the entity. |
| `entity_metadata` | `Option<EntityMetadata>` | Metadata for the scheduled event's location or entity. |
| `creator` | `Option<Box<User>>` | The user who created it. |
| `user_count` | `Option<u32>` | How many users are interested. |
| `image` | `Option<ImageHash>` | The image hash or URL. |
| `recurrence_rule` | `Option<Box<RecurrenceRule>>` | The recurrence rule. |

### `PrivacyLevel`

Who can see the scheduled event.

```rust
pub enum PrivacyLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `GuildOnly` | `,` | Visible to guild members only. |

### `EntityType`

Where the event happens: stage, voice or external location.

```rust
pub enum EntityType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `StageInstance` | `,` | The event runs in a stage channel. |
| `Voice` | `,` | The event runs in a voice channel. |
| `External` | `,` | The event happens at an external location. |

### `Status`

The event lifecycle: scheduled, active, completed or canceled.

```rust
pub enum Status: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Scheduled` | `,` | The event has not started. |
| `Active` | `,` | The event is running. |
| `Completed` | `,` | The event finished. |
| `Canceled` | `,` | The event was canceled. |

### `EntityMetadata`

The location of an external event.

```rust
pub struct EntityMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `location` | `Option<CompactString>` | The location. |

#### Methods

##### `new`

Creates a new `EntityMetadata`.

```rust
pub fn new(location: impl Into<CompactString>) -> Self;
```

### `GuildScheduledEventUser`

Discord's guild scheduled event user object.

```rust
pub struct GuildScheduledEventUser
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_scheduled_event_id` | `ScheduledEventId` | The ID of the scheduled event. |
| `user` | `User` | The user this object describes. |
| `member` | `Option<Box<PartialMember>>` | The guild member this refers to. |

### `RecurrenceRule`

The recurrence pattern for a repeating event.

```rust
pub struct RecurrenceRule
```

| Field | Type | Description |
| --- | --- | --- |
| `start` | `Timestamp` | The start. |
| `end` | `Option<Timestamp>` | The end. |
| `frequency` | `RecurrenceFrequency` | The frequency. |
| `interval` | `u16` | The interval between runs. |
| `by_weekday` | `Option<Vec<RecurrenceWeekday>>` | The by weekday. |
| `by_n_weekday` | `Option<Vec<RecurrenceNWeekday>>` | The by n weekday. |
| `by_month` | `Option<Vec<RecurrenceMonth>>` | The by month. |
| `by_month_day` | `Option<Vec<u8>>` | The by month day. |
| `by_year_day` | `Option<Vec<u16>>` | The by year day. |
| `count` | `Option<u32>` | The count. |

### `RecurrenceFrequency`

Possible recurrence frequency values.

```rust
pub enum RecurrenceFrequency: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Yearly` | `,` | Yearly. |
| `Monthly` | `,` | Monthly. |
| `Weekly` | `,` | Weekly. |
| `Daily` | `,` | Daily. |

### `RecurrenceWeekday`

Possible recurrence weekday values.

```rust
pub enum RecurrenceWeekday: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Monday` | `,` | Monday. |
| `Tuesday` | `,` | Tuesday. |
| `Wednesday` | `,` | Wednesday. |
| `Thursday` | `,` | Thursday. |
| `Friday` | `,` | Friday. |
| `Saturday` | `,` | Saturday. |
| `Sunday` | `,` | Sunday. |

### `RecurrenceNWeekday`

Discord's recurrence n weekday object.

```rust
pub struct RecurrenceNWeekday
```

| Field | Type | Description |
| --- | --- | --- |
| `n` | `u8` | The n. |
| `day` | `RecurrenceWeekday` | The day. |

#### Methods

##### `new`

Creates a new `RecurrenceNWeekday`.

```rust
pub const fn new(n: u8, day: RecurrenceWeekday) -> Self;
```

### `RecurrenceMonth`

Possible recurrence month values.

```rust
pub enum RecurrenceMonth: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `January` | `,` | January. |
| `February` | `,` | February. |
| `March` | `,` | March. |
| `April` | `,` | April. |
| `May` | `,` | May. |
| `June` | `,` | June. |
| `July` | `,` | July. |
| `August` | `,` | August. |
| `September` | `,` | September. |
| `October` | `,` | October. |
| `November` | `,` | November. |
| `December` | `,` | December. |

## `model.sticker`

### `Sticker`

A guild or standard sticker, with format, tags and description.

```rust
pub struct Sticker
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `StickerId` | The object's snowflake ID. |
| `pack_id` | `Option<StickerPackId>` | The ID of the pack. |
| `name` | `CompactString` | The object's name. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `tags` | `CompactString` | Links to the bot, integration or boost that created the role. |
| `kind` | `StickerType` | The object's type discriminator. |
| `format_type` | `StickerFormatType` | The format type. |
| `available` | `bool` | Whether the entity is available. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `user` | `Option<Box<User>>` | The user this object describes. |
| `sort_value` | `Option<u32>` | The sort value. |

### `StickerType`

Whether a sticker is standard, guild-owned or a pack cover.

```rust
pub enum StickerType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Standard` | `,` | A sticker from a Discord sticker pack. |
| `Guild` | `,` | A sticker uploaded to one guild. |

### `StickerFormatType`

The file format a sticker is served in.

```rust
pub enum StickerFormatType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Png` | `,` | A static PNG sticker. |
| `Apng` | `,` | An animated APNG sticker. |
| `Lottie` | `,` | A Lottie JSON sticker. |
| `Gif` | `,` | An animated GIF sticker. |

### `StickerItem`

The trimmed sticker object attached to messages.

```rust
pub struct StickerItem
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `StickerId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `format_type` | `StickerFormatType` | The format type. |

### `StickerPack`

A pack of standard stickers.

```rust
pub struct StickerPack
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `StickerPackId` | The object's snowflake ID. |
| `stickers` | `Vec<Sticker>` | The stickers attached to the message. |
| `name` | `CompactString` | The object's name. |
| `sku_id` | `SkuId` | The ID of the SKU. |
| `cover_sticker_id` | `Option<StickerId>` | The ID of the cover sticker. |
| `description` | `CompactString` | A human-readable description. |
| `banner_asset_id` | `Option<GenericId>` | The ID of the banner asset. |

## `model.soundboard`

### `SoundboardSound`

A soundboard sound with its audio URL, volume and emoji.

```rust
pub struct SoundboardSound
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `sound_id` | `SoundboardSoundId` | The ID of the soundboard sound. |
| `volume` | `f64` | Playback volume. |
| `emoji_id` | `Option<EmojiId>` | The ID of the emoji. |
| `emoji_name` | `Option<CompactString>` | The emoji name. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `available` | `bool` | Whether the entity is available. |
| `user` | `Option<Box<User>>` | The user this object describes. |

## `model.stage_instance`

### `StageInstance`

A live stage: topic, privacy level and the channel it runs in.

```rust
pub struct StageInstance
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `StageInstanceId` | The object's snowflake ID. |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `channel_id` | `ChannelId` | The ID of the channel this belongs to. |
| `topic` | `CompactString` | The topic. |
| `privacy_level` | `PrivacyLevel` | Who can see the event. |
| `discoverable_disabled` | `bool` | The discoverable disabled. |
| `guild_scheduled_event_id` | `Option<ScheduledEventId>` | The ID of the scheduled event. |

### `PrivacyLevel`

Who can see the scheduled event.

```rust
pub enum PrivacyLevel: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Public` | `,` | Public. |
| `GuildOnly` | `,` | Guild Only. |

## `model.webhook`

### `Webhook`

A webhook: channel, token, type, owner and source guild or channel.

```rust
pub struct Webhook
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `WebhookId` | The object's snowflake ID. |
| `kind` | `WebhookType` | The object's type discriminator. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `user` | `Option<Box<User>>` | The user this object describes. |
| `name` | `Option<CompactString>` | The object's name. |
| `avatar` | `Option<ImageHash>` | The avatar hash. |
| `token` | `Option<CompactString>` | The token. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `source_guild` | `Option<SourceGuild>` | The source guild. |
| `source_channel` | `Option<SourceChannel>` | The source channel. |
| `url` | `Option<CompactString>` | The URL. |

### `WebhookType`

Whether a webhook is incoming, a follower, or created by an application.

```rust
pub enum WebhookType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Incoming` | `,` | A webhook that posts into one channel. |
| `ChannelFollower` | `,` | A webhook that mirrors an announcement channel. |
| `Application` | `,` | A webhook created by an application. |

### `SourceGuild`

The guild a follower webhook mirrors.

```rust
pub struct SourceGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |

### `SourceChannel`

The channel a follower webhook mirrors.

```rust
pub struct SourceChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |

## `model.webhook_event`

### `WebhookEventPayload`

The event body, matching the event type.

```rust
pub struct WebhookEventPayload
```

| Field | Type | Description |
| --- | --- | --- |
| `version` | `u8` | The version. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `kind` | `PayloadType` | The object's type discriminator. |
| `event` | `Option<WebhookEvent>` | The event. |

### `PayloadType`

The payload wrapper discriminator.

```rust
pub enum PayloadType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Ping` | `,` | Ping. |
| `Event` | `,` | Event. |

### `WebhookEventType`

Which event arrived, such as an application authorization.

```rust
pub enum WebhookEventType
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ApplicationAuthorized` | `,` | Application Authorized. |
| `ApplicationDeauthorized` | `,` | Application Deauthorized. |
| `EntitlementCreate` | `,` | Entitlement Create. |
| `EntitlementUpdate` | `,` | Entitlement Update. |
| `EntitlementDelete` | `,` | Entitlement Delete. |
| `QuestUserEnrollment` | `,` | Quest User Enrollment. |
| `LobbyMessageCreate` | `,` | Lobby Message Create. |
| `LobbyMessageUpdate` | `,` | Lobby Message Update. |
| `LobbyMessageDelete` | `,` | Lobby Message Delete. |
| `GameDirectMessageCreate` | `,` | Game Direct Message Create. |
| `GameDirectMessageUpdate` | `,` | Game Direct Message Update. |
| `GameDirectMessageDelete` | `,` | Game Direct Message Delete. |

### `WebhookEvent`

An event delivered to an application's event webhook.

```rust
pub struct WebhookEvent
```

| Field | Type | Description |
| --- | --- | --- |
| `timestamp` | `Timestamp` | The timestamp. |
| `data` | `WebhookEventData` | The data. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub fn kind(&self) -> WebhookEventType;
```

### `WebhookEventData`

Possible webhook event data values.

```rust
pub enum WebhookEventData
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ApplicationAuthorized` | `Box<ApplicationAuthorized>` | Application Authorized. |
| `ApplicationDeauthorized` | `Box<ApplicationDeauthorized>` | Application Deauthorized. |
| `EntitlementCreate` | `Box<Entitlement>` | Entitlement Create. |
| `EntitlementUpdate` | `Box<Entitlement>` | Entitlement Update. |
| `EntitlementDelete` | `Box<Entitlement>` | Entitlement Delete. |
| `QuestUserEnrollment` | `,` | Quest User Enrollment. |
| `LobbyMessageCreate` | `Box<LobbyMessage>` | Lobby Message Create. |
| `LobbyMessageUpdate` | `Box<LobbyMessage>` | Lobby Message Update. |
| `LobbyMessageDelete` | `LobbyMessageDelete` | Lobby Message Delete. |
| `GameDirectMessageCreate` | `Box<GameDirectMessage>` | Game Direct Message Create. |
| `GameDirectMessageUpdate` | `Box<GameDirectMessage>` | Game Direct Message Update. |
| `GameDirectMessageDelete` | `Box<GameDirectMessage>` | Game Direct Message Delete. |
| `Unknown` | `WebhookEventType` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub fn kind(&self) -> WebhookEventType;
```

### `ApplicationAuthorized`

Discord's application authorized object.

```rust
pub struct ApplicationAuthorized
```

| Field | Type | Description |
| --- | --- | --- |
| `integration_type` | `Option<IntegrationType>` | The integration type. |
| `user` | `User` | The user this object describes. |
| `scopes` | `Vec<Scope>` | The scopes. |
| `guild` | `Option<Box<Guild>>` | The guild this belongs to, when included. |

### `ApplicationDeauthorized`

Discord's application deauthorized object.

```rust
pub struct ApplicationDeauthorized
```

| Field | Type | Description |
| --- | --- | --- |
| `user` | `User` | The user this object describes. |

### `LobbyMessageDelete`

Discord's lobby message delete object.

```rust
pub struct LobbyMessageDelete
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `MessageId` | The object's snowflake ID. |
| `lobby_id` | `LobbyId` | The ID of the lobby. |

### `GameDirectMessage`

Discord's game direct message object.

```rust
pub struct GameDirectMessage
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `MessageId` | The object's snowflake ID. |
| `kind` | `Option<MessageType>` | The object's type discriminator. |
| `content` | `CompactString` | The message text. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `channel` | `Option<Box<Channel>>` | The channel. |
| `lobby_id` | `Option<LobbyId>` | The ID of the lobby. |
| `author` | `User` | The author. |
| `recipient_id` | `Option<GenericId>` | The ID of the recipient. |
| `timestamp` | `Option<Timestamp>` | The timestamp. |
| `edited_timestamp` | `Option<Timestamp>` | When the edited happened, if known. |
| `flags` | `MessageFlags` | Bitfield of flags attached to the object. |
| `application_id` | `Option<ApplicationId>` | The ID of the application this belongs to. |
| `attachments` | `Vec<Attachment>` | The files attached to the message. |
| `embeds` | `Vec<Embed>` | The rich embeds attached to the message. |
| `components` | `Vec<Component>` | The components attached to the message. |
| `activity` | `Option<MessageActivity>` | The activity. |
| `application` | `Option<Box<PartialApplication>>` | The application this belongs to. |

## `model.invite`

### `Invite`

An invite code plus the channel, guild and inviter it points at.

```rust
pub struct Invite
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `InviteType` | The object's type discriminator. |
| `code` | `CompactString` | The code value. |
| `guild` | `Option<Box<InviteGuild>>` | The guild this belongs to, when included. |
| `channel` | `Option<InviteChannel>` | The channel. |
| `inviter` | `Option<Box<User>>` | The inviter. |
| `target_type` | `Option<InviteTargetType>` | The target type. |
| `target_user` | `Option<Box<User>>` | The target user. |
| `target_application` | `Option<Box<PartialApplication>>` | The target application. |
| `approximate_presence_count` | `Option<u32>` | The approximate online count from the widget. |
| `approximate_member_count` | `Option<u32>` | The approximate member count from the widget. |
| `expires_at` | `Option<Timestamp>` | When the expires happened, if known. |
| `guild_scheduled_event` | `Option<Box<GuildScheduledEvent>>` | The guild scheduled event. |
| `flags` | `Option<InviteFlags>` | Bitfield of flags attached to the object. |
| `roles` | `Vec<InviteRole>` | The roles attached or referenced. |

### `InviteWithMetadata`

Discord's invite with metadata object.

```rust
pub struct InviteWithMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `invite` | `Invite` | The invite. |
| `metadata` | `Option<InviteMetadata>` | Additional structured metadata. |

### `InviteMetadata`

Invite usage, creator and expiry details.

```rust
pub struct InviteMetadata
```

| Field | Type | Description |
| --- | --- | --- |
| `uses` | `u32` | The uses. |
| `max_uses` | `u32` | How many times the invite can be used. |
| `max_age` | `u32` | How long the invite stays valid, in seconds. |
| `temporary` | `bool` | The temporary. |
| `created_at` | `Timestamp` | When the created happened, if known. |

### `InviteType`

Whether the invite targets a guild or a group DM.

```rust
pub enum InviteType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Guild` | `,` | The invite joins a guild. |
| `GroupDm` | `,` | The invite joins a group DM. |
| `Friend` | `,` | Friend. |

### `InviteTargetType`

The extra target an invite opens, such as an activity or stream.

```rust
pub enum InviteTargetType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Stream` | `,` | The invite opens a stream. |
| `EmbeddedApplication` | `,` | The invite opens an embedded application. |

### `InviteGuild`

The trimmed guild data included with an invite.

```rust
pub struct InviteGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `splash` | `Option<ImageHash>` | The invite splash hash. |
| `banner` | `Option<ImageHash>` | The banner hash. |
| `description` | `Option<CompactString>` | A human-readable description. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `features` | `Vec<Feature>` | The feature flags enabled for the guild. |
| `verification_level` | `VerificationLevel` | The guild's verification requirement for members. |
| `vanity_url_code` | `Option<CompactString>` | The guild's vanity invite code. |
| `nsfw_level` | `AgeRestrictionLevel` | The guild's NSFW level. |
| `premium_subscription_count` | `Option<u32>` | How many boosts the guild has. |
| `welcome_screen` | `Option<WelcomeScreen>` | The guild's welcome screen configuration. |

### `InviteChannel`

Discord's invite channel object.

```rust
pub struct InviteChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `kind` | `ChannelType` | The object's type discriminator. |
| `name` | `Option<CompactString>` | The object's name. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `recipients` | `Vec<InviteRecipient>` | The users in the group DM. |

### `InviteRecipient`

Discord's invite recipient object.

```rust
pub struct InviteRecipient
```

| Field | Type | Description |
| --- | --- | --- |
| `username` | `CompactString` | The account username. |

### `InviteRole`

Discord's invite role object.

```rust
pub struct InviteRole
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `RoleId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `position` | `i32` | The position among siblings, lowest first. |
| `color` | `u32` | The primary role color as a 24-bit integer. |
| `colors` | `RoleColors` | The role's primary, secondary and tertiary colors. |
| `icon` | `Option<ImageHash>` | The icon hash. |
| `unicode_emoji` | `Option<CompactString>` | The unicode emoji shown as the role icon. |

### `InviteTargetUsersJobStatus`

Discord's invite target users job status object.

```rust
pub struct InviteTargetUsersJobStatus
```

| Field | Type | Description |
| --- | --- | --- |
| `status` | `InviteTargetUsersJobStatusCode` | The current status. |
| `total_users` | `u32` | The total users. |
| `processed_users` | `u32` | The processed users. |
| `created_at` | `Timestamp` | When the created happened, if known. |
| `completed_at` | `Option<Timestamp>` | When the completed happened, if known. |
| `error_message` | `Option<CompactString>` | The error message. |

### `InviteTargetUsersJobStatusCode`

Possible invite target users job status code values.

```rust
pub enum InviteTargetUsersJobStatusCode: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Unspecified` | `,` | Unspecified. |
| `Processing` | `,` | Processing. |
| `Completed` | `,` | Completed. |
| `Failed` | `,` | Failed. |
