use core::fmt;

macro_rules! wrapper {
    (
        $(#[$meta:meta])*
        $function:ident => $type:ident, $open:literal, $close:literal,
        $js:literal, $example:literal => $output:literal
    ) => {
        #[doc = concat!("Content written as `", $open, "…", $close, "`. Returned by [`", stringify!($function), "`].")]
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        #[must_use = "a formatter does nothing until it is written"]
        pub struct $type<T>(T);

        $(#[$meta])*

        #[doc = concat!("discord.js counterpart: `", $js, "`.")]
        #[doc = concat!("use crate::formatters::", stringify!($function), ";")]
        #[doc = concat!("assert_eq!(", stringify!($function), "(", $example, ").to_string(), ", $output, ");")]
        pub const fn $function<T: fmt::Display>(content: T) -> $type<T> {
            $type(content)
        }

        impl<T: fmt::Display> fmt::Display for $type<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, concat!($open, "{}", $close), self.0)
            }
        }
    };
}

wrapper! {

    bold => Bold, "**", "**", "bold", r#""important""# => r#""**important**""#
}

wrapper! {

    italic => Italic, "_", "_", "italic", r#""maybe""# => r#""_maybe_""#
}

wrapper! {

    underline => Underline, "__", "__", "underline", r#""note""# => r#""__note__""#
}

pub const fn underscore<T: fmt::Display>(content: T) -> Underline<T> {
    Underline(content)
}

wrapper! {

    strikethrough => Strikethrough, "~~", "~~", "strikethrough", r#""old""# => r#""~~old~~""#
}

wrapper! {

    spoiler => Spoiler, "||", "||", "spoiler", r#""the ending""# => r#""||the ending||""#
}

wrapper! {



    inline_code => InlineCode, "`", "`", "inlineCode", r#""cargo build""# => r#""`cargo build`""#
}

wrapper! {



    quote => Quote, "> ", "", "quote", r#""well said""# => r#""> well said""#
}

wrapper! {

    block_quote => BlockQuote, ">>> ", "", "blockQuote", r#""line 1\nline 2""# => r#"">>> line 1\nline 2""#
}

wrapper! {

    subtext => Subtext, "-# ", "", "subtext", r#""edited""# => r#""-# edited""#
}

wrapper! {

    hide_link_embed => HideLinkEmbed, "<", ">", "hideLinkEmbed", r#""https://example.com""# => r#""<https://example.com>""#
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct CodeBlock<'a, T> {
    content: T,
    language: Option<&'a str>,
}

pub const fn code_block<'a, T: fmt::Display>(content: T) -> CodeBlock<'a, T> {
    CodeBlock {
        content,
        language: None,
    }
}

impl<'a, T> CodeBlock<'a, T> {
    pub const fn language(mut self, language: &'a str) -> Self {
        self.language = Some(language);
        self
    }
}

impl<T: fmt::Display> fmt::Display for CodeBlock<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "```{}\n{}\n```",
            self.language.unwrap_or_default(),
            self.content
        )
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum HeadingLevel {
    #[default]
    One,

    Two,

    Three,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct Heading<T> {
    content: T,
    level: HeadingLevel,
}

pub const fn heading<T: fmt::Display>(content: T) -> Heading<T> {
    Heading {
        content,
        level: HeadingLevel::One,
    }
}

impl<T> Heading<T> {
    pub const fn level(mut self, level: HeadingLevel) -> Self {
        self.level = level;
        self
    }
}

impl<T: fmt::Display> fmt::Display for Heading<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let marker = match self.level {
            HeadingLevel::One => "#",
            HeadingLevel::Two => "##",
            HeadingLevel::Three => "###",
        };
        write!(f, "{marker} {}", self.content)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct Hyperlink<'a, T, U> {
    content: T,
    url: U,
    title: Option<&'a str>,
}

pub const fn hyperlink<'a, T: fmt::Display, U: fmt::Display>(
    content: T,
    url: U,
) -> Hyperlink<'a, T, U> {
    Hyperlink {
        content,
        url,
        title: None,
    }
}

impl<'a, T, U> Hyperlink<'a, T, U> {
    pub const fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }
}

impl<T: fmt::Display, U: fmt::Display> fmt::Display for Hyperlink<'_, T, U> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.title {
            Some(title) => write!(f, "[{}]({} \"{title}\")", self.content, self.url),
            None => write!(f, "[{}]({})", self.content, self.url),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ListItem<'a, T> {
    Text(T),

    Nested(&'a [ListItem<'a, T>]),
}

pub trait ListEntry: private::WriteEntry {}

impl<T: private::WriteEntry + ?Sized> ListEntry for T {}

mod private {
    use core::fmt;

    pub trait WriteEntry {
        fn write_entry(
            &self,
            f: &mut fmt::Formatter<'_>,
            depth: usize,
            number: Option<u64>,
            first: &mut bool,
        ) -> Result<Option<u64>, fmt::Error>;
    }

    pub(super) fn write_line(
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        number: Option<u64>,
        first: &mut bool,
        text: &dyn fmt::Display,
    ) -> Result<Option<u64>, fmt::Error> {
        if !*first {
            f.write_str("\n")?;
        }
        *first = false;
        for _ in 0..depth {
            f.write_str("  ")?;
        }
        if let Some(number) = number {
            write!(f, "{number}. {text}")?;
            Ok(Some(number.saturating_add(1)))
        } else {
            write!(f, "- {text}")?;
            Ok(None)
        }
    }
}

impl<T: fmt::Display + ?Sized> private::WriteEntry for T {
    fn write_entry(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        number: Option<u64>,
        first: &mut bool,
    ) -> Result<Option<u64>, fmt::Error> {
        private::write_line(f, depth, number, first, &self)
    }
}

impl<T: fmt::Display> private::WriteEntry for ListItem<'_, T> {
    fn write_entry(
        &self,
        f: &mut fmt::Formatter<'_>,
        depth: usize,
        number: Option<u64>,
        first: &mut bool,
    ) -> Result<Option<u64>, fmt::Error> {
        match self {
            Self::Text(text) => private::write_line(f, depth, number, first, text),
            Self::Nested(items) => {
                let mut nested = number.map(|_| 1);
                for item in *items {
                    nested = item.write_entry(f, depth + 1, nested, first)?;
                }
                Ok(number)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct UnorderedList<'a, T> {
    items: &'a [T],
}

pub const fn unordered_list<T: ListEntry>(items: &[T]) -> UnorderedList<'_, T> {
    UnorderedList { items }
}

impl<T: ListEntry> fmt::Display for UnorderedList<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for item in self.items {
            item.write_entry(f, 0, None, &mut first)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[must_use = "a formatter does nothing until it is written"]
pub struct OrderedList<'a, T> {
    items: &'a [T],
    start: u64,
}

pub const fn ordered_list<T: ListEntry>(items: &[T]) -> OrderedList<'_, T> {
    OrderedList { items, start: 1 }
}

impl<T> OrderedList<'_, T> {
    pub const fn start(mut self, start: u64) -> Self {
        self.start = if start == 0 { 1 } else { start };
        self
    }
}

impl<T: ListEntry> fmt::Display for OrderedList<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        let mut number = Some(self.start);
        for item in self.items {
            number = item.write_entry(f, 0, number, &mut first)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Face {
    Shrug,

    Tableflip,

    Unflip,
}

impl Face {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shrug => r"¯\_(ツ)_/¯",
            Self::Tableflip => "(╯°□°)╯︵ ┻━┻",
            Self::Unflip => "┬─┬ノ( º _ ºノ)",
        }
    }
}

impl fmt::Display for Face {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
