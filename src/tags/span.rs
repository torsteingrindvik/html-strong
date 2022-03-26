use super::Tag;

/// Span.
pub struct Span;

impl Tag for Span {
    fn name(&self) -> &'static str {
        "span"
    }
}
