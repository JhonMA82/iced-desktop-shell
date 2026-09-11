//! Centralized registry of application and shell commands.

use super::id::CommandId;
use super::shortcut::Shortcut;
use iced::keyboard::{Key, Modifiers};
use std::collections::HashMap;

/// Concrete command definition.
#[derive(Debug, Clone)]
pub struct Command {
    pub id: CommandId,
    pub label: String,
    pub description: String,
    pub icon: Option<&'static str>,
    pub shortcut: Option<Shortcut>,
    pub enabled: bool,
}

impl Command {
    pub fn new(
        id: impl Into<CommandId>,
        label: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: description.into(),
            icon: None,
            shortcut: None,
            enabled: true,
        }
    }

    pub fn with_shortcut(mut self, shortcut: Shortcut) -> Self {
        self.shortcut = Some(shortcut);
        self
    }

    pub fn with_icon(mut self, icon: &'static str) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Registry managing all available commands.
#[derive(Debug, Clone, Default)]
pub struct CommandRegistry {
    commands: HashMap<CommandId, Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, command: Command) {
        self.commands.insert(command.id.clone(), command);
    }

    pub fn get(&self, id: &CommandId) -> Option<&Command> {
        self.commands.get(id)
    }

    pub fn is_enabled(&self, id: &CommandId) -> bool {
        self.commands.get(id).is_some_and(|c| c.enabled)
    }

    pub fn set_enabled(&mut self, id: &CommandId, enabled: bool) {
        if let Some(cmd) = self.commands.get_mut(id) {
            cmd.enabled = enabled;
        }
    }

    /// Match an incoming keyboard event to a registered command.
    pub fn find_by_shortcut(&self, key: &Key, modifiers: Modifiers) -> Option<CommandId> {
        for command in self.commands.values() {
            if command.enabled
                && let Some(ref sc) = command.shortcut
                && sc.matches(key, modifiers)
            {
                return Some(command.id.clone());
            }
        }
        None
    }
}
