use super::Tag;

/// Th.
#[derive(Debug, Clone)]
pub struct Th;

impl Tag for Th {
    fn name(&self) -> &'static str {
        "th"
    }
}
