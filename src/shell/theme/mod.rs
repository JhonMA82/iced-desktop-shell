//! Theme definitions, palettes, and style tokens for iced-desktop-shell.

pub mod style;
pub mod tokens;

use serde::{Deserialize, Serialize};
pub use style::{DARK_PALETTE, LIGHT_PALETTE, Palette};

/// Theme mode of the application shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    Dark,
    Light,
}

impl ThemeMode {
    /// Toggle between Dark and Light theme modes.
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    /// Retrieve the color palette associated with this theme mode.
    pub fn palette(self) -> Palette {
        match self {
            Self::Dark => DARK_PALETTE,
            Self::Light => LIGHT_PALETTE,
        }
    }

    /// Map to Iced's built-in theme.
    pub fn to_iced_theme(self) -> iced::Theme {
        match self {
            Self::Dark => iced::Theme::Dark,
            Self::Light => iced::Theme::Light,
        }
    }

    /// Display string for status bar or labels.
    pub fn label(self) -> &'static str {
        match self {
            Self::Dark => "Dark",
            Self::Light => "Light",
        }
    }
}
