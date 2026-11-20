use std::sync::Arc;

use rustydiscord::model::gateway::payload::MessageCreate;
use rustydiscord::prelude::*;

async fn ping(ctx: Context, msg: Arc<MessageCreate>) -> Result<(), HandlerError> {
    if msg.content == "!ping" {
        let reply = msg.reply(&ctx, "Pong!").await?;
        println!("Replied with {}", reply.id);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(std::env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .build();

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!("Logged in as {}", ready.user.tag());
        Ok(())
    });
    client.on(Events::MessageCreate, ping);

    client.login().await?;
    Ok(())
}
