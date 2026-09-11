//! Compact horizontal toolbar strip driving application commands.
//!
//! Domain-agnostic: items carry an opaque id, an optional icon/label pair,
//! and the [`CommandId`] to execute. The shell never interprets the commands.

use super::command::CommandId;
use super::message::ShellMessage;
use super::theme::{Palette, tokens};
use iced::widget::{Row, button, container, text};
use iced::{Alignment, Element, Length};

/// Single entry in the toolbar strip.
#[derive(Debug, Clone)]
pub struct ToolbarItem {
    pub id: String,
    pub label: Option<String>,
    pub icon: Option<String>,
    pub command: CommandId,
}

impl ToolbarItem {
    pub fn new(id: impl Into<String>, command: impl Into<CommandId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            icon: None,
            command: command.into(),
        }
    }

    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

fn caption(item: &ToolbarItem) -> String {
    match (&item.icon, &item.label) {
        (Some(icon), Some(label)) => format!("{icon} {label}"),
        (Some(icon), None) => icon.clone(),
        (None, Some(label)) => label.clone(),
        (None, None) => item.id.clone(),
    }
}

/// Renders the horizontal toolbar strip from a slice of items.
///
/// The returned element owns its data and does not borrow `items`,
/// so callers may pass a locally-built slice.
pub fn view<'a>(items: &[ToolbarItem], palette: Palette) -> Element<'a, ShellMessage> {
    let mut bar = Row::new()
        .spacing(tokens::SPACING_XS)
        .align_y(Alignment::Center)
        .padding([tokens::SPACING_XS, tokens::SPACING_SM]);

    for item in items {
        let cmd_id = item.command.clone();
        let btn = button(
            text(caption(item))
                .size(tokens::FONT_SIZE_SM)
                .color(palette.text_primary),
        )
        .padding([tokens::SPACING_XXS, tokens::SPACING_SM])
        .on_press(ShellMessage::ExecuteCommand(cmd_id))
        .style(move |_theme, status| {
            let bg = if status == iced::widget::button::Status::Hovered {
                palette.surface_hover
            } else {
                palette.surface
            };
            iced::widget::button::Style {
                background: Some(bg.into()),
                text_color: palette.text_primary,
                border: iced::border::Border {
                    color: palette.border_subtle,
                    width: 1.0,
                    radius: tokens::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        });

        bar = bar.push(btn);
    }

    container(bar)
        .width(Length::Fill)
        .height(tokens::TOOLBAR_HEIGHT + tokens::SPACING_SM)
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
