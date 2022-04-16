use super::Tag;

/// H2.
#[derive(Debug, Clone)]
pub struct H2;

impl Tag for H2 {
    fn name(&self) -> &'static str {
        "h2"
    }
}
