# ADR-0007 — OntoStudio shares UI/platform with Strixonomy

## Status

Accepted — **planned post v0.30**

## Context

OntoStudio is specified as a standalone desktop IDE ([ui/ONTOSTUDIO_DESKTOP.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ONTOSTUDIO_DESKTOP.md)). Forking UI or Strixonomy would double maintenance.

## Decision

OntoStudio reuses:

- **OntoUI** (React workspaces, design tokens, WorkspaceStore)
- **Strixonomy** (CLI/LSP or embedded library)
- **Capability Provider** plugin model

OntoStudio implements **WorkspaceHost** for Electron/Tauri; it does not use VS Code APIs. Shell-specific code (windows, native menus) stays in OntoStudio repo/module only.

## Consequences

**Positive:** One UI codebase; faster desktop delivery after OntoUI matures in VS Code.

**Negative:** OntoUI must be host-agnostic before desktop work starts (v0.13–v0.30 prerequisite).

## References

- [platform/ONTOSTUDIO_REUSE.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/ONTOSTUDIO_REUSE.md)
- [adr/0001-ontoui-shared-react-platform.md](0001-ontoui-shared-react-platform.md)
