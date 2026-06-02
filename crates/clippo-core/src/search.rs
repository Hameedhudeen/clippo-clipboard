use std::cmp::Reverse;

use crate::{ClipboardItem, ItemId};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchMatch {
    pub item_id: ItemId,
    pub score: u32,
    pub preview_text: String,
}

#[must_use]
pub fn search_items(items: &[ClipboardItem], query: &str) -> Vec<SearchMatch> {
    let normalized_query = normalize_for_search(query.trim());

    if normalized_query.is_empty() {
        return items
            .iter()
            .map(|item| SearchMatch {
                item_id: item.id,
                score: 0,
                preview_text: item.preview_text.clone(),
            })
            .collect();
    }

    let mut matches = items
        .iter()
        .filter_map(|item| {
            let normalized_preview = normalize_for_search(&item.preview_text);
            normalized_preview.find(&normalized_query).map(|position| {
                let score = if position == 0 {
                    100
                } else if normalized_preview
                    .split_whitespace()
                    .any(|word| word.starts_with(&normalized_query))
                {
                    75
                } else {
                    50
                };

                SearchMatch {
                    item_id: item.id,
                    score,
                    preview_text: item.preview_text.clone(),
                }
            })
        })
        .collect::<Vec<_>>();

    matches.sort_by_key(|item| Reverse(item.score));
    matches
}

fn normalize_for_search(value: &str) -> String {
    value.nfkc().collect::<String>().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClipboardContent, ClipboardItem, ItemId, TimestampMillis};

    fn item(id: u64, text: &str) -> ClipboardItem {
        ClipboardItem::new(
            ItemId(id),
            ClipboardContent::Text {
                text: text.to_string(),
            },
            TimestampMillis(1),
            None,
        )
    }

    #[test]
    fn empty_query_returns_all_items() {
        let items = vec![item(1, "alpha"), item(2, "beta")];

        let matches = search_items(&items, "");

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].item_id, ItemId(1));
    }

    #[test]
    fn ranks_prefix_matches_before_contains_matches() {
        let items = vec![item(1, "copy alpha"), item(2, "alpha copy")];

        let matches = search_items(&items, "alpha");

        assert_eq!(matches[0].item_id, ItemId(2));
        assert_eq!(matches[1].item_id, ItemId(1));
    }

    #[test]
    fn preserves_input_order_for_equal_scores() {
        let items = vec![item(1, "alpha one"), item(2, "alpha two")];

        let matches = search_items(&items, "alpha");

        assert_eq!(matches[0].item_id, ItemId(1));
        assert_eq!(matches[1].item_id, ItemId(2));
    }

    #[test]
    fn normalizes_unicode_for_search() {
        let items = vec![item(1, "Cafe\u{301}")];

        let matches = search_items(&items, "Café");

        assert_eq!(matches[0].item_id, ItemId(1));
    }
}
