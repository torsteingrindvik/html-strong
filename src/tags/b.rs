use super::Tag;

/// b tag.
/// See [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/b).
///
/// Quote:
///     > The <b> HTML element is used to draw the reader's attention to the element's contents, which are not otherwise granted special importance.
pub struct B;

impl Tag for B {
    fn name(&self) -> &'static str {
        "b"
    }
}
