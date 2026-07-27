# UI roadmap mapping

> **Document type:** Engineering / design mapping (GitHub). **Not** the evaluator capability matrix — use [What ships today](../SHIPPED.md) for adoption decisions.
>
> **Purpose:** Connect the [Product Roadmap 2.0](PRODUCT_ROADMAP_2.0.md) UX phases to **shipped** Strixonomy/Strixonomy releases and **planned** platform milestones.
>
> **Canonical shipped matrix:** [What ships today](../SHIPPED.md) · **Platform roadmap:** [GitHub ROADMAP.md](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md) · **v1.0 checklist:** [v1.0_BACKLOG.md](../design/v1.0_BACKLOG.md)

## How to use this document

| Audience | Read |
|----------|------|
| Product / design | [PRODUCT_DESIGN_SPECIFICATION.md](PRODUCT_DESIGN_SPECIFICATION.md), [DESIGN_PHILOSOPHY.md](DESIGN_PHILOSOPHY.md), [INFORMATION_ARCHITECTURE.md](INFORMATION_ARCHITECTURE.md) |
| Frontend contributors | [STATE_MANAGEMENT.md](STATE_MANAGEMENT.md), [COMPONENT_LIBRARY.md](COMPONENT_LIBRARY.md), [COMPONENT_INTERFACES.md](COMPONENT_INTERFACES.md), [DESIGN_TOKENS.md](DESIGN_TOKENS.md) |
| Platform / roadmap | This page + [roadmap.md](../roadmap.md) |
| Evaluators | [What ships today](../SHIPPED.md) first — UI specs describe targets, not all are implemented |

## Product stack (target)

| Layer | Document | Repo status |
|-------|----------|-------------|
| Strixonomy engine | [PLATFORM_ARCHITECTURE.md](PLATFORM_ARCHITECTURE.md) | **Shipped** — `strixonomy-*` crates |
| Strixonomy VS Code | [PRODUCT_DESIGN_SPECIFICATION.md](PRODUCT_DESIGN_SPECIFICATION.md) | **Shipped** — `extension/` + React `webview-ui/` |
| Shared React UI | [COMPONENT_LIBRARY.md](COMPONENT_LIBRARY.md), [UX_PATTERNS.md](UX_PATTERNS.md) | **Partial** — per-panel React webviews; no shared WorkspaceStore yet |
| OntoStudio desktop | [ONTOSTUDIO_DESKTOP.md](ONTOSTUDIO_DESKTOP.md) | **Planned** — post v1.0 |
| Plugin platform | [PLUGIN_PLATFORM.md](PLUGIN_PLATFORM.md), [PLUGIN_API_SPEC.md](PLUGIN_API_SPEC.md) | **Planned** — v0.14 |
| AI layer | [AI_EXPERIENCE.md](AI_EXPERIENCE.md), [AI_ORCHESTRATION_ARCHITECTURE.md](AI_ORCHESTRATION_ARCHITECTURE.md) | **Planned** — v1.1+ ([ADR-0010](../design/adr/0010-ai-features-opt-in.md)) |

---

## Master checklist — Product Roadmap 2.0

Every item from [PRODUCT_ROADMAP_2.0.md](PRODUCT_ROADMAP_2.0.md), with release assignment. **Shipped** = available in the tagged release named in the Target column (current public pin: **v0.25.0**).

