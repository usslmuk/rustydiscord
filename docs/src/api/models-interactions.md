# Models: interactions and commands

Interactions, command options, resolved entities, callbacks and application commands.

## `model.interaction`

### `pub use self::{ data::{ CommandData, CommandDataOption, CommandOptionValue, ComponentData, InteractionData, ModalSubmitData, Resolved, SubmittedActionRow, SubmittedCheckbox, SubmittedComponent, SubmittedEntitySelect, SubmittedFileUpload, SubmittedLabel, SubmittedRadioGroup, SubmittedSelect, SubmittedTextDisplay, SubmittedTextInput, UnknownSubmittedComponent, }, getters::Mentionable, response::{ ActivityInstanceResource, AutocompleteCallbackData, AutocompleteChoices, CallbackResource, DeferredCallbackData, InteractionCallback, InteractionCallbackResponse, InteractionCallbackType, InteractionResponse, MessageCallbackData, ModalCallbackData, }, };`

```rust
pub use self::{ data::{ CommandData, CommandDataOption, CommandOptionValue, ComponentData, InteractionData, ModalSubmitData, Resolved, SubmittedActionRow, SubmittedCheckbox, SubmittedComponent, SubmittedEntitySelect, SubmittedFileUpload, SubmittedLabel, SubmittedRadioGroup, SubmittedSelect, SubmittedTextDisplay, SubmittedTextInput, UnknownSubmittedComponent, }, getters::Mentionable, response::{ ActivityInstanceResource, AutocompleteCallbackData, AutocompleteChoices, CallbackResource, DeferredCallbackData, InteractionCallback, InteractionCallbackResponse, InteractionCallbackType, InteractionResponse, MessageCallbackData, ModalCallbackData, }, };
```

### `Interaction`

An incoming interaction: command, component, modal or autocomplete, plus its context.

```rust
pub struct Interaction
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `InteractionId` | The object's snowflake ID. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `data` | `InteractionData` | The data. |
| `guild` | `Option<InteractionGuild>` | The guild this belongs to, when included. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `channel` | `Option<Box<InteractionChannel>>` | The channel. |
| `channel_id` | `Option<ChannelId>` | The ID of the channel this belongs to. |
| `member` | `Option<Box<InteractionMember>>` | The guild member this refers to. |
| `user` | `Option<Box<User>>` | The user this object describes. |
| `token` | `CompactString` | The token. |
| `version` | `u8` | The version. |
| `message` | `Option<Box<Message>>` | The message this refers to. |
| `app_permissions` | `Permissions` | The app permissions. |
| `locale` | `Option<Locale>` | The locale code. |
| `guild_locale` | `Option<Locale>` | The guild locale. |
| `entitlements` | `Vec<Entitlement>` | The entitlements. |
| `authorizing_integration_owners` | `AuthorizingIntegrationOwners` | The authorizing integration owners. |
| `context` | `Option<InteractionContextType>` | The context. |
| `attachment_size_limit` | `Option<u64>` | The attachment size limit. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> InteractionType;
```

##### `author`

Sets the author line.

```rust
pub fn author(&self) -> Option<&User>;
```

##### `command`

Sends a command to the shard.

```rust
pub fn command(&self) -> Option<&CommandData>;
```

##### `autocomplete`

Returns whether this is an autocomplete interaction.

```rust
pub fn autocomplete(&self) -> Option<&CommandData>;
```

##### `component`

Appends one component to the row.

```rust
pub fn component(&self) -> Option<&ComponentData>;
```

##### `modal`

Returns whether this is a modal submission.

```rust
pub fn modal(&self) -> Option<&ModalSubmitData>;
```

### `InteractionGuild`

The guild context of an interaction, with locale and features.

```rust
pub struct InteractionGuild
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GuildId` | The object's snowflake ID. |
| `locale` | `Option<Locale>` | The locale code. |
| `features` | `Vec<Feature>` | The feature flags enabled for the guild. |

### `InteractionChannel`

The channel context of an interaction.

