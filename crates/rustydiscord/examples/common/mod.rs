#![allow(dead_code)]

use rustydiscord::{Client, model::id::GuildId};

pub(crate) fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();
}

pub(crate) fn config_value(key: &str) -> Option<String> {
    if let Ok(value) = std::env::var(key) {
        return Some(value).filter(|value| !value.is_empty());
    }
    let mut dir = std::env::current_dir().ok()?;
    loop {
        if let Ok(contents) = std::fs::read_to_string(dir.join(".env")) {
            return contents.lines().find_map(|line| {
                let (name, value) = line.trim().split_once('=')?;
                let value = value.trim().trim_matches('"');
                (name.trim() == key && !value.is_empty()).then(|| value.to_owned())
            });
        }
        if !dir.pop() {
            return None;
        }
    }
}

pub(crate) fn token() -> Result<String, &'static str> {
    config_value("DISCORD_TOKEN")
        .ok_or("DISCORD_TOKEN is not set (put it in .env or the environment)")
}

pub(crate) fn test_guild() -> Result<Option<GuildId>, &'static str> {
    config_value("DISCORD_TEST_GUILD_ID")
        .map(|id| {
            id.parse()
                .map_err(|_| "DISCORD_TEST_GUILD_ID is not a guild ID")
        })
        .transpose()
}

pub(crate) fn cleanup_requested() -> bool {
    std::env::args().any(|argument| argument == "--cleanup")
}

pub(crate) fn shutdown_on_ctrl_c(client: &Client) {
    let client = client.clone();
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            println!("Shutting down…");
            client.shutdown();
        }
    });
}
