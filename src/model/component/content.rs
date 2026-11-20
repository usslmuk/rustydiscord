use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::model::{
    id::AttachmentId,
    macros::{int_enum, int_flags},
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct TextDisplay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub content: CompactString,
}

impl TextDisplay {
    #[must_use]
    pub fn new(content: impl Into<CompactString>) -> Self {
        Self {
            id: None,
            content: content.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Thumbnail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub media: Box<UnfurledMediaItem>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub spoiler: bool,
}

impl Thumbnail {
    #[must_use]
    pub fn new(media: UnfurledMediaItem) -> Self {
        Self {
            id: None,
            media: Box::new(media),
            description: None,
            spoiler: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct MediaGallery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub items: Vec<MediaGalleryItem>,
}

impl MediaGallery {
    #[must_use]
    pub const fn new(items: Vec<MediaGalleryItem>) -> Self {
        Self { id: None, items }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct MediaGalleryItem {
    pub media: UnfurledMediaItem,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub spoiler: bool,
}

impl MediaGalleryItem {
    #[must_use]
    pub const fn new(media: UnfurledMediaItem) -> Self {
        Self {
            media,
            description: None,
            spoiler: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct File {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub file: Box<UnfurledMediaItem>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub spoiler: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

impl File {
    #[must_use]
    pub fn new(file: UnfurledMediaItem) -> Self {
        Self {
            id: None,
            file: Box::new(file),
            spoiler: false,
            name: None,
            size: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UnfurledMediaItem {
    pub url: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loading_state: Option<LoadingState>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<UnfurledMediaItemFlags>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_scan_metadata: Option<ContentScanMetadata>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<AttachmentId>,
}

impl UnfurledMediaItem {
    #[must_use]
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self {
            url: url.into(),
            proxy_url: None,
            height: None,
            width: None,
            placeholder: None,
            placeholder_version: None,
            content_type: None,
            loading_state: None,
            flags: None,
            content_scan_metadata: None,
            attachment_id: None,
        }
    }
}

int_enum! {






    #[doc(alias = "UnfurledMediaItemLoadingState")]
    pub enum LoadingState: u8 {

        Unspecified = 0,

        Loading = 1,

        LoadedSuccess = 2,

        LoadedNotFound = 3,
    }
}

int_flags! {



    pub struct UnfurledMediaItemFlags: u64 {

        const IS_ANIMATED = 1 << 0;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ContentScanMetadata {
    pub flags: ContentScanMetadataFlags,
}

int_flags! {



    pub struct ContentScanMetadataFlags: u64 {

        const IS_EXPLICIT_CONTENT = 1 << 0;

        const IS_GORE_CONTENT = 1 << 1;

        const IS_SELF_HARM_CONTENT = 1 << 2;
    }
}
