//! Shell-only tests for the `desktop-shell` crate.
//!
//! These tests cover domain-agnostic infrastructure: theme, shortcuts,
//! command registry, dock order, preferences serde/defaults, dirty/flush
//! throttle, `save_to` roundtrip, and panel size clamping.

use desktop_shell::ShellState;
use desktop_shell::command::{Command, CommandId, CommandRegistry, Shortcut};
use desktop_shell::dock::DockLayout;
use desktop_shell::panel::{PanelId, PanelLocation, PanelState};
use desktop_shell::persistence::ShellPreferences;
use desktop_shell::theme::ThemeMode;

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

#[test]
fn dock_register_clamps_undersized_panel() {
    use desktop_shell::theme::tokens;

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
    use desktop_shell::theme::tokens;

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
