# HTTP interactions

Verify and route interactions from a web server, no gateway needed.

## `http_interactions`


### `HttpResponse`

Status plus body to hand back to the HTTP framework.

```rust
pub struct HttpResponse
```

| Field | Type | Description |
| --- | --- | --- |
| `status` | `u16` | The status. |
| `body` | `Vec<u8>` | The body. |

### `InvalidPublicKey`

The public key was not 64 hex digits of a valid Ed25519 key.

```rust
pub struct InvalidPublicKey;
```

### `InteractionsEndpoint`

Verifies incoming HTTP interactions by signature and routes them into the registered handlers. The gateway-free way to run a bot.

```rust
pub struct InteractionsEndpoint<B: Backend = DashMapBackend> { /* private fields */ }
```

#### Methods

##### `handle`

Verifies the Ed25519 signature and timestamp, then dispatches. Returns the HTTP response plus the background work as a future.

```rust
pub async fn handle( &self, signature: &str, timestamp: &str, body: &[u8], ) -> ( HttpResponse, impl Future<Output = ()> + Send + 'static + use<B>, );
```



