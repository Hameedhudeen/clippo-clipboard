# QA Checklists

Remaining OS, packaging, release, screenshot, and accessibility gates are mapped to required environments and evidence in [EXTERNAL_VALIDATION.md](EXTERNAL_VALIDATION.md).

## Maccy Parity

- Open history with global shortcut.
- Type immediately to search.
- Select an item with keyboard.
- Select an item with pointer.
- Select by numbered shortcut.
- Paste normally.
- Paste without formatting.
- Delete selected item.
- Pin and unpin selected item.
- Confirm pinned item stays above regular history.
- Clear unpinned items.
- Clear all items.
- Pause capture.
- Ignore only the next copy.
- Open preferences.
- Confirm ignored confidential types are not captured.

## Release QA Per OS

- Fresh install.
- Upgrade from previous release if one exists.
- Launch at login setting.
- Global shortcut registration.
- Clipboard monitoring.
- Automatic paste.
- Plain-text paste.
- Settings persistence.
- Clear history.
- Package uninstall.

## Accessibility QA

- Keyboard-only navigation.
- Visible focus state.
- Screen reader labels for history list.
- Screen reader labels for preferences.
- High contrast mode.
- Reduced motion behavior if animations exist.
- Text scaling and display scaling.

## README Landing Page Review

- Status is honest.
- Install instructions match available artifacts.
- Screenshots match current UI.
- Platform support table matches tested platforms.
- Privacy claims match implementation.
- Maccy attribution is present.
- Links resolve.
- No unfinished feature is described as complete.
