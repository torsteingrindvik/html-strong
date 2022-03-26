use super::Tag;

/// Ul.
pub struct Ul;

impl Tag for Ul {
    fn name(&self) -> &'static str {
        "ul"
    }
}
