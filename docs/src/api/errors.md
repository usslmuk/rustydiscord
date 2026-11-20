# Errors and handlers

The crate error and handler failures.

## `error`

### `Error`

Every failure the crate can surface: a REST request or the gateway connection.

```rust
pub enum Error
```

| Variant | Payload | Description |
| --- | --- | --- |
| `Rest` | `#[from] crate::rest::Error` | A REST request failed. The inner error says whether it was transport, HTTP, deserialization or rate limiting. |
| `Gateway` | `#[from] crate::gateway::ReceiveError` | The gateway connection failed. The inner error says whether receive or send broke. |

## `handler`

### `HandlerError`

The boxed error a handler may return. Handlers are closures that yield `Result<(), HandlerError>`.

```rust
pub type HandlerError = Box<dyn std::error::Error + Send + Sync>;
```


