use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

#[cfg(target_os = "linux")]
use std::collections::HashMap;

#[cfg(target_os = "linux")]
use zbus::{
    blocking::{Connection, Proxy},
    zvariant::{OwnedObjectPath, OwnedValue, Value},
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let autostart = XdgAutostart::from_environment();
    let state_store = LinuxShellStateStore::from_environment();
    if should_open_history_by_default(&args) {
        show_history(&state_store);
        return;
    }
    if args.iter().any(|arg| arg == "--status") {
        print_status(&autostart, &state_store);
        return;
    }
    if handle_shell_command(&args, &autostart, &state_store) {
        return;
    }

    print_status(&autostart, &state_store);
}

fn handle_shell_command(
    args: &[String],
    autostart: &XdgAutostart,
    state_store: &LinuxShellStateStore,
) -> bool {
    handle_autostart_command(args, autostart)
        || handle_history_command(args, state_store)
        || handle_state_command(args, state_store)
        || handle_clipboard_command(args, state_store)
}

fn should_open_history_by_default(args: &[String]) -> bool {
    args.len() == 1
}

fn handle_autostart_command(args: &[String], autostart: &XdgAutostart) -> bool {
    if args.iter().any(|arg| arg == "--enable-autostart") {
        enable_autostart(autostart);
        return true;
    }
    if args.iter().any(|arg| arg == "--disable-autostart") {
        disable_autostart(autostart);
        return true;
    }
    if args.iter().any(|arg| arg == "--install-x11-shortcut") {
        install_x11_shortcut();
        return true;
    }
    false
}

fn handle_history_command(args: &[String], state_store: &LinuxShellStateStore) -> bool {
    if args.iter().any(|arg| arg == "--show-history") {
        show_history(state_store);
        return true;
    }
    if let Some(shortcut) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--copy-shortcut="))
        .and_then(first_char)
    {
        copy_history_shortcut(state_store, shortcut);
        return true;
    }
    if let Some(shortcut) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--paste-shortcut="))
        .and_then(first_char)
    {
        paste_history_shortcut(state_store, shortcut, false);
        return true;
    }
    if let Some(shortcut) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--paste-plain-shortcut="))
        .and_then(first_char)
    {
        paste_history_shortcut(state_store, shortcut, true);
        return true;
    }
    if let Some(shortcut) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--delete-shortcut="))
        .and_then(first_char)
    {
        delete_history_shortcut(state_store, shortcut);
        return true;
    }
    if let Some(shortcut) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--toggle-pin-shortcut="))
        .and_then(first_char)
    {
        toggle_history_pin_shortcut(state_store, shortcut);
        return true;
    }
    if let Some(text) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--delete-text="))
    {
        delete_history_text(state_store, text);
        return true;
    }
    if let Some(text) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--toggle-pin="))
    {
        toggle_history_pin(state_store, text);
        return true;
    }
    if args.iter().any(|arg| arg == "--clear-unpinned") {
        clear_unpinned_history(state_store);
        return true;
    }
    if args.iter().any(|arg| arg == "--clear-all") {
        clear_all_history(state_store);
        return true;
    }
    false
}

fn handle_state_command(args: &[String], state_store: &LinuxShellStateStore) -> bool {
    if args.iter().any(|arg| arg == "--pause-capture") {
        toggle_capture_pause(state_store);
        return true;
    }
    if args.iter().any(|arg| arg == "--ignore-next-copy") {
        set_ignore_next_copy(state_store);
        return true;
    }
    if args.iter().any(|arg| arg == "--preferences") {
        show_preferences(state_store);
        return true;
    }
    if args.iter().any(|arg| arg == "--wayland-shortcuts-status") {
        print_wayland_shortcuts_status();
        return true;
    }
    if args.iter().any(|arg| arg == "--wayland-shortcuts-plan") {
        print_wayland_shortcuts_plan();
        return true;
    }
    if args.iter().any(|arg| arg == "--wayland-shortcuts-daemon") {
        run_wayland_shortcuts_daemon();
        return true;
    }
    false
}

fn handle_clipboard_command(args: &[String], state_store: &LinuxShellStateStore) -> bool {
    if args.iter().any(|arg| arg == "--notify-smoke") {
        run_notification_smoke();
        return true;
    }
    if args.iter().any(|arg| arg == "--clipboard-smoke") {
        run_clipboard_smoke(state_store);
        return true;
    }
    if let Some(text) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--clipboard-write="))
    {
        write_clipboard_text(text);
        return true;
    }
    if let Some(text) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--paste-text="))
    {
        paste_text(text, false);
        return true;
    }
    if let Some(text) = args
        .iter()
        .find_map(|arg| arg.strip_prefix("--paste-plain-text="))
    {
        paste_text(text, true);
        return true;
    }
    false
}

fn enable_autostart(autostart: &XdgAutostart) {
    let executable = env::current_exe().unwrap_or_else(|_| PathBuf::from("clippo-linux"));
    if let Err(error) = autostart.set_enabled(&executable, true) {
        eprintln!("Could not enable Clippo autostart: {error}");
    }
}

fn disable_autostart(autostart: &XdgAutostart) {
    if let Err(error) = autostart.set_enabled(Path::new("clippo-linux"), false) {
        eprintln!("Could not disable Clippo autostart: {error}");
    }
}

fn install_x11_shortcut() {
    let config = X11ShortcutConfig::from_environment();
    let executable = env::current_exe().unwrap_or_else(|_| PathBuf::from("clippo-linux"));
    if let Err(error) = config.install_open_history_shortcut(&executable) {
        eprintln!("Could not install X11 shortcut: {error}");
    }
}

fn show_history(state_store: &LinuxShellStateStore) {
    let history = match state_store.load_ordered_history() {
        Ok(history) => history,
        Err(error) => {
            eprintln!("Could not load Linux history: {error}");
            return;
        }
    };

    if history.is_empty() {
        if let Err(error) = ZenityDialog::info("Clippo", "No clipboard history yet.") {
            eprintln!("Could not show history dialog: {error}");
        }
        return;
    }

    let query = match ZenityDialog::search_query() {
        Ok(Some(query)) => query,
        Ok(None) => return,
        Err(error) => {
            eprintln!("Could not show history search dialog: {error}");
            String::new()
        }
    };
    let visible_history = filter_history_for_search(&history, &query);

    if visible_history.is_empty() {
        if let Err(error) = ZenityDialog::info("Clippo", "No clipboard history matches.") {
            eprintln!("Could not show empty search dialog: {error}");
        }
        return;
    }

    match ZenityDialog::select_history_item(&visible_history) {
        Ok(Some(item)) => match ZenityDialog::select_history_action(item.pinned) {
            Ok(Some(HistoryAction::Copy)) => write_clipboard_text(&item.text),
            Ok(Some(HistoryAction::Paste)) => paste_text(&item.text, false),
            Ok(Some(HistoryAction::PastePlain)) => paste_text(&item.text, true),
            Ok(Some(HistoryAction::ShowFullText)) => {
                if let Err(error) = ZenityDialog::info("Clippo Item", &item.text) {
                    eprintln!("Could not show full clipboard text: {error}");
                }
            }
            Ok(Some(HistoryAction::TogglePin)) => toggle_history_pin(state_store, &item.text),
            Ok(Some(HistoryAction::Delete)) => delete_history_text(state_store, &item.text),
            Ok(None) => {}
            Err(error) => {
                eprintln!("Could not show history action dialog: {error}");
                write_clipboard_text(&item.text);
            }
        },
        Ok(None) => {}
        Err(error) => eprintln!("Could not show history dialog: {error}"),
    }
}

fn copy_history_shortcut(state_store: &LinuxShellStateStore, shortcut: char) {
    match state_store.item_by_visible_shortcut(shortcut) {
        Ok(Some(item)) => write_clipboard_text(&item.text),
        Ok(None) => println!("No history item for shortcut {shortcut}."),
        Err(error) => eprintln!("Could not load Linux history: {error}"),
    }
}

fn paste_history_shortcut(state_store: &LinuxShellStateStore, shortcut: char, plain_text: bool) {
    match state_store.item_by_visible_shortcut(shortcut) {
        Ok(Some(item)) => paste_text(&item.text, plain_text),
        Ok(None) => println!("No history item for shortcut {shortcut}."),
        Err(error) => eprintln!("Could not load Linux history: {error}"),
    }
}

fn delete_history_shortcut(state_store: &LinuxShellStateStore, shortcut: char) {
    match state_store.item_by_visible_shortcut(shortcut) {
        Ok(Some(item)) => delete_history_text(state_store, &item.text),
        Ok(None) => println!("No history item for shortcut {shortcut}."),
        Err(error) => eprintln!("Could not load Linux history: {error}"),
    }
}

fn toggle_history_pin_shortcut(state_store: &LinuxShellStateStore, shortcut: char) {
    match state_store.item_by_visible_shortcut(shortcut) {
        Ok(Some(item)) => toggle_history_pin(state_store, &item.text),
        Ok(None) => println!("No history item for shortcut {shortcut}."),
        Err(error) => eprintln!("Could not load Linux history: {error}"),
    }
}

fn show_preferences(state_store: &LinuxShellStateStore) {
    match state_store.load() {
        Ok(state) => {
            let body = format!(
                "Capture paused: {}\nIgnore next copy: {}\nState file: {}",
                state.capture_paused,
                state.ignore_next_copy,
                state_store.state_path().display()
            );
            if let Err(error) = ZenityDialog::info("Clippo Preferences", &body) {
                eprintln!("Could not show preferences dialog: {error}");
            }
        }
        Err(error) => eprintln!("Could not load Linux preferences: {error}"),
    }
}

fn delete_history_text(state_store: &LinuxShellStateStore, text: &str) {
    match state_store.delete_history_text(text) {
        Ok(true) => println!("Deleted history item."),
        Ok(false) => println!("History item not found."),
        Err(error) => eprintln!("Could not delete history item: {error}"),
    }
}

fn toggle_history_pin(state_store: &LinuxShellStateStore, text: &str) {
    match state_store.toggle_history_pin(text) {
        Ok(Some(true)) => println!("Pinned history item."),
        Ok(Some(false)) => println!("Unpinned history item."),
        Ok(None) => println!("History item not found."),
        Err(error) => eprintln!("Could not update pin state: {error}"),
    }
}

fn clear_unpinned_history(state_store: &LinuxShellStateStore) {
    match state_store.clear_unpinned_history() {
        Ok(count) => println!("Cleared {count} unpinned history item(s)."),
        Err(error) => eprintln!("Could not clear unpinned history: {error}"),
    }
}

