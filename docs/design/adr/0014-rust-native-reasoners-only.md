# ADR-0014 — Rust-Native Reasoners Only (No JVM)

## Status

Accepted

## Context

Earlier specs assumed **ELK** and **HermiT** JVM adapters for v0.30 ([ADR-0008](0008-reasoner-adapters-not-built-in-reasoner.md), prior [REASONER_SPEC.md](../REASONER_SPEC.md)). That conflicts with Strixonomy’s Rust-first, local-first goals: Java install friction, packaging complexity, and a hard dependency outside the shipped binary.

Rust OWL reasoners exist today, but none is a drop-in HermiT replacement with identical coverage and battle history:

| Crate / component | Profile | Notes |
|-------------------|---------|-------|
| [whelk-rs](https://github.com/INCATools/whelk-rs) | OWL EL | Horned-OWL ecosystem; experimental; ideal for OBO-scale TBoxes |
| [reasonable](https://github.com/gtfierro/reasonable) | OWL 2 RL | Datalog materialization; fast; not full DL |
| Horned-OWL reasoner interface | — | Trait defined; no bundled DL engine ([Horned-OWL paper](https://drops.dagstuhl.de/entities/document/10.4230/TGDK.2.2.9)) |

Full **OWL 2 DL** classification, consistency, and unsatisfiability explanations are provided by **[OntoLogos](https://github.com/eddiethedean/ontologos)** `ontologos-dl` (1.0.0 workspace, HermiT parity in progress), integrated via `strixonomy-reasoner` per [ADR-0015](0015-adopt-ontologos-reasoner.md).

## Decision

1. **Never depend on Java or JVM reasoners** (ELK, HermiT, Pellet, etc.) in Strixonomy or Strixonomy — not at v0.30, not as optional adapters.
2. Ship **Rust reasoner adapters** behind the existing `ReasonerAdapter` trait ([REASONER_SPEC.md](../REASONER_SPEC.md)), delegating to **OntoLogos** ([ADR-0015](0015-adopt-ontologos-reasoner.md)):
   - **`el`** — `ontologos-el` for OWL EL classification (default for OBO / large terminologies).
   - **`rl`** / **`rdfs`** — `ontologos-rl` / `ontologos-rdfs` where RL/RDFS semantics suffice.
   - **`dl`** — `ontologos-dl` (requires OntoLogos **1.0.0** crates.io publish).
   - **`auto`** — `ontologos-facade` profile routing (1.0.0+).
3. **Plugin reasoners** must be native binaries or WASM — not JVM subprocesses ([PLUGIN_SPEC.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/PLUGIN_SPEC.md)).
4. **ROBOT** remains an optional external CLI for OBO release pipelines; it is not used for in-IDE classification ([OBO_ROBOT_SPEC.md](../OBO_ROBOT_SPEC.md)).

## Consequences

Positive:

- Single-language stack; no Java bootstrap wizard or JAR management.
- Reasoning works offline in the VSIX / `strixonomy` binary.
- Aligns with Horned-OWL + Oxigraph dual stack ([ADR-0013](0013-dual-stack-oxigraph-horned-owl.md)).

Negative:

- Strixonomy v0.30 DL timeline **depends on OntoLogos 1.0.0** HermiT parity, not only Strixonomy UI work.
- EL-only ontologies may classify faster via `el` than `dl`; users need profile guidance in UI.
- Parity with every Protégé + HermiT edge case is validated by OntoLogos conformance tests, not assumed.

## Implementation notes

- v0.6: `strixonomy-reasoner` + `el` / `rl` / `rdfs` adapters; pin `ontologos-* = "0.9"`.
- v0.30: enable `dl` and `auto` adapters; bump to `ontologos-* = "1.0"`; **real** clash-trace explanations (P0 in [PROTEGE_PARITY.md](../PROTEGE_PARITY.md)).
- Do not block on JVM cross-checks in CI; align with [OntoLogos HermiT parity](https://github.com/eddiethedean/ontologos/blob/main/docs/internal/hermit-parity-gap-report.md).

## Related

- Supersedes JVM-specific guidance in prior REASONER_SPEC and amends [ADR-0008](0008-reasoner-adapters-not-built-in-reasoner.md) (adapters remain; JVM adapters are excluded).
- [ADR-0015](0015-adopt-ontologos-reasoner.md) — OntoLogos as reasoner backend.
