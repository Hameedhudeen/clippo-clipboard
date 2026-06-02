mod commands;
mod diagnostics;
mod history;
mod lifecycle;
mod localization;
mod onboarding;
mod privacy;
mod search;
mod selection;
mod settings;
mod time_format;
mod types;
mod view_model;

pub use commands::{ClippoCommand, CommandDecision, CommandRouter, PasteMode, PastePlan};
pub use diagnostics::{DiagnosticEvent, DiagnosticSnapshot};
pub use history::{CaptureState, ClipboardHistory, HistoryConfig, HistoryError};
pub use lifecycle::{AppLifecycleEvent, AppLifecycleState, LifecycleStatus};
pub use localization::{required_localization_keys, LOCALIZATION_KEYS};
pub use onboarding::{OnboardingProgress, OnboardingStep, OnboardingStepId, OnboardingViewModel};
pub use privacy::{CaptureContext, IgnoreDecision, PrivacyRules};
pub use search::{search_items, SearchMatch};
pub use selection::{SelectionMove, SelectionState, SelectionWrapMode};
pub use settings::{
    search_settings, AppearanceMode, ClippoSettings, SettingsError, SettingsSearchResult,
    ShortcutConflict, ShortcutSettings, ValidatedSettings,
};
pub use time_format::{format_timestamp_for_locale, DateTimeFormatStyle};
pub use types::{
    BoundedText, ClipboardContent, ClipboardItem, ClipboardSource, ContentKind, ItemId,
    TimestampMillis, FULL_PREVIEW_TEXT_LIMIT, PREVIEW_TEXT_LIMIT,
};
pub use view_model::{
    FooterAction, PopupFullPreview, PopupRow, PopupSection, PopupState, PopupViewModel,
};
