use super::Tag;

#[derive(Debug, Clone)]
pub struct Html;

impl Tag for Html {
    fn name(&self) -> &'static str {
        "html"
    }
}
