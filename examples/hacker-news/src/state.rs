use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// A mapping between story ID and a story.
#[derive(Debug, Default, Clone)]
pub struct SharedState(pub Arc<RwLock<BTreeMap<usize, crate::story::Story>>>);
