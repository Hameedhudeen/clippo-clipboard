# Linux User Guide

Clippo for Linux has a pre-alpha fallback shell today. The final native GTK4/libadwaita popup is still pending, so this guide separates the current fallback workflow from the intended v1 native workflow.

## Launch

Current packages start a resident background monitor from the desktop launcher or `clippo-linux --background`. The final shell should integrate through status notifier, tray, or a documented desktop-shell fallback where the environment supports it.

## Background And Startup

The background monitor reads the supported Linux clipboard backend, writes history to the XDG state directory, and keeps running until stopped with `clippo-linux --quit` or the Quit Clippo desktop action. On first launch, the fallback shell asks whether Clippo should start automatically after sign-in when Zenity is available; if the dialog backend is unavailable, it points users to `clippo-linux --enable-autostart`. The startup entry uses XDG Autostart with background mode enabled.

## Open Clipboard History

Use the desktop action, `clippo-linux --show-history`, or an installed X11 shortcut to open history. The current fallback opens a zenity search entry first, then a filtered zenity list. If zenity is unavailable, Clippo falls back to a desktop notification instead of silently exiting.

## Select And Paste

- In the fallback dialog, type a search term, press Enter, select a result, then choose Copy, Paste, Paste Without Formatting, Show Full Text, Pin or Unpin, or Delete.
- X11 can use command shortcuts for copy, paste, plain paste, pin/unpin, and delete where helper tools are installed.
- Wayland sessions may fall back to copying the selected text and asking the user to paste manually, depending on portals and compositor policy.

## Manage History

- Pin or unpin frequent items.
- Delete one item.
- Clear unpinned history.
- Clear all history including pinned items.
- Pause capture.
- Ignore only the next copy.

## Wayland

Wayland behavior depends on portals and compositor support. Clippo should document limitations instead of silently failing.

## Troubleshooting

See `docs/WAYLAND.md`, `docs/TROUBLESHOOTING.md`, and `docs/LIMITATIONS.md`.

## Validation Status

The Linux fallback keeps the workflow testable and now has a resident background monitor plus first-run autostart prompt, but it is not the final native UI. The GTK4/libadwaita popup, status notifier integration, GNOME/KDE visual validation, screen reader checks, fractional scaling checks, and Wayland shortcut activation evidence are still required before Linux UI/UX parity can be claimed.
