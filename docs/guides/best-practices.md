# Best practices

Patterns for using Strixonomy and Strixonomy in daily ontology engineering.

## Repository layout

- Keep ontology source files in a dedicated directory (e.g. `ontologies/` or `src/ontology/`)
- Prefer **Turtle (`.ttl`)** when you need byte-stable diffs or Turtle-first refactor ops (move / extract / ontology merge / flatten / cleanup). **Rename / merge / replace** also rewrite RDF/XML, OWL/XML, and OBO. **RDF/XML and OWL/XML** support write-back (v0.21+, semantic re-serialize) — see [Supported formats](../supported-formats.md) and [OWL/XML write-back](owl-xml-workflow.md).
- Use a single-root VS Code workspace when you want the simplest onboarding (multi-root is supported since v0.10 — all folders are indexed)

## When to use SQL vs SPARQL vs classify

| Tool | Best for |
|------|----------|
| **SQL** (`strixonomy query`) | Catalog tables — classes, properties, diagnostics, axioms metadata |
| **SPARQL** (`strixonomy sparql`) | RDF graph patterns over indexed triples |
| **`validate`** | CI gate for parse errors and lint |
| **`classify`** | CI gate for unsatisfiable classes (EL/RL/RDFS/DL/`auto`) |
| **`realize` / `check-instance`** | ABox inferred types and instance checks (v0.23+) |

SQL and SPARQL results truncate silently at 100,000 rows — narrow queries in CI. See [workspace limits](../workspace-limits.md).

## CI recipe picker

| Goal | Command |
|------|---------|
| Lint + parse | `strixonomy validate .` |
| Unsatisfiable classes (EL) | `strixonomy classify . --profile el --format json` |
| Unsatisfiable classes (DL) | `strixonomy classify . --profile dl --format json` |
| Realization | `strixonomy realize . --profile rl --format json` |
| Inspect errors | `strixonomy query . "SELECT code, message FROM diagnostics WHERE severity = 'error'"` |

Examples: [CI integration](../ci-integration.md)

## VS Code workflow

1. Open ontology folder (single-root)
2. Bundled language server works in Restricted Mode — **Trust** only if you set custom `strixonomy.lspPath` or `strixonomy.robotPath`
3. Edit in Entity Inspector or Manchester editor (`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`)
4. Run **Index Workspace** after bulk file changes
5. Use **Query Workbench** for SQL/SPARQL and **DL mode** for Manchester class-expression queries ([dl-query.md](dl-query.md) — honesty limits vs Protégé DL Query / HermiT)

## Protégé teams

Use Strixonomy for Turtle/OBO/XML editing, CI validate/classify (including DL profile), realization/instance checking, and SWRL authoring/validation. Keep Protégé when you need HermiT-identical explanations, **DL Query tab** workflows, or other gaps in [known limitations](../known-limitations.md) — see [Protégé coexistence](protege-coexistence.md) and [DL Query honesty](dl-query.md).

## Rust embedding

Embed Strixonomy in tools or pipelines via the published crates — see [Rust library guide](rust-library.md).

## Related

- [VS Code extension docs](vscode-extension.md)
- [Rust & CLI docs](rust-crates.md)
- [Enterprise evaluation](enterprise-eval.md)
- [FAQ](../faq.md)
