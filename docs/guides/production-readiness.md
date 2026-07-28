# Production readiness and pilot criteria

This page states what Strixonomy / Strixonomy **v0.26.2** (latest tagged) is appropriate for in production-like environments. It is not legal advice and does not replace your organization's risk review.

Canonical capability matrix: [What ships today](../SHIPPED.md). Follow-on pilot weeks: [Enterprise week-2 playbook](enterprise-week-2.md).

## Maturity model

| Level | Version | Meaning |
|-------|---------|---------|
| **Pre-1.0** | **0.26.x (latest tagged)** | Pin `cargo install ontocore-cli --locked --version 0.26.2` in CI. Library APIs may change until [v1.0](../design/v1.0_BACKLOG.md). |
| **Stable CI gates** | 0.26.x | `strixonomy validate`, `strixonomy classify`, `strixonomy realize`, `strixonomy check-instance`, and `strixonomy diff` are documented for CI — see [workspace limits](../workspace-limits.md). |
| **In development** | Next unreleased minor on `main` | May preview upcoming work — pin installs to [TAGGED_RELEASE](../TAGGED_RELEASE), not workspace `Cargo.toml`. |
| **v1.0 target** | Planned | Protégé-competitive OWL 2 DL + OBO in VS Code — [Protégé vs Strixonomy](protege-decision.md); capability truth: [SHIPPED](../SHIPPED.md) + [known limitations](../known-limitations.md). |

Strixonomy **v0.26** is suitable for pilot IDE editing, Linux CI validate/classify/realize/dl-query, and coexistence with Protégé — **not** an org-wide Protégé retirement. RDF/XML and OWL/XML write-back are semantic re-serialize (ships since v0.21). Realization, instance checking, and SWRL (DLSafe + classify materialize) ship since v0.23. Query Workbench **DL** mode / `strixonomy dl-query` ships in v0.24 — see honesty notes in [DL Query vs Query Workbench](dl-query.md) (not full Protégé DL Query tab parity).

## Approved use cases (pilot or production)

| Use case | v0.26 readiness | Notes |
|----------|-----------------|-------|
| CI lint gate on ontology repos | **Suitable** | `strixonomy validate` — [CI integration](../ci-integration.md) |
| CI consistency gate (EL profile) | **Suitable** | `strixonomy classify --profile el` — profile must match ontology |
| CI consistency gate (DL profile) | **Pilot** | `strixonomy classify --profile dl` or `auto` — OntoLogos 1.x; verify on your corpus |
| CI realization / instance checks | **Pilot** | `strixonomy realize` / `strixonomy check-instance` — [realize cookbook](../examples/realize.md) |
| SWRL materialize on classify | **Pilot** | Rules via IDE / LSP / patches; no `strixonomy swrl` CLI — [SWRL examples](../examples/swrl.md) |
| Developer IDE for Turtle/OBO authoring | **Pilot** | Turtle + OBO write-back; pre-1.0 extension APIs |
| Developer IDE for RDF/XML / OWL/XML light edits | **Pilot** | Semantic re-serialize; core ops — [OWL/XML write-back](owl-xml-workflow.md) |
| Workspace refactoring (rename, migrate, move, extract, merge, replace) | **Pilot** | Rename/merge/replace multi-format; move/extract Turtle-first; preview before apply — [Refactoring guide](refactoring.md) |
| Semantic diff in PR review | **Pilot** | `strixonomy diff` + VS Code panel — [Semantic diff](../ide/semantic-diff.md) |
| Ontology browse/query in VS Code | **Pilot** | SQL / SPARQL + Workbench **DL** mode — [DL Query honesty](dl-query.md) |
| Air-gapped VS Code install | **Pilot** | VSIX + SHA256 — [enterprise deployment](enterprise-deployment.md) |
| OBO index + write-back + ROBOT CLI in CI | **Pilot** | Index and edit `.obo`; `strixonomy robot validate` — requires Java + `robot` on PATH — [ROBOT interop](robot-interop.md) |
| Replace Protégé for full OWL 2 DL engineering | **Not supported** | Classification, realize, SWRL, and DL Query Workbench ship; full axiom catalog + Protégé tab parity remain pre-1.0; XML not byte-identical — [Protégé coexistence](protege-coexistence.md) |
| Org-wide mandatory IDE standard | **Defer** | Complete pilot + legal review first |

## Pilot criteria (recommended before wider rollout)

Complete these on a **representative** ontology project (not only tutorial fixtures):

