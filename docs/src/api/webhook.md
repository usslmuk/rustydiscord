# Webhooks

Act as a webhook without logging in as a bot.

## `webhook`

### `WebhookClient`

A bot-less webhook client: send, edit and fetch through a webhook id and token.

```rust
pub struct WebhookClient { /* private fields */ }
```

#### Methods

##### `new`

Builds a client from a webhook id and token.

```rust
pub fn new(id: WebhookId, token: impl Into<String>) -> Self;
```

##### `from_url`

Parses a Discord webhook URL into a client. Fails on anything else.

```rust
pub fn from_url(url: &str) -> Result<Self, InvalidWebhookUrl>;
```

##### `rest`

Uses a preconfigured REST client instead of building one.

```rust
pub fn rest(mut self, rest: Rest) -> Self;
```

##### `id`

Returns the object's snowflake ID.

```rust
pub const fn id(&self) -> WebhookId;
```

##### `send`

Sends a webhook message and returns it.

```rust
pub fn send( &self, body: impl Into<ExecuteWebhook>, ) -> impl Future<Output = Result<Message, Error>> + Send + '_;
```

##### `fetch_message`

Fetches a previously sent webhook message.

```rust
pub async fn fetch_message( &self, message_id: MessageId, thread: Option<ChannelId>, ) -> Result<Message, Error>;
```

##### `edit_message`

Edits a previously sent webhook message.

```rust
pub async fn edit_message( &self, message_id: MessageId, body: &EditWebhookMessage, ) -> Result<Message, Error>;
```

##### `delete_message`

Deletes a previously sent webhook message.

```rust
pub async fn delete_message( &self, message_id: MessageId, thread: Option<ChannelId>, ) -> Result<(), Error>;
```

##### `fetch`

Fetches the webhook object itself.

```rust
pub async fn fetch(&self) -> Result<Webhook, Error>;
```

##### `edit`

Edits the webhook (name, avatar, channel).

```rust
pub async fn edit(&self, body: &ModifyWebhook) -> Result<Webhook, Error>;
```

##### `delete`

Deletes the webhook.

```rust
pub async fn delete(&self) -> Result<(), Error>;
```

### `InvalidWebhookUrl`

The string was not a Discord webhook URL.

```rust
pub struct InvalidWebhookUrl(());
```

### `WebhookError`

Webhook operations that can fail without an HTTP round trip.

```rust
pub enum WebhookError
```

| Variant | Payload | Description |
| --- | --- | --- |
| `MissingToken` | `,` | Missing Token. |
| `Rest` | `#[from] Error` | Rest. |

