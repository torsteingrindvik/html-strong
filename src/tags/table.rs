use super::Tag;

/// Table.
pub struct Table;

impl Tag for Table {
    fn name(&self) -> &'static str {
        "table"
    }
}
