# Proxy

Run REST traffic through a self-hosted proxy without leaking tokens.

### `ProxyConfig`

Forwards REST traffic through a self-hosted proxy, rewriting URLs and stripping secrets from headers.

```rust
pub struct ProxyConfig
```

| Field | Type | Description |
| --- | --- | --- |
| `target` | `String` | The target. |
| `redact_token` | `bool` | The redact token. |

#### Methods

##### `new`

Points REST traffic at `target`, optionally redacting authorization headers before forwarding.

```rust
pub fn new(target: String, redact_token: bool) -> Self;
```

##### `proxy_url_for`

Joins a request path onto the proxy base URL.

```rust
pub fn proxy_url_for(&self, path: &str) -> String;
```

##### `forward_headers`

Copies headers for forwarding, dropping authorization headers when redaction is on.

```rust
pub fn forward_headers(&self, headers: &[(String, String)]) -> Vec<(String, String)>;
```

##### `forward`

Rewrites one request into its proxied URL plus safe headers.

```rust
pub fn forward( &self, path: &str, headers: &[(String, String)], ) -> (String, Vec<(String, String)>);
```
