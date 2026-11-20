# Builders

Typed builders for commands, embed replies and validation. Every builder validates in `.build()`.

## `builders.command.context_menu`

### `ContextMenuCommandBuilder`

Builds context menu command values.

```rust
pub struct ContextMenuCommandBuilder { /* private fields */ }
```

#### Methods

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

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<CreateApplicationCommand, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> CreateApplicationCommand;
```

## `builders.command.slash`

### `SlashCommandBuilder`

Builds slash command values.

```rust
pub struct SlashCommandBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `SlashCommandBuilder`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

##### `option`

Adds a typed option to the command.

```rust
pub fn option(mut self, option: impl Into<CommandOption>) -> Self;
```

##### `subcommand`

Adds a subcommand.

```rust
pub fn subcommand(self, subcommand: SlashCommandSubcommandBuilder) -> Self;
```

##### `subcommand_group`

Adds a group of subcommands.

```rust
pub fn subcommand_group(self, group: SlashCommandSubcommandGroupBuilder) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<CreateApplicationCommand, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> CreateApplicationCommand;
```

### `SlashCommandSubcommandBuilder`

Builds slash command subcommand values.

```rust
pub struct SlashCommandSubcommandBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `SlashCommandSubcommandBuilder`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

##### `option`

Adds a typed option to the subcommand.

```rust
pub fn option(mut self, option: impl Into<CommandOption>) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<CommandOption, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> CommandOption;
```


### `SlashCommandSubcommandGroupBuilder`

Builds slash command subcommand group values.

```rust
pub struct SlashCommandSubcommandGroupBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `SlashCommandSubcommandGroupBuilder`.

```rust
pub fn new(name: impl Into<CompactString>, description: impl Into<CompactString>) -> Self;
```

##### `subcommand`

Adds a subcommand to the group.

```rust
pub fn subcommand(mut self, subcommand: SlashCommandSubcommandBuilder) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<CommandOption, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> CommandOption;
```

## `builders.command.option`

### `SlashCommandStringOption`

##### `choice`

Adds one fixed choice.

```rust
pub fn choice( mut self, name: impl Into<CompactString>, value: impl Into<CompactString>, ) -> Self;
```

##### `choices`

Adds several fixed choices.

```rust
pub fn choices(mut self, choices: impl IntoIterator<Item = Choice<CompactString>>) -> Self;
```

##### `min_length`

Sets the minimum accepted length.

```rust
pub fn min_length(mut self, min_length: u16) -> Self;
```

##### `max_length`

Sets the maximum accepted length.

```rust
pub fn max_length(mut self, max_length: u16) -> Self;
```

##### `autocomplete`

Returns whether this is an autocomplete interaction.

```rust
pub fn autocomplete(mut self, autocomplete: bool) -> Self;
```

### `SlashCommandIntegerOption`

##### `choice`

Adds one fixed choice.

```rust
pub fn choice(mut self, name: impl Into<CompactString>, value: i64) -> Self;
```

##### `choices`

Adds several fixed choices.

```rust
pub fn choices(mut self, choices: impl IntoIterator<Item = Choice<i64>>) -> Self;
```

##### `min_value`

Sets the minimum accepted value.

```rust
pub fn min_value(mut self, min_value: i64) -> Self;
```

##### `max_value`

Sets the maximum accepted value.

```rust
pub fn max_value(mut self, max_value: i64) -> Self;
```

##### `autocomplete`

Returns whether this is an autocomplete interaction.

```rust
pub fn autocomplete(mut self, autocomplete: bool) -> Self;
```

### `SlashCommandNumberOption`

##### `choice`

Adds one fixed choice.

```rust
pub fn choice(mut self, name: impl Into<CompactString>, value: f64) -> Self;
```

##### `choices`

Adds several fixed choices.

```rust
pub fn choices(mut self, choices: impl IntoIterator<Item = Choice<f64>>) -> Self;
```

##### `min_value`

Sets the minimum accepted value.

```rust
pub fn min_value(mut self, min_value: f64) -> Self;
```

##### `max_value`

Sets the maximum accepted value.

```rust
pub fn max_value(mut self, max_value: f64) -> Self;
```

##### `autocomplete`

Returns whether this is an autocomplete interaction.

```rust
pub fn autocomplete(mut self, autocomplete: bool) -> Self;
```

### `SlashCommandChannelOption`

##### `channel_types`

Restricts the selectable channel types.

```rust
pub fn channel_types(mut self, channel_types: impl IntoIterator<Item = ChannelType>) -> Self;
```

### `SlashCommandAttachmentOption`

##### `file_types`

Restricts the accepted file extensions.

