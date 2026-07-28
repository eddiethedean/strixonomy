# Strixonomy Roadmap

> **Evaluators:** start with **[Roadmap summary](roadmap-summary.md)** and **[What ships today](SHIPPED.md)** — this page is the full engineering timeline (661 lines), not a capability matrix.

> **Canonical full roadmap:** [ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md) (includes Era I webapp / WASM / React app no-backend detail).  
> **This RTD page** is a condensed summary for ships + near-term phases.  
> **Which roadmap?** [Roadmap hub](roadmap-hub.md). **What ships today?** [SHIPPED.md](SHIPPED.md).

## Vision

Build the modern open-source platform for ontology engineering.

**Strixonomy engine** is the semantic workspace (CLI, LSP, crates).

**Strixonomy IDE** is the flagship VS Code extension powered by the engine.

Full mission and principles: [Vision](vision.md). Ecosystem layers: [Architecture](architecture.md). Names: [Product identity](guides/product-identity.md).

## Guiding principle

**Strixonomy IDE v0.30 has one primary objective: become a production-ready replacement for Protégé.**

Every feature before v0.30 should answer one question:

> Does this make it easier for ontology engineers to adopt Strixonomy instead of Protégé?

After v0.30, the roadmap shifts from parity to modernization.

!!! warning "Not a Protégé replacement today"
    **v0.28** supports pilot and coexistence workflows — not org-wide Protégé retirement. See [What ships today](SHIPPED.md) and [Known limitations](known-limitations.md) before planning format or IDE migration.

---

## How to read this document

| Document | Role |
|----------|------|
| [What ships today](SHIPPED.md) | **Canonical capability matrix** — what is available in the current release |
| [Protégé parity program](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/README.md) | **v0.30 engineering program** — scope, blockers, release gates |
| [v0.29–v0.30 release phases](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md) | **v0.19–v0.30** versioned parity plan |
| [UI roadmap mapping](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md) | **UI specs ↔ releases** — master checklist for all Product Roadmap 2.0 items |
| [Milestones (shipped)](design/ROADMAP.md) | Per-crate engineering detail for **shipped** v0.1–v0.11 milestones |
| [Protégé parity matrix](design/PROTEGE_PARITY.md) | Historical v0.18 P0/P1/P2 checklist (superseded for planning) |
| [v0.30 backlog](design/v0.30_BACKLOG.md) | Implementation checklist toward v0.30 |
| [Platform overview](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md) | OntoUI / WorkspaceStore architecture (foundation shipped v0.13) |
| [Product Roadmap 2.0](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md) | UI phases with milestone acceptance criteria |
| [Product design (UI)](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/README.md) | Product design specification pack (UX, design system, OntoStudio target) |

**Current release:** v0.28.0

---

## Release phases at a glance

### Timeline

```text
SHIPPED (v0.1–v0.28) ─────────────────────────────────────────────────►
v0.1–v0.4          v0.5–v0.8              v0.9–v0.12           v0.13–v0.28
Engine foundation    IDE depth                Platform & authoring   OntoUI → formats → OWL 2
                                                                       + reasoning/SWRL (v0.23)
                                                                       + refactor/DL Query (v0.24)
                                                                       + viz/SDK/a11y/CI (v0.25)
                                                                       + Protégé test port (v0.26)
                                                                       + Strixonomy rename (v0.27)
                                                                       + Python PyPI reservation (v0.28)

PLANNED (v0.29 → v0.38+) ─────────────────────────────────────────────►
v0.29      v0.30      v0.31      v0.32      v0.33–v0.38+
Trust       Author      Scale      Review     Automate · Extend · Reach
```

Full timeline: [ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md). v0.29–v0.30 phases: [V0_30_PHASES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md).

### Phase index

| Era | Versions | Status | North-star |
|-----|----------|--------|------------|
| **A — Engine foundation** | v0.1–v0.4 | Shipped | Index, browse, diagnose, edit Turtle |
| **B — IDE depth** | v0.5–v0.8 | Shipped | Query, reason, visualize, refactor |
| **C — Platform & authoring** | v0.9–v0.12 | Shipped | Strixonomy identity, semantic workspace, authoring parity |
| **D — OntoUI platform** | v0.13–v0.14 | Shipped | v0.13: WorkspaceStore, focus relay; v0.14: plugin host MVP |
| **E — Desktop UX shell gate** | v0.15–v0.18 | Shipped | Menus, layouts, workflows, migration readiness (not full parity) |
| **F — Full Protégé parity path** | v0.19–v0.28 | Shipped | Semantic core → formats → OWL 2 → reason/SWRL → services → verify → Protégé JUnit behavioral port → Strixonomy rename → Python package reservation |
| **G — Adoption** | v0.29–v0.32 | Planned | Trust, daily authoring, scale, and team review |
| **H — Automation & extension** | v0.33–v0.35 | Planned | Delivery workflows, SDKs, and assisted modeling |
| **I — Reach & governance** | v0.36–v0.38+ | Planned | Web access, collaboration, and enterprise operations |

