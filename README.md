# Clippo

![Clippo icon](assets/icon.svg)

Clippo is a lightweight, native-feeling clipboard manager for macOS, Windows, and Linux, inspired by [Maccy](https://github.com/p0deje/Maccy).

> Status: pre-alpha. Clippo is not ready for daily use yet.

## Why Clippo

Maccy is an excellent macOS clipboard manager. Clippo takes that keyboard-first, privacy-conscious workflow as inspiration and aims to bring equivalent native behavior to Windows and Linux as a separate long-term open-source project.

## Feature Goals

- Searchable clipboard history.
- Global shortcut to open clipboard history.
- Keyboard-first selection and paste workflow.
- Pinning for frequently used clips.
- Paste with or without formatting where the operating system allows it.
- Delete individual items, clear regular history, or clear all history.
- Pause clipboard capture.
- Ignore only the next copy.
- Configurable ignored clipboard types and privacy rules.
- Local-only storage with no telemetry by default.
- Native-feeling UI for each supported operating system.
- Low idle memory usage as a first-class engineering goal.

## Current Implementation

- Shared Rust core for clipboard item modeling, history ordering, search, selection, command routing, settings, privacy rules, localization keys, lifecycle state, and diagnostics.
- Shared persistence crate for JSON storage, migrations, retention, import/export, and crash-safe writes.
- Shared platform abstraction crate for clipboard, paste, shortcuts, tray/menu, notifications, autostart, permissions, polling, and single-instance behavior.
- macOS SwiftUI/AppKit shell scaffold with menu bar UI, search-focused history window, pasteboard polling, copy/paste actions, pause/ignore actions, preferences, launch-at-login, global hotkey registration, notifications, and display-aware placement.
- Windows WinForms shell scaffold with tray menu, global hotkey, clipboard listener, searchable history popup, copy/paste actions, pause/ignore actions, preferences, startup registration, notifications, UAC-aware paste fallback, and Clipboard History coexistence safeguards.
- Linux Rust shell scaffold with X11/Wayland clipboard plumbing, X11 paste automation, Wayland manual-paste fallback, X11 shortcut helper, XDG autostart, desktop launcher actions, `.deb` packaging, zenity fallback dialogs, persisted state/history, and desktop-environment detection.

## Screenshots

Screenshots and short workflow GIFs will be added after the first native shell is usable.

## Platform Plan

| Platform | Target | Notes |
| --- | --- | --- |
| macOS | Native shell scaffold | Uses SwiftUI/AppKit source; runtime validation still needs macOS and Xcode. |
| Windows | Native shell scaffold | Uses WinForms source; runtime validation still needs Windows and the .NET desktop workload. |
| Linux X11 | Shell scaffold plus fallback dialogs | Supports command-backed clipboard, paste, shortcut, autostart, desktop actions, and `.deb` packaging; full GTK/libadwaita UI is still pending. |
| Linux Wayland | Shell scaffold plus documented fallbacks | Clipboard support uses `wl-clipboard` where available; shortcuts and paste automation depend on compositor/portal behavior. |

## Install

Clippo does not have public release artifacts yet. Local development packaging exists for Linux `.deb`, while macOS signing, Windows installer packaging, AppImage, and Flatpak still require native tooling or external validation before release.

```sh
scripts/package-linux-deb.sh
```

## Keyboard-First Workflow

Clippo will be designed so the main workflow can happen without leaving the keyboard:

- Open history with a global shortcut.
- Type to search immediately.
- Select a result with keyboard navigation or a numbered shortcut.
- Paste normally or paste without formatting where supported.
- Pin, delete, clear, pause capture, or ignore the next copy with platform-appropriate shortcuts.

Planned default shortcuts:

| Action | Default |
| --- | --- |
| Open history | `Shift+Meta+C` |
| Paste selected item | `Alt+Enter` |
| Paste without formatting | `Alt+Shift+Enter` |
| Delete selected item | `Alt+Delete` |
| Pin or unpin selected item | `Alt+P` |
| Clear unpinned items | `Alt+Meta+Delete` |
| Clear all items | `Shift+Alt+Meta+Delete` |
| Open preferences | `Meta+Comma` |

`Meta` renders as Command on macOS, Windows key on Windows where appropriate, and Super on Linux where appropriate.

## Architecture Direction

Clippo is built around a Rust shared core with native platform shells:

- Rust shared core for history, search, settings, persistence, privacy rules, and platform-neutral behavior.
- Native macOS shell using SwiftUI/AppKit.
- Native Windows shell using WinUI 3 or an equivalent native Windows UI.
- Native Linux shell using GTK4/libadwaita unless platform research rejects it.

This direction is intended to keep memory usage low while still giving each OS a familiar interface.

## Roadmap

Public roadmap and milestones live in [ROADMAP.md](ROADMAP.md).
The Maccy parity matrix lives in [docs/PARITY.md](docs/PARITY.md).
Project direction, UI/UX constraints, and v1 non-goals live in [docs/PROJECT_DIRECTION.md](docs/PROJECT_DIRECTION.md).
Packaging decisions live in [docs/PACKAGING.md](docs/PACKAGING.md).
Keyboard shortcuts live in [docs/KEYBOARD_SHORTCUTS.md](docs/KEYBOARD_SHORTCUTS.md).
Permission explanations live in [docs/PERMISSIONS.md](docs/PERMISSIONS.md).
Paste behavior lives in [docs/PASTE_BEHAVIOR.md](docs/PASTE_BEHAVIOR.md).
Community feedback policy lives in [docs/COMMUNITY_FEEDBACK.md](docs/COMMUNITY_FEEDBACK.md).
GitHub milestone planning lives in [docs/MILESTONES.md](docs/MILESTONES.md).
Roadmap issue drafts live in [docs/ROADMAP_ISSUES.md](docs/ROADMAP_ISSUES.md).
External validation evidence for remaining OS and release gates lives in [docs/EXTERNAL_VALIDATION.md](docs/EXTERNAL_VALIDATION.md).
Commit-friendly validation log templates live in [docs/validation/](docs/validation/).

Current major milestones:

- v0.1: Buildable local app on at least one OS.
- v0.2: Tested shared core for history, search, persistence, and settings.
- v0.3: One native platform shell usable end to end.
- v0.4: macOS, Windows, and Linux can open, search, copy, and paste history items.
- v0.5: Core Maccy workflow parity across supported OSes.
- v1.0: Documented, packaged, tested release with known limitations clearly stated.

## Privacy Goals

- Clipboard history stays local by default.
- No telemetry by default.
- Logs and diagnostics should redact clipboard contents.
- Users should be able to clear stored clipboard data quickly.
- Platform permission requirements must be documented clearly.

See [docs/PRIVACY.md](docs/PRIVACY.md) for the current privacy model.
See [docs/PERMISSIONS.md](docs/PERMISSIONS.md) for why clipboard access and paste automation permissions may be requested.

## Performance Goals

Clippo is intended to stay lightweight as an always-running utility. The current v1 targets are under 50 MB idle memory where feasible, under 100 ms popup open latency, and near-zero idle CPU when the clipboard has not changed.

Current shared-core benchmark snapshot from 2026-06-01 on Linux 6.14 x86_64:

| Benchmark | Result |
| --- | ---: |
| Add 1,000 history items | 2,153 us |
| Search 1,000 history items | 174 us |
| Save 1,000 items to JSON | 331 us |
| Load 1,000 items from JSON | 205 us |

Native shell memory, popup latency, and CPU wakeup measurements still need to be recorded before stable release. See [docs/PERFORMANCE_TARGETS.md](docs/PERFORMANCE_TARGETS.md).

## FAQ

### Is Clippo ready to use?

Not yet. The project is in pre-alpha. The shared core and shell scaffolds exist, but full native UI parity, packaging, and runtime validation are still in progress.

### Will Linux Wayland have full feature parity?

Clippo will target equivalent workflows on Wayland, but global shortcuts, clipboard access, and paste automation depend on desktop portals and compositor support. Any limitations will be documented instead of hidden.

### What if Clippo's global shortcut conflicts with another app?

Shortcut conflict detection and troubleshooting will be part of the preferences work. The final defaults have not been chosen yet.

### Why does paste automation need permissions?

Automatic paste requires Clippo to interact with the active app after you choose a history item. Each operating system handles that permission differently, so Clippo will document the requirement per platform.

### Will Clippo collect telemetry?

No telemetry is planned by default.

### Is Clippo an official Maccy port?

No. Clippo is planned as a separate project inspired by Maccy.

## Maccy Attribution

Clippo is inspired by [Maccy](https://github.com/p0deje/Maccy), a fast and lightweight clipboard manager for macOS. Clippo is planned as a separate cross-platform project, not an official Maccy port.

The initial parity baseline is Maccy 2.6.1, with a local reference copy stored in `references/maccy/2.6.1/source`.

## Contributing

Read [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/CONTRIBUTOR_SETUP.md](docs/CONTRIBUTOR_SETUP.md) before opening a pull request. Good first issues and help-wanted labels are defined in `.github/labels.yml`.
