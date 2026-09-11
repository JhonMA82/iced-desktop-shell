//! Inspector Property Editor for the demo.

use super::message::DemoMessage;
use super::state::DemoState;
use crate::shell::theme::{Palette, tokens};
use iced::widget::{button, column, container, row, rule, text};
use iced::{Alignment, Element};

pub const PANEL_INSPECTOR: &str = "demo.inspector";
pub const PANEL_OUTPUT: &str = "demo.output";

pub fn view<'a, M>(state: &'a DemoState, palette: Palette) -> Element<'a, M>
where
    M: From<DemoMessage> + Clone + 'a,
{
    let Some(item_id) = &state.selected_item else {
        return container(
            text("Select an element to view properties")
                .size(tokens::FONT_SIZE_SM)
                .color(palette.text_muted),
        )
        .padding(tokens::SPACING_MD)
        .into();
    };

    let Some(props) = state.properties.get(item_id) else {
        return container(text("No properties available").size(tokens::FONT_SIZE_SM)).into();
    };

    let item_id_for_toggle = item_id.clone();
    let item_id_for_inc_x = item_id.clone();
    let item_id_for_dec_x = item_id.clone();
    let item_id_for_inc_y = item_id.clone();
    let item_id_for_dec_y = item_id.clone();

    // 1. General Section
    let general_section = column![
        text("GENERAL")
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
        property_row(
            "Name",
            text(&props.name)
                .size(tokens::FONT_SIZE_SM)
                .color(palette.text_primary)
                .into(),
            palette,
        ),
        property_row(
            "Type",
            text(&props.item_type)
                .size(tokens::FONT_SIZE_SM)
                .color(palette.text_secondary)
                .into(),
            palette,
        ),
        property_row(
            "Enabled",
            button(
                text(if props.enabled {
                    "Active (Yes)"
                } else {
                    "Disabled (No)"
                })
                .size(tokens::FONT_SIZE_XS)
                .color(if props.enabled {
                    palette.success
                } else {
                    palette.warning
                }),
            )
            .padding([tokens::SPACING_XXS, tokens::SPACING_SM])
            .on_press(M::from(DemoMessage::ToggleEnabled(item_id_for_toggle)))
            .style(move |_theme, status| {
                let bg = if status == iced::widget::button::Status::Hovered {
                    palette.surface_hover
                } else {
                    palette.surface_alt
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
            })
            .into(),
            palette,
        ),
    ]
    .spacing(tokens::SPACING_XS);

    // 2. Transform Section
    let transform_section = column![
        text("TRANSFORM")
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
        property_row(
            "X (mm)",
            row![
                text(format!("{:.1}", props.transform_x))
                    .size(tokens::FONT_SIZE_SM)
                    .color(palette.text_primary),
                button(text("-").size(tokens::FONT_SIZE_XS))
                    .padding([tokens::SPACING_XXS, tokens::SPACING_XS])
                    .on_press(M::from(DemoMessage::DecrementX(item_id_for_dec_x))),
                button(text("+").size(tokens::FONT_SIZE_XS))
                    .padding([tokens::SPACING_XXS, tokens::SPACING_XS])
                    .on_press(M::from(DemoMessage::IncrementX(item_id_for_inc_x))),
            ]
            .spacing(tokens::SPACING_XS)
            .align_y(Alignment::Center)
            .into(),
            palette,
        ),
        property_row(
            "Y (mm)",
            row![
                text(format!("{:.1}", props.transform_y))
                    .size(tokens::FONT_SIZE_SM)
                    .color(palette.text_primary),
                button(text("-").size(tokens::FONT_SIZE_XS))
                    .padding([tokens::SPACING_XXS, tokens::SPACING_XS])
                    .on_press(M::from(DemoMessage::DecrementY(item_id_for_dec_y))),
                button(text("+").size(tokens::FONT_SIZE_XS))
                    .padding([tokens::SPACING_XXS, tokens::SPACING_XS])
                    .on_press(M::from(DemoMessage::IncrementY(item_id_for_inc_y))),
            ]
            .spacing(tokens::SPACING_XS)
            .align_y(Alignment::Center)
            .into(),
            palette,
        ),
    ]
    .spacing(tokens::SPACING_XS);

    // 3. Metadata Section
    let metadata_section = column![
        text("METADATA")
            .size(tokens::FONT_SIZE_XS)
            .color(palette.text_muted),
        property_row(
            "Created",
            text(&props.created_at)
                .size(tokens::FONT_SIZE_XS)
                .color(palette.text_secondary)
                .into(),
            palette,
        ),
        property_row(
            "Modified",
            text(&props.modified_at)
                .size(tokens::FONT_SIZE_XS)
                .color(palette.text_secondary)
                .into(),
            palette,
        ),
    ]
    .spacing(tokens::SPACING_XS);

    column![
        general_section,
        rule::horizontal(1),
        transform_section,
        rule::horizontal(1),
        metadata_section,
    ]
    .spacing(tokens::SPACING_MD)
    .padding(tokens::SPACING_SM)
    .into()
}

fn property_row<'a, M>(
    label: &'static str,
    value: Element<'a, M>,
    palette: Palette,
) -> Element<'a, M>
where
    M: Clone + 'a,
{
    row![
        container(
            text(label)
                .size(tokens::FONT_SIZE_XS)
                .color(palette.text_secondary)
        )
        .width(80),
        value,
    ]
    .align_y(Alignment::Center)
    .into()
}
