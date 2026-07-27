# Strixonomy + Strixonomy Documentation Package

> **Note:** This folder contains **engineering specs, ADRs, and shipped milestone detail**. Canonical platform direction lives in **[Vision](../vision.md)**, **[Architecture](../architecture.md)**, and **[Roadmap](../roadmap.md)** ([full ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md)). For **what ships in v0.26.2**, see [What ships today](../SHIPPED.md).

## Documentation layers

| Layer | Where | Audience |
|-------|-------|----------|
| **Platform** | [vision.md](../vision.md), [architecture.md](../architecture.md), [roadmap.md](../roadmap.md) | Everyone — mission, ecosystem, forward plan |
| **Shipped** | [SHIPPED.md](../SHIPPED.md), guides, reference | Users evaluating current capabilities |
| **Product design (UI)** | [ui/README.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/README.md) — [ROADMAP_MAPPING.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md) | Designers, frontend contributors — UX specs mapped to releases |
| **Engineering** | `docs/design/` (this folder) | Contributors — specs, ADRs, v0.1–v0.11 milestones |

For **user-facing guides**, pick a path:

| Path | Audience | Start |
|------|----------|-------|
| [VS Code extension](https://strixonomy-vs.readthedocs.io/en/latest/ide/vscode-extension/) | Explorer, inspector, Query Workbench, reasoner panels | [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy), [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (Cursor), or VSIX — no Rust required |
| [Rust & CLI](https://strixonomy-vs.readthedocs.io/en/latest/guides/rust-crates/) | `strixonomy` CLI, crates.io libraries, CI, LSP integrators | `cargo install strixonomy-cli` |

[Documentation home](https://strixonomy-vs.readthedocs.io/en/latest/) · [What ships today](../SHIPPED.md)

Two related products:

1. **Strixonomy** — Rust semantic workspace engine (`strixonomy-*` crates).
2. **Strixonomy** — VS Code extension (explorer, diagnostics, Turtle + Manchester authoring, Query Workbench, reasoner).

**Sibling project:** [OntoLogos](https://github.com/eddiethedean/ontologos) — Rust ontology reasoner. Strixonomy delegates reasoning to OntoLogos per [ADR-0015](adr/0015-adopt-ontologos-reasoner.md).

**Dependency policy:** [ADR-0016](adr/0016-dependency-first-implementation.md) — thin `strixonomy-*` facades over mature crates. Inventory: [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md). Licenses: [LICENSES.md](LICENSES.md).

## v1.0 exit bar

**[protege-parity program](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/README.md)** — authoritative 1.0 engineering plan ([PRE_1_0_PHASES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md)). **[PROTEGE_PARITY.md](PROTEGE_PARITY.md)** — historical v0.18 P0/P1/P2 checklist. Forward plan: [Roadmap § Strixonomy 1.0](../roadmap.md#strixonomy-10-modern-protege-replacement).

## Document status

Many specs describe **target** behavior. Check the banner at the top of each doc, or:

- **Implemented v0.6 LSP:** [LSP API](https://strixonomy-vs.readthedocs.io/en/latest/lsp-api/) — includes `query`, `sparql`, `parseManchester`, `runReasoner`, `getExplanation`
- **Implemented reasoner:** [Reasoner guide](https://strixonomy-vs.readthedocs.io/en/latest/guides/reasoner/)
- **Implemented SQL tables:** [SQL reference](https://strixonomy-vs.readthedocs.io/en/latest/sql-reference/)
- **Implemented authoring:** [authoring guide](https://strixonomy-vs.readthedocs.io/en/latest/authoring/)
- **ADRs (canonical):** [adr/README.md](adr/README.md)

## Documents

### Product & roadmap

- [PLAN.md](PLAN.md) — tactical product plan (executive summary in [Vision](../vision.md))
- [ROADMAP.md](ROADMAP.md) — **engineering milestones v0.1–v0.11 (shipped detail)**
- [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md) — **external crate inventory**
- [LICENSES.md](LICENSES.md) — third-party license summary
- [protege-parity program](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/README.md) — **1.0 engineering program**
- [PROTEGE_PARITY.md](PROTEGE_PARITY.md) — historical v0.18 compete checklist
- [v1.0_BACKLOG.md](v1.0_BACKLOG.md) — implementation backlog

### Technical specs

- [SPEC.md](SPEC.md) — combined technical specification
- [ARCHITECTURE.md](ARCHITECTURE.md) — **implementation architecture** (crate layout)
- [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) — hybrid forms + Manchester authoring
- [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) — OBO format + ROBOT interop
- [REASONER_SPEC.md](REASONER_SPEC.md) — OntoLogos-backed reasoners (0.9.0 → 1.0.0)
- [SHACL_SPEC.md](SHACL_SPEC.md) — SHACL validation (P1)
- [SEMANTIC_DIFF_SPEC.md](SEMANTIC_DIFF_SPEC.md) — semantic ontology diff
- [LSP_SPEC.md](LSP_SPEC.md) — target language server (v1.0 methods)
- [PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) — plugin system

### UX

- [UI specification pack](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/README.md) — product design, design system, workspace model, AI/plugin UX (**target**; see [ROADMAP_MAPPING.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md) for shipped vs planned)
- [UI_WIREFRAMES.md](UI_WIREFRAMES.md) — text-based VS Code wireframes (legacy; see [WORKSPACE_WIREFRAMES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/WORKSPACE_WIREFRAMES.md))
- [OntoCode_React_UI_Integration_Plan.md](OntoCode_React_UI_Integration_Plan.md) — **React webview migration** (v0.7a → v1.0, [ADR-0017](adr/0017-react-webview-ui.md); panels shipped v0.7–v0.11)

### Historical / backlog

- [MVP_BACKLOG.md](MVP_BACKLOG.md) — v0.1/v0.2 backlog
- [adr/README.md](adr/README.md) — architecture decision records
