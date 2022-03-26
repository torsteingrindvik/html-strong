use super::Tag;

pub struct Doctype;

impl Tag for Doctype {
    fn name(&self) -> &'static str {
        "!DOCTYPE html"
    }

    fn close_tag(&self) -> bool {
        false
    }
}