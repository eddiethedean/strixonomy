# Evaluating Strixonomy for your organization

This page helps security, platform, and ontology teams decide whether Strixonomy **v0.26.2** (latest tagged) fits your workflow. It is honest about **what ships today** vs the v1.0 Protégé-competitive target.

Canonical capability matrix: [What ships today](../SHIPPED.md).

## When not to use Strixonomy (today)

Prefer Protégé or other tools (or wait for product **1.0**) if you need any of the following. Full list: [Known limitations](../known-limitations.md).

- Full **SQL** analytics (JOINs, aggregates) — only SQL-like virtual tables ship today
- **Byte-identical** OWL/XML or RDF/XML that matches Protégé layout
- **JSON-LD / TriG / N-Triples write-back** (read-only today)
- A **curated plugin marketplace** or **production owlmake** without accepting the subprocess SDK — Plugin **SDK 1.0** freezes the wire contract today; marketplace/owlmake hardening remain product **1.0** — [Plugin policy](plugin-policy.md)
- **HermiT-identical** DL explanations or certified Protégé+HermiT equivalence
- **WebProtégé** collaboration
- Move / extract / ontology-merge refactor on **non-Turtle** files (rename/merge/replace already multi-format)

## Enterprise documentation pack

**Start here:** [Procurement and enterprise appendix](procurement-appendix.md) — single index for security, legal, deployment, and governance questionnaires.

| Document | Audience |
|----------|----------|
| [Production readiness](production-readiness.md) | Engineering leadership — pilot vs production criteria |
| [Protégé vs Strixonomy](protege-decision.md) | Ontology teams — when to adopt, keep Protégé, or split |
| [Production evidence protocol](production-evidence.md) | DevOps / QA — self-benchmark on your corpus |
| [Enterprise deployment](enterprise-deployment.md) | Platform / IT — VSIX mirror, CI, air-gap |
| [Platform compatibility](platform-compatibility.md) | Platform — VS Code versions, OS/arch, remote dev |
| [Performance and sizing](performance-sizing.md) | DevOps — limits, pilot benchmarks |
| [Governance](governance.md) | Leadership — sustainability, releases, security policy |
| [Release timeline (non-commitment)](release-timeline.md) | Planning — v0.9/v1.0 goals without fixed dates |
| [LGPL compliance](lgpl-compliance.md) | Legal — horned-owl obligations |
| [Protégé coexistence](protege-coexistence.md) | Ontology teams — split workflow with Protégé |
| [Plugin authoring](plugins.md) | Platform — Plugin SDK 1.0 (frozen wire, lifecycle, providers); see [Plugin policy](plugin-policy.md) |

## What ships today (v0.26.2)

Use the canonical matrix: **[What ships today](../SHIPPED.md)**. Do not maintain a second capability table here — it drifts.

Highlights for evaluators: Turtle/OBO/RDF/XML/OWL/XML write-back; Query Workbench (SQL subset + SPARQL + DL); EL–DL reasoner + realize; multi-format rename/merge/replace; Plugin SDK 1.0 wire; semantic diff. Gaps: [Known limitations](../known-limitations.md) · [Protégé decision](protege-decision.md) · [DL Query honesty](dl-query.md).

**Out of scope for questionnaires:** Strixonomy is a **local-first** IDE/CLI — there is **no** product IdP/SSO, RBAC, or vendor SLA. Commercial support is community OSS only. See [Production readiness](production-readiness.md) and [Governance](governance.md).

## Deployment model

- **Local-first:** Strixonomy indexes files on disk. Ontology content is **not uploaded** to a cloud service by default.
- **Language server:** `strixonomy-lsp` runs as a child process of VS Code over stdio. **Do not expose it to the network** without authentication — see [security policy](../security.md).
- **Offline install:** VSIX from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) + `SHA256SUMS` verification ([release-integrity.md](../release-integrity.md)).
- **CI-only usage:** Teams can run `strixonomy validate` and `strixonomy classify` in pipelines without installing the VS Code extension ([ci-integration.md](../ci-integration.md)).

## Security and compliance

