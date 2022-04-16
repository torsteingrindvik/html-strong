use super::Tag;

/// H1.
#[derive(Debug, Clone)]
pub struct H1;

impl Tag for H1 {
    fn name(&self) -> &'static str {
        "h1"
    }
}
