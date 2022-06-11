use crate::global_attributes::Attribute;

use super::{img, Tag};

/// Iframe.
#[derive(Debug, Clone)]
pub struct Iframe {
    src: img::Src,
    width: Option<img::Width>,
    height: Option<img::Height>,
}

impl Iframe {
    #[must_use]
    pub fn new(url: &str) -> Self {
        Self {
            src: img::Src(url.to_string()),
            width: None,
            height: None,
        }
    }

    #[must_use]
    pub fn new_sized(url: &str, width: usize, height: usize) -> Self {
        Self {
            src: img::Src(url.to_string()),
            width: Some(img::Width(width)),
            height: Some(img::Height(height)),
        }
    }
}

impl Tag for Iframe {
    fn name(&self) -> &'static str {
        "iframe"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        let mut attrs: Vec<&dyn Attribute> = vec![&self.src];

        if let Some(height) = &self.height {
            attrs.push(height);
        }

        if let Some(width) = &self.width {
            attrs.push(width);
        }

        Some(attrs)
    }
}
