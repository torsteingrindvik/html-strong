use super::Tag;

/// Table.
#[derive(Debug, Clone)]
pub struct Table;

impl Tag for Table {
    fn name(&self) -> &'static str {
        "table"
    }
}
