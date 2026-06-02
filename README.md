# Clippo

![Clippo icon](assets/icon.svg)

[![CI](https://github.com/Hameedhudeen/clippo-clipboard/actions/workflows/ci.yml/badge.svg)](https://github.com/Hameedhudeen/clippo-clipboard/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/Hameedhudeen/clippo-clipboard?include_prereleases)](https://github.com/Hameedhudeen/clippo-clipboard/releases)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Clippo is an open-source, cross-platform clipboard history manager for macOS, Windows, and Linux. It is built for people who want a fast keyboard-first clipboard workflow, local-only storage, native desktop behavior, and low background resource usage.

> Status: alpha. Clippo is under active development and is not ready for daily use yet.

## What Clippo Aims To Do

Clippo is designed as a small desktop utility that stays out of the way until you need your clipboard history.

- Keep a searchable history of recent clipboard items.
- Open the history popup with a global shortcut.
- Search immediately when the popup opens.
- Select, copy, or paste items from the keyboard.
- Paste with or without formatting where the operating system allows it.
- Pin frequently used clips above regular history.
- Delete one item, clear regular history, or clear all history.
- Pause clipboard capture when needed.
- Ignore only the next copy.
- Configure ignored clipboard types and privacy rules.
- Choose on first launch whether Clippo should start automatically after sign-in.
- Store data locally with no telemetry by default.
- Use native-feeling macOS, Windows, and Linux interfaces.
- Stay lightweight as an always-running menu bar, tray, or desktop-shell utility until explicitly quit.

## Project Status

Clippo is currently an alpha open-source project. The shared Rust core and native shell scaffolds exist, but the app still needs broader target-host validation, signing, screenshots, and stable release hardening.

| Area | Status |
| --- | --- |
| Shared Rust core | Implemented for history, search, settings, privacy, persistence, commands, lifecycle, localization keys, and diagnostics. |
| macOS shell | SwiftUI/AppKit scaffold exists; an unsigned alpha zipped app bundle is published for tester feedback. |
| Windows shell | WinForms scaffold exists; an unsigned self-contained alpha zip is published for tester feedback. |
| Linux shell | Rust fallback shell exists for a resident background monitor, X11/Wayland clipboard paths, zenity dialogs with notification fallback, desktop actions, autostart, quit command, Wayland shortcut portal daemon, and published alpha package artifacts. |
| Releases | `v0.1.2` alpha artifacts are published for tester feedback; stable cross-platform releases are pending. |

## Screenshots

Screenshots and short workflow GIFs will be added after the first native shell is usable enough to represent the project honestly.

## Install

Clippo has alpha release artifacts for tester feedback. They are not signed or ready for daily use yet.

| Artifact | Download |
| --- | --- |
| Linux AppImage | [Clippo-0.1.2-x86_64.AppImage](https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.2/Clippo-0.1.2-x86_64.AppImage) |
| Debian package | [clippo_0.1.2_amd64.deb](https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.2/clippo_0.1.2_amd64.deb) |
| Flatpak local repo archive | [clippo-0.1.2-flatpak-repo.tar.gz](https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.2/clippo-0.1.2-flatpak-repo.tar.gz) |
| macOS app bundle zip | [Clippo-0.1.2-macos-alpha.zip](https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.2/Clippo-0.1.2-macos-alpha.zip) |
| Windows self-contained zip | [Clippo-0.1.2-windows-x64-alpha.zip](https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.2/Clippo-0.1.2-windows-x64-alpha.zip) |
| Checksums | [SHA256SUMS](https://github.com/Hameedhudeen/clippo-clipboard/releases/download/v0.1.2/SHA256SUMS) |

macOS and Windows alpha downloads are unsigned. Expect normal operating-system warnings until signing and notarization are added.

Contributors can also build from source:

```sh
git clone https://github.com/Hameedhudeen/clippo-clipboard.git
cd clippo-clipboard
scripts/check.sh
```

Linux packaging commands:

```sh
scripts/package-linux-deb.sh
scripts/package-linux-appimage.sh
scripts/package-linux-flatpak.sh
```

macOS signing/notarization and Windows installer signing still require target-platform validation before a stable release.

## Keyboard-First Workflow

The core workflow should work without leaving the keyboard:

- Open history with a global shortcut.
- Type to search immediately.
- Select a result with arrow keys, Enter, pointer selection, or numbered shortcuts.
- Paste normally or paste without formatting where supported.
- Pin, delete, clear, pause capture, ignore the next copy, or open preferences from native actions.

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

## Platform Support Plan

| Platform | Target | Current notes |
| --- | --- | --- |
| macOS | Native menu bar app | SwiftUI/AppKit source exists; signed bundle validation is pending. |
| Windows | Native tray app | WinForms source exists; tray lifecycle is implemented; installer and runtime validation are pending. |
| Linux X11 | Native shell plus fallbacks | Resident monitor, clipboard, paste, shortcut helper, desktop actions, packaging, and fallback dialogs exist. |
| Linux Wayland | Native shell plus documented fallbacks | Resident monitor, clipboard paths, manual paste fallback, and a portal daemon exist; shortcut activation still depends on compositor portal support. |

## Architecture

Clippo uses a shared Rust core with native platform shells:

- `clippo-core`: history, search, selection, command routing, settings, privacy, lifecycle, and diagnostics.
- `clippo-persistence`: local JSON storage, migrations, retention, import/export, and crash-safe writes.
- `clippo-platform`: shared platform traits for clipboard, paste, shortcuts, tray/menu, notifications, autostart, permissions, polling, and single-instance behavior.
- `apps/macos`: native macOS shell scaffold.
- `apps/windows`: native Windows shell scaffold.
- `apps/linux`: Linux shell scaffold and fallback integration.

This structure keeps shared behavior testable while allowing each operating system to feel native.

## Privacy

Clippo is designed around local-first privacy:

- Clipboard history stays on the user's machine by default.
- No telemetry is planned by default.
- Logs and diagnostics should redact clipboard contents.
- Users should be able to clear stored clipboard data quickly.
- Platform permission requirements must be documented clearly.

See [docs/PRIVACY.md](docs/PRIVACY.md) and [docs/PERMISSIONS.md](docs/PERMISSIONS.md).

## Performance Goals

Clippo is intended to stay comfortable as an always-running utility. The v1 targets are under 50 MB idle memory where feasible, under 100 ms popup open latency, and near-zero idle CPU when the clipboard has not changed.

Current shared-core benchmark snapshot from 2026-06-02 on Linux 6.14 x86_64:

| Benchmark | Result |
| --- | ---: |
| Add 1,000 history items | 5,447 us |
| Search 1,000 history items | 389 us |
| Build popup view model for 200 visible rows | 142 us |
| Build searched popup view model for 200 visible rows | 385 us |
| Save 1,000 items to JSON | 817 us |
| Load 1,000 items from JSON | 486 us |

Native shell memory, popup latency, and CPU wakeup measurements still need to be recorded before stable release. See [docs/PERFORMANCE_TARGETS.md](docs/PERFORMANCE_TARGETS.md).

## Roadmap

Clippo's public roadmap is tracked in this repository:

- [ROADMAP.md](ROADMAP.md): current milestone plan.
- [docs/PARITY.md](docs/PARITY.md): workflow parity matrix.
- [docs/PROJECT_DIRECTION.md](docs/PROJECT_DIRECTION.md): project goals, non-goals, and platform targets.
- [docs/UI_UX.md](docs/UI_UX.md): UI/UX reference and native interface rules.
- [docs/EXTERNAL_VALIDATION.md](docs/EXTERNAL_VALIDATION.md): validation gates that require target operating systems.
- [docs/MILESTONES.md](docs/MILESTONES.md): GitHub milestone structure.

Current major milestones:

- v0.1: Buildable local app on at least one OS.
- v0.2: Tested shared core for history, search, persistence, and settings.
- v0.3: One native platform shell usable end to end.
- v0.4: macOS, Windows, and Linux can open, search, copy, and paste history items.
- v0.5: Core clipboard workflow parity across supported OSes.
- v1.0: Documented, packaged, tested release with known limitations clearly stated.

## Contributing

Contributions are welcome, especially:

- Target-platform testing on macOS, Windows, Linux X11, and Linux Wayland.
- Accessibility and screen-reader feedback.
- Packaging help for macOS, Windows, AppImage, Flatpak, and `.deb`.
- Documentation fixes and troubleshooting notes.
- Focused pull requests for roadmap items.

Before opening a pull request, read [CONTRIBUTING.md](CONTRIBUTING.md) and [docs/CONTRIBUTOR_SETUP.md](docs/CONTRIBUTOR_SETUP.md).

## FAQ

### Is Clippo ready to use?

Clippo is available as an alpha release for tester feedback. It is not recommended as a daily-driver clipboard manager yet because full native UI validation, signing, notarization, and stable release hardening are still in progress.

### Will Clippo collect telemetry?

No telemetry is planned by default.

### Will Linux Wayland have full feature parity?

Clippo targets equivalent workflows on Wayland, but global shortcuts, clipboard access, and paste automation depend on desktop portals and compositor support. Limitations will be documented clearly.

### Why does paste automation need permissions?

Automatic paste requires Clippo to return focus to the target app and send the platform paste command after you choose a history item. Each operating system handles this differently, so Clippo documents permission requirements per platform.

## License

Clippo is released under the [MIT License](LICENSE).
