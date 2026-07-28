# PLAN.md — Strixonomy and Strixonomy

> **Executive summary superseded by** [Platform Vision](../vision.md) and [Platform Roadmap](../roadmap.md). This document retains tactical product planning detail for contributors.

## 1. Executive Summary

Strixonomy and Strixonomy form a two-layer product strategy for modern ontology engineering.

**Strixonomy** is the Rust backend: a local-first embedded ontology engine that scans a directory of OWL/RDF/Turtle/JSON-LD/OBO files, builds a semantic index, exposes ontology concepts as queryable tables, validates ontology repositories, performs semantic diffs, and powers editor integrations.

**Strixonomy** is the VS Code extension: a full ontology engineering workbench built on top of Strixonomy. Its long-term goal is to replace Protégé for developers, data engineers, knowledge graph engineers, semantic modelers, and organizations that prefer ontology-as-code workflows in the IDE and CI.

## 2. Product Thesis

Protégé is excellent for traditional ontology editing, but it is not designed around the way modern software teams work:

- version-controlled projects
- pull requests
- CI validation
- semantic diffs
- code review
- local development
- editor-native workflows
- AI-assisted engineering
- documentation pipelines
- automated quality checks

Strixonomy should eventually provide the authoring depth of Protégé while adding the workflow strengths of VS Code.

## 3. Products

### 3.1 Strixonomy

Strixonomy is a Rust library and CLI.

Primary capabilities:

- Recursively scan ontology repositories
- Parse `.owl`, `.rdf`, `.ttl`, `.jsonld`, `.obo`, `.nt`, `.nq`, `.trig`
- Build incremental ontology catalog
- Expose virtual tables for ontology entities
- Run SQL-like queries over ontology structures
- Run SPARQL over RDF triples
- Detect errors, warnings, and quality issues
- Generate semantic diffs
- Generate documentation
- Provide services for a language server

### 3.2 Strixonomy

Strixonomy is a VS Code extension.

Primary capabilities:

- Ontology Explorer sidebar
- Class hierarchy editor
- Object/data/annotation property editor
- Individual editor
- Axiom editor
- Imports manager
- Query workbench
- Reasoner integration
- Graph visualization
- Semantic diff viewer
- Documentation generator
- AI-assisted ontology review

## 4. v0.30.0 Bar

v0.30.0 should be positioned as:

> A Protégé-competitive ontology workbench for OWL 2 DL and OBO maintenance inside VS Code.

