# Migrating from Protégé — first week

This guide helps ontology teams adopt Strixonomy **v0.26** alongside or instead of [Protégé](https://protege.stanford.edu/). For a capability comparison, see [Protégé vs Strixonomy](protege-decision.md) and [What ships today](../SHIPPED.md).

## Before you start

**Strixonomy fits well when you:**

- Version-control ontology files (Turtle, OWL, OBO) in shared repositories
- Want VS Code editing, CI validation, and semantic diff
- Can edit **Turtle (`.ttl`)**, **OBO (`.obo`)**, **RDF/XML (`.owl`/`.rdf`)**, or **OWL/XML (`.owx`)** for write-back (XML is semantic re-serialize — [OWL/XML write-back](owl-xml-workflow.md))
- Need Protégé-like menus, reasoner workflows, explanations, graphs, and imports in the IDE

**Keep Protégé (for now) when you need:**

- **Byte-identical OWL/XML or RDF/XML layout** after save
- A **full DL axiom catalog UI** for every axiom kind and format
- Protégé-specific plugins or a **curated plugin marketplace** (Strixonomy Plugin SDK 1.0 freezes the subprocess wire today; marketplace remains product **1.0** — [Plugin policy](plugin-policy.md))
- WebProtégé-style live collaboration
- **HermiT-identical** DL explanations (OntoLogos is not certified HermiT-identical)

Many teams use **both**: Protégé for heavy axiom authoring or Protégé-only plugins, Strixonomy for browse, light edit, lint, diff, reasoning, DL Query, and CI. See [Protégé coexistence](protege-coexistence.md).

## Honest desktop known gaps (v0.26 tagged)

See [Versions & channels](versions-and-channels.md) if Marketplace lags behind the GitHub Release VSIX.

| Gap | Status |
|-----|--------|
| Byte-identical OWL/XML · RDF/XML layout | Re-serialize only (write-back shipped v0.21) — [owl-xml-workflow](owl-xml-workflow.md) |
| Multi-step semantic undo | Partial workspace runtime (v0.20); full history → **v1.0** |
| Full OntoGraf filter/layout suite | Partial graphs shipped (v0.26 parity expanded); polish → **v1.0** |
| Explain all inference kinds (not only unsat) | Unsat explanations shipped |
| Mid-classify thread kill on the Rust reasoner | Client cancel + ignore late results (v0.18); server may finish CPU work |
| Curated plugin marketplace / production owlmake | **Product 1.0** (SDK 1.0 wire is frozen today) |

Full matrix: [known-limitations](../known-limitations.md) · [SHIPPED](../SHIPPED.md).

## Day 1 — Install and open your project

1. Install [Strixonomy from the Marketplace](../vscode-install.md) or [Open VSX](https://open-vsx.org/extension/ontocode/ontocode).
2. **File → Open Folder…** and select your ontology repository (not a single file).
3. Strixonomy’s **bundled** language server works in Restricted Mode — **Trust** only if you set custom `strixonomy.lspPath` or `strixonomy.robotPath`.
4. Open the **Strixonomy** activity bar and confirm **Ontologies** lists your `.ttl` / `.owl` files.
5. Expand **Classes** and click an entity to open the **Entity Inspector**.

Follow the [first success core path](../guides/first-success.md) if anything is empty after trust + index.

## Day 2 — Map Protégé habits to Strixonomy

| In Protégé | In Strixonomy v0.26 |
|------------|-------------------|
| Class hierarchy tab | **Classes** explorer; toggle **asserted / inferred / combined** after reasoner |
| Entity editor (labels, parents) | **Entity Inspector** (`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`) |
| Manchester syntax | **Manchester editor** panel |
| DL query tab | **Query Workbench → DL mode** (or CLI `strixonomy dl-query`) — [honesty notes](dl-query.md); not full Protégé tab / HermiT identity |
| Reasoner (HermiT, etc.) | **Start / Synchronize / Classify / Consistency / Realize** — EL/RL/RDFS/DL/auto via OntoLogos (not certified HermiT-identical); **Stop** requests engine cancel |
| Explanations | **Explanation** panel (unsat; DL-native steps where available; stale banner after reindex) |
| SWRLTab | **Rule Browser** / **Rule Editor** (author + validate; DLSafe materialize on classify) |
| OWLViz / OntoGraf | **Class / property / import / neighborhood** graphs (asserted/inferred/combined; v0.26 kinds expanded) |
| Imports | **Manage Imports** panel |
| Preferences | Strixonomy settings + plugin preference pages |
| Active ontology | **Active Ontology** selector (multi-root supported) |
| Refactor / move axioms | **Rename**, **Migrate Namespace**, **Move**, **Extract Module** (CLI); **Merge**, **Replace** (IDE only) |
| Diff between versions | **Semantic Diff** panel or `strixonomy diff` in CI |
| Layout / perspectives | Named **Modeling / Reasoning / Review** perspectives; restored tabs offer **Reopen panel** |

## Day 3 — Validate in CI

```yaml
- run: cargo install ontocore-cli --locked --version 0.26.2
- run: strixonomy validate ./src/ontologies
```

Optional: fail on unsatisfiable classes, or assert an instance:

```yaml
- run: strixonomy classify . --profile el --format json
- run: strixonomy realize . --profile rl --format json
```

Full examples: [CI integration](../ci-integration.md) · [Realize cookbook](../examples/realize.md).

## Day 4 — Queries, imports, diagnostics

1. Open **Query Workbench** → run `SELECT short_name, labels FROM classes`.
2. Use **Manage Imports** to add/remove Turtle imports; **Reload Imports** after external changes.
3. Inspect lint issues in **Diagnostics** and the **Problems** panel.

SQL subset limits: [SQL reference](../sql-reference.md). SPARQL: [SPARQL reference](../sparql-reference.md).

## Day 5 — Reasoning, explanations, visualization

1. **Synchronize Reasoner** (reindex + classify) or **Classify Ontology**.
2. Review unsatisfiable classes; open **Explanation** and regenerate if the stale banner appears after edits.
3. Run **Realize** (or inspect realization in the Reasoner panel) for ABox inferred types.
4. Open **Rule Browser** if your ontology uses SWRL; edit or validate rules in **Rule Editor**.
5. Open **Class Graph** in asserted vs inferred mode; truncated graphs show a warning on large ontologies.
6. Set **Hierarchy Mode** to **inferred** or **combined**.

Guide: [Reasoner](reasoner.md) · [SWRL cookbook](../examples/swrl.md).

## Week 1 checkpoint

- [ ] Browse and edit Turtle/OBO/XML entities in VS Code
- [ ] Run reasoner lifecycle (classify / consistency / realize / stop) and open an explanation
- [ ] Optionally open Rule Browser for SWRL (if your ontology uses rules)
- [ ] Run `strixonomy validate` (or classify / realize) in CI
- [ ] Compare a branch with **Semantic Diff** or `strixonomy diff`
- [ ] Document which tasks stay in Protégé vs Strixonomy for your team

## Common friction points

| Issue | Resolution |
|-------|------------|
| Cannot edit OWL/XML / RDF/XML in inspector | Ensure file is `.owl`/`.rdf`/`.owx` with OK parse status (v0.21+); check [Supported formats](../supported-formats.md) and [OWL/XML write-back](owl-xml-workflow.md). Prefer Turtle for full Manchester / refactor |
| SQL query fails | Strixonomy SQL is single-table subset — use SPARQL for graph patterns |
| Reasoner slow or fails on DL | Check [workspace limits](../workspace-limits.md); try `el` profile first |
| Restored panel looks empty | Click **Reopen panel** on the recovery tab (context is reloaded from the last command) |
| Team expects stable plugin ecosystem API | Plugin host MVP shipped; stable semver API is **v1.0** — see [Plugin authoring](plugins.md) |

## Next steps

| Goal | Document |
|------|----------|
| What changed in v0.18 | [migration/v0.18.md](../migration/v0.18.md) |
| Enterprise evaluation pack | [Enterprise evaluation](enterprise-eval.md) |
| Split workflow with Protégé | [Protégé coexistence](protege-coexistence.md) |
| Semantic diff for releases | [Semantic diff](../ide/semantic-diff.md) |
| OBO / ROBOT pipelines | [OBO workflow](obo-workflow.md) · [ROBOT interop](robot-interop.md) |
