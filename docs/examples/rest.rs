#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rest = rustydiscord::rest::Rest::builder()
        .token(std::env::var("DISCORD_TOKEN")?)
        .build();

    let bot = rest.get_current_bot_application_information().await?;
    println!("Application id: {}", bot.id);

    let metrics = rest.metrics().render_prometheus();
    println!("{metrics}");

    Ok(())
}
