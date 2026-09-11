//! Integration and unit tests for iced-desktop-shell.

use iced_desktop_shell::app::Message;
use iced_desktop_shell::app::state::AppState;
use iced_desktop_shell::app::update::update;
use iced_desktop_shell::demo::DemoMessage;
use iced_desktop_shell::demo::{
    APP_NEW, APP_OPEN, APP_QUIT, APP_SAVE, DemoItemId, DemoState, VIEW_TOGGLE_BOTTOM_PANEL,
    VIEW_TOGGLE_EXPLORER, VIEW_TOGGLE_INSPECTOR, register_demo_commands,
};
use iced_desktop_shell::shell::ShellMessage;
use iced_desktop_shell::shell::ShellState;
use iced_desktop_shell::shell::command::{Command, CommandId, CommandRegistry, Shortcut};
use iced_desktop_shell::shell::dock::DockLayout;
use iced_desktop_shell::shell::panel::{PanelId, PanelLocation, PanelState};
use iced_desktop_shell::shell::persistence::ShellPreferences;
use iced_desktop_shell::shell::theme::ThemeMode;

#[test]
fn test_theme_mode_toggle() {
    let theme = ThemeMode::Dark;
    assert_eq!(theme.toggle(), ThemeMode::Light);
    assert_eq!(theme.toggle().toggle(), ThemeMode::Dark);
}

#[test]
fn test_shortcut_formatting_no_leak() {
    let sc = Shortcut::ctrl('s');
    assert_eq!(sc.to_string(), "Ctrl+S");

    let sc2 = Shortcut::ctrl_shift('p');
    assert_eq!(sc2.to_string(), "Ctrl+Shift+P");
}

#[test]
fn test_command_registry() {
    let mut reg = CommandRegistry::new();
    let cmd = Command::new("test.cmd", "Test Command", "A command for testing")
        .with_shortcut(Shortcut::ctrl('t'));

    reg.register(cmd);

    let id = CommandId::from("test.cmd");
    assert!(reg.get(&id).is_some());
    assert!(reg.is_enabled(&id));

    reg.set_enabled(&id, false);
    assert!(!reg.is_enabled(&id));
}

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
fn test_dock_layout_order_source_of_truth() {
    let mut dock = DockLayout::new();
    let p1 = PanelId::from("panel.one");
    let p2 = PanelId::from("panel.two");
    let p3 = PanelId::from("panel.three");

    dock.register(PanelState::new(
        p1.clone(),
        "One",
        PanelLocation::Left,
        200.0,
    ));
    dock.register(PanelState::new(
        p2.clone(),
        "Two",
        PanelLocation::Left,
        250.0,
    ));
    dock.register(PanelState::new(
        p3.clone(),
        "Three",
        PanelLocation::Bottom,
        150.0,
    ));

    let ordered = dock.ordered_panels();
    assert_eq!(ordered.len(), 3);
    assert_eq!(ordered[0].id, p1);
    assert_eq!(ordered[1].id, p2);
    assert_eq!(ordered[2].id, p3);

    let left = dock.panels_at(PanelLocation::Left);
    assert_eq!(left.len(), 2);
    assert_eq!(left[0].id, p1);
    assert_eq!(left[1].id, p2);
}

