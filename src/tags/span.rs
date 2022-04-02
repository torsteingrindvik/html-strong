use super::Tag;

/// Span.
#[derive(Debug, Clone)]
pub struct Span;

impl Tag for Span {
    fn name(&self) -> &'static str {
        "span"
    }
}
