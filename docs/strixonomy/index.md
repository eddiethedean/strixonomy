# Strixonomy engine

The **Strixonomy engine** is the semantic workspace for ontology development. It lives in the [Strixonomy repository](https://github.com/eddiethedean/strixonomy) and powers the **Strixonomy IDE** (VS Code).

**Latest tagged: v0.28.1** · [crates.io search](https://crates.io/search?q=strixonomy)

[![strixonomy](https://img.shields.io/badge/strixonomy-0.28.1-blue)](https://crates.io/crates/strixonomy)
[![core](https://img.shields.io/badge/core-0.28.1-blue)](https://crates.io/crates/strixonomy-core)
[![parser](https://img.shields.io/badge/parser-0.28.1-blue)](https://crates.io/crates/strixonomy-parser)
[![catalog](https://img.shields.io/badge/catalog-0.28.1-blue)](https://crates.io/crates/strixonomy-catalog)
[![query](https://img.shields.io/badge/query-0.28.1-blue)](https://crates.io/crates/strixonomy-query)
[![cli](https://img.shields.io/badge/cli-0.28.1-blue)](https://crates.io/crates/strixonomy-cli)
[![lsp](https://img.shields.io/badge/lsp-0.28.1-blue)](https://crates.io/crates/strixonomy-lsp)
[![owl](https://img.shields.io/badge/owl-0.28.1-blue)](https://crates.io/crates/strixonomy-owl)
[![edit](https://img.shields.io/badge/edit-0.28.1-blue)](https://crates.io/crates/strixonomy-edit)
[![reasoner](https://img.shields.io/badge/reasoner-0.28.1-blue)](https://crates.io/crates/strixonomy-reasoner)
[![diff](https://img.shields.io/badge/diff-0.28.1-blue)](https://crates.io/crates/strixonomy-diff)
[![refactor](https://img.shields.io/badge/refactor-0.28.1-blue)](https://crates.io/crates/strixonomy-refactor)
[![docs](https://img.shields.io/badge/docs-0.28.1-blue)](https://crates.io/crates/strixonomy-docs)

Strixonomy indexes ontology workspaces on disk and provides:

- Workspace discovery and indexing
- RDF/OWL/OBO parsing
- Entity catalog and symbol graph
- SQL virtual tables and SPARQL
- Diagnostics and lint rules
- Refactoring (rename, migrate, move, extract)
- Reasoning integration via [OntoLogos](https://github.com/eddiethedean/ontologos)
- Patch write-back for Turtle, OBO, RDF/XML, and OWL/XML (`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx`)
- Semantic diff (version refs, directories, breaking-change heuristics)
- Documentation export (`strixonomy docs`)
- CLI (`strixonomy`) and LSP (`strixonomy-lsp`)

## Relationship to the IDE and Ontologos

| Product | Role |
|---------|------|
| **Strixonomy engine** | Rust platform — indexing, queries, diagnostics, CLI, LSP |
| **Strixonomy IDE** | VS Code extension — explorer, inspector, webviews, marketplace |
| **Ontologos** | OWL reasoning — classification, consistency, explanations |

The Strixonomy IDE is built on the Strixonomy engine. Ontologos is a separate reasoning stack integrated through `strixonomy-reasoner`. See [Product identity](../guides/product-identity.md).

## Public API

Use the [`strixonomy`](https://crates.io/crates/strixonomy) façade crate:

```rust
use strixonomy::Workspace;

let ws = Workspace::open("./ontology")?;
let diagnostics = ws.diagnostics();
let results = ws.query("SELECT short_name FROM classes")?;
let hits = ws.search("Person");
let graph = ws.import_graph()?;
let diff = ws.diff_against_path("./other")?;
```

`search` returns hits directly (not a `Result`). Other methods that can fail use `?`.
**Stable since v0.10:** `Workspace`, `WorkspaceOptions`, and `strixonomy::diff`. Other `strixonomy-*` internals remain v0.29–v0.30 until v0.30.

```rust
use strixonomy::{Workspace, WorkspaceOptions};

let ws = Workspace::open_with_options(
    WorkspaceOptions::single("./ontology").with_disk_cache(true),
)?;
ws.reindex_incremental()?;
```

Lower-level access remains available through `strixonomy-*` crates. See [crate map](crate-map.md).

## Compatibility (v0.10+)

All crates, binaries, and LSP methods use **`strixonomy`** naming. Upgrading from **v0.11.x**? See [v0.13 migration](../migration/v0.13.md). From v0.10? See [v0.11 migration](../migration/v0.11.md). From v0.8? See [v0.9 migration](../migration/v0.9.md).

| Surface | Name |
|---------|------|
| Implementation crates | `strixonomy-*` |
| CLI binary | `strixonomy` |
| LSP binary | `strixonomy-lsp` |
| LSP methods | `strixonomy/*` |
| Diagnostic source | `strixonomy` |

## Next steps

- [Rust API reference](rust-api.md)
- [Architecture](architecture.md)
- [Crate map](crate-map.md)
- [Workspace engine](workspace-engine.md)
- [Roadmap](roadmap.md)
- [Rust & CLI guide](../guides/rust-crates.md)
