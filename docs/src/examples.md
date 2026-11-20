# Examples

Every example below is a complete program. They all compile against the library, save one as `src/main.rs`, set `DISCORD_TOKEN`, and run it.

## Ping bot

Replies `Pong!` to `!ping`. The smallest useful bot.

```rust
{{#include ../examples/ping.rs}}
```

## Slash commands

Registers a `/ping` command on startup and answers it:

```rust
{{#include ../examples/slash.rs}}
```

## Buttons

An embed menu with Previous/Next/Close buttons, author-locked, auto-disabled on idle:

```rust
{{#include ../examples/buttons.rs}}
```

## Voice

Joins the first voice channel it finds, plays a tone, listens to you for 10 seconds, then leaves. Requires the `voice` feature and the `GUILD_VOICE_STATES` intent:

```rust
{{#include ../examples/voice.rs}}
```

## Raw REST

No gateway, no handlers, just the API plus metrics:

```rust
{{#include ../examples/rest.rs}}
```
