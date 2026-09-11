//! Status bar component displayed at the bottom edge of the shell.

use super::theme::{Palette, tokens};
use iced::widget::{Space, container, row, text};
use iced::{Alignment, Element, Length};

/// State holding status bar segment texts.
#[derive(Debug, Clone)]
pub struct StatusBarState {
    pub left_text: String,
    pub center_text: String,
    pub right_text: String,
}

impl Default for StatusBarState {
    fn default() -> Self {
        Self {
            left_text: "Ready".to_string(),
            center_text: "No Selection".to_string(),
            right_text: "100% | UTF-8".to_string(),
        }
    }
}

/// Render the status bar.
pub fn view<'a, M: 'a>(state: &'a StatusBarState, palette: Palette) -> Element<'a, M> {
    let content = row![
        text(&state.left_text)
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_secondary),
        Space::new().width(Length::Fill),
        text(&state.center_text)
            .size(tokens::FONT_SIZE_XS)
            .color(palette.accent),
        Space::new().width(Length::Fill),
        text(&state.right_text)
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center)
    .padding([0.0, tokens::SPACING_SM])
    .width(Length::Fill);

    container(content)
        .height(tokens::STATUSBAR_HEIGHT)
        .width(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.status_bar_bg.into()),
            border: iced::border::Border {
                color: palette.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
