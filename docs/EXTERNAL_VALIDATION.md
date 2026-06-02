# External Validation Matrix

This matrix tracks the remaining tasks that cannot be honestly marked complete from the current Linux development workspace. Use it when testing on target operating systems, packaging hosts, or GitHub Releases.

## Evidence Rules

- Keep a screenshot, terminal log, package artifact, checksum file, or issue link for every checked item.
- Use `docs/validation/TEMPLATE.md` for small validation logs that should be committed to the repository.
- Use the `Validation evidence` issue template when evidence is too large or too environment-specific to commit directly.
- Update public roadmap or validation status only after the evidence exists in the repository, in a release artifact, or in a linked GitHub issue or release.
- Do not mark a cross-platform parity task complete from one OS result.
- Run `scripts/check-external-validation.sh` after changing this matrix to confirm every validation item still has exact coverage below.
- Run `scripts/export-validation-issues.sh` to generate local GitHub issue body drafts for every unchecked validation gate.
- Run `scripts/export-validation-runbook.sh` to generate a grouped target-host validation runbook with exact evidence collection commands.
- Run `scripts/collect-validation-evidence.sh --task "<exact unchecked task>"` on a target host to create a filled evidence log under `docs/validation/` with environment details and Clippo command output.
- Run `scripts/check-validation-evidence.sh` before checking off a task from a committed evidence log.

## Exact Unchecked Task Coverage

These lines are the public source of truth for remaining external validation gates. Local maintainers may mirror them in a private implementation checklist.

- Add screenshots or animated GIFs for each supported OS once UI exists.
- Add release downloads section linking GitHub Releases when artifacts exist.
- Add badges only after CI, releases, and license metadata are real.
- Verify all README links, images, badges, and anchors render correctly on GitHub.
- Verify behavior with macOS Universal Clipboard.
- Package as signed app bundle for local testing.
- Package as MSIX or MSI for local testing.
- Implement global shortcut support for Wayland through available portals where possible.
- Package as AppImage for broad testing.
- Package as Flatpak if portal integration is viable.
- Keep popup lightweight and fast to open.
- Verify Clippo's popup before v0.5 for search placement, list density, row spacing, shortcut labels, footer behavior, and visual hierarchy.
- Verify each OS shell keeps the core workflow while using native system chrome, fonts, colors, focus rings, and accessibility conventions.
- Run target-host UI reviews before every beta release.
- Review every platform shell against the Clippo UI/UX reference before marking v0.5 parity complete.
- Support screen reader basics for the history list and preferences.
- Support high contrast mode where the OS exposes it.
- Verify no text overlaps at common desktop scaling settings.
- Verify popup positioning near menu bar, taskbar, dock, and multiple displays.
- Open history with global shortcut.
- Select and paste with modifier plus Enter.
- Select and paste with modifier plus click.
- Add screenshots for each OS.
- Add badges only after CI and releases exist.
- Add high-quality README screenshots or GIFs.
- Add demo video or animated walkthrough before portfolio launch.
- Add portfolio case study after first usable release.
- v0.3 one platform shell is usable end-to-end.
- v0.4 all three platform shells can open, search, copy, and paste history items.
- v0.5 core clipboard workflow parity is implemented on all supported OSes.
- v0.6 privacy, ignore rules, and settings are implemented.
- v0.7 packaging exists for all supported OSes.
- v0.8 README landing page, documentation, and parity matrix are complete.
- v0.9 beta release is tested by external users.
- v1.0 release has signed or checksummed artifacts, install docs, known limitations, and no unchecked v1 parity tasks.

## README And Portfolio Assets

| Task | Required environment | Evidence needed |
| --- | --- | --- |
| Add screenshots or animated GIFs for each supported OS | macOS, Windows, Linux X11 or Wayland | Current screenshots or GIFs committed under a docs or assets path and referenced from `README.md` |
| Add release downloads section | Published GitHub Release | README links to actual release artifacts and checksums |
| Add badges | Active CI and at least one published release | Badges point to real workflow/license/release URLs |
| Verify README renders on GitHub | GitHub repository page | Rendered README review with working images, anchors, and tables |
| Add demo video or animated walkthrough | First usable release | Linked demo asset showing open, search, copy, paste, pin, delete, and clear |
| Add portfolio case study | First usable release | Case study with scope, architecture, screenshots, tradeoffs, and maintenance plan |

