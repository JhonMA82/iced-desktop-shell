//! Dock layout abstraction encapsulating multi-panel arrangement.
//!
//! Encapsulates panel placement and serves as the single source of truth
//! for panel visibility, location, size, and display order.

use super::panel::{PanelId, PanelLocation, PanelState};
use super::theme::tokens;
use std::collections::HashMap;

/// High-level multi-panel layout manager.
#[derive(Debug, Clone, Default)]
pub struct DockLayout {
    panels: HashMap<PanelId, PanelState>,
    order: Vec<PanelId>,
}

impl DockLayout {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a panel into the layout. Preserves registration order.
    /// The initial size is normalized to the location minimum (see [`Self::set_size`]).
    pub fn register(&mut self, panel: PanelState) {
        let mut panel = panel;
        panel.size = Self::clamp_size(panel.location, panel.size);
        if !self.panels.contains_key(&panel.id) {
            self.order.push(panel.id.clone());
        }
        self.panels.insert(panel.id.clone(), panel);
    }

    pub fn get(&self, id: &PanelId) -> Option<&PanelState> {
        self.panels.get(id)
    }

    pub fn get_mut(&mut self, id: &PanelId) -> Option<&mut PanelState> {
        self.panels.get_mut(id)
    }

    pub fn is_visible(&self, id: &PanelId) -> bool {
        self.panels.get(id).is_some_and(|p| p.visible)
    }

    pub fn set_visible(&mut self, id: &PanelId, visible: bool) {
        if let Some(panel) = self.panels.get_mut(id) {
            panel.visible = visible;
        }
    }

    pub fn toggle(&mut self, id: &PanelId) {
        if let Some(panel) = self.panels.get_mut(id) {
            panel.toggle();
        }
    }

    /// Update a panel size, clamped to the minimum for its location
    /// (`Left` → `SIDEBAR_MIN_WIDTH`, `Right` → `INSPECTOR_MIN_WIDTH`,
    /// `Bottom` → `BOTTOM_PANEL_MIN_HEIGHT`) and always greater than zero.
    /// Unknown ids are ignored.
    pub fn set_size(&mut self, id: &PanelId, size: f32) {
        if let Some(panel) = self.panels.get_mut(id) {
            panel.size = Self::clamp_size(panel.location, size);
        }
    }

    fn clamp_size(location: PanelLocation, size: f32) -> f32 {
        let min = match location {
            PanelLocation::Left => tokens::SIDEBAR_MIN_WIDTH,
            PanelLocation::Right => tokens::INSPECTOR_MIN_WIDTH,
            PanelLocation::Bottom => tokens::BOTTOM_PANEL_MIN_HEIGHT,
        };
        size.max(min).max(f32::MIN_POSITIVE)
    }

    /// Returns registered panels strictly adhering to the configured order.
    pub fn ordered_panels(&self) -> Vec<&PanelState> {
        self.order
            .iter()
            .filter_map(|id| self.panels.get(id))
            .collect()
    }

    /// Retrieve all registered panels for a specific location in configured order.
    pub fn panels_at(&self, location: PanelLocation) -> Vec<&PanelState> {
        self.order
            .iter()
            .filter_map(|id| self.panels.get(id))
            .filter(|p| p.location == location)
            .collect()
    }

    pub fn panels(&self) -> &HashMap<PanelId, PanelState> {
        &self.panels
    }
}
