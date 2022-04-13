use std::sync::Arc;
use tokio::sync::RwLock;

/// The Hacker News stories.
/// Updated at some time interval.
#[derive(Debug, Default, Clone)]
pub struct SharedState(pub Arc<RwLock<Vec<crate::story::Story>>>);
