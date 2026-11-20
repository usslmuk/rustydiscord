# rustydiscord

The fastest and most complete Rust library for the Discord API. One crate, typed everything, no macros to learn.

This book takes you from zero to a live bot: setup, events, slash commands, buttons, voice, and calling the API directly. Every full example in this book is compiled against the library, so what you read is what runs.

Start with [Getting started](getting-started.md), or jump straight to [your first bot](first-bot.md).

## What you get

- **Gateway**: sharding, session resume, zlib/zstd/ETF transports, typed dispatches
- **REST**: all 249 API routes typed, bucket-accurate rate limiting with backoff, request metrics
- **Cache**: everything cached, swap the backend if you want
- **Builders**: commands, components, embeds, modals. Checked before anything hits the API
- **Voice**: join voice, play and receive audio. Encrypted
- **Collectors and managers**: wait on messages, buttons and events without writing the plumbing yourself
- **HTTP interactions**: run a bot on webhooks alone, signatures verified, no gateway needed
- **Formatters**: timestamps, mentions, markdown, escaping that just works

Targets Discord API v10 and Gateway v10. Requires Rust 1.89 or newer.
