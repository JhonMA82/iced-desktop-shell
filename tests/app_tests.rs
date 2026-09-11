//! Application-level tests for `iced-desktop-shell`.
//!
//! These tests drive app/demo behavior through the public API: the update
//! cycle, command handler dispatch, demo registration, demo selection, and
//! `RibbonTabId` selection via `AppState`/`update`.

use desktop_shell::ShellMessage;
use desktop_shell::command::{CommandId, CommandRegistry};
use desktop_shell::panel::PanelId;
use iced_desktop_shell::app::Message;
use iced_desktop_shell::app::state::AppState;
use iced_desktop_shell::app::update::update;
use iced_desktop_shell::demo::DemoMessage;
use iced_desktop_shell::demo::{
    APP_NEW, APP_OPEN, APP_QUIT, APP_SAVE, DemoItemId, DemoState, VIEW_TOGGLE_BOTTOM_PANEL,
    VIEW_TOGGLE_EXPLORER, VIEW_TOGGLE_INSPECTOR, register_demo_commands,
};

#[test]
fn test_demo_command_registration() {
    let mut reg = CommandRegistry::new();
    register_demo_commands(&mut reg);

    assert!(reg.get(&CommandId::from(APP_NEW)).is_some());
    assert!(reg.get(&CommandId::from(APP_OPEN)).is_some());
    assert!(reg.get(&CommandId::from(APP_SAVE)).is_some());
    assert!(reg.get(&CommandId::from(APP_QUIT)).is_some());
    assert!(reg.get(&CommandId::from(VIEW_TOGGLE_EXPLORER)).is_some());
    assert!(reg.get(&CommandId::from(VIEW_TOGGLE_INSPECTOR)).is_some());
    assert!(
        reg.get(&CommandId::from(VIEW_TOGGLE_BOTTOM_PANEL))
            .is_some()
    );
}

#[test]
fn test_demo_state_selection() {
    let mut demo = DemoState::new();
    assert_eq!(demo.selected_item, Some(DemoItemId::ComponentA));

    demo.selected_item = Some(DemoItemId::ComponentB);
    let props = demo
        .selected_properties()
        .expect("Component B properties should exist");
    assert_eq!(props.name, "Component B");
    assert_eq!(props.transform_x, 240.0);
}

#[test]
fn select_item_sets_status_and_log() {
    let mut state = AppState::default();
    let logs_before = state.demo.logs.len();

    let _ = update(
        &mut state,
        Message::Demo(DemoMessage::SelectItem(DemoItemId::ComponentA)),
    );

    assert_eq!(state.demo.selected_item, Some(DemoItemId::ComponentA));
    assert!(
        state
            .shell
            .status_bar
            .center_text
            .contains(DemoItemId::ComponentA.label())
    );
    assert!(state.demo.logs.len() > logs_before);
}

#[test]
fn toggle_enabled_flips_and_logs() {
    let mut state = AppState::default();
    let before = state
        .demo
        .properties
        .get(&DemoItemId::ComponentA)
        .expect("Component A properties should exist")
        .enabled;

    let _ = update(
        &mut state,
        Message::Demo(DemoMessage::ToggleEnabled(DemoItemId::ComponentA)),
    );

    let after = state
        .demo
        .properties
        .get(&DemoItemId::ComponentA)
        .expect("Component A properties should exist")
        .enabled;
    assert_eq!(after, !before);
    assert!(
        state
            .demo
            .logs
            .last()
            .expect("toggle should append a log entry")
            .contains("enabled")
    );
}

#[test]
fn increment_then_clear_logs() {
    let mut state = AppState::default();
    let before_x = state
        .demo
        .properties
        .get(&DemoItemId::ComponentA)
        .expect("Component A properties should exist")
        .transform_x;

    let _ = update(
        &mut state,
        Message::Demo(DemoMessage::IncrementX(DemoItemId::ComponentA)),
    );
    let after_x = state
        .demo
        .properties
        .get(&DemoItemId::ComponentA)
        .expect("Component A properties should exist")
        .transform_x;
    assert_eq!(after_x, before_x + 5.0);

    let _ = update(&mut state, Message::Demo(DemoMessage::ClearLogs));
    assert!(state.demo.logs.is_empty());
}

#[test]
fn execute_new_command() {
    let mut state = AppState::default();

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(APP_NEW))),
    );

    assert_eq!(state.shell.status_bar.left_text, "New project created");
    assert!(
        state
            .demo
            .logs
            .iter()
            .any(|entry| entry.contains("New project"))
    );
}

#[test]
fn default_active_ribbon_tab_matches_first_tab() {
    let state = AppState::default();
    let first = state
        .shell
        .ribbon_tabs
        .first()
        .expect("demo should register ribbon tabs");
    assert_eq!(state.shell.active_ribbon_tab, first.id);
}

#[test]
fn select_ribbon_tab_sets_id() {
    use desktop_shell::ribbon::model::RibbonTabId;

    let mut state = AppState::default();
    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::SelectRibbonTab(RibbonTabId::from("view"))),
    );
    assert_eq!(state.shell.active_ribbon_tab, RibbonTabId::from("view"));
}

#[test]
fn unknown_ribbon_tab_id_does_not_panic() {
    use desktop_shell::ribbon::model::RibbonTabId;

    let mut state = AppState::default();
    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::SelectRibbonTab(RibbonTabId::from("missing"))),
    );
    assert_eq!(state.shell.active_ribbon_tab.as_str(), "missing");

    // The ribbon view must fall back to the first tab instead of panicking.
    let palette = state.shell.theme.palette();
    let _ = desktop_shell::ribbon::view::view(
        &state.shell.ribbon_tabs,
        &state.shell.active_ribbon_tab,
        &state.shell.commands,
        palette,
    );
}