| UI phase | Deliverable | Status | Target release | Spec |
|----------|-------------|--------|----------------|------|
| **0** | Audit existing VS Code extension | Shipped | v0.13 | [UX_AUDIT_v0.13.md](UX_AUDIT_v0.13.md) |
| **0** | Identify clunky UI flows | Shipped | v0.13 | [UX_PATTERNS.md](UX_PATTERNS.md) |
| **0** | Centralize webview communication | Shipped | v0.13 | [EVENT_SEQUENCE_DIAGRAMS.md](EVENT_SEQUENCE_DIAGRAMS.md) |
| **0** | Create basic WorkspaceStore | Shipped | v0.13 | [STATE_MANAGEMENT.md](STATE_MANAGEMENT.md) |
| **0** | Establish design tokens | Shipped | v0.13 | [DESIGN_TOKENS.md](DESIGN_TOKENS.md) |
| **0** | Migrate legacy UI to reusable components | Partial | v0.13 | [COMPONENT_LIBRARY.md](COMPONENT_LIBRARY.md) |
| **1** | Current Focus model | Shipped | v0.13 | [STATE_MANAGEMENT.md](STATE_MANAGEMENT.md) |
| **1** | Semantic navigation history | Partial | v0.13 | [WORKSPACE_MODEL.md](WORKSPACE_MODEL.md) |
| **1** | Shared explorer/inspector synchronization | Shipped | v0.13 | [INFORMATION_ARCHITECTURE.md](INFORMATION_ARCHITECTURE.md) |
| **1** | Persistent tabs | Planned | v1.0 | [WORKSPACE_WIREFRAMES.md](WORKSPACE_WIREFRAMES.md) |
| **1** | Bottom dock | Planned | v1.0 | [WORKSPACE_WIREFRAMES.md](WORKSPACE_WIREFRAMES.md) |
| **1** | Command palette integration | Shipped | v0.2+ | VS Code commands |
| **1** | Core event bus | Shipped | v0.13 | [EVENT_SEQUENCE_DIAGRAMS.md](EVENT_SEQUENCE_DIAGRAMS.md) |
| **2** | Entity editor MVP | Shipped | v0.7+ | [ENTITY_EDITOR_SPEC.md](ENTITY_EDITOR_SPEC.md) |
| **2** | Inline labels/comments | Shipped | v0.12 | [ENTITY_EDITOR_SPEC.md](ENTITY_EDITOR_SPEC.md) |
| **2** | Hierarchy view | Shipped | v0.2+ | Explorer + reasoner toggle |
| **2** | Relationship cards | Planned | v1.0 | [ENTITY_EDITOR_SPEC.md](ENTITY_EDITOR_SPEC.md) |
| **2** | References view | Partial | v1.0 | LSP find references (v0.8) |
| **2** | Metadata view | Planned | v1.0 | [ENTITY_EDITOR_SPEC.md](ENTITY_EDITOR_SPEC.md) |
| **2** | Diagnostics integration in entity workspace | Partial | v1.0 | [REASONING_EXPERIENCE.md](REASONING_EXPERIENCE.md) |
| **2** | AI explain entity | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **3** | SQL/OntoSQL editor | Shipped | v0.5+ | [QUERY_WORKBENCH.md](QUERY_WORKBENCH.md) |
| **3** | Schema browser | Shipped | v0.13 | [QUERY_WORKBENCH.md](QUERY_WORKBENCH.md) |
| **3** | Query execution | Shipped | v0.5+ | [QUERY_WORKBENCH.md](QUERY_WORKBENCH.md) |
| **3** | Results table | Shipped | v0.5+ | [QUERY_WORKBENCH.md](QUERY_WORKBENCH.md) |
| **3** | Query history | Shipped | v0.5+ | [QUERY_WORKBENCH.md](QUERY_WORKBENCH.md) |
| **3** | Saved queries | Shipped | v0.5+ | [QUERY_WORKBENCH.md](QUERY_WORKBENCH.md) |
| **3** | AI query generation | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **4** | Semantic canvas MVP | Shipped | v0.7+ | [GRAPH_WORKSPACE.md](GRAPH_WORKSPACE.md) |
| **4** | Focus neighborhood graph | Shipped | v0.7+ | [GRAPH_WORKSPACE.md](GRAPH_WORKSPACE.md) |
| **4** | Pan/zoom/select | Shipped | v0.7+ | [GRAPH_RENDERING_ARCHITECTURE.md](GRAPH_RENDERING_ARCHITECTURE.md) |
| **4** | Saved layouts | Planned | v1.0 | [GRAPH_WORKSPACE.md](GRAPH_WORKSPACE.md) |
| **4** | Filters | Shipped | v0.25 | [GRAPH_WORKSPACE.md](GRAPH_WORKSPACE.md) · functional filters; saved filter presets remain v1.0 |
| **4** | Reasoning overlays | Shipped | v0.25 | [GRAPH_WORKSPACE.md](GRAPH_WORKSPACE.md) · unsatisfiable badges + inferred edges; richer overlays remain v1.0 |
| **4** | AI graph explanations | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **5** | Semantic build pipeline UI | Planned | v1.0 | [REASONING_EXPERIENCE.md](REASONING_EXPERIENCE.md) |
| **5** | Problems panel integration | Partial | v1.0 | VS Code Problems + LSP diagnostics |
| **5** | Entity-level reasoning cards | Planned | v1.0 | [REASONING_EXPERIENCE.md](REASONING_EXPERIENCE.md) |
| **5** | Inference explanations | Shipped | v0.9–v0.12 | Explanation panel |
| **5** | Quick fixes | Shipped | v0.11 | LSP code actions |
| **5** | Reasoning history | Planned | v1.0 | [REASONING_EXPERIENCE.md](REASONING_EXPERIENCE.md) |
| **6** | Rename | Shipped | v0.8+ | [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) |
| **6** | Safe delete | Shipped | v0.4+ | Patch + inspector |
| **6** | Merge classes | Planned | v1.0 | [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) |
| **6** | Extract module | Shipped | v0.8+ | [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) |
| **6** | Batch label normalization | Planned | v1.0 | [SEMANTIC_REFACTORING.md](SEMANTIC_REFACTORING.md) |
| **6** | Refactoring preview | Shipped | v0.8+ | Refactor Preview panel |
| **6** | Undo/redo integration | Planned | v1.0 | VS Code undo on file writes |
| **7** | AI sidebar | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **7** | Inline suggestions | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **7** | Generate documentation | Partial | v1.1 | `strixonomy docs` shipped v0.11; AI assist v1.1 |
| **7** | Review ontology | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **7** | Repair diagnostics | Planned | v1.1 | [AI_EXPERIENCE.md](AI_EXPERIENCE.md) |
| **7** | Project-wide AI tasks | Planned | v1.1 | [AI_ORCHESTRATION_ARCHITECTURE.md](AI_ORCHESTRATION_ARCHITECTURE.md) |
| **8** | Manifest | Planned | v0.14 | [PLUGIN_API_SPEC.md](PLUGIN_API_SPEC.md) |
| **8** | Plugin runtime | Planned | v0.14 | [PLUGIN_PLATFORM.md](PLUGIN_PLATFORM.md) |
| **8** | Command API | Planned | v0.14 | [PLUGIN_API_SPEC.md](PLUGIN_API_SPEC.md) |
| **8** | Inspector card API | Planned | v0.14 | [PLUGIN_API_SPEC.md](PLUGIN_API_SPEC.md) |
| **8** | AI provider API | Planned | v1.1 | [PLUGIN_API_SPEC.md](PLUGIN_API_SPEC.md) |
| **8** | Reasoner provider API | Planned | v0.14 | [PLUGIN_API_SPEC.md](PLUGIN_API_SPEC.md) |
| **8** | SDK and examples | Shipped | v0.25 | [guides/plugins.md](../guides/plugins.md) · TOML+subprocess SDK 1.0; marketplace/AI remain later |
| **9** | Semantic diff | Shipped | v0.10+ | [COLLABORATION.md](COLLABORATION.md) |
| **9** | Review workspace | Planned | v1.0 / Post-1.2 | [COLLABORATION.md](COLLABORATION.md) |
| **9** | GitHub integration | Planned | v1.2 | [COLLABORATION.md](COLLABORATION.md) |
| **9** | Semantic PR summaries | Partial | v0.13 / v1.2 | CLI/LSP `--pr-summary` shipped v0.13; UI panel v1.2 |
| **9** | AI review | Planned | v1.1 | [COLLABORATION.md](COLLABORATION.md) |
| **9** | Merge checks | Planned | v1.2 | [COLLABORATION.md](COLLABORATION.md) |
| **10** | Tauri app shell | Planned | Post-1.2 | [ONTOSTUDIO_DESKTOP.md](ONTOSTUDIO_DESKTOP.md) |
| **10** | Shared React UI | Partial | v0.13–Post-1.2 | [COMPONENT_LIBRARY.md](COMPONENT_LIBRARY.md) |
| **10** | Native graph performance | Planned | Post-1.2 | [GRAPH_RENDERING_ARCHITECTURE.md](GRAPH_RENDERING_ARCHITECTURE.md) |
| **10** | Plugin marketplace | Planned | v1.2 | [PLUGIN_PLATFORM.md](PLUGIN_PLATFORM.md) |
| **10** | Local AI support | Planned | Post-1.2 | [AI_ORCHESTRATION_ARCHITECTURE.md](AI_ORCHESTRATION_ARCHITECTURE.md) |
| **10** | Enterprise packaging | Planned | Post-1.2 | [ONTOSTUDIO_DESKTOP.md](ONTOSTUDIO_DESKTOP.md) |
| **11** | Public plugin registry | Planned | v1.2 | [PLUGIN_PLATFORM.md](PLUGIN_PLATFORM.md) |
| **11** | Documentation site | Shipped | v0.9+ | Read the Docs |
| **11** | Sample domain plugins | Planned | v1.2 | [design/PLUGIN_SPEC.md](../design/PLUGIN_SPEC.md) |
| **11** | Tutorials | Partial | ongoing | [first-success](../guides/first-success.md) |
| **11** | Community templates | Planned | v1.2 | — |
| **11** | Enterprise adoption guides | Shipped | v0.11+ | [enterprise-eval](../guides/enterprise-eval.md) |
| **12** | Browser client | Planned | v1.3 (Post-1.2 Era I) | [PLATFORM_ARCHITECTURE.md](PLATFORM_ARCHITECTURE.md) · [ROADMAP.md Era I](../../ROADMAP.md) |
| **12** | Strixonomy WASM (backendless browser engine) | Planned | v1.3 | [ROADMAP.md Era I](../../ROADMAP.md) |
| **12** | React app (no backend) — static SPA | Planned | v1.3 | [ROADMAP.md Era I](../../ROADMAP.md) |
| **12** | Cloud sync | Planned | Post-1.2 | [PLATFORM_ARCHITECTURE.md](PLATFORM_ARCHITECTURE.md) |
| **12** | Team workspaces | Planned | Post-1.2 | [COLLABORATION.md](COLLABORATION.md) |
| **12** | Distributed reasoning | Planned | Post-1.2 | [REASONING_EXPERIENCE.md](REASONING_EXPERIENCE.md) |
| **12** | Shared semantic canvases | Planned | Post-1.2 | [GRAPH_WORKSPACE.md](GRAPH_WORKSPACE.md) |
| **12** | Governance workflows | Planned | Post-1.2 | [guides/governance](../guides/governance.md) |

