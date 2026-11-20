use rustydiscord::brokers::{Broker, MemoryBroker};
use rustydiscord::collection::Collection;
use rustydiscord::opcodes::is_gateway_send_allowed;
use rustydiscord::proxy::ProxyConfig;
use rustydiscord::util::{Equatable, calculate_shard_id};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broker = MemoryBroker::<String>::new(16);
    let mut events = broker.subscribe();
    assert!(broker.publish("hello".to_owned()));
    println!("Bus heard: {}", events.recv().await?);

    let proxy = ProxyConfig::new(String::from("https://proxy.local"), true);
    let (url, headers) = proxy.forward(
        "/api/channels",
        &[(String::from("Authorization"), String::from("Bot secret"))],
    );
    println!("Forward to {url} hiding {} header(s)", headers.len());

    let mut users: Collection<u64, String> = Collection::new();
    users.insert(7, "ferris".into());
    println!("Cached user: {}", users.get(&7u64).unwrap());

    println!("'ready' equals 'ready': {}", "ready".equals("ready"));
    println!("Guild shard: {}", calculate_shard_id(1, 16));
    println!("Opcode 2 sends allowed: {}", is_gateway_send_allowed(2));
    Ok(())
}