fn clear_all_history(state_store: &LinuxShellStateStore) {
    match state_store.clear_all_history() {
        Ok(count) => println!("Cleared {count} history item(s)."),
        Err(error) => eprintln!("Could not clear history: {error}"),
    }
}

fn toggle_capture_pause(state_store: &LinuxShellStateStore) {
    match state_store.toggle_capture_paused() {
        Ok(state) if state.capture_paused => println!("Clipboard capture paused."),
        Ok(_) => println!("Clipboard capture resumed."),
        Err(error) => eprintln!("Could not update capture pause state: {error}"),
    }
}

fn set_ignore_next_copy(state_store: &LinuxShellStateStore) {
    match state_store.set_ignore_next_copy() {
        Ok(()) => println!("Next clipboard change will be ignored."),
        Err(error) => eprintln!("Could not update ignore-next-copy state: {error}"),
    }
}

fn run_notification_smoke() {
    let _ = notify(
        "Notification test",
        "Clippo desktop notifications are available.",
    );
}

fn run_clipboard_smoke(state_store: &LinuxShellStateStore) {
    if should_skip_clipboard_capture(state_store) {
        return;
    }

    let session = SessionKind::detect();
    let mut monitor = ClipboardMonitor::default();
    match LinuxClipboardBackend::for_session(session) {
        Some(backend) => match monitor.poll_changed(|| backend.read_text()) {
            Ok(Some(text)) => {
                if let Err(error) = state_store.add_history_text(&text) {
                    eprintln!("Could not save Linux history item: {error}");
                }
                println!("{} clipboard text: {}", backend.name(), text);
            }
            Ok(None) => eprintln!("No supported Linux clipboard backend returned text."),
            Err(error) => eprintln!("Could not read clipboard text: {error}"),
        },
        None => {
            eprintln!("No supported Linux clipboard backend for this session.");
        }
    }
}

fn write_clipboard_text(text: &str) {
    let session = SessionKind::detect();
    match LinuxClipboardBackend::for_session(session) {
        Some(backend) => match backend.write_text(text) {
            Ok(true) => println!("Wrote clipboard text through {}.", backend.name()),
            Ok(false) => eprintln!(
                "Clipboard write tool for {} is unavailable.",
                backend.name()
            ),
            Err(error) => eprintln!("Could not write clipboard text: {error}"),
        },
        None => eprintln!("No supported Linux clipboard backend for this session."),
    }
}

fn paste_text(text: &str, plain_text: bool) {
    let session = SessionKind::detect();
    match paste_text_for_session(session, text, plain_text) {
        Ok(PasteOutcome::Pasted) if plain_text => {
            println!("Pasted plain text through Linux input automation.");
        }
        Ok(PasteOutcome::Pasted) => println!("Pasted text through Linux input automation."),
        Ok(PasteOutcome::CopiedForManualPaste) if plain_text => {
            println!("Copied plain text. Paste manually because this session blocks automation.");
        }
        Ok(PasteOutcome::CopiedForManualPaste) => {
            println!("Copied text. Paste manually because this session blocks automation.");
        }
        Ok(PasteOutcome::ClipboardToolUnavailable) => {
            eprintln!("Clipboard tool unavailable for this Linux session.");
        }
        Ok(PasteOutcome::PasteToolUnavailable) => {
            eprintln!("Paste automation tool unavailable for this Linux session.");
        }
        Err(error) => eprintln!("Could not paste text: {error}"),
    }
}

fn print_wayland_shortcuts_status() {
    match WaylandGlobalShortcutsPortal::probe() {
        Ok(PortalAvailability::Available { version }) => {
            println!(
                "Wayland GlobalShortcuts portal is available{}.",
                version.map_or_else(String::new, |value| format!(" (version {value})"))
            );
            println!(
                "Run `clippo-linux --wayland-shortcuts-daemon` to bind `Super+Shift+C` through the portal when the compositor allows it."
            );
        }
        Ok(PortalAvailability::Unavailable) => {
            println!(
                "Wayland GlobalShortcuts portal was not found. Configure a desktop shortcut that runs `clippo-linux --show-history`."
            );
        }
        Err(error) => {
            eprintln!("Could not inspect Wayland GlobalShortcuts portal: {error}");
            println!("Configure a desktop shortcut that runs `clippo-linux --show-history`.");
        }
    }
}

fn print_wayland_shortcuts_plan() {
    let create_session = WaylandGlobalShortcutsPortal::create_session_command(
        "clippo_create_shortcuts",
        "clippo_shortcuts",
    );
    let bind_shortcuts = WaylandGlobalShortcutsPortal::bind_shortcuts_command(
        "/org/freedesktop/portal/desktop/session/clippo_shortcuts",
        "clippo_bind_shortcuts",
        &WaylandGlobalShortcutsPortal::default_shortcuts(),
    );
    let list_shortcuts = WaylandGlobalShortcutsPortal::list_shortcuts_command(
        "/org/freedesktop/portal/desktop/session/clippo_shortcuts",
        "clippo_list_shortcuts",
    );
    let monitor = WaylandGlobalShortcutsPortal::monitor_command();

    println!("Wayland GlobalShortcuts portal command plan:");
    println!("1. {}", create_session.shell_line());
    println!("2. Wait for org.freedesktop.portal.Request::Response and read session_handle.");
    println!("3. {}", bind_shortcuts.shell_line());
    println!("4. {}", list_shortcuts.shell_line());
    println!("5. {}", monitor.shell_line());
    for shortcut in WaylandGlobalShortcutsPortal::default_shortcuts() {
        println!(
            "When the Activated signal emits shortcut id `{}`, run `{}`.",
            shortcut.id, shortcut.command
        );
    }
}

#[cfg(target_os = "linux")]
fn run_wayland_shortcuts_daemon() {
    let executable_path = env::current_exe().unwrap_or_else(|_| PathBuf::from("clippo-linux"));
    let daemon = WaylandShortcutDaemon { executable_path };
    match daemon.run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("Could not run Wayland shortcut daemon: {error}");
            println!(
                "Fallback: configure a desktop shortcut that runs `clippo-linux --show-history`."
            );
        }
    }
}

#[cfg(not(target_os = "linux"))]
fn run_wayland_shortcuts_daemon() {
    eprintln!("Wayland shortcut daemon support is only available on Linux.");
}

fn print_status(autostart: &XdgAutostart, state_store: &LinuxShellStateStore) {
    let session = SessionKind::detect();
    let state = state_store.load().unwrap_or_default();
    let desktop = DesktopEnvironment::detect();
    println!("Clippo Linux shell scaffold");
    println!("Autostart enabled: {}", autostart.is_enabled());
    println!("Capture paused: {}", state.capture_paused);
    println!("Ignore next copy: {}", state.ignore_next_copy);
    println!("Desktop shell: {}", desktop.name());
    println!("Tray strategy: {}", desktop.tray_strategy());
    println!("{}", session.shortcut_fallback_message());
    for message in session.clipboard_fallback_messages() {
        println!("{message}");
    }
    match LinuxClipboardBackend::for_session(session) {
        Some(backend) => println!("Clipboard backend: {}", backend.name()),
        None => println!("Clipboard backend: unavailable for unknown session"),
    }
    match LinuxPasteBackend::for_session(session) {
        Some(backend) => println!("Paste backend: {}", backend.name()),
        None => println!("Paste backend: manual paste fallback"),
    }
    if session == SessionKind::Wayland {
        match WaylandGlobalShortcutsPortal::probe() {
            Ok(PortalAvailability::Available { version }) => println!(
                "Wayland shortcut portal: available{}",
                version.map_or_else(String::new, |value| format!(" (version {value})"))
            ),
            Ok(PortalAvailability::Unavailable) => {
                println!("Wayland shortcut portal: unavailable");
            }
            Err(error) => println!("Wayland shortcut portal: unknown ({error})"),
        }
    }
    let placement = PopupPlacement::near_anchor(
        Rect::new(0, 0, 1920, 1080),
        Point::new(1910, 24),
        Size::new(420, 520),
        ScaleFactor::new(1.0),
    );
    println!(
        "Default popup placement: {},{} {}x{}",
        placement.x, placement.y, placement.width, placement.height
    );
}

fn first_char(value: &str) -> Option<char> {
    value.chars().next()
}

