//! Inspector shell container.

use super::message::ShellMessage;
use super::theme::{Palette, tokens};
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

/// Renders the inspector shell container wrapping arbitrary domain content.
pub fn view<'a, M>(
    title: &'a str,
    content: Element<'a, M>,
    palette: Palette,
    width: f32,
    on_close: Option<ShellMessage>,
) -> Element<'a, M>
where
    M: From<ShellMessage> + Clone + 'a,
{
    let mut header_row = row![
        text(title)
            .size(tokens::FONT_SIZE_SM)
            .color(palette.text_primary),
        Space::new().width(Length::Fill),
    ]
    .align_y(Alignment::Center)
    .padding([0.0, tokens::SPACING_SM]);

    if let Some(msg) = on_close {
        let close_btn = button(text("✕").size(tokens::FONT_SIZE_XS))
            .padding([tokens::SPACING_XXS, tokens::SPACING_XS])
            .on_press(M::from(msg))
            .style(move |_theme, status| {
                let bg = if status == iced::widget::button::Status::Hovered {
                    palette.surface_hover
                } else {
                    palette.surface
                };
                iced::widget::button::Style {
                    background: Some(bg.into()),
                    text_color: palette.text_muted,
                    border: iced::border::Border::default(),
                    ..Default::default()
                }
            });
        header_row = header_row.push(close_btn);
    }

    let header = container(header_row)
        .height(tokens::PANEL_HEADER_HEIGHT)
        .width(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.surface_alt.into()),
            border: iced::border::Border {
                color: palette.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    let body = scrollable(content).width(Length::Fill).height(Length::Fill);

    container(column![header, body])
        .width(Length::Fixed(width))
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.surface.into()),
            border: iced::border::Border {
                color: palette.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
