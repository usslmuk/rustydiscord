use rustydiscord::model::rest::ExecuteWebhook;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let webhook =
        rustydiscord::webhook::WebhookClient::from_url(&std::env::var("WEBHOOK_URL")?)?;
    let message = webhook.send(ExecuteWebhook::from("Hello from rustydiscord.")).await?;
    println!("Sent as message {}", message.id);
    Ok(())
}
