# Keyboard Shortcuts

Clippo's default shortcuts follow a compact keyboard-first interaction model and use platform-appropriate labels in native shells.

## Shared Defaults

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

`Meta` should render as Command on macOS, Windows key on Windows where appropriate, and Super on Linux where appropriate.

## Pointer Modifiers

- macOS shell source: click copies the row, and Option-click pastes the row.
- Windows shell source: Alt-click pastes the row, and Alt-Shift-click pastes without formatting.
- Linux fallback: pointer modifier paste remains a native GTK/libadwaita validation item; zenity fallback exposes selection plus command shortcuts instead.
- `scripts/check-global-shortcut-source.sh` verifies that each platform source keeps an open-history shortcut path: Carbon hotkey registration on macOS, `RegisterHotKey` on Windows, X11 xbindkeys on Linux, and the Wayland portal binding plan plus fallback guidance.

## Conflict Policy

Shortcuts must be validated before saving. Platform shells should detect conflicts when OS APIs expose that information and provide troubleshooting guidance when they cannot.
