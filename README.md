# Strixonomy

**Strixonomy** edits OWL, RDF, and OBO ontologies in VS Code—browse entities, change labels and axioms, query, reason, and validate—without leaving Git.

**Install the extension → [First success (~10 min)](https://strixonomy.readthedocs.io/en/latest/guides/first-success/)** (no Rust, no clone).

**Current release: v0.27.0** · [What ships today](https://strixonomy.readthedocs.io/en/latest/SHIPPED/) · [Known limitations](https://strixonomy.readthedocs.io/en/latest/known-limitations/) · [Changelog](CHANGELOG.md) · [Docs](https://strixonomy.readthedocs.io/en/latest/)

[![CI](https://github.com/eddiethedean/strixonomy/actions/workflows/ci.yml/badge.svg)](https://github.com/eddiethedean/strixonomy/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue)](https://github.com/eddiethedean/strixonomy/blob/main/LICENSE-MIT)
[![Docs](https://readthedocs.org/projects/strixonomy/badge/?version=latest)](https://strixonomy.readthedocs.io/en/latest/)
[![VS Code Marketplace](https://badgen.net/vs-marketplace/v/strixonomy.strixonomy?label=VS%20Code%20Marketplace)](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy)
[![Open VSX](https://img.shields.io/open-vsx/v/strixonomy/strixonomy)](https://open-vsx.org/extension/strixonomy/strixonomy)
[![crates.io](https://img.shields.io/crates/v/strixonomy?logo=rust)](https://crates.io/crates/strixonomy)

![Strixonomy product tour](docs/assets/screenshots/product-tour.gif)

## Start here

| I want to… | Start here |
|------------|------------|
| **Edit ontologies in VS Code** | [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) → **[First success (~10 min)](https://strixonomy.readthedocs.io/en/latest/guides/first-success/)** |
| **CI / automation only** | **Linux x64:** release tarball → [CI guide](https://strixonomy.readthedocs.io/en/latest/ci-integration/). **macOS/Windows:** [Install CLI](https://strixonomy.readthedocs.io/en/latest/guides/install-cli/) (`cargo install` 15–30+ min — not needed for the IDE) |
| Decide if it fits | [Known limitations](https://strixonomy.readthedocs.io/en/latest/known-limitations/) · [What ships today](https://strixonomy.readthedocs.io/en/latest/SHIPPED/) · [Evaluate pack](https://strixonomy.readthedocs.io/en/latest/guides/enterprise-eval/) · [Procurement appendix](https://strixonomy.readthedocs.io/en/latest/guides/procurement-appendix/) |
| Try examples | [Examples](https://strixonomy.readthedocs.io/en/latest/examples/) · repo [`examples/`](examples/) |
| Embed in Rust | [Rust library guide](https://strixonomy.readthedocs.io/en/latest/guides/rust-library/) |
| Contribute | [CONTRIBUTING.md](CONTRIBUTING.md) |
| Support | [Support](https://strixonomy.readthedocs.io/en/latest/support/) |

Full documentation: **[Read the Docs](https://strixonomy.readthedocs.io/en/latest/)**. You do not need to clone this repo to use the extension or installed CLI.

<details>
<summary>Names in 10 seconds</summary>

| Name | What it is |
|------|------------|
| **Strixonomy IDE** | VS Code / Cursor extension |
| **Strixonomy engine** | Rust CLI (`strixonomy`), LSP, crates — install CLI with **`cargo install strixonomy-cli`** (not `strixonomy`) |
| **Ontologos** | Bundled reasoner (not a separate install) |

Details: [Product identity](https://strixonomy.readthedocs.io/en/latest/guides/product-identity/). Writable formats and limits: [Supported formats](https://strixonomy.readthedocs.io/en/latest/supported-formats/) · [Known limitations](https://strixonomy.readthedocs.io/en/latest/known-limitations/).
</details>

## See it in action

[Feature tour](https://strixonomy.readthedocs.io/en/latest/ide/feature-tour/) · [First success](https://strixonomy.readthedocs.io/en/latest/guides/first-success/) (~10 min, no clone)

<p>
<img src="docs/assets/screenshots/explorer-inspector.png" alt="Explorer and Entity Inspector" width="48%" />
<img src="docs/assets/screenshots/query-workbench.png" alt="Query Workbench" width="48%" />
</p>

## Install

| Install | Command / link |
|---------|----------------|
| **VS Code extension** | [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy), [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) (Cursor), or [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) `strixonomy-v0.27.0.vsix` |
| **CLI (Linux x64)** | Release tarball — full steps: [CI integration](https://strixonomy.readthedocs.io/en/latest/ci-integration/) |
| **CLI (macOS/Windows)** | `cargo install strixonomy-cli --locked --version 0.27.0` (Rust 1.88+; 15–30+ min cold) — [Install CLI](https://strixonomy.readthedocs.io/en/latest/guides/install-cli/) |
| **Crates** | [`strixonomy`](https://crates.io/crates/strixonomy) on [crates.io](https://crates.io/search?q=strixonomy) |

Release CLI tarballs are **Linux x64 only**. Most IDE users never need the CLI — the extension bundles `strixonomy-lsp`.

> **Writable formats:** `.ttl`, `.obo`, `.owl`/`.rdf`, `.owx` (XML = semantic re-serialize). JSON-LD / N-Triples / TriG are read-only — [Supported formats](https://strixonomy.readthedocs.io/en/latest/supported-formats/). Evaluators: [SHIPPED](https://strixonomy.readthedocs.io/en/latest/SHIPPED/) is capability truth; `docs/protege-parity/` is engineering notes only.

## Quick start

**VS Code:** Install [Strixonomy](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) → open a folder of **`.ttl` / `.obo` / `.owl` / `.rdf` / `.owx`** (editable) or JSON-LD / TriG / N-Triples (browse/query only) → click the **Strixonomy** activity bar. Edit in the Entity Inspector. XML write-back is semantic re-serialize — see [OWL/XML and RDF/XML write-back](https://strixonomy.readthedocs.io/en/latest/guides/owl-xml-workflow/) and [Supported formats](https://strixonomy.readthedocs.io/en/latest/supported-formats/).

> **Workspace Trust:** The **bundled** language server works in Restricted Mode. **Do not Trust the workspace** unless you configured `strixonomy.lspPath` or `strixonomy.robotPath`.

**CI / automation only (not required for the IDE):**

- **Linux x64:** [CI integration](https://strixonomy.readthedocs.io/en/latest/ci-integration/) — download tarball, verify checksums, run the versioned binary.
- **macOS/Windows or from source:** [Install CLI](https://strixonomy.readthedocs.io/en/latest/guides/install-cli/). Cold `cargo install` may take **15–30+ minutes** (Rust 1.88+).

```bash
cargo install strixonomy-cli --locked --version 0.27.0
strixonomy validate /path/to/ontologies
```

**From a clone (same smoke command everywhere):**

```bash
git clone https://github.com/eddiethedean/strixonomy.git && cd strixonomy
cargo run -- validate fixtures
```

## Architecture

```text
┌──────────────────────────────────────────────────────────────┐
│  Strixonomy (VS Code) ──strixonomy-lsp──► Strixonomy (Rust engine) │
│  index · query · diagnostics · refactor · diff · CLI · LSP   │
└──────────────┬─────────────────────────────┬───────────────────┘
               ▼                             ▼
        ┌─────────────┐              ┌──────────────────┐
        │  Ontologos  │              │  Oxigraph /      │
        │  reasoning  │              │  Horned-OWL      │
        └─────────────┘              └──────────────────┘
```

Platform docs: [Vision](https://strixonomy.readthedocs.io/en/latest/vision/) · [Architecture](ARCHITECTURE.md) · [Roadmap hub](https://strixonomy.readthedocs.io/en/latest/roadmap-hub/) · [Protégé vs Strixonomy](https://strixonomy.readthedocs.io/en/latest/guides/protege-decision/)

**v0.27.0** renames OntoCore / OntoCode to **Strixonomy** (compat window for legacy bins, LSP methods, and paths). Protégé-aligned oracles, annotation linkification, `catalog-v001.xml` redirects, and IdPolicy shipped in **v0.26.x**. **Not a Protégé replacement** — see [Known limitations](https://strixonomy.readthedocs.io/en/latest/known-limitations/). Engineering detail: [SHIPPED](https://strixonomy.readthedocs.io/en/latest/SHIPPED/), [v0.27 migration](docs/migration/v0.27.md).

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md). Quick checks:

```bash
cargo test --workspace
cargo build -p strixonomy-lsp --bins
cd extension && npm ci && STRIXONOMY_LSP_BIN=../target/debug/strixonomy-lsp npm test
cd extension/webview-ui && npm ci && npm test
cargo fmt --all && cargo clippy --workspace --all-targets --all-features -- -D warnings
```

**Full CI parity:** `./scripts/run-ci-local.sh`

## License

MIT OR Apache-2.0. Third-party licenses: [LICENSES](https://strixonomy.readthedocs.io/en/latest/design/LICENSES/). Security: [security policy](https://strixonomy.readthedocs.io/en/latest/security/).
