use crate::model::{
    message::{Embed, EmbedAuthor, EmbedField, EmbedFooter, EmbedImage},
    util::Timestamp,
};
use compact_str::CompactString;

use crate::builders::{
    ValidationError,
    check::{self, WEB, WEB_OR_ATTACHMENT},
};

mod limits {
    pub(super) const TITLE: usize = 256;
    pub(super) const DESCRIPTION: usize = 4096;
    pub(super) const FIELDS: usize = 25;
    pub(super) const FIELD_NAME: usize = 256;
    pub(super) const FIELD_VALUE: usize = 1024;
    pub(super) const FOOTER_TEXT: usize = 2048;
    pub(super) const AUTHOR_NAME: usize = 256;

    pub(super) const TOTAL: usize = 6000;

    pub(super) const EMBEDS: usize = 10;

    pub(super) const COLOR: u32 = 0xFF_FFFF;
}

const EMBED_CONTENT: &str =
    "a title, a description, a field, an author, a footer, an image or a thumbnail";

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct EmbedBuilder {
    embed: Embed,
}

impl EmbedBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn title(mut self, title: impl Into<CompactString>) -> Self {
        self.embed.title = Some(title.into());
        self
    }

    pub fn url(mut self, url: impl Into<CompactString>) -> Self {
        self.embed.url = Some(url.into());
        self
    }

    pub fn description(mut self, description: impl Into<CompactString>) -> Self {
        self.embed.description = Some(description.into());
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.embed.color = Some(color);
        self
    }

    pub fn color_rgb(self, red: u8, green: u8, blue: u8) -> Self {
        self.color(u32::from_be_bytes([0, red, green, blue]))
    }

    pub fn author(mut self, author: impl Into<EmbedAuthorBuilder>) -> Self {
        self.embed.author = Some(Box::new(author.into().author));
        self
    }

    pub fn footer(mut self, footer: impl Into<EmbedFooterBuilder>) -> Self {
        self.embed.footer = Some(Box::new(footer.into().footer));
        self
    }

    pub fn field(
        mut self,
        name: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.embed.fields.push(EmbedField::new(name, value, false));
        self
    }

    pub fn inline_field(
        mut self,
        name: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.embed.fields.push(EmbedField::new(name, value, true));
        self
    }

    pub fn fields(mut self, fields: impl IntoIterator<Item = EmbedField>) -> Self {
        self.embed.fields.extend(fields);
        self
    }

    pub fn image(mut self, url: impl Into<CompactString>) -> Self {
        self.embed.image = Some(Box::new(EmbedImage::new(url)));
        self
    }

    pub fn thumbnail(mut self, url: impl Into<CompactString>) -> Self {
        self.embed.thumbnail = Some(Box::new(EmbedImage::new(url)));
        self
    }

    pub fn timestamp(mut self, timestamp: Timestamp) -> Self {
        self.embed.timestamp = Some(timestamp);
        self
    }

    pub fn timestamp_now(self) -> Self {
        self.timestamp(Timestamp::now())
    }

    pub fn build(self) -> Result<Embed, ValidationError> {
        validate(&self.embed)?;
        Ok(self.embed)
    }

    #[must_use]
    pub fn build_unchecked(self) -> Embed {
        self.embed
    }
}

impl From<Embed> for EmbedBuilder {
    fn from(embed: Embed) -> Self {
        Self { embed }
    }
}

fn validate(embed: &Embed) -> Result<(), ValidationError> {
    let mut total = 0;
    if let Some(title) = &embed.title {
        total += check::text(|| "title".into(), title, limits::TITLE)?;
    }
    if let Some(description) = &embed.description {
        total += check::text(|| "description".into(), description, limits::DESCRIPTION)?;
    }
    if let Some(url) = &embed.url {
        check::url(|| "url".into(), url, WEB)?;
    }
    if let Some(color) = embed.color
        && color > limits::COLOR
    {
        return Err(ValidationError::OutOfRange {
            field: "color".into(),
            min: 0,
            max: limits::COLOR.into(),
            actual: color.into(),
        });
    }
    check::count(|| "fields".into(), embed.fields.len(), limits::FIELDS)?;
    for (index, field) in embed.fields.iter().enumerate() {
        total += check::text(
            || format!("fields[{index}].name").into(),
            &field.name,
            limits::FIELD_NAME,
        )?;
        total += check::text(
            || format!("fields[{index}].value").into(),
            &field.value,
            limits::FIELD_VALUE,
        )?;
    }
    if let Some(footer) = &embed.footer {
        total += validate_footer(footer, &IN_EMBED_FOOTER)?;
    }
    if let Some(image) = &embed.image {
        check::url(|| "image.url".into(), &image.url, WEB_OR_ATTACHMENT)?;
    }
    if let Some(thumbnail) = &embed.thumbnail {
        check::url(|| "thumbnail.url".into(), &thumbnail.url, WEB_OR_ATTACHMENT)?;
    }
    if let Some(author) = &embed.author {
        total += validate_author(author, &IN_EMBED_AUTHOR)?;
    }
    if total > limits::TOTAL {
        return Err(ValidationError::TooLong {
            field: "embed".into(),
            max: limits::TOTAL,
            actual: total,
        });
    }
    if !has_content(embed) {
        return Err(ValidationError::Empty {
            field: "embed".into(),
            needs: EMBED_CONTENT,
        });
    }
    Ok(())
}

