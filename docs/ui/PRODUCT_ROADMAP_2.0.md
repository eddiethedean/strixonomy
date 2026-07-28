# Product Roadmap 2.0

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.
>
> **Milestone template:** [platform/MILESTONE_TEMPLATE.md](../platform/MILESTONE_TEMPLATE.md) · **Implementation prompts:** [cursor-prompts/README.md](../cursor-prompts/README.md)

---

## Phase 0 — Stabilize OntoUI (Extension) {#phase-0}

**Phase ID:** 0 · **Target release:** v0.13 · **Status:** planned

### Goal

Centralize OntoUI scaffolding: host adapter, design tokens, and webview communication patterns.

### User-visible outcome

- Extension UI feels consistent (spacing, buttons, typography).
- Fewer duplicate postMessage handlers between panels.

### Technical scope

- WorkspaceHost adapter for VS Code
- Design token CSS variables
- Audit clunky flows; document in ROADMAP_MAPPING

### Files / modules

| Path | Change |
|------|--------|
| `extension/webview-ui/src/host/` | New WorkspaceHost |
| `extension/webview-ui/src/tokens/` | Token → CSS vars |
| `extension/src/webview/` | Centralize message routing |

### Acceptance criteria

- [ ] HostContext used by at least one panel
- [ ] Design tokens applied in global.css
- [ ] Vitest passes for host + tokens

### Tests required

- Unit: host adapter, token vars
- E2E: smoke webview loads

### Risks

| Risk | Mitigation |
|------|------------|
| Breaking existing panels | Incremental migration; keep ?panel= URLs |

### Dependencies

- [platform/ONTOUI.md](../platform/ONTOUI.md)
- [cursor-prompts/01-build-ontoui-workspace-platform.md](../cursor-prompts/01-build-ontoui-workspace-platform.md)
- [cursor-prompts/13-design-tokens-component-library.md](../cursor-prompts/13-design-tokens-component-library.md)

---

## Phase 1 — Workspace foundation {#phase-1}

**Phase ID:** 1 · **Target release:** v0.13 · **Status:** planned

### Goal

Introduce WorkspaceStore, Current Focus, event bus, and WorkspaceRegistry.

### User-visible outcome

- Selecting an entity in explorer updates inspector and graph context consistently (within shared webview / relay).

### Technical scope

- WorkspaceStore global state
- Current Focus + FocusChanged events
- WorkspaceRegistry; App.tsx workspace routing
- Semantic navigation history (stub)

### Files / modules

| Path | Change |
|------|--------|
| `extension/webview-ui/src/store/` | WorkspaceStore |
| `extension/webview-ui/src/workspaces/` | Registry |
| `extension/webview-ui/src/App.tsx` | Workspace routing |

### Acceptance criteria

- [ ] Store tests for focus/selection
- [ ] Registry registers entity, graph, query workspaces
- [ ] FocusChanged documented in EVENT_SEQUENCE_DIAGRAMS

### Tests required

- Unit: store, registry, events
- E2E: entity selection updates focus (best effort)

### Risks

| Risk | Mitigation |
|------|------------|
| Separate VS Code webviews cannot share memory | Extension host relay for cross-panel focus (follow-up) |

### Dependencies

- Phase 0
- [platform/WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)
- [cursor-prompts/02-add-workspacestore.md](../cursor-prompts/02-add-workspacestore.md) through [05-implement-current-focus.md](../cursor-prompts/05-implement-current-focus.md)

---

## Phase 2 — Entity workspace

**Phase ID:** 2 · **Target release:** v0.13–v0.30 · **Status:** partial (inspector shipped v0.12)

### Goal

Entity Workspace MVP: editor driven by Current Focus with relationship and metadata views.

### User-visible outcome

- Edit labels, axioms, OBO fields in inspector with focus-aware navigation.
- Relationship cards and references view (v0.30 subset).

### Technical scope

- EntityInspector → Entity Workspace integration with store
- Inline labels/comments (shipped)
- Hierarchy, references, metadata views (v0.30)
- Diagnostics integration in inspector

### Files / modules

| Path | Change |
|------|--------|
| `extension/webview-ui/src/panels/EntityInspector.tsx` | Store-driven |
| `ui/ENTITY_EDITOR_SPEC.md` | UX reference |

### Acceptance criteria

- [ ] Inspector reads focus from store
- [ ] No redundant getEntity when focus unchanged
- [ ] ENTITY_EDITOR_SPEC P0 items mapped in ROADMAP_MAPPING

### Tests required

- Unit: EntityInspector + store
- Existing e2e inspector tests pass

### Dependencies

- Phase 1
- [cursor-prompts/06-improve-entity-workspace.md](../cursor-prompts/06-improve-entity-workspace.md)

---

## Phase 3 — Query workspace

**Phase ID:** 3 · **Target release:** v0.13 · **Status:** partial (workbench shipped)

### Goal

Query Workspace with schema browser and store-backed history.

### User-visible outcome

- Browse SQL virtual table names and insert into editor.
- Query history persists in session.

### Technical scope

- Query slice in WorkspaceStore
- Schema browser (catalog snapshot)
- SQL/SPARQL editor (shipped)

### Files / modules

| Path | Change |
|------|--------|
| `extension/webview-ui/src/panels/QueryWorkbench.tsx` | Store integration |
| `extension/webview-ui/src/components/SchemaBrowser.tsx` | New stub |

### Acceptance criteria

- [ ] Schema browser lists core tables (classes, entities, diagnostics)
- [ ] QueryExecuted event on run

### Tests required

- QueryWorkbench.test.tsx

### Dependencies

