use html_strong::document_tree::Node;
use serde::{Deserialize, Serialize};

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
#[derive(Debug, Serialize, Deserialize)]
pub enum Frontend {
    /// A close-ish clone of the original HackerNews frontend.
    Original,
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