fn has_content(embed: &Embed) -> bool {
    let text = |text: Option<&str>| text.is_some_and(|text| !check::is_blank(text));
    text(embed.title.as_deref())
        || text(embed.description.as_deref())
        || !embed.fields.is_empty()
        || text(embed.footer.as_ref().map(|footer| footer.text.as_str()))
        || text(embed.author.as_ref().map(|author| author.name.as_str()))
        || embed.image.is_some()
        || embed.thumbnail.is_some()
}

#[must_use]
pub fn embed_length(embed: &Embed) -> usize {
    let mut total = 0;
    for text in [&embed.title, &embed.description].into_iter().flatten() {
        total += check::length(text);
    }
    for field in &embed.fields {
        total += check::length(&field.name) + check::length(&field.value);
    }
    if let Some(footer) = &embed.footer {
        total += check::length(&footer.text);
    }
    if let Some(author) = &embed.author {
        total += check::length(&author.name);
    }
    total
}

pub fn validate_embeds(embeds: &[Embed]) -> Result<(), ValidationError> {
    check::count(|| "embeds".into(), embeds.len(), limits::EMBEDS)?;
    let total = embeds.iter().map(embed_length).sum();
    if total > limits::TOTAL {
        return Err(ValidationError::TooLong {
            field: "embeds".into(),
            max: limits::TOTAL,
            actual: total,
        });
    }
    Ok(())
}

struct AuthorPaths {
    name: &'static str,
    url: &'static str,
    icon_url: &'static str,
}

const IN_EMBED_AUTHOR: AuthorPaths = AuthorPaths {
    name: "author.name",
    url: "author.url",
    icon_url: "author.icon_url",
};

const ALONE_AUTHOR: AuthorPaths = AuthorPaths {
    name: "name",
    url: "url",
    icon_url: "icon_url",
};

fn validate_author(author: &EmbedAuthor, paths: &AuthorPaths) -> Result<usize, ValidationError> {
    let length = check::text(|| paths.name.into(), &author.name, limits::AUTHOR_NAME)?;
    if let Some(url) = &author.url {
        check::url(|| paths.url.into(), url, WEB)?;
    }
    if let Some(icon_url) = &author.icon_url {
        check::url(|| paths.icon_url.into(), icon_url, WEB_OR_ATTACHMENT)?;
    }
    Ok(length)
}

struct FooterPaths {
    text: &'static str,
    icon_url: &'static str,
}

const IN_EMBED_FOOTER: FooterPaths = FooterPaths {
    text: "footer.text",
    icon_url: "footer.icon_url",
};

const ALONE_FOOTER: FooterPaths = FooterPaths {
    text: "text",
    icon_url: "icon_url",
};

fn validate_footer(footer: &EmbedFooter, paths: &FooterPaths) -> Result<usize, ValidationError> {
    let length = check::text(|| paths.text.into(), &footer.text, limits::FOOTER_TEXT)?;
    if let Some(icon_url) = &footer.icon_url {
        check::url(|| paths.icon_url.into(), icon_url, WEB_OR_ATTACHMENT)?;
    }
    Ok(length)
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[must_use = "a builder does nothing until it is passed to `EmbedBuilder::author` or built"]
pub struct EmbedAuthorBuilder {
    author: EmbedAuthor,
}

impl EmbedAuthorBuilder {
    pub fn new(name: impl Into<CompactString>) -> Self {
        Self {
            author: EmbedAuthor::new(name),
        }
    }

    pub fn url(mut self, url: impl Into<CompactString>) -> Self {
        self.author.url = Some(url.into());
        self
    }

    pub fn icon_url(mut self, url: impl Into<CompactString>) -> Self {
        self.author.icon_url = Some(url.into());
        self
    }

    pub fn build(self) -> Result<EmbedAuthor, ValidationError> {
        validate_author(&self.author, &ALONE_AUTHOR)?;
        Ok(self.author)
    }

    #[must_use]
    pub fn build_unchecked(self) -> EmbedAuthor {
        self.author
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[must_use = "a builder does nothing until it is passed to `EmbedBuilder::footer` or built"]
pub struct EmbedFooterBuilder {
    footer: EmbedFooter,
}

impl EmbedFooterBuilder {
    pub fn new(text: impl Into<CompactString>) -> Self {
        Self {
            footer: EmbedFooter::new(text),
        }
    }

    pub fn icon_url(mut self, url: impl Into<CompactString>) -> Self {
        self.footer.icon_url = Some(url.into());
        self
    }

    pub fn build(self) -> Result<EmbedFooter, ValidationError> {
        validate_footer(&self.footer, &ALONE_FOOTER)?;
        Ok(self.footer)
    }

    #[must_use]
    pub fn build_unchecked(self) -> EmbedFooter {
        self.footer
    }
}

macro_rules! from_text {
    ($builder:ident, $model:ident, $part:ident) => {
        impl From<&str> for $builder {
            fn from(text: &str) -> Self {
                Self::new(text)
            }
        }

        impl From<&String> for $builder {
            fn from(text: &String) -> Self {
                Self::new(text)
            }
        }

        impl From<String> for $builder {
            fn from(text: String) -> Self {
                Self::new(text)
            }
        }

        impl From<CompactString> for $builder {
            fn from(text: CompactString) -> Self {
                Self::new(text)
            }
        }

        impl From<$model> for $builder {
            fn from($part: $model) -> Self {
                Self { $part }
            }
        }
    };
}

from_text!(EmbedAuthorBuilder, EmbedAuthor, author);
from_text!(EmbedFooterBuilder, EmbedFooter, footer);