### Supporting design system (no separate UI phase)

| Deliverable | Status | Target | Spec |
|-------------|--------|--------|------|
| Visual design system | Planned | v0.13 | [VISUAL_DESIGN_SYSTEM.md](VISUAL_DESIGN_SYSTEM.md) |
| Human interface guidelines | Planned | v1.0 | [HUMAN_INTERFACE_GUIDELINES.md](HUMAN_INTERFACE_GUIDELINES.md) |
| Interaction principles | Reference | — | [INTERACTION_PRINCIPLES.md](INTERACTION_PRINCIPLES.md) |
| Keyboard shortcuts catalog | Shipped | v0.17 | [KEYBOARD_SHORTCUTS.md](KEYBOARD_SHORTCUTS.md) |
| Accessibility (WCAG) | Shipped | v0.25 | [ACCESSIBILITY_SPEC.md](ACCESSIBILITY_SPEC.md) · EPIC-010 owned webviews + axe; external cert later |
| Telemetry / analytics | Not planned (opt-in only) | — | [TELEMETRY_ANALYTICS.md](TELEMETRY_ANALYTICS.md) |
| Documentation workspace UX | Partial | v1.0 | [PRODUCT_DESIGN_SPECIFICATION.md](PRODUCT_DESIGN_SPECIFICATION.md) §4.6 |

