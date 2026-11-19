use compact_str::CompactString;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

use super::{guild_id_event, newtype_event};
use crate::{
    application::PartialApplication,
    channel::{Channel, ChannelType, ThreadMember},
    emoji::PartialEmoji,
    gateway::Presence,
    id::{ChannelId, GuildId, RoleId, SoundboardSoundId, UserId},
    invite::InviteTargetType,
    macros::int_enum,
    serde_util::extras::{self, Extras, Visit},
    stage_instance::StageInstance,
    user::User,
    util::Timestamp,
    voice::VoiceState,
};

newtype_event! {




    ChannelCreate(Channel)
}

newtype_event! {




    ChannelUpdate(Channel)
}

newtype_event! {




    ChannelDelete(Channel)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelInfo {
    pub guild_id: GuildId,

    #[serde(default)]
    pub channels: Vec<ChannelInfoChannel>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelInfoChannel {
    pub id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<CompactString>,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_util::option_unix_secs"
    )]
    pub voice_start_time: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceChannelStatusUpdate {
    pub id: ChannelId,

    pub guild_id: GuildId,

    pub status: Option<CompactString>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceChannelStartTimeUpdate {
    pub id: ChannelId,

    pub guild_id: GuildId,

    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_util::option_unix_secs"
    )]
    pub voice_start_time: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ThreadCreate {
    pub thread: Channel,

    pub newly_created: bool,
}

impl core::ops::Deref for ThreadCreate {
    type Target = Channel;

    fn deref(&self) -> &Channel {
        &self.thread
    }
}

#[derive(Default)]
struct ThreadCreateExtra {
    newly_created: bool,
}

impl<'de> Extras<'de> for ThreadCreateExtra {
    fn visit<A: serde::de::MapAccess<'de>>(
        &mut self,
        key: &str,
        map: &mut A,
    ) -> Result<Visit, A::Error> {
        if key == "newly_created" {
            self.newly_created = map.next_value()?;
            Ok(Visit::Consumed)
        } else {
            Ok(Visit::Pass)
        }
    }
}

impl<'de> Deserialize<'de> for ThreadCreate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (thread, extra) = extras::deserialize::<Channel, ThreadCreateExtra, D>(deserializer)?;
        Ok(Self {
            thread: thread.ok_or_else(|| D::Error::custom("missing event data"))?,
            newly_created: extra.newly_created,
        })
    }
}

impl Serialize for ThreadCreate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(flatten)]
            thread: &'a Channel,
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            newly_created: bool,
        }

        Flat {
            thread: &self.thread,
            newly_created: self.newly_created,
        }
        .serialize(serializer)
    }
}

newtype_event! {




    ThreadUpdate(Channel)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadDelete {
    pub id: ChannelId,

    pub guild_id: GuildId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,

    #[serde(rename = "type")]
    pub kind: ChannelType,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadListSync {
    pub guild_id: GuildId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_ids: Option<Vec<ChannelId>>,

    #[serde(default)]
    pub threads: Vec<Channel>,

    #[serde(default)]
    pub members: Vec<ThreadMember>,
}

guild_id_event! {




    ThreadMemberUpdate {

        member: ThreadMember
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadMembersUpdate {
    pub id: ChannelId,

    pub guild_id: GuildId,

    pub member_count: u32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub added_members: Vec<AddedThreadMember>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub removed_member_ids: Vec<UserId>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct AddedThreadMember {
    pub member: ThreadMember,

    pub presence: Option<Presence>,
}

impl core::ops::Deref for AddedThreadMember {
    type Target = ThreadMember;

    fn deref(&self) -> &ThreadMember {
        &self.member
    }
}

#[derive(Default)]
struct PresenceExtra {
    presence: Option<Presence>,
}

impl<'de> Extras<'de> for PresenceExtra {
    fn visit<A: serde::de::MapAccess<'de>>(
        &mut self,
        key: &str,
        map: &mut A,
    ) -> Result<Visit, A::Error> {
        if key == "presence" {
            self.presence = map.next_value()?;
            Ok(Visit::Consumed)
        } else {
            Ok(Visit::Pass)
        }
    }
}

impl<'de> Deserialize<'de> for AddedThreadMember {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (member, extra) = extras::deserialize::<ThreadMember, PresenceExtra, D>(deserializer)?;
        Ok(Self {
            member: member.ok_or_else(|| D::Error::custom("missing event data"))?,
            presence: extra.presence,
        })
    }
}

impl Serialize for AddedThreadMember {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(flatten)]
            member: &'a ThreadMember,
            presence: Option<&'a Presence>,
        }

        Flat {
            member: &self.member,
            presence: self.presence.as_ref(),
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelPinsUpdate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteCreate {
    pub channel_id: ChannelId,

    pub code: CompactString,

    pub created_at: Timestamp,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inviter: Option<Box<User>>,

    pub max_age: u32,

    pub max_uses: u32,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_user: Option<Box<User>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_application: Option<Box<PartialApplication>>,

    #[serde(default)]
    pub temporary: bool,

    #[serde(default)]
    pub uses: u32,

    #[serde(default)]
    pub expires_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub role_ids: Vec<RoleId>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InviteDelete {
    pub channel_id: ChannelId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub code: CompactString,
}

newtype_event! {




    StageInstanceCreate(StageInstance)
}

newtype_event! {




    StageInstanceUpdate(StageInstance)
}

newtype_event! {




    StageInstanceDelete(StageInstance)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WebhooksUpdate {
    pub guild_id: GuildId,

    pub channel_id: ChannelId,
}

newtype_event! {




    VoiceStateUpdate(VoiceState)
}

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceServerUpdate {
    pub token: CompactString,

    pub guild_id: GuildId,

    pub endpoint: Option<CompactString>,
}

impl core::fmt::Debug for VoiceServerUpdate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VoiceServerUpdate")
            .field("token", &"<redacted>")
            .field("guild_id", &self.guild_id)
            .field("endpoint", &self.endpoint)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct VoiceChannelEffectSend {
    pub channel_id: ChannelId,

    pub guild_id: GuildId,

    pub user_id: UserId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub emoji: Option<PartialEmoji>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_type: Option<VoiceChannelEffectAnimationType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub animation_id: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_id: Option<SoundboardSoundId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sound_volume: Option<f64>,
}

int_enum! {



    pub enum VoiceChannelEffectAnimationType: u8 {

        Premium = 0,

        Basic = 1,
    }
}
