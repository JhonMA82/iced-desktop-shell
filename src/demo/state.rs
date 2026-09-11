//! State representation for the Technical Desktop Shell Demo.
//!
//! Completely decoupled from the shell infrastructure. Represents a technical
//! hierarchy (Components, Resources, Settings) to validate data flow.

use std::collections::HashMap;

/// Identifiers for demo tree items.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DemoItemId {
    ComponentA,
    ComponentB,
    Resource1,
    Settings,
}

impl DemoItemId {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ComponentA => "Component A",
            Self::ComponentB => "Component B",
            Self::Resource1 => "Resource 1",
            Self::Settings => "Global Settings",
        }
    }

    pub fn item_type(&self) -> &'static str {
        match self {
            Self::ComponentA | Self::ComponentB => "Component",
            Self::Resource1 => "Resource",
            Self::Settings => "Configuration",
        }
    }
}

/// Editable properties for a selected item.
#[derive(Debug, Clone)]
pub struct ItemProperties {
    pub name: String,
    pub item_type: String,
    pub enabled: bool,
    pub transform_x: f32,
    pub transform_y: f32,
    pub created_at: String,
    pub modified_at: String,
}

/// State of the demo domain.
#[derive(Debug, Clone)]
pub struct DemoState {
    pub selected_item: Option<DemoItemId>,
    pub properties: HashMap<DemoItemId, ItemProperties>,
    pub logs: Vec<String>,
}

impl Default for DemoState {
    fn default() -> Self {
        let mut properties = HashMap::new();

        properties.insert(
            DemoItemId::ComponentA,
            ItemProperties {
                name: "Component A".to_string(),
                item_type: "Component".to_string(),
                enabled: true,
                transform_x: 100.0,
                transform_y: 50.0,
                created_at: "2026-09-11 08:00:00".to_string(),
                modified_at: "2026-09-11 08:15:22".to_string(),
            },
        );

        properties.insert(
            DemoItemId::ComponentB,
            ItemProperties {
                name: "Component B".to_string(),
                item_type: "Component".to_string(),
                enabled: false,
                transform_x: 240.0,
                transform_y: 120.0,
                created_at: "2026-09-11 08:05:00".to_string(),
                modified_at: "2026-09-11 08:10:14".to_string(),
            },
        );

        properties.insert(
            DemoItemId::Resource1,
            ItemProperties {
                name: "Resource 1".to_string(),
                item_type: "Resource".to_string(),
                enabled: true,
                transform_x: 0.0,
                transform_y: 0.0,
                created_at: "2026-09-11 07:30:00".to_string(),
                modified_at: "2026-09-11 07:30:00".to_string(),
            },
        );

        properties.insert(
            DemoItemId::Settings,
            ItemProperties {
                name: "Global Settings".to_string(),
                item_type: "Configuration".to_string(),
                enabled: true,
                transform_x: 0.0,
                transform_y: 0.0,
                created_at: "2026-09-11 07:00:00".to_string(),
                modified_at: "2026-09-11 08:20:00".to_string(),
            },
        );

        Self {
            selected_item: Some(DemoItemId::ComponentA),
            properties,
            logs: vec![
                "[System] Application initialized.".to_string(),
                "[Project] Loaded sample technical project.".to_string(),
                "[Explorer] Selected Component A.".to_string(),
            ],
        }
    }
}

impl DemoState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn selected_properties(&self) -> Option<&ItemProperties> {
        self.selected_item
            .as_ref()
            .and_then(|id| self.properties.get(id))
    }

    pub fn add_log(&mut self, message: impl Into<String>) {
        self.logs.push(message.into());
    }
}
