//! Root application coordination layer.
//!
//! Orchestrates the top-level application state, message routing, update cycles,
//! and view construction between `shell/` (reusable GUI shell) and `demo/` (domain).

pub mod message;
pub mod state;
pub mod update;
pub mod view;

pub use message::Message;
pub use state::AppState;
pub use update::update;
pub use view::view;
