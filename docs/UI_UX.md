# UI/UX Reference

Clippo uses its own compact, keyboard-first clipboard workflow as the UI/UX reference. The goal is a familiar native utility on every platform, not a pixel-perfect copy of another app.

## Core Interaction Notes

- Popup opens from a global shortcut or shell entry point.
- Search is immediately focused.
- Results are compact and optimized for keyboard scanning.
- Pinned items stay above regular history.
- Numbered shortcuts select visible items quickly.
- Platform modifiers choose copy, paste, paste-without-formatting, delete, and pin actions.
- Tooltip delay or a native equivalent reveals full item contents.
- Menu, tray, or desktop actions cover clear, pause capture, ignore next copy, and preferences.

## Cross-Platform Preservation

Preserve across all OSes:

- Compact searchable list.
- Search-first flow.
- Keyboard-first navigation.
- Pinned section above regular history.
- Shortcut hints on visible rows.
- Delayed full-content preview or native full-text action.
- Pause and ignore-next states.

`scripts/check-workflow-ui-source.sh` verifies that the current docs and platform shell source still represent Clippo's compact workflow structure: search-first popup flow, compact history list, pinned/history separation, visible shortcut hints, full-preview affordance, and core actions. This source check does not replace target-host screenshot review before beta releases.

Adapt per OS:

- Modifier labels and default shortcut rendering.
- Menu bar versus system tray versus shell integration.
- Permission wording.
- Window chrome, spacing, and typography.

## Wireframe: History Popup

```text
+------------------------------------------+
| Search clipboard history                 |
+------------------------------------------+
| Pinned                                   |
| 1  Frequently used clip             PIN  |
| 2  Another pinned clip              PIN  |
+------------------------------------------+
| History                                  |
| 3  Recent copied text                    |
| 4  https://example.com                   |
| 5  Image (image/png, 800x600)            |
+------------------------------------------+
| Clear   Pause   Ignore Next   Preferences|
+------------------------------------------+
```

## Wireframe: Empty State

```text
+------------------------------------------+
| Search clipboard history                 |
+------------------------------------------+
| No clipboard history yet                 |
+------------------------------------------+
| Pause   Ignore Next   Preferences        |
+------------------------------------------+
```

## Wireframe: Paused State

```text
+------------------------------------------+
| Search clipboard history        Paused   |
+------------------------------------------+
| Existing items remain searchable          |
+------------------------------------------+
| Resume   Ignore Next   Preferences       |
+------------------------------------------+
```

## Platform Notes

- macOS should use menu bar expectations and Apple desktop conventions.
- Windows should use tray expectations and Windows shortcut labels.
- Linux should use GTK/libadwaita conventions and document tray/status notifier limitations.

## Native UI Policy

Clippo should keep the compact utility feel of a native desktop tool while using native controls on each operating system. The platform shells should avoid decorative gradients, blur layers, heavy shadows, custom-drawn backgrounds, and pill-shaped ornamental chrome unless there is a documented OS reason.

`scripts/check-native-ui-policy.sh` scans native shell source for common decorative or custom-drawn UI patterns. If a shell needs one of those APIs for a functional native control, document the reason here and mark the reviewed source line with `clippo-native-ui-reviewed`.

## Light And Dark Mode

Clippo follows the operating system theme through native controls and semantic system colors instead of shipping a custom color palette. The macOS SwiftUI/AppKit shell uses default controls and semantic foreground styles, the Windows shell uses default WinForms system colors, and the Linux fallback uses the desktop dialog theme until the GTK/libadwaita shell is complete.

`scripts/check-theme-policy.sh` blocks hardcoded light/dark colors and forced theme overrides in platform shell source. If a platform-specific color is required for accessibility or a native control state, document the reason here and mark the reviewed source line with `clippo-theme-reviewed`.

## High Contrast Mode

Clippo should inherit high-contrast and increased-contrast behavior from the operating system instead of implementing a separate color theme. The current source-side rule is to use native controls, semantic/system colors, and desktop-themed dialogs, while avoiding owner-drawn lists, forced themes, and contrast opt-outs unless a platform-specific exception is documented.

