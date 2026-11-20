# Component builders

Build buttons, selects, modals and the newer layout components with validation at build time.

## `builders.component.action_row`

### `ActionRowBuilder`

Builds action row values.

```rust
pub struct ActionRowBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ActionRowBuilder`.

```rust
pub fn new() -> Self;
```

##### `component`

Appends one component to the row.

```rust
pub fn component(mut self, component: impl Into<Component>) -> Self;
```

##### `components`

Appends several components to the row.

```rust
pub fn components<C: Into<Component>>( mut self, components: impl IntoIterator<Item = C>, ) -> Self;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub fn id(mut self, id: i32) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<Component, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> Component;
```


## `builders.component.button`

### `ButtonBuilder`

Builds button values.

```rust
pub struct ButtonBuilder { /* private fields */ }
```

#### Methods

##### `primary`

Creates a blurple primary button.

```rust
pub fn primary(custom_id: impl Into<CompactString>) -> Self;
```

##### `secondary`

Creates a grey secondary button.

```rust
pub fn secondary(custom_id: impl Into<CompactString>) -> Self;
```

##### `success`

Creates a green success button.

```rust
pub fn success(custom_id: impl Into<CompactString>) -> Self;
```

##### `danger`

Creates a red danger button.

```rust
pub fn danger(custom_id: impl Into<CompactString>) -> Self;
```

##### `link`

Creates a link button that opens a URL.

```rust
pub fn link(url: impl Into<CompactString>) -> Self;
```

##### `premium`

Creates a SKU purchase button.

```rust
pub fn premium(sku_id: SkuId) -> PremiumButtonBuilder;
```

##### `label`

Sets the text on the button.

```rust
pub fn label(mut self, label: impl Into<CompactString>) -> Self;
```

##### `emoji`

Sets the emoji shown on the button.

```rust
pub fn emoji(mut self, emoji: impl Into<PartialEmoji>) -> Self;
```

##### `disabled`

Disables the button.

```rust
pub fn disabled(mut self, disabled: bool) -> Self;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub fn id(mut self, id: i32) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<Component, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> Component;
```


### `PremiumButtonBuilder`

Builds premium button values.

```rust
pub struct PremiumButtonBuilder { /* private fields */ }
```

#### Methods

##### `disabled`

Disables the button.

```rust
pub fn disabled(mut self, disabled: bool) -> Self;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub fn id(mut self, id: i32) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<Component, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> Component;
```

## `builders.component.content`

### `TextDisplayBuilder`

Builds text display values.

```rust
pub struct TextDisplayBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `TextDisplayBuilder`.

```rust
pub fn new(content: impl Into<CompactString>) -> Self;
```

### `ThumbnailBuilder`

Builds thumbnail values.

```rust
pub struct ThumbnailBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ThumbnailBuilder`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

##### `description`

Sets the alt text.

```rust
pub fn description(mut self, description: impl Into<CompactString>) -> Self;
```

##### `spoiler`

Marks the thumbnail as a spoiler.

```rust
pub fn spoiler(mut self, spoiler: bool) -> Self;
```

### `MediaGalleryBuilder`

Builds media gallery values.

```rust
pub struct MediaGalleryBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `MediaGalleryBuilder`.

```rust
pub fn new() -> Self;
```

##### `item`

Appends one media item.

```rust
pub fn item(mut self, url: impl Into<CompactString>) -> Self;
```

##### `item_with`

Appends one media item with a builder.

```rust
pub fn item_with(mut self, item: MediaGalleryItemBuilder) -> Self;
```

##### `items`

Appends several media items.

```rust
pub fn items<I: Into<MediaGalleryItem>>(mut self, items: impl IntoIterator<Item = I>) -> Self;
```

### `MediaGalleryItemBuilder`

Builds media gallery item values.

```rust
pub struct MediaGalleryItemBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `MediaGalleryItemBuilder`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

##### `description`

Sets the embed body.

```rust
pub fn description(mut self, description: impl Into<CompactString>) -> Self;
```

##### `spoiler`

Marks the thumbnail as a spoiler.

```rust
pub fn spoiler(mut self, spoiler: bool) -> Self;
```


### `FileBuilder`

Builds file values.

```rust
pub struct FileBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `FileBuilder`.

```rust
pub fn new(url: impl Into<CompactString>) -> Self;
```

##### `spoiler`

Marks the thumbnail as a spoiler.

```rust
pub fn spoiler(mut self, spoiler: bool) -> Self;
```

