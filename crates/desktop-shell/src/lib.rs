//! Reusable desktop shell infrastructure (`desktop-shell` crate).
//!
//! Provides the complete technical desktop UI architecture:
//! - Menu and data-driven Ribbon
//! - Command Registry and keyboard shortcuts
//! - Dock Layout and collapsible Panels
//! - Design tokens and Dark/Light theming
//! - Status bar and Preferences persistence

pub mod activity_bar;
pub mod bottom_panel;
pub mod command;
pub mod dock;
pub mod inspector;
pub mod layout;
pub mod menu;
pub mod message;
pub mod panel;
pub mod persistence;
pub mod ribbon;
pub mod state;
pub mod status_bar;
pub mod theme;
pub mod toolbar;
pub mod workspace;

pub use activity_bar::ActivityItem;
pub use command::{Command, CommandId, CommandRegistry, Shortcut};
pub use dock::DockLayout;
pub use layout::render_shell;
pub use message::ShellMessage;
pub use panel::{PanelId, PanelLocation, PanelState};
pub use persistence::ShellPreferences;
pub use state::ShellState;
pub use status_bar::StatusBarState;
pub use theme::ThemeMode;
pub use toolbar::ToolbarItem;
