# Slash commands

Slash commands are registered with Discord, then handled by name when users invoke them.

## Registering

Register once your bot is ready. Guild commands appear instantly, which makes them perfect for testing:

```rust
{{#include ../examples/slash.rs}}
```

## Reading options

Use `cmd.get_string(...)`, `cmd.get_integer(...)`, `cmd.get_boolean(...)` and friends inside a `client.command(...)` handler. Full example with options, subcommands, autocomplete and deferred replies lives in the repository at `examples/slash_commands.rs`.

## Replies

Slash command replies use `MessageCallbackData`:

```rust
cmd.reply(&ctx, "Pong!").await?;
```

For work that takes longer than three seconds, acknowledge first:

```rust
cmd.defer_reply(&ctx, MessageFlags::empty()).await?;
// ... slow work ...
cmd.edit_reply(&ctx, "Done.").await?;
```

Ephemeral replies (only the invoker sees them) are one call:

```rust
cmd.reply(&ctx, MessageCallbackData::from("Just for you.").ephemeral()).await?;
```
