use compact_str::CompactString;
use serde::{Serialize, Serializer};

use super::{Attachments, FileUpload, setters};
use crate::model::{
    component::Component,
    id::{ChannelId, TagId},
    message::{AllowedMentions, AttachmentRequest, Embed, MessageFlags},
    poll::CreatePoll,
    util::{ImageData, Patch},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateWebhook {
    pub name: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<ImageData>,
}

impl CreateWebhook {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            avatar: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyWebhook {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub avatar: Patch<ImageData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<ChannelId>,
}

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct ExecuteWebhook {
    pub content: Option<CompactString>,

    pub username: Option<CompactString>,

    pub avatar_url: Option<CompactString>,

    pub tts: bool,

    pub embeds: Vec<Embed>,

    pub allowed_mentions: Option<AllowedMentions>,

    pub components: Vec<Component>,

    pub attachments: Option<Vec<AttachmentRequest>>,

    pub flags: Option<MessageFlags>,

    pub thread_name: Option<CompactString>,

    pub applied_tags: Vec<TagId>,

    pub poll: Option<CreatePoll>,

    pub files: Vec<FileUpload>,

    pub thread_id: Option<ChannelId>,

    pub with_components: Option<bool>,
}

impl Serialize for ExecuteWebhook {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "Option::is_none")]
            username: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "Option::is_none")]
            avatar_url: Option<&'a CompactString>,
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
            thread_name: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            applied_tags: &'a [TagId],
            #[serde(skip_serializing_if = "Option::is_none")]
            poll: Option<&'a CreatePoll>,
        }

        let Self {
            content,
            username,
            avatar_url,
            tts,
            embeds,
            allowed_mentions,
            components,
            attachments,
            flags,
            thread_name,
            applied_tags,
            poll,
            files,
            thread_id: _,
            with_components: _,
        } = self;
        Wire {
            content: content.as_ref(),
            username: username.as_ref(),
            avatar_url: avatar_url.as_ref(),
            tts: *tts,
            embeds,
            allowed_mentions: allowed_mentions.as_ref(),
            components,
            attachments: Attachments::new(attachments.as_deref(), files),
            flags: *flags,
            thread_name: thread_name.as_ref(),
            applied_tags,
            poll: poll.as_ref(),
        }
        .serialize(serializer)
    }
}

setters::message_body!(ExecuteWebhook {
    content,
    embeds,
    components,
    allowed_mentions,
    flags,
    suppress_notifications,
    files,
    attachments,
    poll,
    tts,
});

impl ExecuteWebhook {
    #[must_use]
    pub fn username(mut self, username: impl Into<CompactString>) -> Self {
        self.username = Some(username.into());
        self
    }

    #[must_use]
    pub fn avatar_url(mut self, url: impl Into<CompactString>) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    #[must_use]
    pub fn thread_name(mut self, name: impl Into<CompactString>) -> Self {
        self.thread_name = Some(name.into());
        self
    }

    #[must_use]
    pub fn applied_tags(mut self, tags: impl IntoIterator<Item = TagId>) -> Self {
        self.applied_tags = tags.into_iter().collect();
        self
    }

    #[must_use]
    pub const fn thread_id(mut self, thread_id: ChannelId) -> Self {
        self.thread_id = Some(thread_id);
        self
    }

