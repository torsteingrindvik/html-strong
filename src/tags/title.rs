use super::Tag;

/// Title tag.
pub struct Title;

impl Tag for Title {
    fn name(&self) -> &'static str {
        "title"
    }
}
