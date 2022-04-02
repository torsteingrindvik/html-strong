use super::Tag;

/// Table row.
#[derive(Debug, Clone)]
pub struct Tr;

impl Tag for Tr {
    fn name(&self) -> &'static str {
        "tr"
    }
}