```rust
pub struct InteractionChannel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `ChannelId` | The object's snowflake ID. |
| `kind` | `ChannelType` | The object's type discriminator. |
| `name` | `Option<CompactString>` | The object's name. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `permissions` | `Option<Permissions>` | The permissions bitfield. |
| `app_permissions` | `Option<Permissions>` | The app permissions. |
| `parent_id` | `Option<ChannelId>` | The ID of the parent channel or category. |
| `position` | `Option<i32>` | The position among siblings, lowest first. |
| `topic` | `Option<CompactString>` | The topic. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `last_message_id` | `Option<MessageId>` | The ID of the most recent message. |
| `last_pin_timestamp` | `Option<Timestamp>` | When the newest pinned message was pinned. |
| `rate_limit_per_user` | `Option<u16>` | Slowmode delay in seconds between messages per user. |
| `flags` | `ChannelFlags` | Bitfield of flags attached to the object. |
| `thread_metadata` | `Option<ThreadMetadata>` | The thread metadata. |
| `recipients` | `Vec<User>` | The users in the group DM. |

### `InteractionMember`

The member context of an interaction.

```rust
pub struct InteractionMember
```

| Field | Type | Description |
| --- | --- | --- |
| `member` | `PartialMember` | The guild member this refers to. |
| `user` | `Option<User>` | The user this object describes. |
| `permissions` | `Permissions` | The permissions bitfield. |

### `InteractionType`

What kind of interaction arrived.

```rust
pub enum InteractionType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Ping` | `,` | A gateway or HTTP ping. |
| `ApplicationCommand` | `,` | A slash command, user command or message command. |
| `MessageComponent` | `,` | A button or select menu was used. |
| `ApplicationCommandAutocomplete` | `,` | The user is typing an autocomplete option. |
| `ModalSubmit` | `,` | A modal was submitted. |

### `InteractionContextType`

Where the interaction was triggered: guild, bot DM or private channel.

```rust
pub enum InteractionContextType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Guild` | `,` | The interaction happened in a guild. |
| `BotDm` | `,` | The interaction happened in the bot's direct messages. |
| `PrivateChannel` | `,` | The interaction happened in a private channel such as a group DM. |

### `MessageInteraction`

The interaction that created a message.

```rust
pub struct MessageInteraction
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `InteractionId` | The object's snowflake ID. |
| `kind` | `InteractionType` | The object's type discriminator. |
| `name` | `CompactString` | The object's name. |
| `user` | `User` | The user this object describes. |
| `member` | `Option<Box<PartialMember>>` | The guild member this refers to. |

### `AuthorizingIntegrationOwners`

Which installation authorized the interaction.

```rust
pub struct AuthorizingIntegrationOwners
```

| Field | Type | Description |
| --- | --- | --- |
| `guild_install` | `Option<GuildInstallOwner>` | The guild install. |
| `user_install` | `Option<UserId>` | The user install. |

### `GuildInstallOwner`

Possible guild install owner values.

```rust
pub enum GuildInstallOwner
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Guild` | `GuildId` | Guild. |
| `BotDm` | `,` | Bot DM. |

#### Methods

##### `guild_id`

Returns the ID of the guild.

```rust
pub const fn guild_id(self) -> Option<GuildId>;
```

## `model.interaction.data`

### `InteractionData`

The interaction payload, matching the interaction type.

```rust
pub enum InteractionData
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Ping` | `,` | Ping. |
| `ApplicationCommand` | `Box<CommandData>` | Application Command. |
| `MessageComponent` | `Box<ComponentData>` | Message Component. |
| `Autocomplete` | `Box<CommandData>` | Autocomplete. |
| `ModalSubmit` | `Box<ModalSubmitData>` | Modal Submit. |
| `Unknown` | `InteractionType` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> InteractionType;
```

##### `command`

Sends a command to the shard.

```rust
pub fn command(&self) -> Option<&CommandData>;
```

##### `custom_id`

Returns the component's custom ID.

```rust
pub fn custom_id(&self) -> Option<&str>;
```

##### `resolved`

Returns the entities resolved for the command options.

```rust
pub fn resolved(&self) -> Option<&Resolved>;
```

### `CommandData`

Slash-command data: name, ID, options and resolved entities.

```rust
pub struct CommandData
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CommandId` | The object's snowflake ID. |
| `name` | `CompactString` | The object's name. |
| `kind` | `CommandType` | The object's type discriminator. |
| `resolved` | `Option<Box<Resolved>>` | The resolved. |
| `options` | `Vec<CommandDataOption>` | The selectable options or command options. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `target_id` | `Option<GenericId>` | The ID of the target. |

#### Methods

##### `focused`

Returns the focused autocomplete option, when one was supplied.

```rust
pub fn focused(&self) -> Option<&CommandDataOption>;
```

### `CommandDataOption`

One supplied command option and its value.

```rust
pub struct CommandDataOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `value` | `CommandOptionValue` | The option's value. |

