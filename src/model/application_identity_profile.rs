use std::collections::BTreeMap;

use compact_str::CompactString;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as DeError};

use crate::model::{
    id::UserId,
    macros::int_enum,
    util::{JsonValue, JsonValueError},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationIdentity {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<UserId>,

    pub provider_type: CompactString,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_id: Option<CompactString>,

    pub provider_issued_user_id: CompactString,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ApplicationIdentityProfile {
    pub username: Option<CompactString>,

    pub metadata: Option<BTreeMap<CompactString, JsonValue>>,

    pub data: Option<ProfileData>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProfileData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary: Option<Box<PrimaryProfileData>>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dynamic: Vec<DynamicField>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PrimaryProfileData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub season: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_name: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rank_image: Option<ProfileMedia>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_rank: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub highest_rank_image: Option<ProfileMedia>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_played_character: Option<CompactString>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_played_character_image: Option<ProfileMedia>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub playtime_hours: Option<f64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_wins: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_wins: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_games: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_games: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_kills: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_kills: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_assists: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_assists: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_deaths: Option<u64>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_period_deaths: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub struct ProfileMedia {
    pub url: CompactString,
}

impl ProfileMedia {
    #[must_use]
    pub fn new(url: impl Into<CompactString>) -> Self {
        Self { url: url.into() }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DynamicField {
    pub name: CompactString,

    pub value: DynamicFieldValue,
}

impl DynamicField {
    #[must_use]
    pub fn new(name: impl Into<CompactString>, value: DynamicFieldValue) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum DynamicFieldValue {
    String(CompactString),

    Number(f64),

    Media(ProfileMedia),

    Unknown(DynamicFieldType),
}

impl DynamicFieldValue {
    #[must_use]
    pub const fn kind(&self) -> DynamicFieldType {
        match self {
            Self::String(_) => DynamicFieldType::String,
            Self::Number(_) => DynamicFieldType::Number,
            Self::Media(_) => DynamicFieldType::Media,
            Self::Unknown(kind) => *kind,
        }
    }
}

int_enum! {



    pub enum DynamicFieldType: u8 {

        String = 1,

        Number = 2,

        Media = 3,
    }
}

#[derive(Deserialize)]
struct RawDynamicField {
    #[serde(rename = "type")]
    kind: DynamicFieldType,
    name: CompactString,
    #[serde(default)]
    value: Option<JsonValue>,
}

impl<'de> Deserialize<'de> for DynamicField {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = RawDynamicField::deserialize(deserializer)?;
        let convert = |error: JsonValueError| D::Error::custom(format_args!("value: {error}"));
        let value = match raw.kind {
            DynamicFieldType::Unknown(_) => DynamicFieldValue::Unknown(raw.kind),
            kind => {
                let value = raw.value.ok_or_else(|| D::Error::missing_field("value"))?;
                match kind {
                    DynamicFieldType::String => {
                        DynamicFieldValue::String(value.to_type().map_err(convert)?)
                    }
                    DynamicFieldType::Number => {
                        DynamicFieldValue::Number(value.to_type().map_err(convert)?)
                    }
                    _ => DynamicFieldValue::Media(value.to_type().map_err(convert)?),
                }
            }
        };
        Ok(Self {
            name: raw.name,
            value,
        })
    }
}

impl Serialize for DynamicField {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum Value<'a> {
            String(&'a str),
            Number(f64),
            Media(&'a ProfileMedia),
        }

        #[derive(Serialize)]
        struct Flat<'a> {
            #[serde(rename = "type")]
            kind: DynamicFieldType,
            name: &'a str,
            #[serde(skip_serializing_if = "Option::is_none")]
            value: Option<Value<'a>>,
        }

        Flat {
            kind: self.value.kind(),
            name: &self.name,
            value: match &self.value {
                DynamicFieldValue::String(value) => Some(Value::String(value)),
                DynamicFieldValue::Number(value) => Some(Value::Number(*value)),
                DynamicFieldValue::Media(value) => Some(Value::Media(value)),
                DynamicFieldValue::Unknown(_) => None,
            },
        }
        .serialize(serializer)
    }
}
