use super::Tag;

/// em.
pub struct Em;

impl Tag for Em {
    fn name(&self) -> &'static str {
        "em"
    }
}
