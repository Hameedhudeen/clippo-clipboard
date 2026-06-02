use serde::{Deserialize, Serialize};

use crate::{ClipboardContent, ClipboardHistory, HistoryError, ItemId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClippoCommand {
    CopySelected {
        item_id: ItemId,
    },
    PasteSelected {
        item_id: ItemId,
        mode: PasteMode,
    },
    DeleteSelected {
        item_id: ItemId,
    },
    TogglePin {
        item_id: ItemId,
        shortcut: Option<char>,
    },
    ClearUnpinned,
    ClearAll,
    PauseCapture,
    ResumeCapture,
    IgnoreNextCopy,
    OpenPreferences,
    EmergencyClearHistoryAndQuit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PasteMode {
    PreserveFormatting,
    PlainText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommandDecision {
    CopyToClipboard { item_id: ItemId },
    Paste { item_id: ItemId, mode: PasteMode },
    OpenPreferences,
    QuitAfterClearingHistory,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PastePlan {
    pub item_id: ItemId,
    pub mode: PasteMode,
    pub clipboard_content: ClipboardContent,
}

#[derive(Debug, Default)]
pub struct CommandRouter;

impl CommandRouter {
    pub fn apply(
        history: &mut ClipboardHistory,
        command: ClippoCommand,
    ) -> Result<CommandDecision, HistoryError> {
        match command {
            ClippoCommand::CopySelected { item_id } => {
                ensure_item_exists(history, item_id)?;
                Ok(CommandDecision::CopyToClipboard { item_id })
            }
            ClippoCommand::PasteSelected { item_id, mode } => {
                ensure_item_exists(history, item_id)?;
                Ok(CommandDecision::Paste { item_id, mode })
            }
            ClippoCommand::DeleteSelected { item_id } => {
                history.delete(item_id)?;
                Ok(CommandDecision::None)
            }
            ClippoCommand::TogglePin { item_id, shortcut } => {
                let item = history
                    .items()
                    .iter()
                    .find(|item| item.id == item_id)
                    .ok_or(HistoryError::NotFound(item_id))?;
                if item.pinned {
                    history.unpin(item_id)?;
                } else {
                    history.pin(item_id, shortcut)?;
                }
                Ok(CommandDecision::None)
            }
            ClippoCommand::ClearUnpinned => {
                history.clear_unpinned();
                Ok(CommandDecision::None)
            }
            ClippoCommand::ClearAll => {
                history.clear_all();
                Ok(CommandDecision::None)
            }
            ClippoCommand::PauseCapture => {
                history.set_capture_enabled(false);
                Ok(CommandDecision::None)
            }
            ClippoCommand::ResumeCapture => {
                history.set_capture_enabled(true);
                Ok(CommandDecision::None)
            }
            ClippoCommand::IgnoreNextCopy => {
                history.ignore_next_copy();
                Ok(CommandDecision::None)
            }
            ClippoCommand::OpenPreferences => Ok(CommandDecision::OpenPreferences),
            ClippoCommand::EmergencyClearHistoryAndQuit => {
                history.clear_all();
                Ok(CommandDecision::QuitAfterClearingHistory)
            }
        }
    }

    pub fn plan_paste(
        history: &ClipboardHistory,
        item_id: ItemId,
        mode: PasteMode,
    ) -> Result<PastePlan, HistoryError> {
        let item = history
            .items()
            .iter()
            .find(|item| item.id == item_id)
            .ok_or(HistoryError::NotFound(item_id))?;

        Ok(PastePlan {
            item_id,
            mode,
            clipboard_content: match mode {
                PasteMode::PreserveFormatting => item.content.clone(),
                PasteMode::PlainText => ClipboardContent::Text {
                    text: item.content.plain_text_for_paste(),
                },
            },
        })
    }
}

fn ensure_item_exists(history: &ClipboardHistory, item_id: ItemId) -> Result<(), HistoryError> {
    if history.items().iter().any(|item| item.id == item_id) {
        Ok(())
    } else {
        Err(HistoryError::NotFound(item_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ClipboardContent, TimestampMillis};

    fn seed_history() -> (ClipboardHistory, ItemId) {
        let mut history = ClipboardHistory::default();
        let id = history
            .add_item(
                ClipboardContent::Text {
                    text: "hello".to_string(),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();
        (history, id)
    }

    #[test]
    fn routes_copy_without_mutating_history() {
        let (mut history, id) = seed_history();

        let decision =
            CommandRouter::apply(&mut history, ClippoCommand::CopySelected { item_id: id })
                .unwrap();

        assert_eq!(decision, CommandDecision::CopyToClipboard { item_id: id });
        assert_eq!(history.items().len(), 1);
    }

    #[test]
    fn routes_plain_text_paste() {
        let (mut history, id) = seed_history();

        let decision = CommandRouter::apply(
            &mut history,
            ClippoCommand::PasteSelected {
                item_id: id,
                mode: PasteMode::PlainText,
            },
        )
        .unwrap();

        assert_eq!(
            decision,
            CommandDecision::Paste {
                item_id: id,
                mode: PasteMode::PlainText
            }
        );
    }

    #[test]
    fn plans_plain_text_paste_without_formatting() {
        let mut history = ClipboardHistory::default();
        let id = history
            .add_item(
                ClipboardContent::RichText {
                    plain_text: "hello\n  world".to_string(),
                    rtf: Some(vec![1, 2, 3]),
                    html: Some("<p>hello<br>  world</p>".to_string()),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();

        let plan = CommandRouter::plan_paste(&history, id, PasteMode::PlainText).unwrap();

        assert_eq!(
            plan.clipboard_content,
            ClipboardContent::Text {
                text: "hello\n  world".to_string()
            }
        );
    }

    #[test]
    fn emergency_clear_removes_all_history() {
        let (mut history, _) = seed_history();

        let decision =
            CommandRouter::apply(&mut history, ClippoCommand::EmergencyClearHistoryAndQuit)
                .unwrap();

        assert_eq!(decision, CommandDecision::QuitAfterClearingHistory);
        assert!(history.items().is_empty());
    }
}
