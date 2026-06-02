# Paste Behavior

Clippo follows a predictable clipboard policy for copy and paste actions:

- Selecting "copy" writes the chosen history item to the system clipboard.
- Selecting "paste" writes the chosen history item to the system clipboard, then asks the OS to paste into the focused app.
- After a successful automatic paste, the selected history item remains the active system clipboard content.
- Clippo does not restore the previous system clipboard after paste in v1, because restoring can surprise users who expect the selected item to remain available for another paste.
- Clippo marks its own clipboard write internally so the next clipboard capture cycle does not add a duplicate history entry.
- If automatic paste is blocked, Clippo should leave the selected item on the clipboard and show a manual-paste fallback.

The shared platform controller returns user-facing fallback guidance for permission or focus failures so native shells can display the same visible message while using OS-native alerts, banners, or tray notifications.

This behavior should be the same across macOS, Windows, and Linux unless an operating system restriction forces a documented exception.
