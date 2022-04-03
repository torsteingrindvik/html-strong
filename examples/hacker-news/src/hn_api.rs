use crate::story::Story;
use anyhow::Result;
use tracing::trace;

pub async fn story_ids() -> Result<Vec<usize>> {
    Ok(
        reqwest::get("https://hacker-news.firebaseio.com/v0/topstories.json")
            .await?
            .json()
            .await?,
    )
}

pub async fn story(id: usize) -> Result<Story> {
    let story_raw = reqwest::get(format!(
        "https://hacker-news.firebaseio.com/v0/item/{id}.json"
    ))
    .await?;
    trace!("Story raw: {story_raw:#?}");

    Ok(story_raw.json().await?)
}
