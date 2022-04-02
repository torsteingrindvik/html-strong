use crate::global_attributes::Attribute;

use super::Tag;

#[derive(Debug, Clone)]
pub enum Charset {
    Utf8,
}

impl Attribute for Charset {
    fn name(&self) -> &'static str {
        "charset"
    }

    fn value(&self) -> String {
        "UTF-8".into()
    }
}

#[derive(Debug, Clone)]
pub enum Name {
    Keywords,
    Description,
    Author,
    Viewport,
    Custom(String),
}

impl Attribute for Name {
    fn name(&self) -> &'static str {
        "name"
    }

    fn value(&self) -> String {
        match self {
            Name::Keywords => "keywords",
            Name::Description => "description",
            Name::Author => "author",
            Name::Viewport => "viewport",
            Name::Custom(custom) => custom,
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub enum HttpEquiv {
    Refresh,
}

impl Attribute for HttpEquiv {
    fn name(&self) -> &'static str {
        "http-equiv"
    }

    fn value(&self) -> String {
        match self {
            HttpEquiv::Refresh => "refresh",
        }
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct Content(String);

impl Content {
    #[must_use]
    pub fn new(content: &str) -> Self {
        Self(content.to_string())
    }
}

impl Attribute for Content {
    fn name(&self) -> &'static str {
        "content"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, Clone)]
pub enum Meta {
    Charset(Charset),
    Name((Name, Content)),
    HttpEquiv((HttpEquiv, Content)),
}

impl Meta {
    #[must_use]
    pub fn name_content(name: &str, content: &str) -> Self {
        Self::Name((Name::Custom(name.to_string()), Content::new(content)))
    }

    #[must_use]
    pub const fn charset_utf8() -> Self {
        Self::Charset(Charset::Utf8)
    }

    #[must_use]
    pub fn author(author: &str) -> Self {
        Self::Name((Name::Author, author.into()))
    }

    #[must_use]
    pub fn description(description: &str) -> Self {
        Self::Name((Name::Description, description.into()))
    }

    #[must_use]
    pub fn refresh(how_often: usize) -> Self {
        Self::HttpEquiv((HttpEquiv::Refresh, how_often.to_string().into()))
    }

    #[must_use]
    pub fn viewport(content: &str) -> Self {
        Self::Name((Name::Viewport, content.into()))
    }

    /// A sane default for the viewport, as instructed by [w3schools](https://www.w3schools.com/TAgs/tag_meta.asp).
    #[must_use]
    pub fn viewport_sane() -> Self {
        Self::viewport("width=device-width, initial-scale=1.0")
    }
}

impl Tag for Meta {
    fn name(&self) -> &'static str {
        "meta"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(match self {
            Meta::Charset(val) => vec![val],
            Meta::Name((name, content)) => vec![name, content],
            Meta::HttpEquiv((http_equiv, content)) => vec![http_equiv, content],
        })
    }

    fn close_tag(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::document_tree::Node;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn render_meta() {
        let result = Node::root()
            .kid(Meta::charset_utf8())
            .kid(Meta::author("John"))
            .kid(Meta::refresh(10))
            .render_string()
            .unwrap();

        let expected = r#"<meta charset="UTF-8"><meta name="author" content="John"><meta http-equiv="refresh" content="10">"#;

        assert_eq!(expected, result);
    }
}
