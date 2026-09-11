//! Bottom panel container (Output, Console, Tasks).

use super::message::ShellMessage;
use super::theme::{Palette, tokens};
use iced::widget::{Row, Space, button, column, container, text};
use iced::{Alignment, Element, Length};

/// State of the bottom panel.
#[derive(Debug, Clone)]
pub struct BottomPanelState {
    pub active_tab: String,
    pub tabs: Vec<String>,
}

impl Default for BottomPanelState {
    fn default() -> Self {
        Self {
            active_tab: "Output".to_string(),
            tabs: vec![
                "Output".to_string(),
                "Console".to_string(),
                "Diagnostics".to_string(),
            ],
        }
    }
}

/// Renders the bottom panel container wrapping caller-provided content.
pub fn view<'a, M>(
    state: &'a BottomPanelState,
    content: Element<'a, M>,
    palette: Palette,
    height: f32,
    on_close: Option<ShellMessage>,
) -> Element<'a, M>
where
    M: From<ShellMessage> + Clone + 'a,
{
    let mut header_row = Row::new()
        .spacing(tokens::SPACING_XXS)
        .align_y(Alignment::Center);

    for tab in &state.tabs {
        let is_active = tab == &state.active_tab;
        let tab_name = tab.clone();
        let btn = button(text(tab).size(tokens::FONT_SIZE_XS).color(if is_active {
            palette.text_primary
        } else {
            palette.text_secondary
        }))
        .padding([tokens::SPACING_XXS, tokens::SPACING_SM])
        .on_press(M::from(ShellMessage::SelectBottomTab(tab_name)))
        .style(move |_theme, status| {
            let bg = if is_active {
                palette.surface
            } else if status == iced::widget::button::Status::Hovered {
                palette.surface_hover
            } else {
                palette.surface_alt
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

        header_row = header_row.push(btn);
    }

    header_row = header_row.push(Space::new().width(Length::Fill));

    if let Some(msg) = on_close {
        let close_btn = button(text("✕").size(tokens::FONT_SIZE_XS))
            .padding([tokens::SPACING_XXS, tokens::SPACING_XS])
            .on_press(M::from(msg))
            .style(move |_theme, status| {
                let bg = if status == iced::widget::button::Status::Hovered {
                    palette.surface_hover
                } else {
                    palette.surface_alt
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

    container(column![header, content])
        .height(Length::Fixed(height))
        .width(Length::Fill)
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