| Phase | Version | Era | Status | UI phases | Theme |
|-------|---------|-----|--------|-----------|-------|
| 1–18 | v0.1–v0.18 | A–E | Shipped | — | See [ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md) |
| 19 | v0.19 | F | Shipped | — | Semantic foundation + program baseline |
| 20 | v0.20 | F | Shipped | 1† | Workspace runtime |
| 21 | v0.21 | F | Shipped | — | RDF/XML + OWL/XML write-back |
| 22 | v0.22 | F | Shipped | 2† | Complete OWL 2 authoring |
| 23 | v0.23 | F | Shipped | 5† | Reasoning parity + SWRL |
| 24 | v0.24 | F | Shipped | 3†, 6† | Refactoring + DL Query parity |
| 25 | v0.25 | F | Shipped | 4†, 8† | Viz + plugin SDK 1.0 + a11y + parity CI |
| 26 | v0.26 | F | Shipped | — | Protégé Desktop JUnit behavioral test port (Waves 1–4) |
| 27 | v0.27 | F | Shipped | — | Rename OntoCore and OntoCode to Strixonomy |
| 28 | v0.28 | F | Shipped | — | Reserve the `strixonomy` Python package identity and establish packaging/release ownership |
| 29 | v0.29 | G | Planned | 0, 5, 8 | Trustworthy projects: release hardening, recovery, conformance |
| 30 | v0.30 | G | Planned | 1, 2, 4, 5, 6 | Fast daily authoring: cohesive editing, reasoning, and undo |
| 31 | v0.31 | G | Planned | 3, 4, 6 | Large ontology productivity, with DataFusion query and Tantivy search provider experiments |
| 32 | v0.32 | G | Planned | 9 | Team review and enforceable semantic policy, including a Regorus policy provider |
| 33 | v0.33 | H | Planned | 8, 11 | Automated delivery, dependable plugin management, and maintained workflow/report adapters |
| 34 | v0.34 | H | Planned | 8, 11 | Integration platform, signed official plugin registry, and WASI sandbox pilot |
| 35 | v0.35 | H | Planned | 2, 3, 4, 7, 9 | Assisted modeling and a governed public plugin marketplace |
| 36 | v0.36 | I | Planned | 10, 12 | Everywhere access: WASM and install-free browser workspace |
| 37 | v0.37 | I | Planned | 9, 12 | Governed collaboration: shared review, approvals, provenance |
| 38 | v0.38+ | I | Planned | 10–12 | Enterprise operations: deployment, policy, observability, scale |

†Partial scope in this release (remainder in later releases). Full mapping: [ROADMAP_MAPPING.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md).

### UI phase reference (Product Roadmap 2.0)

OntoUI work uses **UI phases 0–12** from [Product Roadmap 2.0](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md). They are integrated into release phases above — not a separate track.

| UI phase | Name | Primary releases |
|----------|------|------------------|
| **0** | Stabilize OntoUI | v0.13 (shipped) |
| **1** | Workspace foundation | v0.13 (core shipped); v0.30 (tabs, dock) |
| **2** | Entity workspace | v0.4–v0.12 (MVP); v0.30 (relationship/metadata views); v0.31† (AI explain) |
| **3** | Query workbench | v0.5+ (shipped); v0.13† (schema browser shipped); v0.31† (AI query) |
| **4** | Graph workspace | v0.7+ (shipped); v0.30 (layouts, filters); v0.31† (AI graph) |
| **5** | Reasoning experience | v0.9–v0.13† (store integration shipped); v0.30 (pipeline UI, history) |
| **6** | Semantic refactoring | v0.8+ (shipped); v0.30 (merge, batch, undo) |
| **7** | AI experience | v0.35 |
| **8** | Plugin platform | v0.14 (runtime shipped); v0.33 (manager/workflows); v0.34 (official registry); v0.35+ (marketplace and AI provider API) |
| **9** | Collaboration | v0.10+ (diff); v0.13† (PR summary CLI shipped); v0.32 (review/CI); v0.37 (governed collaboration) |
| **10** | Desktop/browser shells | v0.36 (browser); v0.38+ (enterprise packaging) |
| **11** | Ecosystem & docs | v0.11+ (guides); v0.33 (reference plugins/workflow templates); v0.34 (official registry); v0.35+ (marketplace) |
| **12** | Semantic engineering platform | v0.36 (offline browser); v0.37 (team governance); v0.38+ (enterprise operations) |

> **Note on v0.13–v0.18 (retired labels):** Earlier drafts used v0.13–v0.18 for capabilities that **shipped in v0.3–v0.11** (diagnostics, SQL virtual tables, refactoring, Ontologos reasoning, semantic diff, docs export). Those labels are retired. Forward work from v0.13 onward is defined in the phases below.

---

## Shipped releases (v0.1–v0.19)

### Era A — Engine foundation (v0.1–v0.4)

### v0.1 — Strixonomy foundation (shipped)

**Theme:** Prove the semantic workspace engine — index, catalog, and query ontology files from the CLI.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Rust workspace; CLI skeleton; recursive scanner; file hashing; parser adapters; basic catalog (`ontologies`, `classes`, `properties` tables); SQL and SPARQL query |
| **Strixonomy** | — |

**Exit criteria:** `strixonomy query ./repo "SELECT * FROM classes"` returns indexed classes.

**Dependencies:** `oxigraph`, `sqlparser`, `ignore`, `clap`

---

### v0.2 — Strixonomy explorer (shipped)

**Theme:** Browse ontologies in VS Code via a language server.

**UI phases delivered:** **1** (partial) — explorer trees, entity inspector, VS Code command palette.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | LSP process; workspace indexing command |
| **Strixonomy / OntoUI** | VS Code extension skeleton; ontology explorer; class/property/individual trees; entity inspector; jump to source; hover, go-to-definition, document/workspace symbols |

