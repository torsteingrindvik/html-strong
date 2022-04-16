use super::Attribute;

#[derive(Debug, Clone)]
pub struct Id(String);

impl Id {
    #[must_use]
    pub fn new(value: &str) -> Self {
        Self(value.into())
    }
}

impl Attribute for Id {
    fn name(&self) -> &'static str {
        "id"
    }

    fn value(&self) -> String {
        self.0.clone()
    }
}
