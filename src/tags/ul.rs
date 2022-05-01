use super::Tag;

/// The unordered list element, Ul. See [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/ul).
#[derive(Debug, Clone)]
pub struct Ul;

impl Tag for Ul {
    fn name(&self) -> &'static str {
        "ul"
    }
}
