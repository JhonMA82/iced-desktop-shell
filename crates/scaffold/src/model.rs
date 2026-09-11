//! Declarative preset model: layouts, features, defaults, and panel slots.
//!
//! A [`Layout`] is a visual composition (where things go). A [`Feature`] is a
//! capability (what the app can do). Layouts provide default features, but the
//! two concepts stay separate: the same feature (e.g. `explorer`) renders
//! through different domain slot names depending on the layout.
//!
//! Domain names (slot ids and titles such as "navigation" or "Timeline") live
//! in this model only as the *generated demo's* stub content. The reusable
//! `desktop-shell` crate never sees them.

use std::fmt;

/// Visual composition preset. Kebab names are the CLI contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layout {
    TechnicalRibbon,
    Ide,
    Studio,
    Operator,
    Minimal,
}

/// Capability shipped into a generated app. Only capabilities actually
/// implemented by the renderer may be exposed via `--with`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Feature {
    Ribbon,
    Menu,
    Toolbar,
    ActivityBar,
    Explorer,
    Inspector,
    BottomPanel,
    Statusbar,
    Persistence,
    Theme,
}

///starter theme wired as the initial [`ThemeMode`](https://docs.rs/iced) value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

/// Valid `--layout` values, in stable order.
pub const VALID_LAYOUTS: [&str; 5] = ["technical-ribbon", "ide", "studio", "operator", "minimal"];

/// Valid `--with` / `--without` values, in stable order.
pub const VALID_FEATURES: [&str; 10] = [
    "ribbon",
    "menu",
    "toolbar",
    "activity-bar",
    "explorer",
    "inspector",
    "bottom-panel",
    "statusbar",
    "persistence",
    "theme",
];

/// Reserved capability names that always exist and cannot be removed.
/// (Checked case-insensitively by the validator.)
pub const RESERVED_ALWAYS_PRESENT: [&str; 1] = ["workspace"];

/// A named panel slot declared by a layout preset: `id` is the stable panel
/// identifier, `title` the human-readable label used by the generated demo stub.
#[derive(Debug, Clone, Copy)]
pub struct PanelSlot {
    pub id: &'static str,
    pub title: &'static str,
}

/// Panel slots per position. `None` means the position has no slot in this
/// preset. Slots render only when their feature is enabled: left needs
/// `explorer`, right needs `inspector`, bottom needs `bottom-panel`.
/// Domain-flavoured names (navigation, Timeline, ...) belong to the generated
/// demo stub, never to the shell.
#[derive(Debug, Clone, Copy, Default)]
pub struct PanelSlots {
    pub left: Option<PanelSlot>,
    pub right: Option<PanelSlot>,
    pub bottom: Option<PanelSlot>,
}

