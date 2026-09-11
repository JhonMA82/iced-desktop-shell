//! Preferences persistence using the OS-specific configuration directory.

use super::theme::ThemeMode;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tracing::{info, warn};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "iced-desktop-shell";
const APPLICATION: &str = "iced-desktop-shell";
const PREFS_FILENAME: &str = "preferences.json";

/// Minimum interval between throttled preference writes triggered from hot paths
/// (panel/theme toggles). Kept here — not in theme tokens — because it is a
/// persistence policy, not a UI dimension.
pub const PREFERENCES_SAVE_THROTTLE: Duration = Duration::from_secs(2);

/// Persistent preferences for the desktop shell.
/// Uses `#[serde(default)]` to safely survive future or missing JSON properties.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct ShellPreferences {
    pub theme: ThemeMode,
    pub panel_visibility: HashMap<String, bool>,
}

impl Default for ShellPreferences {
    fn default() -> Self {
        Self {
            theme: ThemeMode::Dark,
            panel_visibility: HashMap::new(),
        }
    }
}

impl ShellPreferences {
    fn config_path() -> Option<PathBuf> {
        ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
            .map(|dirs| dirs.config_dir().join(PREFS_FILENAME))
    }

    /// Load preferences from disk. Falls back safely to defaults on failure without panicking.
    pub fn load() -> Self {
        let Some(path) = Self::config_path() else {
            warn!("Could not resolve OS configuration directory; using default preferences");
            return Self::default();
        };

        if !path.exists() {
            info!(
                "Preferences file does not exist at {:?}; using defaults",
                path
            );
            return Self::default();
        }

        match File::open(&path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(prefs) => {
                        info!("Successfully loaded preferences from {:?}", path);
                        prefs
                    }
                    Err(err) => {
                        warn!(
                            "Preferences file at {:?} was corrupted ({}). Resetting to defaults.",
                            path, err
                        );
                        Self::default()
                    }
                }
            }
            Err(err) => {
                warn!(
                    "Failed to open preferences file at {:?} ({}). Using defaults.",
                    path, err
                );
                Self::default()
            }
        }
    }

    /// Save preferences to disk. Never panics, logs errors.
    pub fn save(&self) {
        let Some(path) = Self::config_path() else {
            warn!("Could not resolve OS configuration directory; skipping preferences save");
            return;
        };

        self.save_to_path(&path);
    }

    /// Save preferences to an explicit path. Returns `true` on success.
    /// Never panics, logs errors. Exposed so tests can round-trip
    /// serialization through a temporary directory instead of the real
    /// OS configuration directory.
    pub fn save_to_path(&self, path: &Path) -> bool {
        if let Some(parent) = path.parent()
            && let Err(err) = fs::create_dir_all(parent)
        {
            warn!(
                "Failed to create preferences directory at {:?}: {}",
                parent, err
            );
            return false;
        }

        match File::create(path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                if let Err(err) = serde_json::to_writer_pretty(writer, self) {
                    warn!("Failed to write preferences to {:?}: {}", path, err);
                    false
                } else {
                    info!("Preferences successfully saved to {:?}", path);
                    true
                }
            }
            Err(err) => {
                warn!("Failed to create preferences file at {:?}: {}", path, err);
                false
            }
        }
    }
}
