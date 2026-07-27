# Strixonomy LSP

**Strixonomy LSP** is currently provided by the **`strixonomy-lsp`** binary and crate. The VS Code extension bundles this server and communicates over stdio.

## Binary

| Platform artifact | Name |
|-------------------|------|
| Language server | `strixonomy-lsp` |
| Bundled path (extension) | `extension/server/<platform>-<arch>/strixonomy-lsp` |

The binary has been named **`strixonomy-lsp`** since the v0.9 Strixonomy identity rename. Wire format may change until v1.0 — see [LSP API reference](../lsp-api.md).

## Custom methods

Strixonomy exposes workspace operations via `strixonomy/*` LSP methods:

| Method | Purpose |
|--------|---------|
| `strixonomy/indexWorkspace` | Index workspace folder |
| `strixonomy/getCatalogSnapshot` | Explorer tree data |
| `strixonomy/getEntity` | Entity inspector payload |
| `strixonomy/getGraph` | Graph visualization data |
| `strixonomy/query` / `strixonomy/sparql` | Query workbench |
| `strixonomy/applyAxiomPatch` | Turtle, OBO, RDF/XML, OWL/XML write-back (format dispatch by extension; XML re-serialize) |
| `strixonomy/parseManchester` | Manchester editor |
| `strixonomy/runReasoner` / `strixonomy/getExplanation` | Reasoning |
| `strixonomy/findUsages` / `strixonomy/previewRefactor` / `strixonomy/applyRefactor` | Refactoring |
| `strixonomy/semanticDiff` | Semantic diff between git refs or directories |

Full wire format: **[LSP API reference](../lsp-api.md)**.

## Rust library

```rust
use strixonomy::lsp::catalog_snapshot_json;
use strixonomy::catalog::IndexBuilder;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let json = catalog_snapshot_json(&catalog)?;
```

Protocol types: `strixonomy::lsp::protocol`.

## Diagnostics

LSP publishes diagnostics with `source: "strixonomy"`. This identifier is unchanged in v0.9 for compatibility.
