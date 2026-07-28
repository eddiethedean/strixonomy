# ADR-0020: Semantic transaction edit model

**Status:** Accepted  
**Date:** 2026-07-13  
**Target:** Strixonomy v0.19.0

## Context

Ontology edits today flow through format-specific patch engines:

- Turtle: `strixonomy-owl::PatchOp` + span surgery
- OBO: `strixonomy-obo::OboPatchOp` + stanza surgery

LSP and CLI call those engines directly. This blocks format-independent editing,
consistent undo semantics, and RDF/XML / OWL/XML write-back (v0.21).

## Decision

Introduce **`strixonomy-edit`** as the canonical apply-path layer:

1. **`Transaction`** — ordered `SemanticChange` list with compose, validate, invert, JSON serde.
2. **`SemanticChange`** — wraps `PatchOp` or `OboPatchOp` until unified axiom modeling lands.
3. **Format adapters** — map transactions to existing Turtle/OBO `apply_patches*` functions.
4. **Wire compatibility** — legacy patch JSON arrays remain accepted; optional `{ "transaction": … }` envelope.

Product code (LSP `applyAxiomPatch`, CLI `strixonomy patch`) routes through `Transaction`.

## Consequences

**Positive**

- Single apply path for Turtle and OBO
- Invertible transactions for undo algebra and tests
- Foundation for RDF/XML and OWL/XML serializers (v0.21)
- Parity manifest and CI can track format-independence progress

**Negative**

- Extra indirection layer until serializers consume a richer semantic model
- Some `Set*` patch ops are not invertible without prior-value capture (documented; `invert()` returns error)

## Non-goals (v0.20)

- Rewriting Turtle/OBO text engines
- Workspace transaction manager (v0.20)
- Extension ontology-level undo stack (VS Code text undo remains user-facing)

## Related

- [ADR-0006](0006-patch-based-write-back.md) — patch write-back
- [ADR-0019](0019-obo-write-back.md) — OBO patches
- [BLOCKER_01](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/04_BLOCKERS/BLOCKER_01_FORMAT_INDEPENDENCE.md)
- [V0_30_PHASES](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md) § v0.19
