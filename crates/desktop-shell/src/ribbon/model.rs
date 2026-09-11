//! Data models for the generic, data-driven Ribbon component.

use super::super::command::CommandId;
use std::fmt;

/// Strongly typed identifier for ribbon tabs.
/// Mirrors the `PanelId` / `CommandId` newtype style.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash)]
pub struct RibbonTabId(pub String);

impl RibbonTabId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RibbonTabId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for RibbonTabId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for RibbonTabId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

/// Single ribbon item referencing a command.
#[derive(Debug, Clone)]
pub enum RibbonItem {
    Button {
        command_id: CommandId,
        label_override: Option<String>,
        icon: Option<&'static str>,
    },
    Separator,
}

impl RibbonItem {
    pub fn button(command_id: impl Into<CommandId>) -> Self {
        Self::Button {
            command_id: command_id.into(),
            label_override: None,
            icon: None,
        }
    }

    pub fn button_with(
        command_id: impl Into<CommandId>,
        label: impl Into<String>,
        icon: &'static str,
    ) -> Self {
        Self::Button {
            command_id: command_id.into(),
            label_override: Some(label.into()),
            icon: Some(icon),
        }
    }
}

/// Logical grouping of related ribbon items.
#[derive(Debug, Clone)]
pub struct RibbonGroup {
    pub title: String,
    pub items: Vec<RibbonItem>,
}

impl RibbonGroup {
    pub fn new(title: impl Into<String>, items: Vec<RibbonItem>) -> Self {
        Self {
            title: title.into(),
            items,
        }
    }
}

/// Tab containing multiple ribbon groups.
#[derive(Debug, Clone)]
pub struct RibbonTab {
    pub id: RibbonTabId,
    pub label: String,
    pub groups: Vec<RibbonGroup>,
}

impl RibbonTab {
    pub fn new(
        id: impl Into<RibbonTabId>,
        label: impl Into<String>,
        groups: Vec<RibbonGroup>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            groups,
        }
    }
}
