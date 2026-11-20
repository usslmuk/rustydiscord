use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;

use rustydiscord::model::channel::Channel;
use rustydiscord::model::gateway::payload::MessageCreate;
use rustydiscord::prelude::*;
use rustydiscord::voice::VoiceData;

fn sine_tone(seconds: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(48_000 * 2 * 2 * seconds as usize);
    for n in 0..48_000 * seconds {
        let wave = (n as f32 / 48_000.0 * 440.0 * 2.0 * std::f32::consts::PI).sin();
        let sample = (wave * 20_000.0) as i16;
        out.extend_from_slice(&sample.to_le_bytes());
        out.extend_from_slice(&sample.to_le_bytes());
    }
    out
}

async fn join(ctx: Context, msg: Arc<MessageCreate>, client: Client) -> Result<(), HandlerError> {
    let Some(guild_id) = msg.guild_id else {
        msg.reply(&ctx, "Use `!join` inside a server.").await?;
        return Ok(());
    };
    let channels = ctx.rest().get_guild_channels(guild_id).await?;
    let Some(channel_id) = channels.iter().find_map(|channel| match channel {
        Channel::Voice(voice) => Some(voice.id),
        _ => None,
    }) else {
        msg.reply(&ctx, "No voice channel found.").await?;
        return Ok(());
    };

    let call = match client.voice().join(guild_id, channel_id).await {
        Ok(call) => call,
        Err(error) => {
            msg.reply(&ctx, format!("Could not join: {error}.")).await?;
            return Ok(());
        }
    };
    msg.reply(&ctx, "Joined. Playing a tone, then listening for 10 seconds.")
        .await?;
    let tone = rustydiscord::voice::Input::pcm(Cursor::new(sine_tone(5)));
    let _track = call.play(tone);

    let mut stream = call.subscribe(msg.author.id);
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut packets: u64 = 0;
    let mut bytes: u64 = 0;
    loop {
        let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
        if remaining.is_zero() {
            break;
        }
        let Ok(packet) = tokio::time::timeout(remaining, stream.next()).await else {
            break;
        };
        let Some(packet) = packet else {
            break;
        };
        packets += 1;
        match &packet.audio {
            VoiceData::Opus(data) => bytes += data.len() as u64,
            VoiceData::Pcm(samples) => bytes += samples.len() as u64 * 2,
            _ => {}
        }
    }
    msg.reply(&ctx, format!("Heard {packets} packets ({bytes} bytes)."))
        .await?;
    call.leave().await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(std::env::var("DISCORD_TOKEN")?)
        .intents(
            Intents::GUILDS
                | Intents::GUILD_MESSAGES
                | Intents::MESSAGE_CONTENT
                | Intents::GUILD_VOICE_STATES,
        )
        .build();

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!("Logged in as {}. Send `!join`.", ready.user.tag());
        Ok(())
    });
    let voice = client.clone();
    client.on(Events::MessageCreate, move |ctx, msg| {
        let voice = voice.clone();
        async move {
            if msg.content.trim() == "!join" {
                join(ctx, msg, voice).await
            } else {
                Ok(())
            }
        }
    });

    client.login().await?;
    Ok(())
}
