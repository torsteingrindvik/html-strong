use std::fmt::Display;

use anyhow::anyhow;
use axum_extra::extract::cookie::Cookie;
use html_strong::document_tree::Node;

use crate::story::Story;

/// Clone of the original Hacker News frontend.
mod original;

/// A frontend must simply be able to consume a vector of stories and turn that into HTML.
pub trait Renderable {
    /// Render the full frontpage using the provided stories.
    fn frontpage(&self, stories: Vec<Story>) -> Node;

    /// Render the comment page for the given story.
    fn comments(&self, story: Story) -> Node;
}

/// The choice of frontend to use.
/// This choice is intended to be storied in a cookie.
///
/// Defaults to a clone of the original frontend if no choice has been made.
#[derive(Debug)]
pub enum Frontend {
    /// A close-ish clone of the original HackerNews frontend.
    Original,
}

impl Display for Frontend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Frontend::Original => "original",
        };

        write!(f, "{}", name)
    }
}

impl TryFrom<&str> for Frontend {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "original" => Ok(Self::Original),
            name => Err(anyhow!("Not a frontend name: {name}")),
        }
    }
}

impl Frontend {
    pub const COOKIE_NAME: &'static str = "frontend-choice";
}

impl TryFrom<Cookie<'_>> for Frontend {
    type Error = anyhow::Error;

    fn try_from(cookie: Cookie) -> Result<Self, Self::Error> {
        if cookie.name() != Frontend::COOKIE_NAME {
            Err(anyhow!("Wrong cookie name for frontend choice"))
        } else {
            let frontend: Self = cookie.value().try_into()?;
            Ok(frontend)
        }
    }
}

impl Default for Frontend {
    fn default() -> Self {
        Self::Original
    }
}

impl Renderable for Frontend {
    fn frontpage(&self, stories: Vec<Story>) -> Node {
        match self {
            Frontend::Original => original::Original.frontpage(stories),
        }
    }

    fn comments(&self, story: Story) -> Node {
        match self {
            Frontend::Original => original::Original.comments(story),
        }
    }
}
