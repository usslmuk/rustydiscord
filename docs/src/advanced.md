# Sharding and HTTP-only bots

## Sharding

One process stops being enough around a few thousand servers. Split the gateway connection into shards:

```rust
use rustydiscord::Shards;

let client = Client::builder(std::env::var("DISCORD_TOKEN")?)
    .intents(Intents::GUILDS)
    .shards(Shards::Count(4))
    .build();
```

`Shards::Auto` asks Discord how many you need. `Shards::Range` pins this process to a slice when you run one process per machine. Everything else, handlers, cache, managers, works the same on every shard.

## HTTP-only bots

If you only need slash commands, skip the gateway entirely. Discord sends interactions to your web server; you verify and answer them:

```rust
let endpoint = client.interactions_endpoint("your-ed25519-public-key")?;
let (response, background) = endpoint
    .handle(signature, timestamp, &body)
    .await;
tokio::spawn(background);
```

`handle` checks the Ed25519 signature and timestamp, decodes the interaction, and routes it through your registered `client.command(...)` handlers. Respond with `response.status` and `response.body` from your HTTP framework. Bad signatures get a 401 without touching your handlers.
