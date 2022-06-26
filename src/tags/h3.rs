use super::Tag;

/// H3.
#[derive(Debug, Clone)]
pub struct H3;

impl Tag for H3 {
    fn name(&self) -> &'static str {
        "h3"
    }
}
