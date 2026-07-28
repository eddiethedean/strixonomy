# Install CLI & CI (detail)

> **Prefer the canonical [Install](install.md) page** (extension first, optional CLI). This page is the **CLI/CI detail matrix** only — do not treat it as a second install SSOT. Artifact names and pins must match [Install](install.md) and [Versions & channels](guides/versions-and-channels.md#air-gap-artifact-manifest).

IDE tutorial: [First success (~10 min)](guides/first-success.md). Canonical product install: [Install](install.md). Names: [Product identity](guides/product-identity.md).

!!! note "First `cargo install` or clone build"
    A cold Rust toolchain may take **15–30+ minutes** to compile Strixonomy on first run. The VS Code extension bundles `strixonomy-lsp` and does not require Rust. **Linux x64 CI should prefer the release tarball** ([CI integration](ci-integration.md)) over `cargo install`.

!!! note "Linux arm64 and non-x64"
    Prebuilt CLI tarballs are **`x86_64-unknown-linux-gnu` only**. On Linux arm64 (and other unsupported targets), use `cargo install strixonomy-cli --locked --version 0.27.0` or the language server bundled in the VSIX — see [platform compatibility](guides/platform-compatibility.md).

## Install matrix (CLI)

| Method | Linux x64 | macOS | Windows | Needs Rust? |
|--------|-----------|-------|---------|-------------|
| `cargo install strixonomy-cli --locked --version 0.27.0` | Yes | Yes | Yes | Yes (1.88+) |
| Release CLI tarball (`strixonomy-v*-x86_64-unknown-linux-gnu`) | Yes | No | No | No |
| Git clone + `cargo run --` | Yes | Yes | Yes | Yes (1.88+) |

!!! note "SQL-like queries"
    `strixonomy query` uses **catalog SQL (subset)** — single-table `SELECT`, limited `WHERE`. Not full SQL — see [SQL reference](sql-reference.md) and [Known limitations](known-limitations.md).

## Prerequisites

| Path | Requires |
|------|----------|
| `cargo install` CLI | Rust 1.88+; `~/.cargo/bin` on your `PATH` |
| Git clone + `cargo run` | Rust 1.88+ |
| Release CLI binaries | No Rust; **Linux x64 only** (download from GitHub Releases). macOS/Windows: use `cargo install` or the VS Code extension. |

**Native toolchain (when using cargo):**

| OS | Extra requirement |
|----|-------------------|
| **Windows** | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ workload) — linkers fail without them |
| **macOS** | Xcode Command Line Tools: `xcode-select --install` |
| **Linux** | Usually covered by distro `build-essential` / equivalent |

After `cargo install`, ensure `~/.cargo/bin` is on your `PATH`. If you see `strixonomy: command not found`, run:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

Add that line to your shell profile (`~/.bashrc`, `~/.zshrc`, etc.) for persistence. If install fails with an MSRV error, run `rustup update stable` and confirm `rustc --version` is **1.88 or newer**.

## Clone + CLI (development)

```bash
git clone https://github.com/eddiethedean/strixonomy.git
cd strixonomy

# Sample fixture ontology (debug build is fine for smoke tests)
cargo run -- inspect fixtures
cargo run -- query fixtures "SELECT short_name, labels FROM classes"
cargo run -- validate fixtures
```

The `fixtures/` directory is included in the repository for examples and tests.

## `cargo install` (no clone)

```bash
cargo install strixonomy-cli --locked --version 0.27.0
```

**Version pinning:** Always pin an exact release in CI with `--version 0.27.0` (see [TAGGED_RELEASE](TAGGED_RELEASE)). Prefer `--locked` for reproducible crates.io installs — see [API stability](guides/api-stability.md) and [release integrity](release-integrity.md). For a longer macOS/Windows walkthrough, see [Install CLI](guides/install-cli.md).

Use **your own ontology directory** — there is no `fixtures/` folder outside a clone:

```bash
strixonomy inspect /path/to/your/ontologies
strixonomy query /path/to/your/ontologies "SELECT * FROM classes"
strixonomy validate /path/to/your/ontologies
```

## Release binaries (no Rust)

**CLI pre-builds are Linux x64 only.** On macOS or Windows, use `cargo install strixonomy-cli` or install the VS Code extension (bundled LSP). Full CI copy-paste: [CI integration](ci-integration.md).

1. Open [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) for the latest **v0.27.x** tag.
2. Download `strixonomy-v<version>-x86_64-unknown-linux-gnu.tar.gz`.
3. Verify with `SHA256SUMS` — see [release-integrity.md](release-integrity.md).
4. Extract and run (replace `0.27.0` with your tag):

```bash
VERSION=0.27.0
ASSET="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
BIN="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu"
tar xzf "${ASSET}"
chmod +x "${BIN}"
./"${BIN}" query /path/to/ontologies "SELECT * FROM classes"
./"${BIN}" validate /path/to/ontologies
```

> **Note:** The extracted binary is versioned (not plain `strixonomy`).

### Air-gapped / offline CLI

1. Download the Linux CLI tarball (and `SHA256SUMS` / `NOTICES`) from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) on a connected machine.
2. Transfer artifacts to the offline environment.
3. Extract and run the versioned binary as above — see [Enterprise deployment](guides/enterprise-deployment.md).

For offline **extension** install, see [vscode-install.md](vscode-install.md).

## Next steps

| Goal | Document |
|------|----------|
| What ships today | [SHIPPED.md](SHIPPED.md) |
| CI validation | [ci-integration.md](ci-integration.md) |
| SQL queries | [sql-reference.md](sql-reference.md) · [query cookbook](examples/queries.md) |
| SPARQL | [sparql-reference.md](sparql-reference.md) |
| DL Query | [examples/dl-query.md](examples/dl-query.md) |
| Plugins CLI | [examples/plugins.md](examples/plugins.md) |
| Patch / authoring | [patch-reference.md](patch-reference.md) |
| LSP integration | [lsp-api.md](lsp-api.md) |
| Troubleshooting | [troubleshooting.md](troubleshooting.md) · [faq.md](faq.md) |
