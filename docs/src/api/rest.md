# REST client

The typed HTTP layer: building requests, rate limits, errors, metrics, files and OAuth.

## `rest`

### `pub use self::{ client::{ AbortHandle, DEFAULT_API, RateLimitAction, RateLimited, Request, Rest, RestBuilder, RetryPolicy, }, error::{ApiError, Error, ErrorKind, FieldError}, route::{ReactionEmoji, Route}, transport::{Body, BoxFuture, Transport, TransportKind, transport_kind}, };`

```rust
pub use self::{ client::{ AbortHandle, DEFAULT_API, RateLimitAction, RateLimited, Request, Rest, RestBuilder, RetryPolicy, }, error::{ApiError, Error, ErrorKind, FieldError}, route::{ReactionEmoji, Route}, transport::{Body, BoxFuture, Transport, TransportKind, transport_kind}, };
```

### `pub use metrics::RestMetrics;`

```rust
pub use metrics::RestMetrics;
```

## `rest.client`

### `DEFAULT_API`

The default API base: https://discord.com/api/v10.

```rust
pub const DEFAULT_API: &str = "https://discord.com/api/v10";
```

### `AbortHandle`

Cancellation token for in-flight requests. Share it, abort it once, every waiter stops.

```rust
pub struct AbortHandle { /* private fields */ }
```

#### Methods

##### `new`

Creates a handle that has not fired yet.

```rust
pub fn new() -> Self;
```

##### `abort`

Trips the handle. In-flight waits end at the next checkpoint.

```rust
pub fn abort(&self);
```

##### `is_aborted`

Whether abort has been called already.

```rust
pub fn is_aborted(&self) -> bool;
```

### `RetryPolicy`

How failed requests are retried: max attempts, exponential base and cap, optional jitter.

```rust
pub struct RetryPolicy
```

| Field | Type | Description |
| --- | --- | --- |
| `max_retries` | `u8` | The max retries. |
| `base` | `Duration` | The base. |
| `cap` | `Duration` | The cap. |
| `jitter` | `bool` | The jitter. |

### `RateLimited`

A 429 as the client saw it: route, scope, global flag and the wait Discord asked for.

```rust
pub struct RateLimited
```

| Field | Type | Description |
| --- | --- | --- |
| `route` | `&'static str` | The route. |
| `global` | `bool` | The global. |
| `scope` | `Option<String>` | The scope. |
| `retry_after` | `Duration` | The retry after. |

### `RateLimitAction`

What the rate-limit hook decides: wait out the delay, or abort and surface the 429.

```rust
pub enum RateLimitAction
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Wait` | `,` | Wait. |
| `Abort` | `,` | Abort. |

### `Rest`

The typed HTTP client. Built once, cloned cheaply, shared everywhere.

```rust
pub struct Rest { /* private fields */ }
```

#### Methods

##### `new`

Builds a client from a bot token with default settings.

```rust
pub fn new(token: impl Into<String>) -> Self;
```

##### `cdn`

The CDN helper bound to this client.

```rust
pub fn cdn(&self) -> &Cdn;
```

##### `guild_widget_image_url`

A ready-to-serve guild widget image URL with optional style.

```rust
pub fn guild_widget_image_url(&self, guild_id: GuildId, style: Option<WidgetStyle>) -> String;
```

##### `builder`

Starts a configured client. Set the token, limits and transport here.

```rust
pub fn builder() -> RestBuilder;
```

##### `invalid_requests`

How many 401/403/429 responses arrived in the last 10 minutes.

```rust
pub fn invalid_requests(&self) -> u32;
```

##### `metrics`

Live counters: requests, 2xx, 429s, 5xx and retries.

```rust
pub fn metrics(&self) -> &RestMetrics;
```

##### `request`

Runs a typed request and deserializes the response body.

```rust
pub fn request<'a, T: DeserializeOwned + 'a>( &'a self, request: Request<'a>, ) -> impl Future<Output = Result<T, Error>> + Send + 'a;
```

##### `request_raw`

Runs a raw method-plus-path request and returns the JSON body.