#### Methods

##### `new`

Creates a new `CommandDataOption`.

```rust
pub fn new(name: impl Into<CompactString>, value: CommandOptionValue) -> Self;
```

### `CommandOptionValue`

A supplied option value: string, integer, boolean or nested options.

```rust
pub enum CommandOptionValue
```

| Variant | Payload | Description |
| --- | --- | --- |
| `SubCommand` | `Vec<CommandDataOption>` | Sub Command. |
| `SubCommandGroup` | `Vec<CommandDataOption>` | Sub Command Group. |
| `String` | `CompactString` | String. |
| `Integer` | `i64` | Integer. |
| `Boolean` | `bool` | Boolean. |
| `User` | `UserId` | User. |
| `Channel` | `ChannelId` | Channel. |
| `Role` | `RoleId` | Role. |
| `Mentionable` | `GenericId` | Mentionable. |
| `Number` | `f64` | Number. |
| `Attachment` | `AttachmentId` | Attachment. |
| `Focused` | `kind: CommandOptionType, value: CompactString,` | Focused. |
| `Unknown` | `CommandOptionType` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> CommandOptionType;
```


### `ComponentData`

Component interaction data: custom ID, values and component type.

```rust
pub struct ComponentData
```

| Field | Type | Description |
| --- | --- | --- |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `component_type` | `ComponentType` | The component type. |
| `values` | `Vec<CompactString>` | The values. |
| `resolved` | `Option<Box<Resolved>>` | The resolved. |

#### Methods

##### `ids`

Returns the custom IDs in the submitted row.

```rust
pub fn ids<T>(&self) -> impl Iterator<Item = Id<T>> + '_;
```

### `ModalSubmitData`

Modal submission data: custom ID and submitted components.

```rust
pub struct ModalSubmitData
```

| Field | Type | Description |
| --- | --- | --- |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `components` | `Vec<SubmittedComponent>` | The components attached to the message. |
| `resolved` | `Option<Box<Resolved>>` | The resolved. |

#### Methods

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub fn get(&self, custom_id: &str) -> Option<&SubmittedComponent>;
```

##### `text_value`

Returns the submitted value as text, when it is text.

```rust
pub fn text_value(&self, custom_id: &str) -> Option<&str>;
```

### `SubmittedComponent`

One component submitted with a modal.

```rust
pub enum SubmittedComponent
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ActionRow` | `SubmittedActionRow` | Action Row. |
| `TextInput` | `SubmittedTextInput` | Text Input. |
| `StringSelect` | `SubmittedSelect` | String Select. |
| `UserSelect` | `SubmittedEntitySelect<UserMarker>` | User Select. |
| `RoleSelect` | `SubmittedEntitySelect<RoleMarker>` | Role Select. |
| `MentionableSelect` | `SubmittedEntitySelect<GenericMarker>` | Mentionable Select. |
| `ChannelSelect` | `SubmittedEntitySelect<ChannelMarker>` | Channel Select. |
| `TextDisplay` | `SubmittedTextDisplay` | Text Display. |
| `Label` | `SubmittedLabel` | Label. |
| `FileUpload` | `SubmittedFileUpload` | File Upload. |
| `RadioGroup` | `SubmittedRadioGroup` | Radio Group. |
| `CheckboxGroup` | `SubmittedSelect` | Checkbox Group. |
| `Checkbox` | `SubmittedCheckbox` | Checkbox. |
| `Unknown` | `UnknownSubmittedComponent` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> ComponentType;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> Option<u32>;
```

##### `custom_id`

Returns the component's custom ID.

```rust
pub fn custom_id(&self) -> Option<&str>;
```

### `SubmittedActionRow`

Discord's submitted action row object.

```rust
pub struct SubmittedActionRow
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `components` | `Vec<SubmittedComponent>` | The components attached to the message. |

### `SubmittedTextInput`

Discord's submitted text input object.

```rust
pub struct SubmittedTextInput
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `value` | `CompactString` | The option's value. |

### `SubmittedSelect`

Discord's submitted select object.

```rust
pub struct SubmittedSelect
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `values` | `Vec<CompactString>` | The values. |

### `SubmittedEntitySelect`

Discord's submitted entity select object.

