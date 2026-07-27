# ADR-0008 — Use Reasoner Adapters Instead of Building a Native Reasoner First

## Status
Accepted

## Context
Reasoners are complex, and existing OWL reasoners are mature.

## Decision
Integrate reasoners through adapters rather than inventing a novel calculus from scratch on day one.

## Amendment ([ADR-0014](0014-rust-native-reasoners-only.md), [ADR-0015](0015-adopt-ontologos-reasoner.md))

v0.6+ adapters are **Rust-native only**, delegating to **[OntoLogos](https://github.com/eddiethedean/ontologos)** (`ontologos-el`, `ontologos-rl`, `ontologos-dl`, etc.). JVM reasoners (ELK, HermiT, Pellet) are **explicitly excluded**.

## Consequences
Positive:
- pluggable reasoner profiles (EL / RL / RDFS / DL)
- avoids maintaining a monolithic reasoner in Strixonomy — `strixonomy-reasoner` is a thin facade
- Rust adapters ship inside the binary — no external runtime
- shared HermiT conformance suite with OntoLogos

Negative:
- v1.0 DL quality depends on OntoLogos 1.0.0 publish ([REASONER_SPEC.md](../REASONER_SPEC.md))
- must validate against fixtures rather than delegating to HermiT JVM
