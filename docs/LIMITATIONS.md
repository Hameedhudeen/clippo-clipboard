# Known Limitations To Validate

Clippo is pre-alpha. These limitations must be validated and rewritten with exact behavior before v1.

## Linux Wayland

Global shortcuts, clipboard monitoring, and paste automation can depend on desktop portals and compositor support. Clippo should document unsupported compositor behavior instead of silently failing.

## Windows Elevated Apps

Paste automation into elevated applications can be restricted by Windows security boundaries. Clippo should detect or document this behavior before v1.

## macOS Permissions

Automatic paste requires macOS Accessibility permission. Future macOS versions may require additional permissions for related automation.

## Cross-Platform Parity

Clippo targets equivalent workflows, not pixel-perfect Maccy UI on every OS.
