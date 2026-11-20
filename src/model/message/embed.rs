use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use super::{EmbedFlags, EmbedMediaFlags};
use crate::model::{component::Component, macros::str_enum, util::Timestamp};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Embed {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<CompactString>,

    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<EmbedType>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub footer: Option<Box<EmbedFooter>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub image: Option<Box<EmbedImage>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail: Option<Box<EmbedImage>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video: Option<Box<EmbedVideo>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider: Option<Box<EmbedProvider>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<Box<EmbedAuthor>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<EmbedField>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<Component>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedFlags>,
}

str_enum! {



    pub enum EmbedType {

        Rich = "rich",

        Image = "image",

        Video = "video",

        Gifv = "gifv",

        Article = "article",

        Link = "link",




        AutoModerationMessage = "auto_moderation_message",




        PollResult = "poll_result",

        Components = "components",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmbedFooter {
    pub text: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<CompactString>,
}

impl EmbedFooter {
    #[must_use]
    pub fn new(text: impl Into<CompactString>) -> Self {
        Self {
            text: text.into(),
            icon_url: None,
            proxy_icon_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
#[doc(alias = "EmbedThumbnail")]
pub struct EmbedImage {
    pub url: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedMediaFlags>,
}

impl EmbedImage {
    #[must_use]
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self {
            url: url.into(),
            proxy_url: None,
            height: None,
            width: None,
            content_type: None,
            placeholder: None,
            placeholder_version: None,
            description: None,
            flags: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmbedVideo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<EmbedMediaFlags>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmbedProvider {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmbedAuthor {
    pub name: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_icon_url: Option<CompactString>,
}

impl EmbedAuthor {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            url: None,
            icon_url: None,
            proxy_icon_url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct EmbedField {
    pub name: CompactString,

    pub value: CompactString,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub inline: bool,
}

impl EmbedField {
    #[must_use]
    pub fn new(
        name: impl Into<CompactString>,
        value: impl Into<CompactString>,
        inline: bool,
    ) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
            inline,
        }
    }
}
