use super::Tag;

/// Script.
#[derive(Debug, Clone)]
pub struct Script;

impl Tag for Script {
    fn name(&self) -> &'static str {
        "script"
    }
}
