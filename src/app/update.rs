//! Update logic orchestrating state changes across Shell and Demo.

use super::message::Message;
use super::state::AppState;
use crate::demo;
use crate::demo::DemoMessage;
use crate::shell::ShellMessage;
use crate::shell::command::CommandId;
use iced::keyboard::Event as KeyboardEvent;
use iced::{Event, Task};
use tracing::info;

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Shell(shell_msg) => handle_shell_message(state, shell_msg),
        Message::Demo(demo_msg) => handle_demo_message(state, demo_msg),
        Message::Event(event, status) => handle_iced_event(state, event, status),
    }
}

fn handle_shell_message(state: &mut AppState, msg: ShellMessage) -> Task<Message> {
    match msg {
        ShellMessage::ExecuteCommand(cmd_id) => execute_command(state, &cmd_id),
        ShellMessage::SelectRibbonTab(index) => {
            state.shell.active_ribbon_tab = index;
            Task::none()
        }
        ShellMessage::TogglePanel(panel_id) => {
            state.shell.dock.toggle(&panel_id);
            state.shell.save_preferences();
            Task::none()
        }
        ShellMessage::SelectBottomTab(tab_name) => {
            state.shell.bottom_panel.active_tab = tab_name;
            Task::none()
        }
        ShellMessage::ToggleTheme => {
            state.shell.theme = state.shell.theme.toggle();
            state.shell.save_preferences();
            let label = state.shell.theme.label();
            state
                .demo
                .add_log(format!("[Theme] Switched to {} mode", label));
            Task::none()
        }
        ShellMessage::CloseAboutDialog => {
            state.shell.show_about_dialog = false;
            Task::none()
        }
        ShellMessage::SetStatusLeft(text) => {
            state.shell.status_bar.left_text = text;
            Task::none()
        }
        ShellMessage::SetStatusCenter(text) => {
            state.shell.status_bar.center_text = text;
            Task::none()
        }
        ShellMessage::SetStatusRight(text) => {
            state.shell.status_bar.right_text = text;
            Task::none()
        }
    }
}

fn handle_demo_message(state: &mut AppState, msg: DemoMessage) -> Task<Message> {
    match msg {
        DemoMessage::SelectItem(item_id) => {
            state.shell.status_bar.center_text = format!("Selected: {}", item_id.label());
            state
                .demo
                .add_log(format!("[Selection] Activated entity: {}", item_id.label()));
            state.demo.selected_item = Some(item_id);
            Task::none()
        }
        DemoMessage::ToggleEnabled(item_id) => {
            let enabled = if let Some(props) = state.demo.properties.get_mut(&item_id) {
                props.enabled = !props.enabled;
                props.enabled
            } else {
                return Task::none();
            };
            state.demo.add_log(format!(
                "[Entity] {} enabled = {}",
                item_id.label(),
                enabled
            ));
            Task::none()
        }
        DemoMessage::IncrementX(item_id) => {
            if let Some(props) = state.demo.properties.get_mut(&item_id) {
                props.transform_x += 5.0;
            }
            Task::none()
        }
        DemoMessage::DecrementX(item_id) => {
            if let Some(props) = state.demo.properties.get_mut(&item_id) {
                props.transform_x = (props.transform_x - 5.0).max(0.0);
            }
            Task::none()
        }
        DemoMessage::IncrementY(item_id) => {
            if let Some(props) = state.demo.properties.get_mut(&item_id) {
                props.transform_y += 5.0;
            }
            Task::none()
        }
        DemoMessage::DecrementY(item_id) => {
            if let Some(props) = state.demo.properties.get_mut(&item_id) {
                props.transform_y = (props.transform_y - 5.0).max(0.0);
            }
            Task::none()
        }
        DemoMessage::ClearLogs => {
            state.demo.logs.clear();
            Task::none()
        }
    }
}

fn execute_command(state: &mut AppState, cmd_id: &CommandId) -> Task<Message> {
    info!("Executing command: {}", cmd_id);

    match cmd_id.as_str() {
        demo::APP_NEW => {
            state.demo.add_log("[Command] App: New project initiated");
            state.shell.status_bar.left_text = "New project created".to_string();
            Task::none()
        }
        demo::APP_OPEN => {
            state
                .demo
                .add_log("[Command] App: Open project dialog triggered");
            state.shell.status_bar.left_text = "Open dialog requested".to_string();
            Task::none()
        }
        demo::APP_SAVE => {
            state
                .demo
                .add_log("[Command] App: Project saved successfully");
            state.shell.status_bar.left_text = "All changes saved".to_string();
            Task::none()
        }
        demo::APP_QUIT => {
            info!("Quit command: closing application via iced::exit()");
            iced::exit()
        }
        demo::VIEW_TOGGLE_EXPLORER => {
            state.shell.dock.toggle(&demo::PANEL_EXPLORER.into());
            state.shell.save_preferences();
            Task::none()
        }
        demo::VIEW_TOGGLE_INSPECTOR => {
            state.shell.dock.toggle(&demo::PANEL_INSPECTOR.into());
            state.shell.save_preferences();
            Task::none()
        }
        demo::VIEW_TOGGLE_BOTTOM_PANEL => {
            state.shell.dock.toggle(&demo::PANEL_OUTPUT.into());
            state.shell.save_preferences();
            Task::none()
        }
        demo::VIEW_TOGGLE_THEME => {
            state.shell.theme = state.shell.theme.toggle();
            state.shell.save_preferences();
            state.demo.add_log(format!(
                "[Theme] Toggled to {} mode",
                state.shell.theme.label()
            ));
            Task::none()
        }
        demo::HELP_ABOUT => {
            state.shell.show_about_dialog = true;
            Task::none()
        }
        other => {
            state
                .demo
                .add_log(format!("[Command] Unhandled command: {}", other));
            Task::none()
        }
    }
}

fn handle_iced_event(
    state: &mut AppState,
    event: Event,
    status: iced::event::Status,
) -> Task<Message> {
    if status == iced::event::Status::Ignored
        && let Event::Keyboard(KeyboardEvent::KeyPressed { key, modifiers, .. }) = event
        && let Some(cmd_id) = state.shell.commands.find_by_shortcut(&key, modifiers)
    {
        return execute_command(state, &cmd_id);
    }
    Task::none()
}
