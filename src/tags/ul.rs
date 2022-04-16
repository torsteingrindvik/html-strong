use super::Tag;

/// Ul.
#[derive(Debug, Clone)]
pub struct Ul;

impl Tag for Ul {
    fn name(&self) -> &'static str {
        "ul"
    }
}
