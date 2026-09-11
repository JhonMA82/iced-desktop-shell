//! Top menu bar offering access to application commands.

use super::command::CommandId;
use super::message::ShellMessage;
use super::theme::{Palette, tokens};
use iced::widget::{Row, button, text};
use iced::{Alignment, Element};

/// Menu item representation for the menu bar.
#[derive(Debug, Clone)]
pub struct MenuItem {
    pub label: String,
    pub command_id: CommandId,
}

impl MenuItem {
    pub fn new(label: impl Into<String>, command_id: impl Into<CommandId>) -> Self {
        Self {
            label: label.into(),
            command_id: command_id.into(),
        }
    }
}

/// Renders the compact top menu bar from a slice of menu items.
///
/// The returned element owns its label/command data and does not borrow `items`,
/// so callers may pass a locally-built slice.
pub fn view<'a>(items: &[MenuItem], palette: Palette) -> Element<'a, ShellMessage> {
    let mut menu_row = Row::new()
        .spacing(tokens::SPACING_XS)
        .align_y(Alignment::Center);

    for item in items {
        let cmd_id = item.command_id.clone();
        let btn = button(
            text(item.label.clone())
                .size(tokens::FONT_SIZE_XS)
                .color(palette.text_secondary),
        )
        .padding([tokens::SPACING_XXS, tokens::SPACING_SM])
        .on_press(ShellMessage::ExecuteCommand(cmd_id))
        .style(move |_theme, status| {
            let bg = if status == iced::widget::button::Status::Hovered {
                palette.surface_hover
            } else {
                palette.surface_alt
            };
            iced::widget::button::Style {
                background: Some(bg.into()),
                text_color: palette.text_primary,
                border: iced::border::Border::default(),
                ..Default::default()
            }
        });

        menu_row = menu_row.push(btn);
    }

    menu_row.into()
}
