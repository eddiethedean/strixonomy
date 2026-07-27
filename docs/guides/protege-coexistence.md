# Protégé coexistence

Guide for teams using Protégé today and evaluating Strixonomy **v0.26**. A [first-week migration guide](protege-migration.md) ships today. RDF/XML and OWL/XML write-back **shipped in v0.21** (semantic re-serialize, not byte-identical); v0.22 completes OWL 2 authoring depth; **v0.23** adds realization, instance checking, and SWRL; **v0.24** adds DL Query — see [OWL/XML write-back](owl-xml-workflow.md), [DL Query](dl-query.md), [What ships today](../SHIPPED.md), and [roadmap](../roadmap.md).

Canonical capability matrix: [What ships today](../SHIPPED.md). Decision matrix: [Protégé vs Strixonomy](protege-decision.md). Gap analysis for adopters: [Known limitations](../known-limitations.md).

## Use Strixonomy for (v0.26)

| Workflow | Status |
|----------|--------|
| Browse ontologies in VS Code | Shipped |
| Edit labels, comments, parents in Turtle | Shipped |
| Edit OBO terms (name, synonyms, defs, is_a, …) | Shipped (engine v0.12; inspector v0.13) |
| Edit RDF/XML / OWL/XML (core inspector + patch ops) | Shipped (v0.21; semantic re-serialize) |
| Complex `SubClassOf` / `EquivalentClasses` / disjoint (Manchester) | Shipped (richest on Turtle) |
| Property chain editing | Shipped (v0.12) |
| Workspace refactoring (rename, migrate namespace, move, extract) | Shipped (Turtle; preview + apply) |
| SQL/SPARQL queries over workspace | Shipped |
| DL Query (Workbench DL mode, CLI, LSP) | Shipped (v0.24+) — [honesty notes](dl-query.md) |
| Graph visualization (class, property, import, neighborhood, and v0.26 kinds) | Shipped |
| CI lint (`strixonomy validate`) | Shipped — suitable for production CI |
| EL/RL/RDFS/DL classification | Shipped |
| OWL 2 DL classification (`dl` / `auto` profiles) | Shipped (OntoLogos 1.x; not certified HermiT-identical) |
| Realization / instance checking (ABox) | Shipped (v0.23) |
| SWRL rule browser / editor / validate | Shipped (v0.23; DLSafe) |
| Inferred hierarchy toggle | Shipped (after reasoner run) |
| OBO format index + `obo_id` in explorer | Shipped |
| ROBOT CLI in CI (`strixonomy robot`) | Shipped (requires Java + `robot` on PATH) |
| Multi-root VS Code workspaces | Shipped (all folders indexed) |
| Semantic diff (CLI / LSP / panel) | Shipped |
| Plugin SDK 1.0 (manifest, lifecycle, providers) | Shipped (v0.26) |

## Keep Protégé for (today)

| Workflow | Why |
|----------|-----|
| HermiT-identical DL explanations / certified HermiT results | OntoLogos DL is not certified HermiT-identical |
| Exact Protégé DL Query tab / Manchester fidelity expectations | Strixonomy ships DL Query (v0.24+) with [documented honesty limits](dl-query.md) — dual-check critical queries if you require Protégé+HermiT identity |
| Full OWL 2 DL axiom catalog for all formats | Partial Manchester + patches — see [SHIPPED](../SHIPPED.md) and [known limitations](../known-limitations.md) |
| Byte-identical OWL/XML or RDF/XML layout | Strixonomy re-serializes for semantic fidelity ([ADR-0021](../design/adr/0021-deterministic-xml-serializers.md)) |
| Workflows that depend on Protégé-specific plugins | Not replicated; Strixonomy uses subprocess SDK 1.0 — not a Protégé plugin host |

## Practical split workflow

1. **Author** routine Turtle, OBO, and light XML changes in VS Code (inspector, Manchester editor, refactoring, patches)
2. **Validate** in CI with `strixonomy validate` and optionally `strixonomy classify --profile el`
3. **Run ROBOT** in CI when needed — [ROBOT interop](robot-interop.md)
4. **Keep Protégé** for Protégé-only plugins, byte-identical XML, or uncovered axiom types
5. Prefer Turtle when you need byte-stable diffs or refactor apply

## Round-trip tips

- After Protégé saves XML, re-index in Strixonomy; expect layout differences, not semantic loss for edited entities/labels/parents/imports.
- Prefer Turtle for shared team authoring when Git diffs must stay readable.
- See [OWL/XML write-back](owl-xml-workflow.md) for supported XML patch ops.

## Related

- [Protégé migration (first week)](protege-migration.md)
- [Production readiness](production-readiness.md)
- [What ships today](../SHIPPED.md)
