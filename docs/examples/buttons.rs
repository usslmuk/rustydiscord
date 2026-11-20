use std::sync::Arc;

use rustydiscord::builders::{ActionRowBuilder, ButtonBuilder, EmbedBuilder, ValidationError};
use rustydiscord::collector::EndReason;
use rustydiscord::model::component::Component;
use rustydiscord::model::gateway::payload::MessageCreate;
use rustydiscord::model::interaction::MessageCallbackData;
use rustydiscord::model::message::Embed;
use rustydiscord::model::rest::{CreateMessage, EditMessage};
use rustydiscord::prelude::*;

const PAGES: &[(&str, &str)] = &[
    ("First page", "Flip through this menu with the buttons below."),
    ("Second page", "Only the author of !panel can turn these pages."),
    ("Last page", "Closing disables every button."),
];

fn page(index: usize, closed: bool) -> Result<(Embed, Component), ValidationError> {
    let (title, text) = PAGES[index];
    let embed = EmbedBuilder::new()
        .title(title)
        .description(text)
        .footer(format!("Page {} of {}", index + 1, PAGES.len()))
        .build()?;
    let buttons = ActionRowBuilder::new()
        .component(
            ButtonBuilder::secondary("previous")
                .label("Previous")
                .disabled(closed || index == 0),
        )
        .component(
            ButtonBuilder::secondary("next")
                .label("Next")
                .disabled(closed || index + 1 == PAGES.len()),
        )
        .component(ButtonBuilder::danger("close").label("Close").disabled(closed))
        .build()?;
    Ok((embed, buttons))
}

async fn panel(ctx: Context, msg: Arc<MessageCreate>) -> Result<(), HandlerError> {
    let (embed, buttons) = page(0, false)?;
    let body = CreateMessage::default().embed(embed).component(buttons);
    let menu = msg.reply(&ctx, body).await?;

    let mut clicks = menu
        .component_collector(&ctx)
        .idle(std::time::Duration::from_secs(120))
        .build();
    let mut index: usize = 0;
    while let Some(click) = clicks.next().await {
        if click.interaction().author().map(|user| user.id) != Some(msg.author.id) {
            click
                .reply(
                    &ctx,
                    MessageCallbackData::from("Only the author of `!panel` can flip its pages.")
                        .ephemeral(),
                )
                .await?;
            continue;
        }
        let closed = click.custom_id == "close";
        match click.custom_id.as_str() {
            "previous" => index = index.saturating_sub(1),
            "next" => index = (index + 1).min(PAGES.len() - 1),
            _ => clicks.stop(),
        }
        let (embed, buttons) = page(index, closed)?;
        let update = MessageCallbackData::default()
            .embeds([embed])
            .components([buttons]);
        click.update(&ctx, update).await?;
    }

    if clicks.end_reason() == Some(EndReason::Idle) {
        let (_, buttons) = page(index, true)?;
        let edit = EditMessage::default().components([buttons]);
        menu.edit(&ctx, &edit).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder(std::env::var("DISCORD_TOKEN")?)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .build();

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!("Logged in as {}. Send `!panel`.", ready.user.tag());
        Ok(())
    });
    client.on(Events::MessageCreate, |ctx, msg| async move {
        if msg.content.trim() == "!panel" {
            panel(ctx, msg).await
        } else {
            Ok(())
        }
    });

    client.login().await?;
    Ok(())
}
