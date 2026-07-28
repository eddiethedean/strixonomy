# OntoUI architecture

> **Status:** v0.13 foundation **shipped** — WorkspaceStore, focus relay, design tokens · **ADR:** [0001-ontoui-shared-react-platform.md](../adr/0001-ontoui-shared-react-platform.md)

## Scope

**OntoUI** is the shared React UI platform for Strixonomy and future OntoStudio. Host-agnostic components and state; hosts provide file I/O, notifications, and shell integration via **WorkspaceHost**.

## Implementation (v0.13)

| Path | Role |
|------|------|
| `extension/webview-ui/` | Vite + React + TypeScript bundle |
| `extension/webview-ui/src/App.tsx` | Routes by `?panel=`; `FocusSyncBootstrap` hydrates store from host |
| `extension/webview-ui/src/context/HostContext.tsx` | `WorkspaceHost` provider |
| `extension/webview-ui/src/store/` | Zustand `WorkspaceStore`, event bus, types |
| `extension/webview-ui/src/host/` | `WorkspaceHost` interface + VS Code implementation |
| `extension/webview-ui/src/workspaces/registry.ts` | Workspace registration |
| `extension/webview-ui/src/tokens/cssVars.ts` | Design tokens → CSS variables |
| `extension/webview-ui/src/components/primitives/` | Shared UI primitives |
| `extension/webview-ui/src/components/SchemaBrowser.tsx` | SQL schema sidebar |
| `extension/webview-ui/src/messages.ts` | Typed postMessage protocol (`focusState`, `reasoningState`, …) |
| `extension/src/focus/focusRelay.ts` | Extension-host relay (webviews cannot share in-memory Zustand) |
| `extension/src/webview/` | Webview lifecycle, CSP, message bridge |

**Store-integrated panels:** Entity Inspector, Graph, Query Workbench, Refactor Preview.

**Still per-panel local state:** Manchester editor, Semantic Diff, Manage Imports (full migration → v0.30).

## Architecture

```text
Extension host (FocusRelayService)
    ↕ focusState / reasoningState / setFocus
WorkspaceHost (VS Code | future OntoStudio)
    ↕ host adapter API
OntoUI shell
├── WorkspaceStore (global state)
├── WorkspaceRegistry → Entity | Graph | Query | … workspaces
├── Design tokens + component library
└── LSP client facade (typed, shared)
```

## WorkspaceHost interface

```ts
interface WorkspaceHost {
  postToCore(message: HostToLspMessage): Promise<unknown>
  showNotification(level: "info" | "warn" | "error", message: string): void
  getTheme(): ThemeTokens
  onMessage(handler: (data: HostMessage) => void): () => void
}
```

VS Code implementation lives in the extension host; OntoStudio will implement the same interface without `vscode.*` APIs.

## Panel → workspace mapping

| Panel (`?panel=`) | Workspace |
|-------------------|-----------|
| `inspector` | Entity Workspace |
| `graph` | Graph Workspace |
| `queryWorkbench` | Query Workspace |
| `manchesterEditor` | Entity Workspace (axiom editor) |
| `refactorPreview` | Refactoring Workspace |
| `semanticDiff` | Diff Workspace |
| `imports` | Import Workspace |

## Build and test

```bash
cd extension/webview-ui && npm test    # Vitest (161+ tests)
cd extension && npm test               # Extension unit tests
cd extension && npm run build:webview
```

## Links

- [WORKSPACE_RUNTIME.md](WORKSPACE_RUNTIME.md)
- [design/adr/0017-react-webview-ui.md](../design/adr/0017-react-webview-ui.md)
- [ui/COMPONENT_LIBRARY.md](../ui/COMPONENT_LIBRARY.md)
- [migration/v0.13.md](../migration/v0.13.md)

## Evolution

Extends [design/adr/0017](../design/adr/0017-react-webview-ui.md) (React webviews) to a named **OntoUI** platform. UX detail: [ui/COMPONENT_LIBRARY.md](../ui/COMPONENT_LIBRARY.md).
