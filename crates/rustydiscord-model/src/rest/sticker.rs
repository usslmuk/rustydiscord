use compact_str::CompactString;
use serde::Serialize;

use super::FileUpload;
use crate::util::Patch;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CreateGuildSticker {
    pub name: CompactString,

    pub description: CompactString,

    pub tags: CompactString,

    pub file: FileUpload,
}

impl CreateGuildSticker {
    #[must_use]
    pub fn new(
        name: impl Into<CompactString>,
        tags: impl Into<CompactString>,
        file: FileUpload,
    ) -> Self {
        Self {
            name: name.into(),
            description: CompactString::const_new(""),
            tags: tags.into(),
            file,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildSticker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description: Patch<CompactString>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<CompactString>,
}