    #[must_use]
    pub const fn with_components(mut self, with_components: bool) -> Self {
        self.with_components = Some(with_components);
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct EditWebhookMessage {
    pub content: Patch<CompactString>,

    pub embeds: Patch<Vec<Embed>>,

    pub flags: Patch<MessageFlags>,

    pub allowed_mentions: Patch<AllowedMentions>,

    pub components: Patch<Vec<Component>>,

    pub attachments: Patch<Vec<AttachmentRequest>>,

    pub poll: Patch<CreatePoll>,

    pub files: Vec<FileUpload>,

    pub thread_id: Option<ChannelId>,

    pub with_components: Option<bool>,
}

impl Serialize for EditWebhookMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(skip_serializing_if = "Patch::is_keep")]
            content: Patch<&'a CompactString>,
            #[serde(skip_serializing_if = "Patch::is_keep")]
            embeds: Patch<&'a Vec<Embed>>,
            #[serde(skip_serializing_if = "Patch::is_keep")]
            flags: Patch<MessageFlags>,
            #[serde(skip_serializing_if = "Patch::is_keep")]
            allowed_mentions: Patch<&'a AllowedMentions>,
            #[serde(skip_serializing_if = "Patch::is_keep")]
            components: Patch<&'a Vec<Component>>,
            #[serde(skip_serializing_if = "Attachments::is_absent")]
            attachments: Attachments<'a>,
            #[serde(skip_serializing_if = "Patch::is_keep")]
            poll: Patch<&'a CreatePoll>,
        }

        let Self {
            content,
            embeds,
            flags,
            allowed_mentions,
            components,
            attachments,
            poll,
            files,
            thread_id: _,
            with_components: _,
        } = self;
        Wire {
            content: content.as_ref(),
            embeds: embeds.as_ref(),
            flags: *flags,
            allowed_mentions: allowed_mentions.as_ref(),
            components: components.as_ref(),
            attachments: Attachments::from_patch(attachments, files),
            poll: poll.as_ref(),
        }
        .serialize(serializer)
    }
}

setters::message_body!(EditWebhookMessage {
    content,
    embeds,
    components,
    allowed_mentions,
    flags,
    files,
    attachments,
    poll,
});

impl EditWebhookMessage {
    #[must_use]
    pub const fn thread_id(mut self, thread_id: ChannelId) -> Self {
        self.thread_id = Some(thread_id);
        self
    }

    #[must_use]
    pub const fn with_components(mut self, with_components: bool) -> Self {
        self.with_components = Some(with_components);
        self
    }
}

impl From<crate::model::message::Embed> for ExecuteWebhook {
    fn from(embed: crate::model::message::Embed) -> Self {
        Self::default().embed(embed)
    }
}

impl From<Vec<crate::model::message::Embed>> for ExecuteWebhook {
    fn from(embeds: Vec<crate::model::message::Embed>) -> Self {
        Self::default().embeds(embeds)
    }
}

impl From<crate::model::message::Embed> for EditWebhookMessage {
    fn from(embed: crate::model::message::Embed) -> Self {
        Self::default().embed(embed)
    }
}

impl From<Vec<crate::model::message::Embed>> for EditWebhookMessage {
    fn from(embeds: Vec<crate::model::message::Embed>) -> Self {
        Self::default().embeds(embeds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn execute_webhook_keeps_query_fields_out_of_the_json() {
        let mut body = ExecuteWebhook::from("hi");
        body.thread_id = Some(ChannelId::new(5));
        body.with_components = Some(true);
        body.thread_name = Some("Release".into());
        body.applied_tags.push(TagId::new(3));
        body.files.push(FileUpload::new("notes.md", "# 1.0"));
        assert_eq!(
            serde_json::to_string(&body).unwrap(),
            concat!(
                r#"{"content":"hi","attachments":[{"id":0,"filename":"notes.md"}],"#,
                r#""thread_name":"Release","applied_tags":["3"]}"#
            )
        );

        let modify = ModifyWebhook {
            avatar: Patch::Clear,
            ..ModifyWebhook::default()
        };
        assert_eq!(
            serde_json::to_string(&modify).unwrap(),
            r#"{"avatar":null}"#
        );
    }

    #[test]
    fn edit_webhook_message() {
        let body = EditWebhookMessage {
            embeds: Patch::Clear,
            poll: Patch::Clear,
            files: vec![FileUpload::new("a.png", &b"a"[..])],
            ..EditWebhookMessage::from("x")
        };
        assert_eq!(
            serde_json::to_string(&body).unwrap(),
            r#"{"content":"x","embeds":null,"attachments":[{"id":0,"filename":"a.png"}],"poll":null}"#
        );
    }
}
