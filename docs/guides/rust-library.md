# Using Strixonomy as a Rust library

Embed **Strixonomy** in tools, pipelines, or custom CLIs via the [`strixonomy`](https://crates.io/crates/strixonomy) façade crate from **crates.io**. You do **not** need to clone this repository.

> Strixonomy (previously branded **OntoIndex** / `ontoindex-*`) is implemented by the `strixonomy-*` crates. See [v0.9 migration](../migration/v0.9.md).

Pre-1.0: public APIs may change between minor releases until v1.0. Pin minors in production. Crates are at **0.28.x**.

!!! tip "Prefer `Workspace`"
    For new code, use the **`Workspace` API** (`strixonomy = "0.28"`). Lower-level `IndexBuilder` remains available for specialized pipelines — see [Rust API](../strixonomy/rust-api.md).

## crates.io first (5 minutes)

1. Create a crate (or open an existing one).
2. Add Strixonomy:

```toml
[dependencies]
strixonomy = "0.28"
```

3. Point `Workspace::open` at **your** ontology directory (any folder of `.ttl` / `.obo` / other indexed formats):

```rust
use strixonomy::Workspace;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws = Workspace::open("./ontologies")?;
    let result = ws.query("SELECT short_name, labels FROM classes")?;
    for row in &result.rows {
        println!("{} — {}", row["short_name"], row["labels"]);
    }
    Ok(())
}
```

4. Run with `cargo run`. First compile pulls Strixonomy dependencies (can take several minutes cold).

**Errors:** `Workspace::open` returns `CatalogError`; `query` / `sparql` return `QueryError`. The façade also exposes a unified [`strixonomy::Error`](https://docs.rs/strixonomy/latest/strixonomy/enum.Error.html) for `From` conversion — see [Errors reference](../errors.md#rust-library-errors).

Method-level params / returns / side effects: [Rust API — Workspace methods](../strixonomy/rust-api.md#workspace-method-reference).

## Minimal `Cargo.toml` recipes

**Query / index only** (façade defaults):

```toml
[dependencies]
strixonomy = "0.28"
```

**Classify + explain** (same crate — reasoner is included):

```toml
[dependencies]
strixonomy = "0.28"
```

```rust
use strixonomy::Workspace;
use strixonomy::reasoner::ReasonerId;

let ws = Workspace::open("./ontologies")?;
let result = ws.classify(ReasonerId::El)?;
```

**Semantic patch / transactions** (extra crates):

```toml
[dependencies]
strixonomy = "0.28"
strixonomy-edit = "0.27"
strixonomy-owl = "0.28"
```

See [Semantic transactions](#semantic-transactions-strixonomy-edit) below.

## Optional: monorepo examples (clone only)

In-repo examples under the unpublished `strixonomy` package need a git clone:

```bash
git clone https://github.com/eddiethedean/strixonomy.git && cd strixonomy
cargo run -p strixonomy --example strixonomy_workspace
cargo run -p strixonomy --example workspace_operations
cargo run -p strixonomy --example error_handling
```

Those examples use `fixtures/` — that directory exists only in a clone, not after `cargo add strixonomy`.

## Lower-level: index and query

```rust
use strixonomy::catalog::IndexBuilder;
use strixonomy::query::query_catalog;

let catalog = IndexBuilder::new().workspace(".").build()?;
let result = query_catalog(&catalog, "SELECT short_name, labels FROM classes")?;
```

## Crate map

See [Strixonomy crate map](../strixonomy/crate-map.md) for the full table. Summary:

| Crate | Role |
|-------|------|
| `strixonomy` | Public façade — `Workspace`, module re-exports |
| `strixonomy-*` | Implementation crates (stable names until v1.0) |

## Classification example

```rust
use strixonomy::Workspace;
use strixonomy::reasoner::ReasonerId;

let ws = Workspace::open(".")?;
let result = ws.classify(ReasonerId::El)?;
println!("consistent: {}", result.consistent);
```

## Workspace options (v0.10+)

```rust
use strixonomy::{Workspace, WorkspaceOptions};

let ws = Workspace::open_with_options(
    WorkspaceOptions::single("./ontology")
        .with_disk_cache(true),
)?;
ws.reindex_incremental()?;
let diff = ws.diff_against_path("./baseline")?;
```

| Option | Purpose |
|--------|---------|
| `WorkspaceOptions::single(path)` | Primary workspace root |
| `with_disk_cache(true)` | Persist parse cache under `.strixonomy/cache/` |
| `reindex_incremental()` | Reuse unchanged documents by content hash |

Semantic diff: `ws.diff()`, `ws.diff_against_path()`, or `strixonomy::diff::diff_git_refs` — see [Semantic diff](../ide/semantic-diff.md).

## Semantic transactions (`strixonomy-edit`)

**v0.19+** ships **`strixonomy-edit`** for ordered, invertible Turtle/OBO edit batches. Use when building undo/redo, audit trails, or multi-step apply pipelines:

```rust
use strixonomy_edit::Transaction;
use strixonomy_owl::PatchOp;

let txn = Transaction::from_turtle(vec![
    PatchOp::SetLabel {
        entity_iri: "http://example.org/Person".into(),
        value: "Person".into(),
    },
]);

let undo = txn.invert()?;
```

Dependency: `strixonomy-edit = "0.27"`. Full API: [Rust API — semantic transactions](../strixonomy/rust-api.md#semantic-transactions-strixonomy-edit-v019) · [docs.rs/strixonomy-edit](https://docs.rs/strixonomy-edit).

## Next steps

| Goal | Doc |
|------|-----|
| Method reference | [Rust API](../strixonomy/rust-api.md) |
| Error types | [Errors](../errors.md#rust-library-errors) |
| CLI instead of embed | [Install CLI & CI (detail)](../install-cli-ci.md) |
| Stability expectations | [API stability](api-stability.md) |
