use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use clippo_core::{
    ClipboardContent, ClipboardHistory, ClipboardSource, CommandRouter, ItemId, PasteMode,
    TimestampMillis,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformError {
    pub code: PlatformErrorCode,
    pub message: String,
}

impl PlatformError {
    #[must_use]
    pub fn new(code: PlatformErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformErrorCode {
    PermissionDenied,
    Unsupported,
    Unavailable,
    Conflict,
    Unknown,
}

pub type PlatformResult<T> = Result<T, PlatformError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClipboardSnapshot {
    pub content: ClipboardContent,
    pub source_application: Option<String>,
    pub sequence: Option<u64>,
}

pub trait ClipboardService {
    fn read_clipboard(&self) -> PlatformResult<Option<ClipboardSnapshot>>;
    fn read_clipboard_batch(&self) -> PlatformResult<Vec<ClipboardSnapshot>> {
        Ok(self.read_clipboard()?.into_iter().collect())
    }
    fn write_clipboard(&self, content: &ClipboardContent) -> PlatformResult<()>;
}

pub trait PasteService {
    fn paste_active_clipboard(&self) -> PlatformResult<()>;
    fn paste_plain_text(&self, text: &str) -> PlatformResult<()>;
}

pub trait FocusService {
    fn target_application_available(&self) -> PlatformResult<bool>;
}

pub trait GlobalShortcutService {
    fn register(&mut self, action: ShortcutAction, shortcut: &str) -> PlatformResult<()>;
    fn unregister(&mut self, action: ShortcutAction) -> PlatformResult<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShortcutAction {
    OpenHistory,
    PasteSelected,
    PasteWithoutFormatting,
    DeleteSelected,
    PinSelected,
    ClearUnpinned,
    ClearAll,
    OpenPreferences,
    PauseCapture,
    IgnoreNextCopy,
}

pub trait NotificationService {
    fn notify(&self, title: &str, body: &str) -> PlatformResult<()>;
}

pub trait TrayMenuService {
    fn set_visible(&self, visible: bool) -> PlatformResult<()>;
    fn set_paused(&self, paused: bool) -> PlatformResult<()>;
    fn show_menu(&self) -> PlatformResult<()>;
}

pub trait AutostartService {
    fn is_enabled(&self) -> PlatformResult<bool>;
    fn set_enabled(&self, enabled: bool) -> PlatformResult<()>;
}

pub trait PermissionService {
    fn paste_automation_allowed(&self) -> PlatformResult<bool>;
    fn open_permission_help(&self) -> PlatformResult<()>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchOutcome {
    PrimaryInstance(SingleInstanceGuard),
    FocusExistingInstance,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleInstanceGuard {
    lock_path: PathBuf,
}

impl SingleInstanceGuard {
    #[must_use]
    pub fn lock_path(&self) -> &Path {
        &self.lock_path
    }
}

impl Drop for SingleInstanceGuard {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.lock_path);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleInstance {
    lock_path: PathBuf,
}

impl SingleInstance {
    #[must_use]
    pub fn new(lock_path: impl Into<PathBuf>) -> Self {
        Self {
            lock_path: lock_path.into(),
        }
    }

    pub fn launch(&self) -> PlatformResult<LaunchOutcome> {
        if let Some(parent) = self.lock_path.parent() {
            fs::create_dir_all(parent).map_err(|error| io_platform_error(&error))?;
        }

        match self.create_lock_guard() {
            Ok(outcome) => Ok(outcome),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                if self.lock_file_has_active_owner()? {
                    return Ok(LaunchOutcome::FocusExistingInstance);
                }
                fs::remove_file(&self.lock_path).map_err(|error| io_platform_error(&error))?;
                self.create_lock_guard()
                    .map_err(|error| io_platform_error(&error))
            }
            Err(error) => Err(io_platform_error(&error)),
        }
    }

    fn create_lock_guard(&self) -> io::Result<LaunchOutcome> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&self.lock_path)?;
        write!(file, "{}", std::process::id())?;
        Ok(LaunchOutcome::PrimaryInstance(SingleInstanceGuard {
            lock_path: self.lock_path.clone(),
        }))
    }

    fn lock_file_has_active_owner(&self) -> PlatformResult<bool> {
        let raw_pid =
            fs::read_to_string(&self.lock_path).map_err(|error| io_platform_error(&error))?;
        let Ok(pid) = raw_pid.trim().parse::<u32>() else {
            return Ok(false);
        };
        Ok(process_is_running(pid))
    }
}

fn io_platform_error(error: &io::Error) -> PlatformError {
    PlatformError::new(PlatformErrorCode::Unknown, error.to_string())
}

fn process_is_running(pid: u32) -> bool {
    if pid == std::process::id() {
        return true;
    }
    platform_process_is_running(pid)
}

#[cfg(target_os = "linux")]
fn platform_process_is_running(pid: u32) -> bool {
    Path::new("/proc").join(pid.to_string()).exists()
}

#[cfg(not(target_os = "linux"))]
fn platform_process_is_running(_pid: u32) -> bool {
    true
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformKind {
    MacOs,
    Windows,
    Linux,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PasteTimingProfile {
    pub clipboard_settle_ms: u64,
    pub focus_restore_ms: u64,
    pub paste_keystroke_gap_ms: u64,
    pub permission_help_delay_ms: u64,
}

#[must_use]
pub fn default_paste_timing(platform: PlatformKind) -> PasteTimingProfile {
    match platform {
        PlatformKind::MacOs => PasteTimingProfile {
            clipboard_settle_ms: 60,
            focus_restore_ms: 90,
            paste_keystroke_gap_ms: 20,
            permission_help_delay_ms: 250,
        },
        PlatformKind::Windows => PasteTimingProfile {
            clipboard_settle_ms: 80,
            focus_restore_ms: 120,
            paste_keystroke_gap_ms: 30,
            permission_help_delay_ms: 300,
        },
        PlatformKind::Linux => PasteTimingProfile {
            clipboard_settle_ms: 100,
            focus_restore_ms: 150,
            paste_keystroke_gap_ms: 35,
            permission_help_delay_ms: 350,
        },
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutMapping {
    pub action: ShortcutAction,
    pub shortcut: String,
    pub display_label: String,
}

#[must_use]
pub fn default_shortcut_mappings(platform: PlatformKind) -> Vec<ShortcutMapping> {
    let meta = match platform {
        PlatformKind::MacOs => "Command",
        PlatformKind::Windows => "Windows",
        PlatformKind::Linux => "Super",
    };

    vec![
        mapping(
            ShortcutAction::OpenHistory,
            "Shift+Meta+C",
            &format!("Shift+{meta}+C"),
        ),
        mapping(ShortcutAction::PasteSelected, "Alt+Enter", "Alt+Enter"),
        mapping(
            ShortcutAction::PasteWithoutFormatting,
            "Alt+Shift+Enter",
            "Alt+Shift+Enter",
        ),
        mapping(ShortcutAction::DeleteSelected, "Alt+Delete", "Alt+Delete"),
        mapping(ShortcutAction::PinSelected, "Alt+P", "Alt+P"),
        mapping(
            ShortcutAction::ClearUnpinned,
            "Alt+Meta+Delete",
            &format!("Alt+{meta}+Delete"),
        ),
        mapping(
            ShortcutAction::ClearAll,
            "Shift+Alt+Meta+Delete",
            &format!("Shift+Alt+{meta}+Delete"),
        ),
        mapping(
            ShortcutAction::OpenPreferences,
            "Meta+Comma",
            &format!("{meta}+Comma"),
        ),
        mapping(
            ShortcutAction::PauseCapture,
            "Alt+MenuIcon",
            "Alt+Menu Icon",
        ),
        mapping(
            ShortcutAction::IgnoreNextCopy,
            "Alt+Shift+MenuIcon",
            "Alt+Shift+Menu Icon",
        ),
    ]
}

fn mapping(action: ShortcutAction, shortcut: &str, display_label: &str) -> ShortcutMapping {
    ShortcutMapping {
        action,
        shortcut: shortcut.to_string(),
        display_label: display_label.to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardActionOutcome {
    Copied { item_id: ItemId },
    Pasted { item_id: ItemId, mode: PasteMode },
    PasteUnavailable { item_id: ItemId, reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PasteFallbackGuidance {
    pub title_key: String,
    pub body_key: String,
    pub primary_action_label_key: String,
}

impl ClipboardActionOutcome {
    #[must_use]
    pub fn fallback_guidance(&self) -> Option<PasteFallbackGuidance> {
        let Self::PasteUnavailable { reason, .. } = self else {
            return None;
        };

        let body_key = if reason.contains("permission") {
            "paste.unavailable.permission.body"
        } else if reason.contains("focus") {
            "paste.unavailable.focus.body"
        } else {
            "paste.unavailable.generic.body"
        };

        Some(PasteFallbackGuidance {
            title_key: "paste.unavailable.title".to_string(),
            body_key: body_key.to_string(),
            primary_action_label_key: "paste.unavailable.manual_action".to_string(),
        })
    }
}

pub struct ClipboardActionController<C, P, F, Perm> {
    clipboard: C,
    paste: P,
    focus: F,
    permissions: Perm,
}

impl<C, P, F, Perm> ClipboardActionController<C, P, F, Perm>
where
    C: ClipboardService,
    P: PasteService,
    F: FocusService,
    Perm: PermissionService,
{
    pub fn new(clipboard: C, paste: P, focus: F, permissions: Perm) -> Self {
        Self {
            clipboard,
            paste,
            focus,
            permissions,
        }
    }

    pub fn copy_selected(
        &self,
        history: &mut ClipboardHistory,
        item_id: ItemId,
    ) -> PlatformResult<ClipboardActionOutcome> {
        let content = history
            .items()
            .iter()
            .find(|item| item.id == item_id)
            .ok_or_else(|| {
                PlatformError::new(PlatformErrorCode::Unavailable, "history item not found")
            })?
            .content
            .clone();
        self.clipboard.write_clipboard(&content)?;
        history.mark_internal_clipboard_write(&content);
        Ok(ClipboardActionOutcome::Copied { item_id })
    }

    pub fn paste_selected(
        &self,
        history: &mut ClipboardHistory,
        item_id: ItemId,
        mode: PasteMode,
    ) -> PlatformResult<ClipboardActionOutcome> {
        if !self.focus.target_application_available()? {
            return Ok(ClipboardActionOutcome::PasteUnavailable {
                item_id,
                reason: "target application focus is unavailable".to_string(),
            });
        }

        let plan = CommandRouter::plan_paste(history, item_id, mode).map_err(|_| {
            PlatformError::new(PlatformErrorCode::Unavailable, "history item not found")
        })?;
        self.clipboard.write_clipboard(&plan.clipboard_content)?;
        history.mark_internal_clipboard_write(&plan.clipboard_content);

        if !self.permissions.paste_automation_allowed()? {
            self.permissions.open_permission_help()?;
            return Ok(ClipboardActionOutcome::PasteUnavailable {
                item_id,
                reason: "paste automation permission is not granted".to_string(),
            });
        }

        match &plan.clipboard_content {
            ClipboardContent::Text { text } if mode == PasteMode::PlainText => {
                self.paste.paste_plain_text(text)?;
            }
            _ => self.paste.paste_active_clipboard()?,
        }

        Ok(ClipboardActionOutcome::Pasted { item_id, mode })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClipboardPollOutcome {
    NoChange,
    IgnoredInternalWrite,
    Captured(ItemId),
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ClipboardPollBatchOutcome {
    pub captured_item_ids: Vec<ItemId>,
    pub ignored_internal_writes: usize,
    pub unchanged_snapshots: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClipboardPollingSchedule {
    floor: u64,
    ceiling: u64,
    current_interval_ms: u64,
}

impl ClipboardPollingSchedule {
    #[must_use]
    pub fn new(min_interval_ms: u64, max_interval_ms: u64) -> Self {
        let min_interval_ms = min_interval_ms.max(100);
        let max_interval_ms = max_interval_ms.max(min_interval_ms);

        Self {
            floor: min_interval_ms,
            ceiling: max_interval_ms,
            current_interval_ms: min_interval_ms,
        }
    }

    #[must_use]
    pub fn current_interval_ms(&self) -> u64 {
        self.current_interval_ms
    }

    pub fn observe(&mut self, outcome: &ClipboardPollOutcome) {
        match outcome {
            ClipboardPollOutcome::Captured(_) | ClipboardPollOutcome::IgnoredInternalWrite => {
                self.current_interval_ms = self.floor;
            }
            ClipboardPollOutcome::NoChange => {
                self.current_interval_ms =
                    (self.current_interval_ms.saturating_mul(2)).min(self.ceiling);
            }
        }
    }
}

impl Default for ClipboardPollingSchedule {
    fn default() -> Self {
        Self::new(250, 2_000)
    }
}

#[derive(Debug, Default)]
pub struct ClipboardPoller {
    last_sequence: Option<u64>,
    last_unsequenced_content: Option<ClipboardContent>,
    schedule: ClipboardPollingSchedule,
}

impl ClipboardPoller {
    #[must_use]
    pub fn with_schedule(schedule: ClipboardPollingSchedule) -> Self {
        Self {
            last_sequence: None,
            last_unsequenced_content: None,
            schedule,
        }
    }

    #[must_use]
    pub fn next_interval_ms(&self) -> u64 {
        self.schedule.current_interval_ms()
    }

    pub fn poll<S: ClipboardService>(
        &mut self,
        service: &S,
        history: &mut ClipboardHistory,
        now: TimestampMillis,
    ) -> PlatformResult<ClipboardPollOutcome> {
        let batch = self.poll_all(service, history, now)?;

        if let Some(item_id) = batch.captured_item_ids.last() {
            return Ok(ClipboardPollOutcome::Captured(*item_id));
        }
        if batch.ignored_internal_writes > 0 {
            return Ok(ClipboardPollOutcome::IgnoredInternalWrite);
        }
        Ok(ClipboardPollOutcome::NoChange)
    }

    pub fn poll_all<S: ClipboardService>(
        &mut self,
        service: &S,
        history: &mut ClipboardHistory,
        now: TimestampMillis,
    ) -> PlatformResult<ClipboardPollBatchOutcome> {
        let mut snapshots = service.read_clipboard_batch()?;
        snapshots.sort_by_key(|snapshot| snapshot.sequence.unwrap_or(u64::MAX));

        let mut batch = ClipboardPollBatchOutcome::default();

        for snapshot in snapshots {
            if let Some(sequence) = snapshot.sequence {
                if self
                    .last_sequence
                    .is_some_and(|last_sequence| sequence <= last_sequence)
                {
                    batch.unchanged_snapshots += 1;
                    continue;
                }
                self.last_sequence = Some(sequence);
            } else if self.last_unsequenced_content.as_ref() == Some(&snapshot.content) {
                batch.unchanged_snapshots += 1;
                continue;
            } else {
                self.last_unsequenced_content = Some(snapshot.content.clone());
            }

            if !history.should_capture_content(&snapshot.content) {
                batch.ignored_internal_writes += 1;
                continue;
            }

            let capture_time = TimestampMillis(now.0 + batch.captured_item_ids.len() as u128);
            let item_id = history
                .add_item(
                    snapshot.content,
                    capture_time,
                    snapshot
                        .source_application
                        .map(|application_name| ClipboardSource {
                            application_name: Some(application_name),
                            bundle_or_process_id: None,
                        }),
                )
                .map_err(|error| {
                    PlatformError::new(PlatformErrorCode::Unknown, format!("{error:?}"))
                })?;
            batch.captured_item_ids.push(item_id);
        }

        let schedule_outcome = if let Some(item_id) = batch.captured_item_ids.last() {
            ClipboardPollOutcome::Captured(*item_id)
        } else if batch.ignored_internal_writes > 0 {
            ClipboardPollOutcome::IgnoredInternalWrite
        } else {
            ClipboardPollOutcome::NoChange
        };
        self.observe_poll_outcome(schedule_outcome);

        Ok(batch)
    }

    fn observe_poll_outcome(&mut self, outcome: ClipboardPollOutcome) -> ClipboardPollOutcome {
        self.schedule.observe(&outcome);
        outcome
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use std::time::{SystemTime, UNIX_EPOCH};

    use clippo_core::{ClipboardContent, PasteMode, TimestampMillis};

    use super::*;

    #[derive(Clone, Default)]
    struct FakeClipboard {
        content: Rc<RefCell<Option<ClipboardContent>>>,
        snapshot: Rc<RefCell<Option<ClipboardSnapshot>>>,
    }

    impl ClipboardService for FakeClipboard {
        fn read_clipboard(&self) -> PlatformResult<Option<ClipboardSnapshot>> {
            Ok(self.snapshot.borrow().clone())
        }

        fn write_clipboard(&self, content: &ClipboardContent) -> PlatformResult<()> {
            self.content.replace(Some(content.clone()));
            Ok(())
        }
    }

    #[derive(Default)]
    struct FakePaste {
        active_count: Cell<usize>,
        plain_text: RefCell<Option<String>>,
    }

    impl PasteService for FakePaste {
        fn paste_active_clipboard(&self) -> PlatformResult<()> {
            self.active_count.set(self.active_count.get() + 1);
            Ok(())
        }

        fn paste_plain_text(&self, text: &str) -> PlatformResult<()> {
            self.plain_text.replace(Some(text.to_string()));
            Ok(())
        }
    }

    struct FakeFocus(bool);

    impl FocusService for FakeFocus {
        fn target_application_available(&self) -> PlatformResult<bool> {
            Ok(self.0)
        }
    }

    struct FakePermissions {
        allowed: bool,
        help_opened: Cell<bool>,
    }

    impl PermissionService for FakePermissions {
        fn paste_automation_allowed(&self) -> PlatformResult<bool> {
            Ok(self.allowed)
        }

        fn open_permission_help(&self) -> PlatformResult<()> {
            self.help_opened.set(true);
            Ok(())
        }
    }

    #[derive(Default)]
    struct BatchClipboard {
        snapshots: RefCell<Vec<ClipboardSnapshot>>,
    }

    impl ClipboardService for BatchClipboard {
        fn read_clipboard(&self) -> PlatformResult<Option<ClipboardSnapshot>> {
            Ok(self.snapshots.borrow().last().cloned())
        }

        fn read_clipboard_batch(&self) -> PlatformResult<Vec<ClipboardSnapshot>> {
            Ok(self.snapshots.borrow().clone())
        }

        fn write_clipboard(&self, _content: &ClipboardContent) -> PlatformResult<()> {
            Ok(())
        }
    }

    fn seeded_history() -> (ClipboardHistory, ItemId) {
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

    fn temp_lock_path() -> std::path::PathBuf {
        std::env::temp_dir().join(format!(
            "clippo-single-instance-{}.lock",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }

    #[test]
    fn defines_platform_paste_timing_profiles() {
        let mac = default_paste_timing(PlatformKind::MacOs);
        let windows = default_paste_timing(PlatformKind::Windows);
        let linux = default_paste_timing(PlatformKind::Linux);

        assert!(mac.clipboard_settle_ms > 0);
        assert!(windows.focus_restore_ms >= mac.focus_restore_ms);
        assert!(linux.permission_help_delay_ms >= windows.permission_help_delay_ms);
    }

    #[test]
    fn polling_schedule_backs_off_and_resets_after_activity() {
        let mut schedule = ClipboardPollingSchedule::new(100, 800);

        schedule.observe(&ClipboardPollOutcome::NoChange);
        assert_eq!(schedule.current_interval_ms(), 200);

        schedule.observe(&ClipboardPollOutcome::NoChange);
        schedule.observe(&ClipboardPollOutcome::NoChange);
        schedule.observe(&ClipboardPollOutcome::NoChange);
        assert_eq!(schedule.current_interval_ms(), 800);

        schedule.observe(&ClipboardPollOutcome::Captured(ItemId(1)));
        assert_eq!(schedule.current_interval_ms(), 100);
    }

    #[test]
    fn copy_selected_writes_item_to_clipboard() {
        let (mut history, item_id) = seeded_history();
        let controller = ClipboardActionController::new(
            FakeClipboard::default(),
            FakePaste::default(),
            FakeFocus(true),
            FakePermissions {
                allowed: true,
                help_opened: Cell::new(false),
            },
        );

        let outcome = controller.copy_selected(&mut history, item_id).unwrap();

        assert_eq!(outcome, ClipboardActionOutcome::Copied { item_id });
    }

    #[test]
    fn paste_selected_uses_plain_text_path() {
        let (mut history, item_id) = seeded_history();
        let controller = ClipboardActionController::new(
            FakeClipboard::default(),
            FakePaste::default(),
            FakeFocus(true),
            FakePermissions {
                allowed: true,
                help_opened: Cell::new(false),
            },
        );

        let outcome = controller
            .paste_selected(&mut history, item_id, PasteMode::PlainText)
            .unwrap();

        assert_eq!(
            outcome,
            ClipboardActionOutcome::Pasted {
                item_id,
                mode: PasteMode::PlainText
            }
        );
    }

    #[test]
    fn paste_selected_leaves_selected_item_on_clipboard() {
        let (mut history, item_id) = seeded_history();
        let clipboard = FakeClipboard::default();
        let written_content = Rc::clone(&clipboard.content);
        let controller = ClipboardActionController::new(
            clipboard,
            FakePaste::default(),
            FakeFocus(true),
            FakePermissions {
                allowed: true,
                help_opened: Cell::new(false),
            },
        );

        let outcome = controller
            .paste_selected(&mut history, item_id, PasteMode::PreserveFormatting)
            .unwrap();

        assert_eq!(
            outcome,
            ClipboardActionOutcome::Pasted {
                item_id,
                mode: PasteMode::PreserveFormatting
            }
        );
        assert_eq!(
            *written_content.borrow(),
            Some(ClipboardContent::Text {
                text: "hello".to_string()
            })
        );
    }

    #[test]
    fn paste_selected_returns_permission_fallback() {
        let (mut history, item_id) = seeded_history();
        let controller = ClipboardActionController::new(
            FakeClipboard::default(),
            FakePaste::default(),
            FakeFocus(true),
            FakePermissions {
                allowed: false,
                help_opened: Cell::new(false),
            },
        );

        let outcome = controller
            .paste_selected(&mut history, item_id, PasteMode::PreserveFormatting)
            .unwrap();

        assert_eq!(
            outcome,
            ClipboardActionOutcome::PasteUnavailable {
                item_id,
                reason: "paste automation permission is not granted".to_string()
            }
        );
        assert_eq!(
            outcome.fallback_guidance().unwrap().body_key,
            "paste.unavailable.permission.body"
        );
    }

    #[test]
    fn paste_selected_detects_missing_focus() {
        let (mut history, item_id) = seeded_history();
        let controller = ClipboardActionController::new(
            FakeClipboard::default(),
            FakePaste::default(),
            FakeFocus(false),
            FakePermissions {
                allowed: true,
                help_opened: Cell::new(false),
            },
        );

        let outcome = controller
            .paste_selected(&mut history, item_id, PasteMode::PreserveFormatting)
            .unwrap();

        assert_eq!(
            outcome,
            ClipboardActionOutcome::PasteUnavailable {
                item_id,
                reason: "target application focus is unavailable".to_string()
            }
        );
    }

    #[test]
    fn poller_captures_only_changed_clipboard_sequences() {
        let mut history = ClipboardHistory::default();
        let clipboard = FakeClipboard::default();
        clipboard.snapshot.replace(Some(ClipboardSnapshot {
            content: ClipboardContent::Text {
                text: "copy".to_string(),
            },
            source_application: Some("Example".to_string()),
            sequence: Some(1),
        }));
        let mut poller = ClipboardPoller::default();

        let first = poller
            .poll(&clipboard, &mut history, TimestampMillis(1))
            .unwrap();
        let second = poller
            .poll(&clipboard, &mut history, TimestampMillis(2))
            .unwrap();

        assert_eq!(first, ClipboardPollOutcome::Captured(ItemId(1)));
        assert_eq!(second, ClipboardPollOutcome::NoChange);
    }

    #[test]
    fn poller_uses_adaptive_interval() {
        let mut history = ClipboardHistory::default();
        let clipboard = FakeClipboard::default();
        let mut poller = ClipboardPoller::with_schedule(ClipboardPollingSchedule::new(100, 400));

        assert_eq!(poller.next_interval_ms(), 100);
        let no_change = poller
            .poll(&clipboard, &mut history, TimestampMillis(1))
            .unwrap();
        assert_eq!(no_change, ClipboardPollOutcome::NoChange);
        assert_eq!(poller.next_interval_ms(), 200);

        clipboard.snapshot.replace(Some(ClipboardSnapshot {
            content: ClipboardContent::Text {
                text: "copy".to_string(),
            },
            source_application: None,
            sequence: Some(1),
        }));
        let captured = poller
            .poll(&clipboard, &mut history, TimestampMillis(2))
            .unwrap();
        assert_eq!(captured, ClipboardPollOutcome::Captured(ItemId(1)));
        assert_eq!(poller.next_interval_ms(), 100);
    }

    #[test]
    fn poller_captures_rapid_clipboard_sequence_batches_in_order() {
        let mut history = ClipboardHistory::default();
        let clipboard = BatchClipboard::default();
        clipboard.snapshots.replace(vec![
            ClipboardSnapshot {
                content: ClipboardContent::Text {
                    text: "first".to_string(),
                },
                source_application: None,
                sequence: Some(1),
            },
            ClipboardSnapshot {
                content: ClipboardContent::Text {
                    text: "second".to_string(),
                },
                source_application: None,
                sequence: Some(2),
            },
        ]);
        let mut poller = ClipboardPoller::default();

        let batch = poller
            .poll_all(&clipboard, &mut history, TimestampMillis(10))
            .unwrap();
        let second_batch = poller
            .poll_all(&clipboard, &mut history, TimestampMillis(11))
            .unwrap();

        assert_eq!(batch.captured_item_ids, vec![ItemId(1), ItemId(2)]);
        assert_eq!(second_batch.captured_item_ids, Vec::<ItemId>::new());
        assert_eq!(second_batch.unchanged_snapshots, 2);
        assert_eq!(history.items()[0].preview_text, "second");
        assert_eq!(history.items()[1].preview_text, "first");
    }

    #[test]
    fn poller_treats_repeated_unsequenced_content_as_unchanged() {
        let mut history = ClipboardHistory::default();
        let clipboard = FakeClipboard::default();
        clipboard.snapshot.replace(Some(ClipboardSnapshot {
            content: ClipboardContent::Text {
                text: "copy".to_string(),
            },
            source_application: None,
            sequence: None,
        }));
        let mut poller = ClipboardPoller::default();

        let first = poller
            .poll(&clipboard, &mut history, TimestampMillis(1))
            .unwrap();
        let second = poller
            .poll(&clipboard, &mut history, TimestampMillis(2))
            .unwrap();

        assert_eq!(first, ClipboardPollOutcome::Captured(ItemId(1)));
        assert_eq!(second, ClipboardPollOutcome::NoChange);
        assert_eq!(history.items()[0].last_used_at, TimestampMillis(1));
    }

    #[test]
    fn single_instance_reports_second_launch_as_existing_instance() {
        let lock_path = temp_lock_path();
        let single_instance = SingleInstance::new(&lock_path);
        let first = single_instance.launch().unwrap();
        let second = single_instance.launch().unwrap();

        assert!(matches!(first, LaunchOutcome::PrimaryInstance(_)));
        assert_eq!(second, LaunchOutcome::FocusExistingInstance);
    }

    #[test]
    fn single_instance_releases_lock_on_drop() {
        let lock_path = temp_lock_path();
        let single_instance = SingleInstance::new(&lock_path);
        let first = single_instance.launch().unwrap();
        drop(first);
        let second = single_instance.launch().unwrap();

        assert!(matches!(second, LaunchOutcome::PrimaryInstance(_)));
    }

    #[test]
    fn single_instance_replaces_corrupt_stale_lock_file() {
        let lock_path = temp_lock_path();
        fs::write(&lock_path, "not-a-pid").unwrap();
        let single_instance = SingleInstance::new(&lock_path);

        let first = single_instance.launch().unwrap();
        let second = single_instance.launch().unwrap();

        assert!(matches!(first, LaunchOutcome::PrimaryInstance(_)));
        assert_eq!(second, LaunchOutcome::FocusExistingInstance);
    }
}
