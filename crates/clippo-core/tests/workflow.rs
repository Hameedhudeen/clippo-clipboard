use clippo_core::{
    search_items, ClipboardContent, ClipboardHistory, ClippoCommand, CommandDecision,
    CommandRouter, PasteMode, PrivacyRules, SelectionMove, SelectionState, SelectionWrapMode,
    TimestampMillis,
};

#[test]
fn maccy_like_core_workflow_can_search_pin_paste_and_clear() {
    let mut history = ClipboardHistory::default();
    let rules = PrivacyRules::maccy_compatible_defaults();
    let context = clippo_core::CaptureContext {
        format_names: Vec::new(),
        source: None,
    };
    let first_content = ClipboardContent::Text {
        text: "first reusable clip".to_string(),
    };
    let second_content = ClipboardContent::Text {
        text: "second clip".to_string(),
    };

    assert!(rules.should_capture(&context, &first_content).capture);
    let first_id = history
        .add_item(first_content, TimestampMillis(1), None)
        .unwrap();
    let second_id = history
        .add_item(second_content, TimestampMillis(2), None)
        .unwrap();

    let matches = search_items(history.items(), "reusable");
    assert_eq!(matches[0].item_id, first_id);

    CommandRouter::apply(
        &mut history,
        ClippoCommand::TogglePin {
            item_id: first_id,
            shortcut: Some('1'),
        },
    )
    .unwrap();
    assert_eq!(history.items()[0].id, first_id);

    let selection = SelectionState::new(history.items().len(), SelectionWrapMode::Wrap)
        .move_selection(SelectionMove::Next);
    assert_eq!(selection.selected_index, Some(1));

    let decision = CommandRouter::apply(
        &mut history,
        ClippoCommand::PasteSelected {
            item_id: second_id,
            mode: PasteMode::PlainText,
        },
    )
    .unwrap();
    assert_eq!(
        decision,
        CommandDecision::Paste {
            item_id: second_id,
            mode: PasteMode::PlainText
        }
    );

    CommandRouter::apply(&mut history, ClippoCommand::ClearUnpinned).unwrap();
    assert_eq!(history.items().len(), 1);
    assert_eq!(history.items()[0].id, first_id);
}
