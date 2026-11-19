use compact_str::CompactString;
use serde::{
    Deserialize, Deserializer, Serialize, Serializer,
    de::{Error as DeError, MapAccess},
};

use super::{guild_id_event, newtype_event};
use crate::{
    application_command::GuildCommandPermissions,
    audit_log::AuditLogEntry,
    auto_moderation::{Action, Rule, TriggerType},
    channel::Channel,
    emoji::Emoji,
    gateway::Presence,
    guild::{Guild, Integration, Member, MemberFlags, Role, UnavailableGuild},
    guild_scheduled_event::GuildScheduledEvent,
    id::{
        ApplicationId, AutoModerationRuleId, ChannelId, GuildId, IntegrationId, MessageId, RoleId,
        ScheduledEventId, SoundboardSoundId, UserId,
    },
    serde_util::extras::{self, Extras, Visit},
    soundboard::SoundboardSound,
    stage_instance::StageInstance,
    sticker::Sticker,
    user::{AvatarDecorationData, Collectibles, User},
    util::{ImageHash, Timestamp},
    voice::VoiceState,
};

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum GuildCreate {
    Available(Box<AvailableGuild>),

    Unavailable(UnavailableGuild),
}

impl GuildCreate {
    #[must_use]
    pub const fn id(&self) -> GuildId {
        match self {
            Self::Available(guild) => guild.guild.id,
            Self::Unavailable(guild) => guild.id,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct AvailableGuild {
    pub guild: Guild,

    pub joined_at: Option<Timestamp>,

    pub large: bool,

    pub member_count: u32,

    pub voice_states: Vec<VoiceState>,

    pub members: Vec<Member>,

    pub channels: Vec<Channel>,

    pub threads: Vec<Channel>,

    pub presences: Vec<Presence>,

    pub stage_instances: Vec<StageInstance>,

    pub guild_scheduled_events: Vec<GuildScheduledEvent>,

    pub soundboard_sounds: Vec<SoundboardSound>,
}

impl core::ops::Deref for AvailableGuild {
    type Target = Guild;

    fn deref(&self) -> &Guild {
        &self.guild
    }
}

impl core::ops::DerefMut for AvailableGuild {
    fn deref_mut(&mut self) -> &mut Guild {
        &mut self.guild
    }
}

#[derive(Default)]
struct GuildCreateExtra {
    id: Option<GuildId>,
    unavailable: Option<bool>,
    joined_at: Option<Timestamp>,
    large: bool,
    member_count: u32,
    voice_states: Vec<VoiceState>,
    members: Vec<Member>,
    channels: Vec<Channel>,
    threads: Vec<Channel>,
    presences: Vec<Presence>,
    stage_instances: Vec<StageInstance>,
    guild_scheduled_events: Vec<GuildScheduledEvent>,
    soundboard_sounds: Vec<SoundboardSound>,
}

impl<'de> Extras<'de> for GuildCreateExtra {
    fn visit<A: MapAccess<'de>>(&mut self, key: &str, map: &mut A) -> Result<Visit, A::Error> {
        match key {
            "id" => {
                let id: GuildId = map.next_value()?;
                self.id = Some(id);
                return Ok(Visit::Replay(id.get()));
            }
            "unavailable" => self.unavailable = map.next_value()?,
            "joined_at" => self.joined_at = map.next_value()?,
            "large" => self.large = map.next_value()?,
            "member_count" => self.member_count = map.next_value()?,
            "voice_states" => self.voice_states = map.next_value()?,
            "members" => self.members = map.next_value()?,
            "channels" => self.channels = map.next_value()?,
            "threads" => self.threads = map.next_value()?,
            "presences" => self.presences = map.next_value()?,
            "stage_instances" => self.stage_instances = map.next_value()?,
            "guild_scheduled_events" => self.guild_scheduled_events = map.next_value()?,
            "soundboard_sounds" => self.soundboard_sounds = map.next_value()?,
            _ => return Ok(Visit::Pass),
        }
        Ok(Visit::Consumed)
    }

    fn recover(&self) -> bool {
        self.unavailable == Some(true) && self.id.is_some()
    }
}

impl<'de> Deserialize<'de> for GuildCreate {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let (guild, extra) = extras::deserialize::<Guild, GuildCreateExtra, D>(deserializer)?;
        let Some(guild) = guild else {
            let id = extra.id.ok_or_else(|| D::Error::missing_field("id"))?;
            return Ok(Self::Unavailable(UnavailableGuild::new(id, true)));
        };
        Ok(Self::Available(Box::new(AvailableGuild {
            guild,
            joined_at: extra.joined_at,
            large: extra.large,
            member_count: extra.member_count,
            voice_states: extra.voice_states,
            members: extra.members,
            channels: extra.channels,
            threads: extra.threads,
            presences: extra.presences,
            stage_instances: extra.stage_instances,
            guild_scheduled_events: extra.guild_scheduled_events,
            soundboard_sounds: extra.soundboard_sounds,
        })))
    }
}

impl Serialize for GuildCreate {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(flatten)]
            guild: &'a Guild,
            #[serde(skip_serializing_if = "Option::is_none")]
            joined_at: Option<Timestamp>,
            large: bool,
            member_count: u32,
            voice_states: &'a [VoiceState],
            members: &'a [Member],
            channels: &'a [Channel],
            threads: &'a [Channel],
            presences: &'a [Presence],
            stage_instances: &'a [StageInstance],
            guild_scheduled_events: &'a [GuildScheduledEvent],
            soundboard_sounds: &'a [SoundboardSound],
        }

        match self {
            Self::Available(guild) => Flat {
                guild: &guild.guild,
                joined_at: guild.joined_at,
                large: guild.large,
                member_count: guild.member_count,
                voice_states: &guild.voice_states,
                members: &guild.members,
                channels: &guild.channels,
                threads: &guild.threads,
                presences: &guild.presences,
                stage_instances: &guild.stage_instances,
                guild_scheduled_events: &guild.guild_scheduled_events,
                soundboard_sounds: &guild.soundboard_sounds,
            }
            .serialize(serializer),
            Self::Unavailable(guild) => guild.serialize(serializer),
        }
    }
}

