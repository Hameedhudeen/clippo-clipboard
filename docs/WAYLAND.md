# Linux Wayland Notes

Wayland support is a v1 target, but some clipboard-manager behavior depends on compositor and portal support.

## Risk Areas

- Global shortcuts.
- Clipboard monitoring.
- Automatic paste.
- Tray or status notifier behavior.
- Popup placement across multiple monitors.

## Policy

Clippo should use XDG portals where available. When a compositor blocks a workflow, Clippo should show a clear fallback or limitation instead of failing silently.

The Linux shell scaffold already prints fallbacks for restricted global shortcuts, clipboard monitoring, and paste automation:

- Configure a desktop shortcut that runs `clippo-linux --show-history` when the compositor or portal does not allow app-managed shortcuts.
- Keep manual copy/paste available when compositor policy blocks clipboard monitoring.
- Copy the selected item and ask the user to paste manually when compositor policy blocks paste automation.

For X11 sessions, `clippo-linux --install-x11-shortcut` writes a managed xbindkeys block for `Super+Shift+C`.

For Wayland sessions, `clippo-linux --wayland-shortcuts-status` probes the session bus for `org.freedesktop.portal.GlobalShortcuts` on `org.freedesktop.portal.Desktop` at `/org/freedesktop/portal/desktop`. The [XDG Desktop Portal Global Shortcuts API](https://flatpak.github.io/xdg-desktop-portal/docs/doc-org.freedesktop.portal.GlobalShortcuts.html) can create shortcut sessions, bind shortcuts, and emit activation signals.

`clippo-linux --wayland-shortcuts-plan` prints the current portal command plan for target-host testing:

- `CreateSession` with stable Clippo handle tokens.
- `BindShortcuts` for `open-history` with preferred trigger `Super+Shift+C`.
- `ListShortcuts` to inspect the session.
- A `gdbus monitor` command for `Activated` signals.

`clippo-linux --wayland-shortcuts-daemon` is the runtime portal path for the Linux shell. It creates a long-running GlobalShortcuts session, requests the `open-history` shortcut with preferred trigger `Super+Shift+C`, listens for `Activated`, and dispatches `clippo-linux --show-history`.

GNOME/KDE evidence is still required before the Wayland shortcut validation task can be marked complete, because compositor support for the portal differs by desktop and distribution.

## Clipboard Backend

The current Linux shell selects `wl-paste` and `wl-copy` for Wayland sessions and reports no captured text when those tools are unavailable or blocked. Runtime validation still needs GNOME Wayland and KDE Wayland because compositor policy can differ.

## Paste Automation

The current Linux shell intentionally does not synthesize paste keystrokes on Wayland. It writes the selected text to the Wayland clipboard backend when available, then falls back to manual paste guidance. This avoids claiming support for compositor-restricted input simulation before a portal-backed implementation is validated.

## Validation Targets

- Ubuntu 24.04 GNOME Wayland.
- Ubuntu 24.04 GNOME X11.
- A current KDE Plasma Wayland environment.
