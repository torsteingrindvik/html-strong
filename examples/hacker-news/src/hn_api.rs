use anyhow::Result;
use cached::proc_macro::cached;
use chrono::{TimeZone, Utc};
use futures::{stream::FuturesOrdered, FutureExt, StreamExt, TryFutureExt};
use reqwest::Url;
use serde::{de::DeserializeOwned, Deserialize};
use tracing::debug;

use crate::comment::Comment;
use crate::story::Story;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ApiStory {
    /// Author
    pub by: String,

    /// Number of comments
    pub descendants: usize,

    /// Story id
    pub id: usize,

    /// Story text, if any
    pub text: Option<String>,

    /// Comments
    pub kids: Option<Vec<usize>>,

    /// Upvotes
    pub score: usize,

    /// Unix time
    pub time: usize,

    /// Story title
    pub title: String,

    /// Story url.
    pub url: Option<Url>,
}

impl ApiStory {
    pub async fn try_into_story(self, rank: usize) -> Result<Story> {
        let comments: Vec<Comment> = resolve_comments(&self)
            .map(|api_comments| api_comments.into_iter().map(Into::into).collect())
            .await;

        Ok(Story {
            author: self.by,
            id: self.id,
            text: self.text.unwrap_or_default(),
            comments,
            upvotes: self.score,
            submission_time: Utc.timestamp(self.time as i64, 0).into(),
            title: self.title,
            url: self.url,
            rank,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub struct ApiComment {
    /// Author
    pub by: String,

    /// Comment id
    pub id: usize,

    /// Comment replies, if any
    pub kids: Option<Vec<usize>>,

    /// Parent comment, or story if "root comment"
    pub parent: usize,

    /// Comment contents, can be HTML.
    /// Must be sanitized in the HN backend, right?
    pub text: String,

    /// Unix time
    pub time: usize,
}

pub async fn story_ids() -> Result<Vec<usize>> {
    Ok(
        reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .await?
            .json()
            .await?,
    )
}

async fn item<T: DeserializeOwned>(id: usize) -> Result<T> {
    let story_raw = reqwest::get(format!(
        "https://hacker-news.firebaseio.com/v0/item/{id}.json"
    ))
    .await?;

    Ok(story_raw.json().await?)
}

#[cached(time = 600, result = true)]
pub async fn story(id: usize) -> Result<ApiStory> {
    item(id).await
}

#[cached(size = 10000, result = true)]
pub async fn comment(id: usize) -> Result<ApiComment> {
    item(id).await
}

async fn resolve_comments(story: &ApiStory) -> Vec<ApiComment> {
    let mut to_be_resolved = FuturesOrdered::new();

    let kids = story.kids.to_owned().unwrap_or_default();

    for comment_id in kids {
        // TODO: Instead of getting only these, recurse.
        // TODO: Add an indent level++ for each level of recursion.
        to_be_resolved.push(
            comment(comment_id)
                .inspect_err(move |e| debug!(?e, %comment_id, "Problem resolving comment")),
        );
    }

    to_be_resolved
        .filter_map(|api_comment_result| async move { api_comment_result.ok() })
        .collect()
        .await
}
