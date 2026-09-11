//! Root application state holding Shell and Demo sub-states.

use super::message::Message;
use crate::demo;
use crate::demo::{DemoState, PANEL_EXPLORER, PANEL_INSPECTOR, PANEL_OUTPUT};
use desktop_shell::ShellState;
use desktop_shell::panel::{PanelLocation, PanelState};
use desktop_shell::theme::tokens;
use iced::Task;

/// Top-level application state.
#[derive(Debug, Clone)]
pub struct AppState {
    pub shell: ShellState,
    pub demo: DemoState,
}

impl Default for AppState {
    fn default() -> Self {
        let mut shell = ShellState::new();

        // 1. Register demo panels into generic dock layout (DockLayout.order source of truth)
        shell.dock.register(PanelState::new(
            PANEL_EXPLORER,
            "Project Explorer",
            PanelLocation::Left,
            tokens::SIDEBAR_DEFAULT_WIDTH,
        ));
        shell.dock.register(PanelState::new(
            PANEL_INSPECTOR,
            "Properties Inspector",
            PanelLocation::Right,
            tokens::INSPECTOR_DEFAULT_WIDTH,
        ));
        shell.dock.register(PanelState::new(
            PANEL_OUTPUT,
            "Output & Diagnostics",
            PanelLocation::Bottom,
            tokens::BOTTOM_PANEL_DEFAULT_HEIGHT,
        ));

        // 2. Register application/demo commands into the shell registry
        demo::register_demo_commands(&mut shell.commands);

        // 3. Setup ribbon tabs defined by demo application
        shell.ribbon_tabs = demo::demo_ribbon_tabs();

        // 4. Select the first ribbon tab generically (no hardcoded id).
        if let Some(first_tab) = shell.ribbon_tabs.first() {
            shell.active_ribbon_tab = first_tab.id.clone();
        }

        // Note: Default is kept strictly pure (no disk I/O)
        Self {
            shell,
            demo: DemoState::new(),
        }
    }
}

impl AppState {
    /// Boot/init constructor returning (AppState, Task<Message>) for Iced 0.14 application runner.
    /// Explicitly loads persistent preferences here, keeping `Default::default()` completely pure.
    pub fn new() -> (Self, Task<Message>) {
        let mut state = Self::default();
        state.shell.load_preferences();
        (state, Task::none())
    }
}
