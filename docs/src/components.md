# Buttons and collectors

Buttons live on messages as components. Clicks arrive as interactions, and a collector turns them into a stream you loop over.

```rust
{{#include ../examples/buttons.rs}}
```

The moving parts:

- `ActionRowBuilder` + `ButtonBuilder` lay out up to five buttons per row. `custom_id` is how you tell them apart later.
- `component_collector(&ctx)` scopes clicks to that message. `.idle(...)` ends the loop after silence so the bot doesn't listen forever.
- `click.update(&ctx, ...)` edits the original message in place. `click.reply(...)` answers the clicker, optionally `.ephemeral()`.
- When the collector ends, disable the buttons so users don't click a dead menu.

> **Tip:** always check the clicker. Without the author check, anyone in the channel can flip someone else's menu.
