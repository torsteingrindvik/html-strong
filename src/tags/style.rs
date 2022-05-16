use super::Tag;

/// Style.
#[derive(Debug, Clone)]
pub struct Style;

impl Tag for Style {
    fn name(&self) -> &'static str {
        "style"
    }
}
