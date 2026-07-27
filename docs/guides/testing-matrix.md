# Contributor testing matrix

Which tests to run — and how long they usually take — by change type.

> **Docs-only PRs:** `./scripts/check-doc-versions.sh` only (~seconds). No Rust/Node required.

## Quick reference

| Change type | Minimum commands | Typical time (warm cache) | Typical time (cold) |
|-------------|------------------|---------------------------|---------------------|
| Docs only | `./scripts/check-doc-versions.sh` | &lt; 1 min | &lt; 1 min |
| Single Rust crate | `cargo test -p <crate>` + `cargo fmt --all --check` | 1–5 min | 10–30 min |
| LSP / handlers | `cargo test -p strixonomy-lsp` + `cargo build -p strixonomy-lsp --bins` | 2–8 min | 15–40 min |
| CLI | `cargo test -p strixonomy-cli` + integration tests touching CLI | 3–10 min | 20–45 min |
| Extension host | `cd extension && npm ci && ONTOCORE_LSP_BIN=../target/debug/strixonomy-lsp npm test` | 2–5 min | 5–15 min |
| Webview UI | `cd extension/webview-ui && npm ci && npm test` | 1–3 min | 3–8 min |
| Release / wide refactor | `./scripts/run-ci-local.sh` | 15–30 min | 30–60+ min |

Cold times include first `cargo build` on a machine without a populated `target/` directory.

## By subsystem

### Rust engine (`crates/strixonomy-*`)

```bash
cargo fmt --all --check
cargo clippy -p strixonomy-<crate> --all-targets -- -D warnings
cargo test -p strixonomy-<crate>
```

Workspace-wide before merge:

```bash
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Language server

```bash
cargo build -p strixonomy-lsp --bins
cargo test -p strixonomy-lsp
# Optional smoke: tests/lsp_smoke.rs via full workspace tests
```

Build the LSP binary before extension tests: `ONTOCORE_LSP_BIN=../target/debug/strixonomy-lsp`.

### VS Code extension

```bash
cargo build -p strixonomy-lsp --bins
cd extension && npm ci
ONTOCORE_LSP_BIN=../target/debug/strixonomy-lsp npm run compile && npm test
```

F5 debugging: [Extension development](extension-development.md) · [Debugging](../debugging.md).

### Webview UI

```bash
cd extension/webview-ui && npm ci && npm test
```

### Plugins

```bash
cargo test -p strixonomy-plugin -p strixonomy-plugin-naming
./scripts/run-ci-local.sh   # when touching host + reference plugins
```

### Documentation

```bash
./scripts/check-doc-versions.sh
pip install -r docs/requirements.txt && ./scripts/build-docs.sh   # optional strict build
```

## Optional dependencies

| Tool | When required |
|------|----------------|
| Java + `robot` on PATH | ROBOT integration tests, `strixonomy robot` manual checks |
| Node 20 | Extension and webview changes |
| Git repo | `strixonomy diff` integration tests |

## Full CI parity

```bash
./scripts/run-ci-local.sh
```

Matches GitHub Actions — use before release-impacting PRs. See [CONTRIBUTING.md](https://github.com/eddiethedean/strixonomy/blob/main/CONTRIBUTING.md).

## Related

- [Architecture tour](architecture-tour.md)
- [Internals](../internals.md)
- [Automation and stability](../automation-stability.md)
