# Your first bot

A bot that answers `!ping` with `Pong!`. Save it as `src/main.rs`, set `DISCORD_TOKEN`, run `cargo run`.

```rust
{{#include ../examples/ping.rs}}
```

What happens, line by line:

- `Client::builder(TOKEN)` starts the builder. `.intents(...)` declares what Discord should send you.
- `.build()` gives you a `Client`. Nothing connects yet.
- `client.on(Events::ClientReady, ...)` runs once when the session is live.
- `client.on(Events::MessageCreate, ping)` runs `ping` for every message event.
- `client.login().await` connects and stays connected. It resumes dropped sessions on its own.

The handler signature is always the same shape: context, event payload in an `Arc`, and a `Result` return. Returning `Err` logs the error and moves on; it never crashes the bot.

Invite the bot with this URL (replace the client id with yours from the developer portal):

```text
https://discord.com/oauth2/authorize?client_id=YOUR_CLIENT_ID&permissions=8&scope=bot%20applications.commands
```

Next: [events and handlers](events.md).
