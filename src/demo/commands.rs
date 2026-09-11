//! Application-level commands and registration for the demo.
//!
//! Separated completely from `shell/`.

use desktop_shell::command::{Command, CommandRegistry, Shortcut};

pub const APP_NEW: &str = "app.new";
pub const APP_OPEN: &str = "app.open";
pub const APP_SAVE: &str = "app.save";
pub const APP_QUIT: &str = "app.quit";

pub const VIEW_TOGGLE_EXPLORER: &str = "view.toggle_explorer";
pub const VIEW_TOGGLE_INSPECTOR: &str = "view.toggle_inspector";
pub const VIEW_TOGGLE_BOTTOM_PANEL: &str = "view.toggle_bottom_panel";
pub const VIEW_TOGGLE_THEME: &str = "view.toggle_theme";

pub const HELP_ABOUT: &str = "help.about";

/// Registers all demo/application-level commands into the shell registry.
pub fn register_demo_commands(registry: &mut CommandRegistry) {
    registry.register(
        Command::new(APP_NEW, "New", "Create a new project")
            .with_shortcut(Shortcut::ctrl('n'))
            .with_icon("📄"),
    );
    registry.register(
        Command::new(APP_OPEN, "Open...", "Open an existing project")
            .with_shortcut(Shortcut::ctrl('o'))
            .with_icon("📂"),
    );
    registry.register(
        Command::new(APP_SAVE, "Save", "Save current changes")
            .with_shortcut(Shortcut::ctrl('s'))
            .with_icon("💾"),
    );
    registry.register(
        Command::new(
            VIEW_TOGGLE_EXPLORER,
            "Explorer",
            "Toggle Project Explorer visibility",
        )
        .with_shortcut(Shortcut::ctrl('b'))
        .with_icon("📁"),
    );
    registry.register(
        Command::new(
            VIEW_TOGGLE_INSPECTOR,
            "Inspector",
            "Toggle Properties Inspector visibility",
        )
        .with_shortcut(Shortcut::ctrl('i'))
        .with_icon("🔍"),
    );
    registry.register(
        Command::new(
            VIEW_TOGGLE_BOTTOM_PANEL,
            "Output Panel",
            "Toggle Bottom Output panel visibility",
        )
        .with_shortcut(Shortcut::ctrl('j'))
        .with_icon("🖥️"),
    );
    registry.register(
        Command::new(
            VIEW_TOGGLE_THEME,
            "Toggle Theme",
            "Switch between Dark and Light mode",
        )
        .with_shortcut(Shortcut::ctrl('t'))
        .with_icon("🌓"),
    );
    registry.register(
        Command::new(HELP_ABOUT, "About", "Display application information").with_icon("ℹ️"),
    );
    registry.register(
        Command::new(APP_QUIT, "Quit", "Exit the application").with_shortcut(Shortcut::ctrl('q')),
    );
}
