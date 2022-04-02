use std::num::NonZeroUsize;

use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug)]
pub struct Colspan(NonZeroUsize);

impl Attribute for Colspan {
    fn name(&self) -> &'static str {
        "colspan"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Shortcut for [`Td::default()`].
#[must_use]
pub fn td() -> Td {
    Td::default()
}

/// Table data cell.
#[derive(Debug, Default)]
pub struct Td {
    colspan: Option<Colspan>,
}

impl Td {
    #[must_use]
    pub fn colspan(span: usize) -> Self {
        Self {
            colspan: Some(Colspan(span.try_into().expect("Must be non-zero"))),
        }
    }
}

impl Tag for Td {
    fn name(&self) -> &'static str {
        "td"
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
