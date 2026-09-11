//! Technical Desktop Shell Demo module.

pub mod commands;
pub mod explorer;
pub mod inspector;
pub mod message;
pub mod state;
pub mod workspace;

pub use commands::{
    APP_NEW, APP_OPEN, APP_QUIT, APP_SAVE, HELP_ABOUT, VIEW_TOGGLE_BOTTOM_PANEL,
    VIEW_TOGGLE_EXPLORER, VIEW_TOGGLE_INSPECTOR, VIEW_TOGGLE_THEME, register_demo_commands,
};
pub use explorer::{PANEL_EXPLORER, view as explorer_view};
pub use inspector::{PANEL_INSPECTOR, PANEL_OUTPUT, view as inspector_view};
pub use message::DemoMessage;
pub use state::{DemoItemId, DemoState};
pub use workspace::view as workspace_view;

use crate::shell::ribbon::{RibbonGroup, RibbonItem, RibbonTab};

/// Build demo-specific ribbon tabs.
pub fn demo_ribbon_tabs() -> Vec<RibbonTab> {
    vec![
        RibbonTab::new(
            "home",
            "Home",
            vec![
                RibbonGroup::new(
                    "Project",
                    vec![
                        RibbonItem::button(APP_NEW),
                        RibbonItem::button(APP_OPEN),
                        RibbonItem::button(APP_SAVE),
                    ],
                ),
                RibbonGroup::new(
                    "Panels",
                    vec![
                        RibbonItem::button(VIEW_TOGGLE_EXPLORER),
                        RibbonItem::button(VIEW_TOGGLE_INSPECTOR),
                        RibbonItem::button(VIEW_TOGGLE_BOTTOM_PANEL),
                    ],
                ),
                RibbonGroup::new("Appearance", vec![RibbonItem::button(VIEW_TOGGLE_THEME)]),
            ],
        ),
        RibbonTab::new(
            "view",
            "View",
            vec![RibbonGroup::new(
                "Windows",
                vec![
                    RibbonItem::button_with(VIEW_TOGGLE_EXPLORER, "Explorer", "📁"),
                    RibbonItem::button_with(VIEW_TOGGLE_INSPECTOR, "Inspector", "🔍"),
                    RibbonItem::button_with(VIEW_TOGGLE_BOTTOM_PANEL, "Output", "🖥️"),
                ],
            )],
        ),
        RibbonTab::new(
            "help",
            "Help",
            vec![RibbonGroup::new(
                "About",
                vec![RibbonItem::button(HELP_ABOUT)],
            )],
        ),
    ]
}