```rust
pub struct SubmittedEntitySelect<T>
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `values` | `Vec<Id<T>>` | The values. |

### `SubmittedTextDisplay`

Discord's submitted text display object.

```rust
pub struct SubmittedTextDisplay
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |

### `SubmittedLabel`

Discord's submitted label object.

```rust
pub struct SubmittedLabel
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `component` | `Box<SubmittedComponent>` | The component. |

### `SubmittedFileUpload`

Discord's submitted file upload object.

```rust
pub struct SubmittedFileUpload
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `values` | `Vec<AttachmentId>` | The values. |

### `SubmittedRadioGroup`

Discord's submitted radio group object.

```rust
pub struct SubmittedRadioGroup
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `value` | `Option<CompactString>` | The option's value. |

### `SubmittedCheckbox`

Discord's submitted checkbox object.

```rust
pub struct SubmittedCheckbox
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `Option<u32>` | The object's snowflake ID. |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `value` | `bool` | The option's value. |

### `UnknownSubmittedComponent`

Discord's unknown submitted component object.

```rust
pub struct UnknownSubmittedComponent
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `ComponentType` | The object's type discriminator. |
| `id` | `Option<u32>` | The object's snowflake ID. |


### `Resolved`

Users, members, roles, channels and attachments resolved for command options.

```rust
pub struct Resolved
```

| Field | Type | Description |
| --- | --- | --- |
| `users` | `BTreeMap<UserId, Box<User>>` | The users. |
| `members` | `BTreeMap<UserId, Box<InteractionMember>>` | The members in this collection. |
| `roles` | `BTreeMap<RoleId, Box<Role>>` | The roles attached or referenced. |
| `channels` | `BTreeMap<ChannelId, Box<InteractionChannel>>` | The channels. |
| `messages` | `BTreeMap<MessageId, Box<Message>>` | The messages. |
| `attachments` | `BTreeMap<AttachmentId, Box<Attachment>>` | The files attached to the message. |


## `model.interaction.getters`

### `Mentionable`

A user or role selected from the guild.

```rust
pub enum Mentionable<'a>
```

| Variant | Payload | Description |
| --- | --- | --- |
| `User` | `user: &'a User, member: Option<&'a InteractionMember>,` | User. |
| `Role` | `&'a Role` | Role. |

