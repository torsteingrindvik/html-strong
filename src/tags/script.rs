use crate::global_attributes::Attribute;

use super::{Tag, img::Src};

/// Script.
#[derive(Debug, Clone)]
pub struct Script {
    src: Option<Src>,
}

impl Script {
    #[must_use]
    pub const fn new() -> Self {
        Self { src: None }
    }

    #[must_use]
    pub fn src(src: &str) -> Self {
        Self {
            src: Some(Src(src.to_string())),
        }
    }
}

impl Default for Script {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag for Script {
    fn name(&self) -> &'static str {
        "script"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        self.src.as_ref().map(|src| vec![src as &dyn Attribute])
    }
}
