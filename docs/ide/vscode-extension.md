# Strixonomy VS Code extension

[![Open VSX](https://img.shields.io/open-vsx/v/strixonomy/strixonomy)](https://open-vsx.org/extension/strixonomy/strixonomy)
[![VS Code Marketplace](https://badgen.net/vs-marketplace/v/strixonomy.strixonomy?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy)

**Strixonomy** is the VS Code ontology IDE powered by **Strixonomy**. It provides the explorer sidebar, Entity Inspector, Query Workbench, Manchester editor, graph panels, reasoner views, and inline diagnostics.

The extension talks to the bundled **Strixonomy LSP** (`strixonomy-lsp`) — you do **not** need Rust installed for normal use.

> **Looking for the CLI or Rust library?** See [Strixonomy overview](../strixonomy/index.md) and [Rust & CLI guide](../guides/rust-crates.md).

## Quick start

1. Install Strixonomy from the [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) or [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (Cursor).
2. Open a folder with `.ttl`, `.obo`, `.owl`, or other ontology files. Bundled language server works in Restricted Mode — **Trust** only if you set custom `strixonomy.lspPath` or `strixonomy.robotPath`.
3. Open the **Strixonomy** activity bar → **Classes** → click an entity.

[:octicons-arrow-right-24: First success in 10 minutes](../guides/first-success.md)

## Install and setup

| Topic | Guide |
|-------|-------|
| Install, trust, bundled LSP | [Install VS Code](../vscode-install.md) |
| Supported formats, activation | [Install VS Code](../vscode-install.md) · [FAQ](../faq.md) |
| Problems after install | [Troubleshooting](../troubleshooting.md) |

## Strixonomy features

| Feature | Guide |
|---------|-------|
| Browse classes, properties, individuals | [First success](../guides/first-success.md) |
| Entity Inspector | [Inspector](inspector.md) |
| Edit Turtle (labels, parents, create/delete) | [Authoring](../authoring.md) |
| Workspace refactoring | [Refactoring](../guides/refactoring.md) |
| SQL and SPARQL | [Query Workbench](query-workbench.md) |
| Complex axioms (Manchester) | [Manchester editor](manchester-editor.md) |
| Class/property/import graphs | [Graph view](graph-view.md) |
| Semantic diff (versions / workspace) | [Semantic diff](semantic-diff.md) |
| EL / RL / RDFS / DL classification | [Reasoner](../guides/reasoner.md) |
| Working alongside Protégé | [Protégé coexistence](../guides/protege-coexistence.md) |

## Architecture

```text
Strixonomy (TypeScript + React webviews)
        │ stdio LSP
Strixonomy LSP (strixonomy-lsp)
        │
Strixonomy engine (strixonomy / strixonomy-*)
```

Strixonomy owns UI and marketplace packaging. Strixonomy owns indexing, queries, diagnostics, and write-back logic.

## Reference

| Topic | Link |
|-------|------|
| What ships today | [SHIPPED](../SHIPPED.md) |
| Webview protocol | [Webview protocol](../webview-protocol.md) |
| LSP API (Strixonomy) | [Strixonomy LSP](../strixonomy/lsp.md) |

## Next steps

| Goal | Document |
|------|----------|
| First success tutorial | [First success](../guides/first-success.md) |
| Section overview | [Strixonomy overview](index.md) |
| Feature tour | [Feature tour](feature-tour.md) |
| Engine / CLI | [Strixonomy overview](../strixonomy/index.md) |
