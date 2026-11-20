use core::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    num::NonZeroU64,
    str::FromStr,
};

use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, Unexpected, Visitor},
};

use crate::model::util::Timestamp;

pub const DISCORD_EPOCH_MS: u64 = 1_420_070_400_000;

pub struct Id<T> {
    value: NonZeroU64,

    marker: PhantomData<fn() -> T>,
}

impl<T> Id<T> {
    #[must_use]
    #[track_caller]
    pub const fn new(value: u64) -> Self {
        match Self::new_checked(value) {
            Some(id) => id,
            None => panic!("snowflake IDs must be non-zero"),
        }
    }

    #[must_use]
    pub const fn new_checked(value: u64) -> Option<Self> {
        match NonZeroU64::new(value) {
            Some(value) => Some(Self::from_nonzero(value)),
            None => None,
        }
    }

    #[must_use]
    pub const fn from_nonzero(value: NonZeroU64) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }

    #[must_use]
    pub const fn get(self) -> u64 {
        self.value.get()
    }

    #[must_use]
    pub const fn into_nonzero(self) -> NonZeroU64 {
        self.value
    }

    #[must_use]
    pub const fn cast<U>(self) -> Id<U> {
        Id::from_nonzero(self.value)
    }

    #[must_use]
    pub const fn timestamp_ms(self) -> u64 {
        (self.get() >> 22) + DISCORD_EPOCH_MS
    }

    #[must_use]
    #[expect(
        clippy::cast_possible_wrap,
        reason = "a snowflake timestamp is at most 2^42 ms after the Discord epoch, far below i64::MAX µs"
    )]
    pub const fn timestamp(self) -> Timestamp {
        Timestamp::from_unix_micros(self.timestamp_ms() as i64 * 1_000)
    }

    #[must_use]
    pub const fn worker_id(self) -> u8 {
        ((self.get() & 0x003E_0000) >> 17) as u8
    }

    #[must_use]
    pub const fn process_id(self) -> u8 {
        ((self.get() & 0x0001_F000) >> 12) as u8
    }

    #[must_use]
    pub const fn increment(self) -> u16 {
        (self.get() & 0x0FFF) as u16
    }

    #[must_use]
    pub const fn from_timestamp_ms(unix_ms: u64) -> Option<Self> {
        if unix_ms <= DISCORD_EPOCH_MS {
            return None;
        }
        let since_epoch = unix_ms - DISCORD_EPOCH_MS;
        if since_epoch >= 1 << 42 {
            return None;
        }
        Self::new_checked(since_epoch << 22)
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u64(self.get());
    }
}

impl<T> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let marker = core::any::type_name::<T>();
        let marker = marker.rsplit("::").next().unwrap_or(marker);
        write!(f, "Id<{marker}>({})", self.value)
    }
}

impl<T> fmt::Display for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.value, f)
    }
}

impl<T> From<NonZeroU64> for Id<T> {
    fn from(value: NonZeroU64) -> Self {
        Self::from_nonzero(value)
    }
}

impl<T> From<Id<T>> for NonZeroU64 {
    fn from(id: Id<T>) -> Self {
        id.value
    }
}

impl<T> From<Id<T>> for u64 {
    fn from(id: Id<T>) -> Self {
        id.get()
    }
}

impl<T> TryFrom<u64> for Id<T> {
    type Error = ParseIdError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        Self::new_checked(value).ok_or(ParseIdError {
            kind: ParseIdErrorKind::Zero,
        })
    }
}

impl<T> FromStr for Id<T> {
    type Err = ParseIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value: u64 = s.parse().map_err(|_| ParseIdError {
            kind: ParseIdErrorKind::NotAnInteger,
        })?;
        Self::try_from(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseIdError {
    kind: ParseIdErrorKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ParseIdErrorKind {
    NotAnInteger,
    Zero,
}

impl ParseIdError {
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.kind == ParseIdErrorKind::Zero
    }
}

impl fmt::Display for ParseIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.kind {
            ParseIdErrorKind::NotAnInteger => "snowflake ID is not a valid unsigned 64-bit integer",
            ParseIdErrorKind::Zero => "snowflake ID must be non-zero",
        })
    }
}

impl core::error::Error for ParseIdError {}

impl<T> Serialize for Id<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(&self.value)
    }
}

impl<'de, T> Deserialize<'de> for Id<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_any(IdVisitor(PhantomData))
    }
}

struct IdVisitor<T>(PhantomData<fn() -> T>);

