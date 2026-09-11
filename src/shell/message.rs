//! Messages processed internally by the shell.

use super::command::CommandId;
use super::panel::PanelId;
use super::ribbon::model::RibbonTabId;

/// Shell-level user actions and events.
#[derive(Debug, Clone)]
pub enum ShellMessage {
    ExecuteCommand(CommandId),
    SelectRibbonTab(RibbonTabId),
    TogglePanel(PanelId),
    SelectBottomTab(String),
    ToggleTheme,
    CloseAboutDialog,
    SetStatusLeft(String),
    SetStatusCenter(String),
    SetStatusRight(String),
}
