use compact_str::CompactString;
use rustydiscord_model::component::{
    Component, Container, Section, Separator, SeparatorSpacing, TextDisplay,
};

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct ContainerBuilder {
    component: Container,
}

impl Default for ContainerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ContainerBuilder {
    pub fn new() -> Self {
        Self {
            component: Container::new(Vec::new()),
        }
    }

    pub fn text(self, content: impl Into<CompactString>) -> Self {
        self.component(TextDisplay::new(content))
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.component.components.push(component.into());
        self
    }

    pub fn components<C: Into<Component>>(
        mut self,
        components: impl IntoIterator<Item = C>,
    ) -> Self {
        self.component
            .components
            .extend(components.into_iter().map(Into::into));
        self
    }

    pub fn accent_color(mut self, color: u32) -> Self {
        self.component.accent_color = Some(color);
        self
    }

    pub fn spoiler(mut self, spoiler: bool) -> Self {
        self.component.spoiler = spoiler;
        self
    }
}

component_builder!(ContainerBuilder => Container);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct SectionBuilder {
    component: Section,
}

impl SectionBuilder {
    pub fn new(accessory: impl Into<Component>) -> Self {
        Self {
            component: Section::new(Vec::new(), accessory.into()),
        }
    }

    pub fn text(self, content: impl Into<CompactString>) -> Self {
        self.component(TextDisplay::new(content))
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.component.components.push(component.into());
        self
    }
}

component_builder!(SectionBuilder => Section);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a message or built"]
pub struct SeparatorBuilder {
    component: Separator,
}

impl SeparatorBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn divider(mut self, divider: bool) -> Self {
        self.component.divider = divider;
        self
    }

    pub fn spacing(mut self, spacing: SeparatorSpacing) -> Self {
        self.component.spacing = Some(spacing);
        self
    }
}

component_builder!(SeparatorBuilder => Separator);
