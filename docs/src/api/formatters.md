# Formatters

Markdown, mentions, timestamps, links and escaping as typed values.

## `formatters.escape`

### `EscapeOptions`

```rust
pub struct EscapeOptions { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `EscapeOptions`.

```rust
pub const fn new() -> Self;
```

##### `none`

Disables every escape.

```rust
pub const fn none() -> Self;
```

##### `escape`

Escapes the text with the configured options.

```rust
pub fn escape<'a>(&self, text: &'a str) -> Cow<'a, str>;
```

### `escape_markdown`

```rust
pub fn escape_markdown(text: &str) -> Cow<'_, str>;
```


## `formatters.link`


### `ChannelLink`

```rust
pub struct ChannelLink { /* private fields */ }
```

#### Methods

##### `guild`

Sets the guild the linked channel belongs to.

```rust
pub const fn guild(mut self, guild: GuildId) -> Self;
```

### `channel_link`

```rust
pub const fn channel_link(channel: ChannelId) -> ChannelLink;
```

### `MessageLink`

```rust
pub struct MessageLink { /* private fields */ }
```

#### Methods

##### `guild`

Sets the guild the linked message belongs to.

```rust
pub const fn guild(mut self, guild: GuildId) -> Self;
```

### `message_link`

```rust
pub const fn message_link(channel: ChannelId, message: MessageId) -> MessageLink;
```

### `ApplicationDirectory`

```rust
pub struct ApplicationDirectory { /* private fields */ }
```

#### Methods

##### `sku`

Links to the given SKU in the application directory.

```rust
pub const fn sku(mut self, sku: SkuId) -> Self;
```

### `application_directory`

```rust
pub const fn application_directory(application: ApplicationId) -> ApplicationDirectory;
```

### `Email`

```rust
pub struct Email<'a> { /* private fields */ }
```

#### Methods

##### `headers`

Adds headers to the mailto link.

```rust
pub const fn headers(mut self, headers: &'a [(&'a str, &'a str)]) -> Self;
```

### `email`

```rust
pub const fn email(address: &str) -> Email<'_>;
```

### `PhoneNumber`

```rust
pub struct PhoneNumber<'a>(&'a str);
```

### `phone_number`

```rust
pub fn phone_number(number: &str) -> Option<PhoneNumber<'_>>;
```

## `formatters.markdown`

### `Bold`

Bold text. Wraps its content in `**`.

```rust
pub struct Bold<T>(T);
```

### `bold`

```rust
pub const fn bold<T: fmt::Display>(content: T) -> Bold<T>;
```

### `Italic`

Italic text. Wraps its content in `_`.

```rust
pub struct Italic<T>(T);
```

### `italic`

```rust
pub const fn italic<T: fmt::Display>(content: T) -> Italic<T>;
```

### `Underline`

Underlined text. Wraps its content in `__`.

```rust
pub struct Underline<T>(T);
```

### `underline`

```rust
pub const fn underline<T: fmt::Display>(content: T) -> Underline<T>;
```

### `underscore`

```rust
pub const fn underscore<T: fmt::Display>(content: T) -> Underline<T>;
```

### `Strikethrough`

Struck-through text. Wraps its content in `~~`.

```rust
pub struct Strikethrough<T>(T);
```

### `strikethrough`

```rust
pub const fn strikethrough<T: fmt::Display>(content: T) -> Strikethrough<T>;
```

### `Spoiler`

Spoiler text. Wraps its content in `||`.

```rust
pub struct Spoiler<T>(T);
```

### `spoiler`

Marks the thumbnail as a spoiler.

```rust
pub const fn spoiler<T: fmt::Display>(content: T) -> Spoiler<T>;
```

### `InlineCode`

Inline code. Wraps its content in backticks.

```rust
pub struct InlineCode<T>(T);
```

### `inline_code`

```rust
pub const fn inline_code<T: fmt::Display>(content: T) -> InlineCode<T>;
```

### `Quote`

A single-line quote. Prefixes its content with `> `.

```rust
pub struct Quote<T>(T);
```

### `quote`

```rust
pub const fn quote<T: fmt::Display>(content: T) -> Quote<T>;
```

### `BlockQuote`

A multi-line block quote. Prefixes its content with `>>> `.

```rust
pub struct BlockQuote<T>(T);
```

### `block_quote`

```rust
pub const fn block_quote<T: fmt::Display>(content: T) -> BlockQuote<T>;
```

### `Subtext`

Small subtext. Prefixes its content with `-# `.

```rust
pub struct Subtext<T>(T);
```

### `subtext`

```rust
pub const fn subtext<T: fmt::Display>(content: T) -> Subtext<T>;
```

### `HideLinkEmbed`

A link that Discord does not unfurl. Wraps its content in angle brackets.

```rust
pub struct HideLinkEmbed<T>(T);
```

### `hide_link_embed`

```rust
pub const fn hide_link_embed<T: fmt::Display>(content: T) -> HideLinkEmbed<T>;
```

### `CodeBlock`

```rust
pub struct CodeBlock<'a, T> { /* private fields */ }
```

#### Methods

##### `language`

Sets the syntax-highlighting language.

```rust
pub const fn language(mut self, language: &'a str) -> Self;
```

### `code_block`

```rust
pub const fn code_block<'a, T: fmt::Display>(content: T) -> CodeBlock<'a, T>;
```

### `HeadingLevel`

Possible heading level values.

```rust
pub enum HeadingLevel
```

| Variant | Payload | Description |
| --- | --- | --- |
| `One` | `,` | One. |
| `Two` | `,` | Two. |
| `Three` | `,` | Three. |

### `Heading`

```rust
pub struct Heading<T> { /* private fields */ }
```

#### Methods

##### `level`

Sets the heading level.

