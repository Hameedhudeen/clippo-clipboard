use crate::{ClipboardContent, ClipboardItem, ClipboardSource, ItemId, TimestampMillis};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryConfig {
    pub max_items: usize,
    pub max_item_bytes: usize,
}

impl Default for HistoryConfig {
    fn default() -> Self {
        Self {
            max_items: 200,
            max_item_bytes: 5 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaptureState {
    pub enabled: bool,
    pub ignore_next_copy: bool,
}

impl Default for CaptureState {
    fn default() -> Self {
        Self {
            enabled: true,
            ignore_next_copy: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum HistoryError {
    ItemTooLarge { actual: usize, maximum: usize },
    NotFound(ItemId),
}

#[derive(Debug, Clone)]
pub struct ClipboardHistory {
    config: HistoryConfig,
    items: Vec<ClipboardItem>,
    next_id: u64,
    capture_state: CaptureState,
    internal_write_marker: Option<InternalWriteMarker>,
}

impl ClipboardHistory {
    #[must_use]
    pub fn new(config: HistoryConfig) -> Self {
        Self {
            config,
            items: Vec::new(),
            next_id: 1,
            capture_state: CaptureState::default(),
            internal_write_marker: None,
        }
    }

    #[must_use]
    pub fn config(&self) -> HistoryConfig {
        self.config
    }

    #[must_use]
    pub fn capture_state(&self) -> CaptureState {
        self.capture_state
    }

    pub fn set_capture_enabled(&mut self, enabled: bool) {
        self.capture_state.enabled = enabled;
    }

    pub fn ignore_next_copy(&mut self) {
        self.capture_state.ignore_next_copy = true;
    }

    #[must_use]
    pub fn should_capture_next_copy(&mut self) -> bool {
        if !self.capture_state.enabled {
            return false;
        }

        if self.capture_state.ignore_next_copy {
            self.capture_state.ignore_next_copy = false;
            return false;
        }

        true
    }

    pub fn mark_internal_clipboard_write(&mut self, content: &ClipboardContent) {
        self.internal_write_marker = Some(InternalWriteMarker::from_content(content));
    }

    #[must_use]
    pub fn should_capture_content(&mut self, content: &ClipboardContent) -> bool {
        if !self.should_capture_next_copy() {
            return false;
        }

        let incoming_marker = InternalWriteMarker::from_content(content);
        if self.internal_write_marker.as_ref() == Some(&incoming_marker) {
            self.internal_write_marker = None;
            return false;
        }

        true
    }

    #[must_use]
    pub fn items(&self) -> &[ClipboardItem] {
        &self.items
    }

    #[must_use]
    pub fn item_at_index(&self, index: usize) -> Option<&ClipboardItem> {
        self.items.get(index)
    }

    #[must_use]
    pub fn item_by_pinned_shortcut(&self, shortcut: char) -> Option<&ClipboardItem> {
        self.items
            .iter()
            .find(|item| item.pinned && item.pinned_shortcut == Some(shortcut))
    }

    pub fn add_item(
        &mut self,
        content: ClipboardContent,
        now: TimestampMillis,
        source: Option<ClipboardSource>,
    ) -> Result<ItemId, HistoryError> {
        let item_size = content.approximate_size_bytes();
        if item_size > self.config.max_item_bytes {
            return Err(HistoryError::ItemTooLarge {
                actual: item_size,
                maximum: self.config.max_item_bytes,
            });
        }

        if let Some(existing_index) = self.items.iter().position(|item| item.content == content) {
            let mut existing = self.items.remove(existing_index);
            existing.last_used_at = now;
            let bounded_preview = existing.content.bounded_preview_text();
            existing.preview_text = bounded_preview.text;
            existing.preview_truncated = bounded_preview.truncated;
            let id = existing.id;
            self.items.insert(0, existing);
            self.sort_items();
            return Ok(id);
        }

        let id = ItemId(self.next_id);
        self.next_id += 1;

        self.items
            .insert(0, ClipboardItem::new(id, content, now, source));
        self.enforce_limit();
        self.sort_items();
        Ok(id)
    }

    pub fn delete(&mut self, id: ItemId) -> Result<(), HistoryError> {
        let before = self.items.len();
        self.items.retain(|item| item.id != id);

        if self.items.len() == before {
            return Err(HistoryError::NotFound(id));
        }

        Ok(())
    }

    pub fn pin(&mut self, id: ItemId, shortcut: Option<char>) -> Result<(), HistoryError> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .ok_or(HistoryError::NotFound(id))?;

        item.pinned = true;
        item.pinned_shortcut = shortcut;
        self.sort_items();
        Ok(())
    }

    pub fn unpin(&mut self, id: ItemId) -> Result<(), HistoryError> {
        let item = self
            .items
            .iter_mut()
            .find(|item| item.id == id)
            .ok_or(HistoryError::NotFound(id))?;

        item.pinned = false;
        item.pinned_shortcut = None;
        self.sort_items();
        Ok(())
    }

    pub fn clear_unpinned(&mut self) {
        self.items.retain(|item| item.pinned);
    }

    pub fn clear_all(&mut self) {
        self.items.clear();
    }

    fn enforce_limit(&mut self) {
        if self.items.len() <= self.config.max_items {
            return;
        }

        let mut regular_items_seen = 0;
        self.items.retain(|item| {
            if item.pinned {
                true
            } else {
                regular_items_seen += 1;
                regular_items_seen <= self.config.max_items
            }
        });
    }

    fn sort_items(&mut self) {
        self.items.sort_by(|left, right| {
            right
                .pinned
                .cmp(&left.pinned)
                .then_with(|| right.last_used_at.cmp(&left.last_used_at))
                .then_with(|| left.id.cmp(&right.id))
        });
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct InternalWriteMarker {
    content_kind: crate::ContentKind,
    preview_text: String,
    approximate_size_bytes: usize,
}

impl InternalWriteMarker {
    fn from_content(content: &ClipboardContent) -> Self {
        Self {
            content_kind: content.kind(),
            preview_text: content.preview_text(),
            approximate_size_bytes: content.approximate_size_bytes(),
        }
    }
}

impl Default for ClipboardHistory {
    fn default() -> Self {
        Self::new(HistoryConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text(value: &str) -> ClipboardContent {
        ClipboardContent::Text {
            text: value.to_string(),
        }
    }

    #[test]
    fn adds_and_orders_items_by_last_use() {
        let mut history = ClipboardHistory::default();

        let first = history
            .add_item(text("first"), TimestampMillis(1), None)
            .unwrap();
        let second = history
            .add_item(text("second"), TimestampMillis(2), None)
            .unwrap();

        assert_eq!(history.items()[0].id, second);
        assert_eq!(history.items()[1].id, first);
    }

    #[test]
    fn deduplicates_existing_content() {
        let mut history = ClipboardHistory::default();

        let first = history
            .add_item(text("same"), TimestampMillis(1), None)
            .unwrap();
        let second = history
            .add_item(text("other"), TimestampMillis(2), None)
            .unwrap();
        let duplicate = history
            .add_item(text("same"), TimestampMillis(3), None)
            .unwrap();

        assert_eq!(first, duplicate);
        assert_eq!(history.items().len(), 2);
        assert_eq!(history.items()[0].id, first);
        assert_eq!(history.items()[1].id, second);
    }

    #[test]
    fn keeps_pinned_items_above_regular_items() {
        let mut history = ClipboardHistory::default();

        let pinned = history
            .add_item(text("pinned"), TimestampMillis(1), None)
            .unwrap();
        let regular = history
            .add_item(text("regular"), TimestampMillis(2), None)
            .unwrap();
        history.pin(pinned, Some('1')).unwrap();

        assert_eq!(history.items()[0].id, pinned);
        assert_eq!(history.items()[1].id, regular);
    }

    #[test]
    fn clears_only_unpinned_items() {
        let mut history = ClipboardHistory::default();

        let pinned = history
            .add_item(text("pinned"), TimestampMillis(1), None)
            .unwrap();
        history
            .add_item(text("regular"), TimestampMillis(2), None)
            .unwrap();
        history.pin(pinned, None).unwrap();

        history.clear_unpinned();

        assert_eq!(history.items().len(), 1);
        assert_eq!(history.items()[0].id, pinned);
    }

    #[test]
    fn deletes_items_by_id() {
        let mut history = ClipboardHistory::default();

        let first = history
            .add_item(text("first"), TimestampMillis(1), None)
            .unwrap();
        history
            .add_item(text("second"), TimestampMillis(2), None)
            .unwrap();

        history.delete(first).unwrap();

        assert_eq!(history.items().len(), 1);
        assert_ne!(history.items()[0].id, first);
    }

    #[test]
    fn unpins_items() {
        let mut history = ClipboardHistory::default();

        let id = history
            .add_item(text("clip"), TimestampMillis(1), None)
            .unwrap();
        history.pin(id, Some('4')).unwrap();
        history.unpin(id).unwrap();

        assert!(!history.items()[0].pinned);
        assert_eq!(history.items()[0].pinned_shortcut, None);
    }

    #[test]
    fn enforces_regular_item_limit() {
        let mut history = ClipboardHistory::new(HistoryConfig {
            max_items: 2,
            max_item_bytes: 1024,
        });

        history
            .add_item(text("one"), TimestampMillis(1), None)
            .unwrap();
        history
            .add_item(text("two"), TimestampMillis(2), None)
            .unwrap();
        history
            .add_item(text("three"), TimestampMillis(3), None)
            .unwrap();

        assert_eq!(history.items().len(), 2);
        assert_eq!(history.items()[0].preview_text, "three");
        assert_eq!(history.items()[1].preview_text, "two");
    }

    #[test]
    fn rejects_items_above_size_limit() {
        let mut history = ClipboardHistory::new(HistoryConfig {
            max_items: 10,
            max_item_bytes: 3,
        });

        let error = history
            .add_item(text("large"), TimestampMillis(1), None)
            .unwrap_err();

        assert_eq!(
            error,
            HistoryError::ItemTooLarge {
                actual: 5,
                maximum: 3
            }
        );
    }

    #[test]
    fn supports_index_and_pinned_shortcut_selection() {
        let mut history = ClipboardHistory::default();

        let first = history
            .add_item(text("first"), TimestampMillis(1), None)
            .unwrap();
        history.pin(first, Some('7')).unwrap();

        assert_eq!(history.item_at_index(0).map(|item| item.id), Some(first));
        assert_eq!(
            history.item_by_pinned_shortcut('7').map(|item| item.id),
            Some(first)
        );
    }

    #[test]
    fn ignore_next_copy_is_consumed_once() {
        let mut history = ClipboardHistory::default();

        history.ignore_next_copy();

        assert!(!history.should_capture_next_copy());
        assert!(history.should_capture_next_copy());
    }

    #[test]
    fn suppresses_clippo_internal_clipboard_write_once() {
        let mut history = ClipboardHistory::default();
        let content = text("from clippo");

        history.mark_internal_clipboard_write(&content);

        assert!(!history.should_capture_content(&content));
        assert!(history.should_capture_content(&content));
    }

    #[test]
    fn accepts_unsupported_clipboard_content_as_metadata() {
        let mut history = ClipboardHistory::default();

        history
            .add_item(
                ClipboardContent::Unsupported {
                    format_names: vec!["custom/type".to_string()],
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();

        assert_eq!(history.items()[0].preview_text, "custom/type");
    }
}
