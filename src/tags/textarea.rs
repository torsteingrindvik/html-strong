use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
struct Name(String);

impl Attribute for Name {
    fn name(&self) -> &'static str {
        "name"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
struct Rows(usize);

impl Attribute for Rows {
    fn name(&self) -> &'static str {
        "rows"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

#[derive(Debug, Clone)]
struct Cols(usize);

impl Attribute for Cols {
    fn name(&self) -> &'static str {
        "cols"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Textare tag.
#[derive(Debug, Clone)]
pub struct Textarea {
    name: Name,
    rows: Rows,
    cols: Cols,
}

impl Textarea {
    #[must_use]
    pub fn new(name: &str, rows: usize, cols: usize) -> Self {
        Self {
            name: Name(name.to_string()),
            rows: Rows(rows),
            cols: Cols(cols),
        }
    }
}

impl Tag for Textarea {
    fn name(&self) -> &'static str {
        "textarea"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(vec![&self.name, &self.rows, &self.cols])
    }
}