**Exit criteria:** User can browse an ontology repo in VS Code.

**Dependencies:** `lsp-server`, `lsp-types`, Strixonomy crates

---

### v0.3 — Diagnostics (shipped)

**Theme:** Surface ontology quality issues inline.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Parse errors; broken imports; undefined prefixes; duplicate labels; missing labels; orphan classes; `diagnostics` SQL virtual table |
| **Strixonomy** | Problems panel integration |

**Exit criteria:** User gets useful ontology diagnostics inline.

**Dependencies:** `oxigraph` (parse errors); in-house lint rules in `strixonomy-diagnostics`

---

### v0.4 — Write-back + Horned-OWL (shipped)

**Theme:** Edit Turtle ontologies without Protégé.

**UI phases delivered:** **2** (partial) — editable entity inspector, simple axiom forms.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | `strixonomy-owl` crate (Horned-OWL catalog bridge); patch-based write-back; create/edit/delete classes, properties, individuals; edit labels, comments, simple `SubClassOf`, deprecated flag; Oxigraph ↔ Horned-OWL consistency tests; CLI `strixonomy patch`; LSP `strixonomy/applyAxiomPatch` |
| **Strixonomy / OntoUI** | Editable entity inspector |

**Exit criteria:** User can edit labels and simple subclass axioms in Turtle; catalog axioms for editing come from Horned-OWL.

**Dependencies:** `horned-owl`, `horned-functional` via `strixonomy-owl` ([ADR-0013](design/adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0006](design/adr/0006-patch-based-write-back.md))

---

### Era B — IDE depth (v0.5–v0.8)

### v0.5 — Query workbench + Manchester MVP (shipped)

**Theme:** Query and author complex class expressions.

**UI phases delivered:** **3** — SQL/SPARQL editor, results table, query history, saved queries; Manchester editor MVP.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Manchester parse/serialize for `SubClassOf` and `EquivalentClasses`; LSP `strixonomy/parseManchester`, `strixonomy/query`, `strixonomy/sparql` |
| **Strixonomy / OntoUI** | SQL and SPARQL query webviews; saved queries, result export, query history; Manchester editor MVP |

**Exit criteria:** User can query ontologies in VS Code and edit complex subclass/equivalent axioms via Manchester.

**Dependencies:** `sqlparser`, `oxigraph`; Manchester in `strixonomy-owl`

---

### v0.6 — Reasoning (shipped)

**Theme:** Rust-native OWL classification and inferred hierarchy.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | `strixonomy-reasoner` crate (Ontologos facade); `el`, `rl`, `rdfs` adapters; profile detection; unsatisfiable classes; classification result cache |
| **Strixonomy** | Reasoner panel; asserted/inferred/combined hierarchy toggle; explanation panel (DL-first on DL) |

**Exit criteria:** User can classify EL ontologies, see inferred hierarchy, and get EL explanations where available.

**Dependencies:** Ontologos `ontologos-*` ([ADR-0015](design/adr/0015-adopt-ontologos-reasoner.md))

---

### v0.7 — Visualization, React UI, OBO & ROBOT (shipped)

**Theme:** Rich IDE panels and biomedical toolchain interop.

**UI phases delivered:** **2** (partial), **4** (partial) — React entity inspector; class/property/import/neighborhood graphs.

Sub-phases: **v0.7a** (React foundation) → **v0.7** (graphs + inspector) → **v0.7b** (OBO + ROBOT).

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | `strixonomy-robot` wrappers (`validate`, `merge`, `report`); OBO format index; graph structure export via `petgraph` |
| **Strixonomy / OntoUI** | `extension/webview-ui/` — Vite + React + TypeScript; typed `postMessage` protocol; CSP-compliant panel host; class/property/import/neighborhood graphs; entity inspector on React; OBO syntax highlighting and id rendering in explorer |

**Exit criteria:** User can navigate ontologies visually; biomedical maintainer can index OBO and run ROBOT in CI.

**Dependencies:** `react`, `vite` (extension); `fastobo`, `fastobo-owl`, `fastobo-validator`; ROBOT CLI ([ADR-0017](design/adr/0017-react-webview-ui.md))

---

### v0.8 — Refactoring + full Manchester (shipped)

**Theme:** Safe large-scale ontology maintenance and full OWL 2 DL expression authoring.

**UI phases delivered:** **6** — refactor preview panel; rename/find references in explorer.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Safe IRI rename, namespace migration, find usages, move entity, extract module; preview/apply refactor plan; full Manchester axiom catalog (restrictions, disjoint, property chains view) |
| **Strixonomy / OntoUI** | Refactor preview panel; Query Workbench and Manchester editor migrated to React; LSP rename and find references |

**Exit criteria:** User can safely refactor ontology repositories and author full OWL 2 DL expression sets via hybrid UI.

**Dependencies:** `horned-owl`, `horned-functional`; React webview UI

---

### Era C — Platform & authoring (v0.9–v0.13)

### v0.9 — Strixonomy platform identity (shipped)

**Theme:** Unified naming, public API, and DL reasoning.

**UI phases delivered:** **5** (partial) — reasoner panel; asserted/inferred/combined hierarchy; EL explanations.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Rename `ontoindex-*` → `strixonomy-*`; CLI `strixonomy`; LSP `strixonomy-lsp` with `strixonomy/*` methods; `strixonomy` façade crate with `Workspace` API; Ontologos 1.0.0 integration (`dl` and `auto` adapters); plugin platform design ([PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md)) |
| **Strixonomy / OntoUI** | Reasoner + explanation panels on React; legacy HTML webviews removed; Strixonomy branding |

