# ADR-0021: Deterministic semantic serializers for RDF/XML and OWL/XML

**Status:** Accepted  
**Date:** 2026-07-13  
**Target:** Strixonomy v0.21.0

## Context

Turtle and OBO write-back use source-text patches (ADR-0006, ADR-0019). RDF/XML and OWL/XML cannot share that strategy without fragile XML span surgery. v0.21 requires open → edit → save → reload **without semantic loss** ([PRE_1_0_PHASES](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/PRE_1_0_PHASES.md), [BLOCKER_01](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/04_BLOCKERS/BLOCKER_01_FORMAT_INDEPENDENCE.md)).

Horned-OWL already provides RDF/XML and OWL/XML writers. Strixonomy must adopt them as first-class save backends on top of the semantic transaction layer (ADR-0020).

## Decision

1. **Semantic re-serialize** RDF/XML (`.rdf`, `.owl`) and OWL/XML (`.owx`) after applying `PatchOp` transactions to an in-memory Horned ontology.
2. **Semantic fidelity over textual fidelity** — whitespace, comment placement, and prefix declaration style may change; catalog/axiom comparison is the acceptance oracle.
3. **Stable emission rules**
   - Preserve ontology IRI / version IRI when present.
   - Emit imports as Horned `Import` components.
   - Prefer prefix maps supplied by the document / workspace namespaces; supplement with standard `rdf` / `rdfs` / `owl` prefixes.
   - Annotation assertions (including `rdfs:label` / `rdfs:comment`) round-trip as Horned `AnnotationAssertion`.
4. **Round-trip verification** uses a cross-format semantic comparator (entity IRIs, labels, parents, imports, ontology IRI) — not byte equality.
5. Turtle and OBO continue to use existing patch engines; only XML formats use this path.

## Consequences

**Positive**

- Unlocks inspector / CLI / LSP edit for required Protégé interchange formats.
- Reuses Horned as the single OWL model for XML serialize (ADR-0013).
- Aligns with ADR-0020 transaction apply for all formats.

**Negative**

- Save may reorder XML elements and rewrite prefixes.
- Incomplete Horned RDF parses remain a risk; incomplete loads surface as diagnostics rather than silent data loss.

**Follow-ups**

- v0.22 closes remaining OWL 2 authoring gaps across serializers.
- Richer blank-node / anonymous expression coverage as fixture corpus expands.
