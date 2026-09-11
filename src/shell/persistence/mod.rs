//! Preferences persistence using the OS-specific configuration directory.

use super::theme::ThemeMode;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use tracing::{info, warn};

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "iced-desktop-shell";
const APPLICATION: &str = "iced-desktop-shell";
const PREFS_FILENAME: &str = "preferences.json";

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

        if let Some(parent) = path.parent()
            && let Err(err) = fs::create_dir_all(parent)
        {
            warn!(
                "Failed to create preferences directory at {:?}: {}",
                parent, err
            );
            return;
        }

        match File::create(&path) {
            Ok(file) => {
                let writer = BufWriter::new(file);
                if let Err(err) = serde_json::to_writer_pretty(writer, self) {
                    warn!("Failed to write preferences to {:?}: {}", path, err);
                } else {
                    info!("Preferences successfully saved to {:?}", path);
                }
            }
            Err(err) => {
                warn!("Failed to create preferences file at {:?}: {}", path, err);
            }
        }
    }
}