## `builders.component.layout`

### `ContainerBuilder`

Builds container values.

```rust
pub struct ContainerBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ContainerBuilder`.

```rust
pub fn new() -> Self;
```

##### `text`

Adds a markdown text block.

```rust
pub fn text(self, content: impl Into<CompactString>) -> Self;
```

##### `component`

Adds one component.

```rust
pub fn component(mut self, component: impl Into<Component>) -> Self;
```

##### `components`

Adds several components.

```rust
pub fn components<C: Into<Component>>( mut self, components: impl IntoIterator<Item = C>, ) -> Self;
```

##### `accent_color`

Sets the accent color from a 24-bit integer.

```rust
pub fn accent_color(mut self, color: u32) -> Self;
```

##### `spoiler`

Marks the container as a spoiler.

```rust
pub fn spoiler(mut self, spoiler: bool) -> Self;
```

### `SectionBuilder`

Builds section values.

```rust
pub struct SectionBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `SectionBuilder`.

```rust
pub fn new(accessory: impl Into<Component>) -> Self;
```

##### `text`

Adds the section text.

```rust
pub fn text(self, content: impl Into<CompactString>) -> Self;
```

##### `component`

Adds the section accessory.

```rust
pub fn component(mut self, component: impl Into<Component>) -> Self;
```

### `SeparatorBuilder`

Builds separator values.

```rust
pub struct SeparatorBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `SeparatorBuilder`.

```rust
pub fn new() -> Self;
```

##### `divider`

Shows or hides the divider line.

```rust
pub fn divider(mut self, divider: bool) -> Self;
```

##### `spacing`

Sets the space above and below.

```rust
pub fn spacing(mut self, spacing: SeparatorSpacing) -> Self;
```

## `builders.component.list`

### `ComponentsVersion`

Possible components version values.

```rust
pub enum ComponentsVersion
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Legacy` | `,` | Legacy. |
| `V2` | `,` | V 2. |

### `validate_components`

```rust
pub fn validate_components( components: &[Component], version: ComponentsVersion, ) -> Result<(), ValidationError>;
```

## `builders.component.modal`

### `ModalBuilder`

Builds modal values.

```rust
pub struct ModalBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ModalBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>, title: impl Into<CompactString>) -> Self;
```

##### `label`

Adds a labeled input.

```rust
pub fn label(self, label: impl Into<CompactString>, field: impl Into<Component>) -> Self;
```

##### `text`

Adds a text input.

```rust
pub fn text(self, content: impl Into<CompactString>) -> Self;
```

##### `component`

Adds one component.

```rust
pub fn component(mut self, component: impl Into<Component>) -> Self;
```

##### `build`

Validates the current state and builds the final value.

```rust
pub fn build(self) -> Result<ModalCallbackData, ValidationError>;
```

##### `build_unchecked`

Builds without running validation. Invalid state is the caller's problem.

```rust
pub fn build_unchecked(self) -> ModalCallbackData;
```

### `LabelBuilder`

Builds label values.

```rust
pub struct LabelBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `LabelBuilder`.

```rust
pub fn new(label: impl Into<CompactString>, field: impl Into<Component>) -> Self;
```

##### `description`

Sets the helper text under the label.

```rust
pub fn description(mut self, description: impl Into<CompactString>) -> Self;
```

### `TextInputBuilder`

Builds text input values.

```rust
pub struct TextInputBuilder { /* private fields */ }
```

#### Methods

##### `short`

Creates a single-line text input.

```rust
pub fn short(custom_id: impl Into<CompactString>) -> Self;
```

##### `paragraph`

Creates a multi-line text input.

```rust
pub fn paragraph(custom_id: impl Into<CompactString>) -> Self;
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

##### `required`

Requires the input before submission.

```rust
pub fn required(mut self, required: bool) -> Self;
```

##### `value`

Sets the initial value.

```rust
pub fn value(mut self, value: impl Into<CompactString>) -> Self;
```

##### `placeholder`

Sets the placeholder text.

```rust
pub fn placeholder(mut self, placeholder: impl Into<CompactString>) -> Self;
```

### `FileUploadBuilder`

Builds file upload values.

```rust
pub struct FileUploadBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `FileUploadBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

##### `min_values`

Sets the minimum number of files required.

```rust
pub fn min_values(mut self, min_values: u8) -> Self;
```

##### `max_values`

Sets the maximum number of files accepted.

```rust
pub fn max_values(mut self, max_values: u8) -> Self;
```

##### `required`

Requires the input before submission.

