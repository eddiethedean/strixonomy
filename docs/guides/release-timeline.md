# Release timeline (non-commitment)

Planning ranges for Strixonomy IDE / Strixonomy engine. **These are product goals, not contractual delivery dates.** Shipped behavior is always defined by [What ships today](../SHIPPED.md) for the version you deploy.

## Current release

| Version | Status | Date (changelog) |
|---------|--------|------------------|
| **0.28.1** | Current (tagged) | 2026-08-06 |
| **0.27.0** | Previous | 2026-07-27 |
| **0.26.2** | Previous | 2026-07-17 |
| **0.26.1** | Previous | 2026-07-16 |
| **0.26.0** | Previous | 2026-07-16 |
| **0.25.0** | Previous | 2026-07-15 |
| **0.24.0** | Previous | 2026-07-14 |
| **0.23.0** | Previous | 2026-07-14 |
| **0.22.0** | Previous | 2026-07-14 |
| **0.21.0** | Previous | 2026-07-13 |
| **0.20.0** | Previous | 2026-07-13 |
| **0.19.0** | Previous | 2026-07-13 |
| **0.18.2** | Previous | 2026-07-13 |
| **0.18.1** | Previous | 2026-07-12 |
| **0.18.0** | Previous | 2026-07-11 |
| **0.15.0** | Previous | 2026-07-08 |
| **0.14.0** | Previous | 2026-07-09 |
| **0.13.0** | Previous | 2026-07-08 |
| **0.12.0** | Previous | 2026-07-06 |
| **0.11.3** | Previous | 2026-07-06 |
| **0.11.2** | Previous | 2026-07-06 |
| **0.11.1** | Previous | 2026-07-06 |
| **0.11.0** | Previous | 2026-07-05 |

The v0.x line may change library APIs, LSP JSON, and SQL virtual table columns between minor releases; pin deployed versions — [README](https://github.com/eddiethedean/strixonomy/blob/main/README.md).

## Documented milestone goals (not dates)

| Target | Goal | Shipped in |
|--------|------|------------|
| **v0.9** | Strixonomy identity — `strixonomy` façade, branding, documentation; OntoLogos 1.0 DL/auto classification | **Shipped** (2026-07-03) |
| **v0.10** | Semantic workspace — incremental index, multi-root, stable `Workspace` API, semantic diff, optional disk cache | **Shipped** (2026-07-04) |
| **v0.11** | Editor depth & distribution — LSP completion, code actions, docs export, imports UI, Open VSX, OBO fastobo read | **Shipped** (2026-07-05) |
| **v0.12** | Authoring parity — OBO write-back, property chains, OWL/XML read, DL explanations | **Shipped** (2026-07-06) |
| **v0.13** | OntoUI platform — WorkspaceStore, focus relay, schema browser, PR summary, semantic tokens | **Shipped** (2026-07-08) |
| **v0.14** | Plugin host MVP — manifests, reference plugins, CLI/LSP hooks, owlmake scaffold | **Shipped** (2026-07-09) |
| **v0.19** | Semantic foundation — `strixonomy-edit` transactions; parity manifest + CI; epics EPIC-001…011 | **Shipped** (2026-07-13) |
| **v0.20** | Workspace runtime — multi-ontology registry, dirty/save, session persistence | **Shipped** (2026-07-13) |
| **v0.21** | RDF/XML + OWL/XML write-back (semantic re-serialize) | **Shipped** (2026-07-13) |
| **v0.22** | Complete OWL 2 authoring (HasKey, DisjointUnion, RBox/ABox, Manchester depth) | **Shipped** (2026-07-14) |
| **v0.23** | Reasoning parity + SWRL (realize, instance check, Rule Browser/Editor) | **Shipped** (2026-07-14) |
| **v0.24** | Refactoring + DL Query parity | **Shipped** (2026-07-14) |
| **v0.25** | Viz + plugin SDK 1.0 + a11y + parity CI | **Shipped** (2026-07-15) |
| **v0.26** | Protégé Desktop JUnit behavioral test port (Waves 1–4) | **Shipped** (2026-07-16) |
| **v0.29** | Trustworthy projects: conformance, recovery, cross-platform confidence, performance budgets | Planned |
| **v0.30** | Fast daily authoring: cohesive editing, reasoning, undo, and migration flow | Planned |
| **v0.31** | Large ontology productivity: scale, deeper query, multi-format operations | Planned |
| **v0.32** | Team review and enforceable semantic policy in CI | Planned |
| **v0.33** | Automated ontology delivery and production workflow plugins | Planned |
| **v0.34** | Python/TypeScript SDKs and MCP integration platform | Planned |
| **v0.35** | Explainable, preview-first assisted modeling | Planned |
| **v0.36** | Install-free browser workspace and WASM engine | Planned |
| **v0.37** | Governed collaboration and semantic conflict resolution | Planned |
| **v0.38+** | Enterprise operations and portfolio scale | Planned |

Canonical forward plan: [Platform roadmap](../roadmap.md). Engineering milestone history: [Milestones (shipped)](../design/ROADMAP.md).

**There are no documented calendar dates for planned phases.** Enterprise plans should not assume a quarter or year without maintainer confirmation outside these docs.

## What each near-term milestone implies

### v0.10 (shipped)

- Semantic diff for PR review workflows (CLI, LSP, VS Code panel)
- Incremental indexing, multi-root workspaces, optional `.strixonomy/cache`
- Stable `strixonomy::Workspace` API
- Does **not** by itself complete Protégé parity or full OBO write-back

### v0.29–v0.32 (planned adoption sequence)

- **v0.29:** prove trust with conformance evidence, safe recovery, cross-platform distribution, and performance budgets
- **v0.30:** complete the daily authoring loop across editing, reasoning, undo, navigation, and migration
- **v0.31:** remove large-project and multi-format productivity barriers
- **v0.32:** make semantic review and policy enforceable in pull requests

## How to plan enterprise adoption without roadmap dates

1. **Now (v0.26):** CI gates + controlled IDE pilot — [production readiness](production-readiness.md)
2. **Run** [production evidence protocol](production-evidence.md) on your corpus
3. **Re-evaluate** at each pinned minor bump using [migration index](../migration/README.md)
4. **Do not** retire Protégé for DL/OBO workflows until items you need are green in [SHIPPED](../SHIPPED.md) and acceptable under [known limitations](../known-limitations.md)

## Design docs vs shipped docs

| Document type | Trust for procurement |
|---------------|----------------------|
| [SHIPPED.md](../SHIPPED.md) | **Canonical** for deployed version |
| User guides under VS Code / Rust tabs | **Implemented** behavior for current release |
| Contributing → Design (specs, ADRs, ROADMAP) | **Target / vision** — may not be implemented |

## Related

- [Governance](governance.md)
- [Roadmap (engineering detail)](../design/ROADMAP.md)
- [Enterprise evaluation](enterprise-eval.md)
- [Changelog](../changelog.md)
