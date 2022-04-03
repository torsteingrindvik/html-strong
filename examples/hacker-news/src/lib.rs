/// Usage of the Hacker News API.
pub mod hn_api;

/// Constants inserted into HTML.
pub mod constants;

/// The main bulk of the Hacker News HTML content.
pub mod html;

/// Axum server related code.
pub mod server;

/// The state: Holds the most recent stories.
pub mod state;

/// Hacker News story definition.
pub mod story;

/// Keeps the newest stories fresh in the background.
pub mod state_worker;