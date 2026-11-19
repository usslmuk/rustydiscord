use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{
    guild::Member,
    id::{ChannelId, GuildId, UserId},
    util::Timestamp,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceState {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub channel_id: Option<ChannelId>,

    pub user_id: UserId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<Member>>,

    pub session_id: CompactString,

    #[serde(default)]
    pub deaf: bool,

    #[serde(default)]
    pub mute: bool,

    #[serde(default)]
    pub self_deaf: bool,

    #[serde(default)]
    pub self_mute: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub self_stream: bool,

    #[serde(default)]
    pub self_video: bool,

    #[serde(default)]
    pub suppress: bool,

    pub request_to_speak_timestamp: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceRegion {
    pub id: CompactString,

    pub name: CompactString,

    pub optimal: bool,

    pub deprecated: bool,

    pub custom: bool,
}

#[cfg(test)]
mod tests {
    use core::mem::size_of;

    use super::*;

    #[test]
    fn sizes() {
        assert!(size_of::<VoiceState>() <= 96, "{}", size_of::<VoiceState>());
    }
}
