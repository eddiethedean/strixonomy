# Ecosystem Architecture

> **Canonical copy:** [docs/architecture.md](docs/architecture.md) (also on [Read the Docs](https://strixonomy.readthedocs.io/en/latest/architecture/)).
>
> Edit **`docs/architecture.md`** for content changes. This root file is a GitHub landing pointer so links from the repository root stay valid.

**Latest tagged: v0.27.0** — v0.27 ships today. Strixonomy (VS Code) + Strixonomy (CLI/LSP/library).

## Quick map

```text
Strixonomy (VS Code) ──strixonomy-lsp──► Strixonomy (Rust engine)
                                      ├── Ontologos (reasoning)
                                      └── Oxigraph / Horned-OWL
```

## Related

| Document | When |
|----------|------|
| [docs/architecture.md](docs/architecture.md) | Full ecosystem overview |
| [docs/design/ARCHITECTURE.md](docs/design/ARCHITECTURE.md) | Contributor crate layout |
| [docs/strixonomy/architecture.md](docs/strixonomy/architecture.md) | Short Strixonomy stack |
| [Platform overview (GitHub)](docs/platform/OVERVIEW.md) | OntoUI / WorkspaceStore implementers |
