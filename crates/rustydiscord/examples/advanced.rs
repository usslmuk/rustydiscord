use std::{
    collections::HashMap,
    future::Future,
    sync::{
        Arc, Mutex, PoisonError,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};

use rustydiscord::{
    HandlerMode, Shards, WebhookClient,
    model::{
        gateway::{Activity, Status, payload::UpdatePresence},
        id::ChannelId,
        rest::{CreateWebhook, EditWebhookMessage, ExecuteWebhook},
    },
    prelude::*,
};

#[path = "common/mod.rs"]
mod common;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

struct Stats {
    started: Instant,
    total: AtomicU64,
    by_name: Mutex<HashMap<String, u64>>,
}

impl Stats {
    fn new() -> Self {
        Self {
            started: Instant::now(),
            total: AtomicU64::new(0),
            by_name: Mutex::new(HashMap::new()),
        }
    }

    fn record(&self, name: &str) {
        self.total.fetch_add(1, Ordering::Relaxed);
        let mut by_name = self.by_name.lock().unwrap_or_else(PoisonError::into_inner);
        match by_name.get_mut(name) {
            Some(count) => *count += 1,
            None => {
                by_name.insert(name.to_owned(), 1);
            }
        }
    }

    fn total(&self) -> u64 {
        self.total.load(Ordering::Relaxed)
    }

    fn top(&self, n: usize) -> Vec<(String, u64)> {
        let by_name = self.by_name.lock().unwrap_or_else(PoisonError::into_inner);
        let mut top: Vec<_> = by_name
            .iter()
            .map(|(name, count)| (name.clone(), *count))
            .collect();
        top.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        top.truncate(n);
        top
    }
}

fn spawn_logged(work: impl Future<Output = Result<(), HandlerError>> + Send + 'static) {
    tokio::spawn(async move {
        if let Err(error) = work.await {
            eprintln!("A task failed: {error}");
        }
    });
}

async fn webhook_demo(ctx: Context, channel_id: ChannelId) -> Result<(), HandlerError> {
    const REASON: Option<&str> = Some("discord-rs advanced example");
    let webhook = ctx
        .webhooks()
        .create(
            channel_id,
            &CreateWebhook::new("discord-rs example"),
            REASON,
        )
        .await?;
    let result = async {
        let token = webhook
            .token
            .as_deref()
            .ok_or("the new webhook has no token")?;
        let client = WebhookClient::new(webhook.id, token);
        let body = ExecuteWebhook::from("Sent through a `WebhookClient`: no bot token needed.")
            .username("Webhook demo");
        let message = client.send(body).await?;
        tokio::time::sleep(Duration::from_secs(3)).await;
        let edit = EditWebhookMessage::from("Edited through the same webhook; gone in 5 seconds.");
        client.edit_message(message.id, &edit).await?;
        tokio::time::sleep(Duration::from_secs(5)).await;
        client.delete_message(message.id, None).await?;
        Ok::<_, HandlerError>(())
    }
    .await;

    ctx.webhooks().delete(webhook.id, REASON).await?;
    result
}

fn rotate_presence(client: &Client, stats: Arc<Stats>) {
    let client = client.clone();
    tokio::spawn(async move {
        let mut every_minute = tokio::time::interval(Duration::from_secs(60));
        every_minute.tick().await;
        loop {
            every_minute.tick().await;
            let activity = Activity::watching(format!("{} events", stats.total()));

            if let Err(error) =
                client.set_presence(&UpdatePresence::new(Status::Online, [activity]))
            {
                eprintln!("Could not update the presence: {error}");
            }
        }
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    common::init_tracing();

    if let Some(url) = common::config_value("DISCORD_WEBHOOK_URL") {
        WebhookClient::from_url(&url)?
            .send("The discord-rs advanced example is starting.")
            .await?;
    }

    let client = Client::builder(common::token()?)
        .intents(Intents::GUILDS | Intents::GUILD_MESSAGES | Intents::MESSAGE_CONTENT)
        .shards(Shards::Auto)
        .presence(UpdatePresence::new(
            Status::Idle,
            [Activity::playing("starting up")],
        ))
        .handler_mode(HandlerMode::Sequential)
        .build();

    let stats = Arc::new(Stats::new());
    let mut events = client.events();
    let counter = Arc::clone(&stats);
    tokio::spawn(async move {
        while let Some(event) = events.next().await {
            counter.record(event.event.name());
        }
    });
    rotate_presence(&client, Arc::clone(&stats));

    client.on(Events::ShardReady, |ctx, ready| async move {
        let shard = ctx.shard_id();
        println!(
            "Shard {} of {} is ready with {} guilds.",
            shard.number(),
            shard.total(),
            ready.guilds.len()
        );
        Ok(())
    });

    client.on(Events::ClientReady, |_ctx, ready| async move {
        println!(
            "Logged in as {}: every shard is ready. Send `!stats` or `!webhook`.",
            ready.user.tag()
        );
        Ok(())
    });

    client.on(Events::MessageCreate, move |ctx, msg| {
        let stats = Arc::clone(&stats);
        async move {
            match msg.content.trim() {
                "!stats" => {
                    let top: Vec<_> = stats
                        .top(5)
                        .into_iter()
                        .map(|(name, count)| format!("`{name}` {count}"))
                        .collect();
                    let text = format!(
                        "{} events in {:.0?} on shard {}: {}",
                        stats.total(),
                        stats.started.elapsed(),
                        ctx.shard_id().number(),
                        top.join(" · ")
                    );
                    spawn_logged(async move {
                        msg.reply(&ctx, text).await?;
                        Ok(())
                    });
                }
                "!webhook" => {
                    let channel_id = msg.channel_id;
                    spawn_logged(webhook_demo(ctx, channel_id));
                }
                _ => {}
            }
            Ok(())
        }
    });

    println!(
        "The handlers need the intents {:?}.",
        client.required_intents()
    );
    common::shutdown_on_ctrl_c(&client);
    client.login().await?;
    Ok(())
}
