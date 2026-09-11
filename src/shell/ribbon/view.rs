//! View rendering for the data-driven Ribbon component.

use super::super::command::CommandRegistry;
use super::super::message::ShellMessage;
use super::super::theme::{Palette, tokens};
use super::model::{RibbonGroup, RibbonItem, RibbonTab};
use iced::widget::{Row, Space, button, column, container, row, rule, text};
use iced::{Alignment, Element, Length};

/// Render the complete ribbon interface.
pub fn view<'a>(
    tabs: &'a [RibbonTab],
    active_tab_index: usize,
    commands: &'a CommandRegistry,
    palette: Palette,
) -> Element<'a, ShellMessage> {
    if tabs.is_empty() {
        return Space::new()
            .width(Length::Fill)
            .height(Length::Fixed(0.0))
            .into();
    }

    // 1. Tab bar header
    let tab_bar = row(tabs.iter().enumerate().map(|(idx, tab)| {
        let is_active = idx == active_tab_index;
        button(
            text(&tab.label)
                .size(tokens::FONT_SIZE_SM)
                .color(if is_active {
                    palette.text_primary
                } else {
                    palette.text_secondary
                }),
        )
        .padding([tokens::SPACING_XS, tokens::SPACING_MD])
        .on_press(ShellMessage::SelectRibbonTab(idx))
        .style(move |_theme, status| {
            let bg = if is_active {
                palette.surface
            } else if status == iced::widget::button::Status::Hovered {
                palette.surface_hover
            } else {
                palette.surface_alt
            };
            let border_color = if is_active {
                palette.accent
            } else {
                palette.border_subtle
            };
            iced::widget::button::Style {
                background: Some(bg.into()),
                text_color: palette.text_primary,
                border: iced::border::Border {
                    color: border_color,
                    width: if is_active { 1.5 } else { 0.0 },
                    radius: tokens::RADIUS_SM.into(),
                },
                ..Default::default()
            }
        })
        .into()
    }))
    .spacing(tokens::SPACING_XXS)
    .padding([tokens::SPACING_XXS, tokens::SPACING_SM]);

    // 2. Active tab groups
    let groups_content: Element<'a, ShellMessage> =
        if let Some(active_tab) = tabs.get(active_tab_index) {
            let mut group_widgets = Vec::new();

            for (i, group) in active_tab.groups.iter().enumerate() {
                if i > 0 {
                    group_widgets.push(
                        rule::vertical(1)
                            .style(move |_| iced::widget::rule::Style {
                                color: palette.border_subtle,
                                radius: 0.0.into(),
                                fill_mode: iced::widget::rule::FillMode::Full,
                                snap: true,
                            })
                            .into(),
                    );
                }
                group_widgets.push(render_group(group, commands, palette));
            }

            row(group_widgets)
                .spacing(tokens::SPACING_SM)
                .padding([tokens::SPACING_XS, tokens::SPACING_SM])
                .height(Length::Fill)
                .align_y(Alignment::Center)
                .into()
        } else {
            Space::new().width(Length::Fill).height(Length::Fill).into()
        };

    container(column![
        tab_bar,
        container(groups_content)
            .width(Length::Fill)
            .height(tokens::RIBBON_HEIGHT - tokens::RIBBON_TAB_HEIGHT)
            .style(move |_| iced::widget::container::Style {
                background: Some(palette.surface.into()),
                border: iced::border::Border {
                    color: palette.border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
    ])
    .width(Length::Fill)
    .height(tokens::RIBBON_HEIGHT)
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

fn render_group<'a>(
    group: &'a RibbonGroup,
    commands: &'a CommandRegistry,
    palette: Palette,
) -> Element<'a, ShellMessage> {
    let mut items_row = Row::new()
        .spacing(tokens::SPACING_XS)
        .align_y(Alignment::Center);

    for item in &group.items {
        match item {
            RibbonItem::Button {
                command_id,
                label_override,
                icon,
            } => {
                let cmd = commands.get(command_id);
                let label = label_override
                    .as_deref()
                    .or_else(|| cmd.map(|c| c.label.as_str()))
                    .unwrap_or_else(|| command_id.as_str());
                let icon_str = icon.or_else(|| cmd.and_then(|c| c.icon)).unwrap_or("⚙️");
                let is_enabled = commands.is_enabled(command_id);

                let btn_content = column![
                    text(icon_str).size(tokens::FONT_SIZE_LG),
                    text(label).size(tokens::FONT_SIZE_XS).color(if is_enabled {
                        palette.text_primary
                    } else {
                        palette.text_muted
                    }),
                ]
                .align_x(Alignment::Center)
                .spacing(tokens::SPACING_XXS);

                let mut btn = button(btn_content)
                    .padding([tokens::SPACING_XS, tokens::SPACING_SM])
                    .style(move |_theme, status| {
                        let bg = if !is_enabled {
                            palette.surface
                        } else if status == iced::widget::button::Status::Hovered {
                            palette.surface_hover
                        } else if status == iced::widget::button::Status::Pressed {
                            palette.surface_active
                        } else {
                            palette.surface
                        };
                        iced::widget::button::Style {
                            background: Some(bg.into()),
                            text_color: if is_enabled {
                                palette.text_primary
                            } else {
                                palette.text_muted
                            },
                            border: iced::border::Border {
                                color: palette.border_subtle,
                                width: 1.0,
                                radius: tokens::RADIUS_SM.into(),
                            },
                            ..Default::default()
                        }
                    });

                if is_enabled {
                    btn = btn.on_press(ShellMessage::ExecuteCommand(command_id.clone()));
                }

                items_row = items_row.push(btn);
            }
            RibbonItem::Separator => {
                items_row =
                    items_row.push(rule::vertical(1).style(move |_| iced::widget::rule::Style {
                        color: palette.border_subtle,
                        radius: 0.0.into(),
                        fill_mode: iced::widget::rule::FillMode::Percent(60.0),
                        snap: true,
                    }));
            }
        }
    }

    column![
        items_row,
        text(&group.title)
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
    ]
    .align_x(Alignment::Center)
    .spacing(tokens::SPACING_XXS)
    .into()
}
