use crate::global_attributes::Attribute;

use super::Tag;

/// Mime
#[derive(Debug, Clone)]
pub struct Title(String);

impl Attribute for Title {
    fn name(&self) -> &'static str {
        "title"
    }

    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Mime
#[derive(Debug, Clone)]
pub enum Rel {
    Stylesheet,
    Icon,
    Alternate,
}

impl Attribute for Rel {
    fn name(&self) -> &'static str {
        "rel"
    }

    fn value(&self) -> String {
        match self {
            Rel::Stylesheet => "stylesheet",
            Rel::Icon => "icon",
            Rel::Alternate => "alternate",
        }
        .into()
    }
}

/// Mime
#[derive(Debug, Clone)]
pub struct Mime(String);

impl Mime {
    #[must_use]
    pub fn new(type_: &str) -> Self {
        Self(type_.to_string())
    }
}

impl From<mime::Mime> for Mime {
    fn from(val: mime::Mime) -> Self {
        Self(val.to_string())
    }
}

impl From<&str> for Mime {
    fn from(val: &str) -> Self {
        Self(val.to_string())
    }
}

impl Attribute for Mime {
    fn name(&self) -> &'static str {
        "type"
    }

    // TODO: Let's see how far this gets us
    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Href
#[derive(Debug, Clone)]
pub struct Href(String);

impl From<url::Url> for Mime {
    fn from(val: url::Url) -> Self {
        Self(val.to_string())
    }
}

impl Href {
    #[must_use]
    pub fn new(url: &str) -> Self {
        Self(url.to_string())
    }
}

impl Attribute for Href {
    fn name(&self) -> &'static str {
        "href"
    }

    // TODO: Let's see how far this gets us
    fn value(&self) -> String {
        self.0.to_string()
    }
}

/// Link.
/// See [reference](https://developer.mozilla.org/en-US/docs/Web/HTML/Link_types).
#[derive(Debug, Clone)]
pub enum Link {
    Stylesheet {
        rel: Rel,
        mime: Mime,
        href: Href,
    },
    Icon {
        rel: Rel,
        href: Href,
    },
    Alternate {
        rel: Rel,
        mime: Mime,
        title: Title,
        href: Href,
    },
}

impl Link {
    pub fn stylesheet<M>(mime: M, url: &str) -> Self
    where
        M: Into<Mime>,
    {
        Self::Stylesheet {
            rel: Rel::Stylesheet,
            mime: mime.into(),
            href: Href::new(url),
        }
    }

    #[must_use]
    pub fn icon(url: &str) -> Self {
        Self::Icon {
            rel: Rel::Icon,
            href: Href::new(url),
        }
    }

    pub fn alternate<M>(mime: M, title: &str, url: &str) -> Self
    where
        M: Into<Mime>,
    {
        Self::Alternate {
            rel: Rel::Alternate,
            mime: mime.into(),
            title: Title(title.into()),
            href: Href::new(url),
        }
    }
}

impl Tag for Link {
    fn name(&self) -> &'static str {
        "link"
    }

    fn attributes(&self) -> Option<Vec<&dyn Attribute>> {
        Some(match &self {
            Link::Stylesheet { rel, mime, href } => vec![rel, mime, href],
            Link::Icon { rel, href } => vec![rel, href],
            Link::Alternate {
                rel,
                mime,
                title,
                href,
            } => vec![rel, mime, title, href],
        })
    }

    fn close_tag(&self) -> bool {
        false
    }
}
// https://news.ycombinator.com/news.css?U4Pc202vc5MEd4M0yfRK

#[cfg(test)]
mod tests {
    use crate::document_tree::o;
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn render_stylesheet() {
        let result = o(Link::stylesheet(
            mime::TEXT_CSS,
            "news.css?QZejSKY7mNWXnObVdaSN",
        ))
        .render_string()
        .unwrap();

        let expected =
            r#"<link rel="stylesheet" type="text/css" href="news.css?QZejSKY7mNWXnObVdaSN">"#;
        assert_eq!(expected, result);
    }

    #[test]
    fn render_icon() {
        let result = o(Link::icon("favicon.ico")).render_string().unwrap();

        let expected = r#"<link rel="icon" href="favicon.ico">"#;
        assert_eq!(expected, result);
    }

    #[test]
    fn render_alternate() {
        let result = o(Link::alternate(
            Mime::new("application/rss+xml"),
            "RSS",
            "rss",
        ))
        .render_string()
        .unwrap();

        let expected =
            r#"<link rel="alternate" type="application/rss+xml" title="RSS" href="rss">"#;
        assert_eq!(expected, result);
    }
}
