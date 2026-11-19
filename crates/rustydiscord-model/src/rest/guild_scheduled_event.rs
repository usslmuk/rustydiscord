use compact_str::CompactString;
use serde::Serialize;

use crate::{
    guild_scheduled_event::{
        EntityMetadata, EntityType, PrivacyLevel, RecurrenceFrequency, RecurrenceMonth,
        RecurrenceNWeekday, RecurrenceRule, RecurrenceWeekday, Status,
    },
    id::{ChannelId, UserId},
    util::{ImageData, Patch, Timestamp},
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListScheduledEventsForGuild {
    pub with_user_count: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetGuildScheduledEvent {
    pub with_user_count: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetGuildScheduledEventUsers {
    pub limit: Option<u8>,

    pub with_member: Option<bool>,

    pub before: Option<UserId>,

    pub after: Option<UserId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RecurrenceRuleData {
    pub start: Timestamp,

    pub frequency: RecurrenceFrequency,

    pub interval: u16,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_weekday: Vec<RecurrenceWeekday>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_n_weekday: Vec<RecurrenceNWeekday>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_month: Vec<RecurrenceMonth>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub by_month_day: Vec<u8>,
}

impl RecurrenceRuleData {
    const fn new(start: Timestamp, frequency: RecurrenceFrequency, interval: u16) -> Self {
        Self {
            start,
            frequency,
            interval,
            by_weekday: Vec::new(),
            by_n_weekday: Vec::new(),
            by_month: Vec::new(),
            by_month_day: Vec::new(),
        }
    }

    #[must_use]
    pub const fn daily(start: Timestamp) -> Self {
        Self::new(start, RecurrenceFrequency::Daily, 1)
    }

    #[must_use]
    pub fn daily_on(start: Timestamp, days: impl IntoIterator<Item = RecurrenceWeekday>) -> Self {
        Self {
            by_weekday: days.into_iter().collect(),
            ..Self::daily(start)
        }
    }

    #[must_use]
    pub fn weekly(start: Timestamp, day: RecurrenceWeekday) -> Self {
        Self {
            by_weekday: vec![day],
            ..Self::new(start, RecurrenceFrequency::Weekly, 1)
        }
    }

    #[must_use]
    pub fn every_other_week(start: Timestamp, day: RecurrenceWeekday) -> Self {
        Self {
            by_weekday: vec![day],
            ..Self::new(start, RecurrenceFrequency::Weekly, 2)
        }
    }

    #[must_use]
    pub fn monthly(start: Timestamp, n: u8, day: RecurrenceWeekday) -> Self {
        Self {
            by_n_weekday: vec![RecurrenceNWeekday::new(n, day)],
            ..Self::new(start, RecurrenceFrequency::Monthly, 1)
        }
    }

    #[must_use]
    pub fn yearly(start: Timestamp, month: RecurrenceMonth, day: u8) -> Self {
        Self {
            by_month: vec![month],
            by_month_day: vec![day],
            ..Self::new(start, RecurrenceFrequency::Yearly, 1)
        }
    }
}

impl From<RecurrenceRule> for RecurrenceRuleData {
    fn from(rule: RecurrenceRule) -> Self {
        Self {
            start: rule.start,
            frequency: rule.frequency,
            interval: rule.interval,
            by_weekday: rule.by_weekday.unwrap_or_default(),
            by_n_weekday: rule.by_n_weekday.unwrap_or_default(),
            by_month: rule.by_month.unwrap_or_default(),
            by_month_day: rule.by_month_day.unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateGuildScheduledEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_metadata: Option<EntityMetadata>,

    pub name: CompactString,

    pub privacy_level: PrivacyLevel,

    pub scheduled_start_time: Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    pub entity_type: EntityType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<ImageData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence_rule: Option<RecurrenceRuleData>,
}

impl CreateGuildScheduledEvent {
    fn new(name: CompactString, start: Timestamp, entity_type: EntityType) -> Self {
        Self {
            channel_id: None,
            entity_metadata: None,
            name,
            privacy_level: PrivacyLevel::GuildOnly,
            scheduled_start_time: start,
            scheduled_end_time: None,
            description: None,
            entity_type,
            image: None,
            recurrence_rule: None,
        }
    }

    #[must_use]
    pub fn stage(channel_id: ChannelId, name: impl Into<CompactString>, start: Timestamp) -> Self {
        Self {
            channel_id: Some(channel_id),
            ..Self::new(name.into(), start, EntityType::StageInstance)
        }
    }

    #[must_use]
    pub fn voice(channel_id: ChannelId, name: impl Into<CompactString>, start: Timestamp) -> Self {
        Self {
            channel_id: Some(channel_id),
            ..Self::new(name.into(), start, EntityType::Voice)
        }
    }

    #[must_use]
    pub fn external(
        location: impl Into<CompactString>,
        name: impl Into<CompactString>,
        start: Timestamp,
        end: Timestamp,
    ) -> Self {
        Self {
            entity_metadata: Some(EntityMetadata::new(location)),
            scheduled_end_time: Some(end),
            ..Self::new(name.into(), start, EntityType::External)
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildScheduledEvent {
    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub channel_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub entity_metadata: Patch<EntityMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<PrivacyLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_start_time: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_end_time: Option<Timestamp>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description: Patch<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub image: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub recurrence_rule: Patch<RecurrenceRuleData>,
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::guild_scheduled_event::{RecurrenceMonth, RecurrenceWeekday};

    fn start() -> Timestamp {
        Timestamp::parse("2026-12-01T18:00:00.000000+00:00").unwrap()
    }

    fn json(rule: &RecurrenceRuleData) -> Value {
        let mut value = serde_json::to_value(rule).unwrap();
        value.as_object_mut().unwrap().remove("start");
        value
    }

    #[test]
    fn recurrence_rules_as_rustydiscord_writes_them() {
        use RecurrenceWeekday::{Friday, Monday, Tuesday};

        assert_eq!(
            json(&RecurrenceRuleData::daily(start())),
            json!({"frequency": 3, "interval": 1})
        );
        assert_eq!(
            json(&RecurrenceRuleData::daily_on(start(), [Monday, Tuesday])),
            json!({"frequency": 3, "interval": 1, "by_weekday": [0, 1]})
        );
        assert_eq!(
            json(&RecurrenceRuleData::weekly(start(), Friday)),
            json!({"frequency": 2, "interval": 1, "by_weekday": [4]})
        );
        assert_eq!(
            json(&RecurrenceRuleData::every_other_week(start(), Friday)),
            json!({"frequency": 2, "interval": 2, "by_weekday": [4]})
        );
        assert_eq!(
            json(&RecurrenceRuleData::monthly(start(), 1, Monday)),
            json!({"frequency": 1, "interval": 1, "by_n_weekday": [{"n": 1, "day": 0}]})
        );
        assert_eq!(
            json(&RecurrenceRuleData::yearly(
                start(),
                RecurrenceMonth::July,
                4
            )),
            json!({"frequency": 0, "interval": 1, "by_month": [7], "by_month_day": [4]})
        );
    }

    #[test]
    fn a_voice_event_names_its_channel() {
        let event = CreateGuildScheduledEvent::voice(ChannelId::new(5), "Movie night", start());
        let value = serde_json::to_value(&event).unwrap();
        assert_eq!(value["channel_id"], "5");
        assert_eq!(value["entity_type"], 2);
        assert_eq!(value["name"], "Movie night");
    }
}
