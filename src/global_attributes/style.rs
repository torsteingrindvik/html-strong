use super::Attribute;

#[derive(Debug, Clone)]
pub struct Style(String);

impl Style {
    #[must_use]
    pub fn new(style: &str) -> Self {
        Self(style.to_string())
    }
}

impl Attribute for Style {
    fn name(&self) -> &'static str {
        "style"
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}
