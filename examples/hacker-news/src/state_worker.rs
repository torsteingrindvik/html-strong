use crate::{hn_api, state::SharedState, story::Story};

use anyhow::Result;
use futures::TryFutureExt;
use futures::{future, stream::FuturesOrdered, FutureExt, StreamExt};
use std::time::{Duration, Instant};
use tracing::{info, warn};

async fn get_stories() -> Result<Vec<Story>> {
    let mut story_ids = hn_api::story_ids().await?;
    info!("Pulled {} story ids", story_ids.len());
    story_ids.truncate(50);

    let mut futures = FuturesOrdered::new();

    for id in story_ids {
        futures.push(
            hn_api::story(id)
                .and_then(|api_story| api_story.try_into_story())
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
        .collect::<Vec<_>>()
        .await;

    info!("Stories resolved in {:?}", now.elapsed());

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
        let stories = stories.into_iter().map(|story| (story.id, story)).collect();

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
