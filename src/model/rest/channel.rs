use core::fmt;

use compact_str::CompactString;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use super::{Attachments, FileUpload};
use crate::model::{
    channel::{
        AutoArchiveDuration, Channel, ChannelFlags, ChannelType, DefaultReaction, ForumLayoutType,
        ForumTag, Overwrite, OverwriteType, SortOrderType, ThreadMember, VideoQualityMode,
    },
    component::Component,
    id::{ApplicationId, ChannelId, EmojiId, RoleId, StickerId, TagId, UserId},
    invite::InviteTargetType,
    message::{AllowedMentions, AttachmentRequest, Embed, Message, MessageFlags},
    permissions::Permissions,
    util::{ImageData, Patch, Timestamp},
};

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildChannel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub icon: Patch<ImageData>,

    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<ChannelType>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub position: Patch<i32>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub topic: Patch<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub nsfw: Patch<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub rate_limit_per_user: Patch<u16>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub bitrate: Patch<u32>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub user_limit: Patch<u32>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub permission_overwrites: Patch<Vec<Overwrite>>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub parent_id: Patch<ChannelId>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub rtc_region: Patch<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub video_quality_mode: Patch<VideoQualityMode>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub default_auto_archive_duration: Patch<AutoArchiveDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_tags: Option<Vec<ForumTagData>>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub default_reaction_emoji: Patch<DefaultReaction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_thread_rate_limit_per_user: Option<u16>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub default_sort_order: Patch<SortOrderType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_forum_layout: Option<ForumLayoutType>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyThread {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub rate_limit_per_user: Patch<u16>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<ChannelFlags>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_tags: Option<Vec<TagId>>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGroupDm {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub icon: Patch<ImageData>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ForumTagData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TagId>,

    pub name: CompactString,

    pub moderated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_id: Option<EmojiId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub emoji_name: Option<CompactString>,
}

impl ForumTagData {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            name: name.into(),
            moderated: false,
            emoji_id: None,
            emoji_name: None,
        }
    }
}

impl From<ForumTag> for ForumTagData {
    fn from(tag: ForumTag) -> Self {
        let ForumTag {
            id,
            name,
            moderated,
            emoji_id,
            emoji_name,
        } = tag;
        Self {
            id: Some(id),
            name,
            moderated,
            emoji_id,
            emoji_name,
        }
    }
}

impl From<&ForumTag> for ForumTagData {
    fn from(tag: &ForumTag) -> Self {
        tag.clone().into()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetChannelPermissions {
    pub implicit: Option<bool>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct SetVoiceChannelStatus {
    pub status: Option<CompactString>,
}

impl SetVoiceChannelStatus {
    #[must_use]
    pub fn new(status: Option<impl Into<CompactString>>) -> Self {
        Self {
            status: status.map(Into::into),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct EditChannelPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow: Option<Permissions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deny: Option<Permissions>,

    #[serde(rename = "type")]
    pub kind: OverwriteType,
}

impl EditChannelPermissions {
    #[must_use]
    pub const fn new(kind: OverwriteType) -> Self {
        Self {
            allow: None,
            deny: None,
            kind,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[non_exhaustive]
pub struct CreateChannelInvite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_age: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u8>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub temporary: bool,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub unique: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_type: Option<InviteTargetType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_user_id: Option<UserId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_application_id: Option<ApplicationId>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub target_user_ids: Vec<UserId>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub role_ids: Vec<RoleId>,

    #[serde(skip)]
    pub target_users_file: Option<FileUpload>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct FollowAnnouncementChannel {
    pub webhook_channel_id: ChannelId,
}

impl FollowAnnouncementChannel {
    #[must_use]
    pub const fn new(webhook_channel_id: ChannelId) -> Self {
        Self { webhook_channel_id }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct GroupDmAddRecipient {
    pub access_token: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nick: Option<CompactString>,
}

impl GroupDmAddRecipient {
    #[must_use]
    pub fn new(access_token: impl Into<String>) -> Self {
        Self {
            access_token: access_token.into(),
            nick: None,
        }
    }
}

impl fmt::Debug for GroupDmAddRecipient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("GroupDmAddRecipient")
            .field("access_token", &"<redacted>")
            .field("nick", &self.nick)
            .finish()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct StartThreadFromMessage {
    pub name: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
}

impl StartThreadFromMessage {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            auto_archive_duration: None,
            rate_limit_per_user: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct StartThreadWithoutMessage {
    pub name: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(rename = "type")]
    pub kind: ChannelType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invitable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,
}

impl StartThreadWithoutMessage {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, kind: ChannelType) -> Self {
        Self {
            name: name.into(),
            auto_archive_duration: None,
            kind,
            invitable: None,
            rate_limit_per_user: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize)]
#[non_exhaustive]
pub struct StartThreadInForumOrMediaChannel {
    pub name: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_archive_duration: Option<AutoArchiveDuration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_limit_per_user: Option<u16>,

    pub message: ForumThreadMessage,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub applied_tags: Vec<TagId>,
}

impl StartThreadInForumOrMediaChannel {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, message: ForumThreadMessage) -> Self {
        Self {
            name: name.into(),
            auto_archive_duration: None,
            rate_limit_per_user: None,
            message,
            applied_tags: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ForumThreadMessage {
    pub content: Option<CompactString>,

    pub embeds: Vec<Embed>,

    pub allowed_mentions: Option<AllowedMentions>,

    pub components: Vec<Component>,

    pub sticker_ids: Vec<StickerId>,

    pub attachments: Option<Vec<AttachmentRequest>>,

    pub flags: Option<MessageFlags>,

    pub files: Vec<FileUpload>,
}

impl From<&str> for ForumThreadMessage {
    fn from(content: &str) -> Self {
        Self {
            content: Some(content.into()),
            ..Self::default()
        }
    }
}

impl From<String> for ForumThreadMessage {
    fn from(content: String) -> Self {
        Self {
            content: Some(content.into()),
            ..Self::default()
        }
    }
}

impl Serialize for ForumThreadMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            embeds: &'a [Embed],
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_mentions: Option<&'a AllowedMentions>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            components: &'a [Component],
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            sticker_ids: &'a [StickerId],
            #[serde(skip_serializing_if = "Attachments::is_absent")]
            attachments: Attachments<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            flags: Option<MessageFlags>,
        }

        let Self {
            content,
            embeds,
            allowed_mentions,
            components,
            sticker_ids,
            attachments,
            flags,
            files,
        } = self;
        Wire {
            content: content.as_ref(),
            embeds,
            allowed_mentions: allowed_mentions.as_ref(),
            components,
            sticker_ids,
            attachments: Attachments::new(attachments.as_deref(), files),
            flags: *flags,
        }
        .serialize(serializer)
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ForumThread {
    pub channel: Channel,

    pub message: Option<Message>,
}

impl<'de> Deserialize<'de> for ForumThread {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        struct Wire {
            #[serde(flatten)]
            channel: Channel,
            #[serde(default)]
            message: Option<Message>,
        }

        let Wire { channel, message } = Wire::deserialize(deserializer)?;
        Ok(Self { channel, message })
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetThreadMember {
    pub with_member: Option<bool>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListThreadMembers {
    pub with_member: Option<bool>,

    pub after: Option<UserId>,

    pub limit: Option<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListArchivedThreads {
    pub before: Option<Timestamp>,

    pub limit: Option<u16>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ListJoinedPrivateArchivedThreads {
    pub before: Option<ChannelId>,

    pub limit: Option<u16>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ArchivedThreads {
    pub threads: Vec<Channel>,

    pub members: Vec<ThreadMember>,

    pub has_more: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::id::GenericId;

    #[test]
    fn modify_bodies() {
        let mut channel = ModifyGuildChannel {
            parent_id: Patch::Clear,
            user_limit: Patch::Set(10),
            ..ModifyGuildChannel::default()
        };
        channel.permission_overwrites = Patch::Set(vec![Overwrite {
            id: GenericId::new(5),
            kind: OverwriteType::Member,
            allow: Permissions::VIEW_CHANNEL,
            deny: Permissions::empty(),
        }]);
        assert_eq!(
            serde_json::to_string(&channel).unwrap(),
            concat!(
                r#"{"user_limit":10,"permission_overwrites":[{"id":"5","type":1,"allow":"1024","#,
                r#""deny":"0"}],"parent_id":null}"#
            )
        );

        let thread = ModifyThread {
            archived: Some(false),
            rate_limit_per_user: Patch::Clear,
            applied_tags: Some(vec![TagId::new(3)]),
            ..ModifyThread::default()
        };
        assert_eq!(
            serde_json::to_string(&thread).unwrap(),
            r#"{"archived":false,"rate_limit_per_user":null,"applied_tags":["3"]}"#
        );

        let group = ModifyGroupDm {
            icon: Patch::Set(ImageData::with_content_type(&b"GIF89a"[..], "image/gif")),
            ..ModifyGroupDm::default()
        };
        assert_eq!(
            serde_json::to_string(&group).unwrap(),
            r#"{"icon":"data:image/gif;base64,R0lGODlh"}"#
        );
    }

    #[test]
    fn invite_and_recipient_bodies() {
        let invite = CreateChannelInvite {
            max_age: Some(0),
            unique: true,
            target_users_file: Some(FileUpload::new("users.csv", "1\n")),
            ..CreateChannelInvite::default()
        };
        assert_eq!(
            serde_json::to_string(&invite).unwrap(),
            r#"{"max_age":0,"unique":true}"#
        );
        assert_eq!(
            serde_json::to_string(&CreateChannelInvite::default()).unwrap(),
            "{}"
        );

        let recipient = GroupDmAddRecipient::new("super-secret");
        assert!(!format!("{recipient:?}").contains("super-secret"));
        assert_eq!(
            serde_json::to_string(&recipient).unwrap(),
            r#"{"access_token":"super-secret"}"#
        );
    }

    #[test]
    fn forum_thread_with_files() {
        let mut message = ForumThreadMessage::from("see log");
        message.files.push(FileUpload::new("log.txt", "ok"));
        let mut body = StartThreadInForumOrMediaChannel::new("Report", message);
        body.applied_tags.push(TagId::new(9));
        assert_eq!(
            serde_json::to_string(&body).unwrap(),
            concat!(
                r#"{"name":"Report","message":{"content":"see log","attachments":"#,
                r#"[{"id":0,"filename":"log.txt"}]},"applied_tags":["9"]}"#
            )
        );
    }

    #[test]
    fn forum_tags_round_trip() {
        let tag: ForumTag = serde_json::from_str(
            r#"{"id":"7","name":"bug","moderated":true,"emoji_id":null,"emoji_name":"🐛"}"#,
        )
        .unwrap();
        assert_eq!(
            serde_json::to_string(&ForumTagData::from(&tag)).unwrap(),
            r#"{"id":"7","name":"bug","moderated":true,"emoji_name":"🐛"}"#
        );
    }
}
