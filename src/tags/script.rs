use super::Tag;

/// Script.
pub struct Script;

impl Tag for Script {
    fn name(&self) -> &'static str {
        "script"
    }
}
