# Getting started

## Prerequisites

Rust 1.89 or newer. Check with:

```sh
rustc --version
```

You also need a Discord application. Create one in the [developer portal](https://discord.com/developers/applications), open the Bot tab, and copy the token. Keep it secret. Every example in this book reads it from the `DISCORD_TOKEN` environment variable:

```sh
export DISCORD_TOKEN="your-token-here"
```

## New project

```sh
cargo new mybot
cd mybot
cargo add rustydiscord tokio --features tokio/full
```

Your `Cargo.toml` needs the library and an async runtime:

```toml
[dependencies]
rustydiscord = "0.0.4"
tokio = { version = "1", features = ["full"] }
```

Voice support is a feature flag. Turn it on when you need it:

```toml
[dependencies]
rustydiscord = { version = "0.0.4", features = ["voice"] }
```

## Intents

Discord only sends your bot the events it asks for. You declare them up front with intents:

```rust
use rustydiscord::model::gateway::Intents;

let intents = Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT;
```

| Intent              | Unlocks                                              |
|---------------------|------------------------------------------------------|
| `GUILDS`            | Servers, channels, roles arriving in cache           |
| `GUILD_MESSAGES`    | Message events in servers                            |
| `MESSAGE_CONTENT`   | The actual text of messages (privileged, enable it on your app page) |
| `GUILD_VOICE_STATES`| Who is in voice, required to join voice yourself     |

> **Tip:** no intent, no events. If a handler never fires, check this list first. Missing `GUILD_VOICE_STATES` is the classic reason voice joins time out.

Next: [your first bot](first-bot.md).
