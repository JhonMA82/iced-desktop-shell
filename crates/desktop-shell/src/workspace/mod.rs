//! Workspace layout container.

use super::theme::Palette;
use iced::widget::container;
use iced::{Element, Length};

/// Generic workspace wrapper.
pub fn container_view<'a, M: 'a>(content: Element<'a, M>, palette: Palette) -> Element<'a, M> {
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.background.into()),
            border: iced::border::Border {
                color: palette.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
