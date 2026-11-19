use compact_str::CompactString;
use rustydiscord_model::{
    component::{Button, ButtonStyle, Component, InteractiveButton, LinkButton, PremiumButton},
    emoji::PartialEmoji,
    id::SkuId,
};

use super::validate;
use crate::{ValidationError, check::Path};

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct ButtonBuilder {
    button: Button,
}

impl ButtonBuilder {
    fn interactive(style: ButtonStyle, custom_id: impl Into<CompactString>) -> Self {
        Self {
            button: InteractiveButton::new(style, custom_id).into(),
        }
    }

    pub fn primary(custom_id: impl Into<CompactString>) -> Self {
        Self::interactive(ButtonStyle::Primary, custom_id)
    }

    pub fn secondary(custom_id: impl Into<CompactString>) -> Self {
        Self::interactive(ButtonStyle::Secondary, custom_id)
    }

    pub fn success(custom_id: impl Into<CompactString>) -> Self {
        Self::interactive(ButtonStyle::Success, custom_id)
    }

    pub fn danger(custom_id: impl Into<CompactString>) -> Self {
        Self::interactive(ButtonStyle::Danger, custom_id)
    }

    pub fn link(url: impl Into<CompactString>) -> Self {
        Self {
            button: LinkButton::new(url).into(),
        }
    }

    pub fn premium(sku_id: SkuId) -> PremiumButtonBuilder {
        PremiumButtonBuilder {
            button: PremiumButton::new(sku_id),
        }
    }

    pub fn label(mut self, label: impl Into<CompactString>) -> Self {
        let label = Some(label.into());
        match &mut self.button {
            Button::Interactive(button) => button.label = label,
            Button::Link(button) => button.label = label,
            _ => {}
        }
        self
    }

    pub fn emoji(mut self, emoji: impl Into<PartialEmoji>) -> Self {
        let emoji = Some(emoji.into());
        match &mut self.button {
            Button::Interactive(button) => button.emoji = emoji,
            Button::Link(button) => button.emoji = emoji,
            _ => {}
        }
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        match &mut self.button {
            Button::Interactive(button) => button.disabled = disabled,
            Button::Link(button) => button.disabled = disabled,
            _ => {}
        }
        self
    }

    pub fn id(mut self, id: i32) -> Self {
        match &mut self.button {
            Button::Interactive(button) => button.id = Some(id),
            Button::Link(button) => button.id = Some(id),
            _ => {}
        }
        self
    }

    pub fn build(self) -> Result<Component, ValidationError> {
        validate::button(&Path::Root, &self.button)?;
        Ok(self.into())
    }

    #[must_use]
    pub fn build_unchecked(self) -> Component {
        self.into()
    }
}

impl From<ButtonBuilder> for Component {
    fn from(button: ButtonBuilder) -> Self {
        Self::Button(button.button)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a row or built"]
pub struct PremiumButtonBuilder {
    button: PremiumButton,
}

impl PremiumButtonBuilder {
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.button.disabled = disabled;
        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.button.id = Some(id);
        self
    }

    pub fn build(self) -> Result<Component, ValidationError> {
        Ok(self.into())
    }

    #[must_use]
    pub fn build_unchecked(self) -> Component {
        self.into()
    }
}

impl From<PremiumButtonBuilder> for Component {
    fn from(button: PremiumButtonBuilder) -> Self {
        button.button.into()
    }
}
