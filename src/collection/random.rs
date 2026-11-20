use std::{
    collections::HashSet,
    hash::{BuildHasher, Hasher, RandomState},
};

pub(crate) struct Rng(u64);

impl Rng {
    pub(crate) fn new() -> Self {
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_u64(0x5EED);
        Self(hasher.finish())
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0xA076_1D64_78BD_642F);
        let product = u128::from(self.0) * u128::from(self.0 ^ 0xE703_7ED1_A0B4_28DB);
        #[expect(
            clippy::cast_possible_truncation,
            reason = "keeping the low 64 bits is the point"
        )]
        let (low, high) = (product as u64, (product >> 64) as u64);
        low ^ high
    }

    pub(crate) fn below(&mut self, bound: usize) -> usize {
        debug_assert!(bound > 0, "no number is below 0");

        let scaled = (u128::from(self.next_u64()) * bound as u128) >> 64;
        usize::try_from(scaled).unwrap_or(0)
    }
}

pub(crate) fn distinct(len: usize, count: usize) -> Vec<usize> {
    debug_assert!(count <= len);
    let mut rng = Rng::new();
    if count * 4 >= len {
        let mut indexes: Vec<usize> = (0..len).collect();
        for at in 0..count {
            let other = at + rng.below(len - at);
            indexes.swap(at, other);
        }
        indexes.truncate(count);
        return indexes;
    }

    let mut chosen =
        HashSet::with_capacity_and_hasher(count, foldhash::fast::RandomState::default());
    let mut picks = Vec::with_capacity(count);
    for upper in len - count..len {
        let pick = rng.below(upper + 1);
        let pick = if chosen.insert(pick) { pick } else { upper };
        if pick == upper {
            chosen.insert(upper);
        }
        picks.push(pick);
    }
    for at in (1..picks.len()).rev() {
        picks.swap(at, rng.below(at + 1));
    }
    picks
}