fn should_skip_clipboard_capture(state_store: &LinuxShellStateStore) -> bool {
    match state_store.load() {
        Ok(state) if state.capture_paused => {
            println!("Clipboard capture is paused.");
            true
        }
        Ok(state) if state.ignore_next_copy => {
            if let Err(error) = state_store.save(LinuxShellState {
                ignore_next_copy: false,
                ..state
            }) {
                eprintln!("Could not consume ignore-next-copy state: {error}");
            } else {
                println!("Ignored this clipboard change.");
            }
            true
        }
        Ok(_) => false,
        Err(error) => {
            eprintln!("Could not read Linux shell state: {error}");
            false
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SessionKind {
    X11,
    Wayland,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DesktopEnvironment {
    Gnome,
    Kde,
    Xfce,
    Unknown,
}

impl DesktopEnvironment {
    fn detect() -> Self {
        Self::from_env_values(
            env::var("XDG_CURRENT_DESKTOP").ok().as_deref(),
            env::var("DESKTOP_SESSION").ok().as_deref(),
        )
    }

    fn from_env_values(current_desktop: Option<&str>, desktop_session: Option<&str>) -> Self {
        let value = format!(
            "{} {}",
            current_desktop.unwrap_or_default(),
            desktop_session.unwrap_or_default()
        )
        .to_lowercase();

        if value.contains("gnome") {
            Self::Gnome
        } else if value.contains("kde") || value.contains("plasma") {
            Self::Kde
        } else if value.contains("xfce") {
            Self::Xfce
        } else {
            Self::Unknown
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::Gnome => "GNOME",
            Self::Kde => "KDE Plasma",
            Self::Xfce => "XFCE",
            Self::Unknown => "unknown desktop",
        }
    }

    fn tray_strategy(self) -> &'static str {
        match self {
            Self::Gnome => {
                "Use desktop actions by default; status notifier may require an extension."
            }
            Self::Kde => "Use StatusNotifierItem when the native tray shell is implemented.",
            Self::Xfce => "Use StatusNotifierItem or legacy tray support when available.",
            Self::Unknown => "Expose desktop actions and document tray support as best effort.",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinuxClipboardBackend {
    X11,
    Wayland,
}

impl LinuxClipboardBackend {
    fn for_session(session: SessionKind) -> Option<Self> {
        match session {
            SessionKind::X11 => Some(Self::X11),
            SessionKind::Wayland => Some(Self::Wayland),
            SessionKind::Unknown => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::X11 => "xclip",
            Self::Wayland => "wl-clipboard",
        }
    }

    fn read_command(self) -> ClipboardCommand {
        match self {
            Self::X11 => ClipboardCommand::new(
                "xclip",
                &[
                    "-selection",
                    "clipboard",
                    "-out",
                    "-target",
                    "text/plain;charset=utf-8",
                ],
            ),
            Self::Wayland => {
                ClipboardCommand::new("wl-paste", &["--no-newline", "--type", "text/plain"])
            }
        }
    }

    fn write_command(self) -> ClipboardCommand {
        match self {
            Self::X11 => ClipboardCommand::new(
                "xclip",
                &[
                    "-selection",
                    "clipboard",
                    "-in",
                    "-target",
                    "text/plain;charset=utf-8",
                ],
            ),
            Self::Wayland => ClipboardCommand::new("wl-copy", &["--type", "text/plain"]),
        }
    }

    fn read_text(self) -> io::Result<Option<String>> {
        let command = self.read_command();
        let output = Command::new(command.program).args(command.args).output();
        match output {
            Ok(output) if output.status.success() => {
                let text = String::from_utf8_lossy(&output.stdout).to_string();
                if text.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(text))
                }
            }
            Ok(_) => Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error),
        }
    }

    fn write_text(self, text: &str) -> io::Result<bool> {
        let command = self.write_command();
        let mut child = match Command::new(command.program)
            .args(command.args)
            .stdin(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(false),
            Err(error) => return Err(error),
        };

        if let Some(mut stdin) = child.stdin.take() {
            use std::io::Write;
            stdin.write_all(text.as_bytes())?;
        }

        Ok(child.wait()?.success())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LinuxPasteBackend {
    X11,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PortalAvailability {
    Available { version: Option<u32> },
    Unavailable,
}

struct WaylandGlobalShortcutsPortal;

impl WaylandGlobalShortcutsPortal {
    const INTERFACE: &'static str = "org.freedesktop.portal.GlobalShortcuts";
    const DESTINATION: &'static str = "org.freedesktop.portal.Desktop";
    const OBJECT_PATH: &'static str = "/org/freedesktop/portal/desktop";

    fn probe_command() -> PortalCommand {
        PortalCommand::new(
            "gdbus",
            &[
                "introspect",
                "--session",
                "--dest",
                Self::DESTINATION,
                "--object-path",
                Self::OBJECT_PATH,
            ],
        )
    }

    fn create_session_command(handle_token: &str, session_handle_token: &str) -> PortalCommand {
        PortalCommand::new(
            "gdbus",
            &[
                "call",
                "--session",
                "--dest",
                Self::DESTINATION,
                "--object-path",
                Self::OBJECT_PATH,
                "--method",
                "org.freedesktop.portal.GlobalShortcuts.CreateSession",
                &format!(
                    "{{'handle_token': <'{}'>, 'session_handle_token': <'{}'>}}",
                    escape_gvariant_string(handle_token),
                    escape_gvariant_string(session_handle_token)
                ),
            ],
        )
    }

    fn bind_shortcuts_command(
        session_handle: &str,
        handle_token: &str,
        shortcuts: &[WaylandShortcutBinding],
    ) -> PortalCommand {
        PortalCommand::new(
            "gdbus",
            &[
                "call",
                "--session",
                "--dest",
                Self::DESTINATION,
                "--object-path",
                Self::OBJECT_PATH,
                "--method",
                "org.freedesktop.portal.GlobalShortcuts.BindShortcuts",
                session_handle,
                &format_shortcuts_gvariant(shortcuts),
                "",
                &format!(
                    "{{'handle_token': <'{}'>}}",
                    escape_gvariant_string(handle_token)
                ),
            ],
        )
    }

    fn list_shortcuts_command(session_handle: &str, handle_token: &str) -> PortalCommand {
        PortalCommand::new(
            "gdbus",
            &[
                "call",
                "--session",
                "--dest",
                Self::DESTINATION,
                "--object-path",
                Self::OBJECT_PATH,
                "--method",
                "org.freedesktop.portal.GlobalShortcuts.ListShortcuts",
                session_handle,
                &format!(
                    "{{'handle_token': <'{}'>}}",
                    escape_gvariant_string(handle_token)
                ),
            ],
        )
    }

    fn monitor_command() -> PortalCommand {
        PortalCommand::new(
            "gdbus",
            &[
                "monitor",
                "--session",
                "--dest",
                Self::DESTINATION,
                "--object-path",
                Self::OBJECT_PATH,
            ],
        )
    }

    fn default_shortcuts() -> [WaylandShortcutBinding; 1] {
        [WaylandShortcutBinding {
            id: "open-history",
            description: "Open Clippo history",
            preferred_trigger: "Super+Shift+C",
            command: "clippo-linux --show-history",
        }]
    }

    fn probe() -> io::Result<PortalAvailability> {
        let command = Self::probe_command();
        match Command::new(command.program).args(&command.args).output() {
            Ok(output) if output.status.success() => {
                let text = String::from_utf8_lossy(&output.stdout);
                Ok(Self::parse_introspection(&text))
            }
            Ok(_) => Ok(PortalAvailability::Unavailable),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                Ok(PortalAvailability::Unavailable)
            }
            Err(error) => Err(error),
        }
    }

    fn parse_introspection(text: &str) -> PortalAvailability {
        if !text.contains(Self::INTERFACE) {
            return PortalAvailability::Unavailable;
        }

        PortalAvailability::Available {
            version: parse_global_shortcuts_version(text),
        }
    }
}

#[cfg(target_os = "linux")]
struct WaylandShortcutDaemon {
    executable_path: PathBuf,
}

#[cfg(target_os = "linux")]
impl WaylandShortcutDaemon {
    const OPEN_HISTORY_ID: &'static str = "open-history";

    fn run(&self) -> io::Result<()> {
        match WaylandGlobalShortcutsPortal::probe()? {
            PortalAvailability::Available { version } => {
                println!(
                    "Wayland GlobalShortcuts portal is available{}.",
                    version.map_or_else(String::new, |value| format!(" (version {value})"))
                );
            }
            PortalAvailability::Unavailable => {
                return Err(io::Error::new(
                    io::ErrorKind::Unsupported,
                    "Wayland GlobalShortcuts portal is unavailable",
                ));
            }
        }

        let connection = Connection::session().map_err(dbus_error)?;
        let portal = Self::portal_proxy(&connection)?;
        let session_handle = Self::create_session(&connection, &portal)?;
        Self::bind_shortcuts(&connection, &portal, &session_handle)?;

        println!("Wayland shortcut daemon is active. Press Super+Shift+C to open Clippo history.");

        let mut activations = portal
            .receive_signal_with_args("Activated", &[(1, Self::OPEN_HISTORY_ID)])
            .map_err(dbus_error)?;
        for activation in &mut activations {
            let body = activation.body();
            let (signal_session_handle, shortcut_id, _timestamp, _options): (
                OwnedObjectPath,
                String,
                u64,
                HashMap<String, OwnedValue>,
            ) = body.deserialize().map_err(dbus_error)?;

            if Self::activation_matches(&session_handle, &signal_session_handle, &shortcut_id) {
                self.open_history()?;
            }
        }

        Ok(())
    }

    fn portal_proxy(connection: &Connection) -> io::Result<Proxy<'_>> {
        Proxy::new(
            connection,
            WaylandGlobalShortcutsPortal::DESTINATION,
            WaylandGlobalShortcutsPortal::OBJECT_PATH,
            WaylandGlobalShortcutsPortal::INTERFACE,
        )
        .map_err(dbus_error)
    }

    fn create_session(connection: &Connection, portal: &Proxy<'_>) -> io::Result<OwnedObjectPath> {
        let handle_token = unique_portal_token("clippo_create_shortcuts");
        let session_handle_token = unique_portal_token("clippo_shortcuts");
        let mut options = HashMap::new();
        options.insert("handle_token", Value::from(handle_token.as_str()));
        options.insert(
            "session_handle_token",
            Value::from(session_handle_token.as_str()),
        );

        let request_handle: OwnedObjectPath = portal
            .call("CreateSession", &(options))
            .map_err(dbus_error)?;
        let mut results = wait_for_request_response(connection, &request_handle)?;
        let session_value = results.remove("session_handle").ok_or_else(|| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "portal response did not include session_handle",
            )
        })?;

        owned_value_to_object_path(session_value)
    }

    fn bind_shortcuts(
        connection: &Connection,
        portal: &Proxy<'_>,
        session_handle: &OwnedObjectPath,
    ) -> io::Result<()> {
        let handle_token = unique_portal_token("clippo_bind_shortcuts");
        let shortcuts = WaylandGlobalShortcutsPortal::default_shortcuts()
            .into_iter()
            .map(|shortcut| {
                let mut properties = HashMap::new();
                properties.insert("description", Value::from(shortcut.description));
                properties.insert("preferred_trigger", Value::from(shortcut.preferred_trigger));
                (shortcut.id, properties)
            })
            .collect::<Vec<_>>();
        let mut options = HashMap::new();
        options.insert("handle_token", Value::from(handle_token.as_str()));

        let request_handle: OwnedObjectPath = portal
            .call("BindShortcuts", &(session_handle, shortcuts, "", options))
            .map_err(dbus_error)?;
        let _results = wait_for_request_response(connection, &request_handle)?;
        Ok(())
    }

    fn activation_matches(
        expected_session_handle: &OwnedObjectPath,
        signal_session_handle: &OwnedObjectPath,
        shortcut_id: &str,
    ) -> bool {
        expected_session_handle == signal_session_handle && shortcut_id == Self::OPEN_HISTORY_ID
    }

    fn open_history(&self) -> io::Result<()> {
        Command::new(&self.executable_path)
            .arg("--show-history")
            .status()
            .map(|_| ())
    }
}

#[cfg(target_os = "linux")]
fn wait_for_request_response(
    connection: &Connection,
    request_handle: &OwnedObjectPath,
) -> io::Result<HashMap<String, OwnedValue>> {
    let request_proxy = Proxy::new(
        connection,
        WaylandGlobalShortcutsPortal::DESTINATION,
        request_handle.as_str(),
        "org.freedesktop.portal.Request",
    )
    .map_err(dbus_error)?;
    let mut responses = request_proxy
        .receive_signal("Response")
        .map_err(dbus_error)?;
    let response = responses
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::UnexpectedEof, "portal response ended"))?;
    let body = response.body();
    let (response_code, results): (u32, HashMap<String, OwnedValue>) =
        body.deserialize().map_err(dbus_error)?;

    match response_code {
        0 => Ok(results),
        1 => Err(io::Error::new(
            io::ErrorKind::Interrupted,
            "portal request was cancelled",
        )),
        _ => Err(io::Error::other(format!(
            "portal request failed with response code {response_code}"
        ))),
    }
}

