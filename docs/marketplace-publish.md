# Publishing Strixonomy to the VS Code Marketplace

Strixonomy is on the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=strixonomy.strixonomy). This checklist is for maintainers publishing new versions.

!!! warning "CI does not publish to VS Code Marketplace"
    The [release workflow](https://github.com/eddiethedean/strixonomy/blob/main/.github/workflows/release.yml) publishes **Open VSX** automatically when `OVSX_PAT` is set. **VS Code Marketplace** requires a **manual** `vsce publish` step **after** the release tag and automated artifacts are complete.

## Release order

1. **Pre-tag:** version bump, CHANGELOG, doc sync ([releasing.md](releasing.md)), tests green on `main`
2. **Tag:** `git tag vX.Y.Z && git push origin vX.Y.Z` — triggers crates.io, GitHub Release, VSIX build, Open VSX
3. **Post-tag (manual):** publish to VS Code Marketplace (`vsce publish` below)
4. **Verify:** Marketplace and Open VSX badges, install smoke test

## Prerequisites

- [Visual Studio Marketplace publisher](https://marketplace.visualstudio.com/manage) account (`strixonomy` publisher id in `extension/package.json`)
- Personal Access Token with Marketplace **Manage** scope
- `vsce` installed (`npm i -g @vscode/vsce` or use project devDependency)
- Multi-platform release VSIX built by [release workflow on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/.github/workflows/release.yml)

## Pre-publish checks

1. Version bumped in `extension/package.json` and root `Cargo.toml` workspace version
2. [CHANGELOG.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/CHANGELOG.md) updated
3. [extension/README.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/extension/README.md) and [docs/vscode-install.md](vscode-install.md) mention Marketplace install and current version
4. Marketplace README images use absolute GitHub URLs (or paths that resolve with `repository.directory: "extension"`) — relative `media/...` links are rewritten against the **repo root** and 404
5. User docs synced per [releasing.md](releasing.md) checklist
6. `npm test` and `cargo test --workspace` pass
7. Short description avoids overstating Protégé parity — point to [Protégé migration guide](guides/protege-migration.md) instead

## Publish command

```bash
cd extension
npm ci && npm run compile
npx vsce publish --no-dependencies
```

For a one-off VSIX without publishing:

```bash
npx vsce package --no-dependencies
```

## After publish

1. Update root README **Choose your path → VS Code** to link Marketplace and Open VSX listings first, VSIX as fallback
2. Verify Open VSX and Marketplace badges on README and [docs/index.md](index.md) (Marketplace: `https://vsmarketplacebadges.dev/version/strixonomy.strixonomy.svg` — shields.io `visual-studio-marketplace` badges are retired)
3. Confirm the release tag and GitHub Release VSIX are already attached (tag **before** publish — do not tag after manual Marketplace publish)

## Token handling

Store `VSCE_PAT` or `AZURE_DEVOPS_EXT_PAT` in CI secrets only; never commit tokens.

## Open VSX (Cursor and other Open VSX clients)

From v0.11.3, the [release workflow on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/.github/workflows/release.yml) publishes the same VSIX to [Open VSX](https://open-vsx.org/) after packaging.

### Prerequisites

- [Open VSX publisher](https://open-vsx.org/user-settings/publisher) namespace `strixonomy` (claim before first release)
- Personal Access Token from Open VSX user settings
- Repository secret `OVSX_PAT` with publish scope

### Manual publish (emergency)

```bash
npm install -g ovsx
ovsx publish dist/strixonomy-v0.12.0.vsix -p "$OVSX_PAT"
```

### After Open VSX publish

1. Verify listing at [open-vsx.org/extension/strixonomy/strixonomy](https://open-vsx.org/extension/strixonomy/strixonomy) (badge: `https://img.shields.io/open-vsx/v/strixonomy/strixonomy`)
2. Confirm Cursor Extensions search finds **Strixonomy**
3. Document Cursor install path in [vscode-install.md](vscode-install.md) Option E
