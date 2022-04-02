use crate::global_attributes::Attribute;

use super::Tag;

/// Source image.
struct Src(String);

impl Attribute for Src {
    fn name(&self) -> &'static str {
        "src"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Image width.
struct Width(usize);

impl Attribute for Width {
    fn name(&self) -> &'static str {
        "width"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Image height.
struct Height(usize);

impl Attribute for Height {
    fn name(&self) -> &'static str {
        "height"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Img.
pub struct Img {
    src: Src,
    width: Option<Width>,
    height: Option<Height>,
}

impl Img {
    #[must_use]
    pub fn new(url: &str) -> Self {
        Self {
            src: Src(url.to_string()),
            width: None,
            height: None,
        }
    }

    #[must_use]
    pub fn new_sized(url: &str, width: usize, height: usize) -> Self {
        Self {
            src: Src(url.to_string()),
            width: Some(Width(width)),
            height: Some(Height(height)),
        }
    }
}

impl Tag for Img {
    fn name(&self) -> &'static str {
        "img"
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
