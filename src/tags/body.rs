use super::Tag;

/// Body.
pub struct Body;

impl Tag for Body {
    fn name(&self) -> &'static str {
        "body"
    }
}
