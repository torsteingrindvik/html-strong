use super::Tag;

/// Div.
pub struct Div;

impl Tag for Div {
    fn name(&self) -> &'static str {
        "div"
    }
}
