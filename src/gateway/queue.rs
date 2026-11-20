use core::{fmt, future::Future, pin::Pin, time::Duration};
use std::sync::Arc;

use crate::model::gateway::ShardId;
use tokio::{
    sync::Mutex,
    time::{Instant, sleep_until},
};

pub(crate) const IDENTIFY_INTERVAL: Duration = Duration::from_millis(5_100);

pub trait IdentifyQueue: fmt::Debug + Send + Sync {
    fn wait_turn(&self, shard: ShardId) -> Pin<Box<dyn Future<Output = ()> + Send>>;
}

#[derive(Clone)]
pub struct LocalIdentifyQueue {
    buckets: Arc<[Mutex<Option<Instant>>]>,
}

impl LocalIdentifyQueue {
    #[must_use]
    pub fn new(max_concurrency: u32) -> Self {
        let buckets = (0..max_concurrency.max(1))
            .map(|_| Mutex::new(None))
            .collect();
        Self { buckets }
    }

    #[must_use]
    pub fn max_concurrency(&self) -> u32 {
        u32::try_from(self.buckets.len()).unwrap_or(u32::MAX)
    }
}

impl Default for LocalIdentifyQueue {
    fn default() -> Self {
        Self::new(1)
    }
}

impl fmt::Debug for LocalIdentifyQueue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("LocalIdentifyQueue")
            .field("max_concurrency", &self.buckets.len())
            .finish()
    }
}

impl IdentifyQueue for LocalIdentifyQueue {
    fn wait_turn(&self, shard: ShardId) -> Pin<Box<dyn Future<Output = ()> + Send>> {
        let buckets = Arc::clone(&self.buckets);
        let bucket = shard.number() as usize % buckets.len();
        Box::pin(async move {
            let mut last = buckets[bucket].lock().await;
            if let Some(last) = *last {
                sleep_until(last + IDENTIFY_INTERVAL).await;
            }
            *last = Some(Instant::now());
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{IDENTIFY_INTERVAL, IdentifyQueue, Instant, LocalIdentifyQueue, ShardId};

    async fn turn(queue: &LocalIdentifyQueue, number: u32) -> Instant {
        queue.wait_turn(ShardId::new(number, 64)).await;
        Instant::now()
    }

    #[tokio::test(start_paused = true)]
    async fn one_identify_per_bucket_every_interval() {
        let queue = LocalIdentifyQueue::new(4);
        let start = Instant::now();

        let mut turns = Vec::new();
        for number in 0..8 {
            turns.push(tokio::spawn({
                let queue = queue.clone();
                async move { turn(&queue, number).await }
            }));
        }
        let mut at = Vec::new();
        for task in turns {
            at.push(task.await.unwrap() - start);
        }

        assert!(at[..4].iter().all(|&after| after.is_zero()), "{at:?}");
        assert!(
            at[4..].iter().all(|&after| after == IDENTIFY_INTERVAL),
            "{at:?}"
        );
    }

    #[tokio::test(start_paused = true)]
    async fn a_shard_waits_for_its_own_previous_identify() {
        let queue = LocalIdentifyQueue::default();
        let first = turn(&queue, 0).await;
        let again = turn(&queue, 0).await;
        assert_eq!(again - first, IDENTIFY_INTERVAL);
    }

    #[tokio::test(start_paused = true)]
    async fn cancelled_waits_take_no_turn() {
        let queue = LocalIdentifyQueue::new(1);
        let start = turn(&queue, 0).await;

        let abandoned = queue.wait_turn(ShardId::new(1, 64));
        let waited = tokio::time::timeout(IDENTIFY_INTERVAL / 2, abandoned).await;
        assert!(waited.is_err(), "the turn came too early");
        let next = turn(&queue, 2).await;
        assert_eq!(next - start, IDENTIFY_INTERVAL);
        let after = turn(&queue, 3).await;
        assert_eq!(after - start, IDENTIFY_INTERVAL * 2);
    }

    #[test]
    fn zero_concurrency_means_one_bucket() {
        assert_eq!(LocalIdentifyQueue::new(0).max_concurrency(), 1);
        assert_eq!(LocalIdentifyQueue::new(16).max_concurrency(), 16);
    }
}
