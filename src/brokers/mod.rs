#![cfg_attr(docsrs, feature(doc_cfg))]

use tokio::sync::broadcast;

pub trait Broker {
    type Message: Clone + Send + Sync + 'static;

    #[must_use]
    fn publish(&self, message: Self::Message) -> bool;

    #[must_use]
    fn subscribe(&self) -> broadcast::Receiver<Self::Message>;
}

#[derive(Debug, Clone)]
pub struct MemoryBroker<T>
where
    T: Clone + Send + Sync + 'static,
{
    sender: broadcast::Sender<T>,
}

impl<T> MemoryBroker<T>
where
    T: Clone + Send + Sync + 'static,
{
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity.max(1));
        Self { sender }
    }
}

impl<T> Broker for MemoryBroker<T>
where
    T: Clone + Send + Sync + 'static,
{
    type Message = T;

    fn publish(&self, message: T) -> bool {
        self.sender.send(message).is_ok()
    }

    fn subscribe(&self) -> broadcast::Receiver<T> {
        self.sender.subscribe()
    }
}

#[cfg(test)]
mod tests {
    use super::{Broker, MemoryBroker};

    #[tokio::test]
    async fn memory_broker_fans_out_to_subscribers() {
        let broker = MemoryBroker::new(16);
        let mut first = broker.subscribe();
        let mut second = broker.subscribe();
        assert!(broker.publish(String::from("hello")));
        assert_eq!(first.recv().await.expect("message"), "hello");
        assert_eq!(second.recv().await.expect("message"), "hello");
    }

    #[tokio::test]
    async fn publish_without_subscribers_reports_loss() {
        let broker = MemoryBroker::<u64>::new(4);
        assert!(!broker.publish(1));
    }
}