#### Methods

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> GenericId;
```


### `CommandData`

##### `leaf_options`

Returns the options at the leaf of the subcommand tree.

```rust
pub fn leaf_options(&self) -> &[CommandDataOption];
```

##### `subcommand`

Adds a subcommand.

```rust
pub fn subcommand(&self) -> Option<&str>;
```

##### `subcommand_group`

Adds a group of subcommands.

```rust
pub fn subcommand_group(&self) -> Option<&str>;
```

##### `get`

Looks up a value by key, cloning the shared handle when present.

```rust
pub fn get(&self, name: &str) -> Option<&CommandOptionValue>;
```

##### `get_string`

Returns string.

```rust
pub fn get_string(&self, name: &str) -> Option<&str>;
```

##### `get_integer`

Returns integer.

```rust
pub fn get_integer<T>(&self, name: &str) -> Option<T> where T: TryFrom<i64>,;
```

##### `get_number`

Returns number.

```rust
pub fn get_number(&self, name: &str) -> Option<f64>;
```

##### `get_boolean`

Returns boolean.

```rust
pub fn get_boolean(&self, name: &str) -> Option<bool>;
```

##### `get_user`

Returns user.

```rust
pub fn get_user(&self, name: &str) -> Option<&User>;
```

##### `get_member`

Returns member.

```rust
pub fn get_member(&self, name: &str) -> Option<(&User, &InteractionMember)>;
```

##### `get_role`

Returns role.

```rust
pub fn get_role(&self, name: &str) -> Option<&Role>;
```

##### `get_channel`

Returns channel.

```rust
pub fn get_channel(&self, name: &str) -> Option<&InteractionChannel>;
```

##### `get_attachment`

Returns attachment.

```rust
pub fn get_attachment(&self, name: &str) -> Option<&Attachment>;
```

##### `get_mentionable`

Returns mentionable.

```rust
pub fn get_mentionable(&self, name: &str) -> Option<Mentionable<'_>>;
```

##### `focused_value`

Returns the focused autocomplete value, when one was supplied.

```rust
pub fn focused_value(&self) -> Option<&str>;
```

##### `target_user`

Returns the target user of a user context-menu command.

```rust
pub fn target_user(&self) -> Option<&User>;
```

##### `target_member`

Returns the target member of a user context-menu command.

```rust
pub fn target_member(&self) -> Option<(&User, &InteractionMember)>;
```

##### `target_message`

Returns the target message of a message context-menu command.

```rust
pub fn target_message(&self) -> Option<&Message>;
```

### `ComponentData`

##### `users`

Returns the users.

```rust
pub fn users(&self) -> impl Iterator<Item = &User> + '_;
```

##### `members`

Returns the member manager.

```rust
pub fn members(&self) -> impl Iterator<Item = (&User, &InteractionMember)> + '_;
```

##### `roles`

Returns the role manager.

```rust
pub fn roles(&self) -> impl Iterator<Item = &Role> + '_;
```

##### `channels`

Returns the channel manager.

```rust
pub fn channels(&self) -> impl Iterator<Item = &InteractionChannel> + '_;
```

##### `mentionables`

Returns the mentionables selected in the modal.

```rust
pub fn mentionables(&self) -> impl Iterator<Item = Mentionable<'_>> + '_;
```

### `ModalSubmitData`

##### `string_select_values`

Returns the selected string-select values.

```rust
pub fn string_select_values(&self, custom_id: &str) -> Option<&[CompactString]>;
```

##### `checkbox_group_values`

Returns the selected checkbox-group values.

```rust
pub fn checkbox_group_values(&self, custom_id: &str) -> Option<&[CompactString]>;
```

##### `radio_group_value`

Returns the selected radio value.

```rust
pub fn radio_group_value(&self, custom_id: &str) -> Option<&str>;
```

##### `checkbox_value`

Returns whether the checkbox was checked.

```rust
pub fn checkbox_value(&self, custom_id: &str) -> Option<bool>;
```

##### `selected_users`

Returns the users selected in the modal.

```rust
pub fn selected_users<'a>( &'a self, custom_id: &str, ) -> Option<impl Iterator<Item = &'a User> + use<'a>>;
```

##### `selected_members`

Returns the members selected in the modal.

```rust
pub fn selected_members<'a>( &'a self, custom_id: &str, ) -> Option<impl Iterator<Item = (&'a User, &'a InteractionMember)> + use<'a>>;
```

##### `selected_roles`

Returns the roles selected in the modal.

```rust
pub fn selected_roles<'a>( &'a self, custom_id: &str, ) -> Option<impl Iterator<Item = &'a Role> + use<'a>>;
```

##### `selected_channels`

Returns the channels selected in the modal.

```rust
pub fn selected_channels<'a>( &'a self, custom_id: &str, ) -> Option<impl Iterator<Item = &'a InteractionChannel> + use<'a>>;
```

##### `selected_mentionables`

Returns the users or roles selected in the modal.

```rust
pub fn selected_mentionables<'a>( &'a self, custom_id: &str, ) -> Option<impl Iterator<Item = Mentionable<'a>> + use<'a>>;
```

##### `uploaded_files`

Returns the files uploaded with the modal.

```rust
pub fn uploaded_files<'a>( &'a self, custom_id: &str, ) -> Option<impl Iterator<Item = &'a Attachment> + use<'a>>;
```

## `model.interaction.response`

### `InteractionCallbackType`

How to answer an interaction: reply, defer, update or show a modal.

```rust
pub enum InteractionCallbackType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Pong` | `,` | Pong. |
| `ChannelMessageWithSource` | `,` | Channel Message With Source. |
| `DeferredChannelMessageWithSource` | `,` | Deferred Channel Message With Source. |
| `DeferredUpdateMessage` | `,` | Deferred Update Message. |
| `UpdateMessage` | `,` | Update Message. |
| `ApplicationCommandAutocompleteResult` | `,` | Application Command Autocomplete Result. |
| `Modal` | `,` | Modal. |
| `PremiumRequired` | `,` | Premium Required. |
| `LaunchActivity` | `,` | Launch Activity. |

### `InteractionResponse`

The response payload sent back to Discord.

```rust
pub enum InteractionResponse
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Pong` | `,` | Pong. |
| `ChannelMessageWithSource` | `Box<MessageCallbackData>` | Channel Message With Source. |
| `DeferredChannelMessageWithSource` | `DeferredCallbackData` | Deferred Channel Message With Source. |
| `DeferredUpdateMessage` | `,` | Deferred Update Message. |
| `UpdateMessage` | `Box<MessageCallbackData>` | Update Message. |
| `AutocompleteResult` | `AutocompleteCallbackData` | Autocomplete Result. |
| `Modal` | `Box<ModalCallbackData>` | Modal. |
| `PremiumRequired` | `,` | Premium Required. |
| `LaunchActivity` | `,` | Launch Activity. |
| `Unknown` | `InteractionCallbackType` | A value this build does not recognize. |

#### Methods

##### `files`

Returns the attached files.

```rust
pub fn files(&self) -> &[FileUpload];
```

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> InteractionCallbackType;
```


