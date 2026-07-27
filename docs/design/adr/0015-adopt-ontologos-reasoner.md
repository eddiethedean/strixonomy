# ADR-0015 — Adopt OntoLogos as Reasoner Backend

## Status

Accepted

## Context

[ADR-0014](0014-rust-native-reasoners-only.md) requires Rust-native reasoning with no JVM. Prior plans assumed Strixonomy would ship direct adapters to `whelk-rs` and `reasonable`, plus an **in-tree OWL 2 DL engine** in `strixonomy-reasoner` ([REASONER_SPEC.md](../REASONER_SPEC.md)).

[OntoLogos](https://github.com/eddiethedean/ontologos) is a sibling Rust workspace (same maintainer) that already provides:

| OntoLogos crate | Role |
|-----------------|------|
| `ontologos-core` | In-memory OWL ontology model |
| `ontologos-parser` | OWL/RDF load via horned-owl |
| `ontologos-profile` | OWL profile detection |
| `ontologos-bridge` | core ↔ horned-owl ↔ reasonable adapters |
| `ontologos-rdfs` | RDFS materialization (→ reasonable) |
| `ontologos-rl` | OWL RL saturation (→ reasonable) |
| `ontologos-el` | OWL EL classification (in-house) |
| `ontologos-query` | Taxonomy queries |
| `ontologos-explain` | Proof graphs and explanations |
| `ontologos-facade` | Multi-profile routing (`classify --profile auto`) — **1.0.0 workspace** |
| `ontologos-dl` | OWL 2 DL reasoner — **1.0.0 workspace, HermiT parity in progress** |
| `ontologos-watch` | File-watch reload — scoped as **Strixonomy hook** |

**crates.io today:** nine crates at **0.9.0** (EL, RL, RDFS, explain, query).  
**GitHub `main`:** workspace **1.0.0** toward HermiT catalog parity (~64% at 2026-06-23).

Building a second DL reasoner and duplicate EL/RL facades inside Strixonomy would fork effort and drift from OntoLogos conformance tests.

## Decision

1. **`strixonomy-reasoner` is a thin integration crate**, not an in-tree DL engine. It implements Strixonomy's `ReasonerAdapter` trait ([REASONER_SPEC.md](../REASONER_SPEC.md)) by delegating to OntoLogos crates.
2. **Version policy:**

   | Strixonomy milestone | OntoLogos dependency | Capabilities unlocked |
   |--------------------|----------------------|------------------------|
   | **v0.6** (initial reasoning) | **0.9.0** (crates.io) | EL (`ontologos-el`), RL (`ontologos-rl`), RDFS (`ontologos-rdfs`), profile detection, taxonomy query, EL-first explanations |
   | **v1.0** (DL parity gate) | **≥ 1.0.0** (crates.io publish) | `ontologos-dl`, `ontologos-facade` auto-routing, production DL explanations via `ontologos-explain` |
   | **v0.9** (incremental index) | **0.9.0+** | Evaluate `ontologos-watch` for file-change → reclassify hook |

3. **Adapter mapping** (Strixonomy surface → OntoLogos backend):

   | `ReasonerAdapter` id | OntoLogos backend | Notes |
   |----------------------|-------------------|-------|
   | `el` | `ontologos-el` | Default for OBO / EL-detectable TBoxes |
   | `rl` | `ontologos-rl` | OWL RL saturation |
   | `rdfs` | `ontologos-rdfs` | Explicit RDFS materialization |
   | `dl` | `ontologos-dl` | **Requires OntoLogos 1.0.0**; MVP blocked until publish |
   | `auto` | `ontologos-facade` | **Requires OntoLogos 1.0.0**; profile routing |

4. **Do not depend on `whelk-rs` directly** in Strixonomy. EL classification goes through `ontologos-el` (OntoLogos may benchmark against whelk-rs internally).
5. **Do not depend on `reasonable` directly** in Strixonomy. RL/RDFS go through `ontologos-rl` / `ontologos-rdfs`.
6. **Dual-stack boundary ([ADR-0013](0013-dual-stack-oxigraph-horned-owl.md)):** Oxigraph remains authoritative for SPARQL and triple-level SQL tables. Reasoning input is built from workspace files via `ontologos-parser` (or bridged from `strixonomy-owl` once v0.4b lands). Classification results are written back into the Strixonomy catalog cache for LSP/explorer inferred views.
7. **Honest parity:** Strixonomy v1.0 DL exit criteria track **OntoLogos 1.0.0 HermiT parity**, not a separate in-tree engine. Migration guide cites [OntoLogos supported constructs](https://github.com/eddiethedean/ontologos/blob/main/docs/reference/supported-constructs.md).

## Consequences

Positive:

- Single Rust reasoner stack across CLI, LSP, and future Python tooling.
- OntoLogos HermiT conformance suite becomes the shared quality gate.
- `strixonomy-reasoner` stays small: trait, catalog bridge, cache, LSP JSON mapping.
- `ontologos-watch` provides a ready integration point for v0.9 incremental workflows.

Negative:

- Strixonomy v1.0 DL timeline **depends on OntoLogos 1.0.0 crates.io publish**, not only Strixonomy UI work.
- Two ontology models in memory (Oxigraph catalog + `ontologos_core::Ontology`) until bridge optimization.
- Partial OWL mapping in OntoLogos (`axiom_count()` ≠ Protégé totals) applies to Strixonomy reasoning until OntoLogos closes gaps.

## Implementation notes

- v0.6: add `strixonomy-reasoner` with `el`, `rl`, `rdfs` adapters; pin `ontologos-* = "0.9.0"`.
- v0.6 UI: reasoner profile picker shows `el` / `rl` / `rdfs`; `dl` disabled with "requires OntoLogos 1.0" until publish.
- v1.0: bump to `ontologos-* = "1.0"`; enable `dl` and `auto` adapters; wire explanation panel to `ontologos-explain`.
- CI: run OntoLogos golden fixtures alongside Strixonomy integration tests on shared `fixtures/`.

## Related

- Amends [ADR-0014](0014-rust-native-reasoners-only.md) (Rust-native via OntoLogos, not in-tree DL).
- Amends [ADR-0008](0008-reasoner-adapters-not-built-in-reasoner.md) (adapters delegate to OntoLogos).
- [ADR-0016](0016-dependency-first-implementation.md) — dependency-first implementation policy
- [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md) — canonical crate inventory
- [REASONER_SPEC.md](../REASONER_SPEC.md) — adapter table
- [OntoLogos ROADMAP](https://github.com/eddiethedean/ontologos/blob/main/ROADMAP.md) — HermiT parity phases
