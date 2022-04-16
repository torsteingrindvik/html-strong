use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::comment::Comment;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Story {
    /// Author
    pub author: String,

    /// Story id
    pub id: usize,

    /// Story text, if any
    pub text: String,

    /// Comments
    pub comments: Vec<Comment>,

    /// Upvotes
    pub upvotes: usize,

    /// When it was submitted
    pub submission_time: DateTime<Local>,

    /// Story title
    pub title: String,

    /// Story url, or nothing if there was no external story.
    pub url: Option<Url>,

    /// Which position on the site (the number besides the upvote arrow)
    pub rank: usize,
}
