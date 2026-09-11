//! Command system module.
//!
//! Separates actions and business logic from UI widgets. Widgets reference
//! `CommandId`, and actions are resolved centrally through the registry.

pub mod id;
pub mod registry;
pub mod shortcut;

pub use id::CommandId;
pub use registry::{Command, CommandRegistry};
pub use shortcut::{Shortcut, ShortcutKey};
