use std::{
    fs,
    io::{self, ErrorKind},
    path::PathBuf,
};

use clippo_core::{ClipboardItem, ClippoSettings};
use serde::{Deserialize, Serialize};

pub const LATEST_SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistenceError {
    pub code: PersistenceErrorCode,
    pub message: String,
}

impl PersistenceError {
    #[must_use]
    pub fn new(code: PersistenceErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PersistenceErrorCode {
    NotFound,
    Corrupt,
    MigrationRequired,
    PermissionDenied,
    Io,
    Unknown,
}

pub type PersistenceResult<T> = Result<T, PersistenceError>;

pub trait HistoryStore {
    fn load_history(&self) -> PersistenceResult<Vec<ClipboardItem>>;
    fn save_history(&self, items: &[ClipboardItem]) -> PersistenceResult<()>;
    fn clear_history(&self) -> PersistenceResult<()>;
}

pub trait SettingsStore {
    fn load_settings(&self) -> PersistenceResult<ClippoSettings>;
    fn save_settings(&self, settings: &ClippoSettings) -> PersistenceResult<()>;
}

pub trait MigrationStore {
    fn current_schema_version(&self) -> PersistenceResult<u32>;
    fn migrate_to_latest(&self) -> PersistenceResult<()>;
}

#[derive(Debug, Clone)]
pub struct JsonFileStore {
    path: PathBuf,
    retention_limit: Option<usize>,
}

impl JsonFileStore {
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            path: path.into(),
            retention_limit: None,
        }
    }

    #[must_use]
    pub fn with_retention_limit(mut self, limit: usize) -> Self {
        self.retention_limit = Some(limit);
        self
    }

    fn load_document(&self) -> PersistenceResult<PersistedDocument> {
        match fs::read_to_string(&self.path) {
            Ok(raw) => serde_json::from_str(&raw).map_err(|error| corrupt_json(&error)),
            Err(error) if error.kind() == ErrorKind::NotFound => Ok(PersistedDocument::default()),
            Err(error) => Err(io_error(&error)),
        }
    }

    fn save_document(&self, document: &PersistedDocument) -> PersistenceResult<()> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent).map_err(|error| io_error(&error))?;
        }

        let temp_path = self.path.with_extension("tmp");
        let raw = serde_json::to_string_pretty(document).map_err(|error| corrupt_json(&error))?;
        fs::write(&temp_path, raw).map_err(|error| io_error(&error))?;
        fs::rename(&temp_path, &self.path).map_err(|error| io_error(&error))?;
        Ok(())
    }

    pub fn export_history_json(items: &[ClipboardItem]) -> PersistenceResult<String> {
        serde_json::to_string_pretty(items).map_err(|error| corrupt_json(&error))
    }

    pub fn import_history_json(value: &str) -> PersistenceResult<Vec<ClipboardItem>> {
        serde_json::from_str(value).map_err(|error| corrupt_json(&error))
    }
}

impl HistoryStore for JsonFileStore {
    fn load_history(&self) -> PersistenceResult<Vec<ClipboardItem>> {
        Ok(self.load_document()?.history)
    }

    fn save_history(&self, items: &[ClipboardItem]) -> PersistenceResult<()> {
        let mut document = self.load_document()?;
        let mut history = items.to_vec();
        if let Some(limit) = self.retention_limit {
            history.truncate(limit);
        }
        document.history = history;
        document.schema_version = LATEST_SCHEMA_VERSION;
        self.save_document(&document)
    }

    fn clear_history(&self) -> PersistenceResult<()> {
        let mut document = self.load_document()?;
        document.history.clear();
        document.schema_version = LATEST_SCHEMA_VERSION;
        self.save_document(&document)
    }
}

impl SettingsStore for JsonFileStore {
    fn load_settings(&self) -> PersistenceResult<ClippoSettings> {
        Ok(self.load_document()?.settings)
    }

    fn save_settings(&self, settings: &ClippoSettings) -> PersistenceResult<()> {
        settings.validate().map_err(|error| {
            PersistenceError::new(PersistenceErrorCode::Unknown, format!("{error:?}"))
        })?;
        let mut document = self.load_document()?;
        document.settings = settings.clone();
        document.schema_version = LATEST_SCHEMA_VERSION;
        self.save_document(&document)
    }
}

impl MigrationStore for JsonFileStore {
    fn current_schema_version(&self) -> PersistenceResult<u32> {
        Ok(self.load_document()?.schema_version)
    }

