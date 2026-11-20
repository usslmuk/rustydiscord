use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    channel::Channel,
    id::{ApplicationId, ChannelId, LobbyId, MessageId, UserId},
    macros::int_flags,
    message::{MessageFlags, MessageType},
    user::User,
    util::Timestamp,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Lobby {
    pub id: LobbyId,

    pub application_id: ApplicationId,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub metadata: BTreeMap<CompactString, CompactString>,

    #[serde(default)]
    pub members: Vec<LobbyMember>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_channel: Option<Box<Channel>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LobbyMember {
    pub id: UserId,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub metadata: BTreeMap<CompactString, CompactString>,

    #[serde(default, skip_serializing_if = "LobbyMemberFlags::is_empty")]
    pub flags: LobbyMemberFlags,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub additional_name: Option<CompactString>,
}

int_flags! {



    pub struct LobbyMemberFlags: u64 {

        const CAN_LINK_LOBBY = 1 << 0;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LobbyMessage {
    pub id: MessageId,

    #[serde(rename = "type")]
    pub kind: MessageType,

    pub content: CompactString,

    pub lobby_id: LobbyId,

    pub channel_id: ChannelId,

    pub author: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lobby_member: Option<LobbyMessageMember>,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub metadata: BTreeMap<CompactString, CompactString>,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::model::serde_util::null_as_default"
    )]
    pub moderation_metadata: BTreeMap<CompactString, CompactString>,

    #[serde(default)]
    pub flags: MessageFlags,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edited_timestamp: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LobbyMessageMember {
    pub additional_name: CompactString,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct LobbyInvite {
    pub code: CompactString,
}
