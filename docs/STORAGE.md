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

- macOS: Application Support directory.
- Windows: AppData directory.
- Linux: XDG data/config directories.

Backup procedure:

1. Quit Clippo.
2. Copy the Clippo JSON history/settings store from the platform data directory.
3. Store the backup somewhere private because it can contain clipboard history.

Restore procedure:

1. Quit Clippo.
2. Replace the Clippo JSON store with the backup copy.
3. Start Clippo and let schema migration run if needed.

Exact platform paths must be finalized when native shells choose their app data directories.

## Clearing Stored Data

The shared core exposes clear-unpinned, clear-all, and emergency clear-history-and-quit flows. Platform shells must wire these actions to their native menu/tray UI before v1.

Until platform shells exist, local test data can be removed by deleting the JSON store file used by the shell or test harness.
