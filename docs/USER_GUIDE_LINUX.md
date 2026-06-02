# Linux User Guide

Clippo for Linux is not implemented yet. This guide defines the intended v1 user workflow.

## Launch

Clippo should integrate with the desktop shell through status notifier, tray, or a documented fallback.

## Open Clipboard History

Use the global shortcut or shell integration to open the compact history popup. Search should be focused immediately.

## Select And Paste

- Press Enter or click an item to select it.
- Use the platform paste shortcut where the desktop environment allows automation.
- Use paste-without-formatting to paste plain text from rich content.

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
