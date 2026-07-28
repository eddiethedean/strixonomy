# OntoCore & OntoCode Roadmap

## Vision

Build the modern open-source platform for ontology engineering.

**OntoCore** is the semantic workspace engine.

**OntoCode** is the flagship IDE powered by OntoCore.

Full mission and principles: [VISION.md](VISION.md). Ecosystem layers: [ARCHITECTURE.md](ARCHITECTURE.md).

## Guiding principle

**OntoCode 1.0 has one primary objective: become a production-ready replacement for Protégé.**

Every feature before 1.0 should answer one question:

> Does this make it easier for ontology engineers to adopt OntoCode instead of Protégé?

After 1.0, the roadmap shifts from parity to modernization.

>  **Not a Protégé replacement today.** v0.26 supports pilot and coexistence workflows — not org-wide Protégé retirement. See [What ships today](docs/SHIPPED.md) and [Known limitations](docs/known-limitations.md).

---

## How to read this document

| Document | Role |
|----------|------|
| [SHIPPED.md](docs/SHIPPED.md) | **Canonical capability matrix** — what is available in the current release |
| [protege-parity/](docs/protege-parity/README.md) | **1.0 engineering program** — scope, blockers, release gates |
| [PRE_1_0_PHASES.md](docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) | **Pre-1.0 release phases** — v0.19–v0.27 → 1.0.0 |
| [ROADMAP_MAPPING.md](docs/ui/ROADMAP_MAPPING.md) | **UI specs ↔ releases** — master checklist for all Product Roadmap 2.0 items |
| [design/ROADMAP.md](docs/design/ROADMAP.md) | Per-crate engineering detail for **shipped** v0.1–v0.11 milestones |
| [PROTEGE_PARITY.md](docs/design/PROTEGE_PARITY.md) | Historical v0.18 P0/P1/P2 checklist (superseded for planning) |
| [v1.0_BACKLOG.md](docs/design/v1.0_BACKLOG.md) | Implementation checklist toward v1.0 |
| [platform/OVERVIEW.md](docs/platform/OVERVIEW.md) | OntoUI / WorkspaceStore architecture (foundation shipped v0.13) |
| [PRODUCT_ROADMAP_2.0.md](docs/ui/PRODUCT_ROADMAP_2.0.md) | UI phases with milestone acceptance criteria |
| [ui/README.md](docs/ui/README.md) | Product design specification pack (UX, design system, OntoStudio target) |

**Current release:** v0.27.0

---

## Release phases at a glance

### Timeline

```text
SHIPPED (v0.1–v0.27) ─────────────────────────────────────────────────►
v0.1–v0.4          v0.5–v0.8              v0.9–v0.12           v0.13–v0.27
Engine foundation    IDE depth                Platform & authoring   OntoUI → formats → OWL 2
  │                    │                        │                      │
  Foundation           Query, reason,           Identity, diff,      UX shell + semantic txn
  Explorer, diag,      graphs, refactor,        OBO write-back,      (v0.18–v0.20) + RDF/OWL-XML
  write-back           Manchester               OWL/XML catalog      WB (v0.21) + OWL 2 (v0.22)
                                                                       + reasoning/SWRL (v0.23)
                                                                       + refactor/DL Query (v0.24)
                                                                       + viz/SDK/a11y/CI (v0.25)
                                                                       + Protégé test port (v0.26)
                                                                       + Strixonomy rename (v0.27)

PLANNED (1.0.0-rc → 1.0) ─────────────────────────────────────────────►
1.0.0-rc               1.0.0
Stabilize              Protégé replacement
All P0 VERIFIED        Full parity ship

WEBAPP PATH (post-1.2) ───────────────────────────────────────────────►
v1.3                 v1.4                    v1.5+
Web foundation       Team workspaces          Cloud collaboration
  │                    │                        │
  Browser shell       Projects + sharing       Live editing + governance
  Service + WASM      Auth + permissions       Enterprise deployment
  React app           Lightweight edits        Hybrid WASM preview
  (no backend)
```

### Phase index

| Era | Versions | Status | North-star |
|-----|----------|--------|------------|
| **A — Engine foundation** | v0.1–v0.4 | Shipped | Index, browse, diagnose, edit Turtle |
| **B — IDE depth** | v0.5–v0.8 | Shipped | Query, reason, visualize, refactor |
| **C — Platform & authoring** | v0.9–v0.12 | Shipped | OntoCore identity, semantic workspace, authoring parity |
| **D — OntoUI platform** | v0.13–v0.14 | Shipped | v0.13: WorkspaceStore, focus relay; v0.14: plugin host MVP |
| **E — Desktop UX shell gate** | v0.15–v0.18 | Shipped | Menus, layouts, workflows, migration readiness (not full parity) |
| **F — Full Protégé parity path** | v0.19–v0.27 | Shipped through v0.27 | Semantic core → formats → OWL 2 → reason/SWRL → services → verify → Protégé JUnit behavioral port → Strixonomy rename |
| **G — Protégé replacement** | 1.0.0 | Planned | Daily OWL/OBO engineering without Protégé |
| **H — Ecosystem** | v1.1–v1.2 | Planned | SDKs, AI, toolchain & collaboration |
| **I — Webapp platform** | v1.3–v1.5+ | Planned | Browser-first ontology engineering: hosted webapp, React app (no backend) via WASM, team workspaces, cloud collaboration |

| Phase | Version | Era | Status | UI phases | Theme |
|-------|---------|-----|--------|-----------|-------|
| 1 | v0.1 | A | Shipped | — | OntoCore foundation |
| 2 | v0.2 | A | Shipped | 1 (partial) | VS Code explorer |
| 3 | v0.3 | A | Shipped | — | Diagnostics |
| 4 | v0.4 | A | Shipped | 2 (partial) | Turtle write-back + Horned-OWL |
| 5 | v0.5 | B | Shipped | 3 | Query workbench + Manchester MVP |
| 6 | v0.6 | B | Shipped | — | Ontologos reasoning (EL/RL/RDFS) |
| 7 | v0.7 | B | Shipped | 2, 4 (partial) | React UI, graphs, OBO + ROBOT |
| 8 | v0.8 | B | Shipped | 6 | Refactoring + full Manchester |
| 9 | v0.9 | C | Shipped | 5 (partial) | OntoCore platform identity |
| 10 | v0.10 | C | Shipped | 9 (partial) | Semantic workspace |
| 11 | v0.11 | C | Shipped | 5, 7, 11 (partial) | Editor depth & distribution |
| 12 | v0.12 | C | Shipped | 2 (P0 exit) | Authoring parity |
| 13 | v0.13 | D | Shipped | 0, 1, 3†, 5†, 9† | Platform hardening |
| 14 | v0.14 | D | Shipped | 8 | Plugin host MVP |
| 15 | v0.15 | E | Shipped | 4†, 5†, 8† | Plugin API + visualization + explanations |
| 16 | v0.16 | E | Shipped | 1†, 2† | Workspace layouts + preferences + imports polish |
| 17 | v0.17 | E | Shipped | — | Menu/toolbar/dialog parity + keyboard workflows |
| 18 | v0.18 | E | Shipped | — | Desktop UX shell gate + migration readiness |
| 19 | v0.19 | F | Shipped | — | Semantic foundation + program baseline |
| 20 | v0.20 | F | Shipped | 1† | Workspace runtime |
| 21 | v0.21 | F | Shipped | — | RDF/XML + OWL/XML write-back |
| 22 | v0.22 | F | Shipped | 2† | Complete OWL 2 authoring |
| 23 | v0.23 | F | Shipped | 5† | Reasoning parity + SWRL |
| 24 | v0.24 | F | Shipped | 3†, 6† | Refactoring + DL Query parity |
| 25 | v0.25 | F | Shipped | 4†, 8† | Viz + plugin SDK 1.0 + a11y + parity CI |
| 26 | v0.26 | F | Shipped | — | Protégé Desktop JUnit behavioral test port (Waves 1–4) |
| 27 | v0.27 | F | Shipped | — | Rename OntoCore and OntoCode to Strixonomy |
| 28 | 1.0.0-rc | F | Planned | — | Stabilize; all P0 VERIFIED |
| 29 | v1.0 | G | Planned | 1–6 exit, 9† | Protégé-competitive release |
| 30 | v1.1 | H | Planned | 7, 2†, 3†, 4†, 8†, 9† | Language bindings & AI primitives |
| 31 | v1.2 | H | Planned | 9, 10, 11 | Ontology toolchain platform |
| 32 | v1.3 | I | Planned | 10, 12† | Webapp foundation + React app (no backend) |
| 33 | v1.4 | I | Planned | 9, 12 | Team workspaces |
| 34 | v1.5+ | I | Planned | 9, 12 | Cloud collaboration + governance |