| Topic | Guidance |
|-------|----------|
| Threat model | [SECURITY.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/SECURITY.md) — path jail, resource limits, Restricted Mode |
| Vulnerability reporting | GitHub Security Advisories (not public issues) |
| `strixonomy.lspPath` | Trusted workspaces only; ignored in VS Code Restricted Mode |
| Resource limits | [workspace-limits.md](../workspace-limits.md) — file count, size, triple caps |
| Telemetry | **No telemetry**. AI features are opt-in per [ADR-0010](../design/adr/0010-ai-features-opt-in.md) (not shipped) |
| Supply chain | SHA256 checksums on release artifacts; `cargo audit` in CI. **Code signing not shipped.** Open VSX publishes automatically when `OVSX_PAT` is set; **VS Code Marketplace publish is always manual**. Either store may lag the GitHub tag ([releasing](../releasing.md)). |

## Licensing

- Strixonomy/Strixonomy crates: **MIT OR Apache-2.0**
- **LGPL:** [`horned-owl`](https://crates.io/crates/horned-owl) is used for OWL modeling and Turtle write-back — review LGPL obligations ([LGPL compliance](lgpl-compliance.md), [LICENSES.md](../design/LICENSES.md), [FAQ](../faq.md))
- **NOTICES file:** Regenerate before releases per [LICENSES.md](../design/LICENSES.md); verify your release process includes third-party attribution

## Known limitations for enterprise layouts

| Limitation | Impact |
|------------|--------|
| **Multi-root VS Code workspaces** | **All folders indexed** (v0.10+) |
| **Write-back** | **Turtle, OBO, RDF/XML, OWL/XML**; JSON-LD / TriG / N-Triples read-only. XML is semantic re-serialize (not byte-identical) |
| **Reasoning** | EL/RL/RDFS/DL/auto via OntoLogos 1.x; explanations DL-first on DL (EL/RL/RDFS alternatives); results may differ from Protégé on partial OWL mappings |
| **CLI release binaries** | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| **Scale** | Workspaces above [workspace limits](../workspace-limits.md) may fail indexing — prefer CLI batch workflows for very large terminologies |
| **ROBOT / Java** | `strixonomy robot` and LSP `runRobot` spawn an external Java `robot` process — not JVM-free for that workflow |

## Protégé coexistence

A [first-week Protégé migration guide](protege-migration.md) ships today. Round-trip workflows (Protégé export → Strixonomy edit → Protégé verify) and byte-identical XML playbooks are **v1.0 targets**. Today:

- [Protégé coexistence guide](protege-coexistence.md) — split workflow when keeping Protégé for specific features
- [OWL/XML and RDF/XML write-back](owl-xml-workflow.md) — semantic re-serialize caveats

- Use Strixonomy for **Turtle / OBO / RDF/XML / OWL/XML editing in VS Code** (XML with caveats), **CI validation**, **SQL/SPARQL + DL Query Workbench**, **Manchester axioms** (richest on Turtle), **workspace refactoring** (rename/merge/replace multi-format; move/extract Turtle-first), **property chain editing**, and **EL/RL/RDFS/DL classification**
- Keep Protégé for **byte-identical XML layout**, **Protégé-specific plugins**, and axiom types still listed under [known limitations](../known-limitations.md) until v1.0
- See [Protégé vs Strixonomy](protege-decision.md) and [What ships today](../SHIPPED.md)

## Evaluation checklist

1. Install from [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy), [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (Cursor), or offline VSIX
2. Complete [First success in 10 minutes](first-success.md) on a representative `.ttl` project
3. Run the [production evidence protocol](production-evidence.md) on your corpus
4. Run `strixonomy validate` and optionally `strixonomy classify --profile el` or `--profile dl` in a test CI job ([ci-integration.md](../ci-integration.md))
5. Optional: `strixonomy realize` on a representative ABox corpus and a SWRL dual-check ([realize](../examples/realize.md) · [swrl](../examples/swrl.md) · [week-2](enterprise-week-2.md))
6. Trial Query Workbench **DL** mode / `strixonomy dl-query` against your Manchester expressions — [dl-query.md](dl-query.md)
7. Review [Protégé decision matrix](protege-decision.md) and [platform compatibility](platform-compatibility.md)
8. Review [security policy](../security.md) and [governance](governance.md) with your platform team
9. Compare [What ships today](../SHIPPED.md) and [known limitations](../known-limitations.md) against your requirements; read [release timeline](release-timeline.md) for planning (no fixed v1.0 date)

## Questions

[FAQ](../faq.md) · [Troubleshooting](../troubleshooting.md) · [errors reference](../errors.md) · [Report an issue](https://github.com/eddiethedean/strixonomy/issues)
