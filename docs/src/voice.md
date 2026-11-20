# Voice

Join a channel, play audio, listen, leave. You need the `voice` feature and the `GUILD_VOICE_STATES` intent, without the intent, joins time out because the bot never sees its own voice state.

```rust
{{#include ../examples/voice.rs}}
```

What is happening:

- `get_guild_channels` lists the server's channels; the first `Channel::Voice` wins.
- `client.voice().join(guild_id, channel_id)` performs the full handshake (voice state + voice server + encrypted UDP) and returns a `Call`. It errors if the handshake fails, which is why this example reports it instead of unwrapping.
- `Input::pcm(...)` feeds raw 48kHz stereo `s16le` samples. `Input::ogg(...)` and `Input::ffmpeg(...)` cover files and URLs when ffmpeg is installed.
- `call.subscribe(user_id)` opens the receive path. Packets arrive as Opus frames or decoded PCM.
- `call.leave().await` disconnects cleanly. Without it the bot sits in the channel forever.

> **Tip:** decode on the fly by subscribing with `ReceiveOptions` instead of raw packets when you want audio you can process.
