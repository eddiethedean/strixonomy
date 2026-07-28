# Protégé vs Strixonomy decision matrix

Use this page to decide **when Strixonomy fits**, **when to keep Protégé**, and **when to run both**. It reflects **v0.27.0** (latest tagged) — see [What ships today](../SHIPPED.md). A [first-week migration guide](protege-migration.md) ships today; extended round-trip playbooks are planned for **v1.0**.

!!! note "Non-goals today (v0.27.0)"
    - **WebProtégé-style collaboration** — out of scope until post-1.0
    - **Byte-identical Protégé OWL/XML / RDF/XML layout** — Strixonomy re-serializes for semantic fidelity ([ADR-0021](../design/adr/0021-deterministic-xml-serializers.md)); write-back itself **ships** in v0.21
    - **Curated plugin marketplace / production owlmake** — Plugin SDK 1.0 freezes the wire contract today; marketplace hardening is product **1.0** — [Plugin policy](plugin-policy.md)
    - **Language SDKs** (Python/TypeScript ontology clients) — embed via Rust `strixonomy`, CLI, or LSP instead
    - **Certified HermiT-identical results** — OntoLogos DL is not certified HermiT-identical; dual-tool check critical audits

## Quick decision

| Your situation | Recommendation |
|----------------|----------------|
| VS Code + Turtle-primary ontologies | **Adopt Strixonomy** (pilot IDE + CI) |
| CI lint/consistency gates without desktop Protégé | **Adopt Strixonomy CLI** (`strixonomy validate` / `classify` / `realize` / `dl-query`) |
| Protégé `.owl` / `.owx` corpora that need light inspector edits | **Adopt Strixonomy** for core write-back; keep Protégé when you need byte-identical XML layout, full axiom catalog, or Protégé-only plugins — [OWL/XML write-back](owl-xml-workflow.md) |
| Full OWL 2 DL axiom catalog on every format | **Split workflow** — Strixonomy for browse/query/CI/classification + supported authoring; keep Protégé for uncovered axiom types until [v1.0](../roadmap.md) |
| OBO release pipelines with in-editor OBO write-back | **Adopt Strixonomy** (inspector since v0.13; patch engine since v0.12) for OBO inspector + `strixonomy patch`; validate with ROBOT in CI |
| SWRL rule editing in IDE | **Adopt Strixonomy** (Rule Browser/Editor since v0.23); keep Protégé if you need full SWTab ecosystem plugins |
| Manchester class-expression queries (DL Query–style) | **Adopt Strixonomy** Query Workbench **DL** mode / `strixonomy dl-query` — [DL Query honesty](dl-query.md) (not full Protégé DL Query tab parity) |
| Enterprise requires vendor SLA / SOC 2 | **Defer** or run limited CI pilot — [Production readiness](production-readiness.md) |
| Air-gapped VS Code + internal artifact mirror | **Pilot** — [Enterprise deployment](enterprise-deployment.md) |

## Capability comparison (v0.27.0 tagged)

| Capability | Protégé | Strixonomy v0.27 | Notes |
|------------|---------|----------------|-------|
| OWL 2 DL classification | Yes | Yes (`dl` / `auto` via OntoLogos 1.x) | Not certified HermiT-identical — [Reasoner guide](reasoner.md) |
| Realization / instance checking | Yes | Yes | CLI `realize` / `check-instance`; Reasoner panel (shipped v0.23) |
| SWRL authoring | SWRLTab | Rule Browser / Editor | DLSafe; validate via LSP (shipped v0.23) |
| Turtle authoring | Manual / plugins | Native write-back | Strixonomy inspector + patches |
| OBO authoring | Native | Native write-back | Patch engine since v0.12; Entity Inspector since v0.13 |
| RDF/XML in-place editing | Yes | Yes (semantic re-serialize) | Not byte-identical to Protégé — [OWL/XML write-back](owl-xml-workflow.md) |
| OWL/XML in-place editing | Yes | Yes (semantic re-serialize) | Core ops; Manchester/refactor limited |
| Manchester axiom editing | Full | OWL 2 Manchester on Turtle (v0.22) | Richest on Turtle; XML core ops — [SHIPPED Manchester scope](../SHIPPED.md#manchester-scope-v022) |
| Disjoint classes | Yes | Yes (v0.9) | Via Manchester / patch JSON |
| Property chain editing | Yes | Yes (v0.12) | Inspector + patch JSON |
| OBO format | Native | Index + write-back | Patch engine since **v0.12**; Entity Inspector since **v0.13** |
| ROBOT integration | Common | CLI wrapper | Java + `robot` required |
| SQL/SPARQL over repo | Plugins / external | Built-in workbench + CLI | |
| DL Query tab | Native | Query Workbench **DL** mode + CLI `dl-query` + LSP | Manchester class expressions; see [DL Query honesty](dl-query.md) |
| Semantic diff in pull requests | Weak default | Strong default | Shipped v0.10 |
| Workspace refactoring | Limited | Rename, migrate, move, extract, merge, replace | Rename/merge/replace multi-format; move/extract Turtle-first |
| CI automation | External scripts | `strixonomy validate` / `classify` / `realize` / `dl-query` | Documented exit codes |
| Local-first / no telemetry | Desktop app | Yes | No cloud upload by default |
| Commercial support | Ecosystem / third parties | **None** — community OSS | |

## Adoption paths

### Path A — CI only (lowest risk)

1. Pin `strixonomy-cli` **0.27.0** in Linux CI
2. Gate merges with `strixonomy validate`
3. Optional: `strixonomy classify --profile el` when ontology is EL; `strixonomy dl-query` for class-expression checks
4. Keep Protégé on engineer desktops unchanged

Docs: [CI integration](../ci-integration.md) · [Production evidence protocol](production-evidence.md)

### Path B — Split workflow (recommended pilot)

1. Author `.ttl` (or editable XML with caveats) in Strixonomy; validate in CI
2. Use Protégé for Protégé-specific plugins, byte-identical XML layout, or axiom types not yet covered in [SHIPPED](../SHIPPED.md) / [known limitations](../known-limitations.md)
3. Standardize on Turtle for shared authoring when you need byte-stable diffs or refactor apply
4. Run 4–8 week pilot — [Production readiness](production-readiness.md)

Docs: [Protégé coexistence](protege-coexistence.md) · [OWL/XML write-back](owl-xml-workflow.md)

### Path C — Protégé replacement (not supported today)

Do **not** plan org-wide Protégé retirement on pre-1.0 alone. Re-evaluate at **v1.0** against [What ships today](../SHIPPED.md) and [known limitations](../known-limitations.md).

## When Strixonomy is a poor fit

- Needs HermiT-/Protégé-grade DL **explanation UX** or a **full axiom editor for every OWL construct** as the sole gate — Strixonomy ships DL classification and DL Query Workbench mode for CI/IDE, but is not a drop-in Protégé replacement for all axiom authoring
- Primary artifacts require **byte-identical Protégé XML** (Strixonomy re-serializes)
- Workspaces exceed [workspace limits](../workspace-limits.md) without split-repo strategy
- Legal cannot accept **LGPL** (`horned-owl`) for desktop distribution — [LGPL compliance](lgpl-compliance.md)
- Procurement requires **SOC 2, HIPAA BAA, or vendor SLA**

## Related

- [What ships today](../SHIPPED.md)
- [Known limitations](../known-limitations.md)
- [Production readiness](production-readiness.md)
- [Enterprise evaluation](enterprise-eval.md)
- [DL Query honesty](dl-query.md)