**Exit criteria:** Contributors and users distinguish Strixonomy (engine) from Strixonomy (IDE); DL/auto classification enabled.

**Dependencies:** Ontologos 1.0.0; breaking release for v0.8 integrators ([ADR-0018](design/adr/0018-ontocore-platform-identity.md))

---

### v0.10 — Semantic workspace (shipped)

**Theme:** Team-scale development workflows.

**UI phases delivered:** **9** (partial) — semantic diff React panel.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Incremental indexing (content-hash reuse); multi-root workspaces; stable `strixonomy::Workspace` API; `strixonomy-diff` crate; `strixonomy diff` CLI; git ref compare; breaking-change detection; optional disk cache (`.strixonomy/cache/`) |
| **Strixonomy / OntoUI** | Semantic diff React panel; LSP `strixonomy/semanticDiff`; multi-root folder support |

**Exit criteria:** User can diff ontology versions, work in multi-root workspaces, and reindex incrementally at scale.

**Dependencies:** `git2`, `horned-owl`, `pulldown-cmark`, `minijinja`

---

### v0.11 — Editor depth & distribution (shipped)

**Theme:** Close editor gaps and expand distribution.

**UI phases delivered:** **5** (quick fixes), **7** (partial — `strixonomy docs` export), **11** (partial — enterprise adoption guides).

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | LSP `textDocument/completion` (Turtle prefix, QName, IRI); diagnostic quick fixes (`undefined_prefix`, `missing_label`, `broken_import`); `strixonomy-docs` crate; `strixonomy docs` CLI (Markdown/HTML); `add_import` / `remove_import` patch ops; OBO read path via `fastobo` (synonyms, defs, xrefs); ADR for v0.30 OBO write-back ([ADR-0019](design/adr/0019-obo-write-back.md)) |
| **Strixonomy / OntoUI** | Manage Imports panel; Open VSX publishing (Cursor); diagnostic code actions; entity inspector panel reuse on navigation; VS Code e2e tests |

**Exit criteria:** Daily Turtle editing, import management, and docs export work without leaving VS Code; extension available on VS Code Marketplace and Open VSX.

**Dependencies:** `fastobo`; `minijinja`, `pulldown-cmark`

---

### v0.12 — Authoring parity (shipped)

**Released:** v0.12.0 (2026-07-06)

**Theme:** Close remaining **P0** OWL and OBO authoring gaps from [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md).

**UI phases delivered:** **2** (P0 exit) — domain/range/characteristics forms, property chain editor, OBO edit forms, preview-before-apply on all axiom types.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Property domain, range, and characteristics authoring (patch ops); individual class/property assertions; expanded annotation assertion editing; property chain editing; full OBO write-back ([OBO_ROBOT_SPEC.md](design/OBO_ROBOT_SPEC.md), [ADR-0019](design/adr/0019-obo-write-back.md)); OWL/XML read support; Horned-OWL → Ontologos bridge improvements; axiom round-trip golden tests (Protégé fixtures) |
| **Strixonomy / OntoUI** | Inspector forms for domain/range/characteristics; property chain editor; OBO write-back in inspector; DL clash-trace explanations via `ontologos-explain` + `ontologos-dl`; Turtle preview before apply for all axiom types |

**Exit criteria:** All **P0 — OWL 2 DL authoring** and **OBO & biomedical** rows in [PROTEGE_PARITY.md](design/PROTEGE_PARITY.md) are green.

**Dependencies:** `horned-owl`, `horned-functional`, `fastobo`, `fastobo-owl`; Ontologos `ontologos-explain`

---

### Era D — OntoUI platform (v0.13)

### v0.13 — Platform hardening (shipped)

**Released:** v0.13.0 (2026-07-08)

**Theme:** OntoUI platform foundation + Strixonomy hardening for plugins (v0.14) and Protégé polish (v0.30).

**UI phases delivered:** **0**, **1**, partial **3** (schema browser), partial **5** (reasoning store integration), partial **9** (PR summary CLI). Checklist: [ROADMAP_MAPPING.md § v0.13](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md)

| Area | Deliverables |
|------|--------------|
| **OntoUI** | `WorkspaceHost` adapter; Zustand `WorkspaceStore` + event bus; Current Focus + `FocusChanged`; `WorkspaceRegistry`; design tokens + shared primitives; extension-host `FocusRelayService`; Entity Inspector, Graph, Query Workbench, and Refactor Preview on store; schema browser in Query Workbench; refactor + reasoning store slices |
| **Strixonomy** | Horned-OWL SQL virtual tables (`restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`, `domain_axioms`, `range_axioms`); LSP `strixonomy/listSqlSchema`; `strixonomy diff --pr-summary`; `.strixonomy/diagnostics.toml`; LSP semantic tokens (Turtle/OBO); `strixonomy docs` class hierarchy + property index; API stability policy; benchmark smoke tests (`tests/bench_index.rs`) |
| **Strixonomy** | Cross-panel focus sync (explorer → inspector + graph); Vitest + extension integration tests for focus relay, schema browser, and store slices; UX audit ([UX_AUDIT_v0.13.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/UX_AUDIT_v0.13.md)) |

