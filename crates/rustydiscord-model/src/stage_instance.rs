use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    id::{ChannelId, GuildId, ScheduledEventId, StageInstanceId},
    macros::int_enum,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct StageInstance {
    pub id: StageInstanceId,

    pub guild_id: GuildId,

    pub channel_id: ChannelId,

    pub topic: CompactString,

    pub privacy_level: PrivacyLevel,

    #[serde(default)]
    pub discoverable_disabled: bool,

    pub guild_scheduled_event_id: Option<ScheduledEventId>,
}

int_enum! {




    #[doc(alias = "StageInstancePrivacyLevel")]
    pub enum PrivacyLevel: u8 {

        Public = 1,

        GuildOnly = 2,
    }
}
