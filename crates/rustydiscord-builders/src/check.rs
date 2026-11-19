use std::{borrow::Cow, fmt};

use crate::ValidationError;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Path<'a> {
    Root,

    Item {
        parent: &'a Path<'a>,
        list: &'static str,
        index: usize,
    },

    Child {
        parent: &'a Path<'a>,
        name: &'static str,
    },
}

impl<'a> Path<'a> {
    pub(crate) const fn item(&'a self, list: &'static str, index: usize) -> Self {
        Self::Item {
            parent: self,
            list,
            index,
        }
    }

    pub(crate) const fn child(&'a self, name: &'static str) -> Self {
        Self::Child { parent: self, name }
    }

    pub(crate) fn field(&self, name: &'static str) -> Cow<'static, str> {
        match self {
            Self::Root => name.into(),
            _ => format!("{self}.{name}").into(),
        }
    }

    pub(crate) fn entry(&self, map: &'static str, key: &str) -> Cow<'static, str> {
        match self {
            Self::Root => format!("{map}.{key}").into(),
            _ => format!("{self}.{map}.{key}").into(),
        }
    }

    pub(crate) fn this(&self) -> Cow<'static, str> {
        self.to_string().into()
    }
}

impl fmt::Display for Path<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Root => Ok(()),
            Self::Item {
                parent: Self::Root,
                list,
                index,
            } => write!(f, "{list}[{index}]"),
            Self::Item {
                parent,
                list,
                index,
            } => write!(f, "{parent}.{list}[{index}]"),
            Self::Child {
                parent: Self::Root,
                name,
            } => f.write_str(name),
            Self::Child { parent, name } => write!(f, "{parent}.{name}"),
        }
    }
}

pub(crate) const WEB: &[&str] = &["http://", "https://"];

pub(crate) const WEB_OR_DISCORD: &[&str] = &["http://", "https://", "discord://"];

pub(crate) const WEB_OR_ATTACHMENT: &[&str] = &["http://", "https://", "attachment://"];

pub(crate) fn length(text: &str) -> usize {
    text.trim().chars().count()
}

pub(crate) fn is_blank(text: &str) -> bool {
    text.trim().is_empty()
}

pub(crate) fn text(
    field: impl FnOnce() -> Cow<'static, str>,
    text: &str,
    max: usize,
) -> Result<usize, ValidationError> {
    let actual = length(text);
    if actual > max {
        return Err(ValidationError::TooLong {
            field: field(),
            max,
            actual,
        });
    }
    Ok(actual)
}

pub(crate) fn required_text(
    field: impl FnOnce() -> Cow<'static, str>,
    text: &str,
    max: usize,
) -> Result<usize, ValidationError> {
    let actual = length(text);
    if actual == 0 {
        return Err(ValidationError::Missing { field: field() });
    }
    if actual > max {
        return Err(ValidationError::TooLong {
            field: field(),
            max,
            actual,
        });
    }
    Ok(actual)
}

pub(crate) fn count(
    field: impl FnOnce() -> Cow<'static, str>,
    actual: usize,
    max: usize,
) -> Result<(), ValidationError> {
    if actual > max {
        return Err(ValidationError::TooMany {
            field: field(),
            max,
            actual,
        });
    }
    Ok(())
}

pub(crate) fn at_least(
    field: impl FnOnce() -> Cow<'static, str>,
    actual: usize,
    min: usize,
) -> Result<(), ValidationError> {
    if actual < min {
        return Err(ValidationError::TooFew {
            field: field(),
            min,
            actual,
        });
    }
    Ok(())
}

pub(crate) fn url(
    field: impl FnOnce() -> Cow<'static, str>,
    url: &str,
    allowed: &'static [&'static str],
) -> Result<(), ValidationError> {
    let has_scheme = |scheme: &&str| {
        url.as_bytes()
            .get(..scheme.len())
            .is_some_and(|start| start.eq_ignore_ascii_case(scheme.as_bytes()))
    };
    if allowed.iter().any(has_scheme) {
        Ok(())
    } else {
        Err(ValidationError::InvalidUrl {
            field: field(),
            allowed,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_counts_code_points_after_trimming() {
        assert_eq!(length(""), 0);
        assert_eq!(length("  \n\t "), 0);
        assert_eq!(length("abc"), 3);
        assert_eq!(length("  abc  "), 3);
        assert_eq!(length("a b"), 3);

        assert_eq!(length("🔥🔥"), 2);

        assert_eq!(length("👨‍👩‍👧‍👦"), 7);

        assert_eq!(length("\u{a0}x\u{a0}"), 1);
        assert_eq!(length("\u{200b}"), 1);
    }

    #[test]
    fn text_reports_the_path_only_on_errors() {
        assert_eq!(text(|| unreachable!(), "abc", 3), Ok(3));
        assert_eq!(
            text(|| "title".into(), "abcd", 3),
            Err(ValidationError::TooLong {
                field: "title".into(),
                max: 3,
                actual: 4
            })
        );
    }

    #[test]
    fn paths_are_written_like_rustydiscord_writes_them() {
        let root = Path::Root;
        assert_eq!(root.field("name"), "name");
        assert!(matches!(root.field("name"), Cow::Borrowed(_)));
        assert_eq!(
            root.entry("name_localizations", "pl"),
            "name_localizations.pl"
        );
        let option = root.item("options", 2);
        let choice = option.item("choices", 0);
        assert_eq!(option.this(), "options[2]");
        let accessory = root.item("components", 1);
        let accessory = accessory.child("accessory");
        assert_eq!(
            accessory.field("custom_id"),
            "components[1].accessory.custom_id"
        );
        assert_eq!(root.child("component").this(), "component");
        assert_eq!(choice.field("name"), "options[2].choices[0].name");
        assert_eq!(
            choice.entry("name_localizations", "de"),
            "options[2].choices[0].name_localizations.de"
        );
    }

    #[test]
    fn a_required_text_must_not_be_blank() {
        assert_eq!(required_text(|| unreachable!(), " a ", 1), Ok(1));
        assert_eq!(
            required_text(|| "description".into(), "   ", 100),
            Err(ValidationError::Missing {
                field: "description".into()
            })
        );
        assert!(matches!(
            required_text(|| "description".into(), "ab", 1),
            Err(ValidationError::TooLong { .. })
        ));
    }

    #[test]
    fn url_checks_the_scheme_without_regard_to_case() {
        assert!(url(|| "url".into(), "https://example.com", WEB).is_ok());
        assert!(url(|| "url".into(), "HTTP://example.com", WEB).is_ok());
        assert!(url(|| "url".into(), "attachment://a.png", WEB).is_err());
        assert!(url(|| "url".into(), "attachment://a.png", WEB_OR_ATTACHMENT).is_ok());
        assert!(url(|| "url".into(), "ftp://example.com", WEB_OR_ATTACHMENT).is_err());
        assert!(url(|| "url".into(), "http:/", WEB).is_err());
        assert!(url(|| "url".into(), "", WEB).is_err());

        assert!(url(|| "url".into(), "ħttps://", WEB).is_err());
    }
}
