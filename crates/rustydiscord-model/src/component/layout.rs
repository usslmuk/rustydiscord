use compact_str::CompactString;
use serde::Serialize;

use super::Component;
use crate::macros::int_enum;

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ActionRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub components: Vec<Component>,
}

impl ActionRow {
    #[must_use]
    pub const fn new(components: Vec<Component>) -> Self {
        Self {
            id: None,
            components,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Section {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub components: Vec<Component>,

    pub accessory: Box<Component>,
}

impl Section {
    #[must_use]
    pub fn new(components: Vec<Component>, accessory: Component) -> Self {
        Self {
            id: None,
            components,
            accessory: Box::new(accessory),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Container {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub components: Vec<Component>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<u32>,

    #[serde(skip_serializing_if = "core::ops::Not::not")]
    pub spoiler: bool,
}

impl Container {
    #[must_use]
    pub const fn new(components: Vec<Component>) -> Self {
        Self {
            id: None,
            components,
            accent_color: None,
            spoiler: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Separator {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub divider: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spacing: Option<SeparatorSpacing>,
}

impl Separator {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            id: None,
            divider: true,
            spacing: None,
        }
    }
}

impl Default for Separator {
    fn default() -> Self {
        Self::new()
    }
}

int_enum! {



    #[doc(alias = "SeparatorSpacingSize")]
    pub enum SeparatorSpacing: u8 {

        Small = 1,

        Large = 2,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct Label {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,

    pub label: CompactString,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<CompactString>,

    pub component: Box<Component>,
}

impl Label {
    #[must_use]
    pub fn new(label: impl Into<CompactString>, component: Component) -> Self {
        Self {
            id: None,
            label: label.into(),
            description: None,
            component: Box::new(component),
        }
    }
}
