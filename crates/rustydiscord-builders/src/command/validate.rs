use std::{borrow::Cow, collections::BTreeMap};

use compact_str::CompactString;
use rustydiscord_model::{
    application_command::{Choice, CommandOption, IntegerOption, NumberOption, StringOption},
    rest::CreateApplicationCommand,
    util::Locale,
};

use crate::{
    ValidationError,
    check::{self, Path},
};

mod limits {
    pub(super) const NAME: usize = 32;
    pub(super) const DESCRIPTION: usize = 100;
    pub(super) const OPTIONS: usize = 25;
    pub(super) const CHOICES: usize = 25;
    pub(super) const CHOICE_NAME: usize = 100;
    pub(super) const CHOICE_VALUE: usize = 100;

    pub(super) const LENGTH: u16 = 6000;

    pub(super) const TOTAL: usize = 8000;

    pub(super) const INTEGER: i64 = (1 << 53) - 1;

    #[expect(clippy::cast_precision_loss, reason = "2^53 is exactly representable")]
    pub(super) const NUMBER: f64 = (1_i64 << 53) as f64;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Level {
    Command,

    Group,

    Subcommand,
}

pub(super) fn slash_command(command: &CreateApplicationCommand) -> Result<(), ValidationError> {
    let root = Path::Root;
    let mut total = slash_names(&root, &command.name, &command.name_localizations)?;
    total += descriptions(
        &root,
        command.description.as_deref().unwrap_or_default(),
        &command.description_localizations,
    )?;
    total += options(&root, &command.options, Level::Command)?;
    if total > limits::TOTAL {
        return Err(ValidationError::TooLong {
            field: "command".into(),
            max: limits::TOTAL,
            actual: total,
        });
    }
    Ok(())
}

pub(super) fn context_menu_command(
    command: &CreateApplicationCommand,
) -> Result<(), ValidationError> {
    check::required_text(|| "name".into(), &command.name, limits::NAME)?;
    for (locale, name) in &command.name_localizations {
        check::required_text(
            || Path::Root.entry("name_localizations", locale.as_str()),
            name,
            limits::NAME,
        )?;
    }
    Ok(())
}

pub(super) fn option_alone(option: &CommandOption) -> Result<(), ValidationError> {
    self::option(&Path::Root, option).map(drop)
}

fn is_name_character(character: char) -> bool {
    matches!(character, '-' | '_' | '\u{2bc}')
        || character.is_alphabetic()
        || character.is_numeric()
        || matches!(
            character,
            '\u{900}'..='\u{97f}' | '\u{a8e0}'..='\u{a8ff}' | '\u{11b00}'..='\u{11b5f}' | '\u{e00}'..='\u{e7f}'
        )
}

fn is_valid_in_name(character: char) -> bool {
    if character.is_ascii() {
        return character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '-' | '_');
    }
    is_name_character(character) && !has_lower_case_form(character)
}

fn has_lower_case_form(character: char) -> bool {
    let mut lower = character.to_lowercase();
    !(lower.len() == 1 && lower.next() == Some(character))
}

fn slash_name(field: impl Fn() -> Cow<'static, str>, name: &str) -> Result<usize, ValidationError> {
    let length = name.chars().count();
    if length == 0 {
        return Err(ValidationError::Missing { field: field() });
    }
    if length > limits::NAME {
        return Err(ValidationError::TooLong {
            field: field(),
            max: limits::NAME,
            actual: length,
        });
    }
    if let Some(character) = name.chars().find(|&character| !is_valid_in_name(character)) {
        return Err(ValidationError::InvalidName {
            field: field(),
            character,
        });
    }
    Ok(length)
}

fn slash_names(
    path: &Path<'_>,
    name: &str,
    localizations: &BTreeMap<Locale, CompactString>,
) -> Result<usize, ValidationError> {
    let mut longest = slash_name(|| path.field("name"), name)?;
    for (locale, name) in localizations {
        let length = slash_name(|| path.entry("name_localizations", locale.as_str()), name)?;
        longest = longest.max(length);
    }
    Ok(longest)
}

fn descriptions(
    path: &Path<'_>,
    description: &str,
    localizations: &BTreeMap<Locale, CompactString>,
) -> Result<usize, ValidationError> {
    let mut longest = check::required_text(
        || path.field("description"),
        description,
        limits::DESCRIPTION,
    )?;
    for (locale, description) in localizations {
        let length = check::required_text(
            || path.entry("description_localizations", locale.as_str()),
            description,
            limits::DESCRIPTION,
        )?;
        longest = longest.max(length);
    }
    Ok(longest)
}

