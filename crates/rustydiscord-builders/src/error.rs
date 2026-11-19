use std::{borrow::Cow, fmt};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ValidationError {
    TooLong {
        field: Cow<'static, str>,

        max: usize,

        actual: usize,
    },

    TooMany {
        field: Cow<'static, str>,

        max: usize,

        actual: usize,
    },

    TooFew {
        field: Cow<'static, str>,

        min: usize,

        actual: usize,
    },

    Missing {
        field: Cow<'static, str>,
    },

    OutOfRange {
        field: Cow<'static, str>,

        min: i64,

        max: i64,

        actual: i64,
    },

    InvalidUrl {
        field: Cow<'static, str>,

        allowed: &'static [&'static str],
    },

    InvalidName {
        field: Cow<'static, str>,

        character: char,
    },

    Invalid {
        field: Cow<'static, str>,

        reason: &'static str,
    },

    Empty {
        field: Cow<'static, str>,

        needs: &'static str,
    },
}

impl ValidationError {
    #[must_use]
    pub fn field(&self) -> &str {
        match self {
            Self::TooLong { field, .. }
            | Self::TooMany { field, .. }
            | Self::TooFew { field, .. }
            | Self::Missing { field }
            | Self::OutOfRange { field, .. }
            | Self::InvalidUrl { field, .. }
            | Self::InvalidName { field, .. }
            | Self::Invalid { field, .. }
            | Self::Empty { field, .. } => field,
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooLong { field, max, actual } => write!(
                f,
                "{field} is too long: {actual} characters, at most {max} allowed"
            ),
            Self::TooMany { field, max, actual } => write!(
                f,
                "{field} has too many items: {actual}, at most {max} allowed"
            ),
            Self::TooFew { field, min, actual } => write!(
                f,
                "{field} has too few items: {actual}, at least {min} required"
            ),
            Self::Missing { field } => write!(f, "{field} is required"),
            Self::OutOfRange {
                field,
                min,
                max,
                actual,
            } => write!(
                f,
                "{field} is out of range: {actual} is not between {min} and {max}"
            ),
            Self::InvalidUrl { field, allowed } => {
                write!(f, "{field} must start with ")?;
                for (index, scheme) in allowed.iter().enumerate() {
                    let separator = match index {
                        0 => "",
                        _ if index + 1 == allowed.len() => " or ",
                        _ => ", ",
                    };
                    write!(f, "{separator}{scheme}")?;
                }
                Ok(())
            }
            Self::InvalidName { field, character } => {
                let mut lower = character.to_lowercase();
                if lower.len() == 1 && lower.next() == Some(*character) {
                    write!(f, "{field} cannot contain {character:?}")
                } else {
                    write!(
                        f,
                        "{field} must be lower case: use '{}' instead of '{character}'",
                        character.to_lowercase()
                    )
                }
            }
            Self::Invalid { field, reason } => write!(f, "{field} is invalid: {reason}"),
            Self::Empty { field, needs } => {
                write!(f, "{field} has no content: it needs {needs}")
            }
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod tests {
    use super::ValidationError;

    #[test]
    fn every_variant_has_a_readable_message() {
        let cases = [
            (
                ValidationError::TooMany {
                    field: "fields".into(),
                    max: 25,
                    actual: 26,
                },
                "fields has too many items: 26, at most 25 allowed",
            ),
            (
                ValidationError::TooFew {
                    field: "components".into(),
                    min: 1,
                    actual: 0,
                },
                "components has too few items: 0, at least 1 required",
            ),
            (
                ValidationError::Missing {
                    field: "name".into(),
                },
                "name is required",
            ),
            (
                ValidationError::OutOfRange {
                    field: "color".into(),
                    min: 0,
                    max: 0xFF_FFFF,
                    actual: 0x100_0000,
                },
                "color is out of range: 16777216 is not between 0 and 16777215",
            ),
            (
                ValidationError::InvalidUrl {
                    field: "url".into(),
                    allowed: &["http://", "https://"],
                },
                "url must start with http:// or https://",
            ),
            (
                ValidationError::InvalidUrl {
                    field: "image.url".into(),
                    allowed: &["http://", "https://", "attachment://"],
                },
                "image.url must start with http://, https:// or attachment://",
            ),
            (
                ValidationError::InvalidName {
                    field: "name".into(),
                    character: 'Ż',
                },
                "name must be lower case: use 'ż' instead of 'Ż'",
            ),
            (
                ValidationError::InvalidName {
                    field: "options[0].name".into(),
                    character: ' ',
                },
                "options[0].name cannot contain ' '",
            ),
            (
                ValidationError::Invalid {
                    field: "options[1].required".into(),
                    reason: "required options must come before optional ones",
                },
                "options[1].required is invalid: required options must come before optional ones",
            ),
            (
                ValidationError::Empty {
                    field: "embed".into(),
                    needs: "a title",
                },
                "embed has no content: it needs a title",
            ),
        ];
        for (error, message) in cases {
            assert_eq!(error.to_string(), message);
            assert!(message.starts_with(error.field()));
        }
    }
}
