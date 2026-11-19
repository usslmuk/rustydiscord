use rustydiscord_model::component::{ActionRow, Component};

use super::validate;
use crate::{ValidationError, check::Path};

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct ActionRowBuilder {
    row: ActionRow,
}

impl Default for ActionRowBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRowBuilder {
    pub fn new() -> Self {
        Self {
            row: ActionRow::new(Vec::new()),
        }
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.row.components.push(component.into());
        self
    }

    pub fn components<C: Into<Component>>(
        mut self,
        components: impl IntoIterator<Item = C>,
    ) -> Self {
        self.row
            .components
            .extend(components.into_iter().map(Into::into));
        self
    }

    pub fn id(mut self, id: i32) -> Self {
        self.row.id = Some(id);
        self
    }

    pub fn build(self) -> Result<Component, ValidationError> {
        validate::action_row(&Path::Root, &self.row)?;
        Ok(self.into())
    }

    #[must_use]
    pub fn build_unchecked(self) -> Component {
        self.into()
    }
}

impl From<ActionRowBuilder> for Component {
    fn from(row: ActionRowBuilder) -> Self {
        Self::ActionRow(row.row)
    }
}
