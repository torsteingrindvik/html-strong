use super::Tag;

pub struct Head;

impl Tag for Head {
    fn name(&self) -> &'static str {
        "head"
    }
}