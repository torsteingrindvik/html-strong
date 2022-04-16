use super::Tag;

#[derive(Debug, Clone)]
pub struct Head;

impl Tag for Head {
    fn name(&self) -> &'static str {
        "head"
    }
}
