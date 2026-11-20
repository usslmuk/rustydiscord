use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Patch<T> {
    #[default]
    Keep,

    Clear,

    Set(T),
}

impl<T> Patch<T> {
    #[must_use]
    pub const fn is_keep(&self) -> bool {
        matches!(self, Self::Keep)
    }

    #[must_use]
    pub const fn is_clear(&self) -> bool {
        matches!(self, Self::Clear)
    }

    #[must_use]
    pub const fn is_set(&self) -> bool {
        matches!(self, Self::Set(_))
    }

    #[must_use]
    pub fn into_set(self) -> Option<T> {
        match self {
            Self::Set(value) => Some(value),
            Self::Keep | Self::Clear => None,
        }
    }

    #[must_use]
    pub const fn as_ref(&self) -> Patch<&T> {
        match self {
            Self::Keep => Patch::Keep,
            Self::Clear => Patch::Clear,
            Self::Set(value) => Patch::Set(value),
        }
    }

    #[must_use]
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Patch<U> {
        match self {
            Self::Keep => Patch::Keep,
            Self::Clear => Patch::Clear,
            Self::Set(value) => Patch::Set(f(value)),
        }
    }
}

impl<T> From<T> for Patch<T> {
    fn from(value: T) -> Self {
        Self::Set(value)
    }
}

impl<T> From<Option<T>> for Patch<T> {
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::Clear, Self::Set)
    }
}

impl<T: Serialize> Serialize for Patch<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Set(value) => value.serialize(serializer),
            Self::Clear => serializer.serialize_none(),
            Self::Keep => Err(serde::ser::Error::custom(
                "`Patch::Keep` has no JSON value; skip the field with \
                 `#[serde(skip_serializing_if = \"Patch::is_keep\")]`",
            )),
        }
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Patch<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize, Debug, PartialEq)]
    struct Body {
        #[serde(default, skip_serializing_if = "Patch::is_keep")]
        nick: Patch<String>,
        #[serde(default, skip_serializing_if = "Patch::is_keep")]
        channel_id: Patch<u64>,
    }

    #[test]
    fn fields_are_left_out_cleared_or_set() {
        let body = Body {
            nick: Patch::Set("Ferris".into()),
            channel_id: Patch::Clear,
        };
        let json = serde_json::to_string(&body).unwrap();
        assert_eq!(json, r#"{"nick":"Ferris","channel_id":null}"#);
        assert_eq!(serde_json::from_str::<Body>(&json).unwrap(), body);

        let unchanged = Body {
            nick: Patch::Keep,
            channel_id: Patch::Keep,
        };
        assert_eq!(serde_json::to_string(&unchanged).unwrap(), "{}");
        assert_eq!(serde_json::from_str::<Body>("{}").unwrap(), unchanged);
    }

    #[test]
    fn keep_alone_cannot_be_serialized() {
        let error = serde_json::to_string(&Patch::<u8>::Keep).unwrap_err();
        assert!(error.to_string().contains("skip_serializing_if"), "{error}");
    }

    #[test]
    fn helpers() {
        let set = Patch::Set(2);
        assert!(set.is_set() && !set.is_keep() && !set.is_clear());
        assert_eq!(set.map(|value| value * 2), Patch::Set(4));
        assert_eq!(set.as_ref(), Patch::Set(&2));
        assert_eq!(set.into_set(), Some(2));
        assert_eq!(Patch::<u8>::Clear.into_set(), None);
        assert_eq!(Patch::<u8>::Clear.map(u16::from), Patch::Clear);
        assert_eq!(Patch::<u8>::Keep.map(u16::from), Patch::Keep);
    }
}
