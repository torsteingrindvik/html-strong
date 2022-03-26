use super::Attribute;

#[derive(Debug)]
pub struct Class(String);

impl Class {
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self(value.into())
    }
}

impl Attribute for Class {
    fn name(&self) -> &'static str {
        "class"
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}