---

## Release summary (by milestone)

### v0.13 — UI phases 0–1 + partial 3/5/9 (shipped 2026-07-07)

| Deliverable | UI phase | Priority |
|-------------|----------|----------|
| Extension UX audit + flow fixes | 0 | P1 |
| WorkspaceStore + Current Focus | 0, 1 | **P0** |
| Centralized webview bus + event bus | 0, 1 | **P0** |
| Design tokens + component library (Inspector + Query Workbench) | 0 | **P0** |
| Explorer ↔ inspector ↔ graph focus relay | 1 | **P0** |
| Semantic navigation history | 1 | P0 (stub) |
| Refactor + reasoning store slices | 5, 6 | P0 / P1 |
| Query schema browser | 3 | P1 |
| Horned-OWL axiom virtual tables | 3 | P1 |
| Semantic PR summary (`strixonomy diff --pr-summary`) | 9 | P1 |
| Accessibility + webview tests | supporting | P1 |

Full list: [roadmap.md § v0.13](../roadmap.md)

### v0.14 — UI phase 8

| Deliverable | UI phase |
|-------------|----------|
| Plugin manifest + runtime | 8 |
| Command API + inspector card API | 8 |
| Reasoner provider API | 8 |
| Plugin SDK + examples | 8 |
| Plugin-contributed Problems panel | 8 |

