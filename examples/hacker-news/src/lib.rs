/// The trait needed to be a frontend, and the frontends available.
pub mod frontend;

/// Usage of the Hacker News API.
pub mod hn_api;

/// Constants inserted into HTML.
pub mod constants;

/// Extras.
/// Like moving from a timestamp to the "x hours ago" / "y minutes ago" type text.
pub mod util;

/// Axum server related code.
pub mod server;

/// The state: Holds the most recent stories.
pub mod state;

/// Hacker News story definition.
pub mod story;

/// Hacker News comment definition.
pub mod comment;

/// Keeps the newest stories fresh in the background.
pub mod state_worker;

/// User settings page.
pub mod settings;
