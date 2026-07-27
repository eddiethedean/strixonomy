# Install the Strixonomy CLI (macOS and Windows)

> **Canonical install:** [Install](../install.md). **Most users never need this page.** The [Strixonomy VS Code / Cursor extension](../vscode-install.md) bundles `strixonomy-lsp`. Install the extension for IDE work and skip compiling the CLI unless you need `strixonomy` for CI, scripting, or local validation outside the editor.

Release CLI tarballs are **Linux x64 only** (`x86_64-unknown-linux-gnu`). On **macOS** and **Windows**, install from crates.io with Rust. On **Linux arm64** (and other non-x64 targets), there is no tarball — use `cargo install` below or the LSP bundled in the VSIX ([platform compatibility](platform-compatibility.md)).

Canonical pin: **`0.27.0`** ([TAGGED_RELEASE](../TAGGED_RELEASE)). See also [Install CLI & CI (detail)](../install-cli-ci.md) and [CI integration](../ci-integration.md) (Linux CI prefers the release tarball).

## Prerequisites

| Requirement | macOS | Windows |
|-------------|-------|---------|
| Rust toolchain | **1.88+** via [rustup](https://rustup.rs/) | **1.88+** via rustup |
| System tools | Xcode Command Line Tools: `xcode-select --install` | [MSVC Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (C++ workload) |
| Disk / time | Expect **15–30+ minutes** cold compile and several GB for cargo cache | Same |

### Apple Silicon notes

- Install the **aarch64** Rust toolchain via rustup on Apple Silicon Macs (default on current macOS).
- Ensure `~/.cargo/bin` is on your `PATH` (rustup installer usually configures this).

### Verify tools

```bash
rustc --version   # 1.88 or newer
cargo --version
```

macOS:

```bash
xcode-select -p   # should print a Developer path
```

## Install

```bash
cargo install strixonomy-cli --locked --version 0.27.0
```

Confirm:

```bash
strixonomy --help
which strixonomy   # typically ~/.cargo/bin/strixonomy
```

!!! warning "Wrong crate name"
    The GitHub repo is `strixonomy`. The crate and CLI are **`strixonomy-cli`** / **`strixonomy`**. Do not run `cargo install strixonomy`.

## First commands

Use **your** ontology directory (there is no `fixtures/` outside a git clone):

```bash
strixonomy validate /path/to/your/ontologies
strixonomy query /path/to/your/ontologies "SELECT * FROM classes"
```

## When to use the extension instead

| Goal | Prefer |
|------|--------|
| Browse / edit in the IDE | [VS Code extension](../vscode-install.md) |
| Local CI-like validate/query without VS Code | This CLI install |
| Linux CI in GitHub Actions | [Release tarball](../ci-integration.md) (faster than compile) |
| Custom `strixonomy.lspPath` without compiling | Prebuilt **`strixonomy-lsp-*`** archives on [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases/tag/v0.27.0) (macOS, Windows, Linux x64/arm64) — set `strixonomy.lspPath` to the extracted binary (trusted workspace) |

## Optional: prebuilt language server (skip `cargo install` for LSP)

Release assets include platform-specific `strixonomy-lsp` binaries (for example `strixonomy-lsp-*-apple-darwin.tar.gz`, `strixonomy-lsp-*-pc-windows-msvc.zip`, Linux gnu/musl and arm64 variants). Download, verify against `SHA256SUMS`, extract, and point `strixonomy.lspPath` at the binary. Most users should keep the **bundled** LSP from the VSIX and skip this.

## Troubleshooting

| Problem | What to try |
|---------|-------------|
| `strixonomy: command not found` | Add `~/.cargo/bin` to `PATH`; open a new terminal |
| Compile errors on Windows | Install MSVC Build Tools with the C++ desktop workload |
| Compile errors on macOS | Re-run `xcode-select --install`; update Xcode CLT |
| Wrong version after install | Re-run with `--version 0.27.0 --locked`; check `strixonomy --version` |
| Slow installs | Expected on first cold build; subsequent upgrades are faster |

## Related

- [Install CLI & CI (detail)](../install-cli-ci.md)
- [Which artifact?](which-artifact.md)
- [Release integrity](../release-integrity.md)
- [Versions and channels](versions-and-channels.md)
