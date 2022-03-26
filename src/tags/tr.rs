use super::Tag;

/// Table row.
pub struct Tr;

impl Tag for Tr {
    fn name(&self) -> &'static str {
        "tr"
    }
}
