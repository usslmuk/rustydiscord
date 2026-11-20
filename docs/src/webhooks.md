# Webhooks and invites

## Sending as a webhook

`WebhookClient` posts without logging in as a bot. Create it from the webhook URL and send like a message:

```rust
{{#include ../examples/webhooks.rs}}
```

## Invite links

Build OAuth2 URLs instead of hand-concatenating query strings:

```rust
let url = rustydiscord::rest::oauth2::AuthorizeUrl::bot(application_id)
    .permissions(Permissions::SEND_MESSAGES)
    .scope([rustydiscord::model::oauth2::Scope::ApplicationsCommands])
    .build();
```

`AuthorizeUrl::code(...)` starts the full authorization-code flow with redirect and state handling.

## Image URLs

Avatar, icon, banner and emoji URLs come from the CDN helper on any `Rest`:

```rust
let cdn = rest.cdn();
let avatar = cdn.user_avatar(user_id, hash, Default::default());
let icon = cdn.guild_icon(guild_id, hash, Default::default());
```

Pass `ImageOptions` for size and animated formats. The helpers produce the exact CDN URLs, no guessing file extensions.
