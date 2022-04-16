use super::Tag;

/// Br.
#[derive(Debug, Clone)]
pub struct Br;

impl Tag for Br {
    fn name(&self) -> &'static str {
        "br"
    }
    fn close_tag(&self) -> bool {
        false
    }
}
