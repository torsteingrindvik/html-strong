use crate::global_attributes::Attribute;

use super::td::Colspan;
use super::Tag;

/// Shortcut for [`Th::default()`].
#[must_use]
pub fn th() -> Th {
    Th::default()
}

/// Th.
#[derive(Debug, Clone, Default)]
pub struct Th {
    colspan: Option<Colspan>,
}

impl Th {
    #[must_use]
    pub fn colspan(span: usize) -> Self {
        Self {
            colspan: Some(Colspan(span.try_into().expect("Must be non-zero"))),
        }
    }
}

impl Tag for Th {
    fn name(&self) -> &'static str {
        "th"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        let mut attrs: Vec<&dyn Attribute> = vec![];

        if let Some(colspan) = &self.colspan {
            attrs.push(colspan);
        }

        if attrs.is_empty() {
            None
        } else {
            Some(attrs)
        }
    }
}
