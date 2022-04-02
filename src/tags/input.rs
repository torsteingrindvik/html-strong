use super::Tag;

/// Input.
#[derive(Debug, Clone)]
pub struct Input;

impl Tag for Input {
    fn name(&self) -> &'static str {
        "input"
    }
}
