# Uninstall and cleanup

Remove Strixonomy from a machine or workspace without leaving stale caches.

## VS Code / Cursor extension

1. Extensions view → **Strixonomy** → **Uninstall** (or uninstall `strixonomy.strixonomy`).
2. If you previously used **OntoCode** (`ontocode.ontocode`), uninstall that listing too — [v0.27 migration](../migration/v0.27.md).
3. Reload the window if prompted.

Custom settings under `strixonomy.*` remain in user/workspace `settings.json` until you delete them.

## Workspace cache and config

Strixonomy may create local directories (safe to delete when you no longer use the IDE on that tree):

| Path | Purpose |
|------|---------|
| `.strixonomy/` | Cache, plugins, diagnostics, session (primary) |
| `.ontocore/` / `.ontocode/` | Legacy dual-read paths from pre–v0.27 |

Deleting these does **not** modify ontology source files (`.ttl`, `.obo`, `.owl`, …).

## CLI / Rust toolchain

```bash
# Optional: remove the cargo-installed CLI
cargo uninstall strixonomy-cli
# Legacy alias install, if present:
cargo uninstall ontocore-cli
```

Linux release tarballs are just extracted binaries — delete the directory you unpacked.

## See also

- [Support](../support.md)
- [Install](../install.md)
- [Product identity](product-identity.md)
