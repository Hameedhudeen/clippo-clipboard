# Migration From Maccy

Maccy import is not a v1 feature.

## Current Finding

Maccy 2.6.1 stores history with SwiftData at:

```text
Application Support/Maccy/Storage.sqlite
```

Importing this data would require a macOS-only migration tool that understands Maccy's SwiftData schema and pasteboard content model.

## v1 Guidance

Clippo should document that users moving from Maccy start with a fresh Clippo history. A future migration tool can be considered after Clippo v1 is stable.
