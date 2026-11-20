macro_rules! component_builder {
    ($builder:ident => $variant:ident) => {
        impl $builder {
            pub fn id(mut self, id: i32) -> Self {
                self.component.id = Some(id);
                self
            }

            pub fn build(
                self,
            ) -> Result<crate::model::component::Component, crate::builders::ValidationError> {
                let component = crate::model::component::Component::from(self);
                super::validate::component(&crate::builders::check::Path::Root, &component)?;
                Ok(component)
            }

            #[must_use]
            pub fn build_unchecked(self) -> crate::model::component::Component {
                self.into()
            }
        }

        impl From<$builder> for crate::model::component::Component {
            fn from(builder: $builder) -> Self {
                Self::$variant(builder.component)
            }
        }
    };
}

mod action_row;
mod button;
mod content;
mod layout;
mod list;
mod modal;
mod select;
mod validate;

pub use self::{
    action_row::ActionRowBuilder,
    button::{ButtonBuilder, PremiumButtonBuilder},
    content::{
        FileBuilder, MediaGalleryBuilder, MediaGalleryItemBuilder, TextDisplayBuilder,
        ThumbnailBuilder,
    },
    layout::{ContainerBuilder, SectionBuilder, SeparatorBuilder},
    list::{ComponentsVersion, validate_components},
    modal::{
        CheckboxBuilder, CheckboxGroupBuilder, CheckboxGroupOptionBuilder, FileUploadBuilder,
        LabelBuilder, ModalBuilder, RadioGroupBuilder, RadioGroupOptionBuilder, TextInputBuilder,
    },
    select::{
        ChannelSelectMenuBuilder, MentionableSelectMenuBuilder, RoleSelectMenuBuilder,
        StringSelectMenuBuilder, StringSelectMenuOptionBuilder, UserSelectMenuBuilder,
    },
};
