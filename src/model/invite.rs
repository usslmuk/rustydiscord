use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    application::PartialApplication,
    channel::ChannelType,
    guild::{AgeRestrictionLevel, Feature, RoleColors, VerificationLevel, WelcomeScreen},
    guild_scheduled_event::GuildScheduledEvent,
    id::{ChannelId, GuildId, RoleId},
    macros::{int_enum, int_flags},
    user::User,
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Invite {
    #[serde(rename = "type")]
    pub kind: InviteType,

    pub code: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild: Option<Box<InviteGuild>>,

    pub channel: Option<InviteChannel>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_application: Option<Box<PartialApplication>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_presence_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approximate_member_count: Option<u32>,

    pub expires_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_scheduled_event: Option<Box<GuildScheduledEvent>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<InviteFlags>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<InviteRole>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "ExtendedInvite")]
pub struct InviteWithMetadata {
    #[serde(flatten)]
    pub invite: Invite,

    #[serde(flatten)]
    pub metadata: Option<InviteMetadata>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteMetadata {
    pub uses: u32,

    pub max_uses: u32,

    pub max_age: u32,

    pub temporary: bool,

    pub created_at: Timestamp,
}

impl core::ops::Deref for InviteWithMetadata {
    type Target = Invite;

    fn deref(&self) -> &Invite {
        &self.invite
    }
}

int_enum! {



    pub enum InviteType: u8 {

        Guild = 0,

        GroupDm = 1,

        Friend = 2,
    }
}

int_enum! {



    pub enum InviteTargetType: u8 {

        Stream = 1,

        EmbeddedApplication = 2,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteGuild {
    pub id: GuildId,

    pub name: CompactString,

    pub splash: Option<ImageHash>,

    pub banner: Option<ImageHash>,

    pub description: Option<CompactString>,

    pub icon: Option<ImageHash>,

    pub features: Vec<Feature>,

    pub verification_level: VerificationLevel,

    pub vanity_url_code: Option<CompactString>,

    #[serde(default)]
    pub nsfw_level: AgeRestrictionLevel,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_subscription_count: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub welcome_screen: Option<WelcomeScreen>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteChannel {
    pub id: ChannelId,

    #[serde(rename = "type")]
    pub kind: ChannelType,

    pub name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<ImageHash>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub recipients: Vec<InviteRecipient>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteRecipient {
    pub username: CompactString,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteRole {
    pub id: RoleId,

    pub name: CompactString,

    pub position: i32,

    pub color: u32,

    pub colors: RoleColors,

    pub icon: Option<ImageHash>,

    pub unicode_emoji: Option<CompactString>,
}

int_flags! {



    pub struct InviteFlags: u64 {

        const IS_GUEST_INVITE = 1 << 0;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteTargetUsersJobStatus {
    pub status: InviteTargetUsersJobStatusCode,

    pub total_users: u32,

    pub processed_users: u32,

    pub created_at: Timestamp,

    #[serde(default)]
    pub completed_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error_message: Option<CompactString>,
}

int_enum! {




    pub enum InviteTargetUsersJobStatusCode: u8 {

        Unspecified = 0,

        Processing = 1,

        Completed = 2,

        Failed = 3,
    }
}
