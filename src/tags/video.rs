use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
pub struct Loop;

impl Attribute for Loop {
    fn name(&self) -> &'static str {
        "loop"
    }

    fn value(&self) -> String {
        "".to_string()
    }
}

#[derive(Debug, Clone)]
pub struct Controls;

impl Attribute for Controls {
    fn name(&self) -> &'static str {
        "controls"
    }

    fn value(&self) -> String {
        "".to_string()
    }
}

/// Video.
/// See [https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video].
#[derive(Debug, Clone)]
pub struct Video {
    loop_: Option<Loop>,
    controls: Option<Controls>,
}

impl Video {
    pub fn new() -> Self {
        Self {
            loop_: None,
            controls: None,
        }
    }

    pub fn loop_(mut self) -> Self {
        self.loop_ = Some(Loop);
        self
    }

    pub fn controls(mut self) -> Self {
        self.controls = Some(Controls);
        self
    }
}

impl Default for Video {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag for Video {
    fn name(&self) -> &'static str {
        "video"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        let mut attrs: Vec<&dyn Attribute> = vec![];

        if let Some(loop_) = &self.loop_ {
            attrs.push(loop_);
        }

        if let Some(controls) = &self.controls {
            attrs.push(controls);
        }

        if attrs.is_empty() {
            None
        } else {
            Some(attrs)
        }
    }
}
