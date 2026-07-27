# Architecture tour (~15 minutes)

One-page map for new contributors: how Strixonomy, Strixonomy, and the webviews connect.

> **Audience:** Contributors and integrators. Evaluators should start with [Architecture](../architecture.md) and [What ships today](../SHIPPED.md).

## Stack at a glance

```text
VS Code (extension/) ──stdio──► strixonomy-lsp ──► strixonomy (facade)
                                      │              │
                                      │              ├── strixonomy-catalog (index)
                                      │              ├── strixonomy-query (SQL/SPARQL)
                                      │              ├── strixonomy-owl / strixonomy-obo (write-back)
                                      │              ├── strixonomy-reasoner → Ontologos
                                      │              └── strixonomy-plugin (host)
                                      │
React webviews (extension/webview-ui/) ◄── postMessage ── extension host
```

## Repository layout

| Path | Role |
|------|------|
| [`crates/strixonomy/`](https://github.com/eddiethedean/strixonomy/tree/main/crates/strixonomy) | Public Rust façade — `Workspace` API |
| [`crates/strixonomy-lsp/`](https://github.com/eddiethedean/strixonomy/tree/main/crates/strixonomy-lsp) | Language server — LSP + custom `strixonomy/*` JSON-RPC |
| [`crates/strixonomy-cli/`](https://github.com/eddiethedean/strixonomy/tree/main/crates/strixonomy-cli) | `strixonomy` CLI binary |
| [`extension/`](https://github.com/eddiethedean/strixonomy/tree/main/extension) | VS Code extension — trees, commands, LSP client, webview host |
| [`extension/webview-ui/`](https://github.com/eddiethedean/strixonomy/tree/main/extension/webview-ui) | React panels (Inspector, graphs, Query Workbench, …) |
| [`fixtures/`](https://github.com/eddiethedean/strixonomy/tree/main/fixtures) | Sample ontologies for tests and tutorials |
| [`docs/`](../index.md) | Read the Docs source (this site) |

Deep crate layout: [design/ARCHITECTURE.md](../design/ARCHITECTURE.md) (contributors only).

## Request flow (edit in VS Code)

1. User edits in **Entity Inspector** (React webview).
2. Webview sends a patch/refactor message to the extension host ([webview protocol](../webview-protocol.md)).
3. Extension calls LSP `strixonomy/applyAxiomPatch` or refactor methods ([LSP API](../lsp-api.md)).
4. `strixonomy-lsp` applies via `strixonomy-edit` transactions and format adapters (`strixonomy-owl`, `strixonomy-obo`).
5. Updated file text returns to VS Code; catalog re-indexes incrementally.

## Where to change what

| You want to… | Start here |
|--------------|------------|
| Fix Turtle/OBO write-back | `crates/strixonomy-owl/`, `crates/strixonomy-obo/` |
| Add LSP method | `crates/strixonomy-lsp/src/handlers.rs`, [LSP API](../lsp-api.md) |
| Add CLI command | `crates/strixonomy-cli/src/main.rs`, [CLI reference](../cli-reference.md) |
| Add VS Code command | `extension/src/commands/` |
| Add React panel | `extension/webview-ui/src/panels/` |
| Add diagnostic rule | `crates/strixonomy-diagnostics/` |

## Next steps

- [Internals](../internals.md) — role-based contributor paths
- [Testing matrix](testing-matrix.md) — which tests to run by change type
- [Extension development](extension-development.md)
- [LSP hello world](lsp-hello-world.md)
