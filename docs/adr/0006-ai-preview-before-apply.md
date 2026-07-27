# ADR-0006 — AI changes require preview before apply

## Status

Accepted — **proposed v1.1+** (AI workflows)

## Context

AI-assisted editing must not bypass Strixonomy validation or user consent ([design/adr/0010-ai-features-opt-in.md](../design/adr/0010-ai-features-opt-in.md)). Patch-based write-back ([design/adr/0006-patch-based-write-back.md](../design/adr/0006-patch-based-write-back.md)) already supports preview.

## Decision

All AI-generated ontology changes MUST follow:

1. Generate proposal (structured patch or refactor ops)
2. Validate via Strixonomy
3. Show preview to user
4. Require explicit approval
5. Apply via existing LSP apply paths
6. Audit log entry

No silent apply. Opt-in required for AI features at workspace or org level.

## Consequences

**Positive:** Aligns AI with refactoring safety model; reduces bad-edit risk.

**Negative:** Extra UX step; latency for large proposals.

## References

- [platform/AI_ORCHESTRATION.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/AI_ORCHESTRATION.md)
- [platform/SEMANTIC_REFACTORING.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/SEMANTIC_REFACTORING.md)
