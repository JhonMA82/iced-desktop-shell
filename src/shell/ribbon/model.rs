//! Data models for the generic, data-driven Ribbon component.

use super::super::command::CommandId;

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
    pub id: String,
    pub label: String,
    pub groups: Vec<RibbonGroup>,
}

impl RibbonTab {
    pub fn new(id: impl Into<String>, label: impl Into<String>, groups: Vec<RibbonGroup>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            groups,
        }
    }
}
