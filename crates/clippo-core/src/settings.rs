use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::HistoryConfig;

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClippoSettings {
    pub history_limit: usize,
    pub max_item_bytes: usize,
    pub clipboard_check_interval_ms: u64,
    pub paste_automatically: bool,
    pub launch_at_login: bool,
    pub show_footer: bool,
    pub show_tray_or_menu_icon: bool,
    pub appearance: AppearanceMode,
    pub shortcuts: ShortcutSettings,
    pub ignored_clipboard_types: Vec<String>,
    pub ignored_applications: Vec<String>,
    pub ignored_content_patterns: Vec<String>,
}

impl Default for ClippoSettings {
    fn default() -> Self {
        Self {
            history_limit: 200,
            max_item_bytes: 5 * 1024 * 1024,
            clipboard_check_interval_ms: 500,
            paste_automatically: true,
            launch_at_login: false,
            show_footer: true,
            show_tray_or_menu_icon: true,
            appearance: AppearanceMode::System,
            shortcuts: ShortcutSettings::default(),
            ignored_clipboard_types: Vec::new(),
            ignored_applications: Vec::new(),
            ignored_content_patterns: Vec::new(),
        }
    }
}

impl ClippoSettings {
    pub fn validate(&self) -> Result<ValidatedSettings, SettingsError> {
        if self.history_limit == 0 {
            return Err(SettingsError::HistoryLimitMustBePositive);
        }

        if self.max_item_bytes == 0 {
            return Err(SettingsError::MaxItemBytesMustBePositive);
        }

        if self.clipboard_check_interval_ms < 100 {
            return Err(SettingsError::ClipboardCheckIntervalTooLow {
                minimum_ms: 100,
                actual_ms: self.clipboard_check_interval_ms,
            });
        }

        self.shortcuts.validate()?;

        Ok(ValidatedSettings {
            settings: self.clone(),
        })
    }

    pub fn reset_to_defaults(&mut self) {
        *self = Self::default();
    }

