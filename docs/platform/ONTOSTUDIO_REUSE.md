# OntoStudio reuse strategy

> **Status:** Planned (post v1.0) · **ADR:** [0007-ontostudio-shares-platform.md](../adr/0007-ontostudio-shares-platform.md)

## Scope

OntoStudio is a **future standalone desktop app**. It reuses OntoUI and Strixonomy — it does not fork React components or duplicate semantic logic.

## Reuse matrix

| Asset | Reuse strategy |
|-------|----------------|
| **OntoUI** | Same npm package / monorepo workspace; WorkspaceHost = Electron/Tauri adapter |
| **Strixonomy** | Bundled `strixonomy-lsp` or in-process library (TBD) |
| **Design tokens** | Shared [DESIGN_TOKENS.json](../ui/DESIGN_TOKENS.json) |
| **Workspaces** | Same Entity, Graph, Query, Reasoning workspaces |
| **Capability Providers** | Same plugin API; desktop marketplace |

## Does not reuse

- VS Code extension host (`extension/src/`)
- VS Code tree views (native OntoStudio explorer)
- VS Code command palette (native menu + palette)

## Shell responsibilities (OntoStudio only)

- Multi-window layouts
- Native file dialogs
- Offline-first packaging
- Enterprise deployment (MSI, notarization)
- Local AI model integration (optional)

## Implementation order

1. OntoUI host-agnostic ([ONTOUI.md](ONTOUI.md)) — **prerequisite**
2. WorkspaceStore + workspaces in Strixonomy — **v0.13–v1.0**
3. Extract OntoUI package if needed for desktop bundling
4. OntoStudio shell prototype — **post v1.0**

## Links

- [ui/ONTOSTUDIO_DESKTOP.md](../ui/ONTOSTUDIO_DESKTOP.md) (UX vision)
- [OVERVIEW.md](OVERVIEW.md)

## Evolution

Desktop vision in [ui/ONTOSTUDIO_DESKTOP.md](../ui/ONTOSTUDIO_DESKTOP.md); this doc defines **reuse boundaries** only.
