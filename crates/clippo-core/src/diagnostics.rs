use serde::{Deserialize, Serialize};

use crate::ClipboardHistory;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticEvent {
    pub category: String,
    pub message: String,
    pub clipboard_contents_redacted: bool,
}

impl DiagnosticEvent {
    #[must_use]
    pub fn redacted(category: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            message: message.into(),
            clipboard_contents_redacted: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSnapshot {
    pub history_item_count: usize,
    pub capture_enabled: bool,
    pub ignore_next_copy: bool,
    pub clipboard_contents_redacted: bool,
}

impl DiagnosticSnapshot {
    #[must_use]
    pub fn from_history(history: &ClipboardHistory) -> Self {
        let capture_state = history.capture_state();
        Self {
            history_item_count: history.items().len(),
            capture_enabled: capture_state.enabled,
            ignore_next_copy: capture_state.ignore_next_copy,
            clipboard_contents_redacted: true,
        }
    }

    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClipboardContent, TimestampMillis};

    use super::*;

    #[test]
    fn diagnostic_snapshot_excludes_clipboard_contents() {
        let mut history = ClipboardHistory::default();
        history
            .add_item(
                ClipboardContent::Text {
                    text: "top secret clipboard text".to_string(),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();

        let exported = DiagnosticSnapshot::from_history(&history)
            .export_json()
            .unwrap();

        assert!(exported.contains("\"clipboard_contents_redacted\": true"));
        assert!(!exported.contains("top secret clipboard text"));
    }
}
