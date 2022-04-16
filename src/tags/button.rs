use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
pub enum Type {
    Submit,
}

impl Attribute for Type {
    fn name(&self) -> &'static str {
        "type"
    }

    fn value(&self) -> String {
        "submit".into()
    }
}

/// A button.
#[derive(Debug, Clone)]
pub struct Button {
    type_: Type,
}

impl Button {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            type_: Type::Submit,
        }
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag for Button {
    fn name(&self) -> &'static str {
        "button"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(vec![&self.type_])
    }
}