newtype_event! {




    GuildUpdate(Guild)
}

newtype_event! {





    GuildDelete(UnavailableGuild)
}

newtype_event! {




    GuildAuditLogEntryCreate(AuditLogEntry)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildBanAdd {
    pub guild_id: GuildId,

    pub user: User,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildBanRemove {
    pub guild_id: GuildId,

    pub user: User,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildEmojisUpdate {
    pub guild_id: GuildId,

    pub emojis: Vec<Emoji>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildStickersUpdate {
    pub guild_id: GuildId,

    pub stickers: Vec<Sticker>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildIntegrationsUpdate {
    pub guild_id: GuildId,
}

guild_id_event! {




    GuildMemberAdd {

        member: Member
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMemberRemove {
    pub guild_id: GuildId,

    pub user: User,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMemberUpdate {
    pub guild_id: GuildId,

    pub roles: Vec<RoleId>,

    pub user: User,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nick: Option<CompactString>,

    #[serde(default)]
    pub avatar: Option<ImageHash>,

    #[serde(default)]
    pub banner: Option<ImageHash>,

    #[serde(default)]
    pub joined_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub premium_since: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deaf: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mute: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub communication_disabled_until: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<MemberFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub avatar_decoration_data: Option<Box<AvatarDecorationData>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collectibles: Option<Box<Collectibles>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildMembersChunk {
    pub guild_id: GuildId,

    pub members: Vec<Member>,

    pub chunk_index: u32,

    pub chunk_count: u32,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub not_found: Vec<UserId>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presences: Vec<Presence>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nonce: Option<CompactString>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildRoleCreate {
    pub guild_id: GuildId,

    pub role: Role,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildRoleUpdate {
    pub guild_id: GuildId,

    pub role: Role,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildRoleDelete {
    pub guild_id: GuildId,

    pub role_id: RoleId,
}

newtype_event! {




    GuildScheduledEventCreate(GuildScheduledEvent)
}

newtype_event! {




    GuildScheduledEventUpdate(GuildScheduledEvent)
}

newtype_event! {




    GuildScheduledEventDelete(GuildScheduledEvent)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildScheduledEventUserAdd {
    pub guild_scheduled_event_id: ScheduledEventId,

    pub user_id: UserId,

    pub guild_id: GuildId,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildScheduledEventUserRemove {
    pub guild_scheduled_event_id: ScheduledEventId,

    pub user_id: UserId,

    pub guild_id: GuildId,
}

newtype_event! {




    GuildSoundboardSoundCreate(SoundboardSound)
}

newtype_event! {




    GuildSoundboardSoundUpdate(SoundboardSound)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildSoundboardSoundDelete {
    pub sound_id: SoundboardSoundId,

    pub guild_id: GuildId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GuildSoundboardSoundsUpdate {
    pub soundboard_sounds: Vec<SoundboardSound>,

    pub guild_id: GuildId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SoundboardSounds {
    pub soundboard_sounds: Vec<SoundboardSound>,

    pub guild_id: GuildId,
}

guild_id_event! {




    IntegrationCreate {

        integration: Integration
    }
}

guild_id_event! {




    IntegrationUpdate {

        integration: Integration
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct IntegrationDelete {
    pub id: IntegrationId,

    pub guild_id: GuildId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,
}

newtype_event! {




    AutoModerationRuleCreate(Rule)
}

newtype_event! {




    AutoModerationRuleUpdate(Rule)
}

newtype_event! {




    AutoModerationRuleDelete(Rule)
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AutoModerationActionExecution {
    pub guild_id: GuildId,

    pub action: Action,

    pub rule_id: AutoModerationRuleId,

    pub rule_trigger_type: TriggerType,

    pub user_id: UserId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<MessageId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alert_system_message_id: Option<MessageId>,

    #[serde(default)]
    pub content: CompactString,

    #[serde(default)]
    pub matched_keyword: Option<CompactString>,

    #[serde(default)]
    pub matched_content: Option<CompactString>,
}

newtype_event! {




    ApplicationCommandPermissionsUpdate(GuildCommandPermissions)
}