†Partial scope in this release (remainder in later releases). Full mapping: [ROADMAP_MAPPING.md](docs/ui/ROADMAP_MAPPING.md).

### UI phase reference (Product Roadmap 2.0)

OntoUI work uses **UI phases 0–12** from [Product Roadmap 2.0](docs/ui/PRODUCT_ROADMAP_2.0.md). They are integrated into release phases above — not a separate track.

| UI phase | Name | Primary releases |
|----------|------|------------------|
| **0** | Stabilize OntoUI | v0.13 (shipped) |
| **1** | Workspace foundation | v0.13 (core shipped); v1.0 (tabs, dock) |
| **2** | Entity workspace | v0.4–v0.12 (MVP); v1.0 (relationship/metadata views); v1.1† (AI explain) |
| **3** | Query workbench | v0.5+ (shipped); v0.13† (schema browser shipped); v1.1† (AI query) |
| **4** | Graph workspace | v0.7+ (shipped); v1.0 (layouts, filters); v1.1† (AI graph) |
| **5** | Reasoning experience | v0.9–v0.13† (store integration shipped); v1.0 (pipeline UI, history) |
| **6** | Semantic refactoring | v0.8+ (shipped); v1.0 (merge, batch, undo) |
| **7** | AI experience | v1.1 |
| **8** | Plugin platform | v0.14 (runtime shipped); v1.1† (AI provider API) |
| **9** | Collaboration | v0.10+ (diff); v0.13† (PR summary CLI shipped); v1.0 (review); v1.2 (GitHub UI) |
| **10** | OntoStudio desktop | v1.2† (marketplace); Post-1.2 (shell, native graph) |
| **11** | Ecosystem & docs | v0.11+ (guides); v1.2 (registry, templates) |
| **12** | Semantic engineering platform | v1.3 (browser shell, React app no-backend, OntoCore WASM); v1.4 (team workspaces); v1.5+ (cloud collaboration) |

> **Note on v0.13–v0.18 (retired labels):** Earlier drafts used v0.13–v0.18 for capabilities that **shipped in v0.3–v0.11** (diagnostics, SQL virtual tables, refactoring, Ontologos reasoning, semantic diff, docs export). Those labels are retired. Forward work from v0.13 onward is defined in the phases below.

---

## Shipped releases (v0.1–v0.19)

### Era A — Engine foundation (v0.1–v0.4)

### v0.1 — OntoCore foundation (shipped)

**Theme:** Prove the semantic workspace engine — index, catalog, and query ontology files from the CLI.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Rust workspace; CLI skeleton; recursive scanner; file hashing; parser adapters; basic catalog (`ontologies`, `classes`, `properties` tables); SQL and SPARQL query |
| **OntoCode** | — |

**Exit criteria:** `ontocore query ./repo "SELECT * FROM classes"` returns indexed classes.

**Dependencies:** `oxigraph`, `sqlparser`, `ignore`, `clap`

---

### v0.2 — OntoCode explorer (shipped)

**Theme:** Browse ontologies in VS Code via a language server.

**UI phases delivered:** **1** (partial) — explorer trees, entity inspector, VS Code command palette.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | LSP process; workspace indexing command |
| **OntoCode / OntoUI** | VS Code extension skeleton; ontology explorer; class/property/individual trees; entity inspector; jump to source; hover, go-to-definition, document/workspace symbols |

**Exit criteria:** User can browse an ontology repo in VS Code.

**Dependencies:** `lsp-server`, `lsp-types`, OntoCore crates

---

### v0.3 — Diagnostics (shipped)

**Theme:** Surface ontology quality issues inline.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Parse errors; broken imports; undefined prefixes; duplicate labels; missing labels; orphan classes; `diagnostics` SQL virtual table |
| **OntoCode** | Problems panel integration |

**Exit criteria:** User gets useful ontology diagnostics inline.

**Dependencies:** `oxigraph` (parse errors); in-house lint rules in `ontocore-diagnostics`

---

### v0.4 — Write-back + Horned-OWL (shipped)

**Theme:** Edit Turtle ontologies without Protégé.

**UI phases delivered:** **2** (partial) — editable entity inspector, simple axiom forms.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-owl` crate (Horned-OWL catalog bridge); patch-based write-back; create/edit/delete classes, properties, individuals; edit labels, comments, simple `SubClassOf`, deprecated flag; Oxigraph ↔ Horned-OWL consistency tests; CLI `ontocore patch`; LSP `ontocore/applyAxiomPatch` |
| **OntoCode / OntoUI** | Editable entity inspector |

**Exit criteria:** User can edit labels and simple subclass axioms in Turtle; catalog axioms for editing come from Horned-OWL.

**Dependencies:** `horned-owl`, `horned-functional` via `ontocore-owl` ([ADR-0013](docs/design/adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0006](docs/design/adr/0006-patch-based-write-back.md))

---

### Era B — IDE depth (v0.5–v0.8)

### v0.5 — Query workbench + Manchester MVP (shipped)

**Theme:** Query and author complex class expressions.

**UI phases delivered:** **3** — SQL/SPARQL editor, results table, query history, saved queries; Manchester editor MVP.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Manchester parse/serialize for `SubClassOf` and `EquivalentClasses`; LSP `ontocore/parseManchester`, `ontocore/query`, `ontocore/sparql` |
| **OntoCode / OntoUI** | SQL and SPARQL query webviews; saved queries, result export, query history; Manchester editor MVP |

**Exit criteria:** User can query ontologies in VS Code and edit complex subclass/equivalent axioms via Manchester.

**Dependencies:** `sqlparser`, `oxigraph`; Manchester in `ontocore-owl`

---

### v0.6 — Reasoning (shipped)

**Theme:** Rust-native OWL classification and inferred hierarchy.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-reasoner` crate (Ontologos facade); `el`, `rl`, `rdfs` adapters; profile detection; unsatisfiable classes; classification result cache |
| **OntoCode** | Reasoner panel; asserted/inferred/combined hierarchy toggle; explanation panel (EL-first) |

**Exit criteria:** User can classify EL ontologies, see inferred hierarchy, and get EL explanations where available.

**Dependencies:** Ontologos `ontologos-*` ([ADR-0015](docs/design/adr/0015-adopt-ontologos-reasoner.md))

---

### v0.7 — Visualization, React UI, OBO & ROBOT (shipped)

**Theme:** Rich IDE panels and biomedical toolchain interop.

**UI phases delivered:** **2** (partial), **4** (partial) — React entity inspector; class/property/import/neighborhood graphs.

