use core::fmt;

use compact_str::CompactString;
use serde::{Serialize, Serializer, ser::SerializeStruct};

use crate::{
    gateway::{Activity, Intents, OpCode, ShardId, Status},
    id::{ChannelId, GuildId, UserId},
    macros::{int_flags, str_enum},
    util::Timestamp,
};

pub trait GatewayCommand: Serialize {
    const OPCODE: OpCode;
}

#[derive(Debug)]
pub struct CommandPayload<'a, T> {
    command: &'a T,
}

impl<'a, T: GatewayCommand> CommandPayload<'a, T> {
    #[must_use]
    pub const fn new(command: &'a T) -> Self {
        Self { command }
    }
}

impl<T: GatewayCommand> Serialize for CommandPayload<'_, T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut payload = serializer.serialize_struct("CommandPayload", 2)?;
        payload.serialize_field("op", &T::OPCODE)?;
        payload.serialize_field("d", self.command)?;
        payload.end()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[serde(transparent)]
pub struct Heartbeat(pub Option<u64>);

impl GatewayCommand for Heartbeat {
    const OPCODE: OpCode = OpCode::Heartbeat;
}

#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct Identify {
    pub token: CompactString,

    pub properties: IdentifyProperties,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_threshold: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shard: Option<ShardId>,

    pub intents: Intents,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence: Option<UpdatePresence>,

    #[serde(skip_serializing_if = "GatewayCapabilities::is_empty")]
    pub capabilities: GatewayCapabilities,
}

int_flags! {




    pub struct GatewayCapabilities: u64 {

        const CHANNEL_OBFUSCATION = 1 << 15;
    }
}

impl GatewayCommand for Identify {
    const OPCODE: OpCode = OpCode::Identify;
}

impl fmt::Debug for Identify {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Identify")
            .field("token", &"<redacted>")
            .field("properties", &self.properties)
            .field("large_threshold", &self.large_threshold)
            .field("shard", &self.shard)
            .field("intents", &self.intents)
            .field("presence", &self.presence)
            .field("capabilities", &self.capabilities)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct IdentifyProperties {
    pub os: CompactString,

    pub browser: CompactString,

    pub device: CompactString,
}

impl IdentifyProperties {
    #[must_use]
    pub fn new(library: &str) -> Self {
        Self {
            os: CompactString::const_new(std::env::consts::OS),
            browser: library.into(),
            device: library.into(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct Resume {
    pub token: CompactString,

    pub session_id: CompactString,

    pub seq: u64,
}

impl GatewayCommand for Resume {
    const OPCODE: OpCode = OpCode::Resume;
}

impl fmt::Debug for Resume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resume")
            .field("token", &"<redacted>")
            .field("session_id", &self.session_id)
            .field("seq", &self.seq)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RequestGuildMembers {
    pub guild_id: GuildId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub presences: bool,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub user_ids: Vec<UserId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<CompactString>,
}

impl RequestGuildMembers {
    #[must_use]
    pub fn query(guild_id: GuildId, query: impl Into<CompactString>, limit: u32) -> Self {
        Self {
            guild_id,
            query: Some(query.into()),
            limit: Some(limit),
            presences: false,
            user_ids: Vec::new(),
            nonce: None,
        }
    }

    #[must_use]
    pub fn all(guild_id: GuildId) -> Self {
        Self::query(guild_id, "", 0)
    }

    #[must_use]
    pub fn users(guild_id: GuildId, user_ids: impl IntoIterator<Item = UserId>) -> Self {
        Self {
            guild_id,
            query: None,
            limit: None,
            presences: false,
            user_ids: user_ids.into_iter().collect(),
            nonce: None,
        }
    }
}

impl GatewayCommand for RequestGuildMembers {
    const OPCODE: OpCode = OpCode::RequestGuildMembers;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RequestSoundboardSounds {
    pub guild_ids: Vec<GuildId>,
}

impl RequestSoundboardSounds {
    #[must_use]
    pub fn new(guild_ids: impl IntoIterator<Item = GuildId>) -> Self {
        Self {
            guild_ids: guild_ids.into_iter().collect(),
        }
    }
}

impl GatewayCommand for RequestSoundboardSounds {
    const OPCODE: OpCode = OpCode::RequestSoundboardSounds;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct RequestChannelInfo {
    pub guild_id: GuildId,

    pub fields: Vec<ChannelInfoField>,
}

impl RequestChannelInfo {
    #[must_use]
    pub fn new(guild_id: GuildId, fields: impl IntoIterator<Item = ChannelInfoField>) -> Self {
        Self {
            guild_id,
            fields: fields.into_iter().collect(),
        }
    }
}

impl GatewayCommand for RequestChannelInfo {
    const OPCODE: OpCode = OpCode::RequestChannelInfo;
}

str_enum! {



    pub enum ChannelInfoField {

        Status = "status",

        VoiceStartTime = "voice_start_time",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct UpdateVoiceState {
    pub guild_id: GuildId,

    pub channel_id: Option<ChannelId>,

    pub self_mute: bool,

    pub self_deaf: bool,
}

impl UpdateVoiceState {
    #[must_use]
    pub const fn new(
        guild_id: GuildId,
        channel_id: Option<ChannelId>,
        self_mute: bool,
        self_deaf: bool,
    ) -> Self {
        Self {
            guild_id,
            channel_id,
            self_mute,
            self_deaf,
        }
    }
}

impl GatewayCommand for UpdateVoiceState {
    const OPCODE: OpCode = OpCode::VoiceStateUpdate;
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UpdatePresence {
    #[serde(with = "crate::serde_util::option_unix_millis")]
    pub since: Option<Timestamp>,

    pub activities: Vec<Activity>,

    pub status: Status,

    pub afk: bool,
}

impl UpdatePresence {
    #[must_use]
    pub fn new(status: Status, activities: impl IntoIterator<Item = Activity>) -> Self {
        Self {
            since: None,
            activities: activities.into_iter().collect(),
            status,
            afk: false,
        }
    }
}

impl GatewayCommand for UpdatePresence {
    const OPCODE: OpCode = OpCode::PresenceUpdate;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn identify_json() {
        let identify = Identify {
            token: "secret".into(),
            properties: IdentifyProperties {
                os: "linux".into(),
                browser: "rustydiscord".into(),
                device: "rustydiscord".into(),
            },
            large_threshold: None,
            shard: Some(ShardId::ONE),
            intents: Intents::GUILDS | Intents::GUILD_MESSAGES,
            presence: None,
            capabilities: GatewayCapabilities::empty(),
        };
        let json = serde_json::to_string(&CommandPayload::new(&identify)).unwrap();
        assert_eq!(
            json,
            r#"{"op":2,"d":{"token":"secret","properties":{"os":"linux","browser":"rustydiscord","device":"rustydiscord"},"shard":[0,1],"intents":513}}"#
        );
        assert!(!format!("{identify:?}").contains("secret"));
    }

    #[test]
    fn resume_json() {
        let resume = Resume {
            token: "secret".into(),
            session_id: "abc".into(),
            seq: 1337,
        };
        let json = serde_json::to_string(&CommandPayload::new(&resume)).unwrap();
        assert_eq!(
            json,
            r#"{"op":6,"d":{"token":"secret","session_id":"abc","seq":1337}}"#
        );
        assert!(!format!("{resume:?}").contains("secret"));
    }

    #[test]
    fn first_heartbeat_is_null() {
        let json = serde_json::to_string(&CommandPayload::new(&Heartbeat(None))).unwrap();
        assert_eq!(json, r#"{"op":1,"d":null}"#);
    }
}
