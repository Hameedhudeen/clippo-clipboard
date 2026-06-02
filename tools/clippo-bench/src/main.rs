use std::{
    fs,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use clippo_core::{
    search_items, ClipboardContent, ClipboardHistory, PopupViewModel, TimestampMillis,
};
use clippo_persistence::{HistoryStore, JsonFileStore};

fn main() {
    let mut history = ClipboardHistory::default();

    let history_duration = measure(|| {
        for index in 0..1_000 {
            history
                .add_item(
                    ClipboardContent::Text {
                        text: format!("benchmark clipboard item {index}"),
                    },
                    TimestampMillis(index),
                    None,
                )
                .unwrap();
        }
    });

    let search_duration = measure(|| {
        let matches = search_items(history.items(), "item 99");
        assert!(!matches.is_empty());
    });

    let popup_view_model_duration = measure(|| {
        let view_model = PopupViewModel::from_history(&history, "");
        let row_count: usize = view_model
            .sections
            .iter()
            .map(|section| section.rows.len())
            .sum();
        assert_eq!(row_count, history.items().len());
    });

    let popup_search_view_model_duration = measure(|| {
        let view_model = PopupViewModel::from_history(&history, "item 99");
        let row_count: usize = view_model
            .sections
            .iter()
            .map(|section| section.rows.len())
            .sum();
        assert!(row_count > 0);
    });

    let path = std::env::temp_dir().join(format!(
        "clippo-bench-{}.json",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    let store = JsonFileStore::new(&path);

    let persistence_save_duration = measure(|| {
        store.save_history(history.items()).unwrap();
    });
    let persistence_load_duration = measure(|| {
        let loaded = store.load_history().unwrap();
        assert_eq!(loaded.len(), history.items().len());
    });
    let _ = fs::remove_file(path);

    println!("history_add_1000={}", format_duration(history_duration));
    println!("search_1000={}", format_duration(search_duration));
    println!(
        "popup_view_model_200={}",
        format_duration(popup_view_model_duration)
    );
    println!(
        "popup_search_view_model_200={}",
        format_duration(popup_search_view_model_duration)
    );
    println!(
        "persistence_save_1000={}",
        format_duration(persistence_save_duration)
    );
    println!(
        "persistence_load_1000={}",
        format_duration(persistence_load_duration)
    );
}

fn measure(operation: impl FnOnce()) -> Duration {
    let started = Instant::now();
    operation();
    started.elapsed()
}

fn format_duration(duration: Duration) -> String {
    format!("{}us", duration.as_micros())
}
