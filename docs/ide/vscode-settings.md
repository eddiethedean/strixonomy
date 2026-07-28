# VS Code extension settings

All settings live under the **Strixonomy** section in VS Code **Settings** (or `settings.json`). Extension id: `strixonomy.strixonomy`.

!!! tip "Restricted Mode works out of the box"
    The bundled language server runs without trusting the workspace. **Trust the folder** only when you set `strixonomy.lspPath` or `strixonomy.robotPath` — those paths are ignored in Restricted Mode.

## Settings reference

| Setting | Type | Default | Description |
|---------|------|---------|-------------|
| `strixonomy.lspPath` | string | `""` | **Trusted workspaces only:** absolute path to a custom `strixonomy-lsp` binary. When empty, the extension uses the bundled server or `PATH`. Ignored in Restricted Mode. |
| `strixonomy.robotPath` | string | `""` | **Trusted workspaces only:** path to the ROBOT CLI or JAR. When empty, ROBOT is resolved from `PATH`. Ignored in Restricted Mode. |
| `strixonomy.autoIndexOnOpen` | boolean | `true` | **Legacy no-op.** Indexing is driven by the language server on workspace open; kept for compatibility with older settings. |
| `strixonomy.queryHistoryLimit` | number | `20` | Maximum recent queries kept in the Query Workbench history. |
| `strixonomy.reasoner.default` | enum | `"el"` | Default profile for **Strixonomy: Run Reasoner**: `el`, `rl`, `rdfs`, `dl`, or `auto`. |
| `strixonomy.reasoner.autoProfile` | boolean | `true` | Emit profile-detection warnings when running the reasoner. |
| `strixonomy.hierarchy.mode` | enum | `"asserted"` | Explorer class hierarchy after classification: `asserted`, `inferred`, or `combined`. |
| `strixonomy.indexCache` | boolean | `false` | Persist parse snapshots under `.strixonomy/cache/` (add to `.gitignore` if enabled). |
| `strixonomy.diagnostics.rules` | object | `{}` | Per-rule overrides (`enabled`, `severity`). Mirrors `.strixonomy/diagnostics.toml`; file config takes precedence when present. |

## Example `settings.json`

```json
{
  "strixonomy.reasoner.default": "auto",
  "strixonomy.hierarchy.mode": "combined",
  "strixonomy.queryHistoryLimit": 50
}
```

Custom LSP (development builds — **trusted workspace required**):

```json
{
  "strixonomy.lspPath": "/absolute/path/to/strixonomy-lsp"
}
```

## Workspace vs user scope

| Setting | Typical scope |
|---------|----------------|
| `lspPath`, `robotPath` | User or workspace (trusted folders only) |
| Reasoner defaults, hierarchy mode | User or workspace |
| `diagnostics.rules` | Workspace (per ontology project) |
| `indexCache` | Workspace |

## Related

- [Install VS Code](../vscode-install.md) · [First success](../guides/first-success.md)
- [Reasoner guide](../guides/reasoner.md) · [ROBOT interop](../guides/robot-interop.md)
- [Configurable diagnostics](../supported-formats.md) · [Troubleshooting](../troubleshooting.md)
