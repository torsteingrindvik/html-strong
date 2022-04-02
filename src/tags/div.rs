use super::Tag;

/// Div.
#[derive(Debug, Clone)]
pub struct Div;

impl Tag for Div {
    fn name(&self) -> &'static str {
        "div"
    }
}
