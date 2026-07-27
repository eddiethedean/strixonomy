# Install Strixonomy and Strixonomy

**Canonical install page.** Extension first (most users). CLI and CI are optional.

| Goal | Go here |
|------|---------|
| Edit ontologies in VS Code / Cursor | [Install the extension](#1-vs-code-cursor-extension-recommended) → [First success (~10 min)](guides/first-success.md) |
| Optional CLI on macOS / Windows | [Install CLI (cargo)](#2-optional-cli) · detail: [macOS/Windows walkthrough](guides/install-cli.md) |
| CI validate on Linux x64 | [CI with release tarball](ci-integration.md) |
| Which binary / crate do I need? | [Which artifact?](guides/which-artifact.md) |

!!! tip "Most IDE users never need Rust"
    The Strixonomy extension bundles `strixonomy-lsp`. Install the extension and skip the CLI unless you need `strixonomy` for CI, scripting, or validation outside the editor.

!!! tip "Docs vs Marketplace"
    Read the Docs `latest` may describe work **after** the last tag. Pins follow [`docs/TAGGED_RELEASE`](TAGGED_RELEASE) (**0.27.0**). See [Versions & channels](guides/versions-and-channels.md).

!!! warning "Wrong crate name"
    The GitHub repo is `strixonomy`. The CLI crate is **`strixonomy-cli`** (`strixonomy` binary). Do **not** run `cargo install strixonomy`.

Canonical pin: **`0.27.0`** ([TAGGED_RELEASE](TAGGED_RELEASE)).

## 1. VS Code / Cursor extension (recommended)

| Method | Platforms | Needs Rust? |
|--------|-----------|-------------|
| [Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) / [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) | Linux, macOS, Windows | No |
| Release VSIX (`strixonomy-v0.27.0.vsix`) | Same | No |

1. Install **Strixonomy** (`strixonomy.strixonomy`).
2. If the store lags the latest GitHub tag, install the VSIX — [Marketplace lag playbook](guides/versions-and-channels.md#when-marketplace-lags-github).
3. Open a folder of `.ttl` / `.obo` / `.owl` / `.rdf` / `.owx` files.
4. Open the **Strixonomy** activity bar.

Full steps (Trust, offline, custom `lspPath`): [VS Code install details](vscode-install.md).

**Next:** [First success (~10 min)](guides/first-success.md).

## 2. Optional CLI

| Method | Linux x64 | macOS | Windows | Needs Rust? |
|--------|-----------|-------|---------|-------------|
| `cargo install strixonomy-cli --locked --version 0.27.0` | Yes | Yes | Yes | Yes (1.88+) |
| Release CLI tarball | Yes | No | No | No |
| Git clone + `cargo run --` | Yes | Yes | Yes | Yes (1.88+) |

!!! note "Cold compile"
    First `cargo install` often takes **15–30+ minutes**. Prefer the Linux x64 tarball for CI — [CI integration](ci-integration.md).

### cargo install (macOS / Windows / any platform)

Prerequisites: Rust **1.88+**; Windows needs [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/); macOS needs Xcode CLT (`xcode-select --install`).

```bash
cargo install strixonomy-cli --locked --version 0.27.0
export PATH="$HOME/.cargo/bin:$PATH"
strixonomy validate /path/to/your/ontologies
```

Longer macOS/Windows walkthrough: [Install CLI](guides/install-cli.md).

### Linux x64 release tarball (CI preferred)

See [CI integration](ci-integration.md) — download `strixonomy-v0.27.0-x86_64-unknown-linux-gnu.tar.gz`, verify `SHA256SUMS`, run `validate` / `classify`.

### From a clone

```bash
git clone https://github.com/eddiethedean/strixonomy.git
cd strixonomy
cargo run -- validate fixtures
```

## 3. What you can edit

Write-back: **`.ttl`**, **`.obo`**, **`.owl`/`.rdf`**, **`.owx`**. XML is semantic re-serialize (not Protégé byte-identical). JSON-LD / N-Triples / TriG stay read-only — [Supported formats](supported-formats.md) · [Capabilities by format](guides/capabilities-by-format.md).

## Related

| Topic | Doc |
|-------|-----|
| Full CLI / CI install matrix (all paths) | [Install CLI & CI (detail)](install-cli-ci.md) |
| VS Code options (offline, Restricted Mode) | [vscode-install.md](vscode-install.md) |
| Release integrity | [release-integrity.md](release-integrity.md) |
| Platform support | [platform-compatibility.md](guides/platform-compatibility.md) |
