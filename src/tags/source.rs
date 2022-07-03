use crate::global_attributes::Attribute;

use super::{img::Src, Tag};

#[derive(Debug, Clone)]
enum Type_ {
    Webm,
}

impl Attribute for Type_ {
    fn name(&self) -> &'static str {
        "type"
    }

    fn value(&self) -> String {
        match self {
            Type_::Webm => "video/webm".to_string(),
        }
    }
}

/// Source.
/// E.g. for use with the [`super::video::Video`] element.
/// See [https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video].
#[derive(Debug, Clone)]
pub struct Source {
    type_: Type_,
    src: Src,
}

impl Source {
    pub fn new_webm<S: AsRef<str>>(source: S) -> Self {
        Self {
            type_: Type_::Webm,
            src: Src(source.as_ref().to_string()),
        }
    }
}

impl Tag for Source {
    fn name(&self) -> &'static str {
        "source"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(vec![&self.src, &self.type_])
    }
}
