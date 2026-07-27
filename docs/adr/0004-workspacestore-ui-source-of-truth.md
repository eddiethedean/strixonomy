# ADR-0004 — WorkspaceStore as UI source of truth

## Status

Accepted — **implemented v0.13** (Zustand store in `extension/webview-ui/src/store/`)

## Context

Per-panel React state causes duplicate LSP fetches and inconsistent entity context between inspector and graph. [ui/STATE_MANAGEMENT.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/STATE_MANAGEMENT.md) specifies a global store — **shipped in v0.13**.

## Decision

**WorkspaceStore** is the single source of truth for OntoUI global state (focus, selection, tabs, layout, reasoning, query, diagnostics, AI, plugins). Workspace components hold **local UI state only** (expansion, scroll, draft text).

Implementation: centralized store module in `extension/webview-ui/src/store/` (library TBD: Zustand or equivalent).

## Consequences

**Positive:** Predictable data flow; testable state; enables Current Focus sync.

**Negative:** Refactor every panel to read/write through store slices.

## References

- [platform/WORKSPACE_RUNTIME.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/WORKSPACE_RUNTIME.md)
- [adr/0003-current-focus-central-ux.md](0003-current-focus-central-ux.md)
