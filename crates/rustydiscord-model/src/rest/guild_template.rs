use compact_str::CompactString;
use serde::Serialize;

use crate::util::Patch;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGuildTemplate {
    pub name: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,
}

impl CreateGuildTemplate {
    #[must_use]
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            name: name.into(),
            description: None,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub description: Patch<CompactString>,
}
