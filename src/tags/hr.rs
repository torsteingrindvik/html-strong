use super::Tag;

/// Hr.
#[derive(Debug, Clone)]
pub struct Hr;

impl Tag for Hr {
    fn name(&self) -> &'static str {
        "hr"
    }

    fn close_tag(&self) -> bool {
        false
    }
}