#[cfg(target_os = "linux")]
fn owned_value_to_object_path(value: OwnedValue) -> io::Result<OwnedObjectPath> {
    let cloned = value.try_clone().map_err(dbus_error)?;
    if let Ok(path) = OwnedObjectPath::try_from(cloned) {
        Ok(path)
    } else {
        let session_handle = String::try_from(value).map_err(dbus_error)?;
        OwnedObjectPath::try_from(session_handle).map_err(dbus_error)
    }
}

#[cfg(target_os = "linux")]
fn unique_portal_token(prefix: &str) -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos());
    format!("{prefix}_{}_{}", std::process::id(), nanos)
}

#[cfg(target_os = "linux")]
fn dbus_error(error: impl std::fmt::Display) -> io::Error {
    io::Error::other(format!("Wayland GlobalShortcuts D-Bus error: {error}"))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct WaylandShortcutBinding {
    id: &'static str,
    description: &'static str,
    preferred_trigger: &'static str,
    command: &'static str,
}

impl LinuxPasteBackend {
    fn for_session(session: SessionKind) -> Option<Self> {
        match session {
            SessionKind::X11 => Some(Self::X11),
            SessionKind::Wayland | SessionKind::Unknown => None,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Self::X11 => "xdotool",
        }
    }

    fn paste_command(self, plain_text: bool) -> PasteCommand {
        match self {
            Self::X11 if plain_text => {
                PasteCommand::new("xdotool", &["key", "--clearmodifiers", "ctrl+shift+v"])
            }
            Self::X11 => PasteCommand::new("xdotool", &["key", "--clearmodifiers", "ctrl+v"]),
        }
    }

    fn paste(self, plain_text: bool) -> io::Result<bool> {
        let command = self.paste_command(plain_text);
        match Command::new(command.program).args(command.args).status() {
            Ok(status) => Ok(status.success()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PasteCommand {
    program: &'static str,
    args: Vec<&'static str>,
}

impl PasteCommand {
    fn new(program: &'static str, args: &[&'static str]) -> Self {
        Self {
            program,
            args: args.to_vec(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PortalCommand {
    program: &'static str,
    args: Vec<String>,
}

impl PortalCommand {
    fn new(program: &'static str, args: &[&str]) -> Self {
        Self {
            program,
            args: args.iter().map(|arg| (*arg).to_string()).collect(),
        }
    }

    fn shell_line(&self) -> String {
        std::iter::once(self.program.to_string())
            .chain(self.args.iter().map(|arg| shell_quote(arg)))
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn format_shortcuts_gvariant(shortcuts: &[WaylandShortcutBinding]) -> String {
    let shortcuts = shortcuts
        .iter()
        .map(|shortcut| {
            format!(
                "('{}', {{'description': <'{}'>, 'preferred_trigger': <'{}'>}})",
                escape_gvariant_string(shortcut.id),
                escape_gvariant_string(shortcut.description),
                escape_gvariant_string(shortcut.preferred_trigger)
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    format!("[{shortcuts}]")
}

fn escape_gvariant_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\'', "\\'")
}

fn shell_quote(value: &str) -> String {
    if value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || "-_./:=+".contains(character))
    {
        value.to_string()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

fn parse_global_shortcuts_version(text: &str) -> Option<u32> {
    let mut in_interface = false;
    for line in text.lines() {
        if line.contains("interface org.freedesktop.portal.GlobalShortcuts") {
            in_interface = true;
        } else if in_interface && line.trim_start().starts_with("interface ") {
            in_interface = false;
        }

        let trimmed = line.trim();
        if in_interface
            && (trimmed.contains("property uint32 version")
                || trimmed.contains("readonly u version"))
        {
            return line
                .split('=')
                .nth(1)
                .and_then(|value| value.trim().trim_end_matches(';').parse().ok());
        }
    }
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PasteOutcome {
    Pasted,
    CopiedForManualPaste,
    ClipboardToolUnavailable,
    PasteToolUnavailable,
}

fn paste_text_for_session(
    session: SessionKind,
    text: &str,
    plain_text: bool,
) -> io::Result<PasteOutcome> {
    let Some(clipboard_backend) = LinuxClipboardBackend::for_session(session) else {
        return Ok(PasteOutcome::ClipboardToolUnavailable);
    };

    if !clipboard_backend.write_text(text)? {
        return Ok(PasteOutcome::ClipboardToolUnavailable);
    }

    let Some(paste_backend) = LinuxPasteBackend::for_session(session) else {
        return Ok(PasteOutcome::CopiedForManualPaste);
    };

    if paste_backend.paste(plain_text)? {
        Ok(PasteOutcome::Pasted)
    } else {
        Ok(PasteOutcome::PasteToolUnavailable)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClipboardCommand {
    program: &'static str,
    args: Vec<&'static str>,
}

impl ClipboardCommand {
    fn new(program: &'static str, args: &[&'static str]) -> Self {
        Self {
            program,
            args: args.to_vec(),
        }
    }
}

#[derive(Debug, Default)]
struct ClipboardMonitor {
    last_text: Option<String>,
}

impl ClipboardMonitor {
    fn poll_changed<F>(&mut self, mut read_text: F) -> io::Result<Option<String>>
    where
        F: FnMut() -> io::Result<Option<String>>,
    {
        let current_text = read_text()?;
        if current_text.is_some() && current_text != self.last_text {
            self.last_text.clone_from(&current_text);
            Ok(current_text)
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Size {
    width: i32,
    height: i32,
}

impl Size {
    const fn new(width: i32, height: i32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl Rect {
    const fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    const fn right(self) -> i32 {
        self.x + self.width
    }

    const fn bottom(self) -> i32 {
        self.y + self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ScaleFactor(f64);

impl ScaleFactor {
    fn new(value: f64) -> Self {
        Self(value.max(1.0))
    }

    fn align(self, logical_pixels: i32) -> i32 {
        let physical = (f64::from(logical_pixels) * self.0).round();
        rounded_f64_to_i32(physical / self.0).unwrap_or(logical_pixels)
    }
}

fn rounded_f64_to_i32(value: f64) -> Option<i32> {
    if !value.is_finite() {
        return None;
    }
    format!("{:.0}", value.round()).parse().ok()
}

struct PopupPlacement;

impl PopupPlacement {
    fn near_anchor(
        work_area: Rect,
        anchor: Point,
        preferred_size: Size,
        scale: ScaleFactor,
    ) -> Rect {
        let width = scale.align(preferred_size.width).min(work_area.width);
        let height = scale.align(preferred_size.height).min(work_area.height);
        let x = anchor.x.clamp(work_area.x, work_area.right() - width);
        let y = anchor.y.clamp(work_area.y, work_area.bottom() - height);
        Rect::new(x, y, width, height)
    }
}

impl SessionKind {
    fn detect() -> Self {
        match env::var("XDG_SESSION_TYPE")
            .unwrap_or_default()
            .to_lowercase()
            .as_str()
        {
            "x11" => Self::X11,
            "wayland" => Self::Wayland,
            _ => Self::Unknown,
        }
    }

    fn shortcut_fallback_message(self) -> &'static str {
        match self {
            Self::X11 => "Global shortcuts use the X11 backend when available.",
            Self::Wayland => {
                "Wayland global shortcuts depend on compositor or portal support. If blocked, configure a desktop shortcut that runs `clippo-linux --show-history`."
            }
            Self::Unknown => {
                "Global shortcut support depends on the active desktop session. If blocked, configure a desktop shortcut that runs `clippo-linux --show-history`."
            }
        }
    }

    fn clipboard_fallback_messages(self) -> &'static [&'static str] {
        match self {
            Self::X11 => &[
                "Clipboard monitoring uses the X11 backend when available.",
                "Automatic paste uses the X11 input backend when available.",
            ],
            Self::Wayland => &[
                "Wayland clipboard monitoring depends on compositor or portal support. If blocked, Clippo should keep manual copy/paste available and show a limitation message.",
                "Wayland paste automation depends on compositor policy. If blocked, Clippo should copy the selected item and ask the user to paste manually.",
            ],
            Self::Unknown => &[
                "Clipboard monitoring depends on the active desktop session.",
                "Paste automation depends on the active desktop session and may fall back to manual paste.",
            ],
        }
    }
}

struct XdgAutostart {
    config_home: PathBuf,
}

impl XdgAutostart {
    fn from_environment() -> Self {
        let config_home = env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .unwrap_or_else(|| PathBuf::from(".config"));
        Self { config_home }
    }

    fn set_enabled(&self, executable_path: &Path, enabled: bool) -> io::Result<()> {
        let path = self.desktop_file_path();
        if enabled {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(path, desktop_entry(executable_path))
        } else if path.exists() {
            fs::remove_file(path)
        } else {
            Ok(())
        }
    }

    fn is_enabled(&self) -> bool {
        self.desktop_file_path().exists()
    }

    fn desktop_file_path(&self) -> PathBuf {
        self.config_home.join("autostart").join("clippo.desktop")
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct LinuxShellState {
    capture_paused: bool,
    ignore_next_copy: bool,
}

struct LinuxShellStateStore {
    state_home: PathBuf,
}

impl LinuxShellStateStore {
    fn from_environment() -> Self {
        let state_home = env::var_os("XDG_STATE_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/state")))
            .unwrap_or_else(|| PathBuf::from(".local/state"));
        Self { state_home }
    }

    fn load(&self) -> io::Result<LinuxShellState> {
        let path = self.state_path();
        if !path.exists() {
            return Ok(LinuxShellState::default());
        }
        parse_state_file(&fs::read_to_string(path)?)
    }

    fn save(&self, state: LinuxShellState) -> io::Result<()> {
        let path = self.state_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serialize_state_file(state))
    }

    fn toggle_capture_paused(&self) -> io::Result<LinuxShellState> {
        let mut state = self.load()?;
        state.capture_paused = !state.capture_paused;
        self.save(state)?;
        Ok(state)
    }

    fn set_ignore_next_copy(&self) -> io::Result<()> {
        let mut state = self.load()?;
        state.ignore_next_copy = true;
        self.save(state)
    }

    fn load_history(&self) -> io::Result<Vec<LinuxHistoryItem>> {
        let path = self.history_path();
        if !path.exists() {
            return Ok(Vec::new());
        }
        parse_history_file(&fs::read_to_string(path)?)
    }

    fn load_ordered_history(&self) -> io::Result<Vec<LinuxHistoryItem>> {
        let mut history = self.load_history()?;
        sort_history_for_display(&mut history);
        Ok(history)
    }

    fn item_by_visible_shortcut(&self, shortcut: char) -> io::Result<Option<LinuxHistoryItem>> {
        Ok(item_by_visible_shortcut(&self.load_ordered_history()?, shortcut).cloned())
    }

    fn save_history(&self, history: &[LinuxHistoryItem]) -> io::Result<()> {
        let path = self.history_path();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, serialize_history_file(history))
    }

    fn add_history_text(&self, text: &str) -> io::Result<()> {
        let mut history = self.load_history()?;
        history.retain(|item| item.text != text);
        history.insert(0, LinuxHistoryItem::new(text));
        sort_history_for_display(&mut history);
        history.truncate(50);
        self.save_history(&history)
    }

    fn delete_history_text(&self, text: &str) -> io::Result<bool> {
        let mut history = self.load_history()?;
        let before = history.len();
        history.retain(|item| item.text != text);
        self.save_history(&history)?;
        Ok(history.len() != before)
    }

    fn toggle_history_pin(&self, text: &str) -> io::Result<Option<bool>> {
        let mut history = self.load_history()?;
        let next_shortcut = next_pinned_shortcut(&history);
        let Some(item) = history.iter_mut().find(|item| item.text == text) else {
            return Ok(None);
        };
        if item.pinned {
            item.pinned = false;
            item.pinned_shortcut = None;
        } else {
            item.pinned = true;
            item.pinned_shortcut = next_shortcut;
        }
        let pinned = item.pinned;
        sort_history_for_display(&mut history);
        self.save_history(&history)?;
        Ok(Some(pinned))
    }

    fn clear_unpinned_history(&self) -> io::Result<usize> {
        let mut history = self.load_history()?;
        let before = history.len();
        history.retain(|item| item.pinned);
        let cleared = before - history.len();
        self.save_history(&history)?;
        Ok(cleared)
    }

    fn clear_all_history(&self) -> io::Result<usize> {
        let history = self.load_history()?;
        let cleared = history.len();
        self.save_history(&[])?;
        Ok(cleared)
    }

    fn state_path(&self) -> PathBuf {
        self.state_home.join("clippo").join("linux-state")
    }

    fn history_path(&self) -> PathBuf {
        self.state_home.join("clippo").join("linux-history")
    }
}

fn serialize_state_file(state: LinuxShellState) -> String {
    format!(
        "capture_paused={}\nignore_next_copy={}\n",
        state.capture_paused, state.ignore_next_copy
    )
}

fn parse_state_file(contents: &str) -> io::Result<LinuxShellState> {
    let mut state = LinuxShellState::default();
    for line in contents.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let parsed = match value {
            "true" => true,
            "false" => false,
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("invalid boolean value for {key}"),
                ));
            }
        };
        match key {
            "capture_paused" => state.capture_paused = parsed,
            "ignore_next_copy" => state.ignore_next_copy = parsed,
            _ => {}
        }
    }
    Ok(state)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LinuxHistoryItem {
    text: String,
    pinned: bool,
    pinned_shortcut: Option<char>,
}

impl LinuxHistoryItem {
    fn new(text: &str) -> Self {
        Self {
            text: text.to_string(),
            pinned: false,
            pinned_shortcut: None,
        }
    }
}

fn sort_history_for_display(history: &mut [LinuxHistoryItem]) {
    history.sort_by_key(|item| !item.pinned);
}

fn item_by_visible_shortcut(
    history: &[LinuxHistoryItem],
    shortcut: char,
) -> Option<&LinuxHistoryItem> {
    history.iter().enumerate().find_map(|(index, item)| {
        let visible_shortcut = item
            .pinned_shortcut
            .unwrap_or_else(|| visible_shortcut_for_index(index).unwrap_or('\0'));
        (visible_shortcut == shortcut).then_some(item)
    })
}

fn filter_history_for_search(history: &[LinuxHistoryItem], query: &str) -> Vec<LinuxHistoryItem> {
    let query = query.trim();
    if query.is_empty() {
        return history.to_vec();
    }

    let query = query.to_lowercase();
    history
        .iter()
        .filter(|item| item.text.to_lowercase().contains(&query))
        .cloned()
        .collect()
}

fn visible_shortcut_for_index(index: usize) -> Option<char> {
    "123456789".chars().nth(index)
}

fn serialize_history_file(history: &[LinuxHistoryItem]) -> String {
    history
        .iter()
        .map(|item| {
            format!(
                "{}\t{}\t{}",
                if item.pinned { "1" } else { "0" },
                item.pinned_shortcut.unwrap_or('-'),
                escape_history_text(&item.text)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_history_file(contents: &str) -> io::Result<Vec<LinuxHistoryItem>> {
    contents
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let fields = line.split('\t').collect::<Vec<_>>();
            let (pinned, pinned_shortcut, text) = match fields.as_slice() {
                [pinned, text] => (*pinned, None, *text),
                [pinned, shortcut, text] => {
                    let shortcut = match *shortcut {
                        "-" | "" => None,
                        value => value.chars().next(),
                    };
                    (*pinned, shortcut, *text)
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "history row has invalid field count",
                    ));
                }
            };
            let Some((_, _)) = line.split_once('\t') else {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    "history row missing tab separator",
                ));
            };
            let pinned = match pinned {
                "1" => true,
                "0" => false,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "history row has invalid pin flag",
                    ));
                }
            };
            Ok(LinuxHistoryItem {
                text: unescape_history_text(text),
                pinned,
                pinned_shortcut,
            })
        })
        .collect()
}

fn next_pinned_shortcut(history: &[LinuxHistoryItem]) -> Option<char> {
    "123456789".chars().find(|shortcut| {
        !history
            .iter()
            .any(|item| item.pinned_shortcut == Some(*shortcut))
    })
}

fn escape_history_text(text: &str) -> String {
    text.replace('\\', "\\\\")
        .replace('\t', "\\t")
        .replace('\n', "\\n")
}

fn unescape_history_text(text: &str) -> String {
    let mut output = String::new();
    let mut chars = text.chars();
    while let Some(character) = chars.next() {
        if character == '\\' {
            match chars.next() {
                Some('t') => output.push('\t'),
                Some('n') => output.push('\n'),
                Some('\\') | None => output.push('\\'),
                Some(other) => {
                    output.push('\\');
                    output.push(other);
                }
            }
        } else {
            output.push(character);
        }
    }
    output
}

struct ZenityDialog;

impl ZenityDialog {
    fn info(title: &str, text: &str) -> io::Result<bool> {
        let command = zenity_info_command(title, text);
        match run_dialog_command(&command) {
            Ok(result) => Ok(result),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                eprintln!("Could not show dialog: zenity is unavailable.");
                notify(title, text)
            }
            Err(error) => Err(error),
        }
    }

    fn search_query() -> io::Result<Option<String>> {
        let command = zenity_search_command();
        match Command::new(command.program).args(command.args).output() {
            Ok(output) if output.status.success() => Ok(Some(
                String::from_utf8_lossy(&output.stdout).trim().to_string(),
            )),
            Ok(_) => Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                notify_missing_dialog_backend("history search dialog");
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    fn select_history_item(history: &[LinuxHistoryItem]) -> io::Result<Option<LinuxHistoryItem>> {
        let command = zenity_history_command(history);
        match Command::new(command.program).args(command.args).output() {
            Ok(output) if output.status.success() => {
                let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(history.iter().find(|item| item.text == selected).cloned())
            }
            Ok(_) => Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                notify_missing_dialog_backend("history list dialog");
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }

    fn select_history_action(pinned: bool) -> io::Result<Option<HistoryAction>> {
        let command = zenity_history_action_command(pinned);
        match Command::new(command.program).args(command.args).output() {
            Ok(output) if output.status.success() => {
                let selected = String::from_utf8_lossy(&output.stdout).trim().to_string();
                Ok(HistoryAction::from_label(&selected))
            }
            Ok(_) => Ok(None),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                notify_missing_dialog_backend("history action dialog");
                Ok(None)
            }
            Err(error) => Err(error),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HistoryAction {
    Copy,
    Paste,
    PastePlain,
    ShowFullText,
    TogglePin,
    Delete,
}

impl HistoryAction {
    const COPY_LABEL: &'static str = "Copy";
    const PASTE_LABEL: &'static str = "Paste";
    const PASTE_PLAIN_LABEL: &'static str = "Paste Without Formatting";
    const SHOW_FULL_TEXT_LABEL: &'static str = "Show Full Text";
    const PIN_LABEL: &'static str = "Pin";
    const UNPIN_LABEL: &'static str = "Unpin";
    const DELETE_LABEL: &'static str = "Delete";

    fn labels(pinned: bool) -> [&'static str; 6] {
        [
            Self::COPY_LABEL,
            Self::PASTE_LABEL,
            Self::PASTE_PLAIN_LABEL,
            Self::SHOW_FULL_TEXT_LABEL,
            if pinned {
                Self::UNPIN_LABEL
            } else {
                Self::PIN_LABEL
            },
            Self::DELETE_LABEL,
        ]
    }

    fn from_label(label: &str) -> Option<Self> {
        match label {
            Self::COPY_LABEL => Some(Self::Copy),
            Self::PASTE_LABEL => Some(Self::Paste),
            Self::PASTE_PLAIN_LABEL => Some(Self::PastePlain),
            Self::SHOW_FULL_TEXT_LABEL => Some(Self::ShowFullText),
            Self::PIN_LABEL | Self::UNPIN_LABEL => Some(Self::TogglePin),
            Self::DELETE_LABEL => Some(Self::Delete),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DialogCommand {
    program: &'static str,
    args: Vec<String>,
}

fn zenity_info_command(title: &str, text: &str) -> DialogCommand {
    DialogCommand {
        program: "zenity",
        args: vec![
            "--info".to_string(),
            "--title".to_string(),
            title.to_string(),
            "--text".to_string(),
            text.to_string(),
            "--width".to_string(),
            "420".to_string(),
        ],
    }
}

fn zenity_search_command() -> DialogCommand {
    DialogCommand {
        program: "zenity",
        args: vec![
            "--entry".to_string(),
            "--title".to_string(),
            "Clippo History".to_string(),
            "--text".to_string(),
            "Search clipboard history".to_string(),
            "--entry-text".to_string(),
            String::new(),
            "--width".to_string(),
            "420".to_string(),
        ],
    }
}

fn zenity_history_command(history: &[LinuxHistoryItem]) -> DialogCommand {
    let mut args = vec![
        "--list".to_string(),
        "--title".to_string(),
        "Clippo History".to_string(),
        "--text".to_string(),
        "Use arrow keys and Enter to copy the selected clipboard item.".to_string(),
        "--column".to_string(),
        "Shortcut".to_string(),
        "--column".to_string(),
        "Pin".to_string(),
        "--column".to_string(),
        "Clip".to_string(),
        "--print-column".to_string(),
        "3".to_string(),
        "--width".to_string(),
        "520".to_string(),
        "--height".to_string(),
        "560".to_string(),
    ];

    for (index, item) in history.iter().enumerate() {
        args.push(
            item.pinned_shortcut
                .map(|shortcut| shortcut.to_string())
                .or_else(|| visible_shortcut_for_index(index).map(|shortcut| shortcut.to_string()))
                .unwrap_or_default(),
        );
        args.push(if item.pinned { "Pinned" } else { "" }.to_string());
        args.push(item.text.clone());
    }

    DialogCommand {
        program: "zenity",
        args,
    }
}

fn zenity_history_action_command(pinned: bool) -> DialogCommand {
    let mut args = vec![
        "--list".to_string(),
        "--title".to_string(),
        "Clippo Action".to_string(),
        "--text".to_string(),
        "Choose an action for the selected clipboard item.".to_string(),
        "--column".to_string(),
        "Action".to_string(),
        "--width".to_string(),
        "360".to_string(),
        "--height".to_string(),
        "300".to_string(),
    ];
    args.extend(
        HistoryAction::labels(pinned)
            .into_iter()
            .map(str::to_string),
    );
    DialogCommand {
        program: "zenity",
        args,
    }
}

fn run_dialog_command(command: &DialogCommand) -> io::Result<bool> {
    match Command::new(command.program).args(&command.args).status() {
        Ok(status) => Ok(status.success()),
        Err(error) => Err(error),
    }
}

fn notify_missing_dialog_backend(context: &str) {
    eprintln!("Could not show {context}: zenity is unavailable.");
    let _ = notify(
        "Dialog unavailable",
        "Clippo could not open its temporary Linux dialog backend in this package.",
    );
}

fn desktop_entry(executable_path: &Path) -> String {
    format!(
        "[Desktop Entry]\nType=Application\nName=Clippo\nComment=Native clipboard manager\nExec={}\nTerminal=false\nX-GNOME-Autostart-enabled=true\n",
        executable_path.display()
    )
}

struct X11ShortcutConfig {
    home: PathBuf,
}

impl X11ShortcutConfig {
    const START_MARKER: &'static str = "# BEGIN CLIPPO SHORTCUTS";
    const END_MARKER: &'static str = "# END CLIPPO SHORTCUTS";

    fn from_environment() -> Self {
        let home = env::var_os("HOME").map_or_else(|| PathBuf::from("."), PathBuf::from);
        Self { home }
    }

    fn install_open_history_shortcut(&self, executable_path: &Path) -> io::Result<()> {
        let path = self.config_path();
        let existing = fs::read_to_string(&path).unwrap_or_default();
        let cleaned = remove_managed_block(&existing, Self::START_MARKER, Self::END_MARKER);
        let block = xbindkeys_block(executable_path);
        fs::write(path, append_block(&cleaned, &block))
    }

    fn config_path(&self) -> PathBuf {
        self.home.join(".xbindkeysrc")
    }
}

fn xbindkeys_block(executable_path: &Path) -> String {
    format!(
        "{start}\n\"{} --show-history\"\n  Mod4+Shift + c\n\"{} --preferences\"\n  Mod4 + comma\n\"{} --clear-unpinned\"\n  Mod4+Control + Delete\n\"{} --clear-all\"\n  Mod4+Shift+Control + Delete\n{}{end}\n",
        executable_path.display(),
        executable_path.display(),
        executable_path.display(),
        executable_path.display(),
        xbindkeys_number_shortcuts(executable_path),
        start = X11ShortcutConfig::START_MARKER,
        end = X11ShortcutConfig::END_MARKER
    )
}

fn xbindkeys_number_shortcuts(executable_path: &Path) -> String {
    (1..=9)
        .flat_map(|shortcut| {
            [
                format!(
                    "\"{} --copy-shortcut={}\"\n  Mod4 + {}\n",
                    executable_path.display(),
                    shortcut,
                    shortcut
                ),
                format!(
                    "\"{} --paste-shortcut={}\"\n  Mod4+Mod1 + {}\n",
                    executable_path.display(),
                    shortcut,
                    shortcut
                ),
                format!(
                    "\"{} --paste-plain-shortcut={}\"\n  Mod4+Shift+Mod1 + {}\n",
                    executable_path.display(),
                    shortcut,
                    shortcut
                ),
                format!(
                    "\"{} --toggle-pin-shortcut={}\"\n  Mod4+Shift + {}\n",
                    executable_path.display(),
                    shortcut,
                    shortcut
                ),
                format!(
                    "\"{} --delete-shortcut={}\"\n  Mod4+Control + {}\n",
                    executable_path.display(),
                    shortcut,
                    shortcut
                ),
            ]
        })
        .collect::<String>()
}

fn remove_managed_block(input: &str, start_marker: &str, end_marker: &str) -> String {
    let mut output = Vec::new();
    let mut skipping = false;

    for line in input.lines() {
        if line == start_marker {
            skipping = true;
            continue;
        }
        if line == end_marker {
            skipping = false;
            continue;
        }
        if !skipping {
            output.push(line);
        }
    }

    output.join("\n").trim().to_string()
}

fn append_block(existing: &str, block: &str) -> String {
    if existing.trim().is_empty() {
        block.to_string()
    } else {
        format!("{}\n\n{}", existing.trim(), block)
    }
}

fn notify(title: &str, body: &str) -> io::Result<bool> {
    match Command::new("notify-send")
        .arg(notification_summary(title))
        .arg(body)
        .status()
    {
        Ok(status) => Ok(status.success()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error),
    }
}

fn notification_summary(title: &str) -> String {
    format!("Clippo: {title}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn wayland_fallback_explains_desktop_shortcut() {
        let message = SessionKind::Wayland.shortcut_fallback_message();
        assert!(message.contains("portal"));
        assert!(message.contains("desktop shortcut"));
        assert!(message.contains("clippo-linux --show-history"));
    }

    #[test]
    fn wayland_global_shortcuts_probe_uses_xdg_portal_introspection() {
        let command = WaylandGlobalShortcutsPortal::probe_command();
        assert_eq!(command.program, "gdbus");
        assert!(command.args.iter().any(|arg| arg == "introspect"));
        assert!(command.args.iter().any(|arg| arg == "--session"));
        assert!(command
            .args
            .iter()
            .any(|arg| arg == "org.freedesktop.portal.Desktop"));
        assert!(command
            .args
            .iter()
            .any(|arg| arg == "/org/freedesktop/portal/desktop"));
    }

    #[test]
    fn wayland_global_shortcuts_create_session_command_uses_portal_api() {
        let command = WaylandGlobalShortcutsPortal::create_session_command(
            "clippo_create_shortcuts",
            "clippo_shortcuts",
        );
        assert_eq!(command.program, "gdbus");
        assert!(command.args.iter().any(|arg| arg == "call"));
        assert!(command
            .args
            .iter()
            .any(|arg| arg == "org.freedesktop.portal.GlobalShortcuts.CreateSession"));
        assert!(command
            .args
            .iter()
            .any(|arg| arg.contains("'handle_token': <'clippo_create_shortcuts'>")));
        assert!(command
            .args
            .iter()
            .any(|arg| arg.contains("'session_handle_token': <'clippo_shortcuts'>")));
    }

    #[test]
    fn wayland_global_shortcuts_bind_command_includes_open_history_shortcut() {
        let shortcuts = WaylandGlobalShortcutsPortal::default_shortcuts();
        let command = WaylandGlobalShortcutsPortal::bind_shortcuts_command(
            "/org/freedesktop/portal/desktop/session/clippo_shortcuts",
            "clippo_bind_shortcuts",
            &shortcuts,
        );
        assert_eq!(command.program, "gdbus");
        assert!(command
            .args
            .iter()
            .any(|arg| arg == "org.freedesktop.portal.GlobalShortcuts.BindShortcuts"));
        assert!(command
            .args
            .iter()
            .any(|arg| arg == "/org/freedesktop/portal/desktop/session/clippo_shortcuts"));
        assert!(command.args.iter().any(|arg| arg.contains("open-history")));
        assert!(command
            .args
            .iter()
            .any(|arg| arg.contains("Open Clippo history")));
        assert!(command.args.iter().any(|arg| arg.contains("Super+Shift+C")));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn wayland_shortcut_daemon_matches_only_open_history_activation() {
        let expected =
            OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/session/clippo_shortcuts")
                .expect("expected session handle should be valid");
        let other_session =
            OwnedObjectPath::try_from("/org/freedesktop/portal/desktop/session/other")
                .expect("other session handle should be valid");

        assert!(WaylandShortcutDaemon::activation_matches(
            &expected,
            &expected,
            "open-history"
        ));
        assert!(!WaylandShortcutDaemon::activation_matches(
            &expected,
            &other_session,
            "open-history"
        ));
        assert!(!WaylandShortcutDaemon::activation_matches(
            &expected,
            &expected,
            "other-shortcut"
        ));
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn wayland_shortcut_daemon_tokens_are_request_path_elements() {
        let token = unique_portal_token("clippo_bind_shortcuts");

        assert!(token.starts_with("clippo_bind_shortcuts_"));
        assert!(token
            .chars()
            .all(|character| character.is_ascii_alphanumeric() || character == '_'));
    }

    #[test]
    fn wayland_global_shortcuts_list_and_monitor_commands_use_portal_api() {
        let list = WaylandGlobalShortcutsPortal::list_shortcuts_command(
            "/org/freedesktop/portal/desktop/session/clippo_shortcuts",
            "clippo_list_shortcuts",
        );
        assert!(list
            .args
            .iter()
            .any(|arg| arg == "org.freedesktop.portal.GlobalShortcuts.ListShortcuts"));

        let monitor = WaylandGlobalShortcutsPortal::monitor_command();
        assert_eq!(monitor.program, "gdbus");
        assert!(monitor.args.iter().any(|arg| arg == "monitor"));
        assert!(monitor
            .args
            .iter()
            .any(|arg| arg == "org.freedesktop.portal.Desktop"));
    }

    #[test]
    fn wayland_global_shortcuts_probe_parses_available_interface() {
        let introspection = r"
            interface org.freedesktop.portal.GlobalShortcuts {
              methods:
              signals:
              properties:
                readonly u version = 2;
            };
        ";

        assert_eq!(
            WaylandGlobalShortcutsPortal::parse_introspection(introspection),
            PortalAvailability::Available { version: Some(2) }
        );
    }

    #[test]
    fn wayland_global_shortcuts_probe_reports_missing_interface() {
        let introspection = "interface org.freedesktop.portal.FileChooser {};";
        assert_eq!(
            WaylandGlobalShortcutsPortal::parse_introspection(introspection),
            PortalAvailability::Unavailable
        );
    }

    #[test]
    fn wayland_clipboard_fallback_explains_manual_paths() {
        let messages = SessionKind::Wayland
            .clipboard_fallback_messages()
            .join("\n");
        assert!(messages.contains("manual copy/paste"));
        assert!(messages.contains("copy the selected item"));
        assert!(messages.contains("paste manually"));
    }

    #[test]
    fn desktop_environment_detection_handles_common_desktops() {
        assert_eq!(
            DesktopEnvironment::from_env_values(Some("GNOME"), None),
            DesktopEnvironment::Gnome
        );
        assert_eq!(
            DesktopEnvironment::from_env_values(Some("KDE"), Some("plasma")),
            DesktopEnvironment::Kde
        );
        assert_eq!(
            DesktopEnvironment::from_env_values(None, Some("xfce")),
            DesktopEnvironment::Xfce
        );
        assert_eq!(
            DesktopEnvironment::from_env_values(None, None),
            DesktopEnvironment::Unknown
        );
    }

    #[test]
    fn desktop_environment_reports_tray_strategy() {
        assert!(DesktopEnvironment::Gnome
            .tray_strategy()
            .contains("extension"));
        assert!(DesktopEnvironment::Kde
            .tray_strategy()
            .contains("StatusNotifierItem"));
        assert!(DesktopEnvironment::Unknown
            .tray_strategy()
            .contains("best effort"));
    }

    #[test]
    fn clipboard_backend_matches_session_kind() {
        assert_eq!(
            LinuxClipboardBackend::for_session(SessionKind::X11),
            Some(LinuxClipboardBackend::X11)
        );
        assert_eq!(
            LinuxClipboardBackend::for_session(SessionKind::Wayland),
            Some(LinuxClipboardBackend::Wayland)
        );
        assert_eq!(
            LinuxClipboardBackend::for_session(SessionKind::Unknown),
            None
        );
    }

    #[test]
    fn clipboard_backends_use_expected_linux_tools() {
        let x11_read = LinuxClipboardBackend::X11.read_command();
        assert_eq!(x11_read.program, "xclip");
        assert!(x11_read.args.contains(&"-selection"));
        assert!(x11_read.args.contains(&"clipboard"));

        let wayland_write = LinuxClipboardBackend::Wayland.write_command();
        assert_eq!(wayland_write.program, "wl-copy");
        assert!(wayland_write.args.contains(&"--type"));
    }

    #[test]
    fn paste_backend_is_available_only_for_x11() {
        assert_eq!(
            LinuxPasteBackend::for_session(SessionKind::X11),
            Some(LinuxPasteBackend::X11)
        );
        assert_eq!(LinuxPasteBackend::for_session(SessionKind::Wayland), None);
        assert_eq!(LinuxPasteBackend::for_session(SessionKind::Unknown), None);
    }

    #[test]
    fn x11_paste_backend_uses_xdotool_shortcuts() {
        let formatted = LinuxPasteBackend::X11.paste_command(false);
        assert_eq!(formatted.program, "xdotool");
        assert_eq!(formatted.args, vec!["key", "--clearmodifiers", "ctrl+v"]);

        let plain = LinuxPasteBackend::X11.paste_command(true);
        assert_eq!(plain.program, "xdotool");
        assert_eq!(plain.args, vec!["key", "--clearmodifiers", "ctrl+shift+v"]);
    }

    #[test]
    fn clipboard_monitor_reports_only_changed_non_empty_text() {
        let mut monitor = ClipboardMonitor::default();
        assert_eq!(monitor.poll_changed(|| Ok(None)).unwrap(), None);
        assert_eq!(
            monitor
                .poll_changed(|| Ok(Some("first".to_string())))
                .unwrap(),
            Some("first".to_string())
        );
        assert_eq!(
            monitor
                .poll_changed(|| Ok(Some("first".to_string())))
                .unwrap(),
            None
        );
        assert_eq!(
            monitor
                .poll_changed(|| Ok(Some("second".to_string())))
                .unwrap(),
            Some("second".to_string())
        );
    }

    #[test]
    fn popup_placement_stays_inside_monitor_work_area() {
        let placement = PopupPlacement::near_anchor(
            Rect::new(100, 50, 800, 600),
            Point::new(880, 620),
            Size::new(420, 520),
            ScaleFactor::new(1.0),
        );
        assert_eq!(placement, Rect::new(480, 130, 420, 520));
    }

    #[test]
    fn popup_placement_clamps_to_small_monitors() {
        let placement = PopupPlacement::near_anchor(
            Rect::new(0, 0, 320, 240),
            Point::new(300, 220),
            Size::new(420, 520),
            ScaleFactor::new(1.0),
        );
        assert_eq!(placement, Rect::new(0, 0, 320, 240));
    }

    #[test]
    fn fractional_scale_alignment_keeps_logical_size_stable() {
        let scale = ScaleFactor::new(1.25);
        assert_eq!(scale.align(420), 420);
        assert_eq!(scale.align(421), 421);
    }

    #[test]
    fn xdg_autostart_writes_and_removes_desktop_file() {
        let root = env::temp_dir().join(format!(
            "clippo-autostart-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        let autostart = XdgAutostart {
            config_home: root.clone(),
        };
        let executable = Path::new("/usr/bin/clippo-linux");

        autostart
            .set_enabled(executable, true)
            .expect("autostart file should be written");
        assert!(autostart.is_enabled());

        let contents = fs::read_to_string(autostart.desktop_file_path())
            .expect("autostart file should be readable");
        assert!(contents.contains("Type=Application"));
        assert!(contents.contains("Exec=/usr/bin/clippo-linux"));

        autostart
            .set_enabled(executable, false)
            .expect("autostart file should be removed");
        assert!(!autostart.is_enabled());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn no_args_open_history_by_default_but_status_is_explicit() {
        assert!(should_open_history_by_default(
            &["clippo-linux".to_string()]
        ));
        assert!(!should_open_history_by_default(&[
            "clippo-linux".to_string(),
            "--status".to_string()
        ]));
    }

    #[test]
    fn xbindkeys_block_installs_open_history_shortcut() {
        let block = xbindkeys_block(Path::new("/usr/bin/clippo-linux"));
        assert!(block.contains("\"/usr/bin/clippo-linux --show-history\""));
        assert!(block.contains("Mod4+Shift + c"));
        assert!(block.contains("\"/usr/bin/clippo-linux --preferences\""));
        assert!(block.contains("Mod4 + comma"));
        assert!(block.contains("\"/usr/bin/clippo-linux --clear-unpinned\""));
        assert!(block.contains("Mod4+Control + Delete"));
        assert!(block.contains("\"/usr/bin/clippo-linux --clear-all\""));
        assert!(block.contains("Mod4+Shift+Control + Delete"));
        assert!(block.contains("\"/usr/bin/clippo-linux --copy-shortcut=1\""));
        assert!(block.contains("Mod4 + 1"));
        assert!(block.contains("\"/usr/bin/clippo-linux --paste-shortcut=1\""));
        assert!(block.contains("Mod4+Mod1 + 1"));
        assert!(block.contains("\"/usr/bin/clippo-linux --paste-plain-shortcut=1\""));
        assert!(block.contains("Mod4+Shift+Mod1 + 1"));
        assert!(block.contains("\"/usr/bin/clippo-linux --toggle-pin-shortcut=1\""));
        assert!(block.contains("Mod4+Shift + 1"));
        assert!(block.contains("\"/usr/bin/clippo-linux --delete-shortcut=1\""));
        assert!(block.contains("Mod4+Control + 1"));
    }

    #[test]
    fn x11_shortcut_install_replaces_only_managed_block() {
        let existing =
            "custom-before\n# BEGIN CLIPPO SHORTCUTS\nold\n# END CLIPPO SHORTCUTS\ncustom-after";
        let cleaned = remove_managed_block(
            existing,
            X11ShortcutConfig::START_MARKER,
            X11ShortcutConfig::END_MARKER,
        );
        let updated = append_block(
            &cleaned,
            &xbindkeys_block(Path::new("/usr/bin/clippo-linux")),
        );
        assert!(updated.contains("custom-before"));
        assert!(updated.contains("custom-after"));
        assert!(!updated.contains("\nold\n"));
        assert!(updated.contains("--show-history"));
    }

    #[test]
    fn x11_shortcut_config_writes_xbindkeysrc() {
        let root = env::temp_dir().join(format!(
            "clippo-xbindkeys-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        fs::create_dir_all(&root).expect("temp home should be created");
        let config = X11ShortcutConfig { home: root.clone() };

        config
            .install_open_history_shortcut(Path::new("/usr/bin/clippo-linux"))
            .expect("xbindkeys config should be written");

        let contents =
            fs::read_to_string(config.config_path()).expect("xbindkeys config should be readable");
        assert!(contents.contains("BEGIN CLIPPO SHORTCUTS"));
        assert!(contents.contains("Mod4+Shift + c"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn linux_shell_state_round_trips_key_value_file() {
        let state = LinuxShellState {
            capture_paused: true,
            ignore_next_copy: true,
        };
        let serialized = serialize_state_file(state);
        assert!(serialized.contains("capture_paused=true"));
        assert_eq!(parse_state_file(&serialized).unwrap(), state);
    }

    #[test]
    fn linux_shell_state_store_toggles_pause_and_sets_ignore_next() {
        let root = env::temp_dir().join(format!(
            "clippo-linux-state-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        let store = LinuxShellStateStore {
            state_home: root.clone(),
        };

        let paused = store
            .toggle_capture_paused()
            .expect("pause state should be saved");
        assert!(paused.capture_paused);

        store
            .set_ignore_next_copy()
            .expect("ignore-next state should be saved");
        let state = store.load().expect("state should load");
        assert!(state.capture_paused);
        assert!(state.ignore_next_copy);

        let resumed = store
            .toggle_capture_paused()
            .expect("pause state should toggle");
        assert!(!resumed.capture_paused);

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn invalid_linux_shell_state_is_rejected() {
        let error = parse_state_file("capture_paused=maybe").unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
    }

    #[test]
    fn linux_history_file_round_trips_escaped_text() {
        let history = vec![
            LinuxHistoryItem {
                text: "first\nline".to_string(),
                pinned: true,
                pinned_shortcut: Some('1'),
            },
            LinuxHistoryItem {
                text: "tab\tbackslash\\".to_string(),
                pinned: false,
                pinned_shortcut: None,
            },
        ];
        let serialized = serialize_history_file(&history);
        assert!(serialized.contains("1\t1\tfirst\\nline"));
        assert_eq!(parse_history_file(&serialized).unwrap(), history);
        let legacy = parse_history_file("1\tlegacy pinned").unwrap();
        assert_eq!(legacy[0].text, "legacy pinned");
        assert!(legacy[0].pinned);
        assert_eq!(legacy[0].pinned_shortcut, None);
    }

    #[test]
    fn linux_history_store_deduplicates_latest_text() {
        let root = env::temp_dir().join(format!(
            "clippo-linux-history-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        let store = LinuxShellStateStore {
            state_home: root.clone(),
        };

        store
            .add_history_text("first")
            .expect("first history item should save");
        store
            .add_history_text("second")
            .expect("second history item should save");
        store
            .add_history_text("first")
            .expect("duplicate history item should save");

        let history = store.load_history().expect("history should load");
        assert_eq!(history.len(), 2);
        assert_eq!(history[0].text, "first");
        assert_eq!(history[1].text, "second");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn linux_history_loads_pinned_items_first_after_reload() {
        let root = env::temp_dir().join(format!(
            "clippo-linux-pinned-order-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        let store = LinuxShellStateStore {
            state_home: root.clone(),
        };

        store
            .save_history(&[
                LinuxHistoryItem::new("recent unpinned"),
                LinuxHistoryItem {
                    text: "older pinned".to_string(),
                    pinned: true,
                    pinned_shortcut: Some('1'),
                },
                LinuxHistoryItem::new("older unpinned"),
            ])
            .expect("history should save");

        let history = store
            .load_ordered_history()
            .expect("ordered history should load");
        assert_eq!(history[0].text, "older pinned");
        assert!(history[0].pinned);
        assert_eq!(history[0].pinned_shortcut, Some('1'));
        assert_eq!(history[1].text, "recent unpinned");
        assert_eq!(history[2].text, "older unpinned");

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn linux_history_resolves_visible_shortcuts_after_reload() {
        let root = env::temp_dir().join(format!(
            "clippo-linux-shortcut-resolution-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        let store = LinuxShellStateStore {
            state_home: root.clone(),
        };

        store
            .save_history(&[
                LinuxHistoryItem::new("recent unpinned"),
                LinuxHistoryItem {
                    text: "older pinned".to_string(),
                    pinned: true,
                    pinned_shortcut: Some('7'),
                },
                LinuxHistoryItem::new("older unpinned"),
            ])
            .expect("history should save");

        assert_eq!(
            store
                .item_by_visible_shortcut('7')
                .expect("shortcut lookup should load")
                .map(|item| item.text),
            Some("older pinned".to_string())
        );
        assert_eq!(
            store
                .item_by_visible_shortcut('2')
                .expect("shortcut lookup should load")
                .map(|item| item.text),
            Some("recent unpinned".to_string())
        );
        assert!(store
            .item_by_visible_shortcut('9')
            .expect("shortcut lookup should load")
            .is_none());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn linux_history_store_deletes_pins_and_clears_items() {
        let root = env::temp_dir().join(format!(
            "clippo-linux-history-actions-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system clock should be after epoch")
                .as_nanos()
        ));
        let store = LinuxShellStateStore {
            state_home: root.clone(),
        };

        store.add_history_text("first").unwrap();
        store.add_history_text("second").unwrap();
        assert_eq!(store.toggle_history_pin("first").unwrap(), Some(true));
        assert!(!store.delete_history_text("missing").unwrap());
        assert_eq!(store.clear_unpinned_history().unwrap(), 1);

        let history = store.load_history().unwrap();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0].text, "first");
        assert!(history[0].pinned);
        assert_eq!(history[0].pinned_shortcut, Some('1'));

        assert_eq!(store.toggle_history_pin("first").unwrap(), Some(false));
        assert_eq!(store.clear_all_history().unwrap(), 1);
        assert!(store.load_history().unwrap().is_empty());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn zenity_history_command_models_compact_popup_rows() {
        let command = zenity_history_command(&[
            LinuxHistoryItem {
                text: "Pinned value".to_string(),
                pinned: true,
                pinned_shortcut: Some('7'),
            },
            LinuxHistoryItem::new("Recent value"),
        ]);
        assert_eq!(command.program, "zenity");
        assert!(command.args.contains(&"--list".to_string()));
        assert!(command.args.contains(&"Shortcut".to_string()));
        assert!(command.args.contains(&"Pin".to_string()));
        assert!(command.args.contains(&"Clip".to_string()));
        assert!(command.args.contains(
            &"Use arrow keys and Enter to copy the selected clipboard item.".to_string()
        ));
        assert!(command.args.contains(&"7".to_string()));
        assert!(command.args.contains(&"Pinned value".to_string()));
        assert!(command.args.contains(&"Recent value".to_string()));
    }

    #[test]
    fn zenity_history_command_omits_unusable_shortcuts_after_nine_rows() {
        let history = (1..=10)
            .map(|index| LinuxHistoryItem::new(&format!("item {index}")))
            .collect::<Vec<_>>();

        let command = zenity_history_command(&history);
        let item_10_index = command
            .args
            .iter()
            .position(|arg| arg == "item 10")
            .expect("item 10 should be present");

        assert_eq!(command.args[item_10_index - 2], "");
    }

    #[test]
    fn zenity_history_action_command_exposes_main_item_actions() {
        let command = zenity_history_action_command(false);
        assert_eq!(command.program, "zenity");
        assert!(command.args.contains(&"--list".to_string()));
        assert!(command
            .args
            .contains(&"Choose an action for the selected clipboard item.".to_string()));
        assert!(command.args.contains(&"Copy".to_string()));
        assert!(command.args.contains(&"Paste".to_string()));
        assert!(command
            .args
            .contains(&"Paste Without Formatting".to_string()));
        assert!(command.args.contains(&"Show Full Text".to_string()));
        assert!(command.args.contains(&"Pin".to_string()));
        assert!(command.args.contains(&"Delete".to_string()));
    }

    #[test]
    fn zenity_history_action_command_switches_pin_label_for_pinned_items() {
        let command = zenity_history_action_command(true);
        assert!(command.args.contains(&"Unpin".to_string()));
        assert!(!command.args.contains(&"Pin".to_string()));
    }

    #[test]
    fn history_action_labels_map_to_actions() {
        assert_eq!(HistoryAction::from_label("Copy"), Some(HistoryAction::Copy));
        assert_eq!(
            HistoryAction::from_label("Paste"),
            Some(HistoryAction::Paste)
        );
        assert_eq!(
            HistoryAction::from_label("Paste Without Formatting"),
            Some(HistoryAction::PastePlain)
        );
        assert_eq!(
            HistoryAction::from_label("Show Full Text"),
            Some(HistoryAction::ShowFullText)
        );
        assert_eq!(
            HistoryAction::from_label("Pin"),
            Some(HistoryAction::TogglePin)
        );
        assert_eq!(
            HistoryAction::from_label("Unpin"),
            Some(HistoryAction::TogglePin)
        );
        assert_eq!(
            HistoryAction::from_label("Delete"),
            Some(HistoryAction::Delete)
        );
        assert_eq!(HistoryAction::from_label("Unknown"), None);
    }

    #[test]
    fn zenity_search_command_starts_with_focused_entry() {
        let command = zenity_search_command();
        assert_eq!(command.program, "zenity");
        assert!(command.args.contains(&"--entry".to_string()));
        assert!(command
            .args
            .contains(&"Search clipboard history".to_string()));
        assert!(command.args.contains(&"--entry-text".to_string()));
    }

    #[test]
    fn missing_dialog_command_reports_not_found() {
        let command = DialogCommand {
            program: "clippo-definitely-missing-dialog-command",
            args: Vec::new(),
        };
        let error = run_dialog_command(&command).expect_err("missing dialog should error");
        assert_eq!(error.kind(), io::ErrorKind::NotFound);
    }

    #[test]
    fn linux_history_search_filters_case_insensitively() {
        let history = vec![
            LinuxHistoryItem::new("Invoice Alpha"),
            LinuxHistoryItem::new("release notes"),
            LinuxHistoryItem::new("ALPHA roadmap"),
        ];

        let filtered = filter_history_for_search(&history, "alpha");
        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].text, "Invoice Alpha");
        assert_eq!(filtered[1].text, "ALPHA roadmap");
    }

    #[test]
    fn linux_history_search_empty_query_returns_all_rows() {
        let history = vec![
            LinuxHistoryItem::new("First"),
            LinuxHistoryItem::new("Second"),
        ];

        assert_eq!(filter_history_for_search(&history, "  "), history);
    }

    #[test]
    fn first_char_reads_shortcut_argument() {
        assert_eq!(first_char("7"), Some('7'));
        assert_eq!(first_char(""), None);
    }

    #[test]
    fn visible_shortcut_for_index_stops_after_nine_items() {
        assert_eq!(visible_shortcut_for_index(0), Some('1'));
        assert_eq!(visible_shortcut_for_index(8), Some('9'));
        assert_eq!(visible_shortcut_for_index(9), None);
    }

    #[test]
    fn zenity_preferences_command_uses_info_dialog() {
        let command = zenity_info_command("Clippo Preferences", "Capture paused: false");
        assert_eq!(command.program, "zenity");
        assert!(command.args.contains(&"--info".to_string()));
        assert!(command.args.contains(&"Clippo Preferences".to_string()));
    }

    #[test]
    fn notification_summary_includes_app_name() {
        assert_eq!(notification_summary("Test"), "Clippo: Test");
    }
}
