# Events and handlers

Everything your bot reacts to arrives as a typed dispatch: `Events::MessageCreate`, `Events::InteractionCreate`, `Events::ClientReady`, and dozens more. You subscribe with `client.on`:

```rust
client.on(Events::MessageCreate, ping);
```

A handler takes a `Context` and the event payload, and returns a `Result`. The payload arrives in an `Arc`, so cloning it into spawned tasks is cheap.

## One-shot vs every time

Most subscriptions fire on every matching event. For ready logic, that is exactly once per session by nature:

```rust
client.on(Events::ClientReady, |_ctx, ready| async move {
    println!("Logged in as {}", ready.user.tag());
    Ok(())
});
```

## Waiting for one thing

When you need the *next* matching event instead of all of them, use a collector. This waits up to 30 seconds for the author's next message and then stops:

```rust
let answers = ctx
    .await_messages(msg.channel_id)
    .filter(move |reply| reply.author.id == msg.author.id)
    .max(1)
    .time(std::time::Duration::from_secs(30))
    .await;
```

Collectors also cover button clicks, reactions, and modals. See [buttons and collectors](components.md).

## The event stream

If you prefer pulling events yourself instead of callbacks, `client.events()` gives you the raw stream:

```rust
let mut events = client.events();
while let Some(event) = events.next().await {
    println!("{event:?}");
}
```

Useful for logging, metrics, or driving your own dispatch.
