use core::ops::Range;

use crate::model::gateway::{GatewayEvent, ShardId};

use crate::gateway::{Config, Shard};

#[track_caller]
pub fn create_shards(
    range: Range<u32>,
    total: u32,
    config: Config,
) -> impl ExactSizeIterator<Item = Shard> + DoubleEndedIterator {
    assert!(total > 0, "the shard total must not be zero");
    assert!(
        range.end <= total,
        "shard range {range:?} reaches past the total of {total}"
    );
    range.map(move |number| Shard::with_config(ShardId::new(number, total), config.clone()))
}

#[must_use]
pub fn reshard_plan(_old_total: u32, new_total: u32) -> Vec<(u32, u32)> {
    assert!(new_total > 0, "the new shard total must not be zero");
    (0..new_total).map(|shard| (shard, new_total)).collect()
}

#[must_use]
pub fn overlapping_shards(old_total: u32, new_total: u32, range: Range<u32>) -> Vec<u32> {
    range.filter(|_| old_total != new_total).collect()
}

#[derive(serde::Deserialize)]
struct BorrowedEnvelope<'a> {
    #[serde(borrow, default)]
    t: Option<&'a str>,
}

pub fn decode_dispatch_borrowed(bytes: &[u8]) -> Result<Option<&str>, serde_json::Error> {
    let envelope: BorrowedEnvelope<'_> = serde_json::from_slice(bytes)?;
    Ok(envelope.t)
}

pub fn spawn_shard_threads(
    range: Range<u32>,
    total: u32,
    config: &Config,
    threads: u32,
    on_event: impl Fn(u32, GatewayEvent) + Send + Sync + 'static,
) -> std::io::Result<Vec<std::thread::JoinHandle<()>>> {
    assert!(total > 0, "the shard total must not be zero");
    assert!(
        range.end <= total,
        "shard range {range:?} reaches past the total of {total}"
    );
    let threads = threads.max(1) as usize;
    let on_event = std::sync::Arc::new(on_event);
    let numbers: Vec<u32> = range.collect();
    let mut handles = Vec::with_capacity(threads.min(numbers.len().max(1)));
    for chunk in numbers.chunks(numbers.len().div_ceil(threads).max(1)) {
        let (shards, config, on_event) = (
            chunk.to_vec(),
            config.clone(),
            std::sync::Arc::clone(&on_event),
        );
        handles.push(std::thread::spawn(move || {
            let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            else {
                return;
            };
            runtime.block_on(async move {
                let mut shards: Vec<Shard> = shards
                    .into_iter()
                    .map(|number| Shard::with_config(ShardId::new(number, total), config.clone()))
                    .collect();
                loop {
                    if shards.is_empty() {
                        return;
                    }
                    let mut index = 0;
                    while index < shards.len() {
                        match shards[index].next_event().await {
                            Some(Ok(event)) => {
                                on_event(shards[index].id().number(), event);
                                index += 1;
                            }
                            Some(Err(error)) if !error.is_fatal() => index += 1,
                            _ => {
                                shards.swap_remove(index);
                            }
                        }
                    }
                }
            });
        }));
    }
    Ok(handles)
}
