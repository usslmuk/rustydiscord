# Power tools

Small modules that solve one problem each. All of this runs below, copy it into `src/main.rs` and watch it work offline:

```rust
{{#include ../examples/power.rs}}
```

What each piece does:

- **Brokers** (`MemoryBroker`): a typed in-process bus. `publish` returns whether anyone listened; `subscribe` gives a Tokio broadcast receiver. Useful for fanning internal events out to background tasks.
- **Proxy** (`ProxyConfig`): rewrite API URLs to your own host and strip secrets from forwarded headers. Point it at a self-hosted proxy when Discord is unreachable directly.
- **Collections** (`Collection`): the ordered map everything cached lives in. Insert, look up, remove, with deterministic iteration order.
- **Opcodes** (`is_gateway_send_allowed` and friends): ask whether an opcode may be sent or received before you touch the wire.
- **Util** (`Equatable`, `calculate_shard_id`, lazy helpers): small correctness tools. `calculate_shard_id` answers which shard owns a guild for a given shard count.
