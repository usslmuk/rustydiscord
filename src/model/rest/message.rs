use compact_str::CompactString;
use serde::{Deserialize, Serialize, Serializer};

use super::{Attachments, FileUpload, setters};
use crate::model::{
    channel::{Channel, ThreadMember},
    component::Component,
    id::{ChannelId, MessageId, RoleId, StickerId, UserId},
    macros::str_enum,
    message::{
        AllowedMentions, AttachmentRequest, Embed, Message, MessageFlags, MessagePin,
        MessageReference, Nonce, ReactionType, SharedClientTheme,
    },
    poll::CreatePoll,
    util::{Patch, Timestamp},
};

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct CreateMessage {
    pub content: Option<CompactString>,

    pub nonce: Option<Nonce>,

    pub tts: bool,

    pub embeds: Vec<Embed>,

    pub allowed_mentions: Option<AllowedMentions>,

    pub message_reference: Option<MessageReference>,

    pub components: Vec<Component>,

    pub sticker_ids: Vec<StickerId>,

    pub attachments: Option<Vec<AttachmentRequest>>,

    pub flags: Option<MessageFlags>,

    pub enforce_nonce: bool,

    pub poll: Option<CreatePoll>,

    pub shared_client_theme: Option<SharedClientTheme>,

    pub files: Vec<FileUpload>,
}

impl Serialize for CreateMessage {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Wire<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            content: Option<&'a CompactString>,
            #[serde(skip_serializing_if = "Option::is_none")]
            nonce: Option<&'a Nonce>,
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            tts: bool,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            embeds: &'a [Embed],
            #[serde(skip_serializing_if = "Option::is_none")]
            allowed_mentions: Option<&'a AllowedMentions>,
            #[serde(skip_serializing_if = "Option::is_none")]
            message_reference: Option<&'a MessageReference>,
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            components: &'a [Component],
            #[serde(skip_serializing_if = "<[_]>::is_empty")]
            sticker_ids: &'a [StickerId],
            #[serde(skip_serializing_if = "Attachments::is_absent")]
            attachments: Attachments<'a>,
            #[serde(skip_serializing_if = "Option::is_none")]
            flags: Option<MessageFlags>,
            #[serde(skip_serializing_if = "core::ops::Not::not")]
            enforce_nonce: bool,
            #[serde(skip_serializing_if = "Option::is_none")]
            poll: Option<&'a CreatePoll>,
            #[serde(skip_serializing_if = "Option::is_none")]
            shared_client_theme: Option<&'a SharedClientTheme>,
        }

        let Self {
            content,
            nonce,
            tts,
            embeds,
            allowed_mentions,
            message_reference,
            components,
            sticker_ids,
            attachments,
            flags,
            enforce_nonce,
            poll,
            shared_client_theme,
            files,
        } = self;
        Wire {
            content: content.as_ref(),
            nonce: nonce.as_ref(),
            tts: *tts,
            embeds,
            allowed_mentions: allowed_mentions.as_ref(),
            message_reference: message_reference.as_ref(),
            components,
            sticker_ids,
            attachments: Attachments::new(attachments.as_deref(), files),
            flags: *flags,
            enforce_nonce: *enforce_nonce,
            poll: poll.as_ref(),
            shared_client_theme: shared_client_theme.as_ref(),
        }
        .serialize(serializer)
    }
}