**Exit criteria (met):**

- [x] **Focus sync** — explorer selection relays to inspector and graph via extension-host focus relay
- [x] **Store ownership** — Entity, Graph, Query, and Refactor Preview workspaces consume WorkspaceStore
- [x] **Design system** — design tokens + shared primitives on Entity Inspector and Query Workbench
- [x] **Schema browser** — browse virtual tables/columns; insert snippets into Query Workbench editor
- [x] **Team workflow** — `strixonomy diff A..B --pr-summary` emits PR-ready Markdown; documented and tested
- [x] **Performance** — benchmark fixtures + sizing guide update
- [x] **Quality** — 161 webview-ui Vitest tests + extension integration tests; accessibility pass on migrated panels
- [x] **API policy** — public `strixonomy` API stability documented on path to v0.30

**Deferred to later releases:** persistent tabs + bottom dock, full panel component migration, SQL `JOIN`/`GROUP BY`, PR summary UI panel, validation report panel, Reasoner/Semantic Diff full store migration → **v0.30** / **v0.32** per original scope boundaries.

**Dependencies:** `sqlparser` + `horned-owl` ([ADR-0011](design/adr/0011-use-sqlparser-for-sql.md), [ADR-0013](design/adr/0013-dual-stack-oxigraph-horned-owl.md)); platform ADRs [0002–0004](adr/README.md)

---

### v0.14 — Plugin host MVP (shipped)

**Released:** v0.14.0 (2026-07-09)

**Theme:** External extensibility without embedding workflow engines in core.

**UI phases delivered:** **8** (plugin platform MVP). Milestone: [Product Roadmap 2.0 phase 8](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PRODUCT_ROADMAP_2.0.md).

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | `PluginHost` runtime; manifest discovery from `.strixonomy/plugins/`; reference plugins (naming validator, Markdown exporter, SHACL scaffold); CLI `plugins list/run`, `validate`/`docs` hooks; LSP `listPlugins`/`runPlugin`; subprocess workflow runner |
| **OntoUI** | Capability provider registry; inspector plugin cards; `WorkspaceStore` plugins slice |
| **Strixonomy** | Plugin commands; owlmake workflow scaffold (`workflow run --plugin owlmake`); workflow output panel |
| **Ecosystem** | `examples/plugin-workspace/` fixture; [Plugin authoring guide](guides/plugins.md) |

**Exit criteria (met):**

- [x] Third party can ship a validation or export plugin without forking Strixonomy (reference plugins + manifest schema)
- [x] owlmake can be invoked from Strixonomy as an external workflow (subprocess scaffold)

**Dependencies:** `strixonomy-plugin` crate; reference plugin binaries; [PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md)

---

### v0.15 — Plugin API + visualization parity + explanation workspace (shipped, partial)

**Released:** v0.15.0 (2026-07-08)

**Theme:** Extend the v0.14 plugin host with permissions, UI views, explanation alternatives, and graph asserted/inferred modes.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Plugin permissions + `api_version = "1"`; subprocess path-jail hardening; explanation alternatives + staleness metadata in LSP/CLI |
| **OntoUI** | Graph asserted/inferred/combined modes, layouts, search; explanation panel with multiple justifications |
| **Strixonomy** | Plugin UI views (dockable panels); plugin commands; explanation staleness warnings |
| **Ecosystem** | `demo-ui-view.toml` fixture; updated [Plugin authoring guide](guides/plugins.md) |

**Exit criteria (partial):** dockable plugin views + commands shipped; graph modes + explanation alternatives shipped; preferences/context actions **shipped in v0.16**.

See [migration/v0.15.md](migration/v0.15.md) and [SHIPPED.md](SHIPPED.md).

---

### v0.16 — Workspace layouts + preferences + imports polish (shipped, partial)

**Released:** v0.16.0 (2026-07-09)

**Theme:** Close the “desktop shell” parity gap: plugin preferences, context actions, and imports/layout polish in the VS Code extension.

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | Plugin command dispatch via LSP `strixonomy/runPlugin` (validator/export/workflow) |
| **OntoUI** | Plugin preferences pages and context actions surfaced from manifest contributions |
| **Strixonomy** | **Plugins: Open Preferences…**, **Plugins: Run Context Action…**, **Reload Imports**, **Reset Layout** commands |
| **Ecosystem** | [v0.16 scope](design/v0.16_SCOPE.md); [migration/v0.16.md](migration/v0.16.md) |

**Exit criteria (partial):** P0 preferences/context actions/imports reload shipped; full layout persistence and workspace perspectives deferred to v0.17+.

See [migration/v0.16.md](migration/v0.16.md) and [SHIPPED.md](SHIPPED.md).

---

### v0.17 — Menus/toolbars/dialog parity (shipped, partial)

**Released:** v0.17.0 (2026-07-10)

See [v0.17 scope](design/v0.17_SCOPE.md), [migration/v0.17.md](migration/v0.17.md), and root [ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md).

---

### v0.18 — Desktop UX shell gate + migration readiness (shipped)

**Released:** v0.18.0 (2026-07-11); patches **v0.18.1** (2026-07-12), **v0.18.2** (2026-07-13)

**Theme:** Close the desktop UX shell gate (menus, layouts, workflows, migration docs). **Not** full functional Protégé parity — see [V0_30_PHASES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md) for v0.19–v0.27.

