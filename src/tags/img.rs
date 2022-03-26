use super::Tag;

/// Img.
pub struct Img;

impl Tag for Img {
    fn name(&self) -> &'static str {
        "img"
    }
}
