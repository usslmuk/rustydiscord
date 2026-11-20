use std::sync::Arc;
use std::time::Duration;

use rustydiscord::brokers::Broker;
use rustydiscord::model::gateway::payload::MessageCreate;
use rustydiscord::model::rest::GetChannelMessages;
use rustydiscord::prelude::*;
use rustydiscord::util::Equatable;

#[path = "common/mod.rs"]
mod common;

async fn maxbot_handler(
    ctx: Context,
    msg: Arc<MessageCreate>,
) -> Result<(), rustydiscord::HandlerError> {
    if msg.content == "!maxbot" {
        let mut query = GetChannelMessages::default();
        query.limit = Some(10);
        let history = ctx
            .messages(msg.channel_id)
            .paginate(query)
            .fetch_all()
            .await?;
        let reply = msg
            .reply(
                &ctx,
                format!("Channel holds {} recent messages.", history.len()),
            )
            .await?;
        let author = msg.author.id;
        let mut collected = ctx
            .message_collector(msg.channel_id)
            .filter(move |incoming| incoming.author.id == author)
            .max(3)
            .time(Duration::from_secs(20))
            .build();
        let mut seen = 0;
        while collected.next().await.is_some() {
            seen += 1;
        }
        reply
            .reply(&ctx, format!("Collected {seen} follow-ups this round."))
            .await?;
    }
    if msg.content == "!ecosystem" {
        let shard = rustydiscord::util::calculate_shard_id(
            msg.guild_id.map_or(1, rustydiscord::model::id::Id::get),
            16,
        );
        let send_ok = rustydiscord::opcodes::is_gateway_send_allowed(2);
        let recv_ok = rustydiscord::opcodes::is_gateway_receive_allowed(0);
        let proxy =
            rustydiscord::proxy::ProxyConfig::new(String::from("https://proxy.local"), true);
        let (url, kept) = proxy.forward(
            "/api/channels",
            &[(String::from("Authorization"), String::from("Bot secret"))],
        );
        let broker = rustydiscord::brokers::MemoryBroker::<String>::new(16);
        let mut rx = broker.subscribe();
        assert!(broker.publish(format!("hello shard {shard}")));
        let echoed = rx.recv().await?;
        let same = "ready".equals("ready");
        msg.reply(
            &ctx,
            format!(
                "shard {shard} send {send_ok} recv {recv_ok} same {same} {url} {} {echoed}",
                kept.len()
            ),
        )
        .await?;
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
            "Logged in as {} — in {} guild(s). Send `!maxbot` or `!ecosystem`.",
            ready.user.tag(),
            ready.guilds.len()
        );
        Ok(())
    });

    client.on(Events::MessageCreate, maxbot_handler);

    common::shutdown_on_ctrl_c(&client);

    client.login().await?;
    Ok(())
}
