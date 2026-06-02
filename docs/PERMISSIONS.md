# Permission Explanations

Clippo must ask for desktop permissions only when a feature needs them, and each native shell must explain the reason before sending the user to the operating system settings screen.

## Clipboard Access

Clippo reads clipboard changes so it can build local history. Clipboard history is stored locally and is filtered by ignored clipboard types, ignored applications, and ignored content patterns where the platform exposes enough metadata.

If clipboard access is unavailable, Clippo should keep the app open and explain that history capture is paused or limited instead of failing silently.

## Paste Automation

Automatic paste writes the selected history item back to the system clipboard, returns focus to the target application, and sends the platform paste command. This needs OS automation support because desktop apps do not expose a universal paste API.

If paste automation is blocked, Clippo should still copy the selected item to the clipboard and show a visible fallback message telling the user to paste manually.

## macOS

Automatic paste requires Accessibility permission. Clippo needs this permission to send the paste shortcut to the active app after the user chooses a history item.

Clippo should not request Screen Recording or Input Monitoring unless a future macOS implementation proves those permissions are required for a specific feature.

## Windows

Windows can restrict paste automation into elevated applications. Clippo should explain that copying still works, but automatic paste may fail across User Account Control boundaries.

## Linux

Linux behavior depends on the session type and desktop environment. X11 generally allows broader clipboard and shortcut integration, while Wayland may require desktop portals or compositor-specific support.

When a compositor blocks global shortcuts, clipboard monitoring, or paste automation, Clippo should show the limitation and link to the Wayland troubleshooting guide.

## Required UI Behavior

- Explain the exact permission before requesting it.
- State that clipboard history stays local by default.
- Provide a manual fallback when automatic paste is unavailable.
- Avoid repeated permission prompts after the user declines.
- Keep permission help reachable from preferences and troubleshooting.
