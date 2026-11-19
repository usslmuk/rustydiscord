use compact_str::CompactString;
use serde::{Serialize, Serializer};

use super::{Attachments, FileUpload, setters};
use crate::{
    component::Component,
    message::{AllowedMentions, AttachmentRequest, Embed, MessageFlags},
    poll::CreatePoll,
};

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct CreateFollowupMessage {
    pub content: Option<CompactString>,

    pub tts: bool,

    pub embeds: Vec<Embed>,

    pub allowed_mentions: Option<AllowedMentions>,

    pub components: Vec<Component>,

    pub attachments: Option<Vec<AttachmentRequest>>,

    pub flags: Option<MessageFlags>,

    pub poll: Option<CreatePoll>,

    pub files: Vec<FileUpload>,
}

impl Serialize for CreateFollowupMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            tts: bool,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            embeds: &'a [Embed],
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_mentions: Option<&'a AllowedMentions>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            components: &'a [Component],
            #[serde(skip_serializing_if = "Attachments::is_absent")]
            attachments: Attachments<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            flags: Option<MessageFlags>,
            #[serde(skip_serializing_if = "Option::is_none")]
            poll: Option<&'a CreatePoll>,
        }

        let Self {
            content,
            tts,
            embeds,
            allowed_mentions,
            components,
            attachments,
            flags,
            poll,
            files,
        } = self;
        Wire {
            content: content.as_ref(),
            tts: *tts,
            embeds,
            allowed_mentions: allowed_mentions.as_ref(),
            components,
            attachments: Attachments::new(attachments.as_deref(), files),
            flags: *flags,
            poll: poll.as_ref(),
        }
        .serialize(serializer)
    }
}

setters::message_body!(CreateFollowupMessage {
    content,
    embeds,
    components,
    allowed_mentions,
    flags,
    ephemeral,
    suppress_notifications,
    files,
    attachments,
    poll,
    tts,
});

impl From<crate::message::Embed> for CreateFollowupMessage {
    fn from(embed: crate::message::Embed) -> Self {
        Self::default().embed(embed)
    }
}

impl From<Vec<crate::message::Embed>> for CreateFollowupMessage {
    fn from(embeds: Vec<crate::message::Embed>) -> Self {
        Self::default().embeds(embeds)
    }
}
