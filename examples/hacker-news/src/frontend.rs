use std::fmt::Display;

use anyhow::anyhow;
use axum_extra::extract::cookie::Cookie;
use html_strong::document_tree::Node;

use crate::settings;
use crate::story::Story;

/// Clone of the original Hacker News frontend.
mod original;

/// An extremely simple plaintext frontend.
mod plain;

/// Happy colors!
/// See [this](https://codepen.io/ericmahoney/pen/oNLMOYw) codepen,
/// posted by user `ericmahoney`.
/// Design credited to [Simon Lürwer](https://dribbble.com/shots/14101951-Banners).
mod candy;

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
#[derive(Debug, PartialEq, Eq)]
pub enum Frontend {
    /// The original frontend.
    Original,

    /// The plain frontend.
    Plain,

    /// The colorful frontend.
    Candy,
}

impl Frontend {
    pub fn as_options(current_choice: Frontend) -> Vec<settings::Option> {
        vec![
            settings::Option::new(
                "Original",
                "A clone of the original Hacker News frontpage",
                current_choice == Frontend::Original,
            ),
            settings::Option::new(
                "Plain",
                "An extremely simple plaintext version",
                current_choice == Frontend::Plain,
            ),
            settings::Option::new(
                "Candy",
                "A colorful variant",
                current_choice == Frontend::Candy,
            ),
        ]
    }
}

impl Display for Frontend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Frontend::Original => "original",
            Frontend::Plain => "plain",
            Frontend::Candy => "candy",
        };

        write!(f, "{}", name)
    }
}

impl TryFrom<&str> for Frontend {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> Result<Self, Self::Error> {
        match name {
            "original" => Ok(Self::Original),
            "plain" => Ok(Self::Plain),
            "candy" => Ok(Self::Candy),
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
            Frontend::Plain => plain::Plain.frontpage(stories),
            Frontend::Candy => candy::Candy.frontpage(stories),
        }
    }

    fn comments(&self, story: Story) -> Node {
        match self {
            Frontend::Original => original::Original.comments(story),
            Frontend::Plain => plain::Plain.comments(story),
            Frontend::Candy => candy::Candy.comments(story),
        }
    }
}