1. **Functional fit** — Compare [What ships today](../SHIPPED.md) and [known limitations](../known-limitations.md) (decision frame: [Protégé vs Strixonomy](protege-decision.md)) against required axiom types and reasoning profile.
2. **Sizing** — Confirm workspace within [limits](../workspace-limits.md); run [production evidence protocol](production-evidence.md) on your corpus — [performance and sizing](performance-sizing.md).
3. **Security** — Platform review of [security policy](../security.md) and [enterprise deployment](enterprise-deployment.md) (LSP stdio, Restricted Mode, path jail).
4. **Legal** — Review LGPL (`horned-owl`) and third-party notices — [LGPL compliance](lgpl-compliance.md).
5. **CI proof** — `validate` and optional `classify` / `realize` in a test pipeline on real branches.
6. **Coexistence** — If migrating from Protégé, follow [Protégé coexistence](protege-coexistence.md) split workflow for 1–2 release cycles.
7. **Exit criteria** — Define what would trigger rollback (e.g. unsatisfiable-class false positives, scale failures, DL axiom editing gaps, SWRL materialize surprises).

Suggested pilot duration: **4–8 weeks** with 3–10 engineers on one ontology repository. After week 1, continue with the [week-2 playbook](enterprise-week-2.md).

## What is stable enough for automation

| Surface | Stability (v0.26 tagged) |
|---------|--------------------------|
| `strixonomy validate` exit codes | Documented for CI |
| `strixonomy classify` exit codes | Documented for CI |
| `strixonomy realize` / `check-instance` exit codes | Documented for CI (pilot corpora) |
| `strixonomy diff` output | Documented for CI; pre-1.0 field names may evolve |
| `strixonomy::Workspace` API | Stable since v0.10; other crates pre-1.0 |
| SQL virtual table column names | May change pre-1.0 |
| LSP `strixonomy/*` JSON | May change pre-1.0 |
| Rust `strixonomy-*` crate APIs | May change pre-1.0 |

Pin CLI version in CI: release binary with `VERSION=0.26.2` or `cargo install ontocore-cli --locked --version 0.26.2`.

## Support and incident response

| Topic | v0.26 policy |
|-------|-------------|
| Commercial support | **Not offered** — community / GitHub issues |
| Security reports | [GitHub Security Advisories](https://github.com/eddiethedean/strixonomy/security/advisories/new) — not public issues |
| Acknowledgment target | Within a few business days ([SECURITY.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/SECURITY.md)) |
| Patch SLA | **No committed SLA** — track [GitHub Security Advisories](https://github.com/eddiethedean/strixonomy/security/advisories) for your version |
| Supported versions | 0.26.x latest tagged; prior minors per [security policy](../security.md) |

Enterprises requiring contractual SLAs should treat Strixonomy as **internal OSS adoption** with your own escalation path to maintainers via GitHub.

## Compliance questionnaires (honest answers)

| Question | Answer from documentation |
|----------|---------------------------|
| Is data sent to the vendor cloud? | **No** by default — local-first ([security](../security.md)) |
| SOC 2 / ISO 27001 certified? | **No** — not claimed |
| HIPAA BAA available? | **No** |
| Telemetry? | **None** shipped |
| Code-signed binaries? | **Not yet** — SHA256 only ([release integrity](../release-integrity.md)) |

### What we will not claim (and signing timeline)

| Claim | Status |
|-------|--------|
| Commercial SLA / paid support | **Not offered** — no planned window until after 1.0 productization |
| SOC 2 / ISO 27001 | **Not claimed** — no certification program in flight |
| HIPAA BAA | **Not offered** |
| Vendor-hosted SaaS telemetry | **None shipped**; local-first by default |
| Code-signed VSIX / CLI | **Not yet** — releases publish `SHA256SUMS` only. Signing is a **post-1.0** hardening candidate, not a near-term commitment |

Use [Procurement appendix](procurement-appendix.md) for questionnaires.

## Reference architecture (pilot)

```text
Developers (VS Code + Strixonomy VSIX)
        │
        ▼
  strixonomy-lsp (stdio, local)
        │
        ▼
  Ontology workspace (.ttl / .obo / .owl / .rdf / .owx editable; JSON-LD / NT / TriG read-only)
        │
        ▼
  CI pipeline (strixonomy validate / classify / realize / dl-query / robot)
        │
        ▼
  Optional: Protégé for byte-identical XML, uncovered axiom types, and Protégé-only plugins
```

## Related

- [Enterprise evaluation](enterprise-eval.md)
- [Enterprise week-2 playbook](enterprise-week-2.md)
- [Production evidence protocol](production-evidence.md)
- [Enterprise deployment](enterprise-deployment.md)
- [Performance and sizing](performance-sizing.md)
- [LGPL compliance](lgpl-compliance.md)
