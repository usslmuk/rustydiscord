# Events

Every event you can subscribe to with `client.on`, its Discord name, payload type, and required intents. `ClientReady` fires once per connection; `Raw` fires for everything.

| Event | Discord name | Payload | Intents |
| --- | --- | --- | --- |
| `ClientReady` | `clientReady` | `Ready` | - |
| `ShardReady` | `shardReady` | `Ready` | - |
| `ShardResume` | `shardResume` | `ShardResume` | - |
| `ShardReconnecting` | `shardReconnecting` | `ShardReconnecting` | - |
| `Invalidated` | `invalidated` | `Invalidated` | - |
| `ShardError` | `shardError` | `ReceiveError` | - |
| `ShardDisconnect` | `shardDisconnect` | `CloseCode` | - |
| `Raw` | `raw` | `DispatchEvent` | - |
| `Debug` | `debug` | `String` | - |
| `Warn` | `warn` | `String` | - |
| `Error` | `error` | `String` | - |
| `ApplicationCommandPermissionsUpdate` | `applicationCommandPermissionsUpdate` | `ApplicationCommandPermissionsUpdate` | - |
| `AutoModerationActionExecution` | `autoModerationActionExecution` | `AutoModerationActionExecution` | AUTO_MODERATION_EXECUTION |
| `AutoModerationRuleCreate` | `autoModerationRuleCreate` | `AutoModerationRuleCreate` | AUTO_MODERATION_CONFIGURATION |
| `AutoModerationRuleUpdate` | `autoModerationRuleUpdate` | `AutoModerationRuleUpdate` | AUTO_MODERATION_CONFIGURATION |
| `AutoModerationRuleDelete` | `autoModerationRuleDelete` | `AutoModerationRuleDelete` | AUTO_MODERATION_CONFIGURATION |
| `EntitlementCreate` | `entitlementCreate` | `EntitlementCreate` | - |
| `EntitlementUpdate` | `entitlementUpdate` | `EntitlementUpdate` | - |
| `EntitlementDelete` | `entitlementDelete` | `EntitlementDelete` | - |
| `SubscriptionCreate` | `subscriptionCreate` | `SubscriptionCreate` | - |
| `SubscriptionUpdate` | `subscriptionUpdate` | `SubscriptionUpdate` | - |
| `SubscriptionDelete` | `subscriptionDelete` | `SubscriptionDelete` | - |
| `ChannelCreate` | `channelCreate` | `ChannelCreate` | GUILDS |
| `ChannelUpdate` | `channelUpdate` | `ChannelUpdate` | GUILDS |
| `ChannelDelete` | `channelDelete` | `ChannelDelete` | GUILDS |
| `ChannelPinsUpdate` | `channelPinsUpdate` | `ChannelPinsUpdate` | GUILDS | DIRECT_MESSAGES |
| `ChannelInfo` | `channelInfo` | `ChannelInfo` | - |
| `ThreadCreate` | `threadCreate` | `ThreadCreate` | GUILDS |
| `ThreadUpdate` | `threadUpdate` | `ThreadUpdate` | GUILDS |
| `ThreadDelete` | `threadDelete` | `ThreadDelete` | GUILDS |
| `ThreadListSync` | `threadListSync` | `ThreadListSync` | GUILDS |
| `ThreadMemberUpdate` | `threadMemberUpdate` | `ThreadMemberUpdate` | GUILDS |
| `ThreadMembersUpdate` | `threadMembersUpdate` | `ThreadMembersUpdate` | GUILDS |
| `GuildCreate` | `guildCreate` | `GuildCreate` | GUILDS |
| `GuildAvailable` | `guildAvailable` | `GuildCreate` | GUILDS |
| `GuildUpdate` | `guildUpdate` | `GuildUpdate` | GUILDS |
| `GuildDelete` | `guildDelete` | `GuildDelete` | GUILDS |
| `GuildUnavailable` | `guildUnavailable` | `GuildUnavailable` | GUILDS |
| `GuildAuditLogEntryCreate` | `guildAuditLogEntryCreate` | `GuildAuditLogEntryCreate` | GUILD_MODERATION |
| `GuildBanAdd` | `guildBanAdd` | `GuildBanAdd` | GUILD_MODERATION |
| `GuildBanRemove` | `guildBanRemove` | `GuildBanRemove` | GUILD_MODERATION |
| `GuildEmojisUpdate` | `guildEmojisUpdate` | `GuildEmojisUpdate` | GUILD_EXPRESSIONS |
| `GuildEmojiCreate` | `emojiCreate` | `GuildEmojiCreate` | GUILD_EXPRESSIONS |
| `GuildEmojiUpdate` | `emojiUpdate` | `GuildEmojiUpdate` | GUILD_EXPRESSIONS |
| `GuildEmojiDelete` | `emojiDelete` | `GuildEmojiDelete` | GUILD_EXPRESSIONS |
| `GuildStickersUpdate` | `guildStickersUpdate` | `GuildStickersUpdate` | GUILD_EXPRESSIONS |
| `GuildStickerCreate` | `stickerCreate` | `GuildStickerCreate` | GUILD_EXPRESSIONS |
| `GuildStickerUpdate` | `stickerUpdate` | `GuildStickerUpdate` | GUILD_EXPRESSIONS |
| `GuildStickerDelete` | `stickerDelete` | `GuildStickerDelete` | GUILD_EXPRESSIONS |
| `GuildIntegrationsUpdate` | `guildIntegrationsUpdate` | `GuildIntegrationsUpdate` | GUILD_INTEGRATIONS |
| `IntegrationCreate` | `integrationCreate` | `IntegrationCreate` | GUILD_INTEGRATIONS |
| `IntegrationUpdate` | `integrationUpdate` | `IntegrationUpdate` | GUILD_INTEGRATIONS |
| `IntegrationDelete` | `integrationDelete` | `IntegrationDelete` | GUILD_INTEGRATIONS |
| `GuildMemberAdd` | `guildMemberAdd` | `GuildMemberAdd` | GUILD_MEMBERS |
| `GuildMemberUpdate` | `guildMemberUpdate` | `GuildMemberUpdate` | GUILD_MEMBERS |
| `GuildMemberRemove` | `guildMemberRemove` | `GuildMemberRemove` | GUILD_MEMBERS |
| `GuildMemberAvailable` | `guildMemberAvailable` | `GuildMemberAvailable` | GUILD_MEMBERS |
| `GuildMembersChunk` | `guildMembersChunk` | `GuildMembersChunk` | - |
| `GuildRoleCreate` | `roleCreate` | `GuildRoleCreate` | GUILDS |
| `GuildRoleUpdate` | `roleUpdate` | `GuildRoleUpdate` | GUILDS |
| `GuildRoleDelete` | `roleDelete` | `GuildRoleDelete` | GUILDS |
| `GuildScheduledEventCreate` | `guildScheduledEventCreate` | `GuildScheduledEventCreate` | GUILD_SCHEDULED_EVENTS |
| `GuildScheduledEventUpdate` | `guildScheduledEventUpdate` | `GuildScheduledEventUpdate` | GUILD_SCHEDULED_EVENTS |
| `GuildScheduledEventDelete` | `guildScheduledEventDelete` | `GuildScheduledEventDelete` | GUILD_SCHEDULED_EVENTS |
| `GuildScheduledEventUserAdd` | `guildScheduledEventUserAdd` | `GuildScheduledEventUserAdd` | GUILD_SCHEDULED_EVENTS |
| `GuildScheduledEventUserRemove` | `guildScheduledEventUserRemove` | `GuildScheduledEventUserRemove` | GUILD_SCHEDULED_EVENTS |
| `GuildSoundboardSoundCreate` | `guildSoundboardSoundCreate` | `GuildSoundboardSoundCreate` | GUILD_EXPRESSIONS |
| `GuildSoundboardSoundUpdate` | `guildSoundboardSoundUpdate` | `GuildSoundboardSoundUpdate` | GUILD_EXPRESSIONS |
| `GuildSoundboardSoundDelete` | `guildSoundboardSoundDelete` | `GuildSoundboardSoundDelete` | GUILD_EXPRESSIONS |
| `GuildSoundboardSoundsUpdate` | `guildSoundboardSoundsUpdate` | `GuildSoundboardSoundsUpdate` | GUILD_EXPRESSIONS |
| `SoundboardSounds` | `soundboardSounds` | `SoundboardSounds` | - |
| `StageInstanceCreate` | `stageInstanceCreate` | `StageInstanceCreate` | GUILDS |
| `StageInstanceUpdate` | `stageInstanceUpdate` | `StageInstanceUpdate` | GUILDS |
| `StageInstanceDelete` | `stageInstanceDelete` | `StageInstanceDelete` | GUILDS |
| `InteractionCreate` | `interactionCreate` | `InteractionCreate` | - |
| `InviteCreate` | `inviteCreate` | `InviteCreate` | GUILD_INVITES |
| `InviteDelete` | `inviteDelete` | `InviteDelete` | GUILD_INVITES |
| `MessageCreate` | `messageCreate` | `MessageCreate` | GUILD_MESSAGES | DIRECT_MESSAGES |
| `MessageUpdate` | `messageUpdate` | `MessageUpdate` | GUILD_MESSAGES | DIRECT_MESSAGES |
| `MessageDelete` | `messageDelete` | `MessageDelete` | GUILD_MESSAGES | DIRECT_MESSAGES |
| `MessageBulkDelete` | `messageDeleteBulk` | `MessageBulkDelete` | GUILD_MESSAGES |
| `MessageReactionAdd` | `messageReactionAdd` | `MessageReactionAdd` | GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS |
| `MessageReactionRemove` | `messageReactionRemove` | `MessageReactionRemove` | GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS |
| `MessageReactionRemoveAll` | `messageReactionRemoveAll` | `MessageReactionRemoveAll` | GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS |
| `MessageReactionRemoveEmoji` | `messageReactionRemoveEmoji` | `MessageReactionRemoveEmoji` | GUILD_MESSAGE_REACTIONS | DIRECT_MESSAGE_REACTIONS |
| `MessagePollVoteAdd` | `messagePollVoteAdd` | `MessagePollVoteAdd` | GUILD_MESSAGE_POLLS | DIRECT_MESSAGE_POLLS |
| `MessagePollVoteRemove` | `messagePollVoteRemove` | `MessagePollVoteRemove` | GUILD_MESSAGE_POLLS | DIRECT_MESSAGE_POLLS |
| `TypingStart` | `typingStart` | `TypingStart` | GUILD_MESSAGE_TYPING | DIRECT_MESSAGE_TYPING |
| `PresenceUpdate` | `presenceUpdate` | `PresenceUpdate` | GUILD_PRESENCES |
| `UserUpdate` | `userUpdate` | `UserUpdate` | - |
| `VoiceStateUpdate` | `voiceStateUpdate` | `VoiceStateUpdate` | GUILD_VOICE_STATES |
| `VoiceServerUpdate` | `voiceServerUpdate` | `VoiceServerUpdate` | - |
| `VoiceChannelEffectSend` | `voiceChannelEffectSend` | `VoiceChannelEffectSend` | GUILD_VOICE_STATES |
| `VoiceChannelStatusUpdate` | `voiceChannelStatusUpdate` | `VoiceChannelStatusUpdate` | GUILDS |
| `VoiceChannelStartTimeUpdate` | `voiceChannelStartTimeUpdate` | `VoiceChannelStartTimeUpdate` | GUILDS |
| `WebhooksUpdate` | `webhooksUpdate` | `WebhooksUpdate` | GUILD_WEBHOOKS |
| `RateLimited` | `rateLimited` | `RateLimited` | - |
| `CacheSweep` | `cacheSweep` | `String` | - |
