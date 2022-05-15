use super::Tag;

/// Footer element.
#[derive(Debug, Clone)]
pub struct Footer;

impl Tag for Footer {
    fn name(&self) -> &'static str {
        "footer"
    }
}
