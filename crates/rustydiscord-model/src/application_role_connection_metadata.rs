use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{Deserialize, Serialize};

use crate::{macros::int_enum, util::Locale};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationRoleConnectionMetadata {
    #[serde(rename = "type")]
    pub kind: MetadataType,

    pub key: CompactString,

    pub name: CompactString,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub name_localizations: BTreeMap<Locale, CompactString>,

    pub description: CompactString,

    #[serde(
        default,
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "crate::serde_util::null_as_default"
    )]
    pub description_localizations: BTreeMap<Locale, CompactString>,
}

impl ApplicationRoleConnectionMetadata {
    #[must_use]
    pub fn new(
        kind: MetadataType,
        key: impl Into<CompactString>,
        name: impl Into<CompactString>,
        description: impl Into<CompactString>,
    ) -> Self {
        Self {
            kind,
            key: key.into(),
            name: name.into(),
            name_localizations: BTreeMap::new(),
            description: description.into(),
            description_localizations: BTreeMap::new(),
        }
    }
}

int_enum! {









    #[doc(alias = "ApplicationRoleConnectionMetadataType")]
    pub enum MetadataType: u8 {

        IntegerLessThanOrEqual = 1,

        IntegerGreaterThanOrEqual = 2,

        IntegerEqual = 3,

        IntegerNotEqual = 4,

        DatetimeLessThanOrEqual = 5,

        DatetimeGreaterThanOrEqual = 6,

        BooleanEqual = 7,

        BooleanNotEqual = 8,
    }
}
