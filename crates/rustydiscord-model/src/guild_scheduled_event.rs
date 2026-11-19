use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    guild::PartialMember,
    id::{ChannelId, GenericId, GuildId, ScheduledEventId, UserId},
    macros::int_enum,
    user::User,
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildScheduledEvent {
    pub id: ScheduledEventId,

    pub guild_id: GuildId,

    pub channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_id: Option<UserId>,

    pub name: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    pub scheduled_start_time: Timestamp,

    pub scheduled_end_time: Option<Timestamp>,

    pub privacy_level: PrivacyLevel,

    pub status: Status,

    pub entity_type: EntityType,

    pub entity_id: Option<GenericId>,

    pub entity_metadata: Option<EntityMetadata>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageHash>,

    pub recurrence_rule: Option<Box<RecurrenceRule>>,
}

int_enum! {




    #[doc(alias = "GuildScheduledEventPrivacyLevel")]
    pub enum PrivacyLevel: u8 {

        GuildOnly = 2,
    }
}

int_enum! {




    #[doc(alias = "GuildScheduledEventEntityType")]
    pub enum EntityType: u8 {

        StageInstance = 1,

        Voice = 2,

        External = 3,
    }
}

int_enum! {







    #[doc(alias = "GuildScheduledEventStatus")]
    pub enum Status: u8 {

        Scheduled = 1,

        Active = 2,

        Completed = 3,

        Canceled = 4,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EntityMetadata {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<CompactString>,
}

impl EntityMetadata {
    #[must_use]
    pub fn new(location: impl Into<CompactString>) -> Self {
        Self {
            location: Some(location.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildScheduledEventUser {
    pub guild_scheduled_event_id: ScheduledEventId,

    pub user: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<PartialMember>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RecurrenceRule {
    pub start: Timestamp,

    pub end: Option<Timestamp>,

    pub frequency: RecurrenceFrequency,

    pub interval: u16,

    pub by_weekday: Option<Vec<RecurrenceWeekday>>,

    pub by_n_weekday: Option<Vec<RecurrenceNWeekday>>,

    pub by_month: Option<Vec<RecurrenceMonth>>,

    pub by_month_day: Option<Vec<u8>>,

    pub by_year_day: Option<Vec<u16>>,

    pub count: Option<u32>,
}

int_enum! {




    pub enum RecurrenceFrequency: u8 {

        Yearly = 0,

        Monthly = 1,

        Weekly = 2,

        Daily = 3,
    }
}

int_enum! {




    pub enum RecurrenceWeekday: u8 {

        Monday = 0,

        Tuesday = 1,

        Wednesday = 2,

        Thursday = 3,

        Friday = 4,

        Saturday = 5,

        Sunday = 6,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RecurrenceNWeekday {
    pub n: u8,

    pub day: RecurrenceWeekday,
}

impl RecurrenceNWeekday {
    #[must_use]
    pub const fn new(n: u8, day: RecurrenceWeekday) -> Self {
        Self { n, day }
    }
}

int_enum! {




    pub enum RecurrenceMonth: u8 {

        January = 1,

        February = 2,

        March = 3,

        April = 4,

        May = 5,

        June = 6,

        July = 7,

        August = 8,

        September = 9,

        October = 10,

        November = 11,

        December = 12,
    }
}
