# Clippo Linux Shell

This folder is the planned native Linux shell for Clippo.

## Target

- GTK4/libadwaita.
- Status notifier or desktop-shell integration where available.
- core compact history popup adapted to Linux desktop conventions.
- X11 support where available.
- Wayland support through portals where available.
- Rust shared core through a narrow FFI boundary.

## Current Status

The current Rust shell scaffold includes session detection, explicit Wayland fallback guidance for restricted global shortcuts, clipboard monitoring, and paste automation, XDG autostart `.desktop` file registration helpers, `notify-send` desktop notification plumbing, clipboard backend plumbing for X11 and Wayland, X11 paste automation through `xdotool`, X11 shortcut configuration through xbindkeys, desktop launcher actions for common menu commands, persisted Linux shell state for pause/ignore-next actions, persisted Linux shell history with pinned shortcut labels, a zenity-backed GTK dialog fallback for history and preferences, monitor-aware popup placement helpers with fractional-scale alignment, and desktop-environment capability detection for GNOME, KDE Plasma, XFCE, and unknown shells.

The native GTK4/libadwaita popup, status notifier integration, and paste automation still require Linux desktop integration work and runtime validation on GNOME Wayland, GNOME X11, and KDE. Wayland GlobalShortcuts portal support now has a real daemon path, but it still needs activation evidence from desktops that expose the portal.

## Display Handling

The shell has a tested popup placement helper that clamps the popup inside the active monitor work area and aligns logical dimensions through a scale factor. The GTK/libadwaita window still needs to call this helper with real monitor geometry from GNOME and KDE before visual QA can be marked complete.

## Clipboard Backends

- X11 clipboard reads/writes use `xclip` with the `clipboard` selection.
- Wayland clipboard reads/writes use `wl-paste` and `wl-copy` from `wl-clipboard`.
- `clippo-linux --clipboard-smoke` attempts one text clipboard read through the detected session backend.
- `clippo-linux --clipboard-write=<text>` writes text through the detected session backend.
- If the compositor or desktop policy blocks access, Clippo must keep manual copy/paste guidance visible rather than failing silently.

## Paste Automation

- X11 paste automation uses `xdotool key --clearmodifiers ctrl+v`.
- X11 plain-text paste uses `xdotool key --clearmodifiers ctrl+shift+v` after writing plain text to the clipboard.
- Wayland sessions copy the selected text and fall back to manual paste guidance unless a validated portal-backed paste path is added later.
- `clippo-linux --paste-text=<text>` writes text to the detected clipboard backend and pastes automatically where supported.
- `clippo-linux --paste-plain-text=<text>` writes plain text and uses the platform plain-text paste shortcut where supported.

## Fallback Dialogs

Until the full GTK4/libadwaita shell is implemented, `clippo-linux --show-history` opens a focused zenity search entry first, filters the persisted Linux shell history, then opens a compact zenity list dialog. The fallback dialog supports keyboard navigation through the native list behavior: type the search term, press Enter, move through results with arrow keys, and press Enter to choose an action for the selected item. The action dialog exposes copy, paste, paste without formatting, show full text, pin or unpin, and delete. Empty search returns the full history. `clippo-linux --preferences` opens a zenity info dialog with the current Linux shell state.

## Local Shell Commands

- `clippo-linux --enable-autostart` writes `clippo.desktop` under the XDG autostart directory.
- `clippo-linux --disable-autostart` removes Clippo from XDG autostart.
- `clippo-linux --install-x11-shortcut` writes a managed `~/.xbindkeysrc` block for `Super+Shift+C` to run `clippo-linux --show-history`, `Super+Comma` to run `clippo-linux --preferences`, `Super+Control+Delete` to clear unpinned history, `Super+Shift+Control+Delete` to clear all history, `Super+1..9` to copy, `Super+Alt+1..9` to paste, `Super+Shift+Alt+1..9` to paste plain text, `Super+Shift+1..9` to pin or unpin, and `Super+Control+1..9` to delete.
- `clippo-linux --show-history` is the command target used by desktop actions and global shortcuts.
- `clippo-linux --copy-shortcut=<number>`, `--paste-shortcut=<number>`, `--paste-plain-shortcut=<number>`, `--toggle-pin-shortcut=<number>`, and `--delete-shortcut=<number>` resolve persisted pinned or visible row shortcuts for fallback keyboard workflows.
- `clippo-linux --wayland-shortcuts-status` probes for the XDG Desktop Portal GlobalShortcuts interface and reports whether the native GTK/libadwaita shell can later use a portal session for Wayland shortcuts.
- `clippo-linux --wayland-shortcuts-plan` prints the `CreateSession`, `BindShortcuts`, `ListShortcuts`, and signal-monitoring commands for target-host Wayland portal testing.
- `clippo-linux --wayland-shortcuts-daemon` owns a GlobalShortcuts portal session, binds the `open-history` shortcut, listens for `Activated`, and dispatches `clippo-linux --show-history`. Target-host GNOME/KDE activation evidence is still needed before the Wayland shortcut task can be closed.
- `clippo-linux --pause-capture` toggles persisted clipboard capture pause state under the XDG state directory.
- `clippo-linux --ignore-next-copy` marks the next clipboard capture as ignored and clears itself after it is consumed.
- `clippo-linux --preferences` is the desktop action target for preferences while the GTK preferences window is still pending.
- `clippo-linux --delete-text=<text>` deletes a matching fallback history item.
- `clippo-linux --toggle-pin=<text>` pins or unpins a matching fallback history item.
- `clippo-linux --clear-unpinned` clears unpinned fallback history.
- `clippo-linux --clear-all` clears all fallback history.
- `clippo-linux --notify-smoke` sends a desktop notification when `notify-send` is available.

## Desktop Shell Integration

Linux packages install desktop actions for Open History, Pause Capture, Ignore Next Copy, Clear Unpinned, Clear All, and Preferences. These provide launcher-menu integration on desktops that expose `.desktop` actions; status notifier tray behavior is still pending.

Clippo detects the current desktop from `XDG_CURRENT_DESKTOP` and `DESKTOP_SESSION`:

- GNOME: desktop actions are the default integration; status notifier may require an extension.
- KDE Plasma: use StatusNotifierItem when the native tray shell is implemented.
- XFCE: use StatusNotifierItem or legacy tray support when available.
- Unknown desktops: expose desktop actions and treat tray support as best effort.

## State

Linux shell state is stored as a small key/value file at `$XDG_STATE_HOME/clippo/linux-state`, or `~/.local/state/clippo/linux-state` when `XDG_STATE_HOME` is not set.

Linux shell history is stored at `$XDG_STATE_HOME/clippo/linux-history`, or `~/.local/state/clippo/linux-history` when `XDG_STATE_HOME` is not set. The fallback history dialog reads this file.

## Packaging

Run `scripts/package-linux-deb.sh` from the repository root to build a local Debian package at `dist/linux/clippo_<version>_amd64.deb`.

Run `scripts/package-linux-appimage.sh` to prepare an AppDir. It creates an AppImage when `appimagetool` is installed; otherwise it leaves the AppDir in `dist/linux/appimage/` for inspection.

The Flatpak manifest is in `packaging/flatpak/app.clippo.Clippo.yml` and targets the current GNOME runtime branch recorded there. Run `scripts/package-linux-flatpak.sh` on a host with `flatpak-builder` to validate the manifest and build the Flatpak directory. Export the result with `flatpak-builder --repo=dist/linux/flatpak-repo` when a locally installable Flatpak repo is needed for testing.