impl<T> Visitor<'_> for IdVisitor<T> {
    type Value = Id<T>;

    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("a non-zero snowflake ID as a string or integer")
    }

    fn visit_u64<E: DeError>(self, value: u64) -> Result<Self::Value, E> {
        Id::new_checked(value).ok_or_else(|| E::invalid_value(Unexpected::Unsigned(0), &self))
    }

    fn visit_i64<E: DeError>(self, value: i64) -> Result<Self::Value, E> {
        u64::try_from(value)
            .map_err(|_| E::invalid_value(Unexpected::Signed(value), &self))
            .and_then(|value| self.visit_u64(value))
    }

    fn visit_str<E: DeError>(self, value: &str) -> Result<Self::Value, E> {
        value
            .parse()
            .map_err(|_| E::invalid_value(Unexpected::Str(value), &self))
    }
}

pub mod marker {
    macro_rules! markers {
        ($($(#[$meta:meta])* $marker:ident => $alias:ident,)*) => {
            $(
                $(#[$meta])*
                #[doc = ""]
                #[doc = concat!("Usually used through the [`", stringify!($alias), "`](super::", stringify!($alias), ") alias.")]
                #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
                pub enum $marker {}
            )*


            pub(super) mod aliases {
                $(
                    #[doc = concat!("ID of a resource marked by [`", stringify!($marker), "`](super::", stringify!($marker), ").")]
                    pub type $alias = super::super::Id<super::$marker>;
                )*
            }
        };
    }

    markers! {

        ApplicationMarker => ApplicationId,

        AttachmentMarker => AttachmentId,

        AuditLogEntryMarker => AuditLogEntryId,

        AutoModerationRuleMarker => AutoModerationRuleId,


        ChannelMarker => ChannelId,

        CommandMarker => CommandId,

        CommandVersionMarker => CommandVersionId,

        EmojiMarker => EmojiId,

        EntitlementMarker => EntitlementId,

        GenericMarker => GenericId,

        GuildMarker => GuildId,

        IntegrationMarker => IntegrationId,

        InteractionMarker => InteractionId,

        LobbyMarker => LobbyId,

        MessageMarker => MessageId,

        RoleMarker => RoleId,

        ScheduledEventMarker => ScheduledEventId,

        SkuMarker => SkuId,

        SoundboardSoundMarker => SoundboardSoundId,

        StageInstanceMarker => StageInstanceId,

        StickerMarker => StickerId,

        StickerPackMarker => StickerPackId,

        SubscriptionMarker => SubscriptionId,

        TagMarker => TagId,

        TeamMarker => TeamId,

        UserMarker => UserId,

        WebhookMarker => WebhookId,
    }
}

pub use marker::aliases::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout() {
        assert_eq!(size_of::<UserId>(), 8);
        assert_eq!(size_of::<Option<UserId>>(), 8);
    }

    #[test]
    fn snowflake_parts() {
        let id = UserId::new(175_928_847_299_117_063);
        assert_eq!(id.timestamp_ms(), 1_462_015_105_796);
        assert_eq!(id.worker_id(), 1);
        assert_eq!(id.process_id(), 0);
        assert_eq!(id.increment(), 7);
    }

    #[test]
    fn from_timestamp_round_trip() {
        let id = MessageId::from_timestamp_ms(1_462_015_105_796).unwrap();
        assert_eq!(id.timestamp_ms(), 1_462_015_105_796);
        assert_eq!(id.increment(), 0);
        assert!(MessageId::from_timestamp_ms(DISCORD_EPOCH_MS).is_none());
        assert!(MessageId::from_timestamp_ms(0).is_none());
        assert!(MessageId::from_timestamp_ms(u64::MAX).is_none());
    }

    #[test]
    fn parse() {
        assert_eq!("1".parse::<GuildId>().unwrap().get(), 1);
        assert!("0".parse::<GuildId>().unwrap_err().is_zero());
        assert!(!"abc".parse::<GuildId>().unwrap_err().is_zero());
        assert!("-1".parse::<GuildId>().is_err());
        assert!("18446744073709551616".parse::<GuildId>().is_err());
    }

    #[test]
    fn debug_and_display() {
        let id = ChannelId::new(42);
        assert_eq!(format!("{id:?}"), "Id<ChannelMarker>(42)");
        assert_eq!(id.to_string(), "42");
    }

    #[test]
    fn ordering_is_chronological() {
        assert!(UserId::new(1) < UserId::new(2));
    }
}
