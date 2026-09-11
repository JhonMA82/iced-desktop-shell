//! Color palettes and styling helpers for dark and light themes.

use iced::{Color, border};

/// Color palette definition for shell elements.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub background: Color,
    pub surface: Color,
    pub surface_hover: Color,
    pub surface_active: Color,
    pub surface_alt: Color,
    pub border: Color,
    pub border_subtle: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_contrast: Color,
    pub status_bar_bg: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
}

pub const DARK_PALETTE: Palette = Palette {
    background: Color::from_rgb(0.11, 0.12, 0.14),
    surface: Color::from_rgb(0.14, 0.15, 0.18),
    surface_hover: Color::from_rgb(0.18, 0.20, 0.24),
    surface_active: Color::from_rgb(0.22, 0.25, 0.30),
    surface_alt: Color::from_rgb(0.12, 0.13, 0.15),
    border: Color::from_rgb(0.20, 0.22, 0.27),
    border_subtle: Color::from_rgb(0.16, 0.17, 0.21),
    text_primary: Color::from_rgb(0.92, 0.93, 0.95),
    text_secondary: Color::from_rgb(0.70, 0.73, 0.78),
    text_muted: Color::from_rgb(0.48, 0.51, 0.57),
    accent: Color::from_rgb(0.20, 0.50, 0.90),
    accent_hover: Color::from_rgb(0.28, 0.58, 0.98),
    accent_contrast: Color::WHITE,
    status_bar_bg: Color::from_rgb(0.09, 0.10, 0.12),
    success: Color::from_rgb(0.25, 0.75, 0.45),
    warning: Color::from_rgb(0.95, 0.65, 0.20),
    error: Color::from_rgb(0.90, 0.30, 0.30),
};

pub const LIGHT_PALETTE: Palette = Palette {
    background: Color::from_rgb(0.95, 0.96, 0.98),
    surface: Color::from_rgb(1.0, 1.0, 1.0),
    surface_hover: Color::from_rgb(0.92, 0.93, 0.96),
    surface_active: Color::from_rgb(0.86, 0.88, 0.92),
    surface_alt: Color::from_rgb(0.97, 0.98, 0.99),
    border: Color::from_rgb(0.82, 0.85, 0.89),
    border_subtle: Color::from_rgb(0.90, 0.92, 0.95),
    text_primary: Color::from_rgb(0.12, 0.14, 0.18),
    text_secondary: Color::from_rgb(0.35, 0.38, 0.45),
    text_muted: Color::from_rgb(0.55, 0.58, 0.65),
    accent: Color::from_rgb(0.12, 0.42, 0.85),
    accent_hover: Color::from_rgb(0.18, 0.48, 0.92),
    accent_contrast: Color::WHITE,
    status_bar_bg: Color::from_rgb(0.88, 0.90, 0.94),
    success: Color::from_rgb(0.18, 0.65, 0.35),
    warning: Color::from_rgb(0.85, 0.55, 0.10),
    error: Color::from_rgb(0.82, 0.22, 0.22),
};

/// Helper to create container styling supporting Iced 0.14 fields via `..Default::default()`.
pub fn container_style(
    bg: Color,
    border_color: Color,
    border_width: f32,
    radius: f32,
) -> iced::widget::container::Style {
    iced::widget::container::Style {
        background: Some(bg.into()),
        border: border::Border {
            color: border_color,
            width: border_width,
            radius: radius.into(),
        },
        ..Default::default()
    }
}
