//! Messages processed by the demo domain.

use super::state::DemoItemId;

/// Demo-specific user interactions.
#[derive(Debug, Clone)]
pub enum DemoMessage {
    SelectItem(DemoItemId),
    ToggleEnabled(DemoItemId),
    IncrementX(DemoItemId),
    DecrementX(DemoItemId),
    IncrementY(DemoItemId),
    DecrementY(DemoItemId),
    ClearLogs,
}
