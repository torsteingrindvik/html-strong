use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default, Clone)]
pub struct SharedState(pub Arc<RwLock<Vec<crate::story::Story>>>);
