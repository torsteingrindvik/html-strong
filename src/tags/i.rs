use super::Tag;

/// i tag.
/// See [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/i).
///
/// Quote:
///     > The <i> HTML element represents a range of text that is set off from
///     > the normal text for some reason, such as idiomatic text, technical terms,
///     > taxonomical designations, among others. Historically, these have been
///     > presented using italicized type, which is the original source of the <i>,
///     > naming of this element.
#[derive(Debug, Clone)]
pub struct I;

impl Tag for I {
    fn name(&self) -> &'static str {
        "i"
    }
}
