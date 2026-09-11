//! Core state of the reusable desktop shell infrastructure.

use super::bottom_panel::BottomPanelState;
use super::command::CommandRegistry;
use super::dock::DockLayout;
use super::persistence::ShellPreferences;
use super::ribbon::RibbonTab;
use super::status_bar::StatusBarState;
use super::theme::ThemeMode;

/// Encapsulated state of the shell.
/// Default is strictly pure and performs NO I/O operations.
#[derive(Debug, Clone)]
pub struct ShellState {
    pub theme: ThemeMode,
    pub commands: CommandRegistry,
    pub ribbon_tabs: Vec<RibbonTab>,
    pub active_ribbon_tab: usize,
    pub dock: DockLayout,
    pub bottom_panel: BottomPanelState,
    pub status_bar: StatusBarState,
    pub show_about_dialog: bool,
}

impl Default for ShellState {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            commands: CommandRegistry::new(),
            ribbon_tabs: Vec::new(),
            active_ribbon_tab: 0,
            dock: DockLayout::new(),
            bottom_panel: BottomPanelState::default(),
            status_bar: StatusBarState::default(),
            show_about_dialog: false,
        }
    }
}

impl ShellState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Load and apply stored preferences from disk.
    pub fn load_preferences(&mut self) {
        let prefs = ShellPreferences::load();
        self.apply_preferences(&prefs);
    }

    /// Apply loaded preferences to current shell state.
    pub fn apply_preferences(&mut self, prefs: &ShellPreferences) {
        self.theme = prefs.theme;
        for (panel_id_str, visible) in &prefs.panel_visibility {
            self.dock
                .set_visible(&panel_id_str.as_str().into(), *visible);
        }
    }

    /// Persist current shell settings to disk.
    pub fn save_preferences(&self) {
        let mut panel_visibility = std::collections::HashMap::new();
        for (id, panel) in self.dock.panels() {
            panel_visibility.insert(id.0.clone(), panel.visible);
        }

        let prefs = ShellPreferences {
            theme: self.theme,
            panel_visibility,
        };
        prefs.save();
    }
}
