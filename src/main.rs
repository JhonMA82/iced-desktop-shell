//! Entry point for iced-desktop-shell.

use iced::event;
use iced_desktop_shell::app::{AppState, Message, update, view};
use tracing::info;
use tracing_subscriber::EnvFilter;

fn main() -> iced::Result {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("iced_desktop_shell=info,iced=warn"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    info!(
        "Starting iced-desktop-shell v{}...",
        env!("CARGO_PKG_VERSION")
    );

    iced::application(AppState::new, update, view)
        .title("iced-desktop-shell")
        .theme(|state: &AppState| state.shell.theme.to_iced_theme())
        .subscription(|_state: &AppState| {
            event::listen_with(|event, status, _window| Some(Message::Event(event, status)))
        })
        .window(iced::window::Settings {
            size: iced::Size::new(1280.0, 800.0),
            min_size: Some(iced::Size::new(800.0, 600.0)),
            position: iced::window::Position::Centered,
            ..Default::default()
        })
        .run()
}
