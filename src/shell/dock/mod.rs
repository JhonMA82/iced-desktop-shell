//! Dock layout abstraction encapsulating multi-panel arrangement.
//!
//! Encapsulates panel placement and serves as the single source of truth
//! for panel visibility, location, size, and display order.

use super::panel::{PanelId, PanelLocation, PanelState};
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
    pub fn register(&mut self, panel: PanelState) {
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