fn throttled_state() -> AppState {
    let mut state = AppState::default();
    // Inside the throttle window: toggles mark dirty without touching disk.
    state.shell.last_preferences_save = Some(std::time::Instant::now());
    state
}

fn panel_visible(state: &AppState, panel: &str) -> bool {
    state
        .shell
        .dock
        .get(&PanelId::from(panel))
        .unwrap_or_else(|| panic!("panel {panel} should be registered"))
        .visible
}

#[test]
fn command_table_registers_all_demo_commands() {
    use iced_desktop_shell::app::handlers::command_table;
    use iced_desktop_shell::demo::{HELP_ABOUT, VIEW_TOGGLE_THEME};

    let table = command_table();
    for id in [
        APP_NEW,
        APP_OPEN,
        APP_SAVE,
        APP_QUIT,
        VIEW_TOGGLE_EXPLORER,
        VIEW_TOGGLE_INSPECTOR,
        VIEW_TOGGLE_BOTTOM_PANEL,
        VIEW_TOGGLE_THEME,
        HELP_ABOUT,
    ] {
        assert!(
            table.contains_key(&CommandId::from(id)),
            "command table should register {id}"
        );
    }
}

#[test]
fn handler_open_sets_status_and_log() {
    use iced_desktop_shell::demo::APP_OPEN;

    let mut state = AppState::default();
    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(APP_OPEN))),
    );

    assert_eq!(state.shell.status_bar.left_text, "Open dialog requested");
    assert!(
        state
            .demo
            .logs
            .iter()
            .any(|entry| entry.contains("Open project"))
    );
}

#[test]
fn handler_save_sets_status_and_log() {
    use iced_desktop_shell::demo::APP_SAVE;

    let mut state = AppState::default();
    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(APP_SAVE))),
    );

    assert_eq!(state.shell.status_bar.left_text, "All changes saved");
    assert!(
        state
            .demo
            .logs
            .iter()
            .any(|entry| entry.contains("Project saved"))
    );
}

#[test]
fn handler_toggle_explorer_flips_visibility_and_marks_dirty() {
    use iced_desktop_shell::demo::PANEL_EXPLORER;

    let mut state = throttled_state();
    let before = panel_visible(&state, PANEL_EXPLORER);

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(
            VIEW_TOGGLE_EXPLORER,
        ))),
    );

    assert_eq!(panel_visible(&state, PANEL_EXPLORER), !before);
    assert!(state.shell.preferences_dirty);
}

#[test]
fn handler_toggle_inspector_flips_visibility_and_marks_dirty() {
    use iced_desktop_shell::demo::PANEL_INSPECTOR;

    let mut state = throttled_state();
    let before = panel_visible(&state, PANEL_INSPECTOR);

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(
            VIEW_TOGGLE_INSPECTOR,
        ))),
    );

    assert_eq!(panel_visible(&state, PANEL_INSPECTOR), !before);
    assert!(state.shell.preferences_dirty);
}

#[test]
fn handler_toggle_bottom_panel_flips_visibility_and_marks_dirty() {
    use iced_desktop_shell::demo::{PANEL_OUTPUT, VIEW_TOGGLE_BOTTOM_PANEL};

    let mut state = throttled_state();
    let before = panel_visible(&state, PANEL_OUTPUT);

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(
            VIEW_TOGGLE_BOTTOM_PANEL,
        ))),
    );

    assert_eq!(panel_visible(&state, PANEL_OUTPUT), !before);
    assert!(state.shell.preferences_dirty);
}

#[test]
fn handler_toggle_theme_flips_mode_and_logs() {
    use iced_desktop_shell::demo::VIEW_TOGGLE_THEME;

    let mut state = throttled_state();
    let before = state.shell.theme;

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(
            VIEW_TOGGLE_THEME,
        ))),
    );

    assert_eq!(state.shell.theme, before.toggle());
    assert!(state.shell.preferences_dirty);
    assert!(
        state
            .demo
            .logs
            .iter()
            .any(|entry| entry.contains("Toggled to"))
    );
}

#[test]
fn handler_show_about_opens_dialog() {
    use iced_desktop_shell::demo::HELP_ABOUT;

    let mut state = AppState::default();
    assert!(!state.shell.show_about_dialog);

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(HELP_ABOUT))),
    );

    assert!(state.shell.show_about_dialog);
}

#[test]
fn handler_unknown_command_logs_without_panic() {
    let mut state = AppState::default();
    let logs_before = state.demo.logs.len();

    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::ExecuteCommand(CommandId::from(
            "app.does_not_exist",
        ))),
    );

    assert!(state.demo.logs.len() > logs_before);
    assert!(
        state
            .demo
            .logs
            .last()
            .expect("unknown command should append a log entry")
            .contains("Unhandled command")
    );
}

#[test]
fn handler_table_dispatch_matches_direct_call() {
    use iced_desktop_shell::app::handlers::command_table;
    use iced_desktop_shell::demo::APP_NEW;

    let table = command_table();
    let handler = table
        .get(&CommandId::from(APP_NEW))
        .expect("app.new should be registered");

    let mut state = AppState::default();
    let _ = handler(&mut state, &CommandId::from(APP_NEW));

    assert_eq!(state.shell.status_bar.left_text, "New project created");
    assert!(
        state
            .demo
            .logs
            .iter()
            .any(|entry| entry.contains("New project"))
    );
}
