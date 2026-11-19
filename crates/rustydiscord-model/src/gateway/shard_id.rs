use core::fmt;

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, SeqAccess, Visitor},
    ser::SerializeTuple,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ShardId {
    number: u32,
    total: u32,
}

impl ShardId {
    pub const ONE: Self = Self::new(0, 1);

    #[must_use]
    #[track_caller]
    pub const fn new(number: u32, total: u32) -> Self {
        match Self::new_checked(number, total) {
            Some(shard) => shard,
            None => panic!("shard number must be less than the (non-zero) shard total"),
        }
    }

    #[must_use]
    pub const fn new_checked(number: u32, total: u32) -> Option<Self> {
        if number < total {
            Some(Self { number, total })
        } else {
            None
        }
    }

    #[must_use]
    #[track_caller]
    #[expect(
        clippy::cast_possible_truncation,
        reason = "result is < total, which is a u32"
    )]
    pub const fn for_guild(guild: crate::id::GuildId, total: u32) -> Self {
        assert!(total > 0, "shard total must be non-zero");
        Self::new(((guild.get() >> 22) % total as u64) as u32, total)
    }

    #[must_use]
    pub const fn number(self) -> u32 {
        self.number
    }

    #[must_use]
    pub const fn total(self) -> u32 {
        self.total
    }
}

impl fmt::Display for ShardId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.number, self.total)
    }
}

impl Serialize for ShardId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tuple = serializer.serialize_tuple(2)?;
        tuple.serialize_element(&self.number)?;
        tuple.serialize_element(&self.total)?;
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for ShardId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ShardIdVisitor;

        impl<'de> Visitor<'de> for ShardIdVisitor {
            type Value = ShardId;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a [shard_id, num_shards] array")
            }

            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<ShardId, A::Error> {
                let number = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(0, &self))?;
                let total = seq
                    .next_element()?
                    .ok_or_else(|| A::Error::invalid_length(1, &self))?;
                if seq.next_element::<serde::de::IgnoredAny>()?.is_some() {
                    return Err(A::Error::invalid_length(3, &self));
                }
                ShardId::new_checked(number, total)
                    .ok_or_else(|| A::Error::custom("shard number must be less than shard total"))
            }
        }

        deserializer.deserialize_tuple(2, ShardIdVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde() {
        let shard: ShardId = serde_json::from_str("[3, 8]").unwrap();
        assert_eq!((shard.number(), shard.total()), (3, 8));
        assert_eq!(serde_json::to_string(&shard).unwrap(), "[3,8]");
        assert!(serde_json::from_str::<ShardId>("[8, 8]").is_err());
        assert!(serde_json::from_str::<ShardId>("[0, 0]").is_err());
        assert!(serde_json::from_str::<ShardId>("[0]").is_err());
        assert!(serde_json::from_str::<ShardId>("[0, 1, 2]").is_err());
    }

    #[test]
    fn checked() {
        assert!(ShardId::new_checked(0, 0).is_none());
        assert!(ShardId::new_checked(1, 1).is_none());
        assert_eq!(ShardId::new_checked(0, 1), Some(ShardId::ONE));
    }
}
