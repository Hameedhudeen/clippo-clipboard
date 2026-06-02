# Roadmap Issue Drafts

These are GitHub-ready issue drafts for roadmap work that should be opened when the repository is published. Keep them aligned with `ROADMAP.md`, `docs/EXTERNAL_VALIDATION.md`, and `docs/POST_V1_RESEARCH.md`.

## v0.3 Native Shell

### Build one native shell end to end

Labels: `platform:linux`, `type:feature`, `priority:high`
Milestone: `v0.3`

Implement one native shell that can open Clippo, capture clipboard history, search, copy, paste, pause capture, ignore next copy, and open preferences.

Acceptance criteria:

- A real native app window or popup opens.
- Search is focused immediately.
- Copy and paste work through platform APIs.
- Manual smoke checklist is updated with screenshots.
- The v0.3 milestone can be marked complete in the roadmap and validation matrix.

## v0.4 Cross-Platform Shells

### Implement macOS shell workflow parity

Labels: `platform:macos`, `type:feature`, `parity:workflow`
Milestone: `v0.4`

Build the macOS SwiftUI/AppKit shell using the shared Rust core and Clippo UI/UX reference.

Acceptance criteria:

- Menu bar icon opens the history popup.
- Global shortcut opens the popup.
- Paste automation handles Accessibility permission.
- Preferences expose shared settings.
- Universal Clipboard behavior is tested or documented.

### Implement Windows shell workflow parity

Labels: `platform:windows`, `type:feature`, `parity:workflow`
Milestone: `v0.4`

Build the Windows native shell with tray integration, global shortcut registration, clipboard monitoring, paste automation, and preferences.

Acceptance criteria:

- Tray icon opens Clippo actions.
- Global shortcut opens the popup.
- Clipboard updates are captured through Windows APIs.
- Elevated-app paste limitation is documented and handled.
- DPI scaling is tested at common settings.

### Implement Linux shell workflow parity

Labels: `platform:linux`, `type:feature`, `parity:workflow`
Milestone: `v0.4`

Build the GTK4/libadwaita Linux shell with desktop-shell integration, clipboard monitoring, shortcut support where available, and Wayland fallback messaging.

Acceptance criteria:

- GNOME Wayland, GNOME X11, and KDE behavior is documented.
- X11 global shortcut and clipboard monitoring are implemented.
- Wayland portal/fallback behavior is implemented where available.
- Status notifier or desktop-shell integration is implemented or documented as unsupported.

## v0.5 Workflow Parity

### Complete Clippo workflow parity review

Labels: `parity:workflow`, `type:task`, `priority:high`
Milestone: `v0.5`

Review the documented Clippo workflow scope before declaring cross-platform workflow parity.

Acceptance criteria:

- `docs/PARITY.md` is updated with current implementation status.
- Any unsupported OS-specific differences are documented.
- UI/UX review has been completed for every shell.
- No unresolved v0.5 parity gates remain in the roadmap or validation matrix.

## v0.7 Packaging

### Add signed or checksummed release artifacts

Labels: `type:release`, `security`, `priority:high`
Milestone: `v0.7`

Package Clippo for macOS, Windows, and Linux with checksums and documented install/uninstall paths.

Acceptance criteria:

- macOS app bundle is built and signing/notarization path is documented.
- Windows MSIX or MSI package is built.
- Linux AppImage and at least one distro package path are built or deferred with rationale.
- Checksums are published in release notes.

## Post-v1 Research

### Evaluate encrypted local history

Labels: `post-v1`, `security`, `privacy`
Milestone: `post-v1`

Research platform keychain-backed encrypted history storage without weakening clear-history, backup/restore, or recovery behavior.

### Evaluate opt-in cloud sync

Labels: `post-v1`, `privacy`, `needs-design`
Milestone: `post-v1`

Research cloud sync only after local-first desktop behavior is stable. The design must include encryption, account deletion, conflict resolution, and a revised threat model.

### Evaluate OCR for copied images

Labels: `post-v1`, `performance`, `privacy`
Milestone: `post-v1`

Research opt-in OCR for images with clear CPU, battery, storage, and privacy limits.

### Evaluate plugin or scripting support

Labels: `post-v1`, `security`, `needs-design`
Milestone: `post-v1`

Research extensibility only after the permission and threat model for exposing clipboard content to scripts is designed.
