# ADR-0013 — Dual Stack: Oxigraph + Horned-OWL

## Status

Accepted (v0.4b+)

## Context

v0.1–v0.2 use Oxigraph RDF parsing and triple-level entity extraction ([ADR-0003](0003-use-oxigraph.md)). That is sufficient for indexing and SPARQL but **insufficient** for Protégé-competitive authoring:

- Complex OWL class expressions are lost or flattened
- Patch-based write-back ([ADR-0006](0006-patch-based-write-back.md)) requires OWL-aware round-trip
- Semantic diff needs axiom-level semantics, not triple grep

[ADR-0002](0002-use-horned-owl.md) originally chose Horned-OWL but was superseded during the MVP. v0.30 requires both layers.

## Decision

Use a **dual stack**:

| Layer | Library | Responsibility |
|-------|---------|----------------|
| **RDF / SPARQL** | Oxigraph | Parse/store triples, SPARQL, triple counts, fast scan |
| **OWL modeling** | Horned-OWL | OWL 2 axiom model, Manchester syntax, class expressions, edit/diff round-trip |

New crate: **`strixonomy-owl`** — Horned-OWL facade (`horned-owl` + `horned-functional` direct per [ADR-0016](0016-dependency-first-implementation.md) Appendix A). Reasoning loads via `ontologos-parser` in `strixonomy-reasoner`, not in `strixonomy-owl`.

### Data flow

```text
Workspace files (.ttl .owl .obo …)
        │
        ├──────────────────┐
        v                  v
   Oxigraph            Horned-OWL
   (triples)           (axioms)
        │                  │
        ├──── catalog ◄────┤
        │   (entities,     │
        │    axioms for    │
        │    edit/diff)    │
        v                  v
   SPARQL + SQL       patch write-back
```

### Sync rules

1. **Catalog entities and axioms** used for editing, diagnostics, and semantic diff come from **Horned-OWL**.
2. **Triple counts and SPARQL** come from **Oxigraph**.
3. **Consistency tests** in CI detect drift between layers on shared fixtures.
4. v0.2 triple-extraction path remains for read-only fast paths until v0.4b migration completes.

## Consequences

Positive:

- Protégé-competitive axiom editing and Manchester round-trip become feasible
- Semantic diff operates on axioms, not ad-hoc triple patterns
- Oxigraph keeps SPARQL performance without reimplementing a triple store

Negative:

- Two parse pipelines per file until incremental sync is optimized
- Horned-OWL + Oxigraph dependency coordination (version pinning)
- Team must understand which layer owns which feature

## Supersedes

- [ADR-0002](0002-use-horned-owl.md) — recommitted via this ADR for v0.4b+

## Related

- [OWL_AUTHORING_SPEC.md](../OWL_AUTHORING_SPEC.md)
- [PROTEGE_PARITY.md](../PROTEGE_PARITY.md)
- [ARCHITECTURE.md](../ARCHITECTURE.md)
