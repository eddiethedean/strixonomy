# Platform and VS Code compatibility

Supported platforms and environments for Strixonomy **v0.27.0** (latest tagged). This page states **what is documented and tested in project CI** — not a formal certification.

Canonical matrix: [What ships today](../SHIPPED.md).

## VS Code extension

| Requirement | Documented value |
|-------------|------------------|
| Minimum VS Code | **1.85+** — [enterprise deployment](enterprise-deployment.md), [vscode-install](../vscode-install.md) |
| Maximum VS Code tested | **Not documented** — test your target VS Code version in pilot |
| Marketplace ID | `strixonomy.strixonomy` |
| Open VSX ID | `strixonomy.strixonomy` — [open-vsx.org/extension/strixonomy/strixonomy](https://open-vsx.org/extension/strixonomy/strixonomy) (Cursor, v0.11+) |
| Offline install | Release VSIX + SHA256 — [release integrity](../release-integrity.md) |
| Workspace trust | **Required** for custom `strixonomy.lspPath`; Restricted Mode uses bundled LSP |

Extension CI runs VS Code E2E on **1.85.0** and stable across Linux, macOS, Windows (see `.github/workflows/extension-vscode-e2e.yml`).

## Bundled language server (`strixonomy-lsp`)

Release VSIX bundles `strixonomy-lsp` for:

| OS | Architecture (documented) |
|----|-------------------------|
| Linux | x64, arm64 |
| macOS | Apple Silicon, Intel |
| Windows | x64 |

No separate LSP install required for standard Marketplace, Open VSX, or VSIX use.

## CLI release binaries

| Platform | Pre-built `strixonomy` CLI on GitHub Releases |
|----------|-----------------------------------------------|
| Linux x64 | **Yes** |
| Linux arm64 | **No** — use `cargo install` or VSIX-bundled LSP only |
| macOS | **No** — use `cargo install strixonomy-cli --locked` |
| Windows | **No** — use `cargo install` or CI on Linux runners |

Pin version: `VERSION=0.27.0` — [Install CLI & CI (detail)](../install-cli-ci.md).

## `cargo install` prerequisites

| Tool | Version |
|------|---------|
| Rust | **1.88+** |
| Node (extension build only) | **20** |

## Remote and containerized development

| Environment | Documented status |
|-------------|-------------------|
| Local desktop VS Code | **Supported** (primary) |
| Remote-SSH | **Pilot only** — install VSIX on remote host; confirm LSP arch matches remote OS — [enterprise deployment](enterprise-deployment.md) |
| Dev Containers | **Not certified** |
| GitHub Codespaces | **Not certified** |
| Web VS Code | **Not documented** |

Pilot checklist for Remote-SSH:

1. Install VSIX on the **remote** VS Code server
2. Confirm bundled `strixonomy-lsp` matches remote OS/architecture
3. Open ontology folder on remote filesystem
4. Re-run [First success](first-success.md)

## Multi-root workspaces

| Feature | Behavior |
|---------|----------|
| Multi-root VS Code workspace | **All folders indexed** (v0.10+); language server registers every root on open |
| Manual **Index Workspace** | May prompt to pick a folder when multiple roots are open |

Ensure each workspace root contains ontology files you expect in the explorer.

## Optional external tools

| Tool | When required |
|------|---------------|
| Java + ROBOT | `strixonomy robot` and LSP `runRobot` — [ROBOT interop](robot-interop.md) |
| Java | Not required for core Strixonomy/Strixonomy paths |

## Related

- [Install VS Code](../vscode-install.md)
- [Enterprise deployment](enterprise-deployment.md)
- [Troubleshooting](../troubleshooting.md)
