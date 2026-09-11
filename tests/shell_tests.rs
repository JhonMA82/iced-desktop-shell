//! Integration and unit tests for iced-desktop-shell.

use iced_desktop_shell::demo::{
    APP_NEW, APP_OPEN, APP_QUIT, APP_SAVE, DemoItemId, DemoState, VIEW_TOGGLE_BOTTOM_PANEL,
    VIEW_TOGGLE_EXPLORER, VIEW_TOGGLE_INSPECTOR, register_demo_commands,
};
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
