# Storage

Clippo's initial local storage format is JSON with explicit schema versioning. The implementation lives in `clippo-persistence`.

## v1 Storage Requirements

- Store history locally.
- Store settings locally.
- Include a schema version.
- Support migrations.
- Use crash-safe writes through a temporary file plus rename.
- Support retention limits.
- Report corrupt files clearly.

## Backup And Restore

Expected locations:

- macOS shell: `~/Library/Application Support/Clippo/history.json`.
- Windows shell: `%LOCALAPPDATA%\Clippo\history.json`.
- Linux fallback shell: `$XDG_STATE_HOME/clippo/linux-history`, or `~/.local/state/clippo/linux-history` when `XDG_STATE_HOME` is not set.

Backup procedure:

1. Quit Clippo.
2. Copy the Clippo JSON history/settings store from the platform data directory.
3. Store the backup somewhere private because it can contain clipboard history.

Restore procedure:

1. Quit Clippo.
2. Replace the Clippo JSON store with the backup copy.
3. Start Clippo and let schema migration run if needed.

The shared `clippo-persistence` JSON schema remains the intended v1 storage contract once the native shells are fully wired through the shared core/FFI boundary.

## Clearing Stored Data

The shared core exposes clear-unpinned, clear-all, and emergency clear-history-and-quit flows. Platform shells must wire these actions to their native menu/tray UI before v1.

Until platform shells exist, local test data can be removed by deleting the JSON store file used by the shell or test harness.
