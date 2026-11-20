# Builders

Builders construct payloads with validation at build time instead of Discord rejections at runtime. Every builder ends in `.build()`, which returns a `Result`:

```rust
{{#include ../examples/builders.rs}}
```

What to reach for:

- `SlashCommandBuilder` and its option types for commands, `CreateApplicationCommand` is what gets registered.
- `EmbedBuilder` for rich messages. `build()` enforces field counts, lengths and total size.
- `ButtonBuilder` (`primary`, `secondary`, `danger`, `success`, `link`) plus `ActionRowBuilder` for layouts. Link buttons take a URL instead of a custom id.
- `ModalBuilder` with `TextInputBuilder::short` or `::paragraph` for forms.
- Every builder has a `build_unchecked` escape hatch. Prefer `build()`, Discord errors are slower to debug than a local `Result`.

Validation errors are descriptive: they name the field and the violated limit.
