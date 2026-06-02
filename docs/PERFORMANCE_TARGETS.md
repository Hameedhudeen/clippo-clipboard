# Performance Targets

Clippo should stay lightweight enough to be comfortable as an always-running desktop utility.

## v1 Targets

- Idle memory: under 50 MB per native shell where feasible.
- Active popup memory: under 90 MB for typical histories.
- Popup open latency: under 100 ms after global shortcut.
- Clipboard ingestion latency: under 500 ms for normal text clips.
- Idle CPU: near zero when clipboard has not changed.
- Battery impact: no measurable sustained CPU wakeups from unnecessary polling.

## Measurement Plan

- Measure idle memory after five minutes with no popup open.
- Measure active popup memory with 200 mixed text entries.
- Measure popup latency from shortcut event to rendered list.
- Measure clipboard ingestion from copy event to searchable history entry.
- Record OS version, desktop environment, display scaling, and build type with every result.

## Shared Core Benchmark

Run the local shared-core benchmark with:

```sh
scripts/bench.sh
```

The benchmark currently measures adding 1,000 history items, searching 1,000 items, building popup view models for the default 200-item visible history, and JSON persistence save/load for 1,000 items.

Latest local shared-core snapshot, measured 2026-06-02 on Linux 6.14 x86_64:

| Benchmark | Result |
| --- | ---: |
| Add 1,000 history items | 5,447 us |
| Search 1,000 history items | 389 us |
| Build popup view model for 200 visible rows | 142 us |
| Build searched popup view model for 200 visible rows | 385 us |
| Save 1,000 items to JSON | 817 us |
| Load 1,000 items from JSON | 486 us |

## Large Clipboard Items

The shared core stores full clipboard content for paste, but normal popup rows carry bounded preview text only. Full preview content is fetched through a separate lazy view-model call for hover or expanded preview, and that full preview is also bounded for display. This prevents routine list rendering from cloning large rich-text or HTML payloads into every row.

## Popup Latency Evidence

The shared popup view-model benchmark is not a substitute for target OS latency testing, but it catches regressions in the CPU-side work that happens before a shell renders the popup. Before marking popup performance complete, collect shortcut-to-render timing on macOS, Windows, Linux X11, and Linux Wayland where supported, then record the measured values and environment details here.

## Clipboard Polling

Polling platforms should use the adaptive polling schedule from `clippo-platform`: recent activity resets to a short interval, while repeated no-change polls back off to reduce CPU wakeups. Platforms that expose event subscriptions should prefer events and use polling only as a fallback.
