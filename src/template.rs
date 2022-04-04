use crate::document_tree::{o, Node};
use crate::tags::{Body, Doctype, Head, Html, Meta};

/// Sets up a default HTML document,
/// with user settable header and body.
pub struct HtmlDocumentBuilder {
    head: Option<Node>,
    body: Option<Node>,
}

/// Creates a [Node] based on the [Head] tag.
/// Default contents inspired by [w3schools](https://www.w3schools.com/TAgs/tag_meta.asp).
#[must_use]
pub fn head() -> Node {
    o(Head)
        .kid(Meta::charset_utf8())
        .kid(Meta::viewport_sane())
        .kid(Meta::refresh(5))
}

/// Builds a simple HTML document,
/// following the W3C template [here](https://www.w3.org/QA/2002/04/valid-dtd-list.html).
///
/// If head is not set, uses the template default (without the title).
/// If body is not set, uses an empty body.
impl HtmlDocumentBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: None,
            body: None,
        }
    }

    #[must_use]
    pub fn with_head(mut self, head: Node) -> Self {
        self.head = Some(head);
        self
    }

    #[must_use]
    pub fn with_body(mut self, body: Node) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self) -> Node {
        let head = self.head.unwrap_or_else(head);
        let body = self.body.unwrap_or_else(|| o(Body));

        Node::root()
            .kid(o(Doctype))
            .kid(o(Html).kid(head).kid(body))
    }
}

impl Default for HtmlDocumentBuilder {
    fn default() -> Self {
        Self::new()
    }
}
