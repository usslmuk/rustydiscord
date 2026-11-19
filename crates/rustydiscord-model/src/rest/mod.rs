use core::fmt::{self, Write as _};

use bytes::Bytes;
use compact_str::CompactString;
use serde::{Deserialize, Serialize, Serializer, ser::SerializeSeq};

use crate::{
    id::UserId,
    message::{AttachmentRequest, AttachmentRequestId},
};

mod application;
mod application_command;
mod application_identity_profile;
mod audit_log;
mod auto_moderation;
mod channel;
mod emoji;
mod entitlement;
mod error_code;
mod guild;
mod guild_scheduled_event;
mod guild_template;
mod interaction;
mod invite;
mod lobby;
mod message;
mod poll;
mod soundboard;
mod stage_instance;
mod sticker;
mod subscription;
mod user;
mod voice;
mod webhook;

pub(crate) mod setters;

pub use self::{
    application::EditCurrentApplication,
    application_command::{
        CreateApplicationCommand, EditApplicationCommand, EditApplicationCommandPermissions,
        GetApplicationCommands,
    },
    application_identity_profile::{
        DeleteApplicationIdentity, GetApplicationIdentitiesByExternalId,
        UpdateApplicationIdentityProfile,
    },
    audit_log::GetGuildAuditLog,
    auto_moderation::{CreateAutoModerationRule, ModifyAutoModerationRule},
    channel::{
        ArchivedThreads, CreateChannelInvite, EditChannelPermissions, FollowAnnouncementChannel,
        ForumTagData, ForumThread, ForumThreadMessage, GetChannelPermissions, GetThreadMember,
        GroupDmAddRecipient, ListArchivedThreads, ListJoinedPrivateArchivedThreads,
        ListThreadMembers, ModifyGroupDm, ModifyGuildChannel, ModifyThread, SetVoiceChannelStatus,
        StartThreadFromMessage, StartThreadInForumOrMediaChannel, StartThreadWithoutMessage,
    },
    emoji::{CreateApplicationEmoji, CreateGuildEmoji, ModifyApplicationEmoji, ModifyGuildEmoji},
    entitlement::{CreateTestEntitlement, ListEntitlements},
    error_code::ErrorCode,
    guild::{
        ActiveThreads, AddGuildMember, BeginGuildPrune, BulkBanResponse, BulkGuildBan,
        CreateGuildBan, CreateGuildChannel, CreateGuildRole, GetGuild, GetGuildBans,
        GetGuildPruneCount, ListGuildMembers, ModifyCurrentMember, ModifyGuild,
        ModifyGuildChannelPosition, ModifyGuildIncidentActions, ModifyGuildMember,
        ModifyGuildOnboarding, ModifyGuildRole, ModifyGuildRolePosition, ModifyGuildWelcomeScreen,
        ModifyGuildWidget, OnboardingPromptData, PromptOptionData, SearchGuildMembers, VanityUrl,
    },
    guild_scheduled_event::{
        CreateGuildScheduledEvent, GetGuildScheduledEvent, GetGuildScheduledEventUsers,
        ListScheduledEventsForGuild, ModifyGuildScheduledEvent, RecurrenceRuleData,
    },
    guild_template::{CreateGuildTemplate, ModifyGuildTemplate},
    interaction::CreateFollowupMessage,
    invite::{BulkTargetUsers, GetInvite},
    lobby::{
        AddLobbyMember, BulkLobbyMember, CreateLobby, CreateOrJoinLobby, GetLobbyMessages,
        LobbyMemberData, LobbyMetadata, ModifyLobby, SendLobbyMessage,
    },
    message::{
        BulkDeleteMessages, ChannelPins, CreateMessage, EditMessage, GetChannelMessages,
        GetChannelPins, GetReactions, MessageSearchResults, SearchAuthorType, SearchEmbedType,
        SearchFilter, SearchGuildMessages, SearchHasType, SearchSortMode, SearchSortOrder,
    },
    poll::GetAnswerVoters,
    soundboard::{CreateGuildSoundboardSound, ModifyGuildSoundboardSound, SendSoundboardSound},
    stage_instance::{CreateStageInstance, ModifyStageInstance},
    sticker::{CreateGuildSticker, ModifyGuildSticker},
    subscription::ListSkuSubscriptions,
    user::{CreateDm, CreateGroupDm, CurrentUserGuild, GetCurrentUserGuilds, ModifyCurrentUser},
    voice::{ModifyCurrentUserVoiceState, ModifyUserVoiceState},
    webhook::{CreateWebhook, EditWebhookMessage, ExecuteWebhook, ModifyWebhook},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Gateway {
    pub url: CompactString,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct GatewayBot {
    pub url: CompactString,

    pub shards: u32,

    pub session_start_limit: SessionStartLimit,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SessionStartLimit {
    pub total: u32,

    pub remaining: u32,

    pub reset_after: u64,

    pub max_concurrency: u32,
}

#[derive(Clone, PartialEq)]
#[non_exhaustive]
pub struct FileUpload {
    pub name: CompactString,

    pub data: Bytes,

    pub content_type: Option<CompactString>,

    pub description: Option<CompactString>,

    pub title: Option<CompactString>,

    pub duration_secs: Option<f64>,

    pub waveform: Option<CompactString>,

    pub is_spoiler: bool,
}

impl FileUpload {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, data: impl Into<Bytes>) -> Self {
        Self {
            name: name.into(),
            data: data.into(),
            content_type: None,
            description: None,
            title: None,
            duration_secs: None,
            waveform: None,
            is_spoiler: false,
        }
    }

    #[must_use]
    pub fn target_users(user_ids: impl IntoIterator<Item = UserId>) -> Self {
        let mut csv = String::new();
        for (index, id) in user_ids.into_iter().enumerate() {
            if index > 0 {
                csv.push('\n');
            }

            let _ = write!(csv, "{id}");
        }
        Self {
            content_type: Some(CompactString::const_new("text/csv")),
            ..Self::new("users.csv", csv)
        }
    }
}

impl fmt::Debug for FileUpload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileUpload")
            .field("name", &self.name)
            .field("data", &format_args!("<{} bytes>", self.data.len()))
            .field("content_type", &self.content_type)
            .field("description", &self.description)
            .field("title", &self.title)
            .field("duration_secs", &self.duration_secs)
            .field("waveform", &self.waveform)
            .field("is_spoiler", &self.is_spoiler)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub(crate) struct Attachments<'a> {
    explicit: Option<&'a [AttachmentRequest]>,
    files: &'a [FileUpload],
}

impl<'a> Attachments<'a> {
    pub(crate) const fn new(
        explicit: Option<&'a [AttachmentRequest]>,
        files: &'a [FileUpload],
    ) -> Self {
        Self { explicit, files }
    }

    pub(crate) const fn is_absent(&self) -> bool {
        self.explicit.is_none() && self.files.is_empty()
    }

    pub(crate) fn from_patch(
        attachments: &'a crate::util::Patch<Vec<AttachmentRequest>>,
        files: &'a [FileUpload],
    ) -> Self {
        use crate::util::Patch;

        let explicit = match attachments {
            Patch::Keep => None,
            Patch::Clear => Some(&[][..]),
            Patch::Set(attachments) => Some(attachments.as_slice()),
        };
        Self::new(explicit, files)
    }

    fn generated(self) -> impl Iterator<Item = (u32, &'a FileUpload)> {
        let explicit = self.explicit.unwrap_or_default();
        self.files
            .iter()
            .enumerate()
            .map(|(index, file)| (u32::try_from(index).unwrap_or(u32::MAX), file))
            .filter(move |(index, _)| {
                !explicit
                    .iter()
                    .any(|entry| entry.id == AttachmentRequestId::Upload(*index))
            })
    }
}

impl Serialize for Attachments<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Generated<'a> {
            id: u32,
            filename: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            title: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            description: Option<&'a str>,
            #[serde(skip_serializing_if = "Option::is_none")]
            duration_secs: Option<f64>,
            #[serde(skip_serializing_if = "Option::is_none")]
            waveform: Option<&'a str>,
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            is_spoiler: bool,
        }

        let explicit = self.explicit.unwrap_or_default();
        let mut seq = serializer.serialize_seq(Some(explicit.len() + self.generated().count()))?;
        for entry in explicit {
            seq.serialize_element(entry)?;
        }
        for (id, file) in self.generated() {
            seq.serialize_element(&Generated {
                id,
                filename: &file.name,
                title: file.title.as_deref(),
                description: file.description.as_deref(),
                duration_secs: file.duration_secs,
                waveform: file.waveform.as_deref(),
                is_spoiler: file.is_spoiler,
            })?;
        }
        seq.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gateway_bot_documented_example() {
        let json = r#"{
            "url": "wss://gateway.discord.gg/",
            "shards": 9,
            "session_start_limit": {
                "total": 1000,
                "remaining": 999,
                "reset_after": 14400000,
                "max_concurrency": 1
            }
        }"#;
        let info: GatewayBot = serde_json::from_str(json).unwrap();
        assert_eq!(info.url, "wss://gateway.discord.gg/");
        assert_eq!(info.shards, 9);
        assert_eq!(info.session_start_limit.max_concurrency, 1);
    }

    fn json(body: &CreateMessage) -> String {
        serde_json::to_string(body).unwrap()
    }

    #[test]
    fn files_get_attachment_entries() {
        let mut body = CreateMessage::default();
        assert_eq!(json(&body), "{}");

        let mut voice = FileUpload::new("voice.ogg", Vec::from(*b"OggS"));
        voice.duration_secs = Some(2.5);
        voice.waveform = Some("AAAA".into());
        let mut image = FileUpload::new("ferris.png", &b"png"[..]);
        image.title = Some("Ferris".into());
        image.description = Some("Ferris the crab".into());
        image.is_spoiler = true;
        body.files = vec![voice, image];
        assert_eq!(
            json(&body),
            r#"{"attachments":[{"id":0,"filename":"voice.ogg","duration_secs":2.5,"waveform":"AAAA"},{"id":1,"filename":"ferris.png","title":"Ferris","description":"Ferris the crab","is_spoiler":true}]}"#
        );

        let mut renamed = AttachmentRequest::upload(1, "crab.png");
        renamed.description = Some("A crab".into());
        let keep =
            AttachmentRequest::existing(crate::id::AttachmentId::new(1_300_000_000_000_000_000));
        body.attachments = Some(vec![keep, renamed]);
        assert_eq!(
            json(&body),
            r#"{"attachments":[{"id":"1300000000000000000"},{"id":1,"filename":"crab.png","description":"A crab"},{"id":0,"filename":"voice.ogg","duration_secs":2.5,"waveform":"AAAA"}]}"#
        );

        body.files.clear();
        body.attachments = Some(Vec::new());
        assert_eq!(json(&body), r#"{"attachments":[]}"#);
    }

    #[test]
    fn file_upload_debug_hides_the_data() {
        let file = FileUpload::new("secret.txt", "not shown");
        let debug = format!("{file:?}");
        assert!(
            debug.contains("<9 bytes>") && !debug.contains("not shown"),
            "{debug}"
        );
    }
}
