# Enterprise week-2 playbook

Continue after the [first-week Protégé migration](protege-migration.md) and [production readiness](production-readiness.md) pilot criteria. This page assumes week 1 already covered install, First success, and basic `validate` CI.

## Goals

1. Prove **DL / realize / SWRL / DL Query** on a **representative** corpus (not only fixtures).
2. Define **rollback** criteria before wider rollout.
3. Keep Protégé for workflows Strixonomy still marks as not supported (byte-identical XML, HermiT identity, Protégé-only plugins) — see [known limitations](../known-limitations.md) and [DL Query honesty](dl-query.md).

## Day plan (suggest ~5 working days)

| Day | Focus | Exit when |
|-----|--------|-----------|
| 1 | Pin CLI/VSIX to **0.27.0**; confirm [Versions & channels](versions-and-channels.md) | Same version in IDE + CI |
| 2 | `strixonomy classify --profile dl` (or `auto`) on your corpus | Exit codes understood; profile warnings reviewed |
| 3 | `strixonomy realize` + sample `check-instance` in CI | [realize cookbook](../examples/realize.md) |
| 4 | SWRL: load a known rule set; classify with materialize; dual-check in Protégé if critical | [SWRL examples](../examples/swrl.md); no surprise consequents |
| 5 | Review [known limitations](../known-limitations.md); write rollback note; [DL Query honesty](dl-query.md) decision | Go / no-go for week 3+ |

## CI recipes to add

```bash
strixonomy validate ./ontologies
strixonomy classify ./ontologies --profile dl --format json
strixonomy realize ./ontologies --profile rl --format json
```

Optional dual-check: re-run critical unsatisfiable / realization cases in Protégé + HermiT and record deltas.

## Rollback criteria (examples)

Fail the pilot (or pause IDE-only edits) if any of:

- Unsatisfiable-class false positives vs dual-tool baseline on agreed corpora
- SWRL materialize changes ABox/TBox in ways reviewers cannot explain
- Workspace exceeds [limits](../workspace-limits.md) or CI timeouts become routine
- Authors need Protégé DL Query daily **and** cannot accept Strixonomy’s [DL Query honesty limits](dl-query.md) / HermiT dual-check plan

## Related

- [Enterprise evaluation](enterprise-eval.md)
- [Production readiness](production-readiness.md)
- [Production evidence protocol](production-evidence.md)
- [Plugin policy](plugin-policy.md)
