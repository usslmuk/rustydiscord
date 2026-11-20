# Formatters

Discord's markdown, mentions and timestamps as typed values that render themselves:

```rust
{{#include ../examples/formatters.rs}}
```

What is covered:

- **Mentions**: `user_mention`, `role_mention`, `channel_mention`, plus slash-command and emoji mentions. They render as `<@id>`, `<#id>` and friends.
- **Timestamps**: `time(...)` with a `TimestampStyle` (relative, short/long date, time). Discord renders them in each reader's timezone and locale.
- **Markdown**: `bold`, `italic`, `code_block`, `quote`, `spoiler` and the rest as wrappers, so you stop hand-rolling asterisks.
- **Escaping**: `escape_markdown` and friends for user input that must print literally. Always escape untrusted text before embedding it, otherwise a username can rewrite your message layout.

Formatters are pure values implementing `Display`. They compose with `format!` and work anywhere a `String` does.
