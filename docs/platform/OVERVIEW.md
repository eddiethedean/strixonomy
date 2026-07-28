# Platform overview

> **v0.20 foundation shipped** (workspace runtime + Turtle patch hardening on top of the v0.13–v0.19 OntoUI / plugin / shell / transaction stack). For evaluator-facing capabilities, read [What ships today](../SHIPPED.md) first.
>
> **This section is design/implementation planning.** Items below may be shipped, partial, or planned — always cross-check [SHIPPED.md](../SHIPPED.md).
>
> **Status:** OntoUI runtime + focus relay **shipped v0.13**; plugin host MVP **shipped v0.14**; plugin permissions/views **shipped v0.15**; preferences/context actions **shipped v0.16**; Protégé-shell menus/dialogs/perspectives **shipped v0.17**; reasoner cancel, stale explanations, layout reopen-with-context **shipped v0.18** · **Terms:** [Glossary](../glossary.md)

## Scope

This document is the **implementation architecture hub** for the Ontologos platform: Strixonomy, OntoUI, Strixonomy, and future OntoStudio. User-facing ecosystem summary: [architecture.md](../architecture.md).

## Layer model

```text
Applications (hosts)
├── Strixonomy (VS Code)          — Implemented (WorkspaceStore + focus relay v0.13)
├── OntoStudio (desktop)        — Planned
└── Future web client           — Proposed

OntoUI (shared React platform)  — v0.13 foundation shipped
├── Workspace runtime (Zustand store, event bus, registry)
├── Design tokens + shared primitives
├── Workspace surfaces (Entity, Graph, Query, …)
└── Host adapter (WorkspaceHost) + extension-host focus relay

Strixonomy (semantic engine)      — Implemented
├── Index, query, diagnostics (configurable rules)
├── Reasoning (Ontologos)
├── Refactoring, diff (--pr-summary), docs export
├── LSP + CLI (semantic tokens, listSqlSchema)
└── Plugin runtime (shipped v0.14–v0.17)

Storage / integration
├── File system, Git
└── External tools (ROBOT, …)
```

## Responsibilities

| Layer | Owns | Does not own |
|-------|------|--------------|
| **Strixonomy** | Semantic truth, indexes, LSP methods, patch apply | VS Code APIs, React components |
| **OntoUI** | Global UI state, workspaces, components, host abstraction | Ontology parsing, reasoning algorithms |
| **Strixonomy** | Extension activation, commands, tree views, webview lifecycle, focus relay | Duplicate ontology logic in TypeScript |
| **OntoStudio** | Native shell, windows, marketplace (planned) | Separate React component tree |

## v0.13 shipped (OntoUI foundation)

| Component | Location | Notes |
|-----------|----------|-------|
| `WorkspaceHost` | `extension/webview-ui/src/host/` | VS Code adapter via `HostContext` |
| `WorkspaceStore` | `extension/webview-ui/src/store/` | Zustand; focus, query, reasoning, refactor slices |
| Event bus | `extension/webview-ui/src/store/events.ts` | `FocusChanged`, `QueryExecuted`, … |
| `WorkspaceRegistry` | `extension/webview-ui/src/workspaces/` | Panel → workspace routing |
| Focus relay | `extension/src/focus/focusRelay.ts` | Cross-webview `focusState` / `reasoningState` |
| Design tokens | `extension/webview-ui/src/tokens/` | `--oc-*` CSS variables |
| Schema browser | `extension/webview-ui/src/components/SchemaBrowser.tsx` | LSP `strixonomy/listSqlSchema` |

**Deferred to v0.30:** persistent tabs, bottom dock, full component migration for every panel.

## Key documents

| Topic | Document |
|-------|----------|
| OntoUI package | [ONTOUI.md](ONTOUI.md) |
| WorkspaceStore, hosts, events | [WORKSPACE_RUNTIME.md](WORKSPACE_RUNTIME.md) |
| Plugin capabilities | [CAPABILITY_PROVIDERS.md](CAPABILITY_PROVIDERS.md) |
| AI safe apply | [AI_ORCHESTRATION.md](AI_ORCHESTRATION.md) |
| Refactoring transactions | [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) |
| Reasoning pipeline | [REASONING_COMPILER.md](REASONING_COMPILER.md) |
| Graph stack | [GRAPH_ARCHITECTURE.md](GRAPH_ARCHITECTURE.md) |
| Query workbench | [QUERY_WORKBENCH_ARCHITECTURE.md](QUERY_WORKBENCH_ARCHITECTURE.md) |
| OntoStudio reuse | [ONTOSTUDIO_REUSE.md](ONTOSTUDIO_REUSE.md) |
| Milestone template | [MILESTONE_TEMPLATE.md](MILESTONE_TEMPLATE.md) |

## Product ADRs

Platform decisions: [adr/README.md](../adr/README.md). Engineering ADRs: [design/adr/README.md](../design/adr/README.md).

## Evolution

Previously described in [ui/PLATFORM_ARCHITECTURE.md](../ui/PLATFORM_ARCHITECTURE.md) and [architecture.md](../architecture.md). This hub consolidates **implementation** detail; those pages remain for ecosystem and UX context.
