//! Root message enum routing actions between Shell and Demo.

use crate::demo::DemoMessage;
use desktop_shell::ShellMessage;

/// Unified message type for the entire desktop application.
#[derive(Debug, Clone)]
pub enum Message {
    Shell(ShellMessage),
    Demo(DemoMessage),
    Event(iced::Event, iced::event::Status),
}

impl From<ShellMessage> for Message {
    fn from(msg: ShellMessage) -> Self {
        Self::Shell(msg)
    }
}

impl From<DemoMessage> for Message {
    fn from(msg: DemoMessage) -> Self {
        Self::Demo(msg)
    }
}
