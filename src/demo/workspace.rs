//! Technical Workspace view for the demo.

use super::state::DemoState;
use crate::shell::theme::{Palette, tokens};
use iced::widget::{Space, column, container, row, text};
use iced::{Alignment, Element, Length};

pub fn view<'a, M: 'a>(state: &'a DemoState, palette: Palette) -> Element<'a, M> {
    let header_info = row![
        text("Technical Viewport")
            .size(tokens::FONT_SIZE_MD)
            .color(palette.text_primary),
        Space::new().width(Length::Fill),
        text("Grid: 10mm | Mode: Preview | Scale: 1:1")
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
    ]
    .align_y(Alignment::Center)
    .padding([tokens::SPACING_SM, tokens::SPACING_MD]);

    let canvas_preview: Element<'a, M> = if let Some(props) = state.selected_properties() {
        let object_box = container(
            column![
                text(&props.name)
                    .size(tokens::FONT_SIZE_SM)
                    .color(palette.text_primary),
                text(format!("Type: {}", props.item_type))
                    .size(tokens::FONT_SIZE_XS)
                    .color(palette.text_secondary),
                text(format!(
                    "Pos: ({:.1}, {:.1})",
                    props.transform_x, props.transform_y
                ))
                .size(tokens::FONT_SIZE_XS)
                .color(palette.accent),
                text(if props.enabled {
                    "Status: Active"
                } else {
                    "Status: Inactive"
                })
                .size(tokens::FONT_SIZE_XS)
                .color(if props.enabled {
                    palette.success
                } else {
                    palette.warning
                }),
            ]
            .spacing(tokens::SPACING_XS)
            .padding(tokens::SPACING_MD)
            .align_x(Alignment::Center),
        )
        .width(220)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.surface.into()),
            border: iced::border::Border {
                color: palette.accent,
                width: 1.5,
                radius: tokens::RADIUS_MD.into(),
            },
            ..Default::default()
        });

        column![
            text("Selected Entity Projection")
                .size(tokens::FONT_SIZE_SM)
                .color(palette.text_muted),
            Space::new()
                .width(Length::Fixed(0.0))
                .height(tokens::SPACING_MD),
            object_box,
            Space::new()
                .width(Length::Fixed(0.0))
                .height(tokens::SPACING_MD),
            text("Interact using Inspector to adjust coordinates and state.")
                .size(tokens::FONT_SIZE_XS)
                .color(palette.text_muted),
        ]
        .align_x(Alignment::Center)
        .into()
    } else {
        column![
            text("No Entity Selected")
                .size(tokens::FONT_SIZE_MD)
                .color(palette.text_muted),
            text("Select a component from the Project Explorer to inspect.")
                .size(tokens::FONT_SIZE_SM)
                .color(palette.text_muted),
        ]
        .align_x(Alignment::Center)
        .spacing(tokens::SPACING_SM)
        .into()
    };

    let centered_canvas = container(canvas_preview)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_| iced::widget::container::Style {
            background: Some(palette.background.into()),
            border: iced::border::Border {
                color: palette.border_subtle,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        });

    column![header_info, centered_canvas]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}
