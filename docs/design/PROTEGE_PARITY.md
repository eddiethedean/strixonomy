# Protégé Parity Matrix (v1.0.0)

> **Superseded for planning.** The authoritative 1.0 engineering program is
> [docs/protege-parity/](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/README.md) — especially
> [PRE_1_0_PHASES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md),
> [PARITY_SCOPE.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/PARITY_SCOPE.md), and
> [PARITY_RELEASE_GATE.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/03_PARITY/PARITY_RELEASE_GATE.md).
>
> **This page** remains a historical v0.18-era P0/P1/P2 checklist and migration
> reference. Do not use it as the live 1.0 implementation plan.
>
> See [Platform roadmap § Strixonomy 1.0](../roadmap.md#strixonomy-10-modern-protege-replacement) and [PLAN.md](PLAN.md) §4.
>
> **Status column:** frozen at **v0.18.0** — **not current** for v0.19+ adoption decisions. Use [What ships today](../SHIPPED.md), [Known limitations](../known-limitations.md), and [Protégé vs Strixonomy](../guides/protege-decision.md) instead.
>
> **v0.21 update:** RDF/XML and OWL/XML write-back **shipped** (semantic re-serialize). Rows below that still say “OWL/XML write-back is v1.0” are historical — ignore them for adoption.
> **Dependencies:** [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md)

## Tier definitions

| Tier | Meaning |
|------|---------|
| **P0 — v1.0 blocker** | Must ship; v1.0 cannot release without green |
| **P1 — v1.0 target** | Expected at launch; documented known gaps if slipped |
| **P2 — post-1.0** | Explicitly out of v1.0 scope |

## P0 — v1.0 blockers

### OWL 2 DL authoring (hybrid UX)

| Item | Spec | Dependency | Status (v0.18) |
|------|------|------------|----------------|
| Quick forms: labels, comments, deprecated | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | **Yes** (Turtle + OBO) |
| Quick forms: `SubClassOf`, domain, range, property characteristics | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | **Yes** (Turtle) |
| Manchester editor for complex class expressions | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-functional` | **Yes** (panel + side completions; no inline buffer autocomplete — v1.0 polish) |
| Axiom types: `SubClassOf`, `EquivalentClasses`, `DisjointClasses` | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | **Yes** |
| Object/data property domain, range, characteristics | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | **Yes** (Turtle) |
| Class/object/data property assertions on individuals | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | **Yes** (Turtle) |
| Annotation assertions | [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md) | `horned-owl` | **Yes** (Turtle) |
| Horned-OWL manipulation layer + round-trip tests | [ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md) | `horned-owl`, `horned-functional` | **Yes** (Turtle catalog + `protege-roundtrip` fixtures) |
| Patch-based write-back from OWL objects | [ADR-0006](adr/0006-patch-based-write-back.md) | in-house patches | **Yes** (Turtle + OBO; OWL/XML write-back is v1.0) |

### Reasoning (Rust-native — [ADR-0014](adr/0014-rust-native-reasoners-only.md))

| Item | Spec | Dependency | Status (v0.18) |
|------|------|------------|----------------|
| `el` adapter (OWL EL) | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-el` | **Yes** |
| `rl` / `rdfs` adapters | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-rl`, `ontologos-rdfs` | **Yes** |
| `dl` adapter (OWL 2 DL classification + consistency) | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-dl` 1.0.0 | **Yes** |
| Unsatisfiable class reporting | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos | **Yes** |
| **Real** unsatisfiability explanations (clash-trace justification) | [REASONER_SPEC.md](REASONER_SPEC.md) | `ontologos-explain` + `ontologos-dl` | **Partial** — EL traces + alternatives; DL uses EL/RL/RDFS fallback with labeled profile (Ontologos DL has no native proof traces yet) |
| Asserted / inferred / combined hierarchy toggle | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos | **Yes** |
| Consistency check | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos | **Yes** (distinct Consistency command; class-level) |
| Zero JVM / Java dependency for reasoning | [ADR-0014](adr/0014-rust-native-reasoners-only.md) | — | **Yes** |

### Editor & LSP

| Item | Spec | Status (v0.18) |
|------|------|----------------|
| Diagnostics → Problems panel | [SPEC.md](SPEC.md) §9 | **Yes** |
| Completion | [SPEC.md](SPEC.md) §9 | **Yes** (Turtle prefix/QName/IRI) |
| Rename (safe IRI) | [SPEC.md](SPEC.md) §9 | **Yes** |
| Find references | [SPEC.md](SPEC.md) §9 | **Yes** |
| Code actions | [SPEC.md](SPEC.md) §9 | **Yes** |
| Semantic tokens | [SPEC.md](SPEC.md) §9 | **Yes** (Turtle, OBO) |
| Hover, go-to-definition, symbols | [docs/lsp-api.md](../lsp-api.md) | **Yes** |

### Workflow & platform

| Item | Spec | Status (v0.18) |
|------|------|----------------|
| Imports management UI | [SPEC.md](SPEC.md) | **Yes** |
| SQL + SPARQL query workbench | [SPEC.md](SPEC.md) | **Yes** |
| Semantic diff + branch/version compare | [SEMANTIC_DIFF_SPEC.md](SEMANTIC_DIFF_SPEC.md) | **Yes** |
| Safe IRI rename across workspace | [ROADMAP.md](ROADMAP.md) v0.8 | **Yes** |
| Graph visualization (class, property, import, neighborhood) | [ROADMAP.md](ROADMAP.md) v0.7 | **Yes** (asserted/inferred/combined, export JSON/CSV, expand depth) |
| Documentation export (Markdown + HTML) | [ROADMAP.md](ROADMAP.md) v0.9 | **Yes** (`strixonomy docs`) |
| CI validation command | CLI `validate` | **Yes** |
| VS Code Marketplace publish | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) | **Yes** |
| Migration guide from Protégé | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) | **Yes** ([protege-migration.md](../guides/protege-migration.md) + honest gaps) |