```rust
pub fn required(mut self, required: bool) -> Self;
```

##### `file_types`

Restricts the accepted file extensions.

```rust
pub fn file_types<T: Into<CompactString>>( mut self, file_types: impl IntoIterator<Item = T>, ) -> Self;
```

### `RadioGroupBuilder`

Builds radio group values.

```rust
pub struct RadioGroupBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `RadioGroupBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

##### `option`

Adds one radio option.

```rust
pub fn option( mut self, label: impl Into<CompactString>, value: impl Into<CompactString>, ) -> Self;
```

##### `option_with`

Adds one radio option with a builder.

```rust
pub fn option_with(mut self, option: RadioGroupOptionBuilder) -> Self;
```

##### `options`

Adds several radio options.

```rust
pub fn options<O: Into<GroupOption>>(mut self, options: impl IntoIterator<Item = O>) -> Self;
```

##### `required`

Requires the input before submission.

```rust
pub fn required(mut self, required: bool) -> Self;
```

### `CheckboxGroupBuilder`

Builds checkbox group values.

```rust
pub struct CheckboxGroupBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `CheckboxGroupBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

##### `option`

Adds one checkbox option.

```rust
pub fn option( mut self, label: impl Into<CompactString>, value: impl Into<CompactString>, ) -> Self;
```

##### `option_with`

Adds one checkbox option with a builder.

```rust
pub fn option_with(mut self, option: CheckboxGroupOptionBuilder) -> Self;
```

##### `options`

Adds several checkbox options.

```rust
pub fn options<O: Into<GroupOption>>(mut self, options: impl IntoIterator<Item = O>) -> Self;
```

##### `min_values`

Sets the minimum number of options that must be checked.

```rust
pub fn min_values(mut self, min_values: u8) -> Self;
```

##### `max_values`

Sets the maximum number of options that may be checked.

```rust
pub fn max_values(mut self, max_values: u8) -> Self;
```

##### `required`

Requires the input before submission.

```rust
pub fn required(mut self, required: bool) -> Self;
```

### `CheckboxBuilder`

Builds checkbox values.

```rust
pub struct CheckboxBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `CheckboxBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

##### `default`

Sets whether the checkbox starts checked.

```rust
pub fn default(mut self, default: bool) -> Self;
```

## `builders.component.select`

### `StringSelectMenuBuilder`

Builds string select menu values.

```rust
pub struct StringSelectMenuBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `StringSelectMenuBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

##### `option`

Adds one selectable option.

```rust
pub fn option( mut self, label: impl Into<CompactString>, value: impl Into<CompactString>, ) -> Self;
```

##### `option_with`

Adds one selectable option with a builder.

```rust
pub fn option_with(mut self, option: StringSelectMenuOptionBuilder) -> Self;
```

##### `options`

Adds several selectable options.

```rust
pub fn options<O: Into<SelectOption>>(mut self, options: impl IntoIterator<Item = O>) -> Self;
```

### `StringSelectMenuOptionBuilder`

Builds string select menu option values.

```rust
pub struct StringSelectMenuOptionBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `StringSelectMenuOptionBuilder`.

```rust
pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self;
```

##### `description`

Sets the option description.

```rust
pub fn description(mut self, description: impl Into<CompactString>) -> Self;
```

##### `emoji`

Sets the option emoji.

```rust
pub fn emoji(mut self, emoji: impl Into<PartialEmoji>) -> Self;
```

##### `default`

Selects the option by default.

```rust
pub fn default(mut self, default: bool) -> Self;
```


### `UserSelectMenuBuilder`

Builds user select menu values.

```rust
pub struct UserSelectMenuBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `UserSelectMenuBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

### `RoleSelectMenuBuilder`

Builds role select menu values.

```rust
pub struct RoleSelectMenuBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `RoleSelectMenuBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

### `MentionableSelectMenuBuilder`

Builds mentionable select menu values.

```rust
pub struct MentionableSelectMenuBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `MentionableSelectMenuBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

### `ChannelSelectMenuBuilder`

Builds channel select menu values.

```rust
pub struct ChannelSelectMenuBuilder { /* private fields */ }
```

#### Methods

##### `new`

Creates a new `ChannelSelectMenuBuilder`.

```rust
pub fn new(custom_id: impl Into<CompactString>) -> Self;
```

##### `channel_types`

Restricts the selectable channel types.

```rust
pub fn channel_types(mut self, channel_types: impl IntoIterator<Item = ChannelType>) -> Self;
```