`scripts/check-high-contrast-source.sh` verifies that the current platform shells keep native controls that can inherit OS contrast behavior and blocks common source patterns that would bypass contrast modes. Runtime screenshots and screen-reader checks in actual high-contrast settings still remain external QA gates before release.

## Layout And Scaling

Clippo should keep popup text readable at common display scaling and text scaling settings. Source-level safeguards include stable popup dimensions, bounded shortcut columns, row preview line limits, DPI-aware Windows forms, wrapping footer controls, and Linux scale-aligned placement helpers.

`scripts/check-layout-scaling-source.sh` verifies those layout hooks stay present in platform shell source. It does not replace visual QA; screenshots at common desktop scaling settings remain required before release.

## Popup Positioning

Clippo should place the history popup using each OS shell's usable work area: macOS visible frames and active Spaces, Windows taskbar-aware working areas, and Linux work-area geometry with fractional-scale alignment. If a shell cannot place the popup precisely because of compositor restrictions, it should document the limitation and keep the popup inside the visible work area.

`scripts/check-popup-positioning-source.sh` verifies that platform placement hooks and clamp logic stay present in source. It does not replace multi-monitor QA near menu bars, taskbars, docks, panels, Spaces, and fractional-scale desktops.

## Quick Action Discoverability

Clippo should expose the core commands through native presentation instead of hiding them behind undocumented shortcuts:

| Action | macOS | Windows | Linux fallback |
| --- | --- | --- | --- |
| Copy | Popup footer, command menu, numbered shortcuts | Popup footer, `Enter`, numbered shortcuts | Zenity selection, `--copy-shortcut`, X11 `Super+1..9` |
| Paste | Popup footer, command menu, modifier-click, numbered shortcuts | Popup footer, double-click, modifier-click, shortcuts | `--paste-shortcut`, X11 `Super+Alt+1..9` |
| Paste without formatting | Actions menu, command menu, numbered shortcuts | Popup footer, modifier-click, shortcuts | `--paste-plain-shortcut`, X11 `Super+Shift+Alt+1..9` |
| Delete | Actions menu, context menu, command menu | Popup footer, shortcuts | `--delete-shortcut`, X11 `Super+Control+1..9` |
| Pin or unpin | Actions menu, context menu, command menu | Popup footer, shortcuts | `--toggle-pin-shortcut`, X11 `Super+Shift+1..9` |
| Clear unpinned | Actions menu, command menu | Tray menu, popup footer, shortcut | Desktop action, `--clear-unpinned`, X11 `Super+Control+Delete` |
| Clear all | Actions menu, command menu | Tray menu, shortcut | Desktop action, `--clear-all`, X11 `Super+Shift+Control+Delete` |
| Pause capture | Menu bar and popup footer | Tray menu, popup footer, preferences | Desktop action, `--pause-capture` |
| Ignore next copy | Menu bar and actions menu | Tray menu and popup footer | Desktop action, `--ignore-next-copy` |
| Preferences | Menu bar, actions menu, `Command+Comma` | Tray menu, popup footer, `Ctrl+Comma` | Desktop action, `--preferences`, X11 `Super+Comma` |

`scripts/check-interaction-source.sh` verifies the current source keeps keyboard and pointer-accessible action paths in each platform shell: macOS command/menu/tap handlers, Windows key and pointer handlers plus footer buttons, and Linux fallback action dialogs plus command shortcuts.

## Intentional Platform Differences

These differences are intentional unless later user research proves they hurt the core workflow:

- Clippo uses OS-native shells instead of imposing one platform's chrome on every desktop.
- Windows uses a system tray entry and `Win+Shift+C`-style shortcut labeling so it does not conflict with Windows Clipboard History on `Win+V`.
- Linux uses GTK/libadwaita conventions for the planned shell and must expose compositor limitations when Wayland restricts shortcuts, clipboard monitoring, paste automation, or status notifier behavior.
- Permission copy differs by OS: macOS explains Accessibility, Windows explains elevated-app paste boundaries, and Linux explains compositor or portal restrictions.
- Shortcut labels are rendered with platform terms: `Command` on macOS, `Windows` on Windows, and `Super` on Linux.
- Popup placement follows native screen/work-area rules.
- Packaging and install flows are platform-native: app bundle on macOS, MSIX/MSI on Windows, and AppImage/Flatpak/`.deb` targets on Linux.
