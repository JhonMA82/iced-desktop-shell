//! Application view combining Shell framing with Demo panels.

use super::message::Message;
use super::state::AppState;
use crate::demo;
use crate::demo::{PANEL_EXPLORER, PANEL_INSPECTOR, PANEL_OUTPUT};
use crate::shell::menu::MenuItem;
use crate::shell::panel::{PanelId, PanelLocation};
use crate::shell::theme::tokens;
use crate::shell::{ShellMessage, render_shell};
use iced::widget::{Column, Space, button, column, row, scrollable, text};
use iced::{Alignment, Element, Length};

pub fn view(state: &AppState) -> Element<'_, Message> {
    let palette = state.shell.theme.palette();

    // Menu items
    let menu_items = [
        MenuItem::new("File: New", demo::APP_NEW),
        MenuItem::new("File: Open", demo::APP_OPEN),
        MenuItem::new("File: Save", demo::APP_SAVE),
        MenuItem::new("View: Explorer", demo::VIEW_TOGGLE_EXPLORER),
        MenuItem::new("View: Inspector", demo::VIEW_TOGGLE_INSPECTOR),
        MenuItem::new("View: Output", demo::VIEW_TOGGLE_BOTTOM_PANEL),
        MenuItem::new("Theme", demo::VIEW_TOGGLE_THEME),
        MenuItem::new("Help: About", demo::HELP_ABOUT),
    ];

    // Build panel views strictly following DockLayout.order
    let mut left_panels = Vec::new();
    let mut right_panels = Vec::new();
    let mut bottom_panels = Vec::new();

    for panel in state.shell.dock.ordered_panels() {
        if panel.visible
            && let Some(view) = resolve_panel_view(state, &panel.id, panel.size, palette)
        {
            match panel.location {
                PanelLocation::Left => left_panels.push(view),
                PanelLocation::Right => right_panels.push(view),
                PanelLocation::Bottom => bottom_panels.push(view),
            }
        }
    }

    let workspace = demo::workspace_view(&state.demo, palette);

    render_shell(
        &state.shell,
        menu_items.as_slice(),
        left_panels,
        right_panels,
        bottom_panels,
        workspace,
    )
}

fn resolve_panel_view<'a>(
    state: &'a AppState,
    panel_id: &PanelId,
    size: f32,
    palette: crate::shell::theme::Palette,
) -> Option<Element<'a, Message>> {
    match panel_id.as_str() {
        PANEL_EXPLORER => Some(demo::explorer_view(
            &state.demo,
            palette,
            size,
            Some(ShellMessage::TogglePanel(panel_id.clone())),
        )),
        PANEL_INSPECTOR => {
            let content = demo::inspector_view(&state.demo, palette);
            Some(crate::shell::inspector::view(
                "Properties",
                content,
                palette,
                size,
                Some(ShellMessage::TogglePanel(panel_id.clone())),
            ))
        }
        PANEL_OUTPUT => {
            let output_content = render_bottom_content(state, palette);
            Some(crate::shell::bottom_panel::view(
                &state.shell.bottom_panel,
                output_content,
                palette,
                size,
                Some(ShellMessage::TogglePanel(panel_id.clone())),
            ))
        }
        _ => None,
    }
}

fn render_bottom_content<'a>(
    state: &'a AppState,
    palette: crate::shell::theme::Palette,
) -> Element<'a, Message> {
    let mut log_items = Column::new()
        .spacing(tokens::SPACING_XXS)
        .padding(tokens::SPACING_SM);

    for log in &state.demo.logs {
        log_items = log_items.push(
            text(log)
                .size(tokens::FONT_SIZE_XS)
                .color(palette.text_secondary),
        );
    }

    let toolbar = row![
        text(format!("Total entries: {}", state.demo.logs.len()))
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
        Space::new().width(Length::Fill),
        button(text("Clear").size(tokens::FONT_SIZE_XS))
            .padding([tokens::SPACING_XXS, tokens::SPACING_SM])
            .on_press(Message::Demo(demo::DemoMessage::ClearLogs))
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
            }),
    ]
    .align_y(Alignment::Center)
    .padding([tokens::SPACING_XXS, tokens::SPACING_SM]);

    let scroll = scrollable(log_items)
        .width(Length::Fill)
        .height(Length::Fill);

    column![toolbar, scroll]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
