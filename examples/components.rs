use std::time::Duration;

use rustydiscord::{
    builders::{ActionRowBuilder, ButtonBuilder, EmbedBuilder, ValidationError},
    collector::{CollectedReaction, EndReason},
    model::{
        component::Component,
        emoji::PartialEmoji,
        interaction::MessageCallbackData,
        message::{Embed, Message},
        rest::{CreateMessage, EditMessage},
    },
    prelude::*,
};

#[path = "common/mod.rs"]
mod common;

const PAGES: &[(&str, &str)] = &[
    (
        "Collectors",
        "A collector gathers the messages, reactions, clicks or modals that arrive where it \
         watches, until a time, an idle time, a count or `stop()`.",
    ),
    (
        "Limits",
        "`time` ends it after a duration, `idle` after a pause, `max` after a number of items; \
         deleting the message, channel or guild ends it too.",
    ),
    (
        "Cost",
        "Events reach collectors through an index by ID: a thousand open menus cost an event \
         one lookup.",
    ),
];

const QUESTIONS: &[(&str, &str)] = &[
    ("What is 6 × 7?", "42"),
    ("How many bits does a Discord snowflake have?", "64"),
    ("Which Discord API version does this library target?", "10"),
];

const YES: &str = "👍";
const NO: &str = "👎";

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
        .component(
            ButtonBuilder::danger("close")
                .label("Close")
                .disabled(closed),
        )
        .build()?;
    Ok((embed, buttons))
}

async fn menu(ctx: Context, msg: &Message) -> Result<(), HandlerError> {
    let (embed, buttons) = page(0, false)?;
    let body = CreateMessage::default().embed(embed).component(buttons);
    let menu = msg.reply(&ctx, body).await?;

    let mut clicks = menu
        .component_collector(&ctx)
        .idle(Duration::from_secs(60))
        .build();
    let mut index: usize = 0;
    while let Some(click) = clicks.next().await {
        if click.interaction().author().map(|user| user.id) != Some(msg.author.id) {
            click
                .reply(
                    &ctx,
                    MessageCallbackData::from("Only the author of `!menu` can turn its pages.")
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

async fn quiz(ctx: Context, msg: &Message) -> Result<(), HandlerError> {
    let index = msg.id.get() % u64::try_from(QUESTIONS.len())?;
    let (question, answer) = QUESTIONS[usize::try_from(index)?];
    msg.reply(&ctx, format!("{question} (30 seconds)")).await?;
    let author = msg.author.id;
    let answers = ctx
        .await_messages(msg.channel_id)
        .filter(move |reply| reply.author.id == author)
        .max(1)
        .time(Duration::from_secs(30))
        .await;
    let verdict = match answers.first() {
        Some(reply) if reply.content.trim() == answer => "Correct!".to_owned(),
        Some(_) => format!("No, it is {answer}."),
        None => format!("Time is up: it was {answer}."),
    };
    msg.reply(&ctx, verdict).await?;
    Ok(())
}

async fn poll(ctx: Context, msg: &Message, question: &str) -> Result<(), HandlerError> {
    let body = CreateMessage::from(format!(
        "**{question}**\nReact with {YES} or {NO}: the result comes in 30 seconds."
    ))
    .no_mentions();
    let poll = msg.reply(&ctx, body).await?;
    poll.react(&ctx, YES).await?;
    poll.react(&ctx, NO).await?;

    let bot = poll.author.id;
    let votes = poll
        .await_reactions(&ctx)
        .filter(move |reaction| {
            reaction.user_id() != Some(bot)
                && matches!(
                    reaction.emoji(),
                    Some(PartialEmoji::Unicode { name, .. }) if name == YES || name == NO
                )
        })
        .dispose(true)
        .time(Duration::from_secs(30))
        .await;
    let count = |emoji: &str| votes.get(emoji).map_or(0, CollectedReaction::count);
    poll.reply(
        &ctx,
        format!("Result: {YES} {} — {NO} {}", count(YES), count(NO)),
    )
    .await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::init_tracing();

    let client = Client::builder(common::token()?)
        .intents(
            Intents::GUILDS
                | Intents::GUILD_MESSAGES
                | Intents::MESSAGE_CONTENT
                | Intents::GUILD_MESSAGE_REACTIONS,
        )
        .build();

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!(
            "Logged in as {}. Send `!menu`, `!quiz` or `!poll <question>`.",
            ready.user.tag()
        );
        Ok(())
    });

    client.on(Events::MessageCreate, |ctx, msg| async move {
        let content = msg.content.trim();
        if content == "!menu" {
            menu(ctx, &msg).await
        } else if content == "!quiz" {
            quiz(ctx, &msg).await
        } else if let Some(question) = content.strip_prefix("!poll ") {
            poll(ctx, &msg, question.trim()).await
        } else {
            Ok(())
        }
    });

    common::shutdown_on_ctrl_c(&client);
    client.login().await?;
    Ok(())
}
