# Changelog

# 0.0.4 - Latest

- Moved pin endpoints to the current `/channels/{id}/messages/pins` paths
- Added the current-member nickname alias endpoint
- Added missing model fields (guild region and owner, create-guild roles, invite expiration, channel icon, role color)
- Pinned the REST base URL to https://discord.com/api/v10 with API_VERSION = 10 used for gateways and routes
- Wired RestMetrics into every request with Rest::metrics and a Prometheus renderer
- Added exponential backoff with jitter for 5xx and transport failures
- Global 429s now pause all traffic and shared scope 429s ride the learned bucket
- Unexpected 429s only delay the retried request while the rest of the bucket keeps flowing
- Added AbortHandle to cancel in-flight requests and an on_rate_limit hook to inspect or abort on 429s
- Added RetryPolicy for configurable retries, backoff base and cap, and jitter
- REST errors carry their route and raw body, with is_rate_limited, is_retryable and discord_code helpers
- Removed the Discord API tracking test and its version constants

# 0.0.3

- Added missing REST endpoints: create/delete guild, member verification get/modify, MFA level, create from template, bulk command permissions, Slack/GitHub webhook execute
- Fixed pin routes to /channels/{id}/pins/{id}
- Added guild widget image CDN helpers
- Hardened ETF translation for eetf 0.12 list encoding
- Full REST implementation

# 0.0.2

Minor fixes

# 0.0.1

Release!