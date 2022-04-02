use super::Tag;

/// Tagless node.
#[derive(Debug, Clone)]
pub struct Invisible;

impl Tag for Invisible {
    fn name(&self) -> &'static str {
        unreachable!()
    }

    fn open_tag(&self) -> bool {
        false
    }
    fn close_tag(&self) -> bool {
        false
    }
}
