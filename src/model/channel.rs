use compact_str::CompactString;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

use crate::model::{
    guild::Member,
    id::{
        ApplicationId, ChannelId, EmojiId, GenericId, GuildId, MessageId, RoleId, TagId, UserId,
        WebhookId,
    },
    macros::{int_enum, int_flags},
    permissions::Permissions,
    user::User,
    util::{ImageHash, Timestamp},
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Channel {
    Text(TextChannel),

    Dm(DmChannel),

    Voice(VoiceChannel),

    GroupDm(GroupDmChannel),

    Category(CategoryChannel),

    Announcement(TextChannel),

    AnnouncementThread(ThreadChannel),

    PublicThread(ThreadChannel),

    PrivateThread(ThreadChannel),

    Stage(VoiceChannel),

    Directory(DirectoryChannel),

    Forum(ForumChannel),

    Media(ForumChannel),

    Unknown(UnknownChannel),
}

impl Channel {
    #[must_use]
    pub const fn id(&self) -> ChannelId {
        match self {
            Self::Text(channel) | Self::Announcement(channel) => channel.id,
            Self::Dm(channel) => channel.id,
            Self::Voice(channel) | Self::Stage(channel) => channel.id,
            Self::GroupDm(channel) => channel.id,
            Self::Category(channel) => channel.id,
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => channel.id,
            Self::Directory(channel) => channel.id,
            Self::Forum(channel) | Self::Media(channel) => channel.id,
            Self::Unknown(channel) => channel.id,
        }
    }

    #[must_use]
    pub const fn kind(&self) -> ChannelType {
        match self {
            Self::Text(_) => ChannelType::GuildText,
            Self::Dm(_) => ChannelType::Dm,
            Self::Voice(_) => ChannelType::GuildVoice,
            Self::GroupDm(_) => ChannelType::GroupDm,
            Self::Category(_) => ChannelType::GuildCategory,
            Self::Announcement(_) => ChannelType::GuildAnnouncement,
            Self::AnnouncementThread(_) => ChannelType::AnnouncementThread,
            Self::PublicThread(_) => ChannelType::PublicThread,
            Self::PrivateThread(_) => ChannelType::PrivateThread,
            Self::Stage(_) => ChannelType::GuildStageVoice,
            Self::Directory(_) => ChannelType::GuildDirectory,
            Self::Forum(_) => ChannelType::GuildForum,
            Self::Media(_) => ChannelType::GuildMedia,
            Self::Unknown(channel) => channel.kind,
        }
    }

    #[must_use]
    pub const fn guild_id(&self) -> Option<GuildId> {
        match self {
            Self::Text(channel) | Self::Announcement(channel) => channel.guild_id,
            Self::Voice(channel) | Self::Stage(channel) => channel.guild_id,
            Self::Category(channel) => channel.guild_id,
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => channel.guild_id,
            Self::Directory(channel) => channel.guild_id,
            Self::Forum(channel) | Self::Media(channel) => channel.guild_id,
            Self::Unknown(channel) => channel.guild_id,
            Self::Dm(_) | Self::GroupDm(_) => None,
        }
    }

    #[must_use]
    pub fn name(&self) -> Option<&str> {
        match self {
            Self::Text(channel) | Self::Announcement(channel) => Some(&channel.name),
            Self::Voice(channel) | Self::Stage(channel) => Some(&channel.name),
            Self::Category(channel) => Some(&channel.name),
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => Some(&channel.name),
            Self::Directory(channel) => Some(&channel.name),
            Self::Forum(channel) | Self::Media(channel) => Some(&channel.name),
            Self::GroupDm(channel) => channel.name.as_deref(),
            Self::Unknown(channel) => channel.name.as_deref(),
            Self::Dm(_) => None,
        }
    }

    #[must_use]
    pub const fn parent_id(&self) -> Option<ChannelId> {
        match self {
            Self::Text(channel) | Self::Announcement(channel) => channel.parent_id,
            Self::Voice(channel) | Self::Stage(channel) => channel.parent_id,
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => channel.parent_id,
            Self::Directory(channel) => channel.parent_id,
            Self::Forum(channel) | Self::Media(channel) => channel.parent_id,
            Self::Unknown(channel) => channel.parent_id,
            Self::Dm(_) | Self::GroupDm(_) | Self::Category(_) => None,
        }
    }

    #[must_use]
    pub const fn flags(&self) -> ChannelFlags {
        match self {
            Self::Text(channel) | Self::Announcement(channel) => channel.flags,
            Self::Dm(channel) => channel.flags,
            Self::Voice(channel) | Self::Stage(channel) => channel.flags,
            Self::GroupDm(channel) => channel.flags,
            Self::Category(channel) => channel.flags,
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => channel.flags,
            Self::Directory(channel) => channel.flags,
            Self::Forum(channel) | Self::Media(channel) => channel.flags,
            Self::Unknown(channel) => channel.flags,
        }
    }

    #[must_use]
    pub fn permission_overwrites(&self) -> &[Overwrite] {
        match self {
            Self::Text(channel) | Self::Announcement(channel) => &channel.permission_overwrites,
            Self::Voice(channel) | Self::Stage(channel) => &channel.permission_overwrites,
            Self::Category(channel) => &channel.permission_overwrites,
            Self::Directory(channel) => &channel.permission_overwrites,
            Self::Forum(channel) | Self::Media(channel) => &channel.permission_overwrites,
            Self::Dm(_)
            | Self::GroupDm(_)
            | Self::AnnouncementThread(_)
            | Self::PublicThread(_)
            | Self::PrivateThread(_)
            | Self::Unknown(_) => &[],
        }
    }

    #[must_use]
    pub const fn as_thread(&self) -> Option<&ThreadChannel> {
        match self {
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => Some(channel),
            _ => None,
        }
    }
}

int_enum! {



    pub enum ChannelType: u8 {

        GuildText = 0,

        Dm = 1,

        GuildVoice = 2,

        GroupDm = 3,

        GuildCategory = 4,

        GuildAnnouncement = 5,

        AnnouncementThread = 10,

        PublicThread = 11,

        PrivateThread = 12,

        GuildStageVoice = 13,

        GuildDirectory = 14,

        GuildForum = 15,

        GuildMedia = 16,
    }
}

impl ChannelType {
    #[must_use]
    pub const fn is_thread(self) -> bool {
        matches!(
            self,
            Self::AnnouncementThread | Self::PublicThread | Self::PrivateThread
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct TextChannel {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    pub position: i32,

    pub permission_overwrites: Vec<Overwrite>,

    pub parent_id: Option<ChannelId>,

    pub nsfw: bool,

    pub topic: Option<CompactString>,

    pub last_message_id: Option<MessageId>,

    pub rate_limit_per_user: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct VoiceChannel {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    pub position: i32,

    pub permission_overwrites: Vec<Overwrite>,

    pub parent_id: Option<ChannelId>,

    pub nsfw: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<CompactString>,

    pub last_message_id: Option<MessageId>,

    pub rate_limit_per_user: u16,

    pub bitrate: u32,

    pub user_limit: u32,

    pub rtc_region: Option<CompactString>,

    pub video_quality_mode: VideoQualityMode,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct CategoryChannel {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    pub position: i32,

    pub permission_overwrites: Vec<Overwrite>,

    pub nsfw: bool,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ForumChannel {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    pub position: i32,

    pub permission_overwrites: Vec<Overwrite>,

    pub parent_id: Option<ChannelId>,

    pub nsfw: bool,

    pub topic: Option<CompactString>,

    pub last_message_id: Option<MessageId>,

    pub rate_limit_per_user: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_auto_archive_duration: Option<AutoArchiveDuration>,

    pub available_tags: Vec<ForumTag>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,

    pub default_reaction_emoji: Option<DefaultReaction>,

    pub default_sort_order: Option<SortOrderType>,

    pub default_forum_layout: ForumLayoutType,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ThreadChannel {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    pub parent_id: Option<ChannelId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<UserId>,

    pub nsfw: bool,

    pub last_message_id: Option<MessageId>,

    pub rate_limit_per_user: u16,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_message_sent: Option<u32>,

    pub thread_metadata: ThreadMetadata,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<ThreadMember>>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub applied_tags: Vec<TagId>,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct DmChannel {
    pub id: ChannelId,

    pub last_message_id: Option<MessageId>,

    pub recipients: Vec<User>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct GroupDmChannel {
    pub id: ChannelId,

    pub name: Option<CompactString>,

    pub icon: Option<ImageHash>,

    pub recipients: Vec<User>,

    pub owner_id: UserId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_id: Option<ApplicationId>,

    pub managed: bool,

    pub last_message_id: Option<MessageId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_pin_timestamp: Option<Timestamp>,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct DirectoryChannel {
    pub id: ChannelId,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    pub name: CompactString,

    pub position: i32,

    pub permission_overwrites: Vec<Overwrite>,

    pub parent_id: Option<ChannelId>,

    pub flags: ChannelFlags,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct UnknownChannel {
    pub id: ChannelId,

    #[serde(skip)]
    pub kind: ChannelType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<GuildId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<ChannelId>,

    pub flags: ChannelFlags,
}

#[derive(Deserialize)]
struct RawChannel {
    id: ChannelId,
    #[serde(rename = "type")]
    kind: ChannelType,
    #[serde(default)]
    guild_id: Option<GuildId>,
    #[serde(default)]
    position: Option<i32>,
    #[serde(default)]
    permission_overwrites: Vec<Overwrite>,
    #[serde(default)]
    name: Option<CompactString>,
    #[serde(default)]
    topic: Option<CompactString>,
    #[serde(default)]
    nsfw: bool,
    #[serde(default)]
    last_message_id: Option<MessageId>,
    #[serde(default)]
    bitrate: Option<u32>,
    #[serde(default)]
    user_limit: Option<u32>,
    #[serde(default)]
    rate_limit_per_user: Option<u16>,
    #[serde(default)]
    recipients: Vec<User>,
    #[serde(default)]
    icon: Option<ImageHash>,
    #[serde(default)]
    owner_id: Option<UserId>,
    #[serde(default)]
    application_id: Option<ApplicationId>,
    #[serde(default)]
    managed: bool,
    #[serde(default)]
    parent_id: Option<ChannelId>,
    #[serde(default)]
    last_pin_timestamp: Option<Timestamp>,
    #[serde(default)]
    rtc_region: Option<CompactString>,
    #[serde(default)]
    video_quality_mode: Option<VideoQualityMode>,
    #[serde(default)]
    message_count: Option<u32>,
    #[serde(default)]
    member_count: Option<u32>,
    #[serde(default)]
    thread_metadata: Option<ThreadMetadata>,
    #[serde(default)]
    member: Option<Box<ThreadMember>>,
    #[serde(default)]
    default_auto_archive_duration: Option<AutoArchiveDuration>,
    #[serde(default)]
    flags: ChannelFlags,
    #[serde(default)]
    total_message_sent: Option<u32>,
    #[serde(default)]
    available_tags: Vec<ForumTag>,
    #[serde(default)]
    applied_tags: Vec<TagId>,
    #[serde(default)]
    default_reaction_emoji: Option<DefaultReaction>,
    #[serde(default)]
    default_thread_rate_limit_per_user: Option<u16>,
    #[serde(default)]
    default_sort_order: Option<SortOrderType>,
    #[serde(default)]
    default_forum_layout: Option<ForumLayoutType>,
}

impl RawChannel {
    fn name<E: DeError>(&mut self) -> Result<CompactString, E> {
        self.name.take().ok_or_else(|| E::missing_field("name"))
    }

    fn into_channel<E: DeError>(mut self) -> Result<Channel, E> {
        Ok(match self.kind {
            ChannelType::GuildText => Channel::Text(self.text()?),
            ChannelType::GuildAnnouncement => Channel::Announcement(self.text()?),
            ChannelType::GuildVoice => Channel::Voice(self.voice()?),
            ChannelType::GuildStageVoice => Channel::Stage(self.voice()?),
            ChannelType::GuildForum => Channel::Forum(self.forum()?),
            ChannelType::GuildMedia => Channel::Media(self.forum()?),
            ChannelType::AnnouncementThread => Channel::AnnouncementThread(self.thread()?),
            ChannelType::PublicThread => Channel::PublicThread(self.thread()?),
            ChannelType::PrivateThread => Channel::PrivateThread(self.thread()?),
            ChannelType::GuildCategory => Channel::Category(CategoryChannel {
                id: self.id,
                guild_id: self.guild_id,
                name: self.name()?,
                position: self.position.unwrap_or_default(),
                permission_overwrites: self.permission_overwrites,
                nsfw: self.nsfw,
                flags: self.flags,
            }),
            ChannelType::Dm => Channel::Dm(DmChannel {
                id: self.id,
                last_message_id: self.last_message_id,
                recipients: self.recipients,
                last_pin_timestamp: self.last_pin_timestamp,
                flags: self.flags,
            }),
            ChannelType::GroupDm => Channel::GroupDm(GroupDmChannel {
                id: self.id,
                name: self.name,
                icon: self.icon,
                recipients: self.recipients,
                owner_id: self.owner_id.ok_or_else(|| E::missing_field("owner_id"))?,
                application_id: self.application_id,
                managed: self.managed,
                last_message_id: self.last_message_id,
                last_pin_timestamp: self.last_pin_timestamp,
                flags: self.flags,
            }),
            ChannelType::GuildDirectory => Channel::Directory(DirectoryChannel {
                id: self.id,
                guild_id: self.guild_id,
                name: self.name()?,
                position: self.position.unwrap_or_default(),
                permission_overwrites: self.permission_overwrites,
                parent_id: self.parent_id,
                flags: self.flags,
            }),
            ChannelType::Unknown(_) => Channel::Unknown(UnknownChannel {
                id: self.id,
                kind: self.kind,
                guild_id: self.guild_id,
                name: self.name,
                parent_id: self.parent_id,
                flags: self.flags,
            }),
        })
    }

    fn text<E: DeError>(mut self) -> Result<TextChannel, E> {
        Ok(TextChannel {
            id: self.id,
            guild_id: self.guild_id,
            name: self.name()?,
            position: self.position.unwrap_or_default(),
            permission_overwrites: self.permission_overwrites,
            parent_id: self.parent_id,
            nsfw: self.nsfw,
            topic: self.topic,
            last_message_id: self.last_message_id,
            rate_limit_per_user: self.rate_limit_per_user.unwrap_or_default(),
            last_pin_timestamp: self.last_pin_timestamp,
            default_auto_archive_duration: self.default_auto_archive_duration,
            default_thread_rate_limit_per_user: self.default_thread_rate_limit_per_user,
            flags: self.flags,
        })
    }

    fn voice<E: DeError>(mut self) -> Result<VoiceChannel, E> {
        Ok(VoiceChannel {
            id: self.id,
            guild_id: self.guild_id,
            name: self.name()?,
            position: self.position.unwrap_or_default(),
            permission_overwrites: self.permission_overwrites,
            parent_id: self.parent_id,
            nsfw: self.nsfw,
            topic: self.topic,
            last_message_id: self.last_message_id,
            rate_limit_per_user: self.rate_limit_per_user.unwrap_or_default(),
            bitrate: self.bitrate.unwrap_or_default(),
            user_limit: self.user_limit.unwrap_or_default(),
            rtc_region: self.rtc_region,
            video_quality_mode: self.video_quality_mode.unwrap_or_default(),
            flags: self.flags,
        })
    }

    fn forum<E: DeError>(mut self) -> Result<ForumChannel, E> {
        Ok(ForumChannel {
            id: self.id,
            guild_id: self.guild_id,
            name: self.name()?,
            position: self.position.unwrap_or_default(),
            permission_overwrites: self.permission_overwrites,
            parent_id: self.parent_id,
            nsfw: self.nsfw,
            topic: self.topic,
            last_message_id: self.last_message_id,
            rate_limit_per_user: self.rate_limit_per_user.unwrap_or_default(),
            default_auto_archive_duration: self.default_auto_archive_duration,
            available_tags: self.available_tags,
            default_thread_rate_limit_per_user: self.default_thread_rate_limit_per_user,
            default_reaction_emoji: self.default_reaction_emoji,
            default_sort_order: self.default_sort_order,
            default_forum_layout: self.default_forum_layout.unwrap_or_default(),
            flags: self.flags,
        })
    }

    fn thread<E: DeError>(mut self) -> Result<ThreadChannel, E> {
        Ok(ThreadChannel {
            id: self.id,
            guild_id: self.guild_id,
            name: self.name()?,
            parent_id: self.parent_id,
            owner_id: self.owner_id,
            nsfw: self.nsfw,
            last_message_id: self.last_message_id,
            rate_limit_per_user: self.rate_limit_per_user.unwrap_or_default(),
            last_pin_timestamp: self.last_pin_timestamp,
            message_count: self.message_count,
            member_count: self.member_count,
            total_message_sent: self.total_message_sent,
            thread_metadata: self
                .thread_metadata
                .ok_or_else(|| E::missing_field("thread_metadata"))?,
            member: self.member,
            applied_tags: self.applied_tags,
            flags: self.flags,
        })
    }
}

impl<'de> Deserialize<'de> for Channel {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        RawChannel::deserialize(deserializer)?.into_channel()
    }
}

#[derive(Serialize)]
struct Tagged<'a, T> {
    #[serde(rename = "type")]
    kind: ChannelType,
    #[serde(flatten)]
    channel: &'a T,
}

impl Serialize for Channel {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let kind = self.kind();
        match self {
            Self::Text(channel) | Self::Announcement(channel) => {
                Tagged { kind, channel }.serialize(serializer)
            }
            Self::Dm(channel) => Tagged { kind, channel }.serialize(serializer),
            Self::Voice(channel) | Self::Stage(channel) => {
                Tagged { kind, channel }.serialize(serializer)
            }
            Self::GroupDm(channel) => Tagged { kind, channel }.serialize(serializer),
            Self::Category(channel) => Tagged { kind, channel }.serialize(serializer),
            Self::AnnouncementThread(channel)
            | Self::PublicThread(channel)
            | Self::PrivateThread(channel) => Tagged { kind, channel }.serialize(serializer),
            Self::Directory(channel) => Tagged { kind, channel }.serialize(serializer),
            Self::Forum(channel) | Self::Media(channel) => {
                Tagged { kind, channel }.serialize(serializer)
            }
            Self::Unknown(channel) => Tagged { kind, channel }.serialize(serializer),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "PermissionOverwrite")]
pub struct Overwrite {
    pub id: GenericId,

    #[serde(rename = "type")]
    pub kind: OverwriteType,

    pub allow: Permissions,

    pub deny: Permissions,
}

impl Overwrite {
    #[must_use]
    pub const fn new(
        id: GenericId,
        kind: OverwriteType,
        allow: Permissions,
        deny: Permissions,
    ) -> Self {
        Self {
            id,
            kind,
            allow,
            deny,
        }
    }

    #[must_use]
    pub const fn role(role_id: RoleId, allow: Permissions, deny: Permissions) -> Self {
        Self::new(role_id.cast(), OverwriteType::Role, allow, deny)
    }

    #[must_use]
    pub const fn member(user_id: UserId, allow: Permissions, deny: Permissions) -> Self {
        Self::new(user_id.cast(), OverwriteType::Member, allow, deny)
    }

    #[must_use]
    pub fn role_id(&self) -> Option<RoleId> {
        (self.kind == OverwriteType::Role).then(|| self.id.cast())
    }

    #[must_use]
    pub fn user_id(&self) -> Option<UserId> {
        (self.kind == OverwriteType::Member).then(|| self.id.cast())
    }
}

int_enum! {



    pub enum OverwriteType: u8 {

        Role = 0,

        Member = 1,
    }
}

int_enum! {




    #[derive(Default)]
    pub enum VideoQualityMode: u8 {

        #[default]
        Auto = 1,

        Full = 2,
    }
}

int_enum! {



    pub enum SortOrderType: u8 {

        LatestActivity = 0,

        CreationDate = 1,
    }
}

int_enum! {



    #[derive(Default)]
    pub enum ForumLayoutType: u8 {

        #[default]
        NotSet = 0,

        ListView = 1,

        GalleryView = 2,
    }
}

int_enum! {




    #[doc(alias = "ThreadAutoArchiveDuration")]
    pub enum AutoArchiveDuration: u16 {

        OneHour = 60,

        OneDay = 1440,

        ThreeDays = 4320,

        OneWeek = 10080,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadMetadata {
    pub archived: bool,

    pub auto_archive_duration: AutoArchiveDuration,

    pub archive_timestamp: Timestamp,

    pub locked: bool,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub invitable: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_timestamp: Option<Timestamp>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ThreadMember {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ChannelId>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    pub join_timestamp: Timestamp,

    pub flags: ThreadMemberFlags,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub member: Option<Box<Member>>,
}

int_flags! {



    pub struct ThreadMemberFlags: u64 {




        const HAS_INTERACTED = 1 << 0;




        const ALL_MESSAGES = 1 << 1;




        const ONLY_MENTIONS = 1 << 2;




        const NO_MESSAGES = 1 << 3;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct DefaultReaction {
    pub emoji_id: Option<EmojiId>,

    pub emoji_name: Option<CompactString>,
}

impl DefaultReaction {
    #[must_use]
    pub const fn custom(emoji_id: EmojiId) -> Self {
        Self {
            emoji_id: Some(emoji_id),
            emoji_name: None,
        }
    }

    #[must_use]
    pub fn unicode(emoji: impl Into<CompactString>) -> Self {
        Self {
            emoji_id: None,
            emoji_name: Some(emoji.into()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ForumTag {
    pub id: TagId,

    pub name: CompactString,

    pub moderated: bool,

    pub emoji_id: Option<EmojiId>,

    pub emoji_name: Option<CompactString>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct FollowedChannel {
    pub channel_id: ChannelId,

    pub webhook_id: WebhookId,
}

int_flags! {



    pub struct ChannelFlags: u64 {




        const GUILD_FEED_REMOVED = 1 << 0;

        const PINNED = 1 << 1;




        const ACTIVE_CHANNELS_REMOVED = 1 << 2;

        const REQUIRE_TAG = 1 << 4;




        const IS_SPAM = 1 << 5;




        const IS_GUILD_RESOURCE_CHANNEL = 1 << 7;




        const CLYDE_AI = 1 << 8;




        const IS_SCHEDULED_FOR_DELETION = 1 << 9;

        const HIDE_MEDIA_DOWNLOAD_OPTIONS = 1 << 15;


        const CHANNEL_OBFUSCATED = 1 << 17;


        const IS_SPOILER_CHANNEL = 1 << 21;
    }
}
