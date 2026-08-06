# Rust API reference

Strixonomy exposes a Rust library through the [`strixonomy`](https://crates.io/crates/strixonomy) façade crate and individual `strixonomy-*` implementation crates.

## Stability

| Surface | Status |
|---------|--------|
| `strixonomy::Workspace` | **Stable since v0.10** — preferred high-level API |
| `strixonomy::catalog::IndexBuilder` | Stable for custom pipelines |
| LSP JSON (`strixonomy/*` methods) | v0.29–v0.30 — may change between minor releases |
| SQL virtual table columns | v0.29–v0.30 — pin versions in production |

Pin dependencies in `Cargo.toml`:

```toml
[dependencies]
strixonomy = "0.28"
```

For CI and reproducible builds: `cargo install strixonomy-cli --locked --version 0.28.1`.

## docs.rs

Generated API documentation is published on [docs.rs](https://docs.rs/strixonomy):

| Crate | docs.rs |
|-------|---------|
| `strixonomy` | [docs.rs/strixonomy](https://docs.rs/strixonomy) |
| `strixonomy-core` | [docs.rs/strixonomy-core](https://docs.rs/strixonomy-core) |
| `strixonomy-parser` | [docs.rs/strixonomy-parser](https://docs.rs/strixonomy-parser) |
| `strixonomy-catalog` | [docs.rs/strixonomy-catalog](https://docs.rs/strixonomy-catalog) |
| `strixonomy-diagnostics` | [docs.rs/strixonomy-diagnostics](https://docs.rs/strixonomy-diagnostics) |
| `strixonomy-query` | [docs.rs/strixonomy-query](https://docs.rs/strixonomy-query) |
| `strixonomy-reasoner` | [docs.rs/strixonomy-reasoner](https://docs.rs/strixonomy-reasoner) |
| `strixonomy-robot` | [docs.rs/strixonomy-robot](https://docs.rs/strixonomy-robot) |
| `strixonomy-owl` | [docs.rs/strixonomy-owl](https://docs.rs/strixonomy-owl) |
| `strixonomy-obo` | [docs.rs/strixonomy-obo](https://docs.rs/strixonomy-obo) |
| `strixonomy-lsp` | [docs.rs/strixonomy-lsp](https://docs.rs/strixonomy-lsp) |
| `strixonomy-diff` | [docs.rs/strixonomy-diff](https://docs.rs/strixonomy-diff) |
| `strixonomy-docs` | [docs.rs/strixonomy-docs](https://docs.rs/strixonomy-docs) |
| `strixonomy-refactor` | [docs.rs/strixonomy-refactor](https://docs.rs/strixonomy-refactor) |
| `strixonomy-edit` | [docs.rs/strixonomy-edit](https://docs.rs/strixonomy-edit) |
| `strixonomy-plugin` | [docs.rs/strixonomy-plugin](https://docs.rs/strixonomy-plugin) (Plugin SDK 1.0 wire; see [Plugin policy](../guides/plugin-policy.md)) |
| `strixonomy-cli` | [docs.rs/strixonomy-cli](https://docs.rs/strixonomy-cli) |

Search all crates: [crates.io search?q=strixonomy](https://crates.io/search?q=strixonomy).

## Book ↔ docs.rs crosswalk

Use **this book** for workflows, limits, and LSP JSON; use **docs.rs** for Rust type signatures and module layout.

| You need… | Start in the book | Rust API (docs.rs) |
|-----------|-------------------|---------------------|
| Open and query a workspace | [Rust library guide](../guides/rust-library.md), [Workspace engine](workspace-engine.md) | [`Workspace`](https://docs.rs/strixonomy/latest/strixonomy/struct.Workspace.html), [`WorkspaceOptions`](https://docs.rs/strixonomy/latest/strixonomy/struct.WorkspaceOptions.html) |
| SQL virtual tables | [SQL reference](../sql-reference.md), [SQL views](sql-views.md) | [`strixonomy::query`](https://docs.rs/strixonomy/latest/strixonomy/query/index.html) |
| SPARQL | [SPARQL reference](../sparql-reference.md) | [`Workspace::sparql`](https://docs.rs/strixonomy/latest/strixonomy/struct.Workspace.html#method.sparql) |
| Turtle patch apply | [Patch JSON](../patch-reference.md), [Authoring](../authoring.md) | [`strixonomy::owl`](https://docs.rs/strixonomy/latest/strixonomy/owl/index.html) |
| Semantic transactions (v0.19+) | [Rust library guide](../guides/rust-library.md#semantic-transactions-strixonomy-edit) | [`strixonomy-edit`](https://docs.rs/strixonomy-edit/latest/strixonomy_edit/struct.Transaction.html) |
| OBO patch apply | [OBO authoring](../ide/obo-authoring.md) | [`strixonomy::obo`](https://docs.rs/strixonomy-obo/latest/strixonomy_obo/index.html) |
| Semantic diff | [Semantic diff](../ide/semantic-diff.md) | [`strixonomy-diff`](https://docs.rs/strixonomy-diff/latest/strixonomy_diff/index.html) |
| Refactoring | [Refactoring guide](../guides/refactoring.md) | [`strixonomy-refactor`](https://docs.rs/strixonomy-refactor/latest/strixonomy_refactor/index.html) |
| Docs export | [Docs export](../guides/docs-export.md) | [`strixonomy::docs`](https://docs.rs/strixonomy/latest/strixonomy/docs/index.html) |
| LSP integration | [LSP API](../lsp-api.md), [LSP overview](lsp.md) | [`strixonomy-lsp`](https://docs.rs/strixonomy-lsp/latest/strixonomy_lsp/index.html) |
| Custom LSP client | [LSP hello world](../guides/lsp-hello-world.md) | — |
| Error codes / exit behavior | [Errors reference](../errors.md) | Crate `thiserror` types per module |
| Resource limits | [Workspace limits](../workspace-limits.md) | Index builder options in `strixonomy-catalog` |

## Recommended entry point: `Workspace`

```rust
use strixonomy::Workspace;

let ws = Workspace::open("./ontologies")?;

// Catalog stats
let stats = ws.stats();
println!("{} classes", stats.class_count);

// Catalog SQL (subset) — not full SQL; see sql-reference.md
let result = ws.query("SELECT short_name, labels FROM classes")?;

// SPARQL over indexed triples
let sparql = ws.sparql("SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10")?;

// Lint diagnostics from indexing
for d in ws.diagnostics() {
    println!("{:?}: {}", d.code, d.message);
}

// Entity search by IRI fragment, short name, or label
let hits = ws.search("Person");
```

`Workspace::open` returns `Result<Workspace, strixonomy::Error>`. Map failure modes with [Errors reference](../errors.md) and [Known limitations](../known-limitations.md).

### Error taxonomy (façade vs core)

| Type | Use when |
|------|----------|
| [`strixonomy::Error`](https://docs.rs/strixonomy/latest/strixonomy/enum.Error.html) | Preferred in application code against the façade crate — `?` from `Workspace` methods |
| `StrixonomyError` / crate-local errors (`CatalogError`, `QueryError`, …) | Lower-level `strixonomy-*` crates or migration notes that still name core errors |
| CLI exit codes | Automation — [Errors](../errors.md) / [workspace limits](../workspace-limits.md). Note: `classify` with unsatisfiable classes does **not** fail the Rust API by itself; the CLI may exit non-zero when unsat is treated as failure |
| LSP error codes | Editor clients — [LSP API](../lsp-api.md) |

Prefer `strixonomy::Error` for new embedders. Compat / rename details: [v0.27 migration](../migration/v0.27.md).

### `WorkspaceOptions`

```rust
use strixonomy::{Workspace, WorkspaceOptions};

let ws = Workspace::open_with_options(
    WorkspaceOptions::single("./ontology")
        .with_disk_cache(true)
        .with_scan_roots(vec!["./imports".into()]),
)?;
ws.reindex_incremental()?;
let diff = ws.diff_against_path("./baseline")?;
```

| Option | Purpose |
|--------|---------|
| `single(path)` | Primary workspace root |
| `with_scan_roots(vec![...])` | Additional scan roots (primary root is always included) |
| `with_disk_cache(true)` | Persist parse snapshots under `.strixonomy/cache/` |

### Workspace method reference

Operational contract for the high-level façade. Full type signatures: [`Workspace` on docs.rs](https://docs.rs/strixonomy/latest/strixonomy/struct.Workspace.html).

| Method | Parameters | Returns | Errors | Notes |
|--------|------------|---------|--------|-------|
| `open(path)` | Workspace root directory | `Workspace` | `CatalogError` | Full index on open |
| `open_with_options(opts)` | `WorkspaceOptions` | `Workspace` | `CatalogError` | Disk cache, multi-root scan |
| `reindex()` | — | `CatalogStats` | `CatalogError` | Full rebuild |
| `reindex_incremental()` | — | `CatalogStats` | `CatalogError` | Hash-based reuse when possible |
| `stats()` / `diagnostics()` | — | counts / `Vec<Diagnostic>` | — | From last successful index |
| `query(sql)` | Catalog SQL (subset) | `QueryResult` | `QueryError` | Rows may truncate at 100k — check limits |
| `sparql(query)` | SPARQL string | `SparqlResult` | `QueryError` | Same row cap; prefer for graph patterns |
| `search(needle)` | IRI fragment / short name / label | hits | — | In-memory catalog search |
| `classify(profile)` | `ReasonerId` | `ClassificationResult` | `ReasonerError` | Does not fail only because classes are unsatisfiable — inspect `consistent` |
| `explain(profile, request)` | Profile + class/request | `ExplanationResult` | `ReasonerError` | Run after classify when needed |
| `reasoner_input()` | — | `ReasonerInput` | `ReasonerError` | Snapshot for advanced reasoner use |
| `diff(other)` | Another `Workspace` | `DiffResult` | — | In-memory catalog compare |
| `diff_against_path(path)` | Other directory | `DiffResult` | `CatalogError` | Indexes the other path |
| `import_graph()` / `import_graph_with` | Optional request | graph payload | `GraphError` | For visualization / export |
| `export_docs(options)` | Format / output options | — | export errors via `Error` | Markdown/HTML |
| `discover_plugins()` | — | plugin list | host errors | Requires `plugins` feature |
| `catalog()` / `root()` / `scan_roots()` | — | refs / paths | — | Advanced / introspection |

**Side effects:** indexing may write under `.strixonomy/cache/` when disk cache is enabled. Patch helpers (`strixonomy::owl` / `strixonomy::obo`) write ontology source files — preview before apply in production tools.

**Unified façade error:** map crate-local errors into [`strixonomy::Error`](https://docs.rs/strixonomy/latest/strixonomy/enum.Error.html) with `?` when your function returns `Result<_, strixonomy::Error>`. Crosswalk: [Errors — Rust library](../errors.md#rust-library-errors).

### Additional helpers

Use `apply_owl_patches` / `apply_obo_patches` from `strixonomy::owl` and `strixonomy::obo` when importing both patch helpers. Refactoring helpers live in [`strixonomy::refactor`](https://docs.rs/strixonomy/latest/strixonomy/refactor/index.html) (`preview_rename_iri`, `apply_refactor_plan_checked`, …). Pass `workspace_root` from `Workspace::root()`.

## Semantic transactions (`strixonomy-edit`, v0.19+)

v0.19 adds a format-aware **transaction** layer for ordered Turtle/OBO edits with compose, validate, and invert:

```rust
use strixonomy_edit::Transaction;
use strixonomy_owl::PatchOp;

let txn = Transaction::from_turtle(vec![
    PatchOp::AddLabel {
        entity_iri: "http://example.org/Person".into(),
        value: "Person".into(),
    },
]);

// Preview / apply via strixonomy::edit or LSP applyAxiomPatch envelope
let inverted = txn.invert()?;
```

- **Book:** [Rust library guide — semantic transactions](../guides/rust-library.md#semantic-transactions-strixonomy-edit)
- **API:** [`strixonomy_edit::Transaction`](https://docs.rs/strixonomy-edit/latest/strixonomy_edit/struct.Transaction.html)
- **Wire format:** [Patch JSON](../patch-reference.md) (legacy patch arrays still accepted; transactions preferred for undo)

## Lower-level API

When you need buffer overrides, partial rebuilds, or direct catalog access:

```rust
use strixonomy::catalog::IndexBuilder;
use strixonomy::query::query_catalog;

let catalog = IndexBuilder::new().workspace(".").build()?;
let result = query_catalog(&catalog, "SELECT * FROM classes")?;
```

See [Workspace engine](workspace-engine.md) for the indexing pipeline and [crate map](crate-map.md) for module boundaries.

## Documentation export (`docs` module)

```rust
use strixonomy::{Workspace, docs::{export_workspace, ExportOptions}};

let ws = Workspace::open("./fixtures")?;
export_workspace(
    ws.catalog(),
    ExportOptions::markdown("/tmp/onto-docs"),
)?;
```

See [Documentation export guide](../guides/docs-export.md).

## Examples in this repository

```bash
cargo run -p strixonomy --example strixonomy_workspace   # Workspace API
cargo run -p strixonomy --example workspace_operations # classify, graph, docs export
cargo run -p strixonomy --example index_and_query      # Workspace + SQL query (fixtures/)
cargo run -p strixonomy --example error_handling        # strixonomy::Error handling
cargo run -p strixonomy --example semantic_diff         # Git/workspace semantic diff (requires git repo)
```

See [Examples index](../examples/index.md) for CLI cookbooks and fixture workflows.

## Related guides

| Topic | Document |
|-------|----------|
| Embedding walkthrough | [Rust library guide](../guides/rust-library.md) |
| CLI and crates overview | [Rust & CLI guide](../guides/rust-crates.md) |
| LSP wire format | [LSP API](../lsp-api.md) |
| SQL virtual tables | [SQL reference](../sql-reference.md) |
| Error codes | [Errors reference](../errors.md) |
| Resource limits | [Workspace limits](../workspace-limits.md) |
