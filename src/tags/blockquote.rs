use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
pub struct Cite(String);

impl Attribute for Cite {
    fn name(&self) -> &'static str {
        "cite"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Blockquote.
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/blockquote).
#[derive(Debug, Clone)]
pub struct Blockquote {
    cite: Option<Cite>,
}

impl Blockquote {
    #[must_use]
    pub const fn new() -> Self {
        Self { cite: None }
    }

    #[must_use]
    pub fn cite(citation: &str) -> Self {
        Self {
            cite: Some(Cite(citation.to_string())),
        }
    }
}

impl Default for Blockquote {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag for Blockquote {
    fn name(&self) -> &'static str {
        "blockquote"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        self.cite.as_ref().map(|cite| vec![cite as &dyn Attribute])
    }
}
