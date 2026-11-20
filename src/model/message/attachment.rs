use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use super::AttachmentFlags;
use crate::model::{
    application::PartialApplication, id::AttachmentId, user::User, util::Timestamp,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Attachment {
    pub id: AttachmentId,

    pub filename: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_type: Option<CompactString>,

    pub size: u64,

    pub url: CompactString,

    pub proxy_url: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_version: Option<u32>,

    #[serde(default, skip_serializing_if = "core::ops::Not::not")]
    pub ephemeral: bool,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waveform: Option<CompactString>,

    #[serde(default)]
    pub flags: AttachmentFlags,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub clip_participants: Vec<User>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub clip_created_at: Option<Timestamp>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<PartialApplication>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AttachmentRequest {
    pub id: AttachmentRequestId,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filename: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_secs: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waveform: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_spoiler: Option<bool>,
}

impl AttachmentRequest {
    #[must_use]
    pub fn upload(index: u32, filename: impl Into<CompactString>) -> Self {
        Self::with_id(AttachmentRequestId::Upload(index), Some(filename.into()))
    }

    #[must_use]
    pub fn existing(id: AttachmentId) -> Self {
        Self::with_id(AttachmentRequestId::Existing(id), None)
    }

    const fn with_id(id: AttachmentRequestId, filename: Option<CompactString>) -> Self {
        Self {
            id,
            filename,
            title: None,
            description: None,
            duration_secs: None,
            waveform: None,
            is_spoiler: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AttachmentRequestId {
    Upload(u32),

    Existing(AttachmentId),
}

impl Serialize for AttachmentRequestId {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Upload(index) => serializer.serialize_u32(*index),
            Self::Existing(id) => id.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for AttachmentRequestId {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct AttachmentRequestIdVisitor;

        impl serde::de::Visitor<'_> for AttachmentRequestIdVisitor {
            type Value = AttachmentRequestId;

            fn expecting(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("an upload index or a snowflake ID")
            }

            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                if let Ok(index) = u32::try_from(value) {
                    return Ok(AttachmentRequestId::Upload(index));
                }
                AttachmentId::new_checked(value)
                    .map(AttachmentRequestId::Existing)
                    .ok_or_else(|| E::invalid_value(serde::de::Unexpected::Unsigned(value), &self))
            }

            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                u64::try_from(value)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Signed(value), &self))
                    .and_then(|value| self.visit_u64(value))
            }

            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                value
                    .parse()
                    .map(AttachmentRequestId::Existing)
                    .map_err(|_| E::invalid_value(serde::de::Unexpected::Str(value), &self))
            }
        }

        deserializer.deserialize_any(AttachmentRequestIdVisitor)
    }
}
