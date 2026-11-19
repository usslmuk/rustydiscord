#![cfg_attr(docsrs, feature(doc_cfg))]

#[must_use]
#[allow(clippy::cast_possible_truncation)]
pub const fn calculate_shard_id(guild_id: u64, shard_count: u32) -> u32 {
    if shard_count == 0 {
        return 0;
    }
    ((guild_id >> 22) % (shard_count as u64)) as u32
}

#[must_use]
pub fn lazy<T, F: FnOnce() -> T>(init: F) -> std::sync::LazyLock<T, F> {
    std::sync::LazyLock::new(init)
}

#[must_use]
pub fn range(end: usize) -> core::ops::Range<usize> {
    0..end
}

pub fn range_stepped(start: usize, end: usize, step: usize) -> impl Iterator<Item = usize> {
    (start..end).step_by(step.max(1))
}

pub trait Equatable {
    fn equals(&self, other: &Self) -> bool;
}

impl<T: PartialEq + ?Sized> Equatable for T {
    fn equals(&self, other: &Self) -> bool {
        self == other
    }
}

#[must_use]
pub fn is_equatable<T: Equatable + ?Sized>(_: &T) -> bool {
    true
}

pub trait JsonEncodable: serde::Serialize {
    fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}

impl<T: serde::Serialize + ?Sized> JsonEncodable for T {}

#[must_use]
pub fn is_json_encodable<T: JsonEncodable + ?Sized>(_: &T) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::{Equatable, JsonEncodable};
    use super::{calculate_shard_id, is_equatable, is_json_encodable, lazy, range, range_stepped};

    #[test]
    fn shard_ids_stay_inside_the_count() {
        assert_eq!(calculate_shard_id(813_847_366_044_157_490, 16), 9);
        assert_eq!(calculate_shard_id(0, 0), 0);
        for count in [1_u32, 2, 7, 16] {
            for id in [1_u64, 1 << 22, u64::MAX] {
                assert!(calculate_shard_id(id, count) < count);
            }
        }
    }

    #[test]
    fn lazy_initializes_once() {
        use std::sync::atomic::{AtomicUsize, Ordering};

        static CALLS: AtomicUsize = AtomicUsize::new(0);
        let value = lazy(|| {
            CALLS.fetch_add(1, Ordering::SeqCst);
            41 + 1
        });
        assert_eq!(*value, 42);
        assert_eq!(*value, 42);
        assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn ranges_cover_every_index() {
        assert_eq!(range(0).count(), 0);
        assert_eq!(range(4).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
        assert_eq!(
            range_stepped(0, 10, 3).collect::<Vec<_>>(),
            vec![0, 3, 6, 9]
        );
        assert_eq!(range_stepped(2, 2, 1).count(), 0);
        assert_eq!(range_stepped(0, 3, 0).collect::<Vec<_>>(), vec![0, 1, 2]);
    }

    #[test]
    fn equality_and_encodability_hold_for_plain_values() {
        assert!(is_equatable(&42_u64));
        assert!(is_equatable("text"));
        assert!("a".equals("a"));
        assert!(!"a".equals("b"));
        assert!(is_json_encodable(&vec![1, 2, 3]));
        assert_eq!(vec![1, 2].to_json(), serde_json::json!([1, 2]));
    }
}
