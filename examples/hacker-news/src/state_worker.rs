use crate::{hn_api, state::SharedState, story::Story};

use anyhow::Result;
use futures::TryFutureExt;
use futures::{future, stream::FuturesOrdered, FutureExt, StreamExt};
use std::env;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

fn read_from_disk() -> Result<Vec<Story>> {
    debug!("Reading stories from disk");
    let reader = std::fs::File::open("stories.json")?;
    Ok(serde_json::from_reader(reader)?)
}

fn store_to_disk(stories: &[Story]) -> Result<()> {
    debug!("Saving stories to disk");
    let writer = std::fs::File::create("stories.json")?;
    Ok(serde_json::to_writer_pretty(writer, stories)?)
}

async fn get_stories() -> Result<Vec<Story>> {
    if env::var_os("LOAD_STORIES").is_some() {
        if let Ok(stories) = read_from_disk() {
            info!(
                "Skipping network fetch- {} stories successfully read from disk",
                stories.len()
            );
            return Ok(stories);
        }
        // If we fail to read from disk, do a normal network fetch.
        // Assume it fails because it did not exist.
    }

    let mut story_ids = hn_api::story_ids().await?;
    info!("Pulled {} story ids", story_ids.len());
    story_ids.truncate(30);

    let mut futures = FuturesOrdered::new();

    for (rank, id) in story_ids.iter().enumerate() {
        futures.push(
            hn_api::story(*id)
                .and_then(move |api_story| api_story.try_into_story(rank + 1)) // Want rank to be 1-indexed
                .map(move |story| (story, id)),
        );
    }

    let now = Instant::now();

    let stories = futures
        .inspect(|(fut, id)| {
            if let Err(e) = fut {
                warn!("Problem (id: {id}) getting story: {e:?}");
            }
        })
        .filter_map(|(fut, _)| future::ready(fut.ok()))
        .inspect(|story| debug!("Id: {} -> {}", story.id, story.title))
        .collect::<Vec<_>>()
        .await;

    info!("Stories resolved in {:?}", now.elapsed());

    if env::var_os("SAVE_STORIES").is_some() && store_to_disk(&stories).is_err() {
        warn!("Could not serialize stories")
    }

    Ok(stories)
}

pub async fn worker(state: SharedState) {
    info!("Story cache worker started");

    loop {
        let stories = match get_stories().await {
            Ok(stories) => stories,
            Err(e) => {
                warn!("Couldn't fetch stories: {e:?}");
                continue;
            }
        };

        // Go from [Story] to [(usize, Story)], to create a mapping.
        // let stories = stories.into_iter().map(|story| (story.id, story)).collect();

        {
            let now = Instant::now();
            let mut state = state.0.write().await;
            *state = stories;
            info!("New story added (held write lock for {:?})", now.elapsed());
        }

        // We don't really need to get new Hacker News stories
        // faster than every 5 minutes.
        tokio::time::sleep(Duration::from_secs(60 * 5)).await;
    }
}
