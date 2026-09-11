//! Design tokens for the desktop shell.
//!
//! Centralizes spacing, radii, dimensions, and typography metrics to prevent
//! magic numbers from leaking into individual widget implementations.

/// Spacing scale (in logical pixels)
pub const SPACING_XXS: f32 = 2.0;
pub const SPACING_XS: f32 = 4.0;
pub const SPACING_SM: f32 = 8.0;
pub const SPACING_MD: f32 = 12.0;
pub const SPACING_LG: f32 = 16.0;
pub const SPACING_XL: f32 = 24.0;

/// Corner radius tokens
pub const RADIUS_NONE: f32 = 0.0;
pub const RADIUS_SM: f32 = 3.0;
pub const RADIUS_MD: f32 = 6.0;
pub const RADIUS_LG: f32 = 8.0;

/// Structural layout dimensions
pub const TOOLBAR_HEIGHT: f32 = 32.0;
pub const MENU_BAR_HEIGHT: f32 = 28.0;
pub const RIBBON_TAB_HEIGHT: f32 = 28.0;
pub const RIBBON_HEIGHT: f32 = 96.0;
pub const STATUSBAR_HEIGHT: f32 = 20.0;
pub const PANEL_HEADER_HEIGHT: f32 = 28.0;

/// Panel sizing tokens
pub const SIDEBAR_MIN_WIDTH: f32 = 180.0;
pub const SIDEBAR_DEFAULT_WIDTH: f32 = 250.0;
pub const INSPECTOR_MIN_WIDTH: f32 = 220.0;
pub const INSPECTOR_DEFAULT_WIDTH: f32 = 280.0;
pub const BOTTOM_PANEL_MIN_HEIGHT: f32 = 120.0;
pub const BOTTOM_PANEL_DEFAULT_HEIGHT: f32 = 190.0;

/// Font size tokens
pub const FONT_SIZE_XS: f32 = 11.0;
pub const FONT_SIZE_SM: f32 = 12.0;
pub const FONT_SIZE_BASE: f32 = 13.0;
pub const FONT_SIZE_MD: f32 = 14.0;
pub const FONT_SIZE_LG: f32 = 16.0;
