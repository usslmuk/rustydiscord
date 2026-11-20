# Opcodes

Which gateway opcodes may be sent or received right now.

### `GATEWAY_SEND_ALLOWLIST`

```rust
pub const GATEWAY_SEND_ALLOWLIST: &[u8] = &[1, 2, 3, 4, 6, 8];
```

### `GATEWAY_RECEIVE_ALLOWLIST`

```rust
pub const GATEWAY_RECEIVE_ALLOWLIST: &[u8] = &[0, 1, 7, 9, 10, 11];
```

### `VOICE_SEND_ALLOWLIST`

```rust
pub const VOICE_SEND_ALLOWLIST: &[u8] = &[0, 1, 3, 5, 7];
```

### `VOICE_RECEIVE_ALLOWLIST`

```rust
pub const VOICE_RECEIVE_ALLOWLIST: &[u8] = &[2, 4, 5, 6, 8, 9];
```

### `is_gateway_send_allowed`

Reports whether the gateway currently accepts the given opcode from clients.

```rust
pub fn is_gateway_send_allowed(opcode: u8) -> bool;
```

### `is_gateway_receive_allowed`

Reports whether the gateway may send the given opcode to clients.

```rust
pub fn is_gateway_receive_allowed(opcode: u8) -> bool;
```

### `is_voice_send_allowed`

```rust
pub fn is_voice_send_allowed(opcode: u8) -> bool;
```

### `is_voice_receive_allowed`

```rust
pub fn is_voice_receive_allowed(opcode: u8) -> bool;
```
