use crate::global_attributes::Attribute;

use super::Tag;

/// For
#[derive(Debug, Clone)]
pub struct For(String);

impl For {
    #[must_use]
    pub fn new(for_: &str) -> Self {
        Self(for_.to_string())
    }
}

impl Attribute for For {
    fn name(&self) -> &'static str {
        "for"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Label.
/// See [reference](TODO).
#[derive(Debug, Clone)]
pub struct Label {
    for_: For,
}

impl Label {
    #[must_use]
    pub fn new(for_: &str) -> Self {
        Self {
            for_: For::new(for_),
        }
    }
}

impl Tag for Label {
    fn name(&self) -> &'static str {
        "label"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(vec![&self.for_])
    }
}
