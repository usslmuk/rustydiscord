# Errors, retries and rate limits

REST calls fail for ordinary reasons: bad input, missing permissions, Discord having a moment. The error tells you which.

## Reading an error

```rust
let error = rest.get_guild_channels(guild_id).await.unwrap_err();
if error.is_rate_limited() {
    println!("Slow down, retry later");
} else if error.is_retryable() {
    println!("Transient, safe to retry");
} else {
    println!("Hard failure on {:?}: {error}", error.route());
}
```

`error.status()` gives the HTTP status, `error.discord_code()` the Discord error code, `error.body()` the raw response body. The `Display` output already includes the route and the Discord message.

## What retries automatically

- **429 rate limit**: waits out `retry_after`, then retries. Global 429s pause everything; unexpected ones only hold the retried request.
- **5xx and network failures**: exponential backoff with jitter, capped at 30 seconds.
- **Anything else**: returned immediately, never retried.

Retries are bounded (3 by default). Tune or disable them with a policy:

```rust
use rustydiscord::rest::RetryPolicy;

let policy = RetryPolicy {
    max_retries: 0,
    ..Default::default()
};
```

## Watching rate limits

React to 429s instead of just waiting through them:

```rust
use rustydiscord::rest::{RateLimitAction, RateLimited};

fn on_limit(info: &RateLimited) -> RateLimitAction {
    println!("429 on {} for {:?}, global: {}", info.route, info.retry_after, info.global);
    RateLimitAction::Wait
}

let rest = rustydiscord::rest::RestBuilder::default()
    .on_rate_limit(on_limit)
    .build();
```

Return `RateLimitAction::Abort` to surface the 429 as an error instead.

## Invalid requests

401s, 403s and user-scope 429s count toward Discord's 10k-per-10-minutes ban window. The client tracks them, `rest.invalid_requests()` reads the current count, and `RestBuilder::invalid_request_limit(n)` refuses to send once the count passes your limit instead of marching into a ban.
