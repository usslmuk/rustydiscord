use compact_str::CompactString;
use serde::Serialize;

use crate::{
    id::{ChannelId, ScheduledEventId},
    stage_instance::PrivacyLevel,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateStageInstance {
    pub channel_id: ChannelId,

    pub topic: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<PrivacyLevel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_start_notification: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event_id: Option<ScheduledEventId>,
}

impl CreateStageInstance {
    #[must_use]
    pub fn new(channel_id: ChannelId, topic: impl Into<CompactString>) -> Self {
        Self {
            channel_id,
            topic: topic.into(),
            privacy_level: None,
            send_start_notification: None,
            guild_scheduled_event_id: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyStageInstance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_level: Option<PrivacyLevel>,
}