```rust
pub fn file_types<T: Into<CompactString>>( mut self, file_types: impl IntoIterator<Item = T>, ) -> Self;
```

## `builders.embed`

### `EmbedBuilder`

Builds embed values.

```rust
pub struct EmbedBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `EmbedBuilder`.

```rust
pub fn new() -> Self;
```

##### `title`

Sets the embed title.

```rust
pub fn title(mut self, title: impl Into<CompactString>) -> Self;
```

##### `url`

Sets the URL the title links to.

```rust
pub fn url(mut self, url: impl Into<CompactString>) -> Self;
```

##### `description`

Sets the embed body.

```rust
pub fn description(mut self, description: impl Into<CompactString>) -> Self;
```

##### `color`

Sets the sidebar color from a 24-bit integer.

```rust
pub fn color(mut self, color: u32) -> Self;
```

##### `color_rgb`

Sets the sidebar color from red, green and blue components.

```rust
pub fn color_rgb(self, red: u8, green: u8, blue: u8) -> Self;
```

##### `author`

Sets the author line.

```rust
pub fn author(mut self, author: impl Into<EmbedAuthorBuilder>) -> Self;
```

##### `footer`

Sets the footer line.

```rust
pub fn footer(mut self, footer: impl Into<EmbedFooterBuilder>) -> Self;
```

##### `field`

Adds a block field.

```rust
pub fn field( mut self, name: impl Into<CompactString>, value: impl Into<CompactString>, ) -> Self;
```

##### `inline_field`

Adds a field that shares its row with neighbors.

```rust
pub fn inline_field( mut self, name: impl Into<CompactString>, value: impl Into<CompactString>, ) -> Self;
```

##### `fields`

Adds several fields at once.

```rust
pub fn fields(mut self, fields: impl IntoIterator<Item = EmbedField>) -> Self;
```

##### `image`

Sets the large image.

```rust
pub fn image(mut self, url: impl Into<CompactString>) -> Self;
```

##### `thumbnail`

Sets the thumbnail.

```rust
pub fn thumbnail(mut self, url: impl Into<CompactString>) -> Self;
```

##### `timestamp`

Sets the timestamp shown in the footer.

```rust
pub fn timestamp(mut self, timestamp: Timestamp) -> Self;
```

##### `timestamp_now`

Sets the timestamp to the current time.

```rust
pub fn timestamp_now(self) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<Embed, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> Embed;
```

### `embed_length`

```rust
pub fn embed_length(embed: &Embed) -> usize;
```

### `validate_embeds`

```rust
pub fn validate_embeds(embeds: &[Embed]) -> Result<(), ValidationError>;
```

### `EmbedAuthorBuilder`

Builds embed author values.

```rust
pub struct EmbedAuthorBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `EmbedAuthorBuilder`.

```rust
pub fn new(name: impl Into<CompactString>) -> Self;
```

##### `url`

Sets the author link.

```rust
pub fn url(mut self, url: impl Into<CompactString>) -> Self;
```

##### `icon_url`

Sets the author icon.

```rust
pub fn icon_url(mut self, url: impl Into<CompactString>) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<EmbedAuthor, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> EmbedAuthor;
```

### `EmbedFooterBuilder`

Builds embed footer values.

```rust
pub struct EmbedFooterBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `EmbedFooterBuilder`.

```rust
pub fn new(text: impl Into<CompactString>) -> Self;
```

##### `icon_url`

Sets the footer icon.

```rust
pub fn icon_url(mut self, url: impl Into<CompactString>) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<EmbedFooter, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> EmbedFooter;
```

## `builders.reply`






## `builders.error`

### `ValidationError`

Possible validation error values.

```rust
pub enum ValidationError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `TooLong` | `field: Cow<'static, str>, max: usize, actual: usize,` | Too Long. |
| `TooMany` | `field: Cow<'static, str>, max: usize, actual: usize,` | Too Many. |
| `TooFew` | `field: Cow<'static, str>, min: usize, actual: usize,` | Too Few. |
| `Missing` | `field: Cow<'static, str>,` | Missing. |
| `OutOfRange` | `field: Cow<'static, str>, min: i64, max: i64, actual: i64,` | Out Of Range. |
| `InvalidUrl` | `field: Cow<'static, str>, allowed: &'static [&'static str],` | Invalid URL. |
| `InvalidName` | `field: Cow<'static, str>, character: char,` | Invalid Name. |
| `Invalid` | `field: Cow<'static, str>, reason: &'static str,` | Invalid. |
| `Empty` | `field: Cow<'static, str>, needs: &'static str,` | Empty. |

#### Methods

##### `field`

Adds a block field.

```rust
pub fn field(&self) -> &str;
```
