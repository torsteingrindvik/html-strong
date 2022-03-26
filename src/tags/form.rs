use super::Tag;

/// Data form.
pub struct Form;

impl Tag for Form {
    fn name(&self) -> &'static str {
        "form"
    }
}
