<p align="center">
  <a href="https://github.com/usslmuk/rustydiscord/actions/workflows/rust.yml"><img src="https://github.com/usslmuk/rustydiscord/actions/workflows/rust.yml/badge.svg" alt="CI"></a>
  <a href="https://crates.io/crates/rustydiscord"><img src="https://img.shields.io/crates/v/rustydiscord" alt="crates.io"></a>
  <img src="https://img.shields.io/badge/rust-1.89%2B-orange" alt="Rust 1.89+">
</p>

<p align="center">
  <img width="256" height="256" alt="rustydiscord logo" src="https://github.com/user-attachments/assets/594e55cb-66a9-4863-825c-8345adda240e" />
</p>
<h1 align="center">rustydiscord</h1>
<p align="center">The fastest and most complete Rust library for the Discord API</p>
<p align="center">Discord API v10 and Gateway v10</p>
<p align="center"><a href="https://usslmuk.github.io/rustydiscord/">Documentation</a></p>

## Installation

```sh
cargo add rustydiscord
```

```toml
[dependencies]
rustydiscord = "0.0.4"
```

Requires Rust 1.89 or newer

## Example

```rust
use std::sync::Arc;

use rustydiscord::model::gateway::payload::MessageCreate;
use rustydiscord::prelude::*;

async fn ping(ctx: Context, msg: Arc<MessageCreate>) -> Result<(), HandlerError> {
    if msg.content == "!ping" {
        let reply = msg.reply(&ctx, "Pong!").await?;
        println!("Replied with {}", reply.id);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(std::env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .build();

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!("Logged in as {}", ready.user.tag());
        Ok(())
    });
    client.on(Events::MessageCreate, ping);

    client.login().await?;
    Ok(())
}
```

## Features

- **Gateway**: Sharding, reconnects that pick up where they left off, zlib/zstd/ETF support, every event typed
- **REST**: All 249 API routes typed, rate limits handled per bucket with backoff, request metrics included
- **Cache**: Caches everything the gateway sends, swap the backend if you want
- **Builders**: Commands, components, embeds, modals. Checked before anything hits the API
- **Voice**: Join voice, play and receive audio. Encrypted
- **Collectors and managers**: Wait on messages, buttons and events without writing the plumbing yourself
- **HTTP interactions**: Run a bot on webhooks alone, signatures verified, no gateway needed
- **Formatters**: Timestamps, mentions, markdown, escaping. Done right

## Coverage

100% coverage of the Discord REST API routes and gateway events, matching discord.js

## Benchmarks

| Benchmark | Result |
|---|---|
| Decode 1kB message | 1.5µs |
| Decode 520kB guild | 750µs |
| 1M event flood | 245k events/s |
| REST burst (300k req) | 227k req/s |
| Memory, idle | 6.7MB |
| Memory, 100 guilds | 45MB |
| Memory, 500 guilds | 208MB |
