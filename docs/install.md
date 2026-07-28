# Install Strixonomy

**Canonical install page and hub.** Most users only need the **Strixonomy IDE** (VS Code/Cursor extension). The **Strixonomy engine** CLI is optional (CI / scripting).

!!! danger "CLI package name"
    Install the CLI with **`cargo install strixonomy-cli`** (binary name: `strixonomy`).
    **`cargo install strixonomy`** installs the **library** crate, not the CLI.

!!! info "Public install pin"
    Pin to **`0.28.0`** ([TAGGED_RELEASE](TAGGED_RELEASE)). GitHub `main` may be **0.28.0** in development — see [Versions & channels](guides/versions-and-channels.md).

## Which path do I need?

```mermaid
flowchart TD
  start[What do you want to do?]
  start --> edit{Edit ontologies in VS Code?}
  edit -->|Yes| ext[Install Strixonomy extension]
  ext --> fs[First success tutorial]
  edit -->|No| ci{Validate or classify in CI?}
  ci -->|Linux x64| tarball[Download release tarball]
  ci -->|macOS / Windows / other| cargo["cargo install strixonomy-cli --version 0.28.0"]
  ci -->|No| rust{Embed in Rust app?}
  rust -->|Yes| lib["strixonomy = \"0.27\" in Cargo.toml"]
  rust -->|No| lsp[Custom editor on strixonomy-lsp]
  tarball --> ciDoc[CI integration guide]
  cargo --> cliDoc[Install CLI detail]
  lib --> rustDoc[Rust library guide]
  lsp --> lspDoc[LSP API]
```

| Goal | Start here |
|------|------------|
| **Not sure which artifact** | [Which artifact?](guides/which-artifact.md) |
| Edit in VS Code / Cursor | [Extension install](#1-vs-code-cursor-extension-recommended) → [First success](guides/first-success.md) |
| CI / automation (Linux x64) | [Release tarball](#linux-x64-release-tarball-ci-preferred) → [CI integration](ci-integration.md) |
| CLI on macOS / Windows | [cargo install](#cargo-install-macos--windows--any-platform) → [Install CLI walkthrough](guides/install-cli.md) |
| Rust library | [Rust library guide](guides/rust-library.md) |
| Product names (IDE vs engine) | [Product identity](guides/product-identity.md) |

!!! tip "Most IDE users never need Rust"
    The Strixonomy IDE bundles `strixonomy-lsp`. Install the extension and skip the CLI unless you need `strixonomy` for CI, scripting, or validation outside the editor.

## 1. VS Code / Cursor extension (recommended)

| Method | Platforms | Needs Rust? |
|--------|-----------|-------------|
| [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) / [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) | Linux, macOS, Windows | No |
| Release VSIX (`strixonomy-v0.28.0.vsix`) | Same | No |

1. Install **Strixonomy** (`strixonomy.strixonomy`).
2. If the store lags the latest GitHub tag, install the VSIX — [Marketplace lag playbook](guides/versions-and-channels.md#when-marketplace-lags-github).
3. Open a folder of `.ttl` / `.obo` / `.owl` / `.rdf` / `.owx` files.
4. Open the **Strixonomy** activity bar.

!!! tip "Restricted Mode works out of the box"
    The **bundled** language server indexes ontologies without trusting the workspace. **Trust the folder** only if you set custom `strixonomy.lspPath` or `strixonomy.robotPath` — see [VS Code install details](vscode-install.md).

**Next:** [First success (~10 min)](guides/first-success.md).

## 2. Optional CLI

| Method | Linux x64 | macOS | Windows | Needs Rust? |
|--------|-----------|-------|---------|-------------|
| `cargo install strixonomy-cli --locked --version 0.28.0` | Yes | Yes | Yes | Yes (1.88+) |
| Release CLI tarball | Yes | No | No | No |
| Git clone + `cargo run --` | Yes | Yes | Yes | Yes (1.88+) |

!!! note "Cold compile"
    First `cargo install` often takes **15–30+ minutes**. Prefer the Linux x64 tarball for CI — [CI integration](ci-integration.md).

### cargo install (macOS / Windows / any platform)

Prerequisites: Rust **1.88+**; Windows needs [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/); macOS needs Xcode CLT (`xcode-select --install`).

```bash
cargo install strixonomy-cli --locked --version 0.28.0
export PATH="$HOME/.cargo/bin:$PATH"
strixonomy validate /path/to/your/ontologies
```

Longer macOS/Windows walkthrough: [Install CLI](guides/install-cli.md).

### Linux x64 release tarball (CI preferred)

See [CI integration](ci-integration.md) — download `strixonomy-v0.28.0-x86_64-unknown-linux-gnu.tar.gz`, verify `SHA256SUMS`, run `validate` / `classify`.

### From a git clone

```bash
git clone https://github.com/eddiethedean/strixonomy.git
cd strixonomy
cargo run -- validate fixtures
```

The `fixtures/` directory exists **only in a clone**. After [First success](guides/first-success.md), prefer a tutorial folder — [Examples index](examples/index.md).

## 3. What you can edit

Write-back: **`.ttl`**, **`.obo`**, **`.owl`/`.rdf`**, **`.owx`**. XML is semantic re-serialize (not Protégé byte-identical). JSON-LD / N-Triples / TriG stay read-only — [Supported formats](supported-formats.md) · [Capabilities by format](guides/capabilities-by-format.md).

## Detail pages (appendices)

| Topic | Doc |
|-------|-----|
| Full CLI / CI install matrix | [Install CLI & CI (detail)](install-cli-ci.md) |
| VS Code options (offline, Restricted Mode, VSIX) | [vscode-install.md](vscode-install.md) |
| Extension settings reference | [VS Code settings](ide/vscode-settings.md) |
| Release integrity | [release-integrity.md](release-integrity.md) |
| Platform support | [platform-compatibility.md](guides/platform-compatibility.md) |
| Air-gap artifact filenames | [Versions & channels](guides/versions-and-channels.md#air-gap-artifact-manifest) |