impl Layout {
    /// CLI kebab-case name.
    pub fn kebab(&self) -> &'static str {
        match self {
            Self::TechnicalRibbon => "technical-ribbon",
            Self::Ide => "ide",
            Self::Studio => "studio",
            Self::Operator => "operator",
            Self::Minimal => "minimal",
        }
    }

    /// Parse a `--layout` value. Returns `None` for unknown names.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "technical-ribbon" => Some(Self::TechnicalRibbon),
            "ide" => Some(Self::Ide),
            "studio" => Some(Self::Studio),
            "operator" => Some(Self::Operator),
            "minimal" => Some(Self::Minimal),
            _ => None,
        }
    }

    /// Default feature set for the preset.
    ///
    /// Design note: `menu` and `statusbar` default on in every preset. They are
    /// the shared shell chrome that keeps a common visual identity and the
    /// `Command -> Menu` backbone alive in every generated app; the direction
    /// document explicitly allows defaults to evolve. Preset identity comes
    /// from the remaining capabilities and from the panel slots below. Pass
    /// `--without menu` / `--without statusbar` for the spartan variant.
    pub fn default_features(self) -> Vec<Feature> {
        match self {
            Self::TechnicalRibbon => vec![
                Feature::Menu,
                Feature::Ribbon,
                Feature::Explorer,
                Feature::Inspector,
                Feature::BottomPanel,
                Feature::Statusbar,
            ],
            Self::Ide => vec![
                Feature::Menu,
                Feature::ActivityBar,
                Feature::Explorer,
                Feature::BottomPanel,
                Feature::Statusbar,
            ],
            Self::Studio => vec![
                Feature::Menu,
                Feature::Toolbar,
                Feature::Explorer,
                Feature::Inspector,
                Feature::BottomPanel,
                Feature::Statusbar,
            ],
            Self::Operator => vec![
                Feature::Menu,
                Feature::Explorer,
                Feature::Inspector,
                Feature::BottomPanel,
                Feature::Statusbar,
            ],
            Self::Minimal => vec![Feature::Menu, Feature::Toolbar, Feature::Statusbar],
        }
    }

    /// Panel slots as [`PanelSlots`] with `(slot_id, demo_title)` per position.
    pub fn slots(self) -> PanelSlots {
        match self {
            Self::TechnicalRibbon => PanelSlots {
                left: Some(PanelSlot {
                    id: "explorer",
                    title: "Explorer",
                }),
                right: Some(PanelSlot {
                    id: "inspector",
                    title: "Inspector",
                }),
                bottom: Some(PanelSlot {
                    id: "output",
                    title: "Output",
                }),
            },
            Self::Ide => PanelSlots {
                left: Some(PanelSlot {
                    id: "explorer",
                    title: "Explorer",
                }),
                right: Some(PanelSlot {
                    id: "inspector",
                    title: "Inspector",
                }),
                bottom: Some(PanelSlot {
                    id: "output",
                    title: "Output",
                }),
            },
            Self::Studio => PanelSlots {
                left: Some(PanelSlot {
                    id: "hierarchy",
                    title: "Hierarchy",
                }),
                right: Some(PanelSlot {
                    id: "properties",
                    title: "Properties",
                }),
                bottom: Some(PanelSlot {
                    id: "timeline",
                    title: "Timeline",
                }),
            },
            Self::Operator => PanelSlots {
                left: Some(PanelSlot {
                    id: "navigation",
                    title: "Navigation",
                }),
                right: Some(PanelSlot {
                    id: "controls",
                    title: "Controls",
                }),
                bottom: Some(PanelSlot {
                    id: "alarms",
                    title: "Alarms",
                }),
            },
            Self::Minimal => PanelSlots {
                left: Some(PanelSlot {
                    id: "explorer",
                    title: "Explorer",
                }),
                right: Some(PanelSlot {
                    id: "inspector",
                    title: "Inspector",
                }),
                bottom: Some(PanelSlot {
                    id: "output",
                    title: "Output",
                }),
            },
        }
    }

    /// One-line description for `list-presets` agent output.
    pub fn describe(self) -> &'static str {
        match self {
            Self::TechnicalRibbon => {
                "CAD/CAE-style shell: menu + data-driven ribbon, explorer, inspector, bottom output, status bar"
            }
            Self::Ide => {
                "IDE workbench pattern: menu + activity rail, explorer sidebar, bottom panel, status bar"
            }
            Self::Studio => {
                "Workspace-first canvas: menu + toolbar, contextual side panels, bottom timeline area"
            }
            Self::Operator => {
                "Operation and monitoring: menu, navigation/controls side panels, alarms bottom, status bar"
            }
            Self::Minimal => "Single-task utility: menu + compact toolbar, workspace, status bar",
        }
    }
}

impl Feature {
    /// CLI kebab-case name.
    pub fn kebab(&self) -> &'static str {
        match self {
            Self::Ribbon => "ribbon",
            Self::Menu => "menu",
            Self::Toolbar => "toolbar",
            Self::ActivityBar => "activity-bar",
            Self::Explorer => "explorer",
            Self::Inspector => "inspector",
            Self::BottomPanel => "bottom-panel",
            Self::Statusbar => "statusbar",
            Self::Persistence => "persistence",
            Self::Theme => "theme",
        }
    }

    /// Parse a `--with` / `--without` entry. Returns `None` for unknown names.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "ribbon" => Some(Self::Ribbon),
            "menu" => Some(Self::Menu),
            "toolbar" => Some(Self::Toolbar),
            "activity-bar" => Some(Self::ActivityBar),
            "explorer" => Some(Self::Explorer),
            "inspector" => Some(Self::Inspector),
            "bottom-panel" => Some(Self::BottomPanel),
            "statusbar" => Some(Self::Statusbar),
            "persistence" => Some(Self::Persistence),
            "theme" => Some(Self::Theme),
            _ => None,
        }
    }

    /// Features the renderer knows how to emit. A preset default outside this
    /// list is rejected as an internal error instead of generating broken code.
    pub fn is_implemented(self) -> bool {
        true
    }
}

impl Theme {
    /// Parse a `--theme` value. Returns `None` for unknown names.
    pub fn parse(raw: &str) -> Option<Self> {
        match raw.trim() {
            "dark" => Some(Self::Dark),
            "light" => Some(Self::Light),
            _ => None,
        }
    }

    pub fn kebab(&self) -> &'static str {
        match self {
            Self::Dark => "dark",
            Self::Light => "light",
        }
    }

    /// ThemeMode variant wired as the generated app's initial theme.
    pub fn theme_mode(self) -> &'static str {
        match self {
            Self::Dark => "ThemeMode::Dark",
            Self::Light => "ThemeMode::Light",
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kebab())
    }
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.kebab())
    }
}
