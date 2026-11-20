# Interaction helpers

Reply, send, edit and manage through any model object.

## `ext`

### `MessageExt`

```rust
pub trait MessageExt: sealed::Message;
```

#### Methods

##### `reply`

Replies in the message's channel, with threading handled for you.

```rust
fn reply<B: Backend>( &self, ctx: &Context<B>, body: impl Into<CreateMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `edit`

Edits the message if it is the bot's own.

```rust
fn edit<B: Backend>( &self, ctx: &Context<B>, body: &EditMessage, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `delete`

Deletes the message.

```rust
fn delete<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `react`

Adds a reaction to the message.

```rust
fn react<'e, B: Backend>( &self, ctx: &Context<B>, emoji: impl Into<ReactionEmoji<'e>>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `pin`

Pins the message.

```rust
fn pin<B: Backend>( &self, ctx: &Context<B>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `unpin`

Unpins the message.

```rust
fn unpin<B: Backend>( &self, ctx: &Context<B>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `start_thread`

Starts a thread on the message.

```rust
fn start_thread<B: Backend>( &self, ctx: &Context<B>, body: &StartThreadFromMessage, reason: Option<&str>, ) -> impl Future<Output = Result<Arc<Channel>, Error>> + Send;
```

##### `crosspost`

Publishes a news message to follower channels.

```rust
fn crosspost<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `remove_reaction`

Removes one user's reaction, or the bot's own.

```rust
fn remove_reaction<'e, B: Backend>( &self, ctx: &Context<B>, emoji: impl Into<ReactionEmoji<'e>>, user: Option<UserId>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `remove_all_reactions`

Clears every reaction from the message.

```rust
fn remove_all_reactions<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `reaction_collector`

Collects reactions on the message with filtering and caps.

```rust
fn reaction_collector<B: Backend>(&self, ctx: &Context<B>) -> CollectorBuilder<ReactionEvent>;
```

##### `await_reactions`

Waits for matching reactions, then returns the grouped results.

```rust
fn await_reactions<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ReactionEvent, mode::All>;
```

##### `component_collector`

Creates a collector for component interactions.

```rust
fn component_collector<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ComponentInteraction>;
```

##### `await_component`

Waits for the next matching component interaction.

```rust
fn await_component<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ComponentInteraction, mode::First>;
```

### `ChannelExt`

Channel-scoped helpers available on every channel type.

```rust
pub trait ChannelExt: sealed::Channel;
```

#### Methods

##### `messages`

Opens the message manager for the channel.

```rust
fn messages<'a, B: Backend>(&self, ctx: &'a Context<B>) -> MessageManager<'a, B>;
```

##### `send`

Sends a message to the channel.

```rust
fn send<B: Backend>( &self, ctx: &Context<B>, body: impl Into<CreateMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `send_typing`

Broadcasts a typing indicator.

```rust
fn send_typing<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `bulk_delete`

Bulk-deletes up to 100 messages younger than two weeks.

```rust
fn bulk_delete<B: Backend>( &self, ctx: &Context<B>, message_ids: Vec<MessageId>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `edit`

Edits the channel (name, topic, rate limits).

```rust
fn edit<B: Backend>( &self, ctx: &Context<B>, body: &ModifyGuildChannel, reason: Option<&str>, ) -> impl Future<Output = Result<Arc<Channel>, Error>> + Send;
```

##### `delete`

Deletes the channel.

```rust
fn delete<B: Backend>( &self, ctx: &Context<B>, reason: Option<&str>, ) -> impl Future<Output = Result<Channel, Error>> + Send;
```

##### `set_permissions`

Sets a permission overwrite on the channel.

```rust
fn set_permissions<B: Backend>( &self, ctx: &Context<B>, target: impl Into<OverwriteTarget>, allow: Permissions, deny: Permissions, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `delete_permissions`

Removes a permission overwrite by id.

```rust
fn delete_permissions<B: Backend>( &self, ctx: &Context<B>, target: impl Into<OverwriteTarget>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `create_webhook`

Creates a webhook in the channel.

```rust
fn create_webhook<B: Backend>( &self, ctx: &Context<B>, body: &CreateWebhook, reason: Option<&str>, ) -> impl Future<Output = Result<Webhook, Error>> + Send;
```

##### `webhooks`

Lists the channel's webhooks.

```rust
fn webhooks<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Collection<WebhookId, Webhook>, Error>> + Send;
```

##### `create_invite`

Creates an invite for the channel.

```rust
fn create_invite<B: Backend>( &self, ctx: &Context<B>, body: &CreateChannelInvite, reason: Option<&str>, ) -> impl Future<Output = Result<InviteWithMetadata, Error>> + Send;
```

##### `invites`

Lists the channel's invites.

```rust
fn invites<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Collection<String, InviteWithMetadata>, Error>> + Send;
```

##### `threads`

Returns the thread manager.

```rust
fn threads<'a, B: Backend>(&self, ctx: &'a Context<B>) -> ThreadManager<'a, B>;
```

##### `thread_members`

Returns the thread-member manager.

```rust
fn thread_members<'a, B: Backend>(&self, ctx: &'a Context<B>) -> ThreadMemberManager<'a, B>;
```

##### `send_soundboard_sound`

Plays a soundboard sound in a voice channel.

```rust
fn send_soundboard_sound<B: Backend>( &self, ctx: &Context<B>, body: &SendSoundboardSound, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `message_collector`

Creates a collector for messages.

```rust
fn message_collector<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<Arc<MessageCreate>>;
```

##### `await_messages`

Waits for the next matching message.

```rust
fn await_messages<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<Arc<MessageCreate>, mode::All>;
```

##### `component_collector`

Creates a collector for component interactions.

```rust
fn component_collector<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ComponentInteraction>;
```

##### `await_component`

Waits for the next matching component interaction.

```rust
fn await_component<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ComponentInteraction, mode::First>;
```

### `UserExt`

DM helpers on any user.

```rust
pub trait UserExt: sealed::User;
```

#### Methods

##### `send`

DMs the user, creating the channel when needed.

```rust
fn send<B: Backend>( &self, ctx: &Context<B>, body: impl Into<CreateMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `create_dm`

Opens (or reuses) a DM channel with the user.

```rust
fn create_dm<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Arc<Channel>, Error>> + Send;
```

### `MemberExt`

Guild member actions: timeout, nickname, roles, kick, ban, permission checks.

```rust
pub trait MemberExt: sealed::Member;
```

#### Methods

##### `edit`

Edits the webhook (name, avatar, channel).

```rust
fn edit<B: Backend>( &self, ctx: &Context<B>, body: &ModifyGuildMember, reason: Option<&str>, ) -> impl Future<Output = Result<Arc<CachedMember>, Error>> + Send;
```

##### `timeout`

Times the member out until a timestamp, or clears it with None.

```rust
fn timeout<B: Backend>( &self, ctx: &Context<B>, until: Option<Timestamp>, reason: Option<&str>, ) -> impl Future<Output = Result<Arc<CachedMember>, Error>> + Send;
```

##### `set_nickname`

Sets the member's nickname, or clears it with None.

```rust
fn set_nickname<B: Backend>( &self, ctx: &Context<B>, nickname: Option<&str>, reason: Option<&str>, ) -> impl Future<Output = Result<Arc<CachedMember>, Error>> + Send;
```

##### `add_role`

Adds a role to the member.

```rust
fn add_role<B: Backend>( &self, ctx: &Context<B>, role_id: RoleId, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `remove_role`

Removes a role from the member.

```rust
fn remove_role<B: Backend>( &self, ctx: &Context<B>, role_id: RoleId, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `kick`

Kicks the member with an optional audit reason.

```rust
fn kick<B: Backend>( &self, ctx: &Context<B>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `ban`

Bans the member with optional pruning and reason.

```rust
fn ban<B: Backend>( &self, ctx: &Context<B>, body: &CreateGuildBan, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `permissions`

The member's guild-wide permissions.

```rust
fn permissions<B: Backend>(&self, ctx: &Context<B>) -> Option<Permissions>;
```

##### `permissions_in`

The member's effective permissions in one channel.

```rust
fn permissions_in<B: Backend>( &self, ctx: &Context<B>, channel_id: ChannelId, ) -> Option<Permissions>;
```

### `GuildExt`

Guild-scoped helpers: channels, roles, bans, emojis, invites, leave.

```rust
pub trait GuildExt: sealed::Guild;
```

#### Methods

##### `members`

Opens the guild's member manager.

```rust
fn members<'a, B: Backend>(&self, ctx: &'a Context<B>) -> MemberManager<'a, B>;
```

##### `roles`

Opens the guild's role manager.

```rust
fn roles<'a, B: Backend>(&self, ctx: &'a Context<B>) -> RoleManager<'a, B>;
```

##### `bans`

Opens the guild's ban manager.

```rust
fn bans<'a, B: Backend>(&self, ctx: &'a Context<B>) -> BanManager<'a, B>;
```

##### `channels`

Opens the guild's channel manager.

```rust
fn channels<'a, B: Backend>(&self, ctx: &'a Context<B>) -> GuildChannelManager<'a, B>;
```

##### `edit`

Edits the guild's settings.

```rust
fn edit<B: Backend>( &self, ctx: &Context<B>, body: &ModifyGuild, reason: Option<&str>, ) -> impl Future<Output = Result<Arc<CachedGuild>, Error>> + Send;
```

##### `leave`

Leaves the guild.

```rust
fn leave<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `emojis`

Opens the guild's emoji manager.

```rust
fn emojis<'a, B: Backend>(&self, ctx: &'a Context<B>) -> EmojiManager<'a, B>;
```

##### `stickers`

Returns the sticker manager.

```rust
fn stickers<'a, B: Backend>(&self, ctx: &'a Context<B>) -> StickerManager<'a, B>;
```

##### `webhooks`

Lists the channel's webhooks.

```rust
fn webhooks<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Collection<WebhookId, Webhook>, Error>> + Send;
```

##### `invites`

Opens the guild's invite manager.

```rust
fn invites<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Collection<String, InviteWithMetadata>, Error>> + Send;
```

##### `scheduled_events`

Opens the guild's scheduled-event manager.

```rust
fn scheduled_events<'a, B: Backend>( &self, ctx: &'a Context<B>, ) -> ScheduledEventManager<'a, B>;
```

##### `stage_instances`

Returns the stage-instance manager.

```rust
fn stage_instances<'a, B: Backend>(&self, ctx: &'a Context<B>) -> StageInstanceManager<'a, B>;
```

##### `soundboard_sounds`

Returns the soundboard-sound manager.

```rust
fn soundboard_sounds<'a, B: Backend>( &self, ctx: &'a Context<B>, ) -> SoundboardSoundManager<'a, B>;
```

##### `commands`

Returns the command manager.

```rust
fn commands<'a, B: Backend>(&self, ctx: &'a Context<B>) -> GuildCommandManager<'a, B>;
```

##### `auto_moderation_rules`

Returns the auto-moderation manager.

```rust
fn auto_moderation_rules<'a, B: Backend>( &self, ctx: &'a Context<B>, ) -> AutoModerationRuleManager<'a, B>;
```

##### `audit_log`

Reads the audit log with filtering.

```rust
fn audit_log<B: Backend>( &self, ctx: &Context<B>, query: &GetGuildAuditLog, ) -> impl Future<Output = Result<AuditLog, Error>> + Send;
```

##### `integrations`

Lists the guild's integrations.

```rust
fn integrations<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Collection<IntegrationId, Integration>, Error>> + Send;
```

### `WebhookExt`

Edit and delete through a webhook reference that carries its token.

```rust
pub trait WebhookExt: sealed::Webhook;
```

#### Methods

##### `send`

Sends through the webhook.

```rust
fn send<B: Backend>( &self, ctx: &Context<B>, body: impl Into<ExecuteWebhook>, ) -> impl Future<Output = Result<Arc<CachedMessage>, WebhookError>> + Send;
```

##### `edit`

Edits a message sent through the webhook.

```rust
fn edit<B: Backend>( &self, ctx: &Context<B>, body: &ModifyWebhook, reason: Option<&str>, ) -> impl Future<Output = Result<Webhook, Error>> + Send;
```

##### `delete`

Deletes the webhook.

```rust
fn delete<B: Backend>( &self, ctx: &Context<B>, reason: Option<&str>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

### `InteractionExt`

Answer interactions: reply, defer, update, follow up, modals.

```rust
pub trait InteractionExt: sealed::Interaction;
```

#### Methods

##### `reply`

Replies to the interaction, optionally ephemeral.

```rust
fn reply<B: Backend>( &self, ctx: &Context<B>, body: impl Into<MessageCallbackData>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `defer_reply`

Acknowledges first so slow work can follow up later.

```rust
fn defer_reply<B: Backend>( &self, ctx: &Context<B>, flags: MessageFlags, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `update`

Edits the message the interaction came from.

```rust
fn update<B: Backend>( &self, ctx: &Context<B>, body: impl Into<MessageCallbackData>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `defer_update`

Acknowledges a component interaction before editing.

```rust
fn defer_update<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `show_modal`

Opens a modal in response to the interaction.

```rust
fn show_modal<B: Backend>( &self, ctx: &Context<B>, modal: ModalCallbackData, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `respond`

Answers autocomplete interactions with choices.

```rust
fn respond<B: Backend>( &self, ctx: &Context<B>, choices: AutocompleteChoices, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `launch_activity`

Launches an embedded activity.

```rust
fn launch_activity<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `fetch_reply`

Fetches the bot's reply to the interaction.

```rust
fn fetch_reply<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `edit_reply`

Edits the bot's reply to the interaction.

```rust
fn edit_reply<B: Backend>( &self, ctx: &Context<B>, body: impl Into<EditWebhookMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `delete_reply`

Deletes the bot's reply to the interaction.

```rust
fn delete_reply<B: Backend>( &self, ctx: &Context<B>, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `follow_up`

Sends a follow-up message to the interaction.

```rust
fn follow_up<B: Backend>( &self, ctx: &Context<B>, body: impl Into<CreateFollowupMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `fetch_follow_up`

Fetches follow up.

```rust
fn fetch_follow_up<B: Backend>( &self, ctx: &Context<B>, message_id: MessageId, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `edit_follow_up`

Edits follow up.

```rust
fn edit_follow_up<B: Backend>( &self, ctx: &Context<B>, message_id: MessageId, body: impl Into<EditWebhookMessage>, ) -> impl Future<Output = Result<Arc<CachedMessage>, Error>> + Send;
```

##### `delete_follow_up`

Removes follow up.

```rust
fn delete_follow_up<B: Backend>( &self, ctx: &Context<B>, message_id: MessageId, ) -> impl Future<Output = Result<(), Error>> + Send;
```

##### `component_collector`

Creates a collector for component interactions.

```rust
fn component_collector<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ComponentInteraction>;
```

##### `await_component`

Waits for the next matching component interaction.

```rust
fn await_component<B: Backend>( &self, ctx: &Context<B>, ) -> CollectorBuilder<ComponentInteraction, mode::First>;
```

##### `await_modal_submit`

Waits for the next matching modal submission.

```rust
fn await_modal_submit<B: Backend>( &self, ctx: &Context<B>, time: Duration, ) -> CollectorBuilder<ModalInteraction, mode::First>;
```

