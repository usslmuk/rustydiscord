use core::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::model::util::Timestamp;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum TimestampStyle {
    ShortTime,

    MediumTime,

    ShortDate,

    LongDate,

    LongDateShortTime,

    FullDateShortTime,

    ShortDateShortTime,

    ShortDateMediumTime,

    RelativeTime,
}

impl TimestampStyle {
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::ShortTime => 't',
            Self::MediumTime => 'T',
            Self::ShortDate => 'd',
            Self::LongDate => 'D',
            Self::LongDateShortTime => 'f',
            Self::FullDateShortTime => 'F',
            Self::ShortDateShortTime => 's',
            Self::ShortDateMediumTime => 'S',
            Self::RelativeTime => 'R',
        }
    }
}

pub trait UnixSeconds {
    fn unix_seconds(&self) -> i64;
}

impl UnixSeconds for i64 {
    fn unix_seconds(&self) -> i64 {
        *self
    }
}

impl UnixSeconds for Timestamp {
    fn unix_seconds(&self) -> i64 {
        self.as_unix_secs()
    }
}

impl UnixSeconds for SystemTime {
    fn unix_seconds(&self) -> i64 {
        match self.duration_since(UNIX_EPOCH) {
            Ok(after) => i64::try_from(after.as_secs()).unwrap_or(i64::MAX),
            Err(before) => {
                let before = before.duration();

                let seconds = before.as_secs() + u64::from(before.subsec_nanos() > 0);
                i64::try_from(seconds).map_or(i64::MIN, |seconds| -seconds)
            }
        }
    }
}

impl<T: UnixSeconds + ?Sized> UnixSeconds for &T {
    fn unix_seconds(&self) -> i64 {
        (**self).unix_seconds()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct Time {
    seconds: i64,
    style: Option<TimestampStyle>,
}

pub fn time(at: impl UnixSeconds) -> Time {
    Time {
        seconds: at.unix_seconds(),
        style: None,
    }
}

impl Time {
    pub const fn style(mut self, style: TimestampStyle) -> Self {
        self.style = Some(style);
        self
    }

    #[must_use]
    pub const fn seconds(self) -> i64 {
        self.seconds
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.style {
            Some(style) => write!(f, "<t:{}:{}>", self.seconds, style.as_char()),
            None => write!(f, "<t:{}>", self.seconds),
        }
    }
}
