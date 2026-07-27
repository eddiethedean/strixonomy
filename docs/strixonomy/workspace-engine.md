# Strixonomy workspace engine

Strixonomy discovers ontology files under a workspace root, parses them, and builds a queryable catalog.

## `Workspace` API

**Stable since v0.10** as the recommended high-level API (pre-1.0 semver still applies until v1.0 — [API stability](../guides/api-stability.md)). Lower-level `IndexBuilder` remains available for custom pipelines.

```rust
use strixonomy::Workspace;

let ws = Workspace::open("/path/to/ontologies")?;

// Catalog stats
let stats = ws.stats();
println!("{} classes", stats.class_count);

// SQL query
let rows = ws.query("SELECT short_name, labels FROM classes")?;

// Entity search (IRI, short name, label)
let hits = ws.search("Person")?;

// Diagnostics from indexing
for d in ws.diagnostics() {
    println!("{:?}: {}", d.code, d.message);
}
```

## Lower-level indexing

```rust
use strixonomy::catalog::IndexBuilder;
use strixonomy::query::query_catalog;

let catalog = IndexBuilder::new()
    .workspace("fixtures")
    .build()?;

let result = query_catalog(&catalog, "SELECT * FROM classes")?;
```

LSP uses `document_overrides` on `IndexBuilder` for unsaved editor buffers.

## Indexing pipeline

1. **Scan** — `WorkspaceScanner` (`strixonomy-core`) walks the workspace with ignore rules.
2. **Parse** — RDF via Oxigraph; Turtle also bridged to Horned-OWL for axioms.
3. **Catalog** — entities, axioms, annotations, imports, namespaces assembled in memory.
4. **Diagnostics** — parse errors + lint rules collected during build.
5. **Store** — Oxigraph in-memory store for SPARQL (not exposed directly; use `sparql_catalog`).

## Resource limits

Indexing enforces caps on file size, entity count, triple count, and query result rows. See [workspace limits](../workspace-limits.md).

## CLI and LSP entry points

| Entry | Command / method |
|-------|------------------|
| CLI | `strixonomy query . "SELECT …"` |
| CLI | `strixonomy validate .` |
| LSP | `strixonomy/indexWorkspace` |

The CLI binary is **`strixonomy`** (`strixonomy-cli` crate). Install with `cargo install strixonomy-cli --locked`.
