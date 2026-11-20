use rustydiscord::prelude::*;

#[path = "common/mod.rs"]
mod common;

async fn ping_handler(
    ctx: Context,
    msg: std::sync::Arc<rustydiscord::model::gateway::payload::MessageCreate>,
) -> Result<(), rustydiscord::HandlerError> {
    if msg.content == "!ping" {
        let started = std::time::Instant::now();
        let reply = msg.reply(&ctx, "Pong!").await?;
        println!(
            "Replied to {} with message {} in {:?}",
            msg.author.tag(),
            reply.id,
            started.elapsed()
        );
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::init_tracing();

    let client = Client::builder(common::token()?)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .build();

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!(
            "Logged in as {} — in {} guild(s). Send `!ping` to test.",
            ready.user.tag(),
            ready.guilds.len()
        );
        Ok(())
    });

    client.on(Events::MessageCreate, ping_handler);

    common::shutdown_on_ctrl_c(&client);

    client.login().await?;
    Ok(())
}
