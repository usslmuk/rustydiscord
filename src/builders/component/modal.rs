use crate::model::{
    component::{
        Checkbox, CheckboxGroup, Component, FileUpload, GroupOption, Label, RadioGroup,
        TextDisplay, TextInput, TextInputStyle,
    },
    interaction::ModalCallbackData,
};
use compact_str::CompactString;

use super::validate;
use crate::builders::ValidationError;

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until `build` is called"]
pub struct ModalBuilder {
    modal: ModalCallbackData,
}

impl ModalBuilder {
    pub fn new(custom_id: impl Into<CompactString>, title: impl Into<CompactString>) -> Self {
        Self {
            modal: ModalCallbackData::new(custom_id, title, Vec::new()),
        }
    }

    pub fn label(self, label: impl Into<CompactString>, field: impl Into<Component>) -> Self {
        self.component(LabelBuilder::new(label, field))
    }

    pub fn text(self, content: impl Into<CompactString>) -> Self {
        self.component(TextDisplay::new(content))
    }

    pub fn component(mut self, component: impl Into<Component>) -> Self {
        self.modal.components.push(component.into());
        self
    }

    pub fn build(self) -> Result<ModalCallbackData, ValidationError> {
        validate::modal(&self.modal)?;
        Ok(self.modal)
    }

    #[must_use]
    pub fn build_unchecked(self) -> ModalCallbackData {
        self.modal
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a modal or built"]
pub struct LabelBuilder {
    component: Label,
}

impl LabelBuilder {
    pub fn new(label: impl Into<CompactString>, field: impl Into<Component>) -> Self {
        Self {
            component: Label::new(label, field.into()),
        }
    }

    pub fn description(mut self, description: impl Into<CompactString>) -> Self {
        self.component.description = Some(description.into());
        self
    }
}

component_builder!(LabelBuilder => Label);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a label or built"]
pub struct TextInputBuilder {
    component: TextInput,
}

impl TextInputBuilder {
    pub fn short(custom_id: impl Into<CompactString>) -> Self {
        Self {
            component: TextInput::new(custom_id, TextInputStyle::Short),
        }
    }

    pub fn paragraph(custom_id: impl Into<CompactString>) -> Self {
        Self {
            component: TextInput::new(custom_id, TextInputStyle::Paragraph),
        }
    }

    pub fn min_length(mut self, min_length: u16) -> Self {
        self.component.min_length = Some(min_length);
        self
    }

    pub fn max_length(mut self, max_length: u16) -> Self {
        self.component.max_length = Some(max_length);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.component.required = required;
        self
    }

    pub fn value(mut self, value: impl Into<CompactString>) -> Self {
        self.component.value = Some(value.into());
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<CompactString>) -> Self {
        self.component.placeholder = Some(placeholder.into());
        self
    }
}

component_builder!(TextInputBuilder => TextInput);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a label or built"]
pub struct FileUploadBuilder {
    component: FileUpload,
}

impl FileUploadBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            component: FileUpload::new(custom_id),
        }
    }

    pub fn min_values(mut self, min_values: u8) -> Self {
        self.component.min_values = Some(min_values);
        self
    }

    pub fn max_values(mut self, max_values: u8) -> Self {
        self.component.max_values = Some(max_values);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.component.required = required;
        self
    }

    pub fn file_types<T: Into<CompactString>>(
        mut self,
        file_types: impl IntoIterator<Item = T>,
    ) -> Self {
        self.component.file_types = file_types.into_iter().map(Into::into).collect();
        self
    }
}

component_builder!(FileUploadBuilder => FileUpload);

macro_rules! group_option {
    ($(#[$meta:meta])* $builder:ident) => {
        $(#[$meta])*
        #[derive(Clone, Debug, PartialEq, Eq)]
        #[must_use = "an option does nothing until it is added to a group"]
        pub struct $builder {
            option: GroupOption,
        }

        impl $builder {




            pub fn new(label: impl Into<CompactString>, value: impl Into<CompactString>) -> Self {
                Self {
                    option: GroupOption::new(label, value),
                }
            }




            pub fn description(mut self, description: impl Into<CompactString>) -> Self {
                self.option.description = Some(description.into());
                self
            }




            pub fn default(mut self, default: bool) -> Self {
                self.option.default = default;
                self
            }
        }

        impl From<$builder> for GroupOption {
            fn from(option: $builder) -> Self {
                option.option
            }
        }
    };
}

group_option! {




    RadioGroupOptionBuilder
}

group_option! {




    CheckboxGroupOptionBuilder
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a label or built"]
pub struct RadioGroupBuilder {
    component: RadioGroup,
}

impl RadioGroupBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            component: RadioGroup::new(custom_id, Vec::new()),
        }
    }

    pub fn option(
        mut self,
        label: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.component.options.push(GroupOption::new(label, value));
        self
    }

    pub fn option_with(mut self, option: RadioGroupOptionBuilder) -> Self {
        self.component.options.push(option.into());
        self
    }

    pub fn options<O: Into<GroupOption>>(mut self, options: impl IntoIterator<Item = O>) -> Self {
        self.component
            .options
            .extend(options.into_iter().map(Into::into));
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.component.required = required;
        self
    }
}

component_builder!(RadioGroupBuilder => RadioGroup);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a label or built"]
pub struct CheckboxGroupBuilder {
    component: CheckboxGroup,
}

impl CheckboxGroupBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            component: CheckboxGroup::new(custom_id, Vec::new()),
        }
    }

    pub fn option(
        mut self,
        label: impl Into<CompactString>,
        value: impl Into<CompactString>,
    ) -> Self {
        self.component.options.push(GroupOption::new(label, value));
        self
    }

    pub fn option_with(mut self, option: CheckboxGroupOptionBuilder) -> Self {
        self.component.options.push(option.into());
        self
    }

    pub fn options<O: Into<GroupOption>>(mut self, options: impl IntoIterator<Item = O>) -> Self {
        self.component
            .options
            .extend(options.into_iter().map(Into::into));
        self
    }

    pub fn min_values(mut self, min_values: u8) -> Self {
        self.component.min_values = Some(min_values);
        self
    }

    pub fn max_values(mut self, max_values: u8) -> Self {
        self.component.max_values = Some(max_values);
        self
    }

    pub fn required(mut self, required: bool) -> Self {
        self.component.required = required;
        self
    }
}

component_builder!(CheckboxGroupBuilder => CheckboxGroup);

#[derive(Clone, Debug, PartialEq, Eq)]
#[must_use = "a builder does nothing until it is added to a label or built"]
pub struct CheckboxBuilder {
    component: Checkbox,
}

impl CheckboxBuilder {
    pub fn new(custom_id: impl Into<CompactString>) -> Self {
        Self {
            component: Checkbox::new(custom_id),
        }
    }

    pub fn default(mut self, default: bool) -> Self {
        self.component.default = default;
        self
    }
}

component_builder!(CheckboxBuilder => Checkbox);
