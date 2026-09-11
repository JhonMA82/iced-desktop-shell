//! Narrow vertical activity rail driving application commands.
//!
//! Domain-agnostic: items carry an opaque id, an icon glyph, and the
//! [`CommandId`] to execute. The shell never interprets what an item means.

use super::command::CommandId;
use super::message::ShellMessage;
use super::theme::{Palette, tokens};
use iced::widget::{Column, button, container, text};
use iced::{Element, Length};

/// Single icon entry in the activity rail.
#[derive(Debug, Clone)]
pub struct ActivityItem {
    pub id: String,
    pub icon: String,
    pub command: CommandId,
}

impl ActivityItem {
    pub fn new(
        id: impl Into<String>,
        icon: impl Into<String>,
        command: impl Into<CommandId>,
    ) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            command: command.into(),
        }
    }
}

/// Renders the vertical activity rail from a slice of items.
///
/// `active` selects the highlighted item by id. An unknown or empty `active`
/// highlights nothing. The returned element owns its data and does not borrow
/// `items`, so callers may pass a locally-built slice.
pub fn view<'a>(
    items: &[ActivityItem],
    active: &str,
    palette: Palette,
) -> Element<'a, ShellMessage> {
    let mut rail = Column::new()
        .spacing(tokens::SPACING_XS)
        .padding(tokens::SPACING_XS);

    for item in items {
        let is_active = item.id == active;
        let cmd_id = item.command.clone();
        let btn = button(
            text(item.icon.clone())
                .size(tokens::FONT_SIZE_MD)
                .color(if is_active {
                    palette.accent
                } else {
                    palette.text_secondary
                }),
        )
        .width(Length::Fixed(tokens::TOOLBAR_HEIGHT))
        .height(Length::Fixed(tokens::TOOLBAR_HEIGHT))
        .on_press(ShellMessage::ExecuteCommand(cmd_id))
        .style(move |_theme, status| {
            let bg = if is_active {
                palette.surface_active
            } else if status == iced::widget::button::Status::Hovered {
                palette.surface_hover
            } else {
                palette.surface
            };
            iced::widget::button::Style {
                background: Some(bg.into()),
                text_color: palette.text_primary,
                border: iced::border::Border {
                    color: if is_active {
                        palette.accent
                    } else {
                        palette.border_subtle
                    },
                    width: if is_active { 1.5 } else { 0.0 },
                    radius: tokens::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        });

        rail = rail.push(btn);
    }

    container(rail)
        .width(Length::Fixed(tokens::TOOLBAR_HEIGHT + tokens::SPACING_SM))
        .height(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.surface_alt.into()),
            border: iced::border::Border {
                color: palette.border,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        })
        .into()
}
