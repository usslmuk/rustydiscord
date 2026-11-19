use rustydiscord::brokers::{Broker, MemoryBroker};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let broker = MemoryBroker::new(16);
    let mut events = broker.subscribe();
    assert!(broker.publish(String::from("ready")));
    let first = events.recv().await?;
    println!("received {first}");
    assert!(broker.publish(String::from("resumed")));
    let second = events.recv().await?;
    println!("received {second}");
    Ok(())
}
