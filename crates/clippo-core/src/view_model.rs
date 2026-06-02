use serde::{Deserialize, Serialize};

use crate::{search_items, BoundedText, ClipboardHistory, ItemId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupViewModel {
    pub search_query: String,
    pub state: PopupState,
    pub sections: Vec<PopupSection>,
    pub footer_actions: Vec<FooterAction>,
}

impl PopupViewModel {
    #[must_use]
    pub fn from_history(history: &ClipboardHistory, search_query: &str) -> Self {
        let capture_state = history.capture_state();
        let ordered_ids = search_items(history.items(), search_query)
            .into_iter()
            .map(|match_result| match_result.item_id)
            .collect::<Vec<_>>();

        let mut pinned_rows = Vec::new();
        let mut history_rows = Vec::new();

        for (visible_index, item_id) in ordered_ids.into_iter().enumerate() {
            let Some(item) = history.items().iter().find(|item| item.id == item_id) else {
                continue;
            };
            let row = PopupRow {
                item_id: item.id,
                visible_shortcut: item.pinned_shortcut.map_or_else(
                    || visible_shortcut_for_index(visible_index).unwrap_or_default(),
                    |shortcut| shortcut.to_string(),
                ),
                preview_text: item.preview_text.clone(),
                preview_truncated: item.preview_truncated,
                full_preview_available: true,
                pinned: item.pinned,
                content_kind_key: item.content_kind_key.clone(),
            };

            if item.pinned {
                pinned_rows.push(row);
            } else {
                history_rows.push(row);
            }
        }

        let mut sections = Vec::new();
        if !pinned_rows.is_empty() {
            sections.push(PopupSection {
                title_key: "history.section.pinned".to_string(),
                rows: pinned_rows,
            });
        }
        if !history_rows.is_empty() {
            sections.push(PopupSection {
                title_key: "history.section.history".to_string(),
                rows: history_rows,
            });
        }

        Self {
            search_query: search_query.to_string(),
            state: PopupState {
                empty: history.items().is_empty(),
                paused: !capture_state.enabled,
                ignore_next_copy: capture_state.ignore_next_copy,
            },
            sections,
            footer_actions: vec![
                FooterAction::ClearUnpinned,
                FooterAction::PauseOrResumeCapture,
                FooterAction::IgnoreNextCopy,
                FooterAction::Preferences,
            ],
        }
    }

    #[must_use]
    pub fn full_preview_for_item(
        history: &ClipboardHistory,
        item_id: ItemId,
    ) -> Option<PopupFullPreview> {
        let item = history.items().iter().find(|item| item.id == item_id)?;
        let BoundedText { text, truncated } = item.content.bounded_full_preview_text();

        Some(PopupFullPreview {
            item_id,
            text,
            truncated,
        })
    }

    #[must_use]
    pub fn item_id_for_visible_shortcut(&self, shortcut: char) -> Option<ItemId> {
        self.sections
            .iter()
            .flat_map(|section| section.rows.iter())
            .find(|row| row.visible_shortcut == shortcut.to_string())
            .map(|row| row.item_id)
    }
}

fn visible_shortcut_for_index(index: usize) -> Option<String> {
    "123456789"
        .chars()
        .nth(index)
        .map(|value| value.to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupState {
    pub empty: bool,
    pub paused: bool,
    pub ignore_next_copy: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupSection {
    pub title_key: String,
    pub rows: Vec<PopupRow>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupRow {
    pub item_id: ItemId,
    pub visible_shortcut: String,
    pub preview_text: String,
    pub preview_truncated: bool,
    pub full_preview_available: bool,
    pub pinned: bool,
    pub content_kind_key: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PopupFullPreview {
    pub item_id: ItemId,
    pub text: String,
    pub truncated: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FooterAction {
    ClearUnpinned,
    PauseOrResumeCapture,
    IgnoreNextCopy,
    Preferences,
}

impl FooterAction {
    #[must_use]
    pub fn label_key(self) -> &'static str {
        match self {
            Self::ClearUnpinned => "action.clear_unpinned",
            Self::PauseOrResumeCapture => "action.pause_capture",
            Self::IgnoreNextCopy => "action.ignore_next_copy",
            Self::Preferences => "action.preferences",
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ClipboardContent, TimestampMillis};

    use super::*;

    #[test]
    fn view_model_groups_pinned_rows_above_history_rows() {
        let mut history = ClipboardHistory::default();
        let regular = history
            .add_item(
                ClipboardContent::Text {
                    text: "regular".to_string(),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();
        let pinned = history
            .add_item(
                ClipboardContent::Text {
                    text: "pinned".to_string(),
                },
                TimestampMillis(2),
                None,
            )
            .unwrap();
        history.pin(pinned, Some('7')).unwrap();

        let view_model = PopupViewModel::from_history(&history, "");

        assert_eq!(view_model.sections[0].title_key, "history.section.pinned");
        assert_eq!(view_model.sections[0].rows[0].item_id, pinned);
        assert_eq!(view_model.sections[0].rows[0].visible_shortcut, "7");
        assert_eq!(view_model.sections[1].rows[0].item_id, regular);
    }

    #[test]
    fn view_model_resolves_visible_shortcuts_to_items() {
        let mut history = ClipboardHistory::default();
        let regular = history
            .add_item(
                ClipboardContent::Text {
                    text: "regular".to_string(),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();
        let pinned = history
            .add_item(
                ClipboardContent::Text {
                    text: "pinned".to_string(),
                },
                TimestampMillis(2),
                None,
            )
            .unwrap();
        history.pin(pinned, Some('7')).unwrap();

        let view_model = PopupViewModel::from_history(&history, "");

        assert_eq!(view_model.item_id_for_visible_shortcut('7'), Some(pinned));
        assert_eq!(view_model.item_id_for_visible_shortcut('2'), Some(regular));
        assert_eq!(view_model.item_id_for_visible_shortcut('9'), None);
    }

    #[test]
    fn view_model_omits_unusable_number_shortcuts_after_nine_items() {
        let mut history = ClipboardHistory::default();

        for index in 1..=10 {
            history
                .add_item(
                    ClipboardContent::Text {
                        text: format!("item {index}"),
                    },
                    TimestampMillis(index),
                    None,
                )
                .unwrap();
        }

        let view_model = PopupViewModel::from_history(&history, "");
        let rows = &view_model.sections[0].rows;

        assert_eq!(rows[8].visible_shortcut, "9");
        assert_eq!(rows[9].visible_shortcut, "");
        assert_eq!(
            view_model.item_id_for_visible_shortcut('9'),
            Some(rows[8].item_id)
        );
    }

    #[test]
    fn view_model_reports_empty_state() {
        let history = ClipboardHistory::default();

        let view_model = PopupViewModel::from_history(&history, "");

        assert!(view_model.state.empty);
        assert!(view_model.sections.is_empty());
    }

    #[test]
    fn view_model_reports_paused_and_ignore_next_state() {
        let mut history = ClipboardHistory::default();
        history.set_capture_enabled(false);
        history.ignore_next_copy();

        let view_model = PopupViewModel::from_history(&history, "");

        assert!(view_model.state.paused);
        assert!(view_model.state.ignore_next_copy);
    }

    #[test]
    fn view_model_includes_full_preview_text() {
        let mut history = ClipboardHistory::default();
        let id = history
            .add_item(
                ClipboardContent::Html {
                    plain_text: "Example".to_string(),
                    html: "<strong>Example</strong>".to_string(),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();

        let view_model = PopupViewModel::from_history(&history, "");
        let full_preview = PopupViewModel::full_preview_for_item(&history, id).unwrap();

        assert!(view_model.sections[0].rows[0].full_preview_available);
        assert_eq!(full_preview.text, "<strong>Example</strong>");
        assert!(!full_preview.truncated);
    }

    #[test]
    fn view_model_does_not_embed_large_full_preview_in_rows() {
        let mut history = ClipboardHistory::default();
        let id = history
            .add_item(
                ClipboardContent::Html {
                    plain_text: "Example".to_string(),
                    html: "<p>".to_string() + &"a".repeat(crate::FULL_PREVIEW_TEXT_LIMIT + 100),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();

        let view_model = PopupViewModel::from_history(&history, "");
        let row = &view_model.sections[0].rows[0];
        let full_preview = PopupViewModel::full_preview_for_item(&history, id).unwrap();

        assert_eq!(row.preview_text, "Example");
        assert!(row.full_preview_available);
        assert!(full_preview.truncated);
        assert!(full_preview.text.len() < crate::FULL_PREVIEW_TEXT_LIMIT + 100);
    }
}
