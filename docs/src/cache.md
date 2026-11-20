# Caching

The client caches what the gateway sends: guilds, channels, roles, members, messages, presences. Reads hit memory instead of the API.

## Turning it on

The default is minimal on purpose, caches cost memory. For a small bot that wants everything, use the recommended preset:

```rust
let client = Client::builder(token)
    .intents(intents)
    .cache(rustydiscord::cache::CacheConfig::rustydiscord())
    .build();
```

That preset keeps every resource unlimited except messages (capped at 200 per channel), with sweeps that prune idle entries the way discord.js does.

## Reading the cache

```rust
if let Some(cache) = ctx.cache() {
    println!("Known guilds: {}", cache.guilds().len());
    println!("Known channels: {}", cache.channels().len());
}
```

Writes go through automatically: every dispatch updates the cache before your handlers run, so what you read is always fresh.

## Tuning it

Build your own config when the preset doesn't fit. Cap what grows without bound (messages, presences) and leave the small stuff unlimited:

```rust
use rustydiscord::cache::{CacheConfig, Limit, Resource};

let config = CacheConfig::builder()
    .limit(Resource::Message, Limit::Max(50))
    .build();
```

The `Resource` enum lists everything cacheable. `Limit::Unlimited`, `Limit::Max(n)` and `Limit::Disabled` cover every policy worth having.
