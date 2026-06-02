use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionWrapMode {
    Wrap,
    Clamp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMove {
    Next,
    Previous,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionState {
    pub selected_index: Option<usize>,
    pub item_count: usize,
    pub wrap_mode: SelectionWrapMode,
}

impl SelectionState {
    #[must_use]
    pub fn new(item_count: usize, wrap_mode: SelectionWrapMode) -> Self {
        Self {
            selected_index: (item_count > 0).then_some(0),
            item_count,
            wrap_mode,
        }
    }

    #[must_use]
    pub fn move_selection(mut self, direction: SelectionMove) -> Self {
        let Some(selected_index) = self.selected_index else {
            return self;
        };

        self.selected_index = Some(match (direction, self.wrap_mode) {
            (SelectionMove::Next, SelectionWrapMode::Wrap) => {
                (selected_index + 1) % self.item_count
            }
            (SelectionMove::Next, SelectionWrapMode::Clamp) => {
                selected_index.saturating_add(1).min(self.item_count - 1)
            }
            (SelectionMove::Previous, SelectionWrapMode::Wrap) => {
                if selected_index == 0 {
                    self.item_count - 1
                } else {
                    selected_index - 1
                }
            }
            (SelectionMove::Previous, SelectionWrapMode::Clamp) => selected_index.saturating_sub(1),
        });

        self
    }

    #[must_use]
    pub fn select_index(mut self, index: usize) -> Self {
        self.selected_index = (index < self.item_count).then_some(index);
        self
    }

    #[must_use]
    pub fn update_item_count(mut self, item_count: usize) -> Self {
        self.item_count = item_count;
        self.selected_index = match (self.selected_index, item_count) {
            (_, 0) => None,
            (Some(index), count) if index < count => Some(index),
            _ => Some(item_count - 1),
        };
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_at_first_item_when_items_exist() {
        let selection = SelectionState::new(3, SelectionWrapMode::Wrap);

        assert_eq!(selection.selected_index, Some(0));
    }

    #[test]
    fn has_no_selection_when_empty() {
        let selection = SelectionState::new(0, SelectionWrapMode::Wrap);

        assert_eq!(selection.selected_index, None);
    }

    #[test]
    fn wraps_forward_and_backward() {
        let selection =
            SelectionState::new(2, SelectionWrapMode::Wrap).move_selection(SelectionMove::Previous);

        assert_eq!(selection.selected_index, Some(1));
        assert_eq!(
            selection.move_selection(SelectionMove::Next).selected_index,
            Some(0)
        );
    }

    #[test]
    fn clamps_forward_and_backward() {
        let selection = SelectionState::new(2, SelectionWrapMode::Clamp)
            .move_selection(SelectionMove::Previous);

        assert_eq!(selection.selected_index, Some(0));
        assert_eq!(
            selection
                .move_selection(SelectionMove::Next)
                .move_selection(SelectionMove::Next)
                .selected_index,
            Some(1)
        );
    }

    #[test]
    fn clears_selection_when_list_becomes_empty() {
        let selection = SelectionState::new(2, SelectionWrapMode::Wrap).update_item_count(0);

        assert_eq!(selection.selected_index, None);
    }
}
