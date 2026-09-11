//! Core state of the reusable desktop shell infrastructure.

use super::bottom_panel::BottomPanelState;
use super::command::CommandRegistry;
use super::dock::DockLayout;
use super::persistence::{PREFERENCES_SAVE_THROTTLE, ShellPreferences};
use super::ribbon::RibbonTab;
use super::ribbon::model::RibbonTabId;
use super::status_bar::StatusBarState;
use super::theme::ThemeMode;
use std::time::Instant;

/// Encapsulated state of the shell.
/// Default is strictly pure and performs NO I/O operations.
#[derive(Debug, Clone)]
pub struct ShellState {
    pub theme: ThemeMode,
    pub commands: CommandRegistry,
    pub ribbon_tabs: Vec<RibbonTab>,
    pub active_ribbon_tab: RibbonTabId,
    pub dock: DockLayout,
    pub bottom_panel: BottomPanelState,
    pub status_bar: StatusBarState,
    pub show_about_dialog: bool,
    /// Set when preferences changed in memory but were not flushed to disk yet.
    pub preferences_dirty: bool,
    /// Last time preferences were written to disk (`None` = never saved).
    pub last_preferences_save: Option<Instant>,
}

impl Default for ShellState {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            commands: CommandRegistry::new(),
            ribbon_tabs: Vec::new(),
            active_ribbon_tab: RibbonTabId::default(),
            dock: DockLayout::new(),
            bottom_panel: BottomPanelState::default(),
            status_bar: StatusBarState::default(),
            show_about_dialog: false,
            preferences_dirty: false,
            last_preferences_save: None,
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

    /// Persist current shell settings to disk (forced write).
    /// Clears the dirty flag and records the save timestamp.
    pub fn save_preferences(&mut self) {
        let mut panel_visibility = std::collections::HashMap::new();
        for (id, panel) in self.dock.panels() {
            panel_visibility.insert(id.0.clone(), panel.visible);
        }

        let prefs = ShellPreferences {
            theme: self.theme,
            panel_visibility,
        };
        prefs.save();
        self.preferences_dirty = false;
        self.last_preferences_save = Some(Instant::now());
    }

    /// Mark preferences as changed; write to disk only when more than
    /// [`PREFERENCES_SAVE_THROTTLE`] elapsed since the last save.
    /// Hot paths (panel/theme toggles) must call this instead of
    /// [`Self::save_preferences`] to avoid a JSON write per toggle.
    pub fn request_save_preferences(&mut self) {
        self.preferences_dirty = true;
        let should_write = match self.last_preferences_save {
            None => true,
            Some(last) => last.elapsed() > PREFERENCES_SAVE_THROTTLE,
        };
        if should_write {
            self.save_preferences();
        }
    }

    /// Write pending preferences to disk, but only when the dirty flag is set.
    /// Always clears the flag afterwards so shutdown paths (`APP_QUIT`)
    /// never lose the latest in-memory settings.
    pub fn flush_preferences(&mut self) {
        if self.preferences_dirty {
            self.save_preferences();
        }
    }
}