```rust
pub async fn request_raw( &self, method: Method, path: &str, body: Option<serde_json::Value>, ) -> Result<serde_json::Value, Error>;
```


### `Request`

One API call under construction: route, query, body, audit reason, abort handle.

```rust
pub struct Request<'a> { /* private fields */ }
```

#### Methods

##### `new`

Starts a request for the given route.

```rust
pub const fn new(route: Route<'a>) -> Self;
```

##### `reason`

Attaches an audit-log reason, percent-encoded for the header.

```rust
pub const fn reason(mut self, reason: Option<&'a str>) -> Self;
```

##### `files`

Attaches file uploads to the request.

```rust
pub const fn files(mut self, files: &'a [FileUpload]) -> Self;
```

##### `abort_on`

Cancels the request when the given handle fires.

```rust
pub fn abort_on(mut self, handle: AbortHandle) -> Self;
```

##### `uploads`

The files attached to this request.

```rust
pub const fn uploads(&self) -> &'a [FileUpload];
```

##### `audit_log_reason`

Attaches a pre-encoded audit-log reason header.

```rust
pub const fn audit_log_reason(&self) -> Option<&'a str>;
```

##### `json`

Serializes the body as JSON. Fails on types that cannot serialize.

```rust
pub fn json<B: Serialize + ?Sized>(mut self, body: &B) -> Result<Self, Error>;
```

##### `route`

The route template that failed, when there is one.

```rust
pub const fn route(&self) -> Route<'a>;
```

### `RestBuilder`

Configures a Rest client: auth, base URL, limits, retries, transport, hooks.

```rust
pub struct RestBuilder { /* private fields */ }
```

#### Methods

##### `token`

Authenticates as a bot.

```rust
pub fn token(mut self, token: impl Into<String>) -> Self;
```

##### `bearer_token`

Authenticates as an OAuth2 bearer instead of a bot.

```rust
pub fn bearer_token(mut self, token: impl Into<String>) -> Self;
```

##### `cdn`

Overrides the CDN base URL.

```rust
pub fn cdn(mut self, cdn: impl Into<String>) -> Self;
```

##### `media_proxy`

Overrides the media proxy base URL.

```rust
pub fn media_proxy(mut self, media_proxy: impl Into<String>) -> Self;
```

##### `api`

Overrides the API base. A trailing /vN version is stripped and replaced by version().

```rust
pub fn api(mut self, api: impl Into<String>) -> Self;
```

##### `version`

Pins the API major version appended to every request. Defaults to the library's API_VERSION.

```rust
pub const fn version(mut self, version: u8) -> Self;
```

##### `timeout`

Per-request timeout. Timed-out requests count as retries when retries remain.

```rust
pub const fn timeout(mut self, timeout: Duration) -> Self;
```

##### `retries`

How many times a failed request is retried. 0 disables retries.

```rust
pub const fn retries(mut self, retries: u8) -> Self;
```

##### `retry_policy`

Replaces the retry behavior wholesale: attempts, base delay, cap and jitter.

```rust
pub fn retry_policy(mut self, policy: RetryPolicy) -> Self;
```

##### `on_rate_limit`

Hook called on every 429 with route, scope and retry delay. Return Abort to surface the error instead of waiting.

```rust
pub fn on_rate_limit( mut self, hook: impl Fn(&RateLimited) -> RateLimitAction + Send + Sync + 'static, ) -> Self;
```

##### `global_requests_per_second`

Global throughput cap across all buckets.

```rust
pub const fn global_requests_per_second(mut self, limit: u32) -> Self;
```

##### `offset`

Safety margin added to every rate-limit wait.

```rust
pub const fn offset(mut self, offset: Duration) -> Self;
```

##### `invalid_request_limit`

Refuses to send once this many invalid responses arrived in 10 minutes. 0 disables the guard.

```rust
pub const fn invalid_request_limit(mut self, limit: u32) -> Self;
```

##### `user_agent_appendix`

Appends a product token to the User-Agent.

```rust
pub fn user_agent_appendix(mut self, appendix: impl Into<String>) -> Self;
```

##### `transport`

