use super::Tag;

/// Li.
pub struct Li;

impl Tag for Li {
    fn name(&self) -> &'static str {
        "li"
    }
}
