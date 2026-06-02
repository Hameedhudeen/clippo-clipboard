# ADR 0002: Maccy 2.6.1 Parity Baseline

## Status

Accepted.

## Context

Clippo is inspired by Maccy's lightweight, keyboard-first macOS clipboard workflow. Because Maccy is macOS-specific, Clippo will implement OS-equivalent behavior rather than directly porting every AppKit implementation detail.

## Decision

Use Maccy 2.6.1 as the initial v1 parity baseline. Keep a local source copy at `references/maccy/2.6.1/source` and re-audit upstream Maccy before Clippo v1.

## Consequences

- Contributors have a stable reference while implementing v1.
- Newer upstream Maccy changes will not silently expand v1 scope.
- Any unavoidable Windows or Linux differences must be documented in the parity matrix.
