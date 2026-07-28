# Strixonomy crate map

Strixonomy is currently implemented by the `strixonomy-*` crates. The [`strixonomy`](https://crates.io/crates/ontocore) crate is the public façade.

## Façade

| Crate | Role |
|-------|------|
| `strixonomy` | Public API — `Workspace`, module re-exports |

```toml
[dependencies]
ontocore = "0.26"
# Optional: strixonomy = { version = "0.19", features = ["lsp", "plugins"] }
```

```rust
use strixonomy::workspace::Workspace;
use strixonomy::catalog::IndexBuilder;  // lower-level
```

## Implementation crates

| Crate | Role |
|-------|------|
| `strixonomy-core` | Types, workspace scanner, limits, path jail |
| `strixonomy-parser` | RDF parsing (Oxigraph), OBO index |
| `strixonomy-catalog` | Index builder, entity API, graph payloads |
| `strixonomy-query` | SQL virtual tables, SPARQL |
| `strixonomy-owl` | Horned-OWL facade, patches, Manchester |
| `strixonomy-obo` | OBO Format 1.4 patch write-back |
| `strixonomy-edit` | Semantic transaction apply path (v0.20) |
| `strixonomy-diagnostics` | Lint rules |
| `strixonomy-reasoner` | OntoLogos classification facade |
| `strixonomy-swrl` | SWRL rule IR, validation, Turtle helpers (v0.24) |
| `strixonomy-refactor` | Workspace refactoring |
| `strixonomy-diff` | Semantic catalog diff, git compare |
| `strixonomy-docs` | Markdown/HTML documentation export |
| `strixonomy-robot` | ROBOT CLI wrappers |
| `strixonomy-lsp` | Language server binary + protocol types |
| `strixonomy-plugin` | Plugin manifest discovery and host runtime (v0.14+) |
| `strixonomy-plugin-naming` | Reference naming validator plugin |
| `strixonomy-plugin-markdown-export` | Reference Markdown export plugin |
| `strixonomy-plugin-shacl` | SHACL validator scaffold plugin |
| `strixonomy-plugin-builtins` | Built-in plugin wiring for CLI/LSP |
| `strixonomy-cli` | `strixonomy` binary |

`strixonomy-robot` is not re-exported by `strixonomy` — use `strixonomy-robot` or the CLI directly for ROBOT interop.

## Module map (`strixonomy`)

| `strixonomy` module | Source crate |
|-------------------|--------------|
| `workspace` | Wraps `strixonomy-catalog` |
| `catalog` | `strixonomy-catalog` |
| `query` | `strixonomy-query` |
| `diagnostics` | `strixonomy-diagnostics` |
| `parser` | `strixonomy-parser` |
| `owl` | `strixonomy-owl` |
| `obo` | `strixonomy-obo` |
| `edit` | `strixonomy-edit` |
| `reasoner` | `strixonomy-reasoner` |
| `swrl` | `strixonomy-swrl` |
| `refactor` | `strixonomy-refactor` |
| `diff` | `strixonomy-diff` |
| `docs` | `strixonomy-docs` |
| `lsp` | `strixonomy-lsp` (feature `lsp`, opt-in) |
| `plugin` | `strixonomy-plugin` (feature `plugins`, opt-in) |

## Examples

```bash
cargo run -p strixonomy --example strixonomy_workspace
cargo run -p strixonomy --example index_and_query
```

See [Rust library guide](../guides/rust-library.md) for classification and error-handling examples.
