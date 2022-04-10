use super::Tag;

/// U. See [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/u).
#[derive(Debug, Clone)]
pub struct U;

impl Tag for U {
    fn name(&self) -> &'static str {
        "u"
    }
}