**Scope (docs/audit-first):** [v0.18_SCOPE.md](design/v0.18_SCOPE.md) · [0.18 parity assessment](https://github.com/eddiethedean/strixonomy/blob/main/docs/PROTEGE_REVERSE_ENGINEERING/ONTOCODE_PARITY/ONTOCODE_0.18_PROTEGE_PARITY_ASSESSMENT.md)

Canonical detail: root [ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md) § v0.18. See [migration/v0.18.md](migration/v0.18.md).

---

### Era F — Full Protégé parity path (v0.19–v0.27) — shipped through v0.27

**Canonical plan:** [V0_30_PHASES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md)

### v0.19 — Semantic foundation + program baseline (shipped)

**Released:** v0.19.0 (2026-07-13)

**Theme:** Freeze parity scope; route Turtle/OBO edits through semantic transactions (`strixonomy-edit`).

| Area | Deliverables |
|------|--------------|
| **Strixonomy** | `Transaction` / `SemanticChange`; LSP + CLI apply via adapters; invert/compose/validate |
| **Parity program** | Frozen scope; `parity/protege-desktop-parity.yaml` + CI validator; epics EPIC-001…011 |

See [migration/v0.19.md](migration/v0.19.md) · [SHIPPED.md](SHIPPED.md) · full detail in [ROADMAP.md § v0.19](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md).

---

### v0.20 — Workspace runtime (shipped)

**Status:** **Shipped** as tagged **v0.20.0**.

**Theme:** Workspace as central runtime for ontology state and transactions.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.20.md](migration/v0.20.md) · full detail in [ROADMAP.md § v0.20](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md).

---

### v0.21 — Required format write-back (shipped)

**Status:** **Shipped** as tagged **v0.21.0**.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.21.md](migration/v0.21.md).

---

### v0.22 — Complete OWL 2 authoring (shipped)

**Status:** **Shipped** as tagged **v0.22.0**.

**Theme:** Every P0 OWL 2 construct authorable across required formats.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.22.md](migration/v0.22.md) · [OWL2_AUTHORING_GAPS.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/06_SUBSYSTEMS/OWL2_AUTHORING_GAPS.md).

---

### v0.25 — UX completion + executable verification (shipped)

**Released:** v0.25.0 (2026-07-15)

**Theme:** Visualization, plugin SDK 1.0, accessibility, parity manifest CI.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.25.md](migration/v0.25.md) · full detail in [ROADMAP.md § v0.25](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md).

---

### v0.26 — Protégé Desktop test port (shipped)

**Released:** v0.26.2 (2026-07-17)

**Theme:** Port portable Protégé Desktop JUnit behaviors into Strixonomy Rust semantic oracles (not a JVM suite runner).

See [SHIPPED.md](SHIPPED.md) · [migration/v0.26.md](migration/v0.26.md) · full detail in [ROADMAP.md § v0.26](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md).

---

### v0.27 — Strixonomy rename (shipped)

**Released:** v0.27.0 (2026-07-27)

**Theme:** Rename the OntoCore Rust engine and OntoCode VS Code extension to **Strixonomy**.

See [SHIPPED.md](SHIPPED.md) · [migration/v0.27.md](migration/v0.27.md) · full detail in [ROADMAP.md § v0.27](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md).

---

### v0.28 — Python package reservation + compat removal (shipped)

**Released:** v0.28.0 (2026-07-28)

**Theme:** Secure the official `strixonomy` identity on PyPI; remove OntoCore/OntoCode compatibility shims from v0.27.

| Area | Deliverables |
|------|--------------|
| **Package identity** | Confirm the `strixonomy` PyPI project name; establish maintainers, recovery ownership, and project metadata linking only to the official repository and documentation |
| **Packaging skeleton** | Add the future SDK package layout and Maturin/PyO3 packaging metadata without exposing unstable Rust internals |
| **Release security** | PyPI upload via GitHub Actions secret `PYPI_API_TOKEN`; 2FA for maintainers; procedure in [releasing.md](releasing.md) |
| **Preview publication** | Publish only a clearly labeled pre-release reservation artifact; its README and import surface must state that the Python SDK is planned for v0.34 and must not claim workspace, query, validation, diff, or reasoning capabilities |
| **Documentation** | Add a package-status page that distinguishes the reserved Python distribution from the shipped CLI, LSP, Rust crates, and the planned v0.34 SDK |

**Non-goals:** Python bindings, CLI subprocess wrappers presented as an SDK, stable Python APIs, or production support. Those remain **v0.31** deliverables.

**Exit criteria:** The official project controls the `strixonomy` PyPI identity through a reproducible GitHub Release publish workflow; ownership and recovery are documented; the published artifact makes no capability claims beyond package reservation.

---

## Planned releases (v0.29 → v0.38+)

Each phase below must deliver a complete user outcome. A phase does not exist merely to introduce a framework, API, or architectural layer. Items may move between releases, but the stated exit criterion is the release gate.

### Era G — Adoption (v0.29–v0.32)

### v0.29 — Trustworthy projects

**User outcome:** Teams can adopt Strixonomy on real repositories without fearing silent corruption, lost work, or unverifiable results.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Release confidence | All P0 parity requirements backed by executable evidence; Turtle/OBO/XML semantic round-trip corpus; cross-platform extension smoke suite |
| Safe editing | Crash-safe writes, transaction recovery, external-change conflict flow, deterministic previews, undo verification |
| Operational clarity | Actionable diagnostics for imports, profiles, unsupported writes, truncation, and stale reasoner results |
| Distribution | Tested CLI and bundled-LSP paths for Linux, macOS, and Windows; signed artifacts where supported |
| Performance baseline | Published cold-index, incremental-index, query, reasoner, and graph budgets on representative small/medium/large corpora |

