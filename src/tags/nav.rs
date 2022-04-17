use super::Tag;

/// nav tag.
/// See [here](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/nav).
#[derive(Debug, Clone)]
pub struct Nav;

impl Tag for Nav {
    fn name(&self) -> &'static str {
        "nav"
    }
}