```rust
pub const fn level(mut self, level: HeadingLevel) -> Self;
```

### `heading`

```rust
pub const fn heading<T: fmt::Display>(content: T) -> Heading<T>;
```

### `Hyperlink`

```rust
pub struct Hyperlink<'a, T, U> { /* private fields */ }
```

#### Methods

##### `title`

Sets the embed title.

```rust
pub const fn title(mut self, title: &'a str) -> Self;
```

### `hyperlink`

```rust
pub const fn hyperlink<'a, T: fmt::Display, U: fmt::Display>( content: T, url: U, ) -> Hyperlink<'a, T, U>;
```

### `ListItem`

Possible list item values.

```rust
pub enum ListItem<'a, T>
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Text` | `T` | Text. |
| `Nested` | `&'a [ListItem<'a, T>]` | Nested. |

### `ListEntry`

```rust
pub trait ListEntry: private::WriteEntry;
```

### `UnorderedList`

```rust
pub struct UnorderedList<'a, T> { /* private fields */ }
```

### `unordered_list`

```rust
pub const fn unordered_list<T: ListEntry>(items: &[T]) -> UnorderedList<'_, T>;
```

### `OrderedList`

```rust
pub struct OrderedList<'a, T> { /* private fields */ }
```

#### Methods

##### `start`

Starts running.

```rust
pub const fn start(mut self, start: u64) -> Self;
```

### `ordered_list`

```rust
pub const fn ordered_list<T: ListEntry>(items: &[T]) -> OrderedList<'_, T>;
```

### `Face`

Possible face values.

```rust
pub enum Face
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Shrug` | `,` | Shrug. |
| `Tableflip` | `,` | Tableflip. |
| `Unflip` | `,` | Unflip. |

#### Methods

##### `as_str`

Returns the string form.

```rust
pub const fn as_str(self) -> &'static str;
```

## `formatters.mention`

### `UserMention`

```rust
pub struct UserMention(UserId);
```

### `user_mention`

```rust
pub const fn user_mention(id: UserId) -> UserMention;
```

### `ChannelMention`

```rust
pub struct ChannelMention(ChannelId);
```

### `channel_mention`

```rust
pub const fn channel_mention(id: ChannelId) -> ChannelMention;
```

### `RoleMention`

```rust
pub struct RoleMention(RoleId);
```

### `role_mention`

```rust
pub const fn role_mention(id: RoleId) -> RoleMention;
```

### `LinkedRoleMention`

```rust
pub struct LinkedRoleMention(RoleId);
```

### `linked_role_mention`

```rust
pub const fn linked_role_mention(id: RoleId) -> LinkedRoleMention;
```

### `SlashCommandMention`

```rust
pub struct SlashCommandMention<'a> { /* private fields */ }
```

#### Methods

##### `subcommand`

Adds a subcommand.

```rust
pub const fn subcommand(mut self, subcommand: &'a str) -> Self;
```

##### `group`

Sets the subcommand group of the mentioned command.

```rust
pub const fn group(mut self, group: &'a str, subcommand: &'a str) -> Self;
```

### `slash_command_mention`

```rust
pub const fn slash_command_mention(name: &str, id: CommandId) -> SlashCommandMention<'_>;
```

### `Emoji`

A guild emoji, with name, image, roles and availability.

```rust
pub struct Emoji<'a> { /* private fields */ }
```

#### Methods

##### `name`

Returns the object's name.

```rust
pub const fn name(mut self, name: &'a str) -> Self;
```

##### `animated`

Marks the emoji as animated.

```rust
pub const fn animated(mut self, animated: bool) -> Self;
```

### `emoji`

Sets the emoji shown on the button.

```rust
pub const fn emoji<'a>(id: EmojiId) -> Emoji<'a>;
```

### `GuildNavigation`

Possible guild navigation values.

```rust
pub enum GuildNavigation
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Browse` | `,` | Browse. |
| `Customize` | `,` | Customize. |
| `Guide` | `,` | Guide. |
| `LinkedRoles` | `,` | Linked Roles. |

#### Methods

##### `as_str`

Returns the string form.

```rust
pub const fn as_str(self) -> &'static str;
```

## `formatters.timestamp`

### `TimestampStyle`

Possible timestamp style values.

```rust
pub enum TimestampStyle
```

| Variant | Payload | Description |
| --- | --- | --- |
| `ShortTime` | `,` | Short Time. |
| `MediumTime` | `,` | Medium Time. |
| `ShortDate` | `,` | Short Date. |
| `LongDate` | `,` | Long Date. |
| `LongDateShortTime` | `,` | Long Date Short Time. |
| `FullDateShortTime` | `,` | Full Date Short Time. |
| `ShortDateShortTime` | `,` | Short Date Short Time. |
| `ShortDateMediumTime` | `,` | Short Date Medium Time. |
| `RelativeTime` | `,` | Relative Time. |

#### Methods

##### `as_char`

Borrows this as char.

```rust
pub const fn as_char(self) -> char;
```

### `UnixSeconds`

```rust
pub trait UnixSeconds;
```

#### Methods

##### `unix_seconds`

Returns the Unix timestamp in seconds.

```rust
fn unix_seconds(&self) -> i64;
```

##### `unix_seconds`

Returns the Unix timestamp in seconds.

```rust
fn unix_seconds(&self) -> i64;
```




### `Time`

```rust
pub struct Time { /* private fields */ }
```

#### Methods

##### `style`

Returns the style.

```rust
pub const fn style(mut self, style: TimestampStyle) -> Self;
```

##### `seconds`

Sets the Unix timestamp in seconds.

```rust
pub const fn seconds(self) -> i64;
```

### `time`

```rust
pub fn time(at: impl UnixSeconds) -> Time;
```
