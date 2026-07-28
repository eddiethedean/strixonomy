# ADR-0016 — Dependency-First Implementation

## Status

Accepted

## Context

Strixonomy/Strixonomy spans indexing, diagnostics, authoring, reasoning, diff, and IDE integration. Reimplementing parsers, reasoners, triple stores, or format libraries would duplicate mature Rust ecosystem work and drift from sibling project [OntoLogos](https://github.com/eddiethedean/ontologos).

Early milestones already delegate:

- **Oxigraph** — RDF/SPARQL ([ADR-0003](0003-use-oxigraph.md))
- **sqlparser** — SQL virtual tables ([ADR-0011](0011-use-sqlparser-for-sql.md))
- **OntoLogos** — reasoning ([ADR-0015](0015-adopt-ontologos-reasoner.md))

Remaining specs (`strixonomy-owl`, `strixonomy-diagnostics`, `strixonomy-diff`, OBO, SHACL) lacked a single policy for choosing external crates vs in-house code.

## Decision

Adopt a **dependency-first** strategy. Canonical inventory: [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md).

### Rules

1. **Do not reimplement** parsers, reasoners, triple stores, OBO parsers, or SHACL engines when a maintained Rust crate covers the profile.
2. **`strixonomy-*` crates are thin facades** — catalog bridge, LSP JSON, CLI, caching, limits, VS Code integration. They do not embed ontology calculus.
3. **OntoLogos is the sole reasoner orchestration layer** ([ADR-0015](0015-adopt-ontologos-reasoner.md)). No parallel EL/RL/DL stacks (`whelk-rs`, `reasonable`, `owl-dl-core` direct).
4. **External CLIs** (ROBOT) are subprocess adapters only — never reimplement merge/report logic ([OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md)).
5. **When no crate exists**, document logic as **intentionally in-house** with explicit scope (quality lints, semantic diff UX, LSP custom protocol).
6. **Upstream gaps** — track issues/PRs upstream; do not silently fork (aligned with [OntoLogos dependency-first](https://github.com/eddiethedean/ontologos/blob/main/docs/internal/design/dependency-first.md)).

### Adding a dependency

1. Update [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md).
2. Note license in [LICENSES.md](../LICENSES.md) if not MIT/Apache-2.0.
3. Prefer workspace `[workspace.dependencies]` pin in root `Cargo.toml`.

## Consequences

Positive:

- Smaller Strixonomy surface; faster milestones.
- Shared quality gates with OntoLogos conformance tests for reasoning.
- Clear contributor guidance before inventing new crates.

Negative:

- Multiple ontology representations in memory (Oxigraph triples, Horned-OWL axioms, `ontologos_core::Ontology`) until bridges mature.
- LGPL-3.0 `horned-owl` requires license documentation ([LICENSES.md](../LICENSES.md)).
- MSRV rises to **1.88** when Horned-OWL / OntoLogos integrate.

## Appendix A — `strixonomy-owl` parsing: horned-owl vs ontologos-parser

**Question:** Should `strixonomy-owl` depend on `horned-owl` directly or on `ontologos-parser`?

| Criterion | `horned-owl` direct | `ontologos-parser` |
|-----------|---------------------|-------------------|
| Authoring / patch write-back | Native `Ontology` axiom API | Extra bridge from `ontologos_core` |
| Manchester / functional syntax | `horned-functional` integration | Indirect |
| Reasoner input alignment | Requires horned-owl → ontologos bridge at classify time | Same model as `strixonomy-reasoner` |
| License | LGPL-3.0 | MIT OR Apache-2.0 wrapper |
| Coupling | OWL library only | Couples authoring crate to OntoLogos release cycle |

**Decision:**

- **`strixonomy-owl` → `horned-owl` + `horned-functional` direct** for axiom model, Manchester, patch write-back, and semantic diff ([ADR-0013](0013-dual-stack-oxigraph-horned-owl.md)).
- **`strixonomy-reasoner` → `ontologos-parser`** for loading workspace files into OntoLogos ([ADR-0015](0015-adopt-ontologos-reasoner.md)).
- **`strixonomy-owl` (or shared bridge module)** converts Horned-OWL ontology → OntoLogos input when invoking the reasoner; avoid a second in-tree parser.

Rationale: authoring needs fine-grained Horned-OWL axiom objects; reasoning already standardizes on OntoLogos. Splitting parse paths matches the dual-stack rule (Oxigraph for triples, Horned-OWL for edit/diff, OntoLogos for classify).

## Appendix B — v0.30 SQL joins: sqlparser vs DataFusion

**Question:** How to deliver joins and aggregations at v0.30 ([v0.30_BACKLOG.md](../v0.30_BACKLOG.md))?

| Approach | Pros | Cons |
|----------|------|------|
| Extend `sqlparser` virtual tables | Consistent with v0.2; lightweight; no Arrow dep | Limited optimizer; manual join implementation |
| Revive DataFusion ([ADR-0004](0004-use-datafusion-for-sql.md)) | Mature execution, aggregations, export | Heavy dependency; virtual table integration cost |

**Decision:**

1. **v0.30 default:** extend hand-rolled virtual-table joins/aggregations in `strixonomy-query` using existing `sqlparser` AST.
2. **Trigger DataFusion revisit** if any of: (a) join queries exceed maintainability, (b) Parquet/Arrow export becomes P0, (c) aggregations need optimizer behavior beyond hand-rolled plans.
3. Document evaluation outcome in [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md) when decided.

## Related

- [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md)
- [LICENSES.md](../LICENSES.md)
- [ADR-0015](0015-adopt-ontologos-reasoner.md)
- [ADR-0013](0013-dual-stack-oxigraph-horned-owl.md)
