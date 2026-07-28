# Versions and channels

**How to pick the right Strixonomy IDE / Strixonomy engine build.** Pin production and CI to a **tagged** release; do not follow `main` docs alone.

## Source of truth

| Source | What it means |
|--------|----------------|
| [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE) | Canonical public install version (today: **0.27.0**) |
| [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) | VSIX, Linux CLI tarball, multi-platform LSP, tutorial zip, checksums |
| [crates.io](https://crates.io/crates/strixonomy-cli) | Published Rust crates and `cargo install strixonomy-cli` |
| [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy) | Extension for VS Code (manual publish; usually matches the tag within hours) |
| [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) | Extension for Cursor / Open VSX clients |
| Read the Docs `latest` | Built from the default branch (`main`) — may describe work **after** the last tag |

## How to check which version you have

| Surface | How |
|---------|-----|
| **VS Code / Cursor extension** | Extensions view → **Strixonomy** → version under the title (or `strixonomy.strixonomy` in the extension details) |
| **CLI** | `strixonomy --version` (or `strixonomy -V`) |
| **Language server** | **Output → Strixonomy Language Server** often logs the binary path; or run the bundled `strixonomy-lsp` with `--version` if you know its path |

If Marketplace / Open VSX is behind GitHub, install the release VSIX for the tag you need — see below.

## Recommended installs (v0.27.0)

| Goal | Command / link |
|------|----------------|
| VS Code | Marketplace **or** download `strixonomy-v0.27.0.vsix` from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases/tag/v0.27.0) |
| Cursor | [Open VSX](https://open-vsx.org/extension/strixonomy/strixonomy) or the same VSIX |
| CLI (pinned) | `cargo install strixonomy-cli --locked --version 0.27.0` |
| CLI (Linux, no compile) | `strixonomy-v0.27.0-x86_64-unknown-linux-gnu.tar.gz` from GitHub Releases |
| Tutorial files (offline) | [`strixonomy-tutorial.zip`](https://github.com/eddiethedean/strixonomy/releases/download/v0.27.0/strixonomy-tutorial.zip) on the same GitHub Release (or curl samples in [First success](first-success.md)) |

Always pin: bare `cargo install strixonomy-cli` resolves to the **latest** crates.io version and can jump without your review.

## Air-gap artifact manifest

Exact filenames on [GitHub Release v0.27.0](https://github.com/eddiethedean/strixonomy/releases/tag/v0.27.0) (verify with `SHA256SUMS` on the same release):

| Artifact | Filename |
|----------|----------|
| VS Code / Cursor extension | `strixonomy-v0.27.0.vsix` |
| CLI (Linux x64 only) | `strixonomy-v0.27.0-x86_64-unknown-linux-gnu.tar.gz` |
| Checksums | `SHA256SUMS` |
| Offline tutorial samples | `strixonomy-tutorial.zip` |
| Prebuilt LSP (platform-specific) | `strixonomy-lsp-*` archives on the same release |

There is **no** Homebrew / winget / Scoop / Docker image for the CLI today — macOS/Windows CI agents use `cargo install strixonomy-cli` or the IDE-bundled LSP. See [Product identity](product-identity.md) and [Install](../install.md).

## When Marketplace lags GitHub

Marketplace publish is always **manual** after the release workflow finishes. **Open VSX** publishes automatically when `OVSX_PAT` is set (see [Marketplace publish](../marketplace-publish.md)). Either store can lag the GitHub tag by hours or longer.

### Playbook: store version ≠ latest tag

1. Check the [latest GitHub Release](https://github.com/eddiethedean/strixonomy/releases/latest) tag (example: `v0.27.0`).
2. In VS Code / Cursor: **Extensions → Strixonomy** — note the installed version.
3. If the store is older than the tag you need:
   - Download `strixonomy-v<version>.vsix` from that Release.
   - **Extensions → … → Install from VSIX…**
   - Reload the window.
4. Confirm the extension version matches the tag under the Extension details.
5. Keep CI pinned with `cargo install strixonomy-cli --locked --version <tag>` (or the Linux tarball) — do not wait for Marketplace for automation.

Re-check Marketplace / Open VSX before an org-wide rollout. Capability truth by version: [What ships today](../SHIPPED.md). Upgrades: [Migration guides](../migration/README.md).

Maintainer notes: [Marketplace publish](../marketplace-publish.md).
