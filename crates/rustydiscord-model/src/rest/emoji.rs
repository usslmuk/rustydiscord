use compact_str::CompactString;
use serde::Serialize;

use crate::{
    id::RoleId,
    util::{ImageData, Patch},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateGuildEmoji {
    pub name: CompactString,

    pub image: ImageData,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<RoleId>,
}

impl CreateGuildEmoji {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, image: ImageData) -> Self {
        Self {
            name: name.into(),
            image,
            roles: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyGuildEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,

    #[serde(skip_serializing_if = "Patch::is_keep")]
    pub roles: Patch<Vec<RoleId>>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct CreateApplicationEmoji {
    pub name: CompactString,

    pub image: ImageData,
}

impl CreateApplicationEmoji {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, image: ImageData) -> Self {
        Self {
            name: name.into(),
            image,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize)]
#[non_exhaustive]
pub struct ModifyApplicationEmoji {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<CompactString>,
}
