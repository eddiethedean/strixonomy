# What ships today (v0.27.0 — latest tagged)

> **Canonical capability matrix.** Update this page on every release. Design specs under [Project](design/README.md) may describe future targets — check here for what is actually available.
>
> **Format write-back truth:** this page and [Supported formats](supported-formats.md) are the source of truth. Tier-1 user docs (README, Home, First success, FAQ, Evaluate pack, LSP/patch/CLI refs) must match them — see [Releasing — Tier-1 capability truth](releasing.md#documentation-sync-checklist-every-release).
>
> **Latest tagged release: v0.27.0** (crates.io, GitHub Releases; Marketplace/Open VSX may lag — see [Versions & channels](guides/versions-and-channels.md)). Pin installs: `cargo install strixonomy-cli --locked --version 0.27.0`.

**Latest tagged: v0.27.0** · [v0.27 migration](migration/v0.27.md) · [v0.26.2 patch](migration/v0.26.2.md) · [CHANGELOG](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md)

## Products

| Product | What it is |
|---------|------------|
| **Strixonomy (IDE)** | VS Code IDE — explorer, React inspector, graphs (asserted/inferred modes), Query Workbench (SQL/SPARQL/DL), Manchester editor, refactor preview, reasoner, explanation panel, plugin commands/views/preferences/context actions |
| **Strixonomy (engine)** | Rust semantic workspace engine — `strixonomy` façade, `strixonomy-*` crates, `strixonomy` CLI, `strixonomy-lsp`, plugin host |

## Capability matrix (v0.27.0 tagged)

| Capability | VS Code | CLI |
|------------|---------|-----|
| Browse classes, properties, individuals | Yes | via SQL |
| Edit labels, comments, parents (`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`) | Yes (React inspector) | `strixonomy patch` |
| Create / delete entities (`.ttl`, XML required formats) | Yes | `strixonomy patch` |
| Complex `SubClassOf` / `EquivalentClasses` (Manchester) | Yes (Turtle) | `strixonomy patch` |
| Disjoint classes (author + view) | Yes (inspector + Manchester) | `strixonomy patch` |
| Domain / range / characteristics / property chains | Yes (inspector + patch; Turtle) | `strixonomy patch` |
| Individual assertions (class/object/data) | Yes (Turtle; class assertion on XML) | `strixonomy patch` |
| Generic annotation assertions | Yes | `strixonomy patch` |
| OBO term edit (name, synonym, def, is_a, …) | Yes (inspector) | `strixonomy patch` |
| Find usages / rename IRI / namespace migration / move / extract module | Yes (preview + apply) | `strixonomy refactor` |
| Merge entities / replace entity references | Yes (preview + apply) | `strixonomy refactor merge` / `replace` |
| New ontology scaffold / export (ROBOT convert or copy) | Yes | `strixonomy new` / export LSP |
| Prefix manager / ontology metadata patches | Yes | `strixonomy patch` |
| Active ontology selector | Yes | LSP `setActiveOntology` |
| Workspace runtime (registry, dirty/save, transactions, session) | Yes | — |
| Menus / toolbars / keybindings / perspectives | Yes | — |
| SQL-like queries | Query Workbench (React) + schema browser | `strixonomy query` |
| SPARQL | Query Workbench (React) | `strixonomy sparql` |
| DL Query (Manchester class expressions) | Query Workbench **DL** mode | `strixonomy dl-query` |
| Graph visualization (class, property hierarchies, individual, import, dependency, neighborhood; query/refactor result graphs) | Yes (React; asserted/inferred/combined; filters; unsatisfiable overlay; Graph\|List; virtualized; export JSON/CSV; expand depth) | LSP `strixonomy/getGraph` |
| OWL EL classification (`el` profile) | Reasoner panel + hierarchy toggle | `strixonomy classify` |
| RL / RDFS classification | Reasoner panel | `strixonomy classify --profile rl\|rdfs` |
| OWL 2 DL classification (`dl` profile) | Reasoner panel + hierarchy toggle | `strixonomy classify --profile dl` |
| Auto profile routing (`auto`) | Reasoner panel | `strixonomy classify --profile auto` |
| Realization / instance checking (ABox) | Reasoner panel realization + LSP `checkInstance` | `strixonomy realize` / `check-instance` |
| Full consistency (TBox + ABox) | Reasoner panel clashes + snapshot detail | via classify / consistency |
| EL / DL explanations (DL-first + alternatives) | Explanation panel (multiple alternatives, staleness detection) | `strixonomy explain` |
| SWRL rule browser / editor / validate | Rule Browser + Rule Editor | patch + LSP SWRL methods |
| Engine-level reasoner cancel | Stop Reasoner | LSP `$/cancelRequest` |
| OBO format index + `obo_id` in explorer | Yes | `strixonomy inspect` |
| ROBOT interop | — | `strixonomy robot validate\|merge\|report` |
| Diagnostics / lint | Problems panel | `strixonomy validate` |
| Hover, go-to-definition, symbols, find references, rename | Yes (hover linkifies labels/comments) | — |
| Annotation hyperlinks + Protégé-default annotation-property order | Yes (Entity Inspector) | — |
| `catalog-v001.xml` import redirects | Yes (index / resolve) | via workspace index |
| Ontology `version_iri` | Yes (inspector / catalog) | `strixonomy inspect` / catalog |
| Turtle completion (prefix, QName, IRI) | Yes (LSP) | — |
| Diagnostic quick fixes (code actions) | Yes | — |
| Turtle imports add/remove | Yes (Manage Imports panel) | `strixonomy patch` (`add_import`, `remove_import`) |
| Documentation export (Markdown / HTML) | — | `strixonomy docs` |
| Patch preview | Inspector / Manchester editor / refactor preview / imports panel | `strixonomy patch --preview` |
| Semantic diff (versions / workspace compare) | Semantic Diff panel (React) | `strixonomy diff` / `--pr-summary` |
| Cross-panel focus sync | Explorer → Inspector + Graph (relay) | — |
| LSP semantic tokens (Turtle, OBO) | Editor highlighting | — |
| Configurable diagnostics | Problems panel + `.strixonomy/diagnostics.toml` | `strixonomy validate` |
| React webview UI | Inspector, graphs, Query Workbench (SQL/SPARQL/DL), Manchester editor, refactor preview, semantic diff, imports | — |
| Accessibility (WCAG 2.2 AA owned surfaces) | Keyboard + SR patterns, DialogShell focus trap, reduced motion, axe Vitest harness | — |
| Plugin SDK 1.0 (manifest + lifecycle + providers) | Plugin commands, views, inspector cards, preferences, context actions; provider pickers via `listPlugins` | `strixonomy plugins` (list/info/enable/disable/run) / `workflow` |
| Plugin permissions (`api_version = "1"`) | Enforced on plugin load/run | Enforced on CLI/LSP plugin host |
| Reference plugins (naming, Markdown export, SHACL scaffold + reasoner/query/refactor/graph stubs) | Via validate + plugins | `strixonomy plugins run` |

## Format support

| Operation | Turtle (`.ttl`) | OBO (`.obo`) | RDF/XML (`.rdf`, `.owl`) | OWL/XML (`.owx`) | JSON-LD, N-Triples, TriG |
|-----------|-----------------|--------------|--------------------------|------------------|---------------------------|
| Index / query | Yes | Yes | Yes (Horned catalog) | Yes (Horned catalog) | Yes |
| Write-back (inspector, patches) | Yes | Yes | Yes (Horned re-serialize) | Yes (Horned re-serialize) | Read-only |
| Refactor apply (rename / merge / replace) | Yes | Yes¹ | Yes¹ | Yes¹ | — |
| Refactor apply (move / extract / ontology merge / flatten / cleanup) | Yes | — | — | — | — |
| Rich OBO metadata (synonyms, defs, xrefs) | — | Yes | — | — | — |

> **OBO versioning:** patch engine write-back since **v0.12**; Entity Inspector write-back since **v0.13**.  
> **XML write-back:** semantic fidelity (ADR-0021); not byte-identical to Protégé saves.  
> **¹ Rename / merge / replace:** format-specific IRI remaps (XML re-serialize; OBO id/reference rewrite). Other refactor ops stay Turtle-first.  
> Deeper capability grid (Manchester, refactor, XML re-serialize): [Capabilities by format](guides/capabilities-by-format.md).

## New in v0.27.0 (latest tagged)

| Capability | Status |
|------------|--------|
| Strixonomy product identity (`strixonomy` / `strixonomy-*`, CLI, LSP, extension) | Shipped |
| Legacy OntoCore/OntoCode compatibility (bins, LSP methods, paths, thin `ontocore` façade) | Shipped |
| Extension settings/state migration from `ontocode.*` | Shipped |
| Dual-read `.strixonomy/` vs `.ontocore/` / `.ontocode/` | Shipped |

## Earlier releases (still shipped)

Capability history for **v0.26 … v0.12** (Protégé Desktop test port, graphs, Plugin SDK 1.0, a11y, DL Query, multi-format rename/merge/replace, realization, SWRL, complete OWL 2 / Manchester authoring, XML write-back, workspace runtime, …) lives in the [CHANGELOG](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md) and [migration index](migration/README.md). The matrix at the top of this page is the source of truth for **what works today**.

Full user-facing delta for the latest tagged release: [CHANGELOG 0.27.0](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md#0270---2026-07-27).

## Release history

Detailed notes for v0.9–v0.21 are in the [CHANGELOG](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md). This page lists **what is available in the latest tagged release**, not every past milestone.

## Manchester scope (v0.22)

**Shipped:** named classes; `and` / `or` / `not`; `some` / `only` / `value` / `Self`; OneOf `{…}`; `min` / `max` / `exact` cardinality; nested restrictions; data restrictions on xsd types; `SubClassOf`, `EquivalentClasses`, and `DisjointClasses` via Manchester editor or patch JSON; domain/range; property chains; HasKey and remaining RBox/ABox ops via patch JSON / inspector.

**Not shipped:** inline Manchester autocomplete in the text buffer.
Remaining 1.0 targets: [known limitations](known-limitations.md) · [Protégé vs Strixonomy](guides/protege-decision.md).

## Known limitations

| Limitation | Notes |
|------------|-------|
| Multi-root VS Code workspaces | **All folders indexed** (v0.10+), including peer folders added after open. Manual **Index Workspace** may prompt when multiple roots are open |
| Write-back | **Turtle, OBO, RDF/XML, OWL/XML**; JSON-LD, N-Triples, TriG read-only. XML is semantic re-serialize (not byte-identical). See [Capabilities by format](guides/capabilities-by-format.md) |
| Refactoring | Rename / merge / replace: Turtle + RDF/XML + OWL/XML + OBO (format-specific remaps). Move / extract / flatten / cleanup imports: Turtle-first. Extract uses direct-reference closure (optional `--locality`) — not full locality profiling |
| Class hierarchy tree | Named-parent edges; **inferred/combined** after reasoner run |
| Reasoning | **EL / RL / RDFS / DL / auto** via Ontologos 1.x (not certified HermiT-identical) |
| CLI release binaries | Linux x64 only; macOS/Windows use `cargo install` or bundled LSP in VSIX |
| Scale | See [workspace limits](workspace-limits.md) (includes walk entry cap) |

## What's next

Forward: Protégé-competitive release (**1.0**). **v0.27.0** is the current tagged release. See **[Platform roadmap](roadmap.md)** · **[Known limitations](known-limitations.md)**.

## Where to learn more

| Topic | Guide |
|-------|-------|
| First success | [First success](guides/first-success.md) |
| Authoring | [Authoring](authoring.md) |
| OWL/XML & RDF/XML write-back | [OWL/XML workflow](guides/owl-xml-workflow.md) |
| OBO | [OBO authoring](ide/obo-authoring.md) |
| Versions | [Versions & channels](guides/versions-and-channels.md) |