**Canonical checklist:** [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — all **P0** items must be green before release.

Exit criterion:

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features in PROTEGE_PARITY.md.

P0 highlights (see parity matrix for full list):

- Hybrid authoring: quick forms + Manchester editor ([OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md))
- Horned-OWL + Oxigraph dual stack ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md))
- Rust-native reasoners via **[OntoLogos](https://github.com/eddiethedean/ontologos)**: `el` (0.9.0) + `dl` (1.0.0) with **real** clash-trace explanations ([REASONER_SPEC.md](REASONER_SPEC.md), [ADR-0014](adr/0014-rust-native-reasoners-only.md), [ADR-0015](adr/0015-adopt-ontologos-reasoner.md))
- Full LSP surface ([SPEC.md](SPEC.md) §9)
- Query workbench, graphs, semantic diff, refactoring
- OBO format + ROBOT interop ([OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md))
- VS Code Marketplace + migration guide from Protégé

## 5. Target Users

### 5.1 Primary Users

- Ontology engineers
- Knowledge graph engineers
- Semantic web developers
- Data governance engineers
- **Biomedical ontology maintainers** — OBO + ROBOT pipelines ([OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md))
- Enterprise taxonomy/modeling teams
- Standards authors
- AI/LLM knowledge modeling teams

### 5.2 Secondary Users

- Data engineers
- Software engineers working with RDF/OWL assets
- Technical writers documenting domain models
- QA teams validating semantic repositories
- Researchers maintaining ontology datasets

## 6. Core Differentiators

Strixonomy should not merely copy Protégé. It should beat Protégé in developer workflows.

Key differentiators:

- Semantic diffs in code review
- CI-friendly validation
- VS Code-native editing
- **Hybrid authoring without leaving VS Code** (forms + Manchester)
- Queryable ontology repository model
- Repo-wide refactoring
- AI-friendly schema and docs generation
- Local-first indexing
- Workspace-aware imports
- File-aware diagnostics
- Documentation generation
- Extension/plugin architecture
- **ROBOT interop** for biomedical release pipelines (not reimplementation)

## 7. Positioning

Possible tagline:

> Build, query, validate, refactor, reason over, and document OWL/RDF ontologies directly in VS Code.

Short positioning:

> Strixonomy is a VS Code-native ontology workbench powered by Strixonomy, a Rust ontology index and query engine.

## 8. Roadmap Summary

- v0.1: Strixonomy scanner, parser, catalog, CLI
- v0.2: Strixonomy explorer and entity inspector
- v0.3: diagnostics and validation
- v0.4a–b: simple write-back + Horned-OWL (`strixonomy-owl`)
- v0.5: query workbench + Manchester MVP
- v0.6: reasoners + real explanations
- **v0.7a: React webview foundation** ([ADR-0017](adr/0017-react-webview-ui.md), [OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md))
- v0.7: graph visualization (React panels)
- v0.7b: OBO + ROBOT interop
- v0.8: refactoring + full Manchester (React query/Manchester panels)
- v0.9: semantic diff, incremental index, docs (React reasoner/diff panels)
- v0.30: [PROTEGE_PARITY.md](PROTEGE_PARITY.md) P0 green + Marketplace + React UI hardening

## 9. Non-Goals for Early Releases

- Hosted collaborative editing
- Multi-user permissions
- Full WebProtégé replacement
- Cloud ontology repository hosting
- Enterprise identity management
- Custom visual graph database hosting
- Reimplementing a triplestore from scratch
- **Reimplementing ROBOT** (interop only — [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md))
- **JVM reasoners** (ELK, HermiT, Pellet — [ADR-0014](adr/0014-rust-native-reasoners-only.md))

## 10. Strategic Implementation Guidance

**Policy:** [ADR-0016](adr/0016-dependency-first-implementation.md) — delegate to mature crates; `strixonomy-*` crates are thin facades. Full inventory: [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

| Layer | Dependency | Strixonomy facade | Phase |
|-------|------------|------------------|-------|
| RDF / SPARQL | `oxigraph` | `strixonomy-parser`, `strixonomy-query` | v0.2 |
| SQL parse | `sqlparser` | `strixonomy-query` | v0.2 |
| Workspace scan | `ignore` | `strixonomy-core` | v0.2 |
| LSP wire | `lsp-server`, `lsp-types` | `strixonomy-lsp` | v0.2 |
| Diagnostics | `oxigraph` + catalog rules | `strixonomy-diagnostics` | v0.3 |
| OWL axioms / Manchester | `horned-owl`, `horned-functional` | `strixonomy-owl` | v0.4b+ |
| Reasoning | OntoLogos `0.9`→`1.0` | `strixonomy-reasoner` | v0.6 / v0.30 |
| Graph structure | `petgraph` | LSP graph export | v0.7 |
| Webview UI | `react`, `vite` | `extension/webview-ui` | v0.7a+ ([ADR-0017](adr/0017-react-webview-ui.md)) |
| OBO | `fastobo`, `fastobo-owl` | `strixonomy-parser` / `strixonomy-owl` | v0.7b |
| ROBOT CI | ROBOT CLI | `strixonomy-robot` | v0.7b |
| File watch | `notify` / `ontologos-watch` | `strixonomy-lsp` | v0.9 |
| Git diff inputs | `git2` | `strixonomy-diff` | v0.9 |
| Docs export | `pulldown-cmark`, `minijinja` | `strixonomy-docs` | v0.9 |
| SHACL (P1) | `rudof` | plugin / diagnostics | v0.30 P1 |

Build on existing mature components — do not reimplement parsers, reasoners, triple stores, OBO parsers, or SHACL engines when a maintained Rust crate covers the profile.

- Use **Horned-OWL** (+ `horned-functional`) for OWL 2 axiom modeling and write-back ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md)).
- Use **Oxigraph** for RDF/SPARQL ([ADR-0003](adr/0003-use-oxigraph.md)).
- Use **sqlparser** virtual tables for SQL; extend for v0.30 joins before considering DataFusion ([ADR-0011](adr/0011-use-sqlparser-for-sql.md)).
- Use **OntoLogos** for all reasoning ([ADR-0015](adr/0015-adopt-ontologos-reasoner.md)).
- Use **React + Vite** for VS Code webview panels ([ADR-0017](adr/0017-react-webview-ui.md)); TypeScript extension host for orchestration only.
- Keep Strixonomy useful as a standalone CLI even without Strixonomy.
