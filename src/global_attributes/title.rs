use super::Attribute;

#[derive(Debug)]
pub struct Title(String);

impl Title {
    #[must_use]
    pub fn new(title: &str) -> Self {
        Self(title.to_string())
    }
}

impl Attribute for Title {
    fn name(&self) -> &'static str {
        "title"
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}
