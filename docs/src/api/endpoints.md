# Endpoints

Every Discord API call, grouped by resource. Each entry shows its HTTP method and path.

## `rest.endpoints.application`

### `Rest`

##### `get_current_application`

Calls `GET /applications/@me`.

```rust
pub fn get_current_application( &self, ) -> impl Future<Output = Result<Application, Error>> + Send + '_;
```

##### `edit_current_application`

Calls `PATCH /applications/@me`.

```rust
pub fn edit_current_application<'a>( &'a self, body: &'a EditCurrentApplication, ) -> impl Future<Output = Result<Application, Error>> + Send + 'a;
```

##### `get_application_activity_instance`

Calls `GET /applications/{application_id}/activity-instances/{instance_id}`.

```rust
pub fn get_application_activity_instance<'a>( &'a self, application_id: ApplicationId, instance_id: &'a str, ) -> impl Future<Output = Result<ActivityInstance, Error>> + Send + 'a;
```

## `rest.endpoints.application_command`

### `Rest`

##### `get_global_application_commands`

Calls `GET /applications/{application_id}/commands`.

```rust
pub fn get_global_application_commands<'a>( &'a self, application_id: ApplicationId, query: &'a GetApplicationCommands, ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a;
```

##### `create_global_application_command`

Calls `POST /applications/{application_id}/commands`.

```rust
pub fn create_global_application_command<'a>( &'a self, application_id: ApplicationId, body: &'a CreateApplicationCommand, ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a;
```

##### `get_global_application_command`

Calls `GET /applications/{application_id}/commands`.

```rust
pub fn get_global_application_command( &self, application_id: ApplicationId, command_id: CommandId, ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + '_;
```

##### `edit_global_application_command`

Calls `PATCH /applications/{application_id}/commands/{command_id}`.

```rust
pub fn edit_global_application_command<'a>( &'a self, application_id: ApplicationId, command_id: CommandId, body: &'a EditApplicationCommand, ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a;
```

##### `delete_global_application_command`

Calls `DELETE /applications/{application_id}/commands/{command_id}`.