### `MessageCallbackData`

A message reply with components, embeds, files and flags.

```rust
pub struct MessageCallbackData
```

| Field | Type | Description |
| --- | --- | --- |
| `tts` | `bool` | The tts. |
| `content` | `Option<CompactString>` | The message text. |
| `embeds` | `Option<Vec<Embed>>` | The rich embeds attached to the message. |
| `allowed_mentions` | `Option<AllowedMentions>` | Controls which mentions in the message produce notifications. |
| `flags` | `Option<MessageFlags>` | Bitfield of flags attached to the object. |
| `components` | `Option<Vec<Component>>` | The components attached to the message. |
| `attachments` | `Option<Vec<AttachmentRequest>>` | The files attached to the message. |
| `poll` | `Option<CreatePoll>` | The poll attached to the message. |
| `files` | `Vec<FileUpload>` | The files attached to this request. |

### `DeferredCallbackData`

A deferred response that keeps the interaction alive.

```rust
pub struct DeferredCallbackData
```

| Field | Type | Description |
| --- | --- | --- |
| `flags` | `MessageFlags` | Bitfield of flags attached to the object. |

#### Methods

##### `new`

Creates a new `DeferredCallbackData`.

```rust
pub const fn new(flags: MessageFlags) -> Self;
```

### `AutocompleteCallbackData`

The autocomplete suggestions for an option.

```rust
pub struct AutocompleteCallbackData
```

| Field | Type | Description |
| --- | --- | --- |
| `choices` | `AutocompleteChoices` | The choices. |

#### Methods

##### `new`

Creates a new `AutocompleteCallbackData`.

```rust
pub const fn new(choices: AutocompleteChoices) -> Self;
```

### `AutocompleteChoices`

Possible autocomplete choices values.

```rust
pub enum AutocompleteChoices
```

| Variant | Payload | Description |
| --- | --- | --- |
| `String` | `Vec<Choice<CompactString>>` | String. |
| `Integer` | `Vec<Choice<i64>>` | Integer. |
| `Number` | `Vec<Choice<f64>>` | Number. |

### `ModalCallbackData`

The modal shown in response to a command or component.

```rust
pub struct ModalCallbackData
```

| Field | Type | Description |
| --- | --- | --- |
| `custom_id` | `CompactString` | The developer-defined ID used to route component interactions. |
| `title` | `CompactString` | The title. |
| `components` | `Vec<Component>` | The components attached to the message. |

#### Methods

##### `new`

Creates a new `ModalCallbackData`.

```rust
pub fn new( custom_id: impl Into<CompactString>, title: impl Into<CompactString>, components: Vec<Component>, ) -> Self;
```

### `InteractionCallbackResponse`

The message or activity Discord created for a callback.

```rust
pub struct InteractionCallbackResponse
```

| Field | Type | Description |
| --- | --- | --- |
| `interaction` | `InteractionCallback` | The interaction this refers to. |
| `resource` | `Option<CallbackResource>` | The resource. |

### `InteractionCallback`

Discord's interaction callback object.

```rust
pub struct InteractionCallback
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `InteractionId` | The object's snowflake ID. |
| `kind` | `InteractionType` | The object's type discriminator. |
| `activity_instance_id` | `Option<CompactString>` | The ID of the activity instance. |
| `response_message_id` | `Option<MessageId>` | The ID of the response message. |
| `response_message_loading` | `Option<bool>` | The response message loading. |
| `response_message_ephemeral` | `Option<bool>` | The response message ephemeral. |

### `CallbackResource`

Discord's callback resource object.

```rust
pub struct CallbackResource
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `InteractionCallbackType` | The object's type discriminator. |
| `activity_instance` | `Option<ActivityInstanceResource>` | The activity instance. |
| `message` | `Option<Box<Message>>` | The message this refers to. |

