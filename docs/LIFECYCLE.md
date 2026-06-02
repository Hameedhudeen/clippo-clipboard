# App Lifecycle Policy

Clippo should behave like a single always-available desktop utility.

## Single Instance

Only one Clippo process should own clipboard capture at a time. A second launch should focus or open the existing instance instead of starting a competing monitor.

## Process Model

For v1, each platform should use one native app process with a shared Rust core. A separate background helper should only be added if an OS requires it.

## Lifecycle Events

The shared core models these lifecycle events:

- First launch.
- Normal launch.
- Second launch.
- Sleep.
- Wake.
- Menu or tray quit request.
- Shutdown request.
- Crash recovery.
- Update restart request.

Lifecycle transitions expose side effects that native shells must honor:

- Start clipboard monitoring after first launch, normal launch, wake, and crash recovery.
- Stop clipboard monitoring during sleep, shutdown, and update restart.
- Save history and settings before menu/tray quit, normal shutdown, and update restart.
- Relaunch after update restart only when the update flow requests it.

## First-Run Onboarding

The shared core exposes an onboarding view model with required steps for clipboard access, global shortcut confirmation, and privacy-default review. Paste automation is shown as an optional permission step when automatic paste is enabled.

Native shells must render this view model with platform-specific permission actions and localized strings from `i18n/`.

## Recovery

History and settings should be saved through crash-safe persistence. After a crash or forced quit, Clippo should restart clipboard monitoring and preserve existing history unless the persisted data is corrupt.
