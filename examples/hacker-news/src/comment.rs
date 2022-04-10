use chrono::{DateTime, Local, TimeZone, Utc};
use serde::Deserialize;

use crate::hn_api::ApiComment;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct Comment {
    /// Author
    pub author: String,

    /// Comment id
    pub id: usize,

    /// Parent comment, or story if "root comment"
    pub parent: usize,

    /// Comment contents, can be HTML.
    /// Must be sanitized in the HN backend, right?
    pub text: String,

    /// Submission time
    pub time: DateTime<Local>,
}

impl From<ApiComment> for Comment {
    fn from(api_comment: ApiComment) -> Self {
        Self {
            author: api_comment.by,
            parent: api_comment.parent,
            text: api_comment.text,
            time: Utc.timestamp(api_comment.time as i64, 0).into(),
            id: api_comment.id,
        }
    }
}