    #[must_use]
    pub fn history_config(&self) -> HistoryConfig {
        HistoryConfig {
            max_items: self.history_limit,
            max_item_bytes: self.max_item_bytes,
        }
    }

    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    pub fn import_json(value: &str) -> Result<Self, SettingsImportError> {
        let settings = serde_json::from_str::<Self>(value)?;
        settings.validate()?;
        Ok(settings)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SettingsSearchResult {
    pub key: &'static str,
    pub category_key: &'static str,
    pub title_key: &'static str,
    pub description_key: &'static str,
    pub search_terms: &'static str,
}

#[must_use]
pub fn search_settings(query: &str) -> Vec<SettingsSearchResult> {
    let tokens = query
        .split_whitespace()
        .map(str::trim)
        .filter(|token| !token.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return SETTINGS_CATALOG.to_vec();
    }

    let mut matches = SETTINGS_CATALOG
        .iter()
        .copied()
        .filter_map(|entry| {
            let haystack = format!("{} {}", entry.key, entry.search_terms).to_ascii_lowercase();

            if !tokens.iter().all(|token| haystack.contains(token)) {
                return None;
            }

            let title = entry.title_key.to_ascii_lowercase();
            let category = entry.category_key.to_ascii_lowercase();
            let score = if tokens.iter().any(|token| title.starts_with(token)) {
                0
            } else if tokens.iter().any(|token| category.starts_with(token)) {
                1
            } else {
                2
            };

            Some((score, entry.title_key, entry))
        })
        .collect::<Vec<_>>();

    matches.sort_by(|left, right| left.0.cmp(&right.0).then_with(|| left.1.cmp(right.1)));
    matches.into_iter().map(|(_, _, entry)| entry).collect()
}

const SETTINGS_CATALOG: &[SettingsSearchResult] = &[
    SettingsSearchResult {
        key: "history_limit",
        category_key: "settings.history_limit.category",
        title_key: "settings.history_limit.title",
        description_key: "settings.history_limit.description",
        search_terms: "history limit maximum number unpinned clipboard items keeps",
    },
    SettingsSearchResult {
        key: "max_item_bytes",
        category_key: "settings.max_item_bytes.category",
        title_key: "settings.max_item_bytes.title",
        description_key: "settings.max_item_bytes.description",
        search_terms: "history maximum item size skip large clipboard entries responsive",
    },
    SettingsSearchResult {
        key: "clipboard_check_interval_ms",
        category_key: "settings.clipboard_check_interval_ms.category",
        title_key: "settings.clipboard_check_interval_ms.title",
        description_key: "settings.clipboard_check_interval_ms.description",
        search_terms: "history clipboard check interval polling frequency events",
    },
    SettingsSearchResult {
        key: "paste_automatically",
        category_key: "settings.paste_automatically.category",
        title_key: "settings.paste_automatically.title",
        description_key: "settings.paste_automatically.description",
        search_terms: "paste automatically selected history item focused app permissions",
    },
    SettingsSearchResult {
        key: "launch_at_login",
        category_key: "settings.launch_at_login.category",
        title_key: "settings.launch_at_login.title",
        description_key: "settings.launch_at_login.description",
        search_terms: "startup launch login start automatically signing in",
    },
    SettingsSearchResult {
        key: "show_footer",
        category_key: "settings.show_footer.category",
        title_key: "settings.show_footer.title",
        description_key: "settings.show_footer.description",
        search_terms: "appearance show hide footer actions history popup",
    },
    SettingsSearchResult {
        key: "show_tray_or_menu_icon",
        category_key: "settings.show_tray_or_menu_icon.category",
        title_key: "settings.show_tray_or_menu_icon.title",
        description_key: "settings.show_tray_or_menu_icon.description",
        search_terms: "appearance show tray menu icon desktop shell entry point",
    },
    SettingsSearchResult {
        key: "appearance",
        category_key: "settings.appearance.category",
        title_key: "settings.appearance.title",
        description_key: "settings.appearance.description",
        search_terms: "appearance theme system light dark",
    },
    SettingsSearchResult {
        key: "shortcuts.open_history",
        category_key: "settings.shortcuts.open_history.category",
        title_key: "settings.shortcuts.open_history.title",
        description_key: "settings.shortcuts.open_history.description",
        search_terms: "shortcuts open history global shortcut clipboard popup",
    },
    SettingsSearchResult {
        key: "shortcuts.paste_selected",
        category_key: "settings.shortcuts.paste_selected.category",
        title_key: "settings.shortcuts.paste_selected.title",
        description_key: "settings.shortcuts.paste_selected.description",
        search_terms: "shortcuts paste selected item",
    },
    SettingsSearchResult {
        key: "shortcuts.paste_without_formatting",
        category_key: "settings.shortcuts.paste_without_formatting.category",
        title_key: "settings.shortcuts.paste_without_formatting.title",
        description_key: "settings.shortcuts.paste_without_formatting.description",
        search_terms: "shortcuts paste without formatting plain text rich clipboard content",
    },
    SettingsSearchResult {
        key: "shortcuts.delete_selected",
        category_key: "settings.shortcuts.delete_selected.category",
        title_key: "settings.shortcuts.delete_selected.title",
        description_key: "settings.shortcuts.delete_selected.description",
        search_terms: "shortcuts delete selected remove item history",
    },
    SettingsSearchResult {
        key: "shortcuts.pin_selected",
        category_key: "settings.shortcuts.pin_selected.category",
        title_key: "settings.shortcuts.pin_selected.title",
        description_key: "settings.shortcuts.pin_selected.description",
        search_terms: "shortcuts pin unpin selected item",
    },
    SettingsSearchResult {
        key: "shortcuts.clear_unpinned",
        category_key: "settings.shortcuts.clear_unpinned.category",
        title_key: "settings.shortcuts.clear_unpinned.title",
        description_key: "settings.shortcuts.clear_unpinned.description",
        search_terms: "shortcuts clear unpinned regular history keeping pinned items",
    },
    SettingsSearchResult {
        key: "shortcuts.clear_all",
        category_key: "settings.shortcuts.clear_all.category",
        title_key: "settings.shortcuts.clear_all.title",
        description_key: "settings.shortcuts.clear_all.description",
        search_terms: "shortcuts clear all full clipboard history",
    },
    SettingsSearchResult {
        key: "shortcuts.open_preferences",
        category_key: "settings.shortcuts.open_preferences.category",
        title_key: "settings.shortcuts.open_preferences.title",
        description_key: "settings.shortcuts.open_preferences.description",
        search_terms: "shortcuts open preferences",
    },
    SettingsSearchResult {
        key: "ignored_clipboard_types",
        category_key: "settings.ignored_clipboard_types.category",
        title_key: "settings.ignored_clipboard_types.title",
        description_key: "settings.ignored_clipboard_types.description",
        search_terms: "privacy ignored clipboard types formats never saved",
    },
    SettingsSearchResult {
        key: "ignored_applications",
        category_key: "settings.ignored_applications.category",
        title_key: "settings.ignored_applications.title",
        description_key: "settings.ignored_applications.description",
        search_terms: "privacy ignored applications apps clipboard changes ignored supported",
    },
    SettingsSearchResult {
        key: "ignored_content_patterns",
        category_key: "settings.ignored_content_patterns.category",
        title_key: "settings.ignored_content_patterns.title",
        description_key: "settings.ignored_content_patterns.description",
        search_terms: "privacy ignored content patterns text sensitive content",
    },
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatedSettings {
    settings: ClippoSettings,
}

impl ValidatedSettings {
    #[must_use]
    pub fn into_inner(self) -> ClippoSettings {
        self.settings
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsImportError {
    Json(String),
    Validation(SettingsError),
}

impl From<serde_json::Error> for SettingsImportError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value.to_string())
    }
}

impl From<SettingsError> for SettingsImportError {
    fn from(value: SettingsError) -> Self {
        Self::Validation(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SettingsError {
    HistoryLimitMustBePositive,
    MaxItemBytesMustBePositive,
    ClipboardCheckIntervalTooLow {
        minimum_ms: u64,
        actual_ms: u64,
    },
    EmptyShortcut {
        action: &'static str,
    },
    ReservedShortcut {
        action: &'static str,
        shortcut: String,
    },
    DuplicateShortcut(ShortcutConflict),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutConflict {
    pub shortcut: String,
    pub first_action: &'static str,
    pub second_action: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AppearanceMode {
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShortcutSettings {
    pub open_history: String,
    pub paste_selected: String,
    pub paste_without_formatting: String,
    pub delete_selected: String,
    pub pin_selected: String,
    pub clear_unpinned: String,
    pub clear_all: String,
    pub open_preferences: String,
}

impl Default for ShortcutSettings {
    fn default() -> Self {
        Self {
            open_history: "Shift+Meta+C".to_string(),
            paste_selected: "Alt+Enter".to_string(),
            paste_without_formatting: "Alt+Shift+Enter".to_string(),
            delete_selected: "Alt+Delete".to_string(),
            pin_selected: "Alt+P".to_string(),
            clear_unpinned: "Alt+Meta+Delete".to_string(),
            clear_all: "Shift+Alt+Meta+Delete".to_string(),
            open_preferences: "Meta+Comma".to_string(),
        }
    }
}

impl ShortcutSettings {
    pub fn validate(&self) -> Result<(), SettingsError> {
        let shortcuts = [
            ("open_history", self.open_history.as_str()),
            ("paste_selected", self.paste_selected.as_str()),
            (
                "paste_without_formatting",
                self.paste_without_formatting.as_str(),
            ),
            ("delete_selected", self.delete_selected.as_str()),
            ("pin_selected", self.pin_selected.as_str()),
            ("clear_unpinned", self.clear_unpinned.as_str()),
            ("clear_all", self.clear_all.as_str()),
            ("open_preferences", self.open_preferences.as_str()),
        ];

        let reserved = reserved_shortcuts();
        let mut seen: HashMap<String, &'static str> = HashMap::new();

        for (action, shortcut) in shortcuts {
            let normalized = normalize_shortcut(shortcut);
            if normalized.is_empty() {
                return Err(SettingsError::EmptyShortcut { action });
            }

            if reserved.contains(normalized.as_str()) {
                return Err(SettingsError::ReservedShortcut {
                    action,
                    shortcut: shortcut.to_string(),
                });
            }

            if let Some(first_action) = seen.insert(normalized.clone(), action) {
                return Err(SettingsError::DuplicateShortcut(ShortcutConflict {
                    shortcut: normalized,
                    first_action,
                    second_action: action,
                }));
            }
        }

        Ok(())
    }
}

fn reserved_shortcuts() -> HashSet<&'static str> {
    HashSet::from(["alt+f4", "meta+q", "ctrl+alt+delete"])
}

fn normalize_shortcut(value: &str) -> String {
    value
        .split('+')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(str::to_lowercase)
        .collect::<Vec<_>>()
        .join("+")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_privacy_preserving() {
        let settings = ClippoSettings::default();

        assert!(settings.paste_automatically);
        assert!(!settings.launch_at_login);
        assert_eq!(settings.history_limit, 200);
    }

    #[test]
    fn validates_default_settings() {
        ClippoSettings::default().validate().unwrap();
    }

    #[test]
    fn rejects_duplicate_shortcuts() {
        let mut settings = ClippoSettings::default();
        settings.shortcuts.clear_all = settings.shortcuts.open_history.clone();

        let error = settings.validate().unwrap_err();

        assert!(matches!(error, SettingsError::DuplicateShortcut(_)));
    }

    #[test]
    fn imports_and_exports_settings_json() {
        let settings = ClippoSettings::default();
        let exported = settings.export_json().unwrap();
        let imported = ClippoSettings::import_json(&exported).unwrap();

        assert_eq!(settings, imported);
    }

    #[test]
    fn searches_settings_by_title_category_and_description() {
        let shortcut_results = search_settings("shortcut");
        assert!(shortcut_results
            .iter()
            .any(|result| result.key == "shortcuts.open_history"));

        let privacy_results = search_settings("privacy applications");
        assert_eq!(privacy_results.len(), 1);
        assert_eq!(privacy_results[0].key, "ignored_applications");

        let paste_results = search_settings("plain text");
        assert_eq!(
            paste_results
                .iter()
                .map(|result| result.key)
                .collect::<Vec<_>>(),
            vec!["shortcuts.paste_without_formatting"]
        );
    }

    #[test]
    fn empty_settings_search_returns_full_catalog() {
        let results = search_settings("   ");

        assert!(results.len() >= 10);
        assert!(results.iter().any(|result| result.key == "history_limit"));
    }
}
