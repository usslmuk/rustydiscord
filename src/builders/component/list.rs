use crate::model::component::Component;

use super::validate::{self, Totals};
use crate::builders::{
    ValidationError,
    check::{self, Path},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ComponentsVersion {
    Legacy,

    V2,
}

const LEGACY_ROWS: usize = 5;

const V2_COMPONENTS: usize = 40;

const V2_TEXT: usize = 4000;

pub fn validate_components(
    components: &[Component],
    version: ComponentsVersion,
) -> Result<(), ValidationError> {
    let root = Path::Root;
    if version == ComponentsVersion::Legacy {
        check::count(|| "components".into(), components.len(), LEGACY_ROWS)?;
    }
    let mut totals = Totals::new();
    for (index, component) in components.iter().enumerate() {
        let here = root.item("components", index);
        let allowed = match version {
            ComponentsVersion::Legacy => matches!(component, Component::ActionRow(_)),
            ComponentsVersion::V2 => matches!(
                component,
                Component::ActionRow(_)
                    | Component::TextDisplay(_)
                    | Component::Section(_)
                    | Component::MediaGallery(_)
                    | Component::File(_)
                    | Component::Separator(_)
                    | Component::Container(_)
            ),
        };
        if !allowed {
            return Err(ValidationError::Invalid {
                field: here.this(),
                reason: match version {
                    ComponentsVersion::Legacy => {
                        "a message without the Components V2 flag can only have action rows"
                    }
                    ComponentsVersion::V2 => {
                        "this component cannot be at the top level of a message"
                    }
                },
            });
        }
        validate::component(&here, component)?;
        totals.walk(&here, component)?;
        if version == ComponentsVersion::V2 && totals.components > V2_COMPONENTS {
            totals.stop_comparing_ids();
        }
    }
    if version == ComponentsVersion::V2 {
        check::count(|| "components".into(), totals.components, V2_COMPONENTS)?;
        if totals.text > V2_TEXT {
            return Err(ValidationError::TooLong {
                field: "components".into(),
                max: V2_TEXT,
                actual: totals.text,
            });
        }
    }
    Ok(())
}
