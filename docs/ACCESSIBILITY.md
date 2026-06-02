# Accessibility And Permissions

Clippo needs to explain platform permissions clearly because clipboard managers interact with sensitive OS features.

Detailed permission copy and fallback behavior lives in [PERMISSIONS.md](PERMISSIONS.md).

## macOS

Automatic paste requires Accessibility permission. The macOS shell must explain that permission before sending the user to System Settings.

Screen Recording or Input Monitoring should only be requested if future implementation proves they are required.

## Windows

Paste automation can be limited around elevated apps. Clippo should document when paste cannot cross privilege boundaries.

## Linux

Wayland environments may restrict global shortcuts, clipboard monitoring, or paste automation. Clippo should use portals where available and document compositor-specific limitations.

## Accessibility QA

Every native shell should verify keyboard focus, screen reader labels for the history list and preferences, high contrast mode, and reduced motion behavior where applicable.

Current source support:

- macOS SwiftUI shell labels the search field, history list, row shortcut hints, pinned state, footer buttons, and preferences toggles. The popup focuses search on appear and exposes full row text through native help.
- Windows WinForms shell assigns accessible names/descriptions to the search field, history list, footer buttons, preferences window, and preferences checkboxes. The popup focuses search when opened.
- Linux fallback relies on native zenity dialog labels, column titles, desktop action names, and command output until the GTK4/libadwaita shell exists.
- Current platform shells do not define custom UI animations. `scripts/check-ui-motion.sh` blocks common custom animation APIs until reduced-motion behavior is explicitly implemented and reviewed.
- Current platform shells inherit high-contrast behavior from native controls and system themes. `scripts/check-high-contrast-source.sh` blocks common contrast opt-outs, owner-drawn controls, and forced Linux themes in source.
- Current platform shells include source-level layout safeguards for scaling and text wrapping. `scripts/check-layout-scaling-source.sh` verifies stable dimensions, wrapping controls, and scale-aware placement hooks before runtime screenshots are collected.
- `scripts/check-accessibility-source.sh` verifies that the current platform shells keep their baseline accessibility labels, initial search focus hooks, and Linux search-before-list flow in source.
- `scripts/check-screen-reader-source.sh` verifies baseline screen-reader source support for history and preferences: macOS labels and hints, Windows accessible names, descriptions, and control roles, and Linux native dialog titles and columns.
- `scripts/check-full-preview-source.sh` verifies that macOS rows expose native help text, Windows rows expose native tooltips, and the Linux fallback exposes a full-text action dialog.

Do not mark accessibility QA complete until VoiceOver, Windows screen reader, GNOME/KDE screen reader, keyboard focus, high contrast, and reduced-motion behavior are tested on the target operating systems.