- Phase 1
- [platform/QUERY_WORKBENCH_ARCHITECTURE.md](../platform/QUERY_WORKBENCH_ARCHITECTURE.md)

---

## Phase 4 — Graph workspace

**Phase ID:** 4 · **Target release:** v0.13–v0.30 · **Status:** partial (GraphPanel shipped)

### Goal

Graph Workspace with focus neighborhood sync and progressive loading.

### User-visible outcome

- Graph centers on Current Focus entity.
- Pan/zoom/select updates focus.

### Technical scope

- Focus sync (Phase 1)
- Saved layouts, filters (v0.30)
- Reasoning overlays (v0.30)

### Files / modules

| Path | Change |
|------|--------|
| `extension/webview-ui/src/panels/GraphPanel.tsx` | Focus subscription |

### Acceptance criteria

- [ ] Graph reloads on FocusChanged
- [ ] Node select updates focus

### Dependencies

- Phase 1
- [platform/GRAPH_ARCHITECTURE.md](../platform/GRAPH_ARCHITECTURE.md)

---

## Phase 5 — Reasoning experience

**Phase ID:** 5 · **Target release:** v0.30 · **Status:** partial (reasoner panels shipped)

### Goal

Reasoning-as-compiler UX: problems panel, reasoning history, entity cards.

### User-visible outcome

- Unsatisfiable classes appear in problems-style list.
- Reasoner run history visible in UI.

### Technical scope

- Reasoning slice in store
- Problems panel (v0.30)
- Quick fixes linkage (partial via LSP codeAction)

### Files / modules

| Path | Change |
|------|--------|
| `extension/webview-ui/src/store/` | reasoning slice |
| Reasoner webview panels | Store integration |

### Acceptance criteria

- [ ] ReasoningCompleted updates store
- [ ] Profile + hierarchy mode in store

### Dependencies

- Phase 1
- [platform/REASONING_COMPILER.md](../platform/REASONING_COMPILER.md)

---

## Phase 6 — Semantic refactoring

**Phase ID:** 6 · **Target release:** v0.13–v0.30 · **Status:** partial (preview/apply shipped)

### Goal

Unified refactoring transaction model with store-backed preview.

### User-visible outcome

- All refactor ops require preview before apply.
- Merge classes and batch label ops (v0.30).

### Technical scope

- Refactoring store slice
- RefactorPreview integration
- Undo/redo (v0.30)

### Acceptance criteria

- [ ] Pending preview in store; cleared on apply/cancel
- [ ] Existing rename/move/extract unchanged functionally

### Dependencies

- [platform/SEMANTIC_REFACTORING.md](../platform/SEMANTIC_REFACTORING.md)

---

## Phase 7 — AI-native workflows

**Phase ID:** 7 · **Target release:** v0.31+ · **Status:** proposed

### Goal

AI sidebar and inline suggestions with mandatory preview-before-apply.

### User-visible outcome

- AI proposes patches; user always sees preview before apply.

### Technical scope

- AI lifecycle state machine (stub → provider)
- Context builder from Current Focus
- Opt-in per ADR-0010

### Acceptance criteria

- [ ] No apply path bypasses PreviewShown state
- [ ] Audit log stub in store

### Dependencies

- Phase 1, 6
- [platform/AI_ORCHESTRATION.md](../platform/AI_ORCHESTRATION.md)
- [adr/0006-ai-preview-before-apply.md](../adr/0006-ai-preview-before-apply.md)

---

## Phase 8 — Plugin platform

**Phase ID:** 8 · **Target release:** v0.14 · **Status:** planned

### Goal

Capability Provider plugin host MVP with manifest and inspector cards.

### User-visible outcome

- Install reference plugin (naming validator) from workspace config.
- Plugin contributes inspector card or diagnostic rule.

### Technical scope

- Strixonomy plugin runtime
- Capability Provider TS/Rust interfaces
- Reference plugins (naming, Markdown export, SHACL)

### Acceptance criteria

- [ ] Plugin load/discover from config
- [ ] At least one reference plugin runs in CI

### Dependencies

- [platform/CAPABILITY_PROVIDERS.md](../platform/CAPABILITY_PROVIDERS.md)
- [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md)

---

## Phase 9 — Collaboration

**Phase ID:** 9 · **Target release:** v0.30–v0.31 · **Status:** partial (semantic diff shipped v0.10)

### Goal

Review workspace, semantic PR summaries, validation reports.

### User-visible outcome

- Semantic diff panel (shipped)
- PR summary from CLI/LSP (v0.13 Strixonomy)
- Review workspace MVP (v0.30)

### Acceptance criteria

- [ ] PR summary in `strixonomy diff` documented and tested
- [ ] Review workspace spec items in ROADMAP_MAPPING tracked

### Dependencies

- Semantic diff (shipped)
- [ui/COLLABORATION.md](COLLABORATION.md)

---

## Phase 10–12 — OntoStudio, ecosystem, platform (summary)

| Phase | Title | Target | Status | Spec |
|-------|-------|--------|--------|------|
| 10 | OntoStudio desktop | Post v0.30 | planned | [ONTOSTUDIO_DESKTOP.md](ONTOSTUDIO_DESKTOP.md), [platform/ONTOSTUDIO_REUSE.md](../platform/ONTOSTUDIO_REUSE.md) |
| 11 | Ecosystem | Post v0.30 | proposed | Plugin registry, tutorials |
| 12 | Semantic engineering platform | Post v0.32 | proposed | Browser client, team workspaces |

Full milestones for Phases 10–12 will be added when OntoUI platform (Phases 0–1) ships.

---

## Success metric

Strixonomy becomes the default modern environment for ontology and knowledge graph engineering — measured by adoption, Protégé parity P0 closure, and plugin ecosystem growth.
