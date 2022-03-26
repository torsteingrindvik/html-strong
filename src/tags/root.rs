use super::Tag;

/// Root node.
pub struct Root;

impl Tag for Root {
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
