<p align="center">
  <img width="256" height="256" alt="rustydiscord Logo" src="https://github.com/user-attachments/assets/594e55cb-66a9-4863-825c-8345adda240e" />
</p>
<div align="center">
  <h1>rustydiscord</h1>
  <p>A fast, typed framework for building Discord bots in Rust</p>
</div>

This is a long focused Project and it is in BETA right now. It can handle very stressful situations without any problems (has been tested).
Please report any bugs / suggestions, and if you want, you can also leave a Pull Request.

Requires Rust 1.89+.

## Installation

```sh
cargo add rustydiscord
```

```toml
[dependencies]
rustydiscord = "0.0.2"
```

## Example usage

```rust
use std::sync::Arc;

use rustydiscord::model::gateway::payload::MessageCreate;
use rustydiscord::prelude::*;

#[rustydiscord::handler]
async fn ping(ctx: Context, msg: Arc<MessageCreate>) {
    if msg.content == "!ping" {
        let reply = msg.reply(&ctx, "Pong!").await?;
        println!("Replied with {}", reply.id);
    }
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

Initially forked from https://github.com/kotoanidev/djs-rs ^^