### `ActivityInstanceResource`

Discord's activity instance resource object.

```rust
pub struct ActivityInstanceResource
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CompactString` | The object's snowflake ID. |

## `model.application_command`

### `ApplicationCommand`

A slash command, user command or message command, with its options and permissions.

```rust
pub struct ApplicationCommand
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `CommandId` | The object's snowflake ID. |
| `kind` | `CommandType` | The object's type discriminator. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `guild_id` | `Option<GuildId>` | The ID of the guild this belongs to. |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `options` | `Vec<CommandOption>` | The selectable options or command options. |
| `default_member_permissions` | `Option<Permissions>` | Permissions required by default for members to use the command. |
| `dm_permission` | `Option<bool>` | The dm permission. |
| `default_permission` | `Option<bool>` | The default permission. |
| `nsfw` | `bool` | Whether the channel or content is age-restricted. |
| `integration_types` | `Vec<IntegrationType>` | The installation contexts where the command is available. |
| `contexts` | `Vec<InteractionContextType>` | The interaction contexts where the command is available. |
| `version` | `CommandVersionId` | The version. |
| `handler` | `Option<EntryPointHandlerType>` | The handler this is attached to. |

### `CommandType`

Whether the command is chat input, user context or message context.

```rust
pub enum CommandType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ChatInput` | `,` | A slash command. |
| `User` | `,` | A user context-menu command. |
| `Message` | `,` | A message context-menu command. |
| `PrimaryEntryPoint` | `,` | Primary Entry Point. |

### `EntryPointHandlerType`

Possible entry point handler type values.

```rust
pub enum EntryPointHandlerType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `AppHandler` | `,` | App Handler. |
| `DiscordLaunchActivity` | `,` | Discord Launch Activity. |

### `CommandOption`

One option on a slash command: subcommand, group or typed value.

```rust
pub enum CommandOption
```

| Variant | Payload | Description |
| --- | --- | --- |
| `SubCommand` | `SubCommandOption` | Sub Command. |
| `SubCommandGroup` | `SubCommandOption` | Sub Command Group. |
| `String` | `StringOption` | String. |
| `Integer` | `IntegerOption` | Integer. |
| `Boolean` | `BasicOption` | Boolean. |
| `User` | `BasicOption` | User. |
| `Channel` | `ChannelOption` | Channel. |
| `Role` | `BasicOption` | Role. |
| `Mentionable` | `BasicOption` | Mentionable. |
| `Number` | `NumberOption` | Number. |
| `Attachment` | `AttachmentOption` | Attachment. |
| `Unknown` | `UnknownOption` | A value this build does not recognize. |

#### Methods

##### `kind`

Returns the object's type discriminator.

```rust
pub const fn kind(&self) -> CommandOptionType;
```

##### `common`

Returns the fields shared by every option kind.

```rust
pub fn common(&self) -> OptionInfo<'_>;
```

##### `name`

Returns the object's name.

```rust
pub fn name(&self) -> &str;
```

##### `options`

Adds several radio options.

```rust
pub fn options(&self) -> &[Self];
```

### `OptionInfo`

The common fields every option kind carries.

```rust
pub struct OptionInfo<'a>
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `&'a str` | The object's name. |
| `name_localizations` | `&'a BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `&'a str` | A human-readable description. |
| `description_localizations` | `&'a BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<&'a str>` | The localized name. |
| `description_localized` | `Option<&'a str>` | The localized description. |

### `CommandOptionType`

The value type an option accepts.

```rust
pub enum CommandOptionType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `SubCommand` | `,` | A subcommand that groups options. |
| `SubCommandGroup` | `,` | A group of subcommands. |
| `String` | `,` | A string value, optionally with fixed choices. |
| `Integer` | `,` | A signed integer value. |
| `Boolean` | `,` | A true or false value. |
| `User` | `,` | A user selected from the guild. |
| `Channel` | `,` | A channel selected from the guild. |
| `Role` | `,` | A role selected from the guild. |
| `Mentionable` | `,` | A user or role selected from the guild. |
| `Number` | `,` | A floating-point number value. |
| `Attachment` | `,` | A file uploaded with the command. |

### `SubCommandOption`

Discord's sub command option object.

```rust
pub struct SubCommandOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `options` | `Vec<CommandOption>` | The selectable options or command options. |

