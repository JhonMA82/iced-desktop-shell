//! Command handler table for application-level commands.
//!
//! Each handler owns the exact behavior previously embedded in the
//! `execute_command` string match. Handlers live in `app/` (not in the
//! shell registry) because they mutate [`AppState`].

use std::collections::HashMap;

use super::message::Message;
use super::state::AppState;
use crate::demo;
use desktop_shell::command::CommandId;
use iced::Task;
use tracing::info;

/// Handler mutating [`AppState`] for a single command.
pub type CommandHandler = fn(&mut AppState, &CommandId) -> Task<Message>;

pub fn handle_new(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state.demo.add_log("[Command] App: New project initiated");
    state.shell.status_bar.left_text = "New project created".to_string();
    Task::none()
}

pub fn handle_open(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state
        .demo
        .add_log("[Command] App: Open project dialog triggered");
    state.shell.status_bar.left_text = "Open dialog requested".to_string();
    Task::none()
}

pub fn handle_save(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state
        .demo
        .add_log("[Command] App: Project saved successfully");
    state.shell.status_bar.left_text = "All changes saved".to_string();
    Task::none()
}

pub fn handle_quit(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    info!("Quit command: closing application via iced::exit()");
    state.shell.flush_preferences();
    iced::exit()
}

pub fn handle_toggle_explorer(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state.shell.dock.toggle(&demo::PANEL_EXPLORER.into());
    state.shell.request_save_preferences();
    Task::none()
}

pub fn handle_toggle_inspector(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state.shell.dock.toggle(&demo::PANEL_INSPECTOR.into());
    state.shell.request_save_preferences();
    Task::none()
}

pub fn handle_toggle_bottom_panel(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state.shell.dock.toggle(&demo::PANEL_OUTPUT.into());
    state.shell.request_save_preferences();
    Task::none()
}

pub fn handle_toggle_theme(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state.shell.theme = state.shell.theme.toggle();
    state.shell.request_save_preferences();
    state.demo.add_log(format!(
        "[Theme] Toggled to {} mode",
        state.shell.theme.label()
    ));
    Task::none()
}

pub fn handle_show_about(state: &mut AppState, _cmd: &CommandId) -> Task<Message> {
    state.shell.show_about_dialog = true;
    Task::none()
}

pub fn unhandled_command(state: &mut AppState, cmd_id: &CommandId) -> Task<Message> {
    state
        .demo
        .add_log(format!("[Command] Unhandled command: {}", cmd_id));
    Task::none()
}

/// Dispatch table mapping each demo command id to its handler.
pub fn command_table() -> HashMap<CommandId, CommandHandler> {
    HashMap::from([
        (CommandId::from(demo::APP_NEW), handle_new as CommandHandler),
        (
            CommandId::from(demo::APP_OPEN),
            handle_open as CommandHandler,
        ),
        (
            CommandId::from(demo::APP_SAVE),
            handle_save as CommandHandler,
        ),
        (
            CommandId::from(demo::APP_QUIT),
            handle_quit as CommandHandler,
        ),
        (
            CommandId::from(demo::VIEW_TOGGLE_EXPLORER),
            handle_toggle_explorer as CommandHandler,
        ),
        (
            CommandId::from(demo::VIEW_TOGGLE_INSPECTOR),
            handle_toggle_inspector as CommandHandler,
        ),
        (
            CommandId::from(demo::VIEW_TOGGLE_BOTTOM_PANEL),
            handle_toggle_bottom_panel as CommandHandler,
        ),
        (
            CommandId::from(demo::VIEW_TOGGLE_THEME),
            handle_toggle_theme as CommandHandler,
        ),
        (
            CommandId::from(demo::HELP_ABOUT),
            handle_show_about as CommandHandler,
        ),
    ])
}