## macOS

| Task | Required environment | Evidence needed |
| --- | --- | --- |
| Verify Universal Clipboard behavior | macOS with another Apple device | QA note covering local copies, Universal Clipboard arrivals, ignore rules, and duplicates |
| Package signed app bundle | macOS with Xcode and signing identity | `Clippo.app`, signing output, and notarization or documented local-signing result |
| Open history with global shortcut | macOS target host | Screen recording or QA log showing `Shift-Command-C` opens history |
| Select and paste with modifier plus Enter | macOS target host with Accessibility permission | QA log showing `Option-Enter` pastes into another app |

## Windows

| Task | Required environment | Evidence needed |
| --- | --- | --- |
| Package as MSIX or MSI | Windows Developer PowerShell with .NET desktop workload and Windows SDK | `.msix` or `.msi` artifact plus install result |
| Open history with global shortcut | Windows target host | QA log showing `Win+Shift+C` opens history |
| Select and paste with modifier plus Enter | Windows target host | QA log showing `Alt+Enter` pastes into another app |
| Select and paste with modifier plus click | macOS, Windows, and native Linux target hosts | QA log showing Option-click on macOS, Alt-click on Windows, and the GTK/libadwaita native equivalent on Linux |

## Linux

| Task | Required environment | Evidence needed |
| --- | --- | --- |
| Wayland global shortcuts through portals | GNOME or KDE Wayland with `org.freedesktop.portal.GlobalShortcuts` | QA log showing portal availability, bind result, activation signal, and history popup opening |
| Package as AppImage | Linux host with `appimagetool` | `.AppImage` artifact, launch result, and checksum |
| Package as Flatpak | Linux host with `flatpak-builder` and portal testing setup | Flatpak build directory or bundle, install result, portal behavior notes, and checksum |
| Keep popup lightweight and fast to open | Target Linux desktop plus macOS and Windows equivalents | Measured popup latency under the target in `docs/PERFORMANCE_TARGETS.md` |

## Cross-Platform UI And Accessibility

| Task | Required environment | Evidence needed |
| --- | --- | --- |
| Compare popup against Clippo UI specification | macOS reference plus each Clippo shell | Side-by-side screenshots covering search placement, density, spacing, shortcut labels, footer, and hierarchy |
| Verify native system chrome and conventions | macOS, Windows, GNOME/KDE | QA notes for fonts, colors, focus rings, title/menu/tray behavior, and accessibility conventions |
| Screen reader basics | All supported OSes | Screen reader announces history list, rows, selected state, and preferences controls |
| High contrast mode | Windows and Linux desktops, macOS contrast setting where applicable | Screenshots and QA notes at high contrast/increased contrast settings |
| Reduced motion | All supported OSes if animations are introduced | QA note confirming OS setting is respected or no animations exist |
| Text overlap and scaling | All supported OSes | Screenshots at common display scaling/text scaling values |
| Popup positioning | Multi-monitor target hosts | QA notes for menu bar, taskbar, dock, multiple displays, Spaces, and fractional scaling |

## Release Milestones

| Milestone | Required evidence |
| --- | --- |
| v0.3 | One platform shell installed and used end-to-end for open, search, copy, paste, pin, delete, clear, pause, ignore, and preferences |
| v0.4 | macOS, Windows, and Linux each open, search, copy, and paste history items |
| v0.5 | core clipboard workflow parity verified on all supported OSes |
| v0.6 | Privacy, ignore rules, and settings verified on all supported OSes |
| v0.7 | macOS, Windows, and Linux package artifacts exist with checksums |
| v0.8 | README, screenshots, docs, and parity matrix reflect actual tested state |
| v0.9 | External beta feedback recorded and triaged |
| v1.0 | Signed or checksummed artifacts, install docs, known limitations, and no unchecked v1 parity tasks |
