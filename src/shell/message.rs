//! Messages processed internally by the shell.

use super::command::CommandId;
use super::panel::PanelId;

/// Shell-level user actions and events.
#[derive(Debug, Clone)]
pub enum ShellMessage {
    ExecuteCommand(CommandId),
    SelectRibbonTab(usize),
    TogglePanel(PanelId),
    SelectBottomTab(String),
    ToggleTheme,
    CloseAboutDialog,
    SetStatusLeft(String),
    SetStatusCenter(String),
    SetStatusRight(String),
}
