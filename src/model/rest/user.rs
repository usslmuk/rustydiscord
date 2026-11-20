use core::fmt;
use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    guild::Feature,
    id::{GuildId, UserId},
    permissions::Permissions,
    util::{ImageData, ImageHash, Patch},
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyCurrentUser {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub avatar: Patch<ImageData>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub banner: Patch<ImageData>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetCurrentUserGuilds {
    pub before: Option<GuildId>,

    pub after: Option<GuildId>,

    pub limit: Option<u8>,

    pub shard: Option<u16>,

    pub with_counts: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CurrentUserGuild {
    pub id: GuildId,

    pub name: CompactString,

    pub icon: Option<ImageHash>,

    #[serde(default)]
    pub banner: Option<ImageHash>,

    pub owner: bool,

    pub permissions: Permissions,

    pub features: Vec<Feature>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateDm {
    pub recipient_id: UserId,
}

impl CreateDm {
    #[must_use]
    pub const fn new(recipient_id: UserId) -> Self {
        Self { recipient_id }
    }
}

#[derive(Clone, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGroupDm {
    pub access_tokens: Vec<String>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub nicks: BTreeMap<UserId, CompactString>,
}

impl CreateGroupDm {
    #[must_use]
    pub fn new(access_tokens: Vec<String>) -> Self {
        Self {
            access_tokens,
            nicks: BTreeMap::new(),
        }
    }
}

impl fmt::Debug for CreateGroupDm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CreateGroupDm")
            .field(
                "access_tokens",
                &format_args!("<{} redacted>", self.access_tokens.len()),
            )
            .field("nicks", &self.nicks)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bodies() {
        let modify = ModifyCurrentUser {
            username: Some("ferris".into()),
            avatar: Patch::Clear,
            ..ModifyCurrentUser::default()
        };
        assert_eq!(
            serde_json::to_string(&modify).unwrap(),
            r#"{"username":"ferris","avatar":null}"#
        );

        let mut group = CreateGroupDm::new(vec!["super-secret".into()]);
        group.nicks.insert(UserId::new(7), "Ferris".into());
        assert_eq!(
            serde_json::to_string(&group).unwrap(),
            r#"{"access_tokens":["super-secret"],"nicks":{"7":"Ferris"}}"#
        );
        let debug = format!("{group:?}");
        assert!(!debug.contains("super-secret"), "{debug}");
        assert!(debug.contains("<1 redacted>"), "{debug}");
    }

    #[test]
    fn current_user_guild() {
        let guild: CurrentUserGuild = serde_json::from_str(
            r#"{
                "id": "80351110224678912", "name": "1337 Krew",
                "icon": "8342729096ea3675442027381ff50dfe",
                "banner": "bb42bdc37653b7cf58c4c8cc622e76cb", "owner": true,
                "permissions": "36953089", "features": ["COMMUNITY", "NEWS"],
                "approximate_member_count": 3268, "approximate_presence_count": 784
            }"#,
        )
        .unwrap();
        assert!(guild.owner);
        assert!(
            guild
                .permissions
                .contains(Permissions::CREATE_INSTANT_INVITE)
        );
        assert_eq!(guild.approximate_member_count, Some(3268));
    }
}