setters::message_body!(CreateMessage {
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

impl CreateMessage {
    #[must_use]
    pub const fn reply_to(mut self, message_id: MessageId) -> Self {
        self.message_reference = Some(MessageReference::reply(message_id));
        self
    }

    #[must_use]
    pub fn message_reference(mut self, reference: MessageReference) -> Self {
        self.message_reference = Some(reference);
        self
    }

    #[must_use]
    pub fn sticker(mut self, sticker_id: StickerId) -> Self {
        self.sticker_ids.push(sticker_id);
        self
    }

    #[must_use]
    pub fn stickers(mut self, sticker_ids: impl IntoIterator<Item = StickerId>) -> Self {
        self.sticker_ids = sticker_ids.into_iter().collect();
        self
    }

    #[must_use]
    pub fn nonce(mut self, nonce: Nonce, enforce: bool) -> Self {
        self.nonce = Some(nonce);
        self.enforce_nonce = enforce;
        self
    }

    #[must_use]
    pub fn shared_client_theme(mut self, theme: SharedClientTheme) -> Self {
        self.shared_client_theme = Some(theme);
        self
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct EditMessage {
    pub content: Patch<CompactString>,

    pub embeds: Patch<Vec<Embed>>,

    pub flags: Patch<MessageFlags>,

    pub allowed_mentions: Patch<AllowedMentions>,

    pub components: Patch<Vec<Component>>,

    pub attachments: Patch<Vec<AttachmentRequest>>,

    pub files: Vec<FileUpload>,
}

impl Serialize for EditMessage {
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
        }

        let Self {
            content,
            embeds,
            flags,
            allowed_mentions,
            components,
            attachments,
            files,
        } = self;
        Wire {
            content: content.as_ref(),
            embeds: embeds.as_ref(),
            flags: *flags,
            allowed_mentions: allowed_mentions.as_ref(),
            components: components.as_ref(),
            attachments: Attachments::from_patch(attachments, files),
        }
        .serialize(serializer)
    }
}

setters::message_body!(EditMessage {
    content,
    embeds,
    components,
    allowed_mentions,
    flags,
    files,
    attachments,
});

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetChannelMessages {
    pub around: Option<MessageId>,

    pub before: Option<MessageId>,

    pub after: Option<MessageId>,

    pub limit: Option<u8>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetReactions {
    pub kind: Option<ReactionType>,

    pub after: Option<UserId>,

    pub limit: Option<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct BulkDeleteMessages {
    pub messages: Vec<MessageId>,
}

impl BulkDeleteMessages {
    #[must_use]
    pub const fn new(messages: Vec<MessageId>) -> Self {
        Self { messages }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct GetChannelPins {
    pub before: Option<Timestamp>,

    pub limit: Option<u8>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ChannelPins {
    pub items: Vec<MessagePin>,

    pub has_more: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SearchGuildMessages {
    pub limit: Option<u8>,

    pub offset: Option<u16>,

    pub max_id: Option<MessageId>,

    pub min_id: Option<MessageId>,

    pub slop: Option<u8>,

    pub content: Option<CompactString>,

    pub channel_id: Vec<ChannelId>,

    pub author_type: Vec<SearchFilter<SearchAuthorType>>,

    pub author_id: Vec<UserId>,

    pub mentions: Vec<UserId>,

    pub mentions_role_id: Vec<RoleId>,

    pub mention_everyone: Option<bool>,

    pub replied_to_user_id: Vec<UserId>,

    pub replied_to_message_id: Vec<MessageId>,

    pub pinned: Option<bool>,

    pub has: Vec<SearchFilter<SearchHasType>>,

    pub embed_type: Vec<SearchEmbedType>,

    pub embed_provider: Vec<CompactString>,

    pub link_hostname: Vec<CompactString>,

    pub attachment_filename: Vec<CompactString>,

    pub attachment_extension: Vec<CompactString>,

    pub sort_by: Option<SearchSortMode>,

    pub sort_order: Option<SearchSortOrder>,

    pub include_nsfw: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SearchFilter<T> {
    Include(T),

    Exclude(T),
}

impl<T> From<T> for SearchFilter<T> {
    fn from(value: T) -> Self {
        Self::Include(value)
    }
}

str_enum! {




    pub enum SearchAuthorType {

        User = "user",

        Bot = "bot",

        Webhook = "webhook",
    }
}

str_enum! {




    pub enum SearchHasType {

        Image = "image",

        Sound = "sound",

        Video = "video",

        File = "file",

        Sticker = "sticker",

        Embed = "embed",

        Link = "link",

        Poll = "poll",

        Snapshot = "snapshot",
    }
}

str_enum! {




    pub enum SearchEmbedType {

        Image = "image",

        Video = "video",

        Gif = "gif",

        Sound = "sound",

        Article = "article",
    }
}

str_enum! {




    pub enum SearchSortMode {

        Timestamp = "timestamp",

        Relevance = "relevance",
    }
}

str_enum! {



    pub enum SearchSortOrder {

        Ascending = "asc",

        Descending = "desc",
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MessageSearchResults {
    pub doing_deep_historical_index: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub documents_indexed: Option<u64>,

    pub total_results: u64,

    pub messages: Vec<Vec<Message>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub threads: Vec<Channel>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub members: Vec<ThreadMember>,
}

impl From<crate::model::message::Embed> for CreateMessage {
    fn from(embed: crate::model::message::Embed) -> Self {
        Self::default().embed(embed)
    }
}

impl From<Vec<crate::model::message::Embed>> for CreateMessage {
    fn from(embeds: Vec<crate::model::message::Embed>) -> Self {
        Self::default().embeds(embeds)
    }
}

impl From<crate::model::message::Embed> for EditMessage {
    fn from(embed: crate::model::message::Embed) -> Self {
        Self::default().embed(embed)
    }
}

impl From<Vec<crate::model::message::Embed>> for EditMessage {
    fn from(embeds: Vec<crate::model::message::Embed>) -> Self {
        Self::default().embeds(embeds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{id::AttachmentId, message::Embed};

    fn json<T: Serialize>(body: &T) -> String {
        serde_json::to_string(body).unwrap()
    }

    #[test]
    fn create_message_fields() {
        let mut body = CreateMessage::from("Hello, World!");
        body.nonce = Some(Nonce::String("n-1".into()));
        body.enforce_nonce = true;
        body.tts = true;
        body.embeds.push(Embed {
            title: Some("Hello, Embed!".into()),
            ..Embed::default()
        });
        body.sticker_ids
            .push(StickerId::new(1_300_000_000_000_000_000));
        body.flags = Some(MessageFlags::SUPPRESS_NOTIFICATIONS);
        assert_eq!(
            json(&body),
            concat!(
                r#"{"content":"Hello, World!","nonce":"n-1","tts":true,"#,
                r#""embeds":[{"title":"Hello, Embed!"}],"sticker_ids":["1300000000000000000"],"#,
                r#""flags":4096,"enforce_nonce":true}"#
            )
        );
        assert_eq!(json(&CreateMessage::default()), "{}");
    }

    #[test]
    fn edit_message_patches() {
        assert_eq!(json(&EditMessage::default()), "{}");

        let body = EditMessage {
            flags: Patch::Set(MessageFlags::SUPPRESS_EMBEDS),
            components: Patch::Clear,
            allowed_mentions: Patch::Clear,
            ..EditMessage::default()
        };
        assert_eq!(
            json(&body),
            r#"{"flags":4,"allowed_mentions":null,"components":null}"#
        );

        let mut body = EditMessage {
            attachments: Patch::Set(vec![AttachmentRequest::existing(AttachmentId::new(
                1_300_000_000_000_000_001,
            ))]),
            files: vec![FileUpload::new("new.png", &b"png"[..])],
            ..EditMessage::default()
        };
        assert_eq!(
            json(&body),
            r#"{"attachments":[{"id":"1300000000000000001"},{"id":0,"filename":"new.png"}]}"#
        );

        body.attachments = Patch::Clear;
        assert_eq!(
            json(&body),
            r#"{"attachments":[{"id":0,"filename":"new.png"}]}"#
        );
        body.files.clear();
        assert_eq!(json(&body), r#"{"attachments":[]}"#);
    }

    #[test]
    fn bulk_delete_and_pins() {
        let body = BulkDeleteMessages::new(vec![MessageId::new(1), MessageId::new(2)]);
        assert_eq!(json(&body), r#"{"messages":["1","2"]}"#);

        let pins: ChannelPins = serde_json::from_str(r#"{"items":[],"has_more":false}"#).unwrap();
        assert!(pins.items.is_empty() && !pins.has_more);
    }
}
