use super::Tag;

/// The preformatted text element.
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/pre).
#[derive(Debug, Clone)]
pub struct Pre;

impl Tag for Pre {
    fn name(&self) -> &'static str {
        "pre"
    }
}
