//! Visual layout assembler for the desktop shell.
//!
//! Consumes panel elements by ownership and builds the top-level desktop tree.

use super::menu::MenuItem;
use super::message::ShellMessage;
use super::state::ShellState;
use super::theme::tokens;
use iced::widget::{Row, Space, button, column, container, text};
use iced::{Alignment, Element, Length};

/// Renders the shell layout consuming panels by ownership.
pub fn render_shell<'a, 'b, M>(
    shell: &'a ShellState,
    menu_items: &'b [MenuItem],
    left_panels: Vec<Element<'a, M>>,
    right_panels: Vec<Element<'a, M>>,
    bottom_panels: Vec<Element<'a, M>>,
    workspace_view: Element<'a, M>,
) -> Element<'a, M>
where
    M: From<ShellMessage> + Clone + 'a,
{
    let palette = shell.theme.palette();

    // 1. Menu bar
    let menu_bar = super::menu::view(menu_items, palette).map(M::from);

    // 2. Ribbon
    let ribbon = super::ribbon::view::view(
        &shell.ribbon_tabs,
        &shell.active_ribbon_tab,
        &shell.commands,
        palette,
    )
    .map(M::from);

    // 3. Middle area: Left panels + Center workspace + Right panels (consumed by ownership)
    let mut middle_row = Row::new().width(Length::Fill).height(Length::Fill);

    for panel in left_panels {
        middle_row = middle_row.push(panel);
    }

    middle_row = middle_row.push(
        container(workspace_view)
            .width(Length::Fill)
            .height(Length::Fill),
    );

    for panel in right_panels {
        middle_row = middle_row.push(panel);
    }

    // 4. Central column: Middle row + Bottom panels (consumed by ownership)
    let mut central_col = column![middle_row].width(Length::Fill).height(Length::Fill);

    for panel in bottom_panels {
        central_col = central_col.push(panel);
    }

    // 5. Status bar
    let status_bar = super::status_bar::view(&shell.status_bar, palette);

    // 6. Complete shell stack
    let main_content = column![menu_bar, ribbon, central_col, status_bar,]
        .width(Length::Fill)
        .height(Length::Fill);

    if shell.show_about_dialog {
        render_modal_overlay(main_content.into(), render_about_dialog(palette))
    } else {
        main_content.into()
    }
}

fn render_about_dialog<'a, M>(palette: super::theme::Palette) -> Element<'a, M>
where
    M: From<ShellMessage> + Clone + 'a,
{
    let dialog = column![
        text("About iced-desktop-shell")
            .size(tokens::FONT_SIZE_LG)
            .color(palette.text_primary),
        text("An experimental technical desktop application boilerplate for Rust & Iced.")
            .size(tokens::FONT_SIZE_SM)
            .color(palette.text_secondary),
        text("License: MIT OR Apache-2.0")
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
        text("Architecture: State / Message / Update / View with clean App/Shell separation.")
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
        Space::new().width(Length::Fill).height(tokens::SPACING_MD),
        button(text("Close").size(tokens::FONT_SIZE_SM))
            .padding([tokens::SPACING_XS, tokens::SPACING_LG])
            .on_press(M::from(ShellMessage::CloseAboutDialog))
            .style(move |_theme, status| {
                let bg = if status == iced::widget::button::Status::Hovered {
                    palette.accent_hover
                } else {
                    palette.accent
                };
                iced::widget::button::Style {
                    background: Some(bg.into()),
                    text_color: palette.accent_contrast,
                    border: iced::border::Border {
                        color: palette.border,
                        width: 1.0,
                        radius: tokens::RADIUS_SM.into(),
                    },
                    ..Default::default()
                }
            })
    ]
    .align_x(Alignment::Center)
    .width(Length::Fill)
    .spacing(tokens::SPACING_SM)
    .padding(tokens::SPACING_LG);

    container(dialog)
        .width(380)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.surface.into()),
            border: iced::border::Border {
                color: palette.border,
                width: 1.5,
                radius: tokens::RADIUS_MD.into(),
            },
            ..Default::default()
        })
        .into()
}

fn render_modal_overlay<'a, M>(base: Element<'a, M>, modal: Element<'a, M>) -> Element<'a, M>
where
    M: Clone + 'a,
{
    let overlay = container(modal)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill);

    iced::widget::stack![base, overlay].into()
}
