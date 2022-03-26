use super::Tag;

/// Center.
#[deprecated(note = "Use CSS instead: https://www.w3schools.com/tags/tag_center.asp")]
pub struct Center;

impl Tag for Center {
    fn name(&self) -> &'static str {
        "center"
    }
}
