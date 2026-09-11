//! Generic, extensible panel abstractions for side and bottom layout containers.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Generic, string-based panel identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PanelId(pub String);

impl PanelId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PanelId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for PanelId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for PanelId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Docking location for panels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PanelLocation {
    Left,
    Right,
    Bottom,
}

/// Dynamic panel state.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanelState {
    pub id: PanelId,
    pub title: String,
    pub visible: bool,
    pub location: PanelLocation,
    pub size: f32,
}

impl PanelState {
    pub fn new(
        id: impl Into<PanelId>,
        title: impl Into<String>,
        location: PanelLocation,
        size: f32,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            visible: true,
            location,
            size,
        }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
    }
}
