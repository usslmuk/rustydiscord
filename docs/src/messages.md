# Messages and embeds

## Replying

`msg.reply` answers in the same channel:

```rust
msg.reply(&ctx, "Pong!").await?;
```

Anything that turns into a message body works: plain text, embeds, components. The reply returns the sent message, so you can chain off it.

## Sending embeds

Build embeds with `EmbedBuilder`. Every method returns the builder, and `build()` validates it before anything hits the network:

```rust
use rustydiscord::builders::EmbedBuilder;

let embed = EmbedBuilder::new()
    .title("Server report")
    .description("Everything is fine.")
    .inline_field("Members", "128")
    .inline_field("Online", "64")
    .footer("Checked just now")
    .build()?;

msg.reply(&ctx, embed).await?;
```

> **Tip:** `build()` fails on invalid input (empty title, too many fields) instead of letting Discord reject it. Handle the `Result` like any other error.

## Reading history

Page through a channel without loading everything at once:

```rust
use rustydiscord::model::rest::GetChannelMessages;

let mut query = GetChannelMessages::default();
query.limit = Some(100);
let history = ctx
    .messages(msg.channel_id)
    .paginate(query)
    .fetch_all()
    .await?;
println!("Saw {} messages", history.len());
```

`fetch_all` follows pagination for you. Each page respects the rate limiter, so bulk reads never get you banned.
