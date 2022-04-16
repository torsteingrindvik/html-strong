use super::Tag;

/// em.
#[derive(Debug, Clone)]
pub struct Em;

impl Tag for Em {
    fn name(&self) -> &'static str {
        "em"
    }
}
