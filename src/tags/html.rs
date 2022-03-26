use super::Tag;

pub struct Html;

impl Tag for Html {
    fn name(&self) -> &'static str {
        "html"
    }
}