Brings your own HTTP transport. Required when the hyper feature is off.

```rust
pub fn transport(mut self, transport: impl Transport) -> Self;
```

##### `build`

Builds the client. Nothing is sent until the first request.

```rust
pub fn build(self) -> Rest;
```


## `rest.transport`

### `BoxFuture`

A boxed future: the transport's return type.

```rust
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
```

### `TransportKind`

Which transport is driving this client: hyper or a custom one.

```rust
pub enum TransportKind
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Hyper` | `,` | Hyper. |
| `Custom` | `,` | Custom. |

### `transport_kind`

Reports which transport the build uses.

```rust
pub const fn transport_kind() -> TransportKind;
```

### `Transport`

Sending failed or the connection broke mid-flight.

```rust
pub trait Transport: Send + Sync + 'static;
```

#### Methods

##### `send`

Sends one request, returns the raw response.

```rust
fn send( &self, request: http::Request<Body>, ) -> BoxFuture<'_, Result<http::Response<Bytes>, Box<dyn Error + Send + Sync>>>;
```

##### `sleep`

Sleeps without blocking the runtime. Rate-limit waits go through here, so custom transports stay honest in tests.

```rust
fn sleep(&self, duration: Duration) -> BoxFuture<'static, ()>;
```

### `Body`

A request body as byte chunks, cheap to move and concatenate.

```rust
pub struct Body { /* private fields */ }
```

#### Methods

##### `chunks`

The body as slices, in order.

```rust
pub fn chunks(&self) -> &[Bytes];
```

##### `len`

Total body length in bytes.

```rust
pub fn len(&self) -> usize;
```

##### `is_empty`

Whether there is nothing to send.

```rust
pub fn is_empty(&self) -> bool;
```

##### `into_bytes`

Concatenates the chunks into one buffer.

```rust
pub fn into_bytes(mut self) -> Bytes;
```

## `rest.metrics`

### `RestMetrics`

Atomic counters for the request pipeline: totals, status classes and retries.

```rust
pub struct RestMetrics { /* private fields */ }
```

#### Methods

##### `record_request`

Counts one attempted request.

```rust
pub fn record_request(&self);
```

##### `record_status`

Counts one response by status class.

```rust
pub fn record_status(&self, status: u16);
```

##### `record_retry`

Counts one scheduled retry.

```rust
pub fn record_retry(&self);
```

##### `render_prometheus`

Renders every counter in Prometheus exposition format.

```rust
pub fn render_prometheus(&self) -> String;
```

## `rest.error`

### `Error`

Every REST failure: what kind, its HTTP status, route, parsed Discord error and up to 4KiB of raw body.

```rust
pub struct Error { /* private fields */ }
```

#### Methods

##### `kind`

The classified failure.

```rust
pub const fn kind(&self) -> &ErrorKind;
```

##### `into_parts`

Splits the error into its kind and the underlying transport source, if any.

```rust
pub fn into_parts(self) -> (ErrorKind, Option<Box<dyn std::error::Error + Send + Sync>>);
```

##### `status`

The HTTP status, when the failure came with a response.

```rust
pub fn status(&self) -> Option<StatusCode>;
```

##### `route`

The route template that failed, when there is one.

```rust
pub fn route(&self) -> Option<&'static str>;
```

##### `api_error`

The parsed Discord error body, when the response carried one.

```rust
pub fn api_error(&self) -> Option<&ApiError>;
```

##### `body`

The raw response bytes, capped at 4KiB, when a body was kept.

```rust
pub fn body(&self) -> Option<&[u8]>;
```

##### `discord_code`

Discord's numeric error code, when the response carried one.

```rust
pub fn discord_code(&self) -> Option<ErrorCode>;
```

##### `is_rate_limited`

Whether this is a 429.

```rust
pub fn is_rate_limited(&self) -> bool;
```

##### `is_retryable`

Whether retrying makes sense: 429s, 5xx, transport failures and timeouts.

```rust
pub fn is_retryable(&self) -> bool;
```

### `ErrorKind`

Possible error kind values.

```rust
pub enum ErrorKind
```