#### Methods

##### `new`

Creates a new `SubCommandOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `StringOption`

Discord's string option object.

```rust
pub struct StringOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |
| `choices` | `Vec<Choice<CompactString>>` | The choices. |
| `min_length` | `Option<u16>` | The min length. |
| `max_length` | `Option<u16>` | The max length. |
| `autocomplete` | `bool` | Whether the option supports autocomplete. |

#### Methods

##### `new`

Creates a new `StringOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `IntegerOption`

Discord's integer option object.

```rust
pub struct IntegerOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |
| `choices` | `Vec<Choice<i64>>` | The choices. |
| `min_value` | `Option<i64>` | The min value. |
| `max_value` | `Option<i64>` | The max value. |
| `autocomplete` | `bool` | Whether the option supports autocomplete. |

#### Methods

##### `new`

Creates a new `IntegerOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `NumberOption`

Discord's number option object.

```rust
pub struct NumberOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |
| `choices` | `Vec<Choice<f64>>` | The choices. |
| `min_value` | `Option<f64>` | The min value. |
| `max_value` | `Option<f64>` | The max value. |
| `autocomplete` | `bool` | Whether the option supports autocomplete. |

#### Methods

##### `new`

Creates a new `NumberOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `BasicOption`

Discord's basic option object.

```rust
pub struct BasicOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |

#### Methods

##### `new`

Creates a new `BasicOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `ChannelOption`

Discord's channel option object.

```rust
pub struct ChannelOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |
| `channel_types` | `Vec<ChannelType>` | The channel types. |

#### Methods

##### `new`

Creates a new `ChannelOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `AttachmentOption`

Discord's attachment option object.

```rust
pub struct AttachmentOption
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |
| `file_types` | `Vec<CompactString>` | The file types. |

#### Methods

##### `new`

Creates a new `AttachmentOption`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

### `UnknownOption`

Discord's unknown option object.

```rust
pub struct UnknownOption
```

| Field | Type | Description |
| --- | --- | --- |
| `kind` | `CommandOptionType` | The object's type discriminator. |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `description` | `CompactString` | A human-readable description. |
| `description_localizations` | `BTreeMap<Locale, CompactString>` | Translated descriptions keyed by locale. |
| `name_localized` | `Option<CompactString>` | The localized name. |
| `description_localized` | `Option<CompactString>` | The localized description. |
| `required` | `bool` | Whether the option must be supplied. |

### `Choice`

One fixed choice offered by an option.

```rust
pub struct Choice<T>
```

| Field | Type | Description |
| --- | --- | --- |
| `name` | `CompactString` | The object's name. |
| `name_localizations` | `BTreeMap<Locale, CompactString>` | Translated names keyed by locale. |
| `value` | `T` | The option's value. |

#### Methods

##### `new`

Creates a new `Choice`.

```rust
pub fn new(name: impl Into<CompactString>, value: T) -> Self;
```



### `GuildCommandPermissions`

The permission overwrites configured for a guild command.

```rust
pub struct GuildCommandPermissions
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GenericId` | The object's snowflake ID. |
| `application_id` | `ApplicationId` | The ID of the application this belongs to. |
| `guild_id` | `GuildId` | The ID of the guild this belongs to. |
| `permissions` | `Vec<CommandPermission>` | The permissions bitfield. |

### `CommandPermission`

One role, user or channel rule for a command.

```rust
pub struct CommandPermission
```

| Field | Type | Description |
| --- | --- | --- |
| `id` | `GenericId` | The object's snowflake ID. |
| `kind` | `CommandPermissionType` | The object's type discriminator. |
| `permission` | `bool` | The permission. |

#### Methods

##### `new`

Creates a new `CommandPermission`.

```rust
pub const fn new(id: GenericId, kind: CommandPermissionType, permission: bool) -> Self;
```

##### `everyone`

Returns the rule that applies to everyone.

```rust
pub const fn everyone(guild_id: GuildId) -> GenericId;
```

##### `all_channels`

Returns the rule that applies to all channels.

```rust
pub const fn all_channels(guild_id: GuildId) -> Option<GenericId>;
```

### `CommandPermissionType`

Whether a command permission targets a role, user or channel.

```rust
pub enum CommandPermissionType: u8
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Role` | `,` | The rule applies to a role. |
| `User` | `,` | The rule applies to a user. |
| `Channel` | `,` | The rule applies to a channel. |
