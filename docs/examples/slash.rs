use std::sync::Arc;

use rustydiscord::builders::SlashCommandBuilder;
use rustydiscord::prelude::*;

fn ping_command(client: &Client) {
    client.command("ping", |ctx, cmd| async move {
        cmd.reply(&ctx, "Pong!").await?;
        Ok(())
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(std::env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILDS)
        .build();

    let commands = Arc::new(vec![
        SlashCommandBuilder::new("ping", "Replies with Pong!").build()?,
    ]);
    client.on(Events::ClientReady, move |ctx, ready| {
        let commands = Arc::clone(&commands);
        async move {
            let guild_id = ready
                .guilds
                .first()
                .ok_or("the bot is in no guild")?
                .id;
            let registered = ctx.guild_commands(guild_id).set(&commands).await?;
            println!(
                "Logged in as {}; commands live in guild {guild_id}.",
                ready.user.tag()
            );
            println!("Registered: {}", registered.len());
            Ok(())
        }
    });
    ping_command(&client);

    client.login().await?;
    Ok(())
}
