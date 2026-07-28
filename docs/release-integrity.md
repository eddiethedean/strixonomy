# Release integrity

How to verify Strixonomy IDE / Strixonomy engine release artifacts from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases).

## Checksums

Each release includes `SHA256SUMS` with SHA-256 hashes of:

- `strixonomy-v<version>-x86_64-unknown-linux-gnu.tar.gz` (CLI binary, Linux x64)
- `strixonomy-lsp-v<version>-<platform>.tar.gz` / `.zip` (per-platform LSP)
- `strixonomy-v<version>.vsix` (VS Code extension; example: `strixonomy-v0.27.0.vsix`)
- `NOTICES` (third-party license summary)

Verify after download:

```bash
shasum -a 256 -c SHA256SUMS
```

On Linux you may use `sha256sum -c SHA256SUMS` instead.

## Worked example (Linux x64 CLI)

```bash
VERSION=0.27.0   # replace with the release tag you are verifying
curl -fsSLO "https://github.com/eddiethedean/strixonomy/releases/download/v${VERSION}/SHA256SUMS"
curl -fsSLO "https://github.com/eddiethedean/strixonomy/releases/download/v${VERSION}/strixonomy-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
sha256sum -c SHA256SUMS
tar xzf "strixonomy-v${VERSION}-x86_64-unknown-linux-gnu.tar.gz"
chmod +x "strixonomy-v${VERSION}-x86_64-unknown-linux-gnu"
./strixonomy-v${VERSION}-x86_64-unknown-linux-gnu --version
```

## crates.io

Rust crates are published from CI using a restricted `CARGO_REGISTRY_TOKEN`. Install with:

```bash
cargo install strixonomy-cli --locked
```

Prefer `--locked` so dependency versions match the published crate.

## VS Code extension

1. Download the `.vsix` from the release matching your platform (multi-platform VSIX bundles `strixonomy-lsp`).
2. Verify against `SHA256SUMS`.
3. Install via **Extensions → Install from VSIX…**

`strixonomy.lspPath` is a **trusted-admin** setting. In VS Code **Restricted Mode** (untrusted workspace), the extension ignores workspace `lspPath` and uses the bundled server.

## Dependency auditing

CI runs `cargo audit` on the Rust workspace (see [`.cargo/audit.toml`](https://github.com/eddiethedean/strixonomy/blob/main/.cargo/audit.toml) for documented ignores of transitive advisories that cannot be upgraded in-tree). Report vulnerable dependencies via [security.md](security.md).

## Future: signed artifacts

Code signing and Sigstore attestations for release binaries are planned. Until then, use checksums and install from the official GitHub Releases page only.
