# Examples

Runnable assets for Strixonomy and Strixonomy. **Canonical documentation:** [docs/examples/index.md](../docs/examples/index.md) on Read the Docs.

## Quick start (git clone)

```bash
cargo run -- query fixtures "SELECT * FROM classes"
cargo run -- validate fixtures
cargo run -p strixonomy-workspace --example index_and_query
```

## End-to-end workflow (clone)

Edit → validate → classify → diff on the bundled `fixtures/` tree:

```bash
# 1. Index and inspect
cargo run -- inspect fixtures

# 2. Lint gate (CI-style)
cargo run -- validate fixtures

# 3. EL consistency gate
cargo run -- classify fixtures --profile el --format json

# 4. Semantic diff (requires git checkout)
cargo run -- diff HEAD..WORKTREE --format markdown

# 5. Optional: patch preview on Turtle
cargo run -- patch fixtures/example.ttl '[]' --preview
```

VS Code equivalent: [First success (~10 min)](../docs/guides/first-success.md) → edit in inspector → **Index Workspace** → Reasoner → Semantic Diff panel.

## Contents

| Path | Description |
|------|-------------|
| `index_and_query.rs` | `Workspace` + SQL query on `fixtures/` |
| `strixonomy_workspace.rs` | High-level `Workspace` API |
| `workspace_operations.rs` | Classify, import graph, docs export |
| `error_handling.rs` | Error handling |
| `semantic_diff.rs` | Semantic diff |
| `obo-workflow/` | OBO smoke workspace |
| `protege-roundtrip/` | Protégé-style Turtle / RDF/XML / OWL/XML fixtures (v0.18 expanded) |
| `plugin-workspace/` | Sample `.strixonomy/plugins/` manifests for plugin host demos |

Query cookbook: [docs/examples/queries.md](../docs/examples/queries.md) (not `examples/queries.md`).