**Non-goals:** New marketplaces, AI surfaces, or hosted services.

**Exit criterion:** The production evidence protocol passes on the maintained conformance corpus on all supported platforms, with no open data-loss or silent-semantic-change defect.

### v0.30 — Fast daily authoring

<a id="strixonomy-10-modern-protege-replacement"></a>

**User outcome:** An ontology engineer can complete the normal edit → understand → validate → reason → save loop without panel hunting or returning to Protégé.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Cohesive entity workspace | Relationship, reference, metadata, diagnostic, and reasoning cards in one navigable workspace |
| Editing flow | Complete forms + Manchester + source loop; multi-step undo/redo; batch label normalization; guided merge with impact preview |
| Reasoning flow | One-click validate/classify pipeline, entity-level explanations, run history, stale-result handling, Problems integration |
| Workspace UX | Persistent working set, semantic back/forward history, saved graph layouts and filters, command/search affordances |
| Migration | Protégé round-trip playbooks, OBO workflows, and explicit supported/unsupported handoff guidance |

**Non-goals:** Broad third-party integrations and cloud collaboration.

**Exit criterion:** Maintained OWL 2 DL and OBO scenario tests complete end to end in VS Code with no required Protégé step except documented P2 cases.

Track trust and parity work in [V0_30_PHASES.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md) and [v0.30_BACKLOG.md](design/v0.30_BACKLOG.md).

### v0.31 — Large ontology productivity

**User outcome:** Large, multi-ontology projects remain responsive and operations that currently require format conversion work directly.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Scale | Background incremental indexing, bounded memory, cancellable streaming results, graph virtualization beyond current truncation caps |
| Query depth | SQL joins, ordering, grouping, limits, explainable query plans, saved parameterized queries; SPARQL result streaming |
| Multi-format operations | Move, extract, flatten, cleanup, and ontology merge for RDF/XML, OWL/XML, and OBO where semantics permit |
| Navigation | Workspace-wide symbol/search ranking, import-aware dependency exploration, large-result filtering |
| Performance UX | Progress, cancellation, partial results, resource-budget warnings, and benchmark regression gates |
| Provider experiments | Prototype DataFusion for advanced analytical queries and Tantivy for ranked multilingual entity search behind Plugin SDK 1.0; promote either into the core engine only if benchmarks, index lifecycle, and UX show that a plugin boundary is the wrong fit |

**Non-goals:** A distributed database or remote execution service.

**Exit criterion:** Published large-corpus budgets pass without UI stalls or unannounced truncation, and the supported multi-format refactor matrix has semantic round-trip coverage.

### v0.32 — Team review and policy

**User outcome:** Teams can review ontology meaning—not just text—and enforce shared quality policy before merge.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Review workspace | Axiom-level semantic diff, impact graph, inferred-change summary, annotation-aware review, reviewer notes |
| CI policy | Versioned policy configuration, severity thresholds, baseline/suppression workflow, machine-readable evidence bundle |
| Pull requests | GitHub Checks annotations and semantic PR summaries with links back to entities and evidence |
| Governance basics | Ownership rules by ontology/module/namespace, required approvals, deprecation and IRI policy checks |
| Reproducibility | Pin and report engine, reasoner, plugin, import, and policy versions in every review |
| Rego policy provider | Maintained Microsoft Regorus adapter evaluates repository Rego policy over semantic diffs, catalog facts, and diagnostics; returns machine-readable violations to the Review workspace and CI |

**Non-goals:** Real-time co-editing.

**Exit criterion:** A repository can block a pull request on semantic breaking changes or policy violations and produce an auditable review bundle locally and in CI.

### Era H — Automation and extension (v0.33–v0.35)

### v0.33 — Automated ontology delivery

