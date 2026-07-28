# Known limitations

> **Latest tagged release: v0.28.0.** Pin CI to a tagged version from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) or crates.io — see [What ships today](SHIPPED.md) and [Versions & channels](guides/versions-and-channels.md). **Not a full Protégé replacement today** — coexistence and pilot workflows are the supported path until **v0.30**.

Honest limits for evaluators and new users.

## Editable formats

| Can edit (write-back) | Index / browse / query only |
|-----------------------|-----------------------------|
| Turtle (`.ttl`) | JSON-LD, N-Triples, N-Quads, TriG |
| OBO (`.obo`) | |
| RDF/XML (`.rdf`, `.owl`) | |
| OWL/XML (`.owx`) | |

Entity Inspector and patch write-back apply to **`.ttl`, `.obo`, `.owl`/`.rdf`, and `.owx`**. XML write-back is **semantic re-serialize** (ADR-0021), not byte-identical to Protégé. See [Supported formats](supported-formats.md) and [OWL/XML workflow](guides/owl-xml-workflow.md).

Axiom annotations on XML match named entities (`axiom_op` + `subject_iri` + optional `related_iri`); complex class-expression identity is not fully covered. See [patch-reference](patch-reference.md).

## Catalog SQL (subset)

`strixonomy query` and Query Workbench **SQL mode** are **not** full SQL. Supported: single-table `SELECT`, limited `WHERE` (`=`, `!=`, `AND`, `OR`, booleans). **No** `JOIN`, `GROUP BY`, `ORDER BY`, or `LIMIT`. Prefer [SPARQL](sparql-reference.md) for graph patterns. Details: [SQL reference](sql-reference.md).

## CLI binaries

| Platform | Prebuilt CLI tarball | Recommended install |
|----------|----------------------|---------------------|
| Linux x64 | Yes (GitHub Releases) | Tarball or `cargo install strixonomy-cli --locked` |
| macOS | No | `cargo install strixonomy-cli --locked` (Rust **1.88+**; first build 15–30+ min) |
| Windows | No | `cargo install strixonomy-cli --locked` |

Interactive editing does **not** need the CLI — use the [VS Code / Cursor extension](vscode-install.md) (bundled language server on all platforms).

## Plugins and owlmake

Plugin **SDK 1.0** freezes the TOML + subprocess JSON wire (`api_version = "1"`) — safe to author against today. Production workflow integration and curated discovery are planned for **v0.33**. See [Plugin authoring](guides/plugins.md) and [Plugin policy](guides/plugin-policy.md).

## API stability (v0.x)

Published crates are **0.28.x**. Library APIs, LSP JSON, and SQL table columns may change between minor releases on the v0.x line; compatibility guarantees are stated per surface. Pin in CI: `cargo install strixonomy-cli --locked --version 0.28.0`.

## Reasoning

EL / RL / RDFS / DL classification ships via **Ontologos 1.x** (crates pinned in the workspace). Explanations are **DL-first** for the DL profile (with EL/RL/RDFS alternatives). Realization and instance checking ship in v0.23. **DL Query** (Workbench DL mode, `strixonomy dl-query`, LSP `strixonomy/dlQuery`) ships in v0.24. **Stop** sets an engine cancel flag and ignores late results. See [Reasoner guide](guides/reasoner.md) and [DL Query](guides/dl-query.md).

## Refactoring

**Rename / merge / replace** apply to Turtle, RDF/XML (`.owl`/`.rdf`), OWL/XML (`.owx`), and OBO (semantic re-serialize for XML — ADR-0021; OBO id/reference rewrite). **Move entity/axioms**, **module extract**, and **ontology merge / flatten / cleanup imports** remain **Turtle-first** (non-Turtle files skipped with warnings). See [What ships today](SHIPPED.md) and [v0.24 migration](migration/v0.24.md).

## Layout persistence

Webview **tabs** survive VS Code reload. Restored tabs offer **Reopen panel** using the last saved command + context. Full Protégé-style dock/layout serialization remains a **v0.30** IDE-shell item. Named perspectives open a fixed panel set.

## Large ontologies

Graphs may be **truncated** (badge in the Graph panel). Prefer narrower search, lower neighborhood depth, or asserted-only mode. See [workspace limits](workspace-limits.md).

## When not to use Strixonomy today

- You need **byte-identical OWL/XML or RDF/XML** that matches Protégé layout — Strixonomy re-serializes for semantic fidelity (ADR-0021); use Protégé when layout identity matters.
- You need **JSON-LD / TriG / N-Triples write-back** — still read-only; use Turtle or convert.
- You need **move / extract / ontology-merge refactor on non-Turtle files** — those operations stay Turtle-first (rename/merge/replace already multi-format). See [v0.24 migration](migration/v0.24.md).
- You need **full SQL analytics** — use SPARQL or an external store.
- You need a **curated plugin marketplace** or **production owlmake** without accepting the subprocess SDK — these are planned for **v0.33** — [Plugin policy](guides/plugin-policy.md) (wire contract is already frozen as SDK 1.0).
- You need **WebProtégé-style collaboration** — governed collaboration is planned for **v0.37**.
- You need **HermiT-identical** DL explanations or certified Protégé+HermiT equivalence — dual-tool checks remain recommended for critical audits.

More: [First success (~10 min)](guides/first-success.md) · [Protégé migration](guides/protege-migration.md) · [Protégé decision](guides/protege-decision.md) · [FAQ](faq.md)