    fn migrate_to_latest(&self) -> PersistenceResult<()> {
        let mut document = self.load_document()?;
        document.schema_version = LATEST_SCHEMA_VERSION;
        self.save_document(&document)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct PersistedDocument {
    schema_version: u32,
    history: Vec<ClipboardItem>,
    settings: ClippoSettings,
}

impl Default for PersistedDocument {
    fn default() -> Self {
        Self {
            schema_version: LATEST_SCHEMA_VERSION,
            history: Vec::new(),
            settings: ClippoSettings::default(),
        }
    }
}

fn io_error(error: &io::Error) -> PersistenceError {
    PersistenceError::new(PersistenceErrorCode::Io, error.to_string())
}

fn corrupt_json(error: &serde_json::Error) -> PersistenceError {
    PersistenceError::new(PersistenceErrorCode::Corrupt, error.to_string())
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use clippo_core::{ClipboardContent, ClipboardHistory, TimestampMillis};

    use super::*;

    fn temp_file(name: &str) -> PathBuf {
        let stamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("clippo-{name}-{stamp}.json"))
    }

    fn sample_history() -> Vec<ClipboardItem> {
        let mut history = ClipboardHistory::default();
        history
            .add_item(
                ClipboardContent::Text {
                    text: "one".to_string(),
                },
                TimestampMillis(1),
                None,
            )
            .unwrap();
        history
            .add_item(
                ClipboardContent::Text {
                    text: "two".to_string(),
                },
                TimestampMillis(2),
                None,
            )
            .unwrap();
        history.items().to_vec()
    }

    #[test]
    fn saves_and_loads_history() {
        let path = temp_file("history");
        let store = JsonFileStore::new(&path);
        let items = sample_history();

        store.save_history(&items).unwrap();

        assert_eq!(store.load_history().unwrap(), items);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn saves_and_loads_settings() {
        let path = temp_file("settings");
        let store = JsonFileStore::new(&path);
        let settings = ClippoSettings {
            history_limit: 42,
            ..ClippoSettings::default()
        };

        store.save_settings(&settings).unwrap();

        assert_eq!(store.load_settings().unwrap().history_limit, 42);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn applies_retention_limit_when_saving_history() {
        let path = temp_file("retention");
        let store = JsonFileStore::new(&path).with_retention_limit(1);

        store.save_history(&sample_history()).unwrap();

        assert_eq!(store.load_history().unwrap().len(), 1);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn migrates_schema_version_to_latest() {
        let path = temp_file("migration");
        fs::write(
            &path,
            r#"{"schema_version":0,"history":[],"settings":{"history_limit":200,"max_item_bytes":5242880,"clipboard_check_interval_ms":500,"paste_automatically":true,"launch_at_login":false,"show_footer":true,"show_tray_or_menu_icon":true,"appearance":"System","shortcuts":{"open_history":"Shift+Meta+C","paste_selected":"Alt+Enter","paste_without_formatting":"Alt+Shift+Enter","delete_selected":"Alt+Delete","pin_selected":"Alt+P","clear_unpinned":"Alt+Meta+Delete","clear_all":"Shift+Alt+Meta+Delete","open_preferences":"Meta+Comma"},"ignored_clipboard_types":[],"ignored_applications":[],"ignored_content_patterns":[]}}"#,
        )
        .unwrap();
        let store = JsonFileStore::new(&path);

        store.migrate_to_latest().unwrap();

        assert_eq!(
            store.current_schema_version().unwrap(),
            LATEST_SCHEMA_VERSION
        );
        let _ = fs::remove_file(path);
    }

    #[test]
    fn reports_corrupt_documents() {
        let path = temp_file("corrupt");
        fs::write(&path, "not json").unwrap();
        let store = JsonFileStore::new(&path);

        let error = store.load_history().unwrap_err();

        assert_eq!(error.code, PersistenceErrorCode::Corrupt);
        let _ = fs::remove_file(path);
    }

    #[test]
    fn clears_history() {
        let path = temp_file("clear");
        let store = JsonFileStore::new(&path);
        store.save_history(&sample_history()).unwrap();

        store.clear_history().unwrap();

        assert!(store.load_history().unwrap().is_empty());
        let _ = fs::remove_file(path);
    }

    #[test]
    fn exports_and_imports_history_json() {
        let items = sample_history();

        let exported = JsonFileStore::export_history_json(&items).unwrap();
        let imported = JsonFileStore::import_history_json(&exported).unwrap();

        assert_eq!(items, imported);
    }
}
