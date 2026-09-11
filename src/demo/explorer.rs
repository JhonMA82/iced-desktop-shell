//! Project Explorer view for the demo.

use super::message::DemoMessage;
use super::state::{DemoItemId, DemoState};
use desktop_shell::ShellMessage;
use desktop_shell::theme::{Palette, tokens};
use iced::widget::{Space, button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

pub const PANEL_EXPLORER: &str = "demo.explorer";

/// Renders the demo project explorer panel.
pub fn view<'a, M>(
    state: &'a DemoState,
    palette: Palette,
    width: f32,
    on_close: Option<ShellMessage>,
) -> Element<'a, M>
where
    M: From<DemoMessage> + From<ShellMessage> + Clone + 'a,
{
    let mut header_row = row![
        text("Project Explorer")
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

    let project_root = text("📦 Project Root")
        .size(tokens::FONT_SIZE_SM)
        .color(palette.text_secondary);

    let components_header = text("  ▾ 📁 Components")
        .size(tokens::FONT_SIZE_SM)
        .color(palette.text_secondary);

    let item_a = render_tree_item(DemoItemId::ComponentA, state, palette);
    let item_b = render_tree_item(DemoItemId::ComponentB, state, palette);

    let resources_header = text("  ▾ 📁 Resources")
        .size(tokens::FONT_SIZE_SM)
        .color(palette.text_secondary);

    let res_1 = render_tree_item(DemoItemId::Resource1, state, palette);

    let settings_item = render_tree_item(DemoItemId::Settings, state, palette);

    let tree_content = column![
        project_root,
        components_header,
        item_a,
        item_b,
        resources_header,
        res_1,
        text("  ▾ ⚙️ Settings")
            .size(tokens::FONT_SIZE_SM)
            .color(palette.text_secondary),
        settings_item,
    ]
    .spacing(tokens::SPACING_XS)
    .padding(tokens::SPACING_SM);

    let scroll = scrollable(tree_content)
        .width(Length::Fill)
        .height(Length::Fill);

    container(column![header, scroll])
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

fn render_tree_item<'a, M>(
    item_id: DemoItemId,
    state: &'a DemoState,
    palette: Palette,
) -> Element<'a, M>
where
    M: From<DemoMessage> + Clone + 'a,
{
    let is_selected = state.selected_item.as_ref() == Some(&item_id);
    let icon = match item_id {
        DemoItemId::ComponentA | DemoItemId::ComponentB => "    🧩",
        DemoItemId::Resource1 => "    📄",
        DemoItemId::Settings => "    🔧",
    };

    button(
        row![
            text(icon).size(tokens::FONT_SIZE_XS),
            text(format!(" {}", item_id.label()))
                .size(tokens::FONT_SIZE_SM)
                .color(if is_selected {
                    palette.accent
                } else {
                    palette.text_primary
                }),
        ]
        .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .padding([tokens::SPACING_XXS, tokens::SPACING_SM])
    .on_press(M::from(DemoMessage::SelectItem(item_id)))
    .style(move |_theme, status| {
        let bg = if is_selected {
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
                color: if is_selected {
                    palette.accent
                } else {
                    palette.border_subtle
                },
                width: if is_selected { 1.0 } else { 0.0 },
                radius: tokens::RADIUS_SM.into(),
            },
            ..Default::default()
        }
    })
    .into()
}
