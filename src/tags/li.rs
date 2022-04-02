use super::Tag;

/// Li.
#[derive(Debug, Clone)]
pub struct Li;

impl Tag for Li {
    fn name(&self) -> &'static str {
        "li"
    }
}