Full list: [roadmap.md § v0.14](../roadmap.md)

### v1.0 — UI phases 1–6 exit polish + partial 9

| Deliverable | UI phase |
|-------------|----------|
| Persistent tabs + bottom dock | 1 |
| Relationship cards, references, metadata views | 2 |
| Entity diagnostics integration | 2 |
| Graph saved layouts, filters, reasoning overlays | 4 |
| Semantic build pipeline + reasoning history | 5 |
| Entity-level reasoning cards | 5 |
| Merge classes + batch label normalization | 6 |
| Undo/redo on refactor writes | 6 |
| Review workspace (MVP) | 9 |
| HIG + keyboard shortcuts | supporting |

Full list: [roadmap.md § v1.0](../roadmap.md)

### v1.1 — UI phase 7 + deferred AI from 2/3/4/8/9

| Deliverable | UI phase |
|-------------|----------|
| AI sidebar + inline suggestions | 7 |
| AI explain entity | 2 |
| AI query generation | 3 |
| AI graph explanations | 4 |
| AI ontology review + repair diagnostics | 7, 9 |
| Project-wide AI tasks | 7 |
| AI provider API | 8 |
| MCP context bridge | 7 |

Full list: [roadmap.md § v1.1](../roadmap.md)

### v1.2 — UI phases 9 + 11

| Deliverable | UI phase |
|-------------|----------|
| GitHub integration | 9 |
| Merge checks | 9 |
| Semantic PR summaries (UI) | 9 |
| Public plugin registry + marketplace UI | 10, 11 |
| Sample domain plugins | 11 |
| Community templates | 11 |
| QC / workflow status dashboard | toolchain |

Full list: [roadmap.md § v1.2](../roadmap.md)

### Post-1.2 — UI phases 10 + 12

| Deliverable | UI phase |
|-------------|----------|
| OntoStudio Tauri shell | 10 |
| Native graph performance | 10 |
| Local AI support | 10 |
| Enterprise packaging | 10 |
| Browser client (hosted service + React app no-backend) | 12 |
| Strixonomy WASM MVP (engine for React app no-backend) | 12 |
| React app (no backend) — static SPA | 12 |
| Cloud sync + team workspaces | 12 |
| Distributed reasoning | 12 |
| Shared semantic canvases | 12 |
| Governance workflows | 12 |
| Live collaboration + PR review UX | 9, 12 |

Full list: [roadmap.md § Post-1.2](../roadmap.md)

---

## Telemetry & analytics

[TELEMETRY_ANALYTICS.md](TELEMETRY_ANALYTICS.md) describes optional product analytics. **Default policy:** no telemetry shipped ([security.md](../security.md), [ADR-0010](../design/adr/0010-ai-features-opt-in.md)). Any future analytics must be opt-in.

## Related legacy docs

| Doc | Status |
|-----|--------|
| [design/UI_WIREFRAMES.md](../design/UI_WIREFRAMES.md) | Superseded for detail by [WORKSPACE_WIREFRAMES.md](WORKSPACE_WIREFRAMES.md) |
| [design/OntoCode_React_UI_Integration_Plan.md](../design/OntoCode_React_UI_Integration_Plan.md) | Historical migration plan — React panels shipped v0.7–v0.11 ([ADR-0017](../design/adr/0017-react-webview-ui.md)) |

## Document index

See [README.md](README.md) for the full specification pack listing.