### OBO & biomedical

| Item | Spec | Dependency | Status (v0.18) |
|------|------|------------|----------------|
| OBO format read/write | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | `fastobo`, `fastobo-owl` | **Yes** |
| ROBOT interop (`validate`, `merge`, `report`) | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | ROBOT CLI | **Yes** |
| OBO id rendering in explorer / Manchester autocomplete | [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) | `fastobo` | **Yes** (explorer `obo_id`; Manchester side completions) |

## P1 — v1.0 targets

| Item | Spec | Dependency | Notes |
|------|------|------------|-------|
| SHACL validation via adapter | [SHACL_SPEC.md](SHACL_SPEC.md) | `rudof` | Scaffold plugin exists; full rudof adapter open |
| SWRL rule **viewing** (authoring is P2) | [PROTEGE_PARITY.md](PROTEGE_PARITY.md) | in-house | Open |
| Instance checking | [REASONER_SPEC.md](REASONER_SPEC.md) | OntoLogos `ontologos-abox` (1.0+) | Open |
| Plugin API stable + 3 reference plugins | [PLUGIN_SPEC.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md) | — | Host MVP + 3 refs shipped; **semver-stable** API is v1.0 |
| SQL joins and aggregations | [SPEC.md](SPEC.md) | `sqlparser` extend | Open (subset SQL today) |
| Incremental workspace index | [ARCHITECTURE.md](ARCHITECTURE.md) | — | **Shipped** (v0.9+) |
| Performance benchmarks (large ontology targets) | [v1.0_BACKLOG.md](v1.0_BACKLOG.md) | — | Open (truncation messaging shipped) |
| OWL/XML · RDF/XML write-back | [known-limitations](../known-limitations.md) | Horned | Explicit v1.0 authoring gap |
| Full DL axiom catalog UI / inline Manchester autocomplete | [SHIPPED.md](../SHIPPED.md) | — | Open polish |
| Native DL clash-trace proofs (no EL fallback) | Ontologos | `ontologos-dl` proofs | Blocked on reasoner; fallback labeled today |

## P2 — post-1.0

| Item | Notes |
|------|-------|
| WebProtégé / collaborative editing | [PLAN.md](PLAN.md) §9 non-goal |
| Full SWRL authoring | |
| Protégé plugin compatibility | |
| Protégé-scale plugin marketplace | v1.0 ships API + reference plugins only |
| Reimplementing ROBOT | Interop only per [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md) |

## Honest scope statement

**“Compete with Protégé” at v1.0 means:**

- Primary IDE for **ontology engineering in VS Code** (general OWL 2 DL + OBO maintenance).
- **Hybrid authoring** (forms + Manchester) and **real reasoning** match Protégé’s core loop.
- **Semantic diff, CI, and VS Code integration** exceed Protégé.

**It does not mean:**

- Every Protégé tutorial works unchanged.
- SWRL authoring, WebProtégé, or the full Protégé plugin catalog.
- Bit-for-bit identical results to HermiT on every ontology (Rust DL engine is test-validated, not JVM-cross-checked).

## Pre-1.0 (Era E) closeout

Through **v0.18**, the Desktop parity gate is closed for the agreed scope (menus/dialogs, reasoner lifecycle + cancel, layout reopen, explanation stale UX, fixtures, migration docs). Remaining P0 **Partial** is DL native clash traces (Ontologos limitation with labeled fallback). Remaining authoring gaps that are still P0-adjacent for v1.0: OWL/XML write-back and full DL axiom catalog polish.

## v1.0 exit criterion

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable in VS Code.
> Protégé is required only for **P2** features listed above.