| Variant | Payload | Description |
| --- | --- | --- |
| `BuildRequest` | `,` | Build Request. |
| `Transport` | `,` | Transport. |
| `Timeout` | `,` | Timeout. |
| `Aborted` | `,` | Aborted. |
| `Response` | `status: StatusCode, route: &'static str, error: Option<Box<ApiError>>, body: Option<Box<[u8]>>,` | Response. |
| `Deserialize` | `,` | Deserialize. |
| `NotYetIndexed` | `retry_after: Duration, documents_indexed: u64,` | Not Yet Indexed. |
| `InvalidRequestLimit` | `count: u32, limit: u32` | Invalid Request Limit. |

### `ApiError`

Discord's error object: numeric code, message, and flattened field errors.

```rust
pub struct ApiError
```

| Field | Type | Description |
| --- | --- | --- |
| `code` | `ErrorCode` | The code. |
| `message` | `String` | The message. |
| `field_errors` | `Vec<FieldError>` | The field errors. |
| `errors` | `Option<Value>` | The errors. |

### `FieldError`

One validation failure: where it is, which rule broke, and what Discord said.

```rust
pub struct FieldError
```

| Field | Type | Description |
| --- | --- | --- |
| `path` | `String` | The path. |
| `code` | `String` | The code. |
| `message` | `String` | The message. |


## `rest.route`


### `ReactionEmoji`

An emoji reference for reaction endpoints: unicode or custom id.

```rust
pub enum ReactionEmoji<'a>
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Unicode` | `&'a str` | Unicode. |
| `Custom` | `name: &'a str, id: EmojiId` | Custom. |





## `rest.query`



































## `rest.multipart`



## `rest.cdn`

### `DEFAULT_CDN`

The default CDN host Discord serves assets from.

```rust
pub const DEFAULT_CDN: &str = "https://cdn.discordapp.com";
```

### `DEFAULT_MEDIA_PROXY`

The default media proxy host.

```rust
pub const DEFAULT_MEDIA_PROXY: &str = "https://media.discordapp.net";
```

### `ImageFormat`

PNG, JPEG, WebP, GIF or Lottie, validated against what Discord serves.

```rust
pub enum ImageFormat
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Webp` | `,` | Webp. |
| `Png` | `,` | Png. |
| `Jpeg` | `,` | Jpeg. |
| `Gif` | `,` | Gif. |

#### Methods

##### `extension`

Returns the file extension for the image format.

```rust
pub const fn extension(self) -> &'static str;
```

### `ImageSize`

Power-of-two image sizes from 16 to 4096.

```rust
pub enum ImageSize
```

| Variant | Payload | Description |
| --- | --- | --- |
| `S16` | `,` | S 16. |
| `S32` | `,` | S 32. |
| `S64` | `,` | S 64. |
| `S128` | `,` | S 128. |
| `S256` | `,` | S 256. |
| `S512` | `,` | S 512. |
| `S1024` | `,` | S 1024. |
| `S2048` | `,` | S 2048. |
| `S4096` | `,` | S 4096. |

#### Methods

##### `pixels`

Returns the longest edge in pixels.

```rust
pub const fn pixels(self) -> u16;
```

##### `from_pixels`

Builds an image size from a pixel value.

```rust
pub const fn from_pixels(pixels: u16) -> Option<Self>;
```

### `ImageOptions`

Image format and size options for CDN URLs.

```rust
pub struct ImageOptions
```

| Field | Type | Description |
| --- | --- | --- |
| `format` | `Option<ImageFormat>` | The format. |
| `size` | `Option<ImageSize>` | The size. |
| `force_static` | `bool` | The force static. |

#### Methods

##### `new`

Builds a client from a bot token with default settings.

```rust
pub const fn new() -> Self;
```

##### `format`

Returns the image format.

```rust
pub const fn format(mut self, format: ImageFormat) -> Self;
```

##### `size`

Returns the number of entries.

```rust
pub const fn size(mut self, size: ImageSize) -> Self;
```

##### `force_static`

Requests a static image instead of an animated one.

