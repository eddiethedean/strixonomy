# strixonomy

**Strixonomy** is the public façade API for the semantic workspace engine.

This crate re-exports the stable surface of Strixonomy for Rust applications. Implementation lives in the `strixonomy-*` crates (`strixonomy-core`, `strixonomy-catalog`, `strixonomy-query`, `strixonomy-edit`, etc.) — use those directly when you need lower-level control.

## Quick start

```rust
use strixonomy::Workspace;

let workspace = Workspace::open(".")?;
let diagnostics = workspace.diagnostics();

let result = workspace.query("SELECT short_name, labels FROM classes")?;
for row in &result.rows {
    println!("{:?}", row);
}
```

## Modules

| Module | Role |
|--------|------|
| `workspace` | High-level `Workspace::open` API — incremental index, diff, import graph |
| `diff` | Semantic catalog diff, version refs, breaking-change heuristics |
| `catalog` | Index builder and entity catalog |
| `query` | SQL virtual tables and SPARQL |
| `diagnostics` | Lint rule collection |
| `parser` | RDF/OBO parsing |
| `owl` | Horned-OWL bridge, patches, Manchester |
| `reasoner` | Ontologos classification facade |
| `refactor` | Workspace refactoring |
| `docs` | Markdown/HTML documentation export |
| `lsp` | LSP protocol types (feature `lsp`, enabled by default in docs.rs via `all-features`) |

## Ecosystem

| Component | Role |
|-----------|------|
| **Strixonomy** (`strixonomy`, `strixonomy-*`) | Semantic workspace engine — this crate |
| **Ontologos** | Reasoning engine (classification, consistency, explanations) |
| **Strixonomy IDE** | VS Code extension powered by the Strixonomy engine |
| **External plugins** (e.g. [owlmake](https://github.com/INCATools/owlmake)) | Workflow/build/release automation — integrate via Strixonomy plugin APIs, not core dependencies |

## Features

| Feature | Description |
|---------|-------------|
| `lsp` | Re-export `strixonomy_lsp::protocol` and `catalog_snapshot_json` |
| `plugins` | Plugin manifest discovery via `strixonomy::plugin` (v0.14 foundation) |

Default features are **empty** — enable `lsp` when you need LSP wire types:

```toml
strixonomy = { version = "0.26", features = ["lsp"] }
```

## Binaries

- **CLI:** `strixonomy` (`strixonomy-cli` crate) — `cargo install strixonomy-cli --locked`
- **LSP:** `strixonomy-lsp` — bundled in the Strixonomy VS Code extension

## Documentation

- [docs.rs](https://docs.rs/strixonomy)
- [What ships today](https://strixonomy-vs.readthedocs.io/en/latest/SHIPPED/) · [CHANGELOG](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md)
- [docs/strixonomy/](https://github.com/eddiethedean/strixonomy/tree/main/docs/strixonomy) in this repository

**Current version: 0.27.0**
