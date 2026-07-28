# Extension development

Guide for contributors working on the **Strixonomy** VS Code extension (`extension/`). End-user extension docs: [VS Code extension overview](../ide/vscode-extension.md).

## Repository layout

| Path | Role |
|------|------|
| `extension/src/` | Extension host — activation, commands, tree views, LSP client |
| `extension/webview-ui/` | **OntoUI** React app (Vite) — inspector, graph, query workbench, etc. |
| `extension/server/<platform>/` | Bundled `strixonomy-lsp` binary (populated by build scripts) |
| `crates/strixonomy-lsp/` | Language server implementation (Rust) |

Design specs live under `docs/design/` and `docs/ui/` (not at repo root).

## Prerequisites

- Rust **1.88+**, Node **20**, npm
- VS Code **1.85+** for F5 debugging

See [Contributing](../contributing.md) for full workspace setup.

## Build and test

```bash
# Rust LSP binary
cargo build -p strixonomy-lsp --bins

# Extension + webviews
cd extension
npm ci
npm run compile          # builds webview-ui + esbuild bundle
STRIXONOMY_LSP_BIN="../target/debug/strixonomy-lsp" npm test
```

Press **F5** in VS Code with the `extension/` folder open (or use **Run Extension** launch config) after `npm run compile`.

!!! warning "`npm run watch` does not rebuild webviews"
    `npm run watch` runs **esbuild only** (extension host). After editing React under `webview-ui/`, run `npm run build:webview` or `npm run compile` before F5 — otherwise the Extension Development Host will show a stale UI.

## Key docs

| Topic | Doc |
|-------|-----|
| Contributor workflow | [contributing.md](../contributing.md) |
| LSP / webview debugging | [debugging.md](../debugging.md) |
| Host ↔ React messages | [webview-protocol.md](../webview-protocol.md) |
| Custom LSP methods | [lsp-api.md](../lsp-api.md) |
| OntoUI platform | [platform/OVERVIEW.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md) |
| UI specs | [ui/README.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/README.md) |

## Pre-PR checks

Run extension-related CI locally:

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build -p strixonomy-lsp --bins
cd extension/webview-ui && npm ci && npm test
cd extension && STRIXONOMY_LSP_BIN=../target/debug/strixonomy-lsp npm ci && npm run compile && npm test
./scripts/check-doc-versions.sh
```

For full CI parity (Rust, docs, packaging, VS Code e2e): `./scripts/run-ci-local.sh` — see [Contributing](../contributing.md).

Plugin UI work: see [Capability providers](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/CAPABILITY_PROVIDERS.md) and `extension/webview-ui/src/capabilities/`.

## Related

- [LSP hello world](lsp-hello-world.md) — minimal stdio client
- [Internals](../internals.md) — contributor hub and role-based paths
