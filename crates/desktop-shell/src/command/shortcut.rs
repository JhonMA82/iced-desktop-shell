//! Keyboard shortcuts representation and matching.

use iced::keyboard::key::Named;
use iced::keyboard::{Key, Modifiers};
use std::fmt;

/// Keyboard shortcut representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shortcut {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
    pub key: ShortcutKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutKey {
    Char(char),
    Named(Named),
}

impl Shortcut {
    pub const fn ctrl(c: char) -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: false,
            key: ShortcutKey::Char(c),
        }
    }

    pub const fn ctrl_shift(c: char) -> Self {
        Self {
            ctrl: true,
            shift: true,
            alt: false,
            key: ShortcutKey::Char(c),
        }
    }

    pub const fn ctrl_named(named: Named) -> Self {
        Self {
            ctrl: true,
            shift: false,
            alt: false,
            key: ShortcutKey::Named(named),
        }
    }

    /// Check if an incoming iced keyboard event matches this shortcut.
    pub fn matches(&self, key: &Key, modifiers: Modifiers) -> bool {
        let ctrl_pressed = modifiers.control() || modifiers.command();
        if self.ctrl != ctrl_pressed {
            return false;
        }
        if self.shift != modifiers.shift() {
            return false;
        }
        if self.alt != modifiers.alt() {
            return false;
        }

        match (&self.key, key) {
            (ShortcutKey::Char(sc_char), Key::Character(s)) => {
                s.chars().next().map(|c| c.to_ascii_lowercase())
                    == Some(sc_char.to_ascii_lowercase())
            }
            (ShortcutKey::Named(sc_named), Key::Named(k_named)) => sc_named == k_named,
            _ => false,
        }
    }
}

impl fmt::Display for Shortcut {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ctrl {
            write!(f, "Ctrl+")?;
        }
        if self.alt {
            write!(f, "Alt+")?;
        }
        if self.shift {
            write!(f, "Shift+")?;
        }
        match &self.key {
            ShortcutKey::Char(c) => {
                for up in c.to_uppercase() {
                    write!(f, "{}", up)?;
                }
                Ok(())
            }
            ShortcutKey::Named(Named::Enter) => write!(f, "Enter"),
            ShortcutKey::Named(Named::Escape) => write!(f, "Esc"),
            ShortcutKey::Named(Named::Space) => write!(f, "Space"),
            ShortcutKey::Named(Named::Tab) => write!(f, "Tab"),
            ShortcutKey::Named(Named::Backspace) => write!(f, "Backspace"),
            _ => write!(f, "Key"),
        }
    }
}
