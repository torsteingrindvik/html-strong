use crate::global_attributes::Attribute;

use super::{link::Href, Tag};

pub enum Target {
    Blank,
    Parent,
    Self_,
    Top,
}

// TODO: Verify these
impl Attribute for Target {
    fn name(&self) -> &'static str {
        "target"
    }

    fn value(&self) -> String {
        match self {
            Target::Blank => "blank",
            Target::Parent => "parent",
            Target::Self_ => "self",
            Target::Top => "top",
        }
        .into()
    }
}

pub struct A {
    href: Option<Href>,
    target: Option<Target>,
}

impl A {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            href: None,
            target: None,
        }
    }

    #[must_use]
    pub fn href(href: &str) -> Self {
        Self::new().with_href(href)
    }

    #[must_use]
    pub fn with_href(mut self, href: &str) -> Self {
        self.href = Some(Href::new(href));
        self
    }

    #[must_use]
    pub const fn with_target(mut self, target: Target) -> Self {
        self.target = Some(target);
        self
    }
}

impl Default for A {
    fn default() -> Self {
        Self::new()
    }
}

impl Tag for A {
    fn name(&self) -> &'static str {
        "a"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        match (&self.href, &self.target) {
            (None, None) => None,
            (None, Some(t)) => Some(vec![t]),
            (Some(h), None) => Some(vec![h]),
            (Some(h), Some(t)) => Some(vec![h, t]),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        document_tree::o,
        tags::{Span, Td, B},
    };
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn render_visit_w3schools() {
        let link = o(A::href("https://www.w3schools.com")).add_text("Visit W3Schools.com!");

        let expected = r#"<a href="https://www.w3schools.com">Visit W3Schools.com!</a>"#;
        let result = link.render_string().unwrap();

        assert_eq!(expected, result);
    }

    #[test]
    fn render_a_hackernews_header() {
        let a = |href, text| o(A::href(href)).add_text(text);
        let a2 = |href| o(A::href(href)).add_text(href);
        let spacer = " | ";

        let td_links = o(Td).add_style("line-height:12pt; height:10px;").kid(
            o(Span)
                .add_class("pagetop")
                .kid(o(B).add_class("hnname").kid(a("news", "Hacker News")))
                .kid(a("newest", "new"))
                .add_text(spacer)
                .kid(a("newcomments", "comments"))
                .add_text(spacer)
                .kid(a2("ask"))
                .add_text(spacer)
                .kid(a2("show"))
                .add_text(spacer)
                .kid(a2("jobs"))
                .add_text(spacer)
                .kid(a2("submit")),
        );
        let expected = r#"<a href="https://www.w3schools.com">Visit W3Schools.com!</a>"#;
        let result = td_links.render_string().unwrap();

        assert_eq!(expected, result);
    }
}