const fn nests(option: &CommandOption) -> bool {
    matches!(
        option,
        CommandOption::SubCommand(_) | CommandOption::SubCommandGroup(_)
    )
}

const fn required(option: &CommandOption) -> Option<bool> {
    match option {
        CommandOption::String(option) => Some(option.required),
        CommandOption::Integer(option) => Some(option.required),
        CommandOption::Number(option) => Some(option.required),
        CommandOption::Boolean(option)
        | CommandOption::User(option)
        | CommandOption::Role(option)
        | CommandOption::Mentionable(option) => Some(option.required),
        CommandOption::Channel(option) => Some(option.required),
        CommandOption::Attachment(option) => Some(option.required),
        CommandOption::Unknown(option) => Some(option.required),
        _ => None,
    }
}

pub(super) fn options(
    path: &Path<'_>,
    options: &[CommandOption],
    level: Level,
) -> Result<usize, ValidationError> {
    check::count(|| path.field("options"), options.len(), limits::OPTIONS)?;
    let first_nests = options.first().is_some_and(nests);
    let mut optional_seen = false;
    let mut total = 0;
    for (index, option) in options.iter().enumerate() {
        let here = path.item("options", index);
        let misplaced = match level {
            Level::Command if nests(option) != first_nests => {
                Some("subcommands and subcommand groups cannot be mixed with other options")
            }
            Level::Group if !matches!(option, CommandOption::SubCommand(_)) => {
                Some("a subcommand group can only contain subcommands")
            }
            Level::Subcommand if nests(option) => {
                Some("a subcommand cannot contain subcommands or subcommand groups")
            }
            _ => None,
        };
        if let Some(reason) = misplaced {
            return Err(ValidationError::Invalid {
                field: here.this(),
                reason,
            });
        }
        if options[..index]
            .iter()
            .any(|other| other.name() == option.name())
        {
            return Err(ValidationError::Invalid {
                field: here.field("name"),
                reason: "another option at this level has the same name",
            });
        }
        if let Some(required) = required(option) {
            if required && optional_seen {
                return Err(ValidationError::Invalid {
                    field: here.field("required"),
                    reason: "required options must come before optional ones",
                });
            }
            optional_seen |= !required;
        }
        total += self::option(&here, option)?;
    }
    Ok(total)
}

fn option(path: &Path<'_>, option: &CommandOption) -> Result<usize, ValidationError> {
    let info = option.common();
    let mut total = slash_names(path, info.name, info.name_localizations)?;
    total += descriptions(path, info.description, info.description_localizations)?;
    total += match option {
        CommandOption::SubCommand(subcommand) => {
            options(path, &subcommand.options, Level::Subcommand)?
        }
        CommandOption::SubCommandGroup(group) => options(path, &group.options, Level::Group)?,
        CommandOption::String(option) => string(path, option)?,
        CommandOption::Integer(option) => integer(path, option)?,
        CommandOption::Number(option) => number(path, option)?,
        _ => 0,
    };
    Ok(total)
}

fn choice_names<T>(path: &Path<'_>, choice: &Choice<T>) -> Result<usize, ValidationError> {
    let mut longest =
        check::required_text(|| path.field("name"), &choice.name, limits::CHOICE_NAME)?;
    for (locale, name) in &choice.name_localizations {
        let length = check::required_text(
            || path.entry("name_localizations", locale.as_str()),
            name,
            limits::CHOICE_NAME,
        )?;
        longest = longest.max(length);
    }
    Ok(longest)
}

fn autocomplete(
    path: &Path<'_>,
    autocomplete: bool,
    choices: usize,
) -> Result<(), ValidationError> {
    if autocomplete && choices > 0 {
        return Err(ValidationError::Invalid {
            field: path.field("autocomplete"),
            reason: "an option with choices cannot use autocomplete",
        });
    }
    Ok(())
}

fn string(path: &Path<'_>, option: &StringOption) -> Result<usize, ValidationError> {
    check::count(
        || path.field("choices"),
        option.choices.len(),
        limits::CHOICES,
    )?;
    let mut total = 0;
    for (index, choice) in option.choices.iter().enumerate() {
        let here = path.item("choices", index);
        total += choice_names(&here, choice)?;
        total += check::text(|| here.field("value"), &choice.value, limits::CHOICE_VALUE)?;
    }
    if let Some(min) = option.min_length
        && min > limits::LENGTH
    {
        return Err(ValidationError::OutOfRange {
            field: path.field("min_length"),
            min: 0,
            max: limits::LENGTH.into(),
            actual: min.into(),
        });
    }
    if let Some(max) = option.max_length {
        let lowest = option.min_length.unwrap_or(0).max(1);
        if !(lowest..=limits::LENGTH).contains(&max) {
            return Err(ValidationError::OutOfRange {
                field: path.field("max_length"),
                min: lowest.into(),
                max: limits::LENGTH.into(),
                actual: max.into(),
            });
        }
    }
    autocomplete(path, option.autocomplete, option.choices.len())?;
    Ok(total)
}

fn integer_value(
    field: impl FnOnce() -> Cow<'static, str>,
    value: i64,
) -> Result<(), ValidationError> {
    if (-limits::INTEGER..=limits::INTEGER).contains(&value) {
        Ok(())
    } else {
        Err(ValidationError::OutOfRange {
            field: field(),
            min: -limits::INTEGER,
            max: limits::INTEGER,
            actual: value,
        })
    }
}

fn integer(path: &Path<'_>, option: &IntegerOption) -> Result<usize, ValidationError> {
    check::count(
        || path.field("choices"),
        option.choices.len(),
        limits::CHOICES,
    )?;
    let mut total = 0;
    for (index, choice) in option.choices.iter().enumerate() {
        let here = path.item("choices", index);
        total += choice_names(&here, choice)?;
        integer_value(|| here.field("value"), choice.value)?;
    }
    if let Some(min) = option.min_value {
        integer_value(|| path.field("min_value"), min)?;
    }
    if let Some(max) = option.max_value {
        integer_value(|| path.field("max_value"), max)?;
    }
    if let (Some(min), Some(max)) = (option.min_value, option.max_value)
        && min > max
    {
        return Err(ValidationError::Invalid {
            field: path.field("max_value"),
            reason: "max_value is below min_value, so no value can be entered",
        });
    }
    autocomplete(path, option.autocomplete, option.choices.len())?;
    Ok(total)
}

fn number_value(
    field: impl FnOnce() -> Cow<'static, str>,
    value: f64,
) -> Result<(), ValidationError> {
    if value.is_finite() && value.abs() <= limits::NUMBER {
        Ok(())
    } else {
        Err(ValidationError::Invalid {
            field: field(),
            reason: "a number must be finite and between -2^53 and 2^53",
        })
    }
}

fn number(path: &Path<'_>, option: &NumberOption) -> Result<usize, ValidationError> {
    check::count(
        || path.field("choices"),
        option.choices.len(),
        limits::CHOICES,
    )?;
    let mut total = 0;
    for (index, choice) in option.choices.iter().enumerate() {
        let here = path.item("choices", index);
        total += choice_names(&here, choice)?;
        number_value(|| here.field("value"), choice.value)?;
    }
    if let Some(min) = option.min_value {
        number_value(|| path.field("min_value"), min)?;
    }
    if let Some(max) = option.max_value {
        number_value(|| path.field("max_value"), max)?;
    }
    if let (Some(min), Some(max)) = (option.min_value, option.max_value)
        && min > max
    {
        return Err(ValidationError::Invalid {
            field: path.field("max_value"),
            reason: "max_value is below min_value, so no value can be entered",
        });
    }
    autocomplete(path, option.autocomplete, option.choices.len())?;
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ascii_shortcut_agrees_with_the_full_check() {
        for character in (0..128_u8).map(char::from) {
            assert_eq!(
                is_valid_in_name(character),
                is_name_character(character) && !has_lower_case_form(character),
                "{character:?}"
            );
        }
    }

    #[test]
    fn names_follow_discords_pattern() {
        let valid = |name: &str| slash_name(|| "name".into(), name).is_ok();

        assert!(valid("ping"));
        assert!(valid("donʼt"));
        assert!(valid("नमस्ते"));
        assert!(valid(&"ż".repeat(32)));
        assert!(valid("user-info_2"));

        assert!(valid("สวัสดี"));
        assert!(valid("天気"));
        assert!(valid("٣"));

        assert!(!valid("Ping"));
        assert!(!valid("Żółw"));
        assert!(!valid("don't"));

        assert!(!valid("two words"));
        assert!(!valid("ping!"));
        assert!(!valid("🔥"));
        assert!(!valid(""));
        assert!(!valid(&"a".repeat(33)));
    }

    #[test]
    fn a_title_case_letter_must_be_lower_case() {
        assert_eq!(
            slash_name(|| "name".into(), "\u{1c5}"),
            Err(ValidationError::InvalidName {
                field: "name".into(),
                character: '\u{1c5}'
            })
        );
        assert!(slash_name(|| "name".into(), "\u{1c6}").is_ok());
    }
}
