# Calling the REST API directly

Managers cover the common paths. When you need the raw API - all 249 routes are typed - use `Rest` straight:

```rust
{{#include ../examples/rest.rs}}
```

`render_prometheus` exposes request, 2xx, 429, 5xx and retry counters. Point your scraper at it or log it on an interval:

```rust
let metrics = rest.metrics().render_prometheus();
```

## Error handling

Failures carry everything needed to decide what to do:

```rust
match rest.get_guild_channels(guild_id).await {
    Ok(channels) => println!("{} channels", channels.len()),
    Err(error) if error.is_rate_limited() => println!("slow down"),
    Err(error) if error.is_retryable() => println!("transient, retry"),
    Err(error) => println!("hard failure: {error}"),
}
```

`error.route()` names the endpoint, `error.body()` holds the raw response (capped at 4KiB), and `error.discord_code()` gives the numeric Discord error code when Discord sent one.

## Cancelling and custom retry policy

Requests can be aborted mid-flight, and the retry policy is yours to tune:

```rust
let abort = rustydiscord::rest::AbortHandle::new();
let request = rustydiscord::rest::Request::new(
    rustydiscord::rest::Route::GetGatewayBot,
)
.abort_on(abort.clone());
abort.abort();
```

```rust
let rest = rustydiscord::rest::Rest::builder()
    .token(std::env::var("DISCORD_TOKEN")?)
    .retry_policy(rustydiscord::rest::RetryPolicy {
        max_retries: 5,
        ..Default::default()
    })
    .build();
```
