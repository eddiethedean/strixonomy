# Rust & CLI (Strixonomy)

> **Canonical embedder guide:** [Rust library guide](rust-library.md) — `Workspace`, classification, transactions, and examples.
>
> This page is a short CLI-oriented index. For VS Code only, see [Strixonomy VS Code extension](../ide/vscode-extension.md).

**Strixonomy** is the Rust semantic workspace engine: `strixonomy` CLI, `strixonomy-*` crates on [crates.io](https://crates.io/search?q=ontocore), and `strixonomy-lsp` (bundled in the VS Code extension).

## Quick start

```bash
cargo install ontocore-cli --locked --version 0.26.2
strixonomy query /path/to/ontologies "SELECT * FROM classes"
strixonomy validate /path/to/ontologies
```

**Linux CI:** prefer the [release binary](../ci-integration.md) over `cargo install` on every job.

[:octicons-arrow-right-24: Install CLI & CI (detail)](../install-cli-ci.md)

## CLI workflows

| Task | Guide / command |
|------|-----------------|
| Index and inspect | `strixonomy inspect <workspace>` — [CLI reference](../cli-reference.md) |
| SQL virtual tables | `strixonomy query` — [Strixonomy SQL views](../strixonomy/sql-views.md) |
| SPARQL | `strixonomy sparql` — [SPARQL reference](../sparql-reference.md) |
| Lint / CI gate | `strixonomy validate` — [CI integration](../ci-integration.md) |
| EL / RL / RDFS classify | `strixonomy classify` — [Reasoner](reasoner.md) |
| Turtle / OBO patches | `strixonomy patch` — [Patch reference](../patch-reference.md) |
| Workspace refactor | `strixonomy refactor` — [Refactoring guide](refactoring.md) |

## Rust library embedding

| Topic | Guide |
|-------|-------|
| **Start here** | [Rust library guide](rust-library.md) |
| API crosswalk | [Rust API reference](../strixonomy/rust-api.md) |
| Crate map | [strixonomy/crate-map.md](../strixonomy/crate-map.md) |
| `Workspace` example | [`examples/strixonomy_workspace.rs`](https://github.com/eddiethedean/strixonomy/blob/main/examples/strixonomy_workspace.rs) |

Primary dependency: `ontocore = "0.26"`.

## Related

- [Strixonomy overview](../strixonomy/index.md)
- [Which artifact?](which-artifact.md)
- [API stability (pre-1.0)](api-stability.md)
