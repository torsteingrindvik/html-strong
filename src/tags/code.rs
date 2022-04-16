use super::Tag;

/// The code element.
/// See [MDN](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/code).
#[derive(Debug, Clone)]
pub struct Code;

impl Tag for Code {
    fn name(&self) -> &'static str {
        "code"
    }
}