```rust
pub const fn force_static(mut self, force_static: bool) -> Self;
```

### `default_avatar_index`

```rust
pub const fn default_avatar_index(user_id: UserId, discriminator: u16) -> u8;
```

### `Cdn`

Builds asset URLs: avatars, icons, banners, emoji, stickers, sounds.

```rust
pub struct Cdn { /* private fields */ }
```

#### Methods

##### `new`

Points the builder at a CDN host and media proxy.

```rust
pub fn new(cdn: impl Into<String>, media_proxy: impl Into<String>) -> Self;
```

##### `base`

The CDN host in use.

```rust
pub fn base(&self) -> &str;
```

##### `media_proxy`

The media proxy host in use.

```rust
pub fn media_proxy(&self) -> &str;
```

##### `emoji`

Returns the emoji URL.

```rust
pub fn emoji(&self, emoji_id: EmojiId, animated: bool, options: ImageOptions) -> String;
```

##### `guild_icon`

Returns the guild icon URL.

```rust
pub fn guild_icon(&self, guild_id: GuildId, hash: ImageHash, options: ImageOptions) -> String;
```

##### `guild_splash`

Returns the guild splash URL.

```rust
pub fn guild_splash( &self, guild_id: GuildId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `guild_discovery_splash`

Returns the guild discovery splash URL.

```rust
pub fn guild_discovery_splash( &self, guild_id: GuildId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `guild_banner`

Returns the guild banner URL.

```rust
pub fn guild_banner( &self, guild_id: GuildId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `user_banner`

Returns the user banner URL.

```rust
pub fn user_banner(&self, user_id: UserId, hash: ImageHash, options: ImageOptions) -> String;
```

##### `default_user_avatar`

Returns the default avatar URL for the discriminator.

```rust
pub fn default_user_avatar(&self, index: u8) -> String;
```

##### `user_avatar`

Returns the user avatar URL.

```rust
pub fn user_avatar(&self, user_id: UserId, hash: ImageHash, options: ImageOptions) -> String;
```

##### `guild_member_avatar`

Returns the guild member avatar URL.

```rust
pub fn guild_member_avatar( &self, guild_id: GuildId, user_id: UserId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `guild_member_banner`

Returns the guild member banner URL.

```rust
pub fn guild_member_banner( &self, guild_id: GuildId, user_id: UserId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `avatar_decoration`

Returns the avatar decoration URL.

```rust
pub fn avatar_decoration(&self, asset: ImageHash) -> String;
```

##### `application_icon`

Returns the application icon URL.

```rust
pub fn application_icon( &self, application_id: ApplicationId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `application_cover`

Returns the application cover URL.

```rust
pub fn application_cover( &self, application_id: ApplicationId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `application_asset`

Returns the application asset URL.

```rust
pub fn application_asset( &self, application_id: ApplicationId, asset_id: &str, options: ImageOptions, ) -> String;
```

##### `achievement_icon`

Returns the achievement icon URL.

```rust
pub fn achievement_icon( &self, application_id: ApplicationId, achievement_id: GenericId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `store_page_asset`

Stores page asset in the cache.

```rust
pub fn store_page_asset( &self, application_id: ApplicationId, asset_id: GenericId, options: ImageOptions, ) -> String;
```

##### `sticker_pack_banner`

Returns the sticker-pack banner URL.

```rust
pub fn sticker_pack_banner(&self, banner_asset_id: GenericId, options: ImageOptions) -> String;
```

##### `team_icon`

Returns the team icon URL.

```rust
pub fn team_icon(&self, team_id: TeamId, hash: ImageHash, options: ImageOptions) -> String;
```

##### `sticker`

Returns the sticker manager.

```rust
pub fn sticker(&self, sticker_id: StickerId, format_type: StickerFormatType) -> String;
```

##### `role_icon`

Returns the role icon URL.

```rust
pub fn role_icon(&self, role_id: RoleId, hash: ImageHash, options: ImageOptions) -> String;
```

##### `guild_scheduled_event_cover`

Returns the scheduled-event cover URL.

```rust
pub fn guild_scheduled_event_cover( &self, event_id: ScheduledEventId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `guild_tag_badge`

Returns the guild tag badge URL.

```rust
pub fn guild_tag_badge( &self, guild_id: GuildId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `channel_icon`

Returns the channel icon URL.

```rust
pub fn channel_icon( &self, channel_id: ChannelId, hash: ImageHash, options: ImageOptions, ) -> String;
```

##### `soundboard_sound`

Returns the soundboard-sound audio URL.

```rust
pub fn soundboard_sound(&self, sound_id: SoundboardSoundId) -> String;
```

##### `guild_widget_json`

Returns the guild widget JSON URL.

```rust
pub fn guild_widget_json(&self, guild_id: GuildId) -> String;
```

##### `guild_widget_png`

Returns the guild widget image URL.

```rust
pub fn guild_widget_png(&self, guild_id: GuildId) -> String;
```

## `rest.oauth2`

### `AUTHORIZE_URL`

The OAuth2 authorization endpoint every authorize URL is built on.

```rust
pub const AUTHORIZE_URL: &str = "https://discord.com/oauth2/authorize";
```

### `OAuth2Credentials`

A client id plus secret pair for the OAuth2 token endpoints. Debug output redacts the secret.

```rust
pub struct OAuth2Credentials { /* private fields */ }
```

#### Methods

##### `new`

Bundles the id and secret.

```rust
pub fn new(client_id: ApplicationId, client_secret: impl Into<String>) -> Self;
```

##### `client_id`

The application id.

```rust
pub const fn client_id(&self) -> ApplicationId;
```

### `TokenTypeHint`

Tells the revocation endpoint what kind of token is being revoked.

```rust
pub enum TokenTypeHint
```

| Variant | Payload | Description |
| --- | --- | --- |
| `AccessToken` | `,` | Access Token. |
| `RefreshToken` | `,` | Refresh Token. |

#### Methods

##### `as_str`

Returns the string form.

```rust
pub const fn as_str(self) -> &'static str;
```

### `Prompt`

Forces the consent screen, or none for a silent check.

```rust
pub enum Prompt
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Consent` | `,` | Consent. |
| `None` | `,` | None. |

### `AuthorizeUrl`

Builds OAuth2 authorize URLs: bots, code grants, scopes, state, redirects, guild pre-selects.

```rust
pub struct AuthorizeUrl { /* private fields */ }
```

#### Methods

##### `new`

Starts an authorize URL for the given application.

```rust
pub const fn new(client_id: ApplicationId) -> Self;
```

##### `bot`

Starts a bot-install URL with the bot scope.

```rust
pub fn bot(client_id: ApplicationId) -> Self;
```

##### `code`

Starts an authorization-code URL with a redirect.

```rust
pub fn code(client_id: ApplicationId, redirect_uri: impl Into<String>) -> Self;
```

##### `scopes`

Adds the requested scopes.

```rust
pub fn scopes(mut self, scopes: impl IntoIterator<Item = Scope>) -> Self;
```

##### `state`

Adds the round-tripped state parameter.

```rust
pub fn state(mut self, state: impl Into<String>) -> Self;
```

##### `redirect_uri`

Sets where Discord sends the user back.

```rust
pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self;
```

##### `prompt`

Forces consent or stays silent.

```rust
pub const fn prompt(mut self, prompt: Prompt) -> Self;
```

##### `permissions`

 Pre-selects the bot permissions on the install screen.

```rust
pub const fn permissions(mut self, permissions: Permissions) -> Self;
```

##### `guild_id`

Pre-selects the guild on the install screen.

```rust
pub const fn guild_id(mut self, guild_id: GuildId) -> Self;
```

##### `disable_guild_select`

Locks the guild picker to the given guild.

```rust
pub const fn disable_guild_select(mut self, disable: bool) -> Self;
```

##### `integration_type`

Targets a user install instead of a guild install.

```rust
pub const fn integration_type(mut self, integration_type: IntegrationType) -> Self;
```

##### `build`

Renders the final URL.

```rust
pub fn build(&self) -> String;
```


## `rest.ratelimit`






