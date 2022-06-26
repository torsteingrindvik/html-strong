use super::Tag;

/// Main.
#[derive(Debug, Clone)]
pub struct Main;

impl Tag for Main {
    fn name(&self) -> &'static str {
        "main"
    }
}
