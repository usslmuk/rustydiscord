use core::fmt;
use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::Serialize;

use crate::{id::UserId, lobby::LobbyMemberFlags, message::MessageFlags, util::Patch};

pub type LobbyMetadata = BTreeMap<CompactString, CompactString>;

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateLobby {
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: LobbyMetadata,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<LobbyMemberData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct LobbyMemberData {
    pub id: UserId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LobbyMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<LobbyMemberFlags>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub additional_name: Patch<CompactString>,
}

impl LobbyMemberData {
    #[must_use]
    pub const fn new(id: UserId) -> Self {
        Self {
            id,
            metadata: None,
            flags: None,
            additional_name: Patch::Keep,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CreateOrJoinLobby {
    pub secret: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lobby_metadata: Option<LobbyMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_metadata: Option<LobbyMetadata>,
}

impl CreateOrJoinLobby {
    #[must_use]
    pub fn new(secret: impl Into<CompactString>) -> Self {
        Self {
            secret: secret.into(),
            idle_timeout_seconds: None,
            lobby_metadata: None,
            member_metadata: None,
        }
    }
}

impl fmt::Debug for CreateOrJoinLobby {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateOrJoinLobby")
            .field("secret", &"<redacted>")
            .field("idle_timeout_seconds", &self.idle_timeout_seconds)
            .field("lobby_metadata", &self.lobby_metadata)
            .field("member_metadata", &self.member_metadata)
            .finish()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ModifyLobby {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LobbyMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<LobbyMemberData>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u32>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct AddLobbyMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<LobbyMetadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<LobbyMemberFlags>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub additional_name: Patch<CompactString>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct BulkLobbyMember {
    #[serde(flatten)]
    pub member: LobbyMemberData,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub remove_member: bool,
}

impl BulkLobbyMember {
    #[must_use]
    pub const fn upsert(member: LobbyMemberData) -> Self {
        Self {
            member,
            remove_member: false,
        }
    }

    #[must_use]
    pub const fn remove(id: UserId) -> Self {
        Self {
            member: LobbyMemberData::new(id),
            remove_member: true,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct SendLobbyMessage {
    pub content: CompactString,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub metadata: LobbyMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<MessageFlags>,
}

impl SendLobbyMessage {
    #[must_use]
    pub fn new(content: impl Into<CompactString>) -> Self {
        Self {
            content: content.into(),
            metadata: BTreeMap::new(),
            flags: None,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetLobbyMessages {
    pub limit: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn member_bodies() {
        let add = AddLobbyMember {
            flags: Some(LobbyMemberFlags::CAN_LINK_LOBBY),
            additional_name: Patch::Clear,
            ..AddLobbyMember::default()
        };
        assert_eq!(
            serde_json::to_string(&add).unwrap(),
            r#"{"flags":1,"additional_name":null}"#
        );
        let mut join = CreateOrJoinLobby::new("lobby-secret");
        join.member_metadata = Some(LobbyMetadata::new());
        assert_eq!(
            serde_json::to_string(&join).unwrap(),
            r#"{"secret":"lobby-secret","member_metadata":{}}"#
        );
        assert!(!format!("{join:?}").contains("lobby-secret"));
    }
}
