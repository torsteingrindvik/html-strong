use super::Tag;

/// Root node.
#[derive(Debug, Clone)]
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
