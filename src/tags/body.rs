use super::Tag;

/// Body.
#[derive(Debug, Clone)]
pub struct Body;

impl Tag for Body {
    fn name(&self) -> &'static str {
        "body"
    }
}
