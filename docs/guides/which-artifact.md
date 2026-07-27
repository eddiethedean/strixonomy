# Which artifact do I need?

Strixonomy is a product family. You usually need **one** primary artifact — not all of them.

## Decision table

| I want to… | Install | Docs |
|------------|---------|------|
| Browse and edit ontologies in VS Code | [Strixonomy extension](../install.md) (Marketplace or Open VSX) | [First success (~10 min)](first-success.md) |
| Validate, query, or classify ontologies in CI | **Linux x64:** release tarball from [Releases](https://github.com/eddiethedean/strixonomy/releases) (preferred). **macOS / Windows / other:** `cargo install strixonomy-cli --locked --version 0.26.2` | [CI integration](../ci-integration.md) · [Install](../install.md) · [Install CLI & CI (detail)](../install-cli-ci.md) |
| Embed indexing/query in a Rust application | `strixonomy = "0.26"` in `Cargo.toml` | [Rust library guide](rust-library.md) |
| Build a custom editor on the language server | Bundle or spawn `strixonomy-lsp` | [LSP API](../lsp-api.md) |
| Run OWL reasoning (classification, explanations) | Included via Strixonomy — no separate install | [Reasoner guide](reasoner.md) |

## Product names (30 seconds)

| Name | What it is |
|------|------------|
| **Strixonomy** | VS Code / Cursor extension (IDE) |
| **Strixonomy** | Rust engine — `strixonomy` CLI, `strixonomy-lsp`, and `strixonomy-*` crates |
| **Ontologos** | External reasoner library used by Strixonomy (not installed separately for normal use) |

The extension bundles `strixonomy-lsp`. You do **not** need Rust to use the IDE.

## Common combinations

**Solo ontology author:** Strixonomy extension only.

**Team with Git + CI:** Strixonomy extension locally + `strixonomy validate` (and optional `classify` / `diff`) in CI.

**Rust service or pipeline:** `strixonomy` crate or `strixonomy-cli` — no VS Code required.

**Air-gapped enterprise:** VSIX + optional CLI tarball from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) — [Enterprise deployment](enterprise-deployment.md).

## What ships today

Canonical capability matrix: [What ships today](../SHIPPED.md).

## Next step

- IDE path → [First success (~10 min core path)](first-success.md)
- CLI / Rust path → [Rust & CLI guide](rust-crates.md)
