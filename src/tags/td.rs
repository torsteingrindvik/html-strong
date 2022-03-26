use super::Tag;

/// Table data cell.
pub struct Td;

impl Tag for Td {
    fn name(&self) -> &'static str {
        "td"
    }
}