Sub-phases: **v0.7a** (React foundation) → **v0.7** (graphs + inspector) → **v0.7b** (OBO + ROBOT).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-robot` wrappers (`validate`, `merge`, `report`); OBO format index; graph structure export via `petgraph` |
| **OntoCode / OntoUI** | `extension/webview-ui/` — Vite + React + TypeScript; typed `postMessage` protocol; CSP-compliant panel host; class/property/import/neighborhood graphs; entity inspector on React; OBO syntax highlighting and id rendering in explorer |

**Exit criteria:** User can navigate ontologies visually; biomedical maintainer can index OBO and run ROBOT in CI.

**Dependencies:** `react`, `vite` (extension); `fastobo`, `fastobo-owl`, `fastobo-validator`; ROBOT CLI ([ADR-0017](docs/design/adr/0017-react-webview-ui.md))

---

### v0.8 — Refactoring + full Manchester (shipped)

**Theme:** Safe large-scale ontology maintenance and full OWL 2 DL expression authoring.

**UI phases delivered:** **6** — refactor preview panel; rename/find references in explorer.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Safe IRI rename, namespace migration, find usages, move entity, extract module; preview/apply refactor plan; full Manchester axiom catalog (restrictions, disjoint, property chains view) |
| **OntoCode / OntoUI** | Refactor preview panel; Query Workbench and Manchester editor migrated to React; LSP rename and find references |

**Exit criteria:** User can safely refactor ontology repositories and author full OWL 2 DL expression sets via hybrid UI.

**Dependencies:** `horned-owl`, `horned-functional`; React webview UI

---

### Era C — Platform & authoring (v0.9–v0.12)

### v0.9 — OntoCore platform identity (shipped)

**Theme:** Unified naming, public API, and DL reasoning.

**UI phases delivered:** **5** (partial) — reasoner panel; asserted/inferred/combined hierarchy; EL explanations.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Rename `ontoindex-*` → `ontocore-*`; CLI `ontocore`; LSP `ontocore-lsp` with `ontocore/*` methods; `ontocore` façade crate with `Workspace` API; Ontologos 1.0.0 integration (`dl` and `auto` adapters); plugin platform design ([PLUGIN_SPEC.md](docs/design/PLUGIN_SPEC.md)) |
| **OntoCode / OntoUI** | Reasoner + explanation panels on React; legacy HTML webviews removed; OntoCore branding |

**Exit criteria:** Contributors and users distinguish OntoCore (engine) from OntoCode (IDE); DL/auto classification enabled.

**Dependencies:** Ontologos 1.0.0; breaking release for v0.8 integrators ([ADR-0018](docs/design/adr/0018-ontocore-platform-identity.md))

---

### v0.10 — Semantic workspace (shipped)

**Theme:** Team-scale development workflows.

**UI phases delivered:** **9** (partial) — semantic diff React panel.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Incremental indexing (content-hash reuse); multi-root workspaces; stable `ontocore::Workspace` API; `ontocore-diff` crate; `ontocore diff` CLI; git ref compare; breaking-change detection; optional disk cache (`.ontocore/cache/`) |
| **OntoCode / OntoUI** | Semantic diff React panel; LSP `ontocore/semanticDiff`; multi-root folder support |

**Exit criteria:** User can diff ontology versions, work in multi-root workspaces, and reindex incrementally at scale.

**Dependencies:** `git2`, `horned-owl`, `pulldown-cmark`, `minijinja`

---

### v0.11 — Editor depth & distribution (shipped)

**Theme:** Close editor gaps and expand distribution.

**UI phases delivered:** **5** (quick fixes), **7** (partial — `ontocore docs` export), **11** (partial — enterprise adoption guides).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | LSP `textDocument/completion` (Turtle prefix, QName, IRI); diagnostic quick fixes (`undefined_prefix`, `missing_label`, `broken_import`); `ontocore-docs` crate; `ontocore docs` CLI (Markdown/HTML); `add_import` / `remove_import` patch ops; OBO read path via `fastobo` (synonyms, defs, xrefs); ADR for v1.0 OBO write-back ([ADR-0019](docs/design/adr/0019-obo-write-back.md)) |
| **OntoCode / OntoUI** | Manage Imports panel; Open VSX publishing (Cursor); diagnostic code actions; entity inspector panel reuse on navigation; VS Code e2e tests |

**Exit criteria:** Daily Turtle editing, import management, and docs export work without leaving VS Code; extension available on VS Code Marketplace and Open VSX.

**Dependencies:** `fastobo`; `minijinja`, `pulldown-cmark`

---

### v0.12 — Authoring parity (shipped)

**Released:** v0.12.0 (2026-07-06)

**Theme:** Close remaining **P0** OWL and OBO authoring gaps from [PROTEGE_PARITY.md](docs/design/PROTEGE_PARITY.md).

**UI phases delivered:** **2** (P0 exit) — domain/range/characteristics forms, property chain editor, OBO edit forms, preview-before-apply on all axiom types.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Property domain, range, and characteristics authoring (patch ops); individual class/property assertions; expanded annotation assertion editing; property chain editing; full OBO write-back ([OBO_ROBOT_SPEC.md](docs/design/OBO_ROBOT_SPEC.md), [ADR-0019](docs/design/adr/0019-obo-write-back.md)); OWL/XML read support; Horned-OWL → Ontologos bridge improvements; axiom round-trip golden tests (Protégé fixtures) |
| **OntoCode / OntoUI** | Inspector forms for domain/range/characteristics; property chain editor; OBO write-back in inspector; DL clash-trace explanations via `ontologos-explain` + `ontologos-dl`; Turtle preview before apply for all axiom types |

**Exit criteria:** All **P0 — OWL 2 DL authoring** and **OBO & biomedical** rows in [PROTEGE_PARITY.md](docs/design/PROTEGE_PARITY.md) are green.

**Dependencies:** `horned-owl`, `horned-functional`, `fastobo`, `fastobo-owl`; Ontologos `ontologos-explain`

---

### Era D — OntoUI platform (v0.13)

### v0.13 — Platform hardening (shipped)

**Released:** v0.13.0 (2026-07-08)

**Theme:** OntoUI platform foundation + OntoCore hardening for plugins (v0.14) and Protégé polish (v1.0).

**UI phases delivered:** **0**, **1**, partial **3** (schema browser), partial **5** (reasoning store integration), partial **9** (PR summary CLI). Checklist: [ROADMAP_MAPPING.md § v0.13](docs/ui/ROADMAP_MAPPING.md)

| Area | Deliverables |
|------|--------------|
| **OntoUI** | `WorkspaceHost` adapter; Zustand `WorkspaceStore` + event bus; Current Focus + `FocusChanged`; `WorkspaceRegistry`; design tokens + shared primitives; extension-host `FocusRelayService`; Entity Inspector, Graph, Query Workbench, and Refactor Preview on store; schema browser in Query Workbench; refactor + reasoning store slices |
| **OntoCore** | Horned-OWL SQL virtual tables (`restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`, `domain_axioms`, `range_axioms`); LSP `ontocore/listSqlSchema`; `ontocore diff --pr-summary`; `.ontocore/diagnostics.toml`; LSP semantic tokens (Turtle/OBO); `ontocore docs` class hierarchy + property index; API stability policy; benchmark smoke tests (`tests/bench_index.rs`) |
| **OntoCode** | Cross-panel focus sync (explorer → inspector + graph); Vitest + extension integration tests for focus relay, schema browser, and store slices; UX audit ([UX_AUDIT_v0.13.md](docs/ui/UX_AUDIT_v0.13.md)) |

**Exit criteria (met):**

- [x] **Focus sync** — explorer selection relays to inspector and graph via extension-host focus relay
- [x] **Store ownership** — Entity, Graph, Query, and Refactor Preview workspaces consume WorkspaceStore
- [x] **Design system** — design tokens + shared primitives on Entity Inspector and Query Workbench
- [x] **Schema browser** — browse virtual tables/columns; insert snippets into Query Workbench editor
- [x] **Team workflow** — `ontocore diff A..B --pr-summary` emits PR-ready Markdown; documented and tested
- [x] **Performance** — benchmark fixtures + sizing guide update
- [x] **Quality** — 161 webview-ui Vitest tests + extension integration tests; accessibility pass on migrated panels
- [x] **API policy** — public `ontocore` API stability documented on path to 1.0

**Deferred to later releases:** persistent tabs + bottom dock, full panel component migration, SQL `JOIN`/`GROUP BY`, PR summary UI panel, validation report panel, Reasoner/Semantic Diff full store migration → **v1.0** / **v1.2** per original scope boundaries.

**Dependencies:** `sqlparser` + `horned-owl` ([ADR-0011](docs/design/adr/0011-use-sqlparser-for-sql.md), [ADR-0013](docs/design/adr/0013-dual-stack-oxigraph-horned-owl.md)); platform ADRs [0002–0004](docs/adr/README.md)

---

### v0.14 — Plugin host MVP (shipped)

**Released:** v0.14.0 (2026-07-09)

**Theme:** External extensibility without embedding workflow engines in core.

**UI phases delivered:** **8** (plugin platform MVP). Milestone: [Product Roadmap 2.0 phase 8](docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `PluginHost` runtime; manifest discovery from `.ontocore/plugins/`; reference plugins (naming validator, Markdown exporter, SHACL scaffold); CLI `plugins list/run`, `validate`/`docs` hooks; LSP `listPlugins`/`runPlugin`; subprocess workflow runner |
| **OntoUI** | Capability provider registry; inspector plugin cards; `WorkspaceStore` plugins slice |
| **OntoCode** | Plugin commands; owlmake workflow scaffold (`workflow run --plugin owlmake`); workflow output panel |
| **Ecosystem** | `examples/plugin-workspace/` fixture; [Plugin authoring guide](docs/guides/plugins.md) |

**Exit criteria (met):**

- [x] Third party can ship a validation or export plugin without forking OntoCore (reference plugins + manifest schema)
- [x] owlmake can be invoked from OntoCode as an external workflow (subprocess scaffold)

**Dependencies:** `ontocore-plugin` crate; reference plugin binaries; [PLUGIN_SPEC.md](docs/design/PLUGIN_SPEC.md)

---

## Pre-1.0 closeout + planned releases (v0.15 → v1.2+)

---

### Era E — Desktop UX shell gate (v0.15–v0.18)

> **Scope note:** These phases target **Protégé Desktop parity** only. WebProtégé parity (live collaboration, permissions, notifications, etc.) remains post-1.0.

### v0.15 — Plugin API + visualization parity + explanation workspace (shipped)

**Released:** v0.15.0 (2026-07-08)

**Theme:** Turn the v0.14 plugin host MVP into a stable extensibility surface, and close the biggest remaining “daily Protégé” UX gaps around visualization and explanations.

**Primary specs:**
- Plugin API: [docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/API.md](docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/API.md)
- Visualization: [docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/OWLVIZ.md](docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/OWLVIZ.md), [docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/ONTOGRAF.md](docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/ONTOGRAF.md), [docs/PROTEGE_REVERSE_ENGINEERING/WORKFLOWS/VISUALIZATION.md](docs/PROTEGE_REVERSE_ENGINEERING/WORKFLOWS/VISUALIZATION.md)
- Explanations + reasoning workflow: [docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/EXPLANATION.md](docs/PROTEGE_REVERSE_ENGINEERING/PLUGINS/EXPLANATION.md), [docs/PROTEGE_REVERSE_ENGINEERING/REASONING/INFERENCE_WORKFLOWS.md](docs/PROTEGE_REVERSE_ENGINEERING/REASONING/INFERENCE_WORKFLOWS.md)

| Area | Deliverables |
|------|--------------|
| **OntoCore** | **Plugin API v0** (versioned contract + capability discovery + lifecycle hooks + permission declarations); subprocess constraints hardened (timeouts, output caps, workspace path jail); explanation APIs exposed through LSP/CLI (request explanation, cancel, progress, caching); inference artifacts structured for UI consumption (unsats, inferred edges, justifications) |
| **OntoUI** | **Plugin contributions beyond “cards”**: plugin-defined commands, views, context actions, and preferences pages (wired through a central command registry + event bus); **Explanation workspace MVP** (justification list + details + navigation); graph workspace upgrades toward **OWLViz + OntoGraf parity** (asserted/inferred modes, multiple layouts, filters, search, incremental expansion, export) |
| **OntoCode** | Command palette / menus surface plugin commands; graph and explanation workspaces participate in focus sync and selection synchronization; UX polish for reasoning “dirty” state, progress, cancellation, and stale-explanation detection/regeneration |
| **Ecosystem** | Reference plugins updated to new API surface; plugin authoring guide updated with commands/views/preferences examples; fixtures that validate OWLViz-style and explanation workflows |

**Exit criteria:**
- Plugin can register **at least one dockable view** and **one command** via a versioned contract (no private UI coupling), and be safely disabled/unloaded.
- User can open a **class hierarchy graph** with **asserted vs inferred** modes, pan/zoom/search/center, and navigate back to the entity editor.
- User can generate an explanation for an **unsatisfiable class** with **multiple alternative justifications**, click through supporting axioms/entities, and detect/regenerate stale explanations after edits.

---

### v0.16 — Workspace layouts + preferences + imports polish (shipped)

**Released:** v0.16.0 (2026-07-09)

**Theme:** Close the “desktop shell” parity gap: plugin preferences, context actions, and imports/layout polish in the VS Code extension.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Plugin command dispatch via LSP `ontocore/runPlugin` (validator/export/workflow) |
| **OntoUI** | Plugin preferences pages and context actions surfaced from manifest contributions |
| **OntoCode** | **Plugins: Open Preferences…**, **Plugins: Run Context Action…**, **Reload Imports**, **Reset Layout** commands |
| **Ecosystem** | [v0.16 scope](docs/design/v0.16_SCOPE.md); [migration/v0.16.md](docs/migration/v0.16.md) |

**Exit criteria (partial):** P0 preferences/context actions/imports reload shipped; full layout persistence and workspace perspectives deferred to v0.17+.

See [migration/v0.16.md](docs/migration/v0.16.md) and [SHIPPED.md](docs/SHIPPED.md).

---

### v0.17 — Menus/toolbars/dialog parity + keyboard-first workflows (shipped)

**Released:** v0.17.0 (2026-07-10)

**Theme:** Make every Protégé “menu action” and common dialog-driven workflow available in OntoCode’s command system with strong keyboard and context-menu affordances.

**Primary specs:**
- Menus: [docs/PROTEGE_REVERSE_ENGINEERING/UI/MENUS.md](docs/PROTEGE_REVERSE_ENGINEERING/UI/MENUS.md)
- Toolbars: [docs/PROTEGE_REVERSE_ENGINEERING/UI/TOOLBARS.md](docs/PROTEGE_REVERSE_ENGINEERING/UI/TOOLBARS.md)
- Dialogs: [docs/PROTEGE_REVERSE_ENGINEERING/UI/DIALOGS.md](docs/PROTEGE_REVERSE_ENGINEERING/UI/DIALOGS.md)

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Stable command metadata + enablement predicates (dirty state, selection state, reasoner state, read-only imports); undo/redo action labels for semantic edits; structured dialog schemas for “new ontology / import / export / prefix manager / metadata” flows |
| **OntoUI** | Menu + toolbar surfaces backed by a centralized command registry; entity/context menus across trees, axiom lists, search results; keybinding registry + conflict detection; core dialogs parity (new/open/save-as/import/export, rename, delete confirmation with impact summary, prefix manager, ontology metadata, reasoner settings, metrics/search) |
| **OntoCode** | Command palette parity for menu/context actions; keyboard navigation through dialogs and toolbars; help/about + error log viewer + plugin info surfaces |

**Exit criteria:**
- Every high-frequency Protégé action in the reverse-engineered checklists is accessible via **command palette + menus/toolbars**, with correct enablement and keyboard shortcuts.
- Critical dialogs support live validation and do not allow invalid IRIs/prefixes/imports to be persisted.

See [migration/v0.17.md](docs/migration/v0.17.md) and [SHIPPED.md](docs/SHIPPED.md).

---

### v0.18 — Desktop UX shell gate + migration readiness (shipped)

**Released:** v0.18.0 (2026-07-11); patches **v0.18.1** (2026-07-12), **v0.18.2** (2026-07-13)

**Theme:** Close the desktop UX shell gate (menus, layouts, workflows, migration docs). **Not** full functional Protégé parity — that is the objective of [v0.19–v0.25](docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md).

**Scope (docs/audit-first):** [v0.18_SCOPE.md](docs/design/v0.18_SCOPE.md) · [0.18 parity assessment](docs/PROTEGE_REVERSE_ENGINEERING/ONTOCODE_PARITY/ONTOCODE_0.18_PROTEGE_PARITY_ASSESSMENT.md)

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Parity regression fixtures (round-trip + workflow-level); performance hardening for large ontologies (virtualized trees/graphs, incremental refresh where possible); reasoning/explanation reliability (caching, cancellation, progress); import/serialization edge-case coverage |
| **OntoUI / OntoCode** | Parity audits against the reverse-engineered checklist (workspace shell, menus/toolbars/dialogs, views, explanations, visualization); accessibility pass on remaining panels; migration guide drafts (“from Protégé to OntoCode”) with honest known-gap list (desktop only) |

**Exit criteria:**
- **Desktop UX shell gate = 100%** for the agreed pre-1.0 scope (desktop only), backed by the reverse-engineering specs and fixture-driven regression checks.
- A working “Protégé → OntoCode” migration path is documented for common workflows (modeling, reasoning, explanations, visualization, imports, preferences).

See [migration/v0.18.md](docs/migration/v0.18.md) and [SHIPPED.md](docs/SHIPPED.md).

---

### Era F — Full Protégé parity path (v0.19–v0.25)

**Canonical plan:** [PRE_1_0_PHASES.md](docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) · **Sequencing:** [EXECUTION_ORDER.md](docs/protege-parity/05_IMPLEMENTATION/EXECUTION_ORDER.md) · **Status:** [PARITY_STATUS.md](docs/protege-parity/03_PARITY/PARITY_STATUS.md)

v0.18 closed the desktop UX shell gate. **v0.19 shipped** the semantic transaction layer and parity program baseline. v0.20–v0.25 implement the remaining P0 blockers defined in [docs/protege-parity/](docs/protege-parity/README.md).

### v0.19 — Semantic foundation + program baseline (shipped)

**Released:** v0.19.0 (2026-07-13)

**Theme:** Freeze parity scope; route Turtle/OBO edits through semantic transactions.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `ontocore-edit` crate (`SemanticChange`, `Transaction` compose/validate/invert); Turtle/OBO LSP + CLI apply via transactions; format adapters over existing patch engines |
| **Parity program** | Frozen scope; machine-readable `parity/protege-desktop-parity.yaml` + CI validator; GitHub epics EPIC-001…011 |
| **Docs** | ADR-0020; matrix/evidence/status sync; [migration/v0.19.md](docs/migration/v0.19.md) |

**Exit criteria:** All supported Turtle/OBO edits flow through semantic transactions; Turtle/OBO regression-free; parity status reproducible from evidence. *(RDF/XML and OWL/XML write-back remain v0.21.)*

**Blockers addressed:** [BLOCKER_01](docs/protege-parity/04_BLOCKERS/BLOCKER_01_FORMAT_INDEPENDENCE.md) (transaction foundation), [BLOCKER_11](docs/protege-parity/04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md) (manifest skeleton)

See [migration/v0.19.md](docs/migration/v0.19.md) and [SHIPPED.md](docs/SHIPPED.md).

---

### v0.20 — Workspace runtime (shipped)

**Status:** **Shipped** as tagged **v0.20.0**. Canonical RTD copy: [docs/roadmap.md](docs/roadmap.md).

**Theme:** Workspace as central runtime for ontology state and transactions.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | `undo_patches` on `applyAxiomPatch`; registry snapshot in `getWorkspaceUiState` |
| **OntoCode** | Ontology registry; dirty/save coordinator; transaction manager + event bus; selection/navigation; session persistence; external-change recovery |
| **Also shipped** | Turtle/OBO patch hardening ([#286](https://github.com/eddiethedean/strixonomy/pull/286)–#291) |

**Exit criteria:** Multi-ontology workflows pass end-to-end tests; workspace state survives restart — met.

**Blockers:** [BLOCKER_03](docs/protege-parity/04_BLOCKERS/BLOCKER_03_WORKSPACE.md) — EPIC [#249](https://github.com/eddiethedean/strixonomy/issues/249)

See [migration/v0.20.md](docs/migration/v0.20.md).

---

### v0.21 — Required format write-back (shipped)

**Theme:** RDF/XML and OWL/XML semantic round-trip.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | RDF/XML and OWL/XML serializer adapters; cross-format semantic comparator; Protégé fixture corpus |

**Exit criteria:** Turtle, OBO, RDF/XML, OWL/XML open → edit → save → reload without semantic loss.

**Shipped:** ADR-0021, Horned writers, `ontocore-edit` XML adapters, CLI/LSP/editable gates, PAR-FMT-003/004 closed.

**Blockers:** [BLOCKER_01](docs/protege-parity/04_BLOCKERS/BLOCKER_01_FORMAT_INDEPENDENCE.md), [format audit](docs/protege-parity/02_PROTEGE_AUDIT/PROTEGE_FILE_FORMAT_AUDIT.md)

See [migration/v0.21.md](docs/migration/v0.21.md).

---

### v0.22 — Complete OWL 2 authoring (shipped)

**Status:** **Shipped** as tagged **v0.22.0**.

**Theme:** Every P0 OWL 2 construct authorable across required formats.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Missing TBox/RBox/ABox axioms; keys, datatypes, restrictions, axiom annotations |
| **OntoCode** | Structured editors; validation; undo/redo coverage |

**Exit criteria:** All P0 OWL 2 constructs VERIFIED end-to-end.

**Blockers:** [BLOCKER_02](docs/protege-parity/04_BLOCKERS/BLOCKER_02_OWL2_AUTHORING.md)

See [migration/v0.22.md](docs/migration/v0.22.md).

---

### v0.23 — Reasoning parity + SWRL (shipped)

**Theme:** TBox/ABox reasoning workflows and SWRL subsystem.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Realization, instance checking, native DL explanations; SWRL parse/serialize/validate |
| **OntoCode** | Rule browser/editor; inferred/asserted views |

**Exit criteria:** Reasoning and SWRL P0 requirements VERIFIED.

**Blockers:** [BLOCKER_04](docs/protege-parity/04_BLOCKERS/BLOCKER_04_REASONING.md), [BLOCKER_05](docs/protege-parity/04_BLOCKERS/BLOCKER_05_SWRL.md)

---

### v0.24 — Semantic services completion (shipped)

**Theme:** Refactoring and query/search parity on stable semantic core.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Merge, module extraction, SWRL-aware refactor; DL Query; semantic search |
| **OntoCode** | Refactor preview; query export and navigation |

**Exit criteria:** Refactoring and query P0 requirements VERIFIED.

**Blockers:** [BLOCKER_06](docs/protege-parity/04_BLOCKERS/BLOCKER_06_REFACTORING.md), [BLOCKER_07](docs/protege-parity/04_BLOCKERS/BLOCKER_07_QUERY.md)

---

### v0.25 — UX completion + executable verification (shipped)

**Released:** v0.25.0 (2026-07-15)

**Theme:** Visualization, plugin SDK 1.0, accessibility, parity manifest CI.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Graph model completion; plugin SDK freeze; parity manifest + CI release gate |
| **OntoCode** | Accessibility audit closed; conformance suite aggregation |

**Exit criteria:** Plugin SDK 1.0 stable; every P0 requirement has automated CI evidence.

**Blockers:** [BLOCKER_08](docs/protege-parity/04_BLOCKERS/BLOCKER_08_VISUALIZATION.md)–[BLOCKER_11](docs/protege-parity/04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md)

See [migration/v0.25.md](docs/migration/v0.25.md) and [SHIPPED.md](docs/SHIPPED.md).

---

### v0.26 — Protégé Desktop test port (shipped)

**Released:** v0.27.0 (2026-07-17)

**Theme:** Port portable Protégé Desktop JUnit behaviors into OntoCode Rust semantic oracles (not a JVM suite runner).

| Area | Deliverables |
|------|--------------|
| **OntoCore / tests** | Inventory [`parity/protege-test-port.yaml`](parity/protege-test-port.yaml); Wave 1–4 suites `tests/protege_port_*.rs` (edit oracles + render/links + utils/Foundry + IdPolicy + `catalog-v001`); fixtures under [`examples/protege-roundtrip/ported/`](examples/protege-roundtrip/ported/); `ontocore-owl` `render`/`links`/`util`; `ontocore-obo` `obofoundry`/`idpolicy`; `ontocore-catalog` `xml_catalog`; inspector annotation order + linkification; LSP hover linkification; `OntologyDocument.version_iri` |
| **Verification** | [`scripts/validate-protege-test-port.py`](scripts/validate-protege-test-port.py); CI + local CI wiring; `PAR-*` `test_ids` linkage |

**Exit criteria (met):** Every `PORT_W1`/`PORT_W2`/`PORT_W3`/`PORT_W4` inventory row has OntoCode tests (or an explicit `gap`); Wave 1–4 suites green in CI.

**Docs:** [PROTEGE_TEST_PORT.md](docs/protege-parity/03_PARITY/PROTEGE_TEST_PORT.md)

**Shipped follow-ons (full Upstream-audit scope, minus gated Fresh-entity UX):**

| Item | Status |
|------|--------|
| IdPolicy semantic oracles (`idpolicy/*.owl`) | Done — `ontocore-obo` + `tests/protege_port_idpolicy.rs` |
| `catalog-v001.xml` import redirects | Done — `xml_catalog` + `resolve_import_document` + `PORT_W4` |
| Fixture hardening (`twoEq`, AmbiguousName, version IRI) | Done |
| Expanded Foundry JSON stress | Done — `obofoundry_expanded.json` |
| Fresh-entity / IdRange / Edge product ports | Still SKIP (no auto-ID UX) |

**Still not useful from Desktop:** OSGi bundle tests, Mac AWT, PreferencesManager, SelectionPlane/PopupMenuId, JVM suite runner, byte-identical XML.

See [migration/v0.26.md](docs/migration/v0.26.md) and [SHIPPED.md](docs/SHIPPED.md).

---

### v0.27 — Strixonomy rename (shipped)

**Released:** v0.27.0 (2026-07-27)

**Theme:** Rename the OntoCore Rust engine and OntoCode VS Code extension to **Strixonomy**.

| Area | Deliverables |
|------|--------------|
| **Rust packages** | Publish the `strixonomy` façade and corresponding `strixonomy-*` crates; rename package metadata, Rust import paths, feature references, examples, and generated documentation |
| **Binaries and protocol** | Rename the CLI to `strixonomy` and language server to `strixonomy-lsp`; update release archives, environment variables, logs, User-Agent strings, protocol branding, and bundled-binary discovery |
| **VS Code extension** | Publish **Strixonomy for VS Code** under the Strixonomy publisher and extension identity; rename the activity-bar surface, commands, settings, view IDs, context keys, walkthroughs, icons, Marketplace/Open VSX metadata, telemetry-free diagnostics, and bundled server assets |
| **Compatibility** | Keep time-bounded `ontocore` crate/binary shims or migration packages where registries permit; accept legacy settings and workspace locations with deprecation messages; migrate saved webview state, plugin manifests, command invocations, and CI examples |
| **Distribution** | Reserve and verify GitHub, crates.io, npm, Marketplace, Open VSX, documentation, domain, and release-artifact namespaces before public cutover; document extension-listing migration and supply-chain ownership |
| **Documentation** | Add a v0.27 migration guide and mechanical rename table; update current product/install documentation while retaining historical release notes; `docs/TAGGED_RELEASE` and install pins set to **0.27.0** on the ship commit (tag the same day) |
| **Verification** | Add clean-install, v0.26-upgrade, CLI compatibility, LSP launch, extension-state migration, plugin compatibility, package publication dry-run, and stale-identifier audits to CI |

**Exit criteria:**

- A new user can install the extension, CLI, and Rust façade entirely through Strixonomy names.
- An existing v0.26 installation upgrades without silent loss of settings, workspace state, plugins, or automation.
- Legacy OntoCore/OntoCode identifiers either continue through documented compatibility shims or fail with an actionable migration message.
- Release artifacts and public documentation use Strixonomy as the primary identity; old names remain only in migration and historical contexts.

---

### 1.0.0-rc — Release candidate (planned)

**Theme:** Stabilize only — no new major features or scope changes.

**Exit criteria:** All P0 VERIFIED; all release gates pass; zero open P0 defects; APIs frozen.

---

### Era G — Protégé replacement (v1.0)

### v1.0 — Protégé-competitive release (planned)

**Theme:** Production-ready Protégé replacement in VS Code.

**UI phases:** **1–6** exit polish, partial **9** (review workspace). Milestones: [Product Roadmap 2.0 phases 2–6](docs/ui/PRODUCT_ROADMAP_2.0.md). Wireframes: [WORKSPACE_WIREFRAMES.md](docs/ui/WORKSPACE_WIREFRAMES.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | All [protege-parity P0 requirements](docs/protege-parity/03_PARITY/PARITY_RELEASE_GATE.md) green; all **P1** items green or documented known gaps; stable CLI/API/LSP semver 1.0; `examples/protege-roundtrip/` ontology set with workflow doc; performance benchmarks published |
| **OntoUI** | **[1]** Persistent tabs + bottom dock ([WORKSPACE_WIREFRAMES](docs/ui/WORKSPACE_WIREFRAMES.md)). **[2]** Relationship cards, references view, metadata view; entity workspace diagnostics integration ([ENTITY_EDITOR_SPEC](docs/ui/ENTITY_EDITOR_SPEC.md)). **[4]** Graph saved layouts, filters, reasoning overlays ([GRAPH_WORKSPACE](docs/ui/GRAPH_WORKSPACE.md)). **[5]** Semantic build pipeline UI; entity-level reasoning cards; reasoning history; Problems ↔ reasoning integration ([REASONING_EXPERIENCE](docs/ui/REASONING_EXPERIENCE.md)). **[6]** Merge classes; batch label normalization; undo/redo on refactor and patch writes ([SEMANTIC_REFACTORING](docs/ui/SEMANTIC_REFACTORING.md)). **[9]** Review workspace MVP ([COLLABORATION](docs/ui/COLLABORATION.md)). Supporting: HIG + keyboard shortcuts ([HUMAN_INTERFACE_GUIDELINES](docs/ui/HUMAN_INTERFACE_GUIDELINES.md), [KEYBOARD_SHORTCUTS](docs/ui/KEYBOARD_SHORTCUTS.md)) |
| **OntoCode** | Complete hybrid authoring loop (forms + Manchester + Turtle/OBO); full IDE surface (explorer, search, diagnostics, refactoring, query workbench, visualization, reasoning); React webview hardening; VS Code Marketplace + Open VSX publish as 1.0 |
| **Toolchain** | ODK project layout recognition (`src/ontology/`, catalog files, import structure); ODK QC and release workflow surfacing; ROBOT-compatible operations where practical; import existing ODK/ROBOT/owlmake workflows (Makefile, GitHub Actions); Protégé migration guide with honest parity table |
| **Ecosystem** | Ontologos 1.0.0 reasoner gate satisfied; published `ontocore` + `ontocore-*` 1.0.0 on crates.io |

**Already shipped (v0.5–v0.14):** entity editor MVP, query workbench, graph canvas, reasoner panel, refactoring preview, semantic diff, WorkspaceStore + focus relay, schema browser, plugin host MVP — see [ROADMAP_MAPPING.md](docs/ui/ROADMAP_MAPPING.md).

**Exit criteria:**

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features in [PARITY_SCOPE.md](docs/protege-parity/PARITY_SCOPE.md).

Track implementation: [PRE_1_0_PHASES.md](docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md) · [v1.0_BACKLOG.md](docs/design/v1.0_BACKLOG.md)

**Dependencies:** Ontologos 1.0.0; `react` / `vite` (extension `webview-ui`); [cursor-prompts/](docs/cursor-prompts/README.md) 06–07, 11–12

#### OntoCode 1.0 — Modern Protégé replacement

<a id="ontocode-10-modern-protege-replacement"></a>

##### Editing scope

- Complete ontology editing (classes, properties, individuals, annotations)
- Manchester syntax for complex expressions
- Turtle write-back (primary authoring format)
- OBO editing (read + write)
- Import management

##### IDE scope

- Explorer, search, diagnostics, refactoring
- Query workbench (SQL + SPARQL)
- Graph visualization (class, property, import, neighborhood)
- Reasoner panel with EL/RL/RDFS/DL/auto profiles and explanations

##### Toolchain integration scope

OntoCode integrates with the existing ontology toolchain through OntoCore — **not** by reimplementing ROBOT, ODK, or owlmake inside the engine.

- **owlmake** — first-class workflow plugin; build/release actions in IDE
- **ROBOT** — merge, reason, convert, validate via existing ROBOT semantics
- **ODK** — project layout, QC workflows, release workflows, zero-config repo open
- **Protégé migration** — import projects, preserve IRIs, guide users off desktop-only workflows

Ontologos provides **reasoning**. OntoCore provides the **workspace platform** and **plugin hosting**. owlmake and peers provide **workflow automation**. OntoCode presents all three in one IDE.

---

### Era H — Ecosystem expansion (v1.1+)

### v1.1 — Language bindings & AI primitives (planned)

**Theme:** Cross-language integration and AI-native tooling foundations.

**UI phases:** **7** (primary), deferred AI from **2**, **3**, **4**, **8**, **9**. Milestone: [Product Roadmap 2.0 phase 7](docs/ui/PRODUCT_ROADMAP_2.0.md). [ADR-0010](docs/design/adr/0010-ai-features-opt-in.md).

Former roadmap labels **v0.17 (Language Bindings)** and **v0.18 (AI Platform)** are consolidated here.

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Python SDK (workspace index, query, validate, diff); TypeScript SDK (LSP client helpers, webview protocol types); MCP server exposing workspace context (entities, axioms, diagnostics, query results) |
| **OntoUI** | **[7]** AI sidebar; inline suggestions; review ontology; repair diagnostics; project-wide AI tasks; MCP context bridge ([AI_EXPERIENCE](docs/ui/AI_EXPERIENCE.md), [AI_ORCHESTRATION_ARCHITECTURE](docs/ui/AI_ORCHESTRATION_ARCHITECTURE.md)). **[2†]** AI explain entity. **[3†]** AI query generation. **[4†]** AI graph explanations. **[8†]** AI provider API ([PLUGIN_API_SPEC](docs/ui/PLUGIN_API_SPEC.md)). **[9†]** AI review ([COLLABORATION](docs/ui/COLLABORATION.md)). All AI: read-only suggestions with preview/approval |
| **OntoCode** | MCP-driven semantic context for external AI tools; documentation generation hooks (extends v0.11 `ontocore docs`) |
| **Ecosystem** | Published SDK packages; MCP server installable via `cargo install` or pip |

**Exit criteria:** Python and TypeScript consumers can index and query ontologies without shelling to CLI; MCP clients can retrieve structured ontology context from an open workspace.

**Dependencies:** MCP protocol; PyO3 or subprocess bridge TBD in ADR; [cursor-prompts/09-add-ai-action-lifecycle.md](docs/cursor-prompts/09-add-ai-action-lifecycle.md)

---

### v1.2 — Ontology Toolchain Platform (planned)

**Theme:** Mature external workflow integration beyond the reference owlmake plugin.

**UI phases:** **9**, **10**, **11**. Milestones: [Product Roadmap 2.0 phases 9 + 11](docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Build API (compile/merge/materialize); Release API (version, tag, publish artifacts); Validation API (plug-in QC pipelines); plugin discovery and install from registry; semver-compatible plugin contracts |
| **OntoUI** | **[9]** GitHub integration; semantic PR summaries (UI panel); merge checks ([COLLABORATION](docs/ui/COLLABORATION.md)). **[10, 11]** Public plugin registry + marketplace UI; sample domain plugins ([PLUGIN_PLATFORM](docs/ui/PLUGIN_PLATFORM.md), [PLUGIN_SPEC](docs/design/PLUGIN_SPEC.md)). **[11]** Community templates. Workflow / QC status dashboard |
| **OntoCode** | Production-ready owlmake plugin integration; QC reports (HTML/Markdown/JSON) in IDE |
| **Ecosystem** | Official GitHub Actions for ontology CI/CD; plugin marketplace; third-party workflow plugins |

**Exit criteria:** ODK-style release pipeline runnable end-to-end from OntoCode with discoverable, versioned plugins.

---

### Era I — Webapp platform (v1.3+)

> **Scope note:** The webapp is a new product surface on the OntoCore platform, not a replacement for the VS Code extension. OntoCode remains the power-user IDE; the webapp prioritizes onboarding, review, shared workspaces, and browser-first collaboration.
>
> **Two product shapes (same OntoUI):**
> 1. **OntoCode Web (hosted)** — React browser app backed by OntoCore **service mode** (HTTP/WebSocket). Default for team workspaces, large ontologies, and collaboration.
> 2. **React app (no backend)** — static SPA that runs OntoCore entirely in-browser via **WASM**. No OntoCore server, no auth service, no remote API — open files/samples locally, browse, validate, query, and export. Ideal for demos, embeddings, offline review, and GitHub Pages–style static deploys.
>
> Both share TypeScript/OntoUI components and WorkspaceHost adapters; only the engine transport differs (service client vs WASM Worker).

### v1.3 — Webapp foundation (planned)

**Theme:** Ship OntoCode in the browser — hosted service path **and** a **React app (no backend)** powered by OntoCore WASM.

**UI phases:** **10**, partial **12**. Milestones: [Product Roadmap 2.0 phases 10–12](docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Long-running service mode; workspace/project API over HTTP or WebSocket; serialized catalog snapshots; query/validate/diff endpoints; server-side workspace sandboxing and resource limits. **WASM track (required by React app no-backend):** `ontocore-wasm` (or equivalent) crate targeting `wasm32`; worker-friendly JS/TS bindings for open/index snapshot, browse catalog, validate, and run constrained SQL/SPARQL; virtual FS / in-memory workspace for file blobs (no native disk); documented capability matrix vs native (reasoning/plugins/ROBOT may be reduced or deferred); size/perf budget and CI smoke tests for browser load |
| **OntoUI** | Shared React component package usable by VS Code webviews, hosted OntoCode Web, and the **React app (no backend)**; responsive shell; browser-safe graph/query/inspector workspaces; design tokens aligned across surfaces; WorkspaceHost adapters for service mode and for WASM (Worker message bus) |
| **OntoCode Web (hosted)** | Browser app MVP against service mode: open sample/project workspace, browse entities, run validation, run SQL/SPARQL, view semantic diffs, export reports; read-only by default with explicit edit preview gates |
| **React app (no backend)** | First-class static product: Vite/React SPA that embeds OntoCore WASM; open local ontology files or bundled samples; entity explorer, inspector, Query Workbench, validation, and semantic diff **with zero server dependency**; deployable as static assets (CDN / GitHub Pages); documented limits vs hosted mode; same OntoUI shell as hosted Web where capabilities overlap |
| **Ecosystem** | Deployment guide for local server, container image, single-user hosted instance, **and** static React SPA + WASM assets; API docs for embedding catalog/query/review views; published npm package wrapping WASM bindings; template repo or `create-` starter for embedding the React no-backend app |

**Exit criteria:** Without installing VS Code, a user can (a) use **OntoCode Web (hosted)** via OntoCore service mode, or (b) open the **React app (no backend)** from a static URL or local static server, load a sample or local file through WASM, and in either shape inspect entities, run validation/query, and review semantic diffs. Path (b) must work with **no** OntoCore backend process.

**Dependencies:** Stable OntoCore 1.x API; TypeScript SDK from v1.1; plugin/toolchain APIs from v1.2; ADR for hosted service security and workspace isolation; **ADR for OntoCore WASM crate boundaries** (which subsystems compile for `wasm32`, Worker vs main-thread, FS/network stubs, and deferred features); Vite/static-hosting packaging ADR for the React no-backend app.

---

### v1.4 — Team workspaces (planned)

**Theme:** Shared ontology projects for review, curation, and lightweight browser editing.

**UI phases:** **9**, **12**. Milestones: [Product Roadmap 2.0 phases 9 + 12](docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Project metadata store; branch/snapshot model; review annotations; proposal/apply workflow for semantic patches; audit log primitives; WASM feature expansion for lightweight browser edits (label/comment/simple axiom patches) with patch export that matches service-mode semantic patch format |
| **OntoUI** | Review workspace; comment threads on entities/axioms/diffs; assignment/status filters; browser editor forms for labels, comments, simple class/property changes, and controlled patch application |
| **OntoCode Web** | Team project dashboard; invite/share links; role-aware project views; pull-request-style semantic review; report publishing for validation, reasoning, and release workflows |
| **React app (no backend)** | Optional offline edit preview: apply simple patches locally via WASM, export semantic patch / Turtle diff for later apply in hosted Web or VS Code; remains deployable as pure static assets |
| **Ecosystem** | GitHub/GitLab integration for branch import/export; webhook support for CI validation and release reports |

**Exit criteria:** A small ontology team can review, comment on, approve, and apply ontology changes in the browser while preserving Git-backed provenance.

**Dependencies:** Authentication and authorization ADR; collaboration data model; semantic patch proposal format shared with VS Code.

---

### v1.5+ — Cloud collaboration + governance (planned)

**Theme:** Production multi-user ontology engineering with live collaboration and organizational controls.

**UI phases:** **9**, **12**. Milestones: [Product Roadmap 2.0 phases 9 + 12](docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Multi-tenant deployment profile; collaborative session protocol; conflict detection/merge for semantic edits; scalable reasoning/query workers; policy hooks for governance workflows; optional hybrid clients (WASM for local preview + service workers for shared sessions) |
| **OntoUI** | Live presence; shared semantic canvases; real-time review state; governance dashboards; large-ontology performance modes for browser graph and table views |
| **OntoCode Web** | Organization workspaces; permissions; notifications; live edit/review sessions; hosted release dashboards; governance workflows for approval, deprecation, publication, and compliance evidence |
| **React app (no backend)** | Remains available as a static offline/review surface; optional hybrid mode where the same React shell can attach to a hosted session when the user opts in (default stays no-backend) |
| **Ecosystem** | Enterprise deployment patterns; identity provider integration; observability; backup/export guarantees; hosted plugin/runtime policy |

**Exit criteria:** Ontology teams can run a governed browser-first workflow for collaborative editing, review, validation, release, and audit without relying on a desktop-only toolchain.

**Dependencies:** Multi-tenant security review; persistence architecture; hosted plugin sandboxing; operational runbooks.

---

### Post-v1.5 — Ecosystem modernization (planned)

**Theme:** Shift from Protégé parity to ecosystem leadership.

**UI phases:** **10**, **12**, plus collaboration items from **9**. Milestones: [Product Roadmap 2.0 phases 10–12](docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **OntoCore** | Semantic workspace APIs (persistent semantic databases); plugin marketplace maturity; advanced graph analytics; webapp service/runtime hardening from v1.3–v1.5; WASM parity hardening (reasoning profiles, larger workspaces, publishable npm/CDN packages) |
| **OntoUI** | **[10]** OntoStudio Tauri app shell; shared React UI (OntoCode + OntoStudio + OntoCode Web + React app no-backend); native graph performance; local AI support; enterprise packaging ([ONTOSTUDIO_DESKTOP](docs/ui/ONTOSTUDIO_DESKTOP.md), [GRAPH_RENDERING_ARCHITECTURE](docs/ui/GRAPH_RENDERING_ARCHITECTURE.md), [COMPONENT_LIBRARY](docs/ui/COMPONENT_LIBRARY.md)). **[12]** Browser client (hosted service + React app no-backend / WASM); cloud sync; team workspaces; distributed reasoning; shared semantic canvases; governance workflows ([PLATFORM_ARCHITECTURE](docs/ui/PLATFORM_ARCHITECTURE.md), [COLLABORATION](docs/ui/COLLABORATION.md), [GRAPH_WORKSPACE](docs/ui/GRAPH_WORKSPACE.md), [governance](docs/guides/governance.md)). **[9, 12]** Live collaboration; ontology PR review; advanced visualization (large-graph layout, temporal diff) |
| **OntoCode** | AI-assisted ontology engineering (modeling suggestions, axiom completion); live collaboration; ontology review in pull requests; deep links between VS Code workspaces and OntoCode Web reviews |
| **Ecosystem** | Enterprise governance tooling; knowledge graph tooling integrations; documentation generators via plugin APIs |

**Strategic framing:** OntoCore provides the platform. owlmake (and peers) provide workflow, build, and release automation. OntoCode surfaces both through the UI. The goal is ecosystem collaboration — not absorbing or replacing every tool in the stack.

---

## Long-term goal

OntoCore becomes the foundation for modern ontology tooling.

OntoCode becomes the flagship IDE.

Ontologos becomes the flagship Rust reasoning engine.
