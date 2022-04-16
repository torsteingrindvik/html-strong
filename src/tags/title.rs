use super::Tag;

/// Title tag.
#[derive(Debug, Clone)]
pub struct Title;

impl Tag for Title {
    fn name(&self) -> &'static str {
        "title"
    }
}