#[test]
fn test_preferences_serde_default() {
    let json = "{}";
    let prefs: ShellPreferences = serde_json::from_str(json).expect("Deserialization failed");
    assert_eq!(prefs.theme, ThemeMode::Dark);
    assert!(prefs.panel_visibility.is_empty());
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

// ---------------------------------------------------------------------------
// Fase 1 — Tarea 1: update() puro (sin I/O) vía API pública.
// ---------------------------------------------------------------------------

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

// ---------------------------------------------------------------------------
// Fase 1 — Tarea 2: throttle + dirty flag de preferencias.
// ---------------------------------------------------------------------------

#[test]
fn preferences_request_marks_dirty_without_disk_write_when_throttled() {
    let mut shell = ShellState::default();
    assert!(!shell.preferences_dirty);
    assert!(shell.last_preferences_save.is_none());

    // Saved 1s ago: inside the 2s throttle window, so no disk write happens.
    shell.last_preferences_save =
        Some(std::time::Instant::now() - std::time::Duration::from_secs(1));
    shell.request_save_preferences();

    assert!(shell.preferences_dirty);
    // A write would have reset the timestamp to ~now; proving it is untouched
    // proves no write happened.
    let elapsed = shell
        .last_preferences_save
        .expect("timestamp should survive a throttled request")
        .elapsed();
    assert!(
        elapsed >= std::time::Duration::from_secs(1),
        "throttled request must not rewrite the save timestamp"
    );
}

#[test]
fn preferences_flush_without_dirty_writes_nothing() {
    let mut shell = ShellState::default();
    shell.flush_preferences();
    assert!(!shell.preferences_dirty);
    assert!(
        shell.last_preferences_save.is_none(),
        "flush with clean flag must not touch disk"
    );
}

#[test]
fn preferences_flush_clears_dirty_flag() {
    // Redirect the OS config dir to a temp location so the forced flush
    // exercises the real write path without touching user data.
    let dir = std::env::temp_dir().join(format!("iced-shell-flush-{}", std::process::id()));
    unsafe { std::env::set_var("XDG_CONFIG_HOME", &dir) };

    let mut shell = ShellState {
        preferences_dirty: true,
        ..Default::default()
    };
    shell.flush_preferences();

    assert!(!shell.preferences_dirty);
    assert!(shell.last_preferences_save.is_some());

    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn preferences_save_to_path_roundtrip() {
    let prefs = ShellPreferences::default();
    let path = std::env::temp_dir().join(format!("prefs-{}-roundtrip.json", std::process::id()));

    assert!(prefs.save_to_path(&path));
    let raw = std::fs::read_to_string(&path).expect("temp prefs should be readable");
    let back: ShellPreferences = serde_json::from_str(&raw).expect("temp prefs should parse");
    assert_eq!(back.theme, prefs.theme);
    assert!(back.panel_visibility.is_empty());

    let _ = std::fs::remove_file(&path);
}

// ---------------------------------------------------------------------------
// Fase 1 — Tarea 3: RibbonTabId en vez de índice.
// ---------------------------------------------------------------------------

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
    use iced_desktop_shell::shell::ribbon::model::RibbonTabId;

    let mut state = AppState::default();
    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::SelectRibbonTab(RibbonTabId::from("view"))),
    );
    assert_eq!(state.shell.active_ribbon_tab, RibbonTabId::from("view"));
}

#[test]
fn unknown_ribbon_tab_id_does_not_panic() {
    use iced_desktop_shell::shell::ribbon::model::RibbonTabId;

    let mut state = AppState::default();
    let _ = update(
        &mut state,
        Message::Shell(ShellMessage::SelectRibbonTab(RibbonTabId::from("missing"))),
    );
    assert_eq!(state.shell.active_ribbon_tab.as_str(), "missing");

    // The ribbon view must fall back to the first tab instead of panicking.
    let palette = state.shell.theme.palette();
    let _ = iced_desktop_shell::shell::ribbon::view::view(
        &state.shell.ribbon_tabs,
        &state.shell.active_ribbon_tab,
        &state.shell.commands,
        palette,
    );
}

// ---------------------------------------------------------------------------
// Fase 1 — Tarea 4: clamp de tamaño de paneles.
// ---------------------------------------------------------------------------

#[test]
fn dock_register_clamps_undersized_panel() {
    use iced_desktop_shell::shell::theme::tokens;

    let mut dock = DockLayout::new();
    dock.register(PanelState::new(
        PanelId::from("tiny"),
        "Tiny",
        PanelLocation::Left,
        5.0,
    ));

    let panel = dock
        .get(&PanelId::from("tiny"))
        .expect("panel should be registered");
    assert_eq!(panel.size, tokens::SIDEBAR_MIN_WIDTH);
}

#[test]
fn dock_set_size_clamps_to_location_minimum() {
    use iced_desktop_shell::shell::theme::tokens;

    let mut dock = DockLayout::new();
    dock.register(PanelState::new(
        PanelId::from("left"),
        "Left",
        PanelLocation::Left,
        250.0,
    ));
    dock.register(PanelState::new(
        PanelId::from("right"),
        "Right",
        PanelLocation::Right,
        280.0,
    ));
    dock.register(PanelState::new(
        PanelId::from("bottom"),
        "Bottom",
        PanelLocation::Bottom,
        190.0,
    ));

    dock.set_size(&PanelId::from("left"), 0.0);
    assert_eq!(
        dock.get(&PanelId::from("left")).expect("left panel").size,
        tokens::SIDEBAR_MIN_WIDTH
    );

    dock.set_size(&PanelId::from("right"), 0.0 - 42.0);
    assert_eq!(
        dock.get(&PanelId::from("right")).expect("right panel").size,
        tokens::INSPECTOR_MIN_WIDTH
    );

    dock.set_size(&PanelId::from("bottom"), 10.0);
    assert_eq!(
        dock.get(&PanelId::from("bottom"))
            .expect("bottom panel")
            .size,
        tokens::BOTTOM_PANEL_MIN_HEIGHT
    );

    // Sane sizes pass through untouched; unknown ids are ignored, not panics.
    dock.set_size(&PanelId::from("left"), 400.0);
    assert_eq!(
        dock.get(&PanelId::from("left")).expect("left panel").size,
        400.0
    );
    dock.set_size(&PanelId::from("ghost"), 500.0);
}

// ---------------------------------------------------------------------------
// Fase 2: tabla de handlers de comandos.
// ---------------------------------------------------------------------------

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
