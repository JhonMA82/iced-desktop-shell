//! Declarative scaffolding model for `iced-desktop-shell` application presets.
//!
//! One core, presets as data plus composition: layouts declare their default
//! feature set and their panel slots, and the renderer composes small reusable
//! string fragments. There are no `templates/<preset>/` copies of whole apps.
//!
//! The crate has zero external dependencies: argument parsing happens in the
//! `xtask` binary and TOML output is generated as plain text.

pub mod model;
pub mod render;
pub mod spec;

pub use model::{Feature, Layout, Theme, VALID_FEATURES, VALID_LAYOUTS};
pub use render::{render_project, write_project};
pub use spec::{GenerateOptions, ResolvedSpec, ScaffoldError, resolve};
