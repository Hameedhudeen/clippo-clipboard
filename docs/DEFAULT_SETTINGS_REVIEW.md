# Default Settings Review

Reviewed on 2026-06-01 for the v1 planning baseline.

The default settings should keep Clippo useful immediately while avoiding surprising startup behavior, excessive memory use, or hidden network activity.

| Setting | Default | Review decision |
| --- | --- | --- |
| History limit | 200 items | Large enough for daily use, small enough for predictable memory and storage. |
| Maximum item size | 5 MB | Prevents very large clipboard payloads from slowing startup, search, and persistence. |
| Clipboard check interval | 500 ms | Reasonable fallback for polling platforms before adaptive scheduling is applied. |
| Paste automatically | Enabled | Matches the expected core workflow, with permission fallback required. |
| Launch at login | Disabled | Avoids starting background capture without an explicit user choice. |
| Footer | Shown | Keeps core actions discoverable while the app is new. |
| Tray or menu icon | Shown | Gives users a native way to open, pause, troubleshoot, and quit Clippo. |
| Appearance | System | Follows the operating system by default. |
| Ignored clipboard types | Platform privacy defaults plus user list | Keeps sensitive and transient formats out of history where metadata is available. |
| Ignored applications | Empty user list | Users can add apps after platform support exposes source application metadata. |
| Ignored content patterns | Empty user list | Avoids surprising filtering until users opt in. |

## Review Outcome

The defaults are acceptable for beta entry once native shells expose first-run permission explanations and preferences. Before v1, this review should be repeated with real memory, startup, and paste-automation measurements from each supported OS.
