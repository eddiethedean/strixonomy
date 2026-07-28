# Production evidence protocol

Strixonomy does **not** publish third-party case studies or customer logos. Before a Fortune 500 rollout, run this **self-benchmark protocol** on your own ontology corpus and record results for your risk review.

Canonical limits: [workspace limits](../workspace-limits.md). Pilot criteria: [production readiness](production-readiness.md).

> **Note:** Tutorial fixtures (~88 triples) are smoke tests only — not representative of enterprise terminologies.

## When to run this

- Before expanding an IDE pilot beyond a single team
- Before mandating `strixonomy validate` / `classify` on production branches
- Before retiring Protégé for any workflow Strixonomy will own

## Prerequisites

- Pin Strixonomy **0.28.0** — `cargo install strixonomy-cli --locked --version 0.28.0` or release binary with SHA256 verification — [release integrity](../release-integrity.md)
- A **representative clone** of your production ontology tree (not sanitized tutorial data)
- Record host OS, CPU, RAM, and CI runner specs

## Step 1 — Catalog sizing

```bash
ONTO=/path/to/your/ontologies
strixonomy inspect "$ONTO" --format json
```

Record:

| Metric | Your value | Tier guidance |
|--------|------------|---------------|
| `ontology_count` | | See [performance sizing](performance-sizing.md) |
| `triple_count` | | Must be &lt; 20M |
| `class_count` | | |
| `diagnostic_error_count` | | |

**Pass:** Counts within [workspace limits](../workspace-limits.md). **Fail:** Split workspace or shard by module — incremental indexing shipped in v0.10 ([release timeline](release-timeline.md)).

## Step 2 — CI latency budget

```bash
time strixonomy validate "$ONTO"
```

Optional:

```bash
time strixonomy classify "$ONTO" --profile el --format json
```

**Suggested acceptance (adjust for your pipeline):**

| Gate | Suggested target |
|------|------------------|
| `validate` on `main` corpus | Completes within your CI stage budget (e.g. &lt; 5 min) |
| `classify` (if used) | Completes within reasoner stage budget; profile matches ontology |

**Fail:** Narrow path (`strixonomy validate ./src/ontologies`), shard by module, or run classify on a schedule instead of every PR.

## Step 3 — Reasoner comparison (if using Protégé today)

On the same corpus:

1. Run Protégé DL/EL classification (your current standard)
2. Run `strixonomy classify "$ONTO" --profile el --format json`
3. Compare `unsatisfiable` class lists and materialized edges

**Document:** Matches, false positives, false negatives, and profile warnings. Strixonomy docs state results **may differ** from Protégé — this step quantifies risk for your org.

## Step 4 — Authoring spot check (IDE pilot)

On 3–5 real edit tasks (label change, parent add, Manchester axiom, IRI rename):

1. Perform in Strixonomy on `.ttl` files
2. Run `strixonomy validate` and review Git diff
3. Optional: re-open in Protégé to confirm round-trip

**Pass:** Edits persist, CI clean, team accepts diff quality. **Fail:** Keep Protégé for that workflow — [Protégé decision matrix](protege-decision.md).

## Step 5 — Record evidence

Store internally:

| Artifact | Purpose |
|----------|---------|
| `inspect` JSON output | Sizing proof |
| CI timing logs | Latency proof |
| Reasoner diff notes | Parity risk |
| Pinned version + SHA256 | Supply chain |
| `NOTICES` from release | Legal review |

## What this protocol does not prove

- Long-term maintainer SLA (see [Governance](governance.md))
- Security pen-test results (not published)
- Remote SSH / Codespaces certification (see [Platform compatibility](platform-compatibility.md))
- Formal benchmark suite (v1.0 backlog)

## Related

- [Performance and sizing](performance-sizing.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Protégé decision matrix](protege-decision.md)