**User outcome:** Maintainers can run repeatable build, QC, and release workflows from the IDE or CI without hand-wiring every tool.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Workflow execution | Maintained adapter for [EBISPOT/owlmake](https://github.com/EBISPOT/owlmake), including executable discovery, version checks, workspace configuration, and `build`/`qc`/`release`/`report` step mapping; import existing Makefile, ODK, ROBOT, and GitHub Actions workflows |
| Release pipeline | Merge, materialize, validate, report, version, package, and publish stages with preview and resumable logs |
| Dependable plugin manager | Explicit registries and organization catalogs; workspace/user/organization/bundled scopes; deterministic compatibility and dependency resolution; `.strixonomy/plugins.toml` declarations plus a reproducible lockfile |
| Safe lifecycle | Permission review, external-tool mode selection, transactional install/update/rollback, disable/uninstall, health checks, failure quarantine, and matching VS Code/CLI/CI behavior |
| Maintained adapters | [rudof](https://github.com/rudof-project/rudof) for SHACL/ShEx/DCTAP validation and conversion; Typst for publication-quality reports; mdBook for searchable documentation; pinned EBISPOT/owlmake workflow integration |
| Results | Unified HTML/Markdown/JSON QC reports linked to source entities and CI evidence |
| Templates | Maintained OBO, OWL, and mixed-project starter workflows |

**Non-goals:** Reimplementing ROBOT, ODK, or every workflow engine inside Strixonomy.

Implementation contract: [Plugin manager and registry specification](plugin-manager-plan.md).

**Exit criterion:** A fresh workstation restores the same pinned plugin set from declarations and lockfile, survives a failed update without losing the previous version, and a maintained ODK-style fixture runs a real pinned EBISPOT/owlmake workflow locally and in CI with equivalent artifacts, diagnostics, logs, and QC evidence.

### v0.34 — Integration platform

**User outcome:** Other tools can embed Strixonomy reliably instead of shelling out and scraping output.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Python SDK | Typed workspace, query, validation, diff, patch, and reasoner APIs with notebooks and async support |
| TypeScript SDK | Typed LSP/client helpers, webview protocol package, and Node workspace client |
| MCP server | Read, query, validate, explain, diff, and preview-change tools with workspace trust and explicit write approval |
| Compatibility | Capability negotiation, deprecation windows, contract tests, generated schemas, migration guides |
| Examples | CI bot, notebook analysis, documentation generator, and custom editor integrations |
| Official plugin registry | Signed, cross-platform artifacts; verified publisher namespaces; compatibility and permission metadata; revocation, mirrors, and offline lockfile restoration |
| Sandbox pilot | Wasmtime/WASI execution tier with explicit filesystem/network capabilities and resource limits; native subprocess plugins remain clearly labeled as unsandboxed code |

**Non-goals:** AI-generated changes; this release supplies dependable integration primitives.

**Exit criterion:** Python, TypeScript, and MCP consumers complete the same reference index/query/validate/diff workflow with contract-tested equivalent results, and the official registry rejects tampered or revoked artifacts while restoring a verified pinned set offline.

### v0.35 — Assisted modeling

**User outcome:** Engineers receive useful modeling help while retaining control, provenance, and deterministic review.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Explain | Plain-language entity, axiom, diagnostic, query, graph, and reasoner-result explanations grounded in workspace evidence |
| Propose | Label, definition, axiom, query, repair, and documentation suggestions as previewable semantic patches |
| Review | Project-wide ontology review with citations to affected entities, confidence, policy checks, and impact analysis |
| Providers | Local and remote model providers through an opt-in provider API; budgets and data-boundary controls |
| Safety | No silent writes; preview/approve/apply lifecycle, provenance record, reversible change sets, prompt-injection boundaries |
| Governed marketplace | Public discovery, verified publisher onboarding, ratings/reviews, compatibility evidence, security reporting, moderation and takedown, and explicit re-consent when permissions expand |

**Non-goals:** Autonomous publishing or unreviewed ontology mutation.

**Exit criterion:** Every generated change is evidence-linked, policy-checked, previewable as a semantic diff, explicitly approved, and reversible; public marketplace publication also passes publisher, artifact, compatibility, permission, and moderation gates.

### Era I — Reach and governance (v0.36–v0.38+)

### v0.36 — Install-free ontology workspace

**User outcome:** Anyone can inspect, query, validate, and review an ontology from a browser without installing Rust, Java, or VS Code.

| High-value investment | Deliverables |
|-----------------------|--------------|
| WASM engine | Local-file indexing, catalog browse, validation, constrained query, semantic diff, and graph exploration |
| Browser workspace | Static/offline-capable React application with drag/drop, URL/file-system access where available, and no required backend |
| Shareable review | Portable evidence bundles and read-only review links that preserve privacy by default |
| Performance | Worker-based execution, progressive loading, explicit browser resource limits |

**Non-goals:** Server-side reasoning parity or real-time collaboration.

**Exit criterion:** The reference ontology review workflow runs offline in supported browsers and produces the same portable evidence schema as desktop/CI.

### v0.37 — Governed collaboration

**User outcome:** Distributed teams can coordinate ontology changes with approvals, provenance, and controlled conflict resolution.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Shared review | Team workspaces, threaded entity/axiom comments, assignments, approvals, and review status |
| Change coordination | Semantic conflict detection, patch rebasing, branch comparison, controlled merge |
| Governance | Roles, namespace ownership, approval policies, immutable audit history, provenance export |
| Integration | Repository-backed identity and events first; optional hosted synchronization behind explicit deployment |

**Non-goals:** Replacing Git hosting or generic project-management systems.

**Exit criterion:** Two teams can propose overlapping changes, detect semantic conflicts, obtain required approvals, and export a complete audit record.

### v0.38+ — Enterprise operations

**User outcome:** Organizations can deploy and operate Strixonomy predictably across many teams and large ontology portfolios.

| High-value investment | Deliverables |
|-----------------------|--------------|
| Deployment | Managed desktop/browser distribution, offline installation, configuration policy, upgrade channels |
| Security | SSO/RBAC adapters, secrets integration, plugin allowlists, signed artifacts, supply-chain attestations |
| Portfolio operations | Cross-repository catalog, dependency and version visibility, policy dashboards, migration coordination |
| Observability | Privacy-preserving health metrics, audit export, performance/SLO reporting, support bundles |
| Scale | Remote execution and distributed reasoning only where measured workloads justify them |

**Non-goals:** Mandatory cloud dependency or hidden telemetry.

**Exit criterion:** A reference multi-team deployment passes documented security, upgrade, recovery, audit, and performance runbooks.

---

## Long-term goal

Strixonomy becomes the foundation for modern ontology tooling.

Strixonomy becomes the flagship IDE.

Ontologos becomes the flagship Rust reasoning engine.
