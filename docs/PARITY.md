# Maccy Parity Matrix

Clippo's initial parity baseline is Maccy 2.6.1, verified as the latest GitHub release on 2026-06-01 and re-audited on 2026-06-01 before v1 scope work. Maintainers may keep local reference material outside the public repository, but the public baseline is the upstream Maccy release.

Latest upstream release checked: Maccy 2.6.1, release commit `dec6601`, GitHub release page `https://github.com/p0deje/Maccy/releases`.

Clippo targets equivalent workflows on macOS, Windows, and Linux. Exact behavior can differ where OS permission models, desktop portals, or shell integrations require it.

## Core Workflow

| Maccy 2.6.1 behavior | Clippo target | Status |
| --- | --- | --- |
| Open clipboard history with a global shortcut | Platform-equivalent global shortcut | Implemented in macOS and Windows scaffolds; X11 helper exists; Wayland portal support pending |
| Open clipboard history from menu bar icon | Menu bar, tray, or desktop shell equivalent | Implemented in macOS menu bar, Windows tray, and Linux desktop actions |
| Type immediately to search history | Focused search on popup open | Implemented in shared core plus macOS/Windows scaffolds; final Linux GTK search pending |
| Select item with Enter, click, or numbered shortcut | Keyboard and pointer selection | Shared selection model exists; macOS/Windows shell selection exists; Linux fallback click selection and shortcut command resolution exist |
| Select and paste with modifier shortcut | Platform paste automation where allowed | Shared command routing plus macOS/Windows/X11 paste paths and numbered paste shortcuts exist; final cross-shell validation pending |
| Paste without formatting | Plain-text paste path where supported | Shared paste planning plus macOS/Windows/X11 plain-text shortcut paths exist |
| Delete selected item | Remove one history item | Shared core, macOS commands, Windows popup, and Linux fallback history commands exist |
| Show full text after hover delay | Tooltip or expanded preview | Shared full-preview data exists; Windows tooltip exists; final cross-shell UI validation pending |
| Pin or unpin item | Pinned item state and stable shortcut | Shared core, macOS commands, Windows popup, and Linux fallback history commands exist |
| Keep pinned items above regular history | Shared ordering rule | Implemented in shared core and popup view model |
| Clear unpinned history | Preserve pinned items | Implemented in shared core and shell scaffolds/fallbacks |
| Clear all history including pinned | Remove all items | Implemented in shared core and shell scaffolds/fallbacks |
| Disable clipboard capture | Pause capture state | Implemented in shared core and shell scaffolds/fallbacks |
| Ignore only the next copy | One-shot ignore state | Implemented in shared core and shell scaffolds/fallbacks |
| Open preferences | Native preferences window | Implemented in macOS/Windows scaffolds and Linux zenity fallback; final GTK/libadwaita preferences pending |

## UI/UX Reference

Clippo should feel familiar to Maccy users. The target is not a pixel-perfect clone, because Windows and Linux need native shell conventions, but the main interaction model should stay close to Maccy:

- Compact popup centered around immediate search.
- Keyboard-first list navigation.
- Pinned items kept visually and behaviorally distinct at the top.
- Visible shortcut hints for fast selection.
- Menu bar, tray, or desktop-shell actions equivalent to Maccy's menu actions.
- Tooltip or native equivalent for delayed full-content preview.
- Preferences organized around behavior, appearance, shortcuts, ignored types, and platform permissions.

`scripts/check-maccy-ui-parity-source.sh` guards this source-level structure across the docs and platform shells. Side-by-side screenshots and target-host visual review are still required before the UI parity validation tasks can be marked complete.

## Advanced Behavior

| Maccy 2.6.1 behavior | Clippo target | Status |
| --- | --- | --- |
| Ignore confidential or transient clipboard types | OS-specific ignore rules | Implemented in shared privacy defaults for Maccy pasteboard types, Windows-sensitive formats, and Linux-sensitive MIME types |
| Customize ignored clipboard types | Shared settings plus platform mapping | Implemented in shared settings import/export and validation |
| Tune clipboard check interval | Setting where polling is used | Implemented in shared settings with validation |
| Handle shortcut conflicts | Platform-specific validation and help | Shared shortcut validation exists; native conflict detection still platform-dependent |
| Ignore Universal Clipboard on macOS | macOS-specific privacy option if detectable | Shared privacy rule exists; macOS runtime validation pending |
| Explain password-field shortcut limitations | Platform-specific troubleshooting | Documented in permissions/troubleshooting; live validation pending |
| Localization support | Decide v1 language scope and translation workflow | English and pseudo-localization files with validation exist |

## Platform Difference Policy

- macOS should be closest to Maccy because both use Apple desktop APIs.
- Windows should match workflows using Windows-native shortcuts, tray behavior, and paste automation.
- Linux X11 should support equivalent workflows where desktop APIs allow it.
- Linux Wayland limitations must be documented clearly because clipboard access, global shortcuts, and paste automation can depend on portals and compositor support.

## Documented OS Differences

| Area | Maccy behavior | Clippo cross-platform difference |
| --- | --- | --- |
| Menu entry point | macOS menu bar app | macOS should use the menu bar, Windows should use a tray icon, and Linux should use status notifier or desktop-shell integration where available. |
| Modifier labels | macOS Command/Option labels | Clippo stores shared `Meta` shortcuts and renders Command, Windows, or Super labels per OS. |
| Paste automation | macOS Accessibility-backed paste | Windows may be blocked by elevated apps, and Linux Wayland may depend on compositor or portal support. Manual paste fallback is required. |
| Clipboard monitoring | NSPasteboard polling and macOS pasteboard metadata | Windows and Linux need platform-specific clipboard APIs, and Wayland may expose fewer events or metadata. |
| Popup placement | macOS menu bar, Spaces, and screen behavior | Windows and Linux must account for taskbars, panels, multi-monitor scaling, and compositor placement restrictions. |
| Ignored types | Maccy pasteboard type defaults | Clippo keeps Maccy-compatible macOS defaults and adds Windows format names and Linux MIME type rules where exposed. |
| Localization | Maccy's mature community translations | Clippo v1 requires English plus pseudo-localization checks; community translations are post-v1 unless contributors add them earlier. |