```rust
pub fn delete_global_application_command( &self, application_id: ApplicationId, command_id: CommandId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `bulk_overwrite_global_application_commands`

Calls `PUT /applications/{application_id}/commands`.

```rust
pub fn bulk_overwrite_global_application_commands<'a>( &'a self, application_id: ApplicationId, commands: &'a [CreateApplicationCommand], ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a;
```

##### `get_guild_application_commands`

Calls `GET /applications/{application_id}/guilds/{guild_id}/commands`.

```rust
pub fn get_guild_application_commands<'a>( &'a self, application_id: ApplicationId, guild_id: GuildId, query: &'a GetApplicationCommands, ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a;
```

##### `create_guild_application_command`

Calls `POST /applications/{application_id}/guilds/{guild_id}/commands`.

```rust
pub fn create_guild_application_command<'a>( &'a self, application_id: ApplicationId, guild_id: GuildId, body: &'a CreateApplicationCommand, ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a;
```

##### `get_guild_application_command`

Calls `GET /applications/{application_id}/guilds/{guild_id}/commands`.

```rust
pub fn get_guild_application_command( &self, application_id: ApplicationId, guild_id: GuildId, command_id: CommandId, ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + '_;
```

##### `edit_guild_application_command`

Calls `PATCH /applications/{application_id}/guilds/{guild_id}/commands/{command_id}`.

```rust
pub fn edit_guild_application_command<'a>( &'a self, application_id: ApplicationId, guild_id: GuildId, command_id: CommandId, body: &'a EditApplicationCommand, ) -> impl Future<Output = Result<ApplicationCommand, Error>> + Send + 'a;
```

##### `delete_guild_application_command`

Calls `DELETE /applications/{application_id}/guilds/{guild_id}/commands/{command_id}`.

```rust
pub fn delete_guild_application_command( &self, application_id: ApplicationId, guild_id: GuildId, command_id: CommandId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `bulk_overwrite_guild_application_commands`

Calls `PUT /applications/{application_id}/guilds/{guild_id}/commands`.

```rust
pub fn bulk_overwrite_guild_application_commands<'a>( &'a self, application_id: ApplicationId, guild_id: GuildId, commands: &'a [CreateApplicationCommand], ) -> impl Future<Output = Result<Vec<ApplicationCommand>, Error>> + Send + 'a;
```

##### `get_guild_application_command_permissions`

Calls `GET /applications/{application_id}/guilds/{guild_id}/commands/permissions`.

```rust
pub fn get_guild_application_command_permissions( &self, application_id: ApplicationId, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<GuildCommandPermissions>, Error>> + Send + '_;
```

##### `get_application_command_permissions`

Calls `GET /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions`.

```rust
pub fn get_application_command_permissions( &self, application_id: ApplicationId, guild_id: GuildId, command_id: CommandId, ) -> impl Future<Output = Result<GuildCommandPermissions, Error>> + Send + '_;
```

##### `edit_application_command_permissions`

Calls `PUT /applications/{application_id}/guilds/{guild_id}/commands/{command_id}/permissions`.

```rust
pub fn edit_application_command_permissions<'a>( &'a self, application_id: ApplicationId, guild_id: GuildId, command_id: CommandId, body: &'a EditApplicationCommandPermissions, ) -> impl Future<Output = Result<GuildCommandPermissions, Error>> + Send + 'a;
```

##### `bulk_overwrite_guild_command_permissions`

Calls `PUT /applications/{application_id}/guilds/{guild_id}/commands/permissions`.

```rust
pub fn bulk_overwrite_guild_command_permissions<'a>( &'a self, application_id: ApplicationId, guild_id: GuildId, body: &'a [GuildCommandPermissions], ) -> impl Future<Output = Result<Vec<GuildCommandPermissions>, Error>> + Send + 'a;
```

## `rest.endpoints.application_identity_profile`

### `Rest`

##### `update_application_identity_profile`

Calls `PATCH /applications/{application_id}/users/{user_id}/identities/{provider_issued_user_id}/profile`.

```rust
pub fn update_application_identity_profile<'a>( &'a self, application_id: ApplicationId, user_id: UserId, provider_issued_user_id: &'a str, body: &'a UpdateApplicationIdentityProfile, ) -> impl Future<Output = Result<Option<ApplicationIdentityProfile>, Error>> + Send + 'a;
```

##### `get_application_identity_profile`

Calls `GET /applications/{application_id}/users/{user_id}/identities/{provider_issued_user_id}/profile`.

```rust
pub fn get_application_identity_profile<'a>( &'a self, application_id: ApplicationId, user_id: UserId, provider_issued_user_id: &'a str, ) -> impl Future<Output = Result<ApplicationIdentityProfile, Error>> + Send + 'a;
```

##### `get_application_identities_by_user_id`

Calls `GET /users/{user_id}/application-identities/{application_id}`.

```rust
pub fn get_application_identities_by_user_id( &self, user_id: UserId, application_id: ApplicationId, ) -> impl Future<Output = Result<Vec<ApplicationIdentity>, Error>> + Send + '_;
```

##### `get_application_identities_by_external_id`

Calls `GET /applications/{application_id}/application-identities/{provider_type}/{provider_issued_user_id}`.

```rust
pub fn get_application_identities_by_external_id<'a>( &'a self, application_id: ApplicationId, provider_type: &'a str, provider_issued_user_id: &'a str, query: &'a GetApplicationIdentitiesByExternalId, ) -> impl Future<Output = Result<Vec<ApplicationIdentity>, Error>> + Send + 'a;
```

##### `delete_application_identity`

Calls `POST /users/{user_id}/application-identities/{application_id}/{provider_type}/{provider_issued_user_id}/delete`.

```rust
pub fn delete_application_identity<'a>( &'a self, user_id: UserId, application_id: ApplicationId, provider_type: &'a str, provider_issued_user_id: &'a str, body: &'a DeleteApplicationIdentity, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.application_role_connection_metadata`

### `Rest`

##### `get_application_role_connection_metadata_records`

Calls `PUT /applications/{application_id}/role-connections/metadata`.

```rust
pub fn get_application_role_connection_metadata_records( &self, application_id: ApplicationId, ) -> impl Future<Output = Result<Vec<ApplicationRoleConnectionMetadata>, Error>> + Send + '_;
```

##### `update_application_role_connection_metadata_records`

Calls `PUT /applications/{application_id}/role-connections/metadata`.

```rust
pub fn update_application_role_connection_metadata_records<'a>( &'a self, application_id: ApplicationId, records: &'a [ApplicationRoleConnectionMetadata], ) -> impl Future<Output = Result<Vec<ApplicationRoleConnectionMetadata>, Error>> + Send + 'a;
```

## `rest.endpoints.audit_log`

### `Rest`

##### `get_guild_audit_log`

Calls `GET /guilds/{guild_id}/audit-logs`. Reads the audit log with action, user and pagination filters.

```rust
pub fn get_guild_audit_log<'a>( &'a self, guild_id: GuildId, query: &'a GetGuildAuditLog, ) -> impl Future<Output = Result<AuditLog, Error>> + Send + 'a;
```

## `rest.endpoints.auto_moderation`

### `Rest`

##### `list_auto_moderation_rules_for_guild`

Calls `GET /guilds/{guild_id}/auto-moderation/rules`.

```rust
pub fn list_auto_moderation_rules_for_guild( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Rule>, Error>> + Send + '_;
```

##### `get_auto_moderation_rule`

Calls `GET /guilds/{guild_id}/auto-moderation/rules/{rule_id}`.

```rust
pub fn get_auto_moderation_rule( &self, guild_id: GuildId, rule_id: AutoModerationRuleId, ) -> impl Future<Output = Result<Rule, Error>> + Send + '_;
```

##### `create_auto_moderation_rule`

Calls `POST /guilds/{guild_id}/auto-moderation/rules`. Accepts an audit-log reason.

```rust
pub fn create_auto_moderation_rule<'a>( &'a self, guild_id: GuildId, body: &'a CreateAutoModerationRule, reason: Option<&'a str>, ) -> impl Future<Output = Result<Rule, Error>> + Send + 'a;
```

##### `modify_auto_moderation_rule`

Calls `PATCH /guilds/{guild_id}/auto-moderation/rules/{rule_id}`. Accepts an audit-log reason.

```rust
pub fn modify_auto_moderation_rule<'a>( &'a self, guild_id: GuildId, rule_id: AutoModerationRuleId, body: &'a ModifyAutoModerationRule, reason: Option<&'a str>, ) -> impl Future<Output = Result<Rule, Error>> + Send + 'a;
```

##### `delete_auto_moderation_rule`

Calls `DELETE /guilds/{guild_id}/auto-moderation/rules/{rule_id}`. Accepts an audit-log reason.

```rust
pub fn delete_auto_moderation_rule<'a>( &'a self, guild_id: GuildId, rule_id: AutoModerationRuleId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.channel`

### `Rest`

##### `get_channel`

Calls `GET /channels/{channel_id}`.

```rust
pub fn get_channel( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<Channel, Error>> + Send + '_;
```

##### `modify_channel`

Calls `PATCH /channels/{channel_id}`. Accepts an audit-log reason.

```rust
pub fn modify_channel<'a>( &'a self, channel_id: ChannelId, body: &'a ModifyGuildChannel, reason: Option<&'a str>, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `modify_thread`

Calls `PATCH /channels/{channel_id}`. Accepts an audit-log reason.

```rust
pub fn modify_thread<'a>( &'a self, channel_id: ChannelId, body: &'a ModifyThread, reason: Option<&'a str>, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `modify_group_dm`

Calls `PATCH /channels/{channel_id}`.

```rust
pub fn modify_group_dm<'a>( &'a self, channel_id: ChannelId, body: &'a ModifyGroupDm, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `delete_channel`

Calls `DELETE /channels/{channel_id}`. Accepts an audit-log reason. Deletes a channel. Threads and DMs close the same way.

```rust
pub fn delete_channel<'a>( &'a self, channel_id: ChannelId, reason: Option<&'a str>, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `get_channel_member_permissions`

Calls `GET /channels/{channel_id}/permissions/member/{user_id}`.

```rust
pub fn get_channel_member_permissions<'a>( &'a self, channel_id: ChannelId, user_id: UserId, query: &'a GetChannelPermissions, ) -> impl Future<Output = Result<Permissions, Error>> + Send + 'a;
```

##### `get_channel_role_permissions`

Calls `GET /channels/{channel_id}/permissions/role/{role_id}`.

```rust
pub fn get_channel_role_permissions<'a>( &'a self, channel_id: ChannelId, role_id: RoleId, query: &'a GetChannelPermissions, ) -> impl Future<Output = Result<Permissions, Error>> + Send + 'a;
```

##### `set_voice_channel_status`

Calls `PUT /channels/{channel_id}/voice-status`. Accepts an audit-log reason.

```rust
pub fn set_voice_channel_status<'a>( &'a self, channel_id: ChannelId, body: &'a SetVoiceChannelStatus, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `edit_channel_permissions`

Calls `PUT /channels/{channel_id}/permissions/{overwrite_id}`. Accepts an audit-log reason.

```rust
pub fn edit_channel_permissions<'a>( &'a self, channel_id: ChannelId, overwrite_id: GenericId, body: &'a EditChannelPermissions, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `delete_channel_permission`

Calls `DELETE /channels/{channel_id}/permissions/{overwrite_id}`. Accepts an audit-log reason.

```rust
pub fn delete_channel_permission<'a>( &'a self, channel_id: ChannelId, overwrite_id: GenericId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_channel_invites`

Calls `GET /channels/{channel_id}/invites`.

```rust
pub fn get_channel_invites( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<Vec<InviteWithMetadata>, Error>> + Send + '_;
```

##### `create_channel_invite`

Calls `POST /channels/{channel_id}/invites`. Accepts an audit-log reason.

```rust
pub fn create_channel_invite<'a>( &'a self, channel_id: ChannelId, body: &'a CreateChannelInvite, reason: Option<&'a str>, ) -> impl Future<Output = Result<InviteWithMetadata, Error>> + Send + 'a;
```

##### `follow_announcement_channel`

Calls `POST /channels/{channel_id}/followers`. Accepts an audit-log reason.

```rust
pub fn follow_announcement_channel<'a>( &'a self, channel_id: ChannelId, body: &'a FollowAnnouncementChannel, reason: Option<&'a str>, ) -> impl Future<Output = Result<FollowedChannel, Error>> + Send + 'a;
```

##### `trigger_typing_indicator`

Calls `POST /channels/{channel_id}/typing`.

```rust
pub fn trigger_typing_indicator( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `group_dm_add_recipient`

Calls `PUT /channels/{channel_id}/recipients/{user_id}`.

```rust
pub fn group_dm_add_recipient<'a>( &'a self, channel_id: ChannelId, user_id: UserId, body: &'a GroupDmAddRecipient, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `group_dm_remove_recipient`

Calls `DELETE /channels/{channel_id}/recipients/{user_id}`.

```rust
pub fn group_dm_remove_recipient( &self, channel_id: ChannelId, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `start_thread_from_message`

Calls `POST /channels/{channel_id}/messages/{message_id}/threads`. Accepts an audit-log reason.

```rust
pub fn start_thread_from_message<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, body: &'a StartThreadFromMessage, reason: Option<&'a str>, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `start_thread_without_message`

Calls `POST /channels/{channel_id}/threads`. Accepts an audit-log reason.

```rust
pub fn start_thread_without_message<'a>( &'a self, channel_id: ChannelId, body: &'a StartThreadWithoutMessage, reason: Option<&'a str>, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `start_thread_in_forum_or_media_channel`

Calls `POST /channels/{channel_id}/threads`. Accepts an audit-log reason.

```rust
pub fn start_thread_in_forum_or_media_channel<'a>( &'a self, channel_id: ChannelId, body: &'a StartThreadInForumOrMediaChannel, reason: Option<&'a str>, ) -> impl Future<Output = Result<ForumThread, Error>> + Send + 'a;
```

##### `join_thread`

Calls `PUT /channels/{channel_id}/thread-members/@me`.

```rust
pub fn join_thread( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `add_thread_member`

Calls `PUT /channels/{channel_id}/thread-members/{user_id}`.

```rust
pub fn add_thread_member( &self, channel_id: ChannelId, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `leave_thread`

Calls `DELETE /channels/{channel_id}/thread-members/@me`.

```rust
pub fn leave_thread( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `remove_thread_member`

Calls `DELETE /channels/{channel_id}/thread-members/{user_id}`.

```rust
pub fn remove_thread_member( &self, channel_id: ChannelId, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `get_thread_member`

Calls `GET /channels/{channel_id}/thread-members/{user_id}`.

```rust
pub fn get_thread_member<'a>( &'a self, channel_id: ChannelId, user_id: UserId, query: &'a GetThreadMember, ) -> impl Future<Output = Result<ThreadMember, Error>> + Send + 'a;
```

##### `list_thread_members`

Calls `GET /channels/{channel_id}/thread-members`.

```rust
pub fn list_thread_members<'a>( &'a self, channel_id: ChannelId, query: &'a ListThreadMembers, ) -> impl Future<Output = Result<Vec<ThreadMember>, Error>> + Send + 'a;
```

##### `list_public_archived_threads`

Calls `GET /channels/{channel_id}/threads/archived/public`.

```rust
pub fn list_public_archived_threads<'a>( &'a self, channel_id: ChannelId, query: &'a ListArchivedThreads, ) -> impl Future<Output = Result<ArchivedThreads, Error>> + Send + 'a;
```

##### `list_private_archived_threads`

Calls `GET /channels/{channel_id}/threads/archived/private`.

```rust
pub fn list_private_archived_threads<'a>( &'a self, channel_id: ChannelId, query: &'a ListArchivedThreads, ) -> impl Future<Output = Result<ArchivedThreads, Error>> + Send + 'a;
```

##### `list_joined_private_archived_threads`

Calls `GET /channels/{channel_id}/users/@me/threads/archived/private`.

```rust
pub fn list_joined_private_archived_threads<'a>( &'a self, channel_id: ChannelId, query: &'a ListJoinedPrivateArchivedThreads, ) -> impl Future<Output = Result<ArchivedThreads, Error>> + Send + 'a;
```

## `rest.endpoints.emoji`

### `Rest`

##### `list_guild_emojis`

Calls `GET /guilds/{guild_id}/emojis`.

```rust
pub fn list_guild_emojis( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Emoji>, Error>> + Send + '_;
```

##### `get_guild_emoji`

Calls `GET /guilds/{guild_id}/emojis/{emoji_id}`.

```rust
pub fn get_guild_emoji( &self, guild_id: GuildId, emoji_id: EmojiId, ) -> impl Future<Output = Result<Emoji, Error>> + Send + '_;
```

##### `create_guild_emoji`

Calls `POST /guilds/{guild_id}/emojis`. Accepts an audit-log reason.

```rust
pub fn create_guild_emoji<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildEmoji, reason: Option<&'a str>, ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a;
```

##### `modify_guild_emoji`

Calls `PATCH /guilds/{guild_id}/emojis/{emoji_id}`. Accepts an audit-log reason.

```rust
pub fn modify_guild_emoji<'a>( &'a self, guild_id: GuildId, emoji_id: EmojiId, body: &'a ModifyGuildEmoji, reason: Option<&'a str>, ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a;
```

##### `delete_guild_emoji`

Calls `DELETE /guilds/{guild_id}/emojis/{emoji_id}`. Accepts an audit-log reason.

```rust
pub fn delete_guild_emoji<'a>( &'a self, guild_id: GuildId, emoji_id: EmojiId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `list_application_emojis`

Calls `GET /applications/{application_id}/emojis`.

```rust
pub fn list_application_emojis( &self, application_id: ApplicationId, ) -> impl Future<Output = Result<Vec<Emoji>, Error>> + Send + '_;
```

##### `get_application_emoji`

Calls `GET /applications/{application_id}/emojis/{emoji_id}`.

```rust
pub fn get_application_emoji( &self, application_id: ApplicationId, emoji_id: EmojiId, ) -> impl Future<Output = Result<Emoji, Error>> + Send + '_;
```

##### `create_application_emoji`

Calls `POST /applications/{application_id}/emojis`.

```rust
pub fn create_application_emoji<'a>( &'a self, application_id: ApplicationId, body: &'a CreateApplicationEmoji, ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a;
```

##### `modify_application_emoji`

Calls `PATCH /applications/{application_id}/emojis/{emoji_id}`.

```rust
pub fn modify_application_emoji<'a>( &'a self, application_id: ApplicationId, emoji_id: EmojiId, body: &'a ModifyApplicationEmoji, ) -> impl Future<Output = Result<Emoji, Error>> + Send + 'a;
```

##### `delete_application_emoji`

Calls `DELETE /applications/{application_id}/emojis/{emoji_id}`.

```rust
pub fn delete_application_emoji( &self, application_id: ApplicationId, emoji_id: EmojiId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

## `rest.endpoints.entitlement`

### `Rest`

##### `list_entitlements`

Calls `GET /applications/{application_id}/entitlements`.

```rust
pub fn list_entitlements<'a>( &'a self, application_id: ApplicationId, query: &'a ListEntitlements, ) -> impl Future<Output = Result<Vec<Entitlement>, Error>> + Send + 'a;
```

##### `get_entitlement`

Calls `GET /applications/{application_id}/entitlements/{entitlement_id}`.

```rust
pub fn get_entitlement( &self, application_id: ApplicationId, entitlement_id: EntitlementId, ) -> impl Future<Output = Result<Entitlement, Error>> + Send + '_;
```

##### `consume_entitlement`

Calls `POST /applications/{application_id}/entitlements/{entitlement_id}/consume`.

```rust
pub fn consume_entitlement( &self, application_id: ApplicationId, entitlement_id: EntitlementId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `create_test_entitlement`

Calls `POST /applications/{application_id}/entitlements`.

```rust
pub fn create_test_entitlement<'a>( &'a self, application_id: ApplicationId, body: &'a CreateTestEntitlement, ) -> impl Future<Output = Result<Entitlement, Error>> + Send + 'a;
```

##### `delete_test_entitlement`

Calls `DELETE /applications/{application_id}/entitlements/{entitlement_id}`.

```rust
pub fn delete_test_entitlement( &self, application_id: ApplicationId, entitlement_id: EntitlementId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

## `rest.endpoints.guild`

### `Rest`

##### `get_guild`

Returns guild.

```rust
pub fn get_guild<'a>( &'a self, guild_id: GuildId, query: &'a GetGuild, ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a;
```

##### `get_guild_preview`

Calls `GET /guilds/{guild_id}/preview`.

```rust
pub fn get_guild_preview( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Preview, Error>> + Send + '_;
```

##### `modify_guild`

Calls `PATCH /guilds/{guild_id}`. Accepts an audit-log reason.

```rust
pub fn modify_guild<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuild, reason: Option<&'a str>, ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a;
```

##### `create_guild`

Creates guild.

```rust
pub fn create_guild<'a>( &'a self, body: &'a CreateGuild, ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a;
```

##### `delete_guild`

Calls `DELETE /guilds/{guild_id}`.

```rust
pub fn delete_guild( &self, guild_id: GuildId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `get_guild_member_verification`

Calls `GET /guilds/{guild_id}/member-verification`.

```rust
pub fn get_guild_member_verification( &self, guild_id: GuildId, ) -> impl Future<Output = Result<GuildMembershipScreening, Error>> + Send + '_;
```

##### `modify_guild_member_verification`

Calls `PATCH /guilds/{guild_id}/member-verification`. Accepts an audit-log reason.

```rust
pub fn modify_guild_member_verification<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuildMembershipScreening, reason: Option<&'a str>, ) -> impl Future<Output = Result<GuildMembershipScreening, Error>> + Send + 'a;
```

##### `modify_guild_mfa_level`

Calls `POST /guilds/{guild_id}/mfa`. Accepts an audit-log reason.

```rust
pub fn modify_guild_mfa_level<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuildMfaLevel, reason: Option<&'a str>, ) -> impl Future<Output = Result<MfaLevel, Error>> + Send + 'a;
```

##### `get_guild_channels`

Calls `GET /guilds/{guild_id}/channels`.

```rust
pub fn get_guild_channels( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Channel>, Error>> + Send + '_;
```

##### `create_guild_channel`

Calls `POST /guilds/{guild_id}/channels`. Accepts an audit-log reason.

```rust
pub fn create_guild_channel<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildChannel, reason: Option<&'a str>, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `modify_guild_channel_positions`

Calls `PATCH /guilds/{guild_id}/channels`. Accepts an audit-log reason.

```rust
pub fn modify_guild_channel_positions<'a>( &'a self, guild_id: GuildId, positions: &'a [ModifyGuildChannelPosition], reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `list_active_guild_threads`

Calls `GET /guilds/{guild_id}/threads/active`.

```rust
pub fn list_active_guild_threads( &self, guild_id: GuildId, ) -> impl Future<Output = Result<ActiveThreads, Error>> + Send + '_;
```

##### `get_guild_member`

Calls `GET /guilds/{guild_id}/member-verification`.

```rust
pub fn get_guild_member( &self, guild_id: GuildId, user_id: UserId, ) -> impl Future<Output = Result<Member, Error>> + Send + '_;
```

##### `list_guild_members`

Calls `GET /guilds/{guild_id}/members`. Pages through members; the cache fills as pages arrive.

```rust
pub fn list_guild_members<'a>( &'a self, guild_id: GuildId, query: &'a ListGuildMembers, ) -> impl Future<Output = Result<Vec<Member>, Error>> + Send + 'a;
```

##### `search_guild_members`

Calls `GET /guilds/{guild_id}/members/search`. Searches by username prefix. Needs the members intent to be useful.

```rust
pub fn search_guild_members<'a>( &'a self, guild_id: GuildId, query: &'a SearchGuildMembers, ) -> impl Future<Output = Result<Vec<Member>, Error>> + Send + 'a;
```

##### `get_guild_member_permissions`

Calls `GET /guilds/{guild_id}/permissions/member/{user_id}`.

```rust
pub fn get_guild_member_permissions( &self, guild_id: GuildId, user_id: UserId, ) -> impl Future<Output = Result<Permissions, Error>> + Send + '_;
```

##### `add_guild_member`

Calls `PUT /guilds/{guild_id}/members/{user_id}`.

```rust
pub fn add_guild_member<'a>( &'a self, guild_id: GuildId, user_id: UserId, body: &'a AddGuildMember, ) -> impl Future<Output = Result<Option<Member>, Error>> + Send + 'a;
```

##### `modify_guild_member`

Calls `PATCH /guilds/{guild_id}/member-verification`. Accepts an audit-log reason.

```rust
pub fn modify_guild_member<'a>( &'a self, guild_id: GuildId, user_id: UserId, body: &'a ModifyGuildMember, reason: Option<&'a str>, ) -> impl Future<Output = Result<Member, Error>> + Send + 'a;
```

##### `modify_current_member`

Calls `PATCH /guilds/{guild_id}/members/@me`. Accepts an audit-log reason.

```rust
pub fn modify_current_member<'a>( &'a self, guild_id: GuildId, body: &'a ModifyCurrentMember, reason: Option<&'a str>, ) -> impl Future<Output = Result<Member, Error>> + Send + 'a;
```

##### `modify_current_member_nickname`

Calls `PATCH /guilds/{guild_id}/members/@me/nick`. Accepts an audit-log reason.

```rust
pub fn modify_current_member_nickname<'a>( &'a self, guild_id: GuildId, body: &'a ModifyCurrentMemberNickname, reason: Option<&'a str>, ) -> impl Future<Output = Result<ModifyCurrentMemberNickname, Error>> + Send + 'a;
```

##### `add_guild_member_role`

Calls `PUT /guilds/{guild_id}/members/{user_id}/roles/{role_id}`. Accepts an audit-log reason.

```rust
pub fn add_guild_member_role<'a>( &'a self, guild_id: GuildId, user_id: UserId, role_id: RoleId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `remove_guild_member_role`

Calls `DELETE /guilds/{guild_id}/members/{user_id}/roles/{role_id}`. Accepts an audit-log reason.

```rust
pub fn remove_guild_member_role<'a>( &'a self, guild_id: GuildId, user_id: UserId, role_id: RoleId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `remove_guild_member`

Calls `DELETE /guilds/{guild_id}/members/{user_id}/roles/{role_id}`. Accepts an audit-log reason.

```rust
pub fn remove_guild_member<'a>( &'a self, guild_id: GuildId, user_id: UserId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_guild_bans`

Calls `GET /guilds/{guild_id}/bans`.

```rust
pub fn get_guild_bans<'a>( &'a self, guild_id: GuildId, query: &'a GetGuildBans, ) -> impl Future<Output = Result<Vec<Ban>, Error>> + Send + 'a;
```

##### `get_guild_ban`

Calls `GET /guilds/{guild_id}/bans`.

```rust
pub fn get_guild_ban( &self, guild_id: GuildId, user_id: UserId, ) -> impl Future<Output = Result<Ban, Error>> + Send + '_;
```

##### `create_guild_ban`

Calls `PUT /guilds/{guild_id}/bans/{user_id}`. Accepts an audit-log reason. Bans a user, optionally deleting recent messages.

```rust
pub fn create_guild_ban<'a>( &'a self, guild_id: GuildId, user_id: UserId, body: &'a CreateGuildBan, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `remove_guild_ban`

Calls `DELETE /guilds/{guild_id}/bans/{user_id}`. Accepts an audit-log reason.

```rust
pub fn remove_guild_ban<'a>( &'a self, guild_id: GuildId, user_id: UserId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `bulk_guild_ban`

Calls `POST /guilds/{guild_id}/bulk-ban`. Accepts an audit-log reason. Bans many users in one call and reports who failed.

```rust
pub fn bulk_guild_ban<'a>( &'a self, guild_id: GuildId, body: &'a BulkGuildBan, reason: Option<&'a str>, ) -> impl Future<Output = Result<BulkBanResponse, Error>> + Send + 'a;
```

##### `get_guild_roles`

Calls `GET /guilds/{guild_id}/roles`.

```rust
pub fn get_guild_roles( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Role>, Error>> + Send + '_;
```

##### `get_guild_role`

Calls `GET /guilds/{guild_id}/roles`.

```rust
pub fn get_guild_role( &self, guild_id: GuildId, role_id: RoleId, ) -> impl Future<Output = Result<Role, Error>> + Send + '_;
```

##### `get_guild_role_permissions`

Calls `GET /guilds/{guild_id}/permissions/role/{role_id}`.

```rust
pub fn get_guild_role_permissions( &self, guild_id: GuildId, role_id: RoleId, ) -> impl Future<Output = Result<Permissions, Error>> + Send + '_;
```

##### `get_guild_role_member_counts`

Calls `GET /guilds/{guild_id}/roles/member-counts`.

```rust
pub fn get_guild_role_member_counts( &self, guild_id: GuildId, ) -> impl Future<Output = Result<HashMap<RoleId, u32>, Error>> + Send + '_;
```

##### `create_guild_role`

Calls `POST /guilds/{guild_id}/roles`. Accepts an audit-log reason.

```rust
pub fn create_guild_role<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildRole, reason: Option<&'a str>, ) -> impl Future<Output = Result<Role, Error>> + Send + 'a;
```

##### `modify_guild_role_positions`

Calls `PATCH /guilds/{guild_id}/roles`. Accepts an audit-log reason.

```rust
pub fn modify_guild_role_positions<'a>( &'a self, guild_id: GuildId, positions: &'a [ModifyGuildRolePosition], reason: Option<&'a str>, ) -> impl Future<Output = Result<Vec<Role>, Error>> + Send + 'a;
```

##### `modify_guild_role`

Calls `PATCH /guilds/{guild_id}/roles`. Accepts an audit-log reason.

```rust
pub fn modify_guild_role<'a>( &'a self, guild_id: GuildId, role_id: RoleId, body: &'a ModifyGuildRole, reason: Option<&'a str>, ) -> impl Future<Output = Result<Role, Error>> + Send + 'a;
```

##### `delete_guild_role`

Calls `DELETE /guilds/{guild_id}/roles/{role_id}`. Accepts an audit-log reason.

```rust
pub fn delete_guild_role<'a>( &'a self, guild_id: GuildId, role_id: RoleId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_guild_prune_count`

Calls `GET /guilds/{guild_id}/prune`.

```rust
pub fn get_guild_prune_count<'a>( &'a self, guild_id: GuildId, query: &'a GetGuildPruneCount, ) -> impl Future<Output = Result<u32, Error>> + Send + 'a;
```

##### `begin_guild_prune`

Calls `POST /guilds/{guild_id}/prune`. Accepts an audit-log reason. Counts (and optionally kicks) members inactive for the given days.

```rust
pub fn begin_guild_prune<'a>( &'a self, guild_id: GuildId, body: &'a BeginGuildPrune, reason: Option<&'a str>, ) -> impl Future<Output = Result<Option<u32>, Error>> + Send + 'a;
```

##### `get_guild_voice_regions`

Calls `GET /guilds/{guild_id}/regions`.

```rust
pub fn get_guild_voice_regions( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<VoiceRegion>, Error>> + Send + '_;
```

##### `get_guild_invites`

Calls `GET /guilds/{guild_id}/invites`.

```rust
pub fn get_guild_invites( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<InviteWithMetadata>, Error>> + Send + '_;
```

##### `get_guild_integrations`

Calls `GET /guilds/{guild_id}/integrations`.

```rust
pub fn get_guild_integrations( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Integration>, Error>> + Send + '_;
```

##### `delete_guild_integration`

Calls `DELETE /guilds/{guild_id}/integrations/{integration_id}`. Accepts an audit-log reason.

```rust
pub fn delete_guild_integration<'a>( &'a self, guild_id: GuildId, integration_id: IntegrationId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_guild_widget_settings`

Calls `GET /guilds/{guild_id}/widget`.

```rust
pub fn get_guild_widget_settings( &self, guild_id: GuildId, ) -> impl Future<Output = Result<WidgetSettings, Error>> + Send + '_;
```

##### `modify_guild_widget`

Calls `PATCH /guilds/{guild_id}/widget`. Accepts an audit-log reason.

```rust
pub fn modify_guild_widget<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuildWidget, reason: Option<&'a str>, ) -> impl Future<Output = Result<WidgetSettings, Error>> + Send + 'a;
```

##### `get_guild_widget`

Calls `GET /guilds/{guild_id}/widget`.

```rust
pub fn get_guild_widget( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Widget, Error>> + Send + '_;
```

##### `get_guild_vanity_url`

Calls `GET /guilds/{guild_id}/vanity-url`.

```rust
pub fn get_guild_vanity_url( &self, guild_id: GuildId, ) -> impl Future<Output = Result<VanityUrl, Error>> + Send + '_;
```

##### `get_guild_welcome_screen`

Calls `GET /guilds/{guild_id}/welcome-screen`.

```rust
pub fn get_guild_welcome_screen( &self, guild_id: GuildId, ) -> impl Future<Output = Result<WelcomeScreen, Error>> + Send + '_;
```

##### `modify_guild_welcome_screen`

Calls `PATCH /guilds/{guild_id}/welcome-screen`. Accepts an audit-log reason.

```rust
pub fn modify_guild_welcome_screen<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuildWelcomeScreen, reason: Option<&'a str>, ) -> impl Future<Output = Result<WelcomeScreen, Error>> + Send + 'a;
```

##### `get_guild_onboarding`

Calls `GET /guilds/{guild_id}/onboarding`.

```rust
pub fn get_guild_onboarding( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Onboarding, Error>> + Send + '_;
```

##### `modify_guild_onboarding`

Calls `PUT /guilds/{guild_id}/onboarding`. Accepts an audit-log reason.

```rust
pub fn modify_guild_onboarding<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuildOnboarding, reason: Option<&'a str>, ) -> impl Future<Output = Result<Onboarding, Error>> + Send + 'a;
```

##### `modify_guild_incident_actions`

Calls `PUT /guilds/{guild_id}/incident-actions`.

```rust
pub fn modify_guild_incident_actions<'a>( &'a self, guild_id: GuildId, body: &'a ModifyGuildIncidentActions, ) -> impl Future<Output = Result<IncidentsData, Error>> + Send + 'a;
```

## `rest.endpoints.guild_scheduled_event`

### `Rest`

##### `list_scheduled_events_for_guild`

Calls `GET /guilds/{guild_id}/scheduled-events`.

```rust
pub fn list_scheduled_events_for_guild<'a>( &'a self, guild_id: GuildId, query: &'a ListScheduledEventsForGuild, ) -> impl Future<Output = Result<Vec<GuildScheduledEvent>, Error>> + Send + 'a;
```

##### `create_guild_scheduled_event`

Calls `POST /guilds/{guild_id}/scheduled-events`. Accepts an audit-log reason.

```rust
pub fn create_guild_scheduled_event<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildScheduledEvent, reason: Option<&'a str>, ) -> impl Future<Output = Result<GuildScheduledEvent, Error>> + Send + 'a;
```

##### `get_guild_scheduled_event`

Calls `GET /guilds/{guild_id}/scheduled-events/{event_id}`.

```rust
pub fn get_guild_scheduled_event<'a>( &'a self, guild_id: GuildId, event_id: ScheduledEventId, query: &'a GetGuildScheduledEvent, ) -> impl Future<Output = Result<GuildScheduledEvent, Error>> + Send + 'a;
```

##### `modify_guild_scheduled_event`

Calls `PATCH /guilds/{guild_id}/scheduled-events/{event_id}`. Accepts an audit-log reason.

```rust
pub fn modify_guild_scheduled_event<'a>( &'a self, guild_id: GuildId, event_id: ScheduledEventId, body: &'a ModifyGuildScheduledEvent, reason: Option<&'a str>, ) -> impl Future<Output = Result<GuildScheduledEvent, Error>> + Send + 'a;
```

##### `delete_guild_scheduled_event`

Calls `DELETE /guilds/{guild_id}/scheduled-events/{event_id}`.

```rust
pub fn delete_guild_scheduled_event( &self, guild_id: GuildId, event_id: ScheduledEventId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `get_guild_scheduled_event_users`

Calls `GET /guilds/{guild_id}/scheduled-events/{event_id}/users`.

```rust
pub fn get_guild_scheduled_event_users<'a>( &'a self, guild_id: GuildId, event_id: ScheduledEventId, query: &'a GetGuildScheduledEventUsers, ) -> impl Future<Output = Result<Vec<GuildScheduledEventUser>, Error>> + Send + 'a;
```

## `rest.endpoints.guild_template`

### `Rest`

##### `get_guild_template`

Calls `GET /guilds/templates/{template_code}`.

```rust
pub fn get_guild_template<'a>( &'a self, template_code: &'a str, ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a;
```

##### `get_guild_templates`

Calls `GET /guilds/{guild_id}/templates`.

```rust
pub fn get_guild_templates( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<GuildTemplate>, Error>> + Send + '_;
```

##### `create_guild_template`

Calls `POST /guilds/{guild_id}/templates`.

```rust
pub fn create_guild_template<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildTemplate, ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a;
```

##### `create_guild_from_template`

Calls `POST /guilds/templates/{template_code}`.

```rust
pub fn create_guild_from_template<'a>( &'a self, template_code: &'a str, body: &'a CreateGuildFromTemplate, ) -> impl Future<Output = Result<Guild, Error>> + Send + 'a;
```

##### `sync_guild_template`

Calls `PUT /guilds/{guild_id}/templates/{template_code}`.

```rust
pub fn sync_guild_template<'a>( &'a self, guild_id: GuildId, template_code: &'a str, ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a;
```

##### `modify_guild_template`

Calls `PATCH /guilds/{guild_id}/templates/{template_code}`.

```rust
pub fn modify_guild_template<'a>( &'a self, guild_id: GuildId, template_code: &'a str, body: &'a ModifyGuildTemplate, ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a;
```

##### `delete_guild_template`

Calls `DELETE /guilds/{guild_id}/templates/{template_code}`.

```rust
pub fn delete_guild_template<'a>( &'a self, guild_id: GuildId, template_code: &'a str, ) -> impl Future<Output = Result<GuildTemplate, Error>> + Send + 'a;
```

## `rest.endpoints.interaction`


### `Rest`

##### `create_interaction_response`

Calls `GET /webhooks/{application_id}/{interaction_token}/messages/@original`. Answers an interaction. Must happen within 3 seconds of receiving it.

```rust
pub fn create_interaction_response<'a>( &'a self, interaction_id: InteractionId, interaction_token: &'a str, response: &'a InteractionResponse, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `create_interaction_response_with_response`

Calls `GET /webhooks/{application_id}/{interaction_token}/messages/@original`.

```rust
pub fn create_interaction_response_with_response<'a>( &'a self, interaction_id: InteractionId, interaction_token: &'a str, response: &'a InteractionResponse, ) -> impl Future<Output = Result<InteractionCallbackResponse, Error>> + Send + 'a;
```

##### `get_original_interaction_response`

Calls `GET /webhooks/{application_id}/{interaction_token}/messages/@original`.

```rust
pub fn get_original_interaction_response<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `edit_original_interaction_response`

Calls `PATCH /webhooks/{application_id}/{interaction_token}/messages/@original`.

```rust
pub fn edit_original_interaction_response<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, body: &'a EditWebhookMessage, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `delete_original_interaction_response`

Calls `DELETE /webhooks/{application_id}/{interaction_token}/messages/@original`.

```rust
pub fn delete_original_interaction_response<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `create_followup_message`

Calls `POST /webhooks/{application_id}/{interaction_token}`. Sends a follow-up after the initial response.

```rust
pub fn create_followup_message<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, body: &'a CreateFollowupMessage, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `get_followup_message`

Calls `GET /webhooks/{application_id}/{interaction_token}/messages/{message_id}`.

```rust
pub fn get_followup_message<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, message_id: MessageId, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `edit_followup_message`

Calls `PATCH /webhooks/{application_id}/{interaction_token}/messages/{message_id}`.

```rust
pub fn edit_followup_message<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, message_id: MessageId, body: &'a EditWebhookMessage, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `delete_followup_message`

Calls `DELETE /webhooks/{application_id}/{interaction_token}/messages/{message_id}`.

```rust
pub fn delete_followup_message<'a>( &'a self, application_id: ApplicationId, interaction_token: &'a str, message_id: MessageId, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.invite`

### `Rest`

##### `get_invite`

Calls `GET /invites/{invite_code}`.

```rust
pub fn get_invite<'a>( &'a self, invite_code: &'a str, query: &'a GetInvite, ) -> impl Future<Output = Result<Invite, Error>> + Send + 'a;
```

##### `delete_invite`

Calls `DELETE /invites/{invite_code}`. Accepts an audit-log reason.

```rust
pub fn delete_invite<'a>( &'a self, invite_code: &'a str, reason: Option<&'a str>, ) -> impl Future<Output = Result<Invite, Error>> + Send + 'a;
```

##### `get_target_users`

Calls `GET /invites/{invite_code}/target-users`.

```rust
pub fn get_target_users<'a>( &'a self, invite_code: &'a str, ) -> impl Future<Output = Result<Vec<UserId>, Error>> + Send + 'a;
```

##### `add_target_user`

Calls `PUT /invites/{invite_code}/target-users/{user_id}`.

```rust
pub fn add_target_user<'a>( &'a self, invite_code: &'a str, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `remove_target_user`

Calls `DELETE /invites/{invite_code}/target-users/{user_id}`.

```rust
pub fn remove_target_user<'a>( &'a self, invite_code: &'a str, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `update_target_users`

Calls `PUT /invites/{invite_code}/target-users`. Replaces the target-user CSV for a scheduled-event invite.

```rust
pub fn update_target_users<'a>( &'a self, invite_code: &'a str, file: &'a FileUpload, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `bulk_add_target_users`

Calls `POST /invites/{invite_code}/target-users/bulk-add`.

```rust
pub fn bulk_add_target_users<'a>( &'a self, invite_code: &'a str, body: &'a BulkTargetUsers, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `bulk_delete_target_users`

Calls `POST /invites/{invite_code}/target-users/bulk-delete`.

```rust
pub fn bulk_delete_target_users<'a>( &'a self, invite_code: &'a str, body: &'a BulkTargetUsers, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_target_users_job_status`

Calls `GET /invites/{invite_code}/target-users/job-status`.

```rust
pub fn get_target_users_job_status<'a>( &'a self, invite_code: &'a str, ) -> impl Future<Output = Result<InviteTargetUsersJobStatus, Error>> + Send + 'a;
```

## `rest.endpoints.lobby`

### `Rest`

##### `create_lobby`

Calls `POST /lobbies`. Creates a social layer lobby.

```rust
pub fn create_lobby<'a>( &'a self, body: &'a CreateLobby, ) -> impl Future<Output = Result<Lobby, Error>> + Send + 'a;
```

##### `create_or_join_lobby`

Calls `PUT /lobbies`. Creates a lobby or joins the matching one.

```rust
pub fn create_or_join_lobby<'a>( &'a self, body: &'a CreateOrJoinLobby, ) -> impl Future<Output = Result<Lobby, Error>> + Send + 'a;
```

##### `get_lobby`

Calls `GET /lobbies/{lobby_id}`.

```rust
pub fn get_lobby( &self, lobby_id: LobbyId, ) -> impl Future<Output = Result<Lobby, Error>> + Send + '_;
```

##### `modify_lobby`

Calls `PATCH /lobbies/{lobby_id}`.

```rust
pub fn modify_lobby<'a>( &'a self, lobby_id: LobbyId, body: &'a ModifyLobby, ) -> impl Future<Output = Result<Lobby, Error>> + Send + 'a;
```

##### `delete_lobby`

Calls `DELETE /lobbies/{lobby_id}`.

```rust
pub fn delete_lobby( &self, lobby_id: LobbyId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `add_lobby_member`

Calls `PUT /lobbies/{lobby_id}/members/{user_id}`.

```rust
pub fn add_lobby_member<'a>( &'a self, lobby_id: LobbyId, user_id: UserId, body: &'a AddLobbyMember, ) -> impl Future<Output = Result<LobbyMember, Error>> + Send + 'a;
```

##### `bulk_update_lobby_members`

Calls `POST /lobbies/{lobby_id}/members/bulk`.

```rust
pub fn bulk_update_lobby_members<'a>( &'a self, lobby_id: LobbyId, members: &'a [BulkLobbyMember], ) -> impl Future<Output = Result<Vec<LobbyMember>, Error>> + Send + 'a;
```

##### `remove_lobby_member`

Calls `DELETE /lobbies/{lobby_id}/members/{user_id}`.

```rust
pub fn remove_lobby_member( &self, lobby_id: LobbyId, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `leave_lobby`

Calls `DELETE /lobbies/{lobby_id}/members/@me`.

```rust
pub fn leave_lobby( &self, lobby_id: LobbyId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `link_channel_to_lobby`

Calls `PATCH /lobbies/{lobby_id}/channel-linking`.

```rust
pub fn link_channel_to_lobby( &self, lobby_id: LobbyId, channel_id: ChannelId, ) -> impl Future<Output = Result<Lobby, Error>> + Send + '_;
```

##### `unlink_channel_from_lobby`

Calls `PATCH /lobbies/{lobby_id}/channel-linking`.

```rust
pub fn unlink_channel_from_lobby( &self, lobby_id: LobbyId, ) -> impl Future<Output = Result<Lobby, Error>> + Send + '_;
```

##### `create_lobby_channel_invite_for_self`

Calls `POST /lobbies/{lobby_id}/members/@me/invites`.

```rust
pub fn create_lobby_channel_invite_for_self( &self, lobby_id: LobbyId, ) -> impl Future<Output = Result<LobbyInvite, Error>> + Send + '_;
```

##### `create_lobby_channel_invite_for_user`

Calls `POST /lobbies/{lobby_id}/members/{user_id}/invites`.

```rust
pub fn create_lobby_channel_invite_for_user( &self, lobby_id: LobbyId, user_id: UserId, ) -> impl Future<Output = Result<LobbyInvite, Error>> + Send + '_;
```

##### `send_lobby_message`

Calls `POST /lobbies/{lobby_id}/messages`.

```rust
pub fn send_lobby_message<'a>( &'a self, lobby_id: LobbyId, body: &'a SendLobbyMessage, ) -> impl Future<Output = Result<LobbyMessage, Error>> + Send + 'a;
```

##### `get_lobby_messages`

Calls `GET /lobbies/{lobby_id}/messages`.

```rust
pub fn get_lobby_messages<'a>( &'a self, lobby_id: LobbyId, query: &'a GetLobbyMessages, ) -> impl Future<Output = Result<Vec<LobbyMessage>, Error>> + Send + 'a;
```

##### `update_lobby_message_moderation_metadata`

Calls `PUT /lobbies/{lobby_id}/messages/{message_id}/moderation-metadata`.

```rust
pub fn update_lobby_message_moderation_metadata<'a>( &'a self, lobby_id: LobbyId, message_id: MessageId, metadata: &'a LobbyMetadata, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.message`

### `Rest`

##### `get_channel_messages`

Calls `GET /channels/{channel_id}/messages`. Reads history directly, no cache involved.

```rust
pub fn get_channel_messages<'a>( &'a self, channel_id: ChannelId, query: &'a GetChannelMessages, ) -> impl Future<Output = Result<Vec<Message>, Error>> + Send + 'a;
```

##### `search_guild_messages`

Calls `GET /guilds/{guild_id}/messages/search`.

```rust
pub fn search_guild_messages<'a>( &'a self, guild_id: GuildId, query: &'a SearchGuildMessages, ) -> impl Future<Output = Result<MessageSearchResults, Error>> + Send + 'a;
```

##### `get_channel_message`

Calls `GET /channels/{channel_id}/messages`.

```rust
pub fn get_channel_message( &self, channel_id: ChannelId, message_id: MessageId, ) -> impl Future<Output = Result<Message, Error>> + Send + '_;
```

##### `create_message`

Calls `POST /channels/{channel_id}/messages`. Sends a message. Threads, polls and attachments ride along in the body.

```rust
pub fn create_message<'a>( &'a self, channel_id: ChannelId, body: &'a CreateMessage, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `crosspost_message`

Calls `POST /channels/{channel_id}/messages/{message_id}/crosspost`.

```rust
pub fn crosspost_message( &self, channel_id: ChannelId, message_id: MessageId, ) -> impl Future<Output = Result<Message, Error>> + Send + '_;
```

##### `create_reaction`

Calls `PUT /channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me`.

```rust
pub fn create_reaction<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, emoji: ReactionEmoji<'a>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `delete_own_reaction`

Calls `DELETE /channels/{channel_id}/messages/{message_id}/reactions/{emoji}/@me`.

```rust
pub fn delete_own_reaction<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, emoji: ReactionEmoji<'a>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `delete_user_reaction`

Calls `DELETE /channels/{channel_id}/messages/{message_id}/reactions/{emoji}/{user_id}`.

```rust
pub fn delete_user_reaction<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, emoji: ReactionEmoji<'a>, user_id: UserId, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_reactions`

Calls `GET /channels/{channel_id}/messages/{message_id}/reactions/{emoji}`.

```rust
pub fn get_reactions<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, emoji: ReactionEmoji<'a>, query: &'a GetReactions, ) -> impl Future<Output = Result<Vec<User>, Error>> + Send + 'a;
```

##### `delete_all_reactions`

Calls `DELETE /channels/{channel_id}/messages/{message_id}/reactions`.

```rust
pub fn delete_all_reactions( &self, channel_id: ChannelId, message_id: MessageId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `delete_all_reactions_for_emoji`

Calls `DELETE /channels/{channel_id}/messages/{message_id}/reactions/{emoji}`.

```rust
pub fn delete_all_reactions_for_emoji<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, emoji: ReactionEmoji<'a>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `edit_message`

Calls `PATCH /channels/{channel_id}/messages/{message_id}`.

```rust
pub fn edit_message<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, body: &'a EditMessage, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `delete_message`

Calls `DELETE /channels/{channel_id}/messages/{message_id}`.

```rust
pub fn delete_message( &self, channel_id: ChannelId, message_id: MessageId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `bulk_delete_messages`

Calls `POST /channels/{channel_id}/messages/bulk-delete`. Accepts an audit-log reason. Deletes up to 100 messages younger than two weeks. Older messages fail.

```rust
pub fn bulk_delete_messages<'a>( &'a self, channel_id: ChannelId, body: &'a BulkDeleteMessages, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_channel_pins`

Calls `GET /channels/{channel_id}/messages/pins`.

```rust
pub fn get_channel_pins<'a>( &'a self, channel_id: ChannelId, query: &'a GetChannelPins, ) -> impl Future<Output = Result<ChannelPins, Error>> + Send + 'a;
```

##### `pin_message`

Calls `PUT /channels/{channel_id}/messages/pins/{message_id}`. Accepts an audit-log reason.

```rust
pub fn pin_message<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `unpin_message`

Calls `DELETE /channels/{channel_id}/messages/pins/{message_id}`. Accepts an audit-log reason.

```rust
pub fn unpin_message<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.oauth2`

### `Rest`

##### `get_current_bot_application_information`

Calls `GET /oauth2/applications/@me`.

```rust
pub fn get_current_bot_application_information( &self, ) -> impl Future<Output = Result<Application, Error>> + Send + '_;
```

##### `get_current_authorization_information`

Calls `GET /oauth2/@me`.

```rust
pub fn get_current_authorization_information( &self, ) -> impl Future<Output = Result<CurrentAuthorizationInformation, Error>> + Send + '_;
```

##### `exchange_code`

Calls `POST /oauth2/token`. Trades an authorization code for tokens during the OAuth2 flow.

```rust
pub fn exchange_code<'a>( &'a self, credentials: &'a OAuth2Credentials, code: &'a str, redirect_uri: &'a str, ) -> impl Future<Output = Result<AccessTokenResponse, Error>> + Send + 'a;
```

##### `refresh_access_token`

Calls `POST /oauth2/token`. Refreshes an expired access token with its refresh token.

```rust
pub fn refresh_access_token<'a>( &'a self, credentials: &'a OAuth2Credentials, refresh_token: &'a str, ) -> impl Future<Output = Result<AccessTokenResponse, Error>> + Send + 'a;
```

##### `client_credentials_grant`

Calls `POST /oauth2/token`. Machine-to-machine tokens without a user.

```rust
pub fn client_credentials_grant<'a>( &'a self, credentials: &'a OAuth2Credentials, scopes: &'a [Scope], ) -> impl Future<Output = Result<AccessTokenResponse, Error>> + Send + 'a;
```

##### `revoke_token`

Calls `POST /oauth2/token/revoke`. Revokes an access or refresh token.

```rust
pub fn revoke_token<'a>( &'a self, credentials: &'a OAuth2Credentials, token: &'a str, hint: Option<TokenTypeHint>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.poll`

### `Rest`

##### `get_answer_voters`

Calls `GET /channels/{channel_id}/polls/{message_id}/answers/{answer_id}`. Lists who voted for an answer, paginated.

```rust
pub fn get_answer_voters<'a>( &'a self, channel_id: ChannelId, message_id: MessageId, answer_id: u32, query: &'a GetAnswerVoters, ) -> impl Future<Output = Result<Vec<User>, Error>> + Send + 'a;
```

##### `end_poll`

Calls `POST /channels/{channel_id}/polls/{message_id}/expire`.

```rust
pub fn end_poll( &self, channel_id: ChannelId, message_id: MessageId, ) -> impl Future<Output = Result<Message, Error>> + Send + '_;
```

## `rest.endpoints.sku`

### `Rest`

##### `list_skus`

Calls `GET /applications/{application_id}/skus`.

```rust
pub fn list_skus( &self, application_id: ApplicationId, ) -> impl Future<Output = Result<Vec<Sku>, Error>> + Send + '_;
```

## `rest.endpoints.soundboard`

### `Rest`

##### `send_soundboard_sound`

Calls `POST /channels/{channel_id}/send-soundboard-sound`.

```rust
pub fn send_soundboard_sound<'a>( &'a self, channel_id: ChannelId, body: &'a SendSoundboardSound, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `list_default_soundboard_sounds`

Calls `GET /soundboard-default-sounds`.

```rust
pub fn list_default_soundboard_sounds( &self, ) -> impl Future<Output = Result<Vec<SoundboardSound>, Error>> + Send + '_;
```

##### `list_guild_soundboard_sounds`

Calls `GET /guilds/{guild_id}/soundboard-sounds`.

```rust
pub fn list_guild_soundboard_sounds( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<SoundboardSound>, Error>> + Send + '_;
```

##### `get_guild_soundboard_sound`

Calls `GET /guilds/{guild_id}/soundboard-sounds/{sound_id}`.

```rust
pub fn get_guild_soundboard_sound( &self, guild_id: GuildId, sound_id: SoundboardSoundId, ) -> impl Future<Output = Result<SoundboardSound, Error>> + Send + '_;
```

##### `create_guild_soundboard_sound`

Calls `POST /guilds/{guild_id}/soundboard-sounds`. Accepts an audit-log reason.

```rust
pub fn create_guild_soundboard_sound<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildSoundboardSound, reason: Option<&'a str>, ) -> impl Future<Output = Result<SoundboardSound, Error>> + Send + 'a;
```

##### `modify_guild_soundboard_sound`

Calls `PATCH /guilds/{guild_id}/soundboard-sounds/{sound_id}`. Accepts an audit-log reason.

```rust
pub fn modify_guild_soundboard_sound<'a>( &'a self, guild_id: GuildId, sound_id: SoundboardSoundId, body: &'a ModifyGuildSoundboardSound, reason: Option<&'a str>, ) -> impl Future<Output = Result<SoundboardSound, Error>> + Send + 'a;
```

##### `delete_guild_soundboard_sound`

Calls `DELETE /guilds/{guild_id}/soundboard-sounds/{sound_id}`. Accepts an audit-log reason.

```rust
pub fn delete_guild_soundboard_sound<'a>( &'a self, guild_id: GuildId, sound_id: SoundboardSoundId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.stage_instance`

### `Rest`

##### `create_stage_instance`

Calls `POST /stage-instances`. Accepts an audit-log reason.

```rust
pub fn create_stage_instance<'a>( &'a self, body: &'a CreateStageInstance, reason: Option<&'a str>, ) -> impl Future<Output = Result<StageInstance, Error>> + Send + 'a;
```

##### `get_stage_instance`

Calls `GET /stage-instances/{channel_id}`.

```rust
pub fn get_stage_instance( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<StageInstance, Error>> + Send + '_;
```

##### `modify_stage_instance`

Calls `PATCH /stage-instances/{channel_id}`. Accepts an audit-log reason.

```rust
pub fn modify_stage_instance<'a>( &'a self, channel_id: ChannelId, body: &'a ModifyStageInstance, reason: Option<&'a str>, ) -> impl Future<Output = Result<StageInstance, Error>> + Send + 'a;
```

##### `delete_stage_instance`

Calls `DELETE /stage-instances/{channel_id}`. Accepts an audit-log reason.

```rust
pub fn delete_stage_instance<'a>( &'a self, channel_id: ChannelId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.sticker`

### `Rest`

##### `get_sticker`

Calls `GET /stickers/{sticker_id}`.

```rust
pub fn get_sticker( &self, sticker_id: StickerId, ) -> impl Future<Output = Result<Sticker, Error>> + Send + '_;
```

##### `list_sticker_packs`

Calls `GET /sticker-packs`.

```rust
pub fn list_sticker_packs( &self, ) -> impl Future<Output = Result<Vec<StickerPack>, Error>> + Send + '_;
```

##### `get_sticker_pack`

Calls `GET /sticker-packs/{pack_id}`.

```rust
pub fn get_sticker_pack( &self, pack_id: StickerPackId, ) -> impl Future<Output = Result<StickerPack, Error>> + Send + '_;
```

##### `list_guild_stickers`

Calls `GET /guilds/{guild_id}/stickers`.

```rust
pub fn list_guild_stickers( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Sticker>, Error>> + Send + '_;
```

##### `get_guild_sticker`

Calls `GET /guilds/{guild_id}/stickers/{sticker_id}`.

```rust
pub fn get_guild_sticker( &self, guild_id: GuildId, sticker_id: StickerId, ) -> impl Future<Output = Result<Sticker, Error>> + Send + '_;
```

##### `create_guild_sticker`

Calls `POST /guilds/{guild_id}/stickers`. Accepts an audit-log reason. Uploads a sticker from a file upload.

```rust
pub fn create_guild_sticker<'a>( &'a self, guild_id: GuildId, body: &'a CreateGuildSticker, reason: Option<&'a str>, ) -> impl Future<Output = Result<Sticker, Error>> + Send + 'a;
```

##### `modify_guild_sticker`

Calls `PATCH /guilds/{guild_id}/stickers/{sticker_id}`. Accepts an audit-log reason.

```rust
pub fn modify_guild_sticker<'a>( &'a self, guild_id: GuildId, sticker_id: StickerId, body: &'a ModifyGuildSticker, reason: Option<&'a str>, ) -> impl Future<Output = Result<Sticker, Error>> + Send + 'a;
```

##### `delete_guild_sticker`

Calls `DELETE /guilds/{guild_id}/stickers/{sticker_id}`. Accepts an audit-log reason.

```rust
pub fn delete_guild_sticker<'a>( &'a self, guild_id: GuildId, sticker_id: StickerId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.subscription`

### `Rest`

##### `list_sku_subscriptions`

Calls `GET /skus/{sku_id}/subscriptions`.

```rust
pub fn list_sku_subscriptions<'a>( &'a self, sku_id: SkuId, query: &'a ListSkuSubscriptions, ) -> impl Future<Output = Result<Vec<Subscription>, Error>> + Send + 'a;
```

##### `get_sku_subscription`

Calls `GET /skus/{sku_id}/subscriptions/{subscription_id}`.

```rust
pub fn get_sku_subscription( &self, sku_id: SkuId, subscription_id: SubscriptionId, ) -> impl Future<Output = Result<Subscription, Error>> + Send + '_;
```

## `rest.endpoints.user`

### `Rest`

##### `get_current_user`

Calls `GET /users/@me`.

```rust
pub fn get_current_user(&self) -> impl Future<Output = Result<CurrentUser, Error>> + Send + '_;
```

##### `get_user`

Calls `GET /users/{user_id}`.

```rust
pub fn get_user( &self, user_id: UserId, ) -> impl Future<Output = Result<User, Error>> + Send + '_;
```

##### `modify_current_user`

Calls `PATCH /users/@me`.

```rust
pub fn modify_current_user<'a>( &'a self, body: &'a ModifyCurrentUser, ) -> impl Future<Output = Result<CurrentUser, Error>> + Send + 'a;
```

##### `get_current_user_guilds`

Calls `GET /users/@me/guilds`.

```rust
pub fn get_current_user_guilds<'a>( &'a self, query: &'a GetCurrentUserGuilds, ) -> impl Future<Output = Result<Vec<CurrentUserGuild>, Error>> + Send + 'a;
```

##### `get_current_user_guild_member`

Calls `GET /users/@me/guilds/{guild_id}/member`.

```rust
pub fn get_current_user_guild_member( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Member, Error>> + Send + '_;
```

##### `leave_guild`

Calls `DELETE /users/@me/guilds/{guild_id}`.

```rust
pub fn leave_guild( &self, guild_id: GuildId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

##### `create_dm`

Calls `POST /users/@me/channels`. Opens (or reuses) a DM channel with a user.

```rust
pub fn create_dm<'a>( &'a self, body: &'a CreateDm, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `create_group_dm`

Calls `POST /users/@me/channels`.

```rust
pub fn create_group_dm<'a>( &'a self, body: &'a CreateGroupDm, ) -> impl Future<Output = Result<Channel, Error>> + Send + 'a;
```

##### `get_current_user_connections`

Calls `GET /users/@me/connections`.

```rust
pub fn get_current_user_connections( &self, ) -> impl Future<Output = Result<Vec<Connection>, Error>> + Send + '_;
```

##### `get_current_user_application_role_connection`

Calls `PUT /users/@me/applications/{application_id}/role-connection`.

```rust
pub fn get_current_user_application_role_connection( &self, application_id: ApplicationId, ) -> impl Future<Output = Result<ApplicationRoleConnection, Error>> + Send + '_;
```

##### `update_current_user_application_role_connection`

Calls `PUT /users/@me/applications/{application_id}/role-connection`.

```rust
pub fn update_current_user_application_role_connection<'a>( &'a self, application_id: ApplicationId, body: &'a ApplicationRoleConnection, ) -> impl Future<Output = Result<ApplicationRoleConnection, Error>> + Send + 'a;
```

##### `delete_current_user_application_role_connection`

Removes current user application role connection.

```rust
pub fn delete_current_user_application_role_connection( &self, application_id: ApplicationId, ) -> impl Future<Output = Result<(), Error>> + Send + '_;
```

## `rest.endpoints.voice`

### `Rest`

##### `list_voice_regions`

Calls `GET /voice/regions`.

```rust
pub fn list_voice_regions( &self, ) -> impl Future<Output = Result<Vec<VoiceRegion>, Error>> + Send + '_;
```

##### `get_current_user_voice_state`

Calls `GET /guilds/{guild_id}/voice-states/@me`.

```rust
pub fn get_current_user_voice_state( &self, guild_id: GuildId, ) -> impl Future<Output = Result<VoiceState, Error>> + Send + '_;
```

##### `get_user_voice_state`

Calls `GET /guilds/{guild_id}/voice-states/{user_id}`.

```rust
pub fn get_user_voice_state( &self, guild_id: GuildId, user_id: UserId, ) -> impl Future<Output = Result<VoiceState, Error>> + Send + '_;
```

##### `modify_current_user_voice_state`

Calls `PATCH /guilds/{guild_id}/voice-states/@me`.

```rust
pub fn modify_current_user_voice_state<'a>( &'a self, guild_id: GuildId, body: &'a ModifyCurrentUserVoiceState, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `modify_user_voice_state`

Calls `PATCH /guilds/{guild_id}/voice-states/{user_id}`.

```rust
pub fn modify_user_voice_state<'a>( &'a self, guild_id: GuildId, user_id: UserId, body: &'a ModifyUserVoiceState, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

## `rest.endpoints.webhook`


### `Rest`

##### `create_webhook`

Calls `POST /channels/{channel_id}/webhooks`. Accepts an audit-log reason.

```rust
pub fn create_webhook<'a>( &'a self, channel_id: ChannelId, body: &'a CreateWebhook, reason: Option<&'a str>, ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a;
```

##### `get_channel_webhooks`

Calls `GET /channels/{channel_id}/webhooks`.

```rust
pub fn get_channel_webhooks( &self, channel_id: ChannelId, ) -> impl Future<Output = Result<Vec<Webhook>, Error>> + Send + '_;
```

##### `get_guild_webhooks`

Calls `GET /guilds/{guild_id}/webhooks`.

```rust
pub fn get_guild_webhooks( &self, guild_id: GuildId, ) -> impl Future<Output = Result<Vec<Webhook>, Error>> + Send + '_;
```

##### `get_webhook`

Calls `GET /webhooks/{webhook_id}`.

```rust
pub fn get_webhook( &self, webhook_id: WebhookId, ) -> impl Future<Output = Result<Webhook, Error>> + Send + '_;
```

##### `get_webhook_with_token`

Calls `GET /webhooks/{webhook_id}/{webhook_token}`.

```rust
pub fn get_webhook_with_token<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a;
```

##### `modify_webhook`

Calls `PATCH /webhooks/{webhook_id}`. Accepts an audit-log reason.

```rust
pub fn modify_webhook<'a>( &'a self, webhook_id: WebhookId, body: &'a ModifyWebhook, reason: Option<&'a str>, ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a;
```

##### `modify_webhook_with_token`

Calls `PATCH /webhooks/{webhook_id}/{webhook_token}`.

```rust
pub fn modify_webhook_with_token<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, body: &'a ModifyWebhook, ) -> impl Future<Output = Result<Webhook, Error>> + Send + 'a;
```

##### `delete_webhook`

Calls `DELETE /webhooks/{webhook_id}`. Accepts an audit-log reason.

```rust
pub fn delete_webhook<'a>( &'a self, webhook_id: WebhookId, reason: Option<&'a str>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `delete_webhook_with_token`

Calls `DELETE /webhooks/{webhook_id}/{webhook_token}`.

```rust
pub fn delete_webhook_with_token<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `execute_webhook`

Calls `POST /webhooks/{webhook_id}/{webhook_token}`. Posts as the webhook. No bot token needed when a token is attached.

```rust
pub fn execute_webhook<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, body: &'a ExecuteWebhook, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `execute_webhook_and_wait`

Calls `POST /webhooks/{webhook_id}/{webhook_token}`. Posts as the webhook and waits for the created message.

```rust
pub fn execute_webhook_and_wait<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, body: &'a ExecuteWebhook, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `execute_slack_webhook`

Calls `POST /webhooks/{webhook_id}/{webhook_token}/slack`.

```rust
pub fn execute_slack_webhook<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, body: &'a ExecuteWebhook, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `execute_github_webhook`

Calls `POST /webhooks/{webhook_id}/{webhook_token}/github`.

```rust
pub fn execute_github_webhook<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, body: &'a ExecuteWebhook, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```

##### `get_webhook_message`

Calls `GET /webhooks/{webhook_id}/{webhook_token}/messages/{message_id}`.

```rust
pub fn get_webhook_message<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, message_id: MessageId, thread_id: Option<ChannelId>, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `edit_webhook_message`

Calls `PATCH /webhooks/{webhook_id}/{webhook_token}/messages/{message_id}`.

```rust
pub fn edit_webhook_message<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, message_id: MessageId, body: &'a EditWebhookMessage, ) -> impl Future<Output = Result<Message, Error>> + Send + 'a;
```

##### `delete_webhook_message`

Calls `DELETE /webhooks/{webhook_id}/{webhook_token}/messages/{message_id}`.

```rust
pub fn delete_webhook_message<'a>( &'a self, webhook_id: WebhookId, webhook_token: &'a str, message_id: MessageId, thread_id: Option<ChannelId>, ) -> impl Future<Output = Result<(), Error>> + Send + 'a;
```
