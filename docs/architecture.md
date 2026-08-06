# Ecosystem Architecture

> **Audience:** Evaluators, adopters, and new contributors — **canonical user-facing architecture**.
>
> **Capability truth:** [What ships today](SHIPPED.md). Contributor crate layout: [Implementation architecture](design/ARCHITECTURE.md). Short stack: [Strixonomy architecture](strixonomy/architecture.md).

**Latest tagged: v0.28.1** — v0.27 ships today. **v0.28 in progress** on `main` (compat shim removal). **Strixonomy IDE** (VS Code) + **Strixonomy engine** (CLI / LSP / library) + **Ontologos** (reasoning).

## Shipped today

```text
Strixonomy IDE (VS Code) ──strixonomy-lsp──► Strixonomy engine
                                              ├── Ontologos (EL / RL / RDFS / DL)
                                              └── Oxigraph / Horned-OWL
Applications that ship today: VS Code extension · CLI · GitHub Actions (via CLI)
Plugin SDK 1.0 wire (TOML + subprocess) ships; manager → v0.33; official registry → v0.34; marketplace → v0.35+
```

| Layer | Role |
|-------|------|
| **Strixonomy IDE** | Explorer, inspector, Query Workbench, graphs, reasoner UI, plugins |
| **Strixonomy engine** | Index, query, diagnostics, patch, refactor, diff, LSP, CLI |
| **Ontologos** | Classification, consistency, explanations |

Writable formats and limits: [Supported formats](supported-formats.md) · [Known limitations](known-limitations.md).

## Future (not shipped)

Curated plugin marketplace, production owlmake hardening, language SDKs, MCP server, OntoStudio desktop, AI-native surfaces — see [Platform roadmap](roadmap.md). Do **not** treat these as product claims.

Engineering-only trees (`docs/design/`, `docs/platform/`, `docs/ui/`) are targets or implementer notes — not the capability matrix.

!!! note "Plugin platform"
    Plugin **SDK 1.0** freezes the TOML + subprocess JSON wire (`api_version = "1"`) — safe to author against today. Manager/owlmake, official registry, and public marketplace are planned for **v0.33**, **v0.34**, and **v0.35+** respectively — [Plugin policy](guides/plugin-policy.md) · [Plugin authoring](guides/plugins.md).

## Responsibilities

### Ontologos

Reasoning algorithms and semantic inference. The Strixonomy engine delegates classification, consistency, and explanations to Ontologos — it does not embed a separate reasoner.

### Strixonomy engine

Reusable semantic workspace platform: index, query, diagnostics, refactoring, and semantic diff. Consumed by the VS Code IDE, CLI, and Rust library.

**Plugin platform status:**

- **Shipped (SDK 1.0):** frozen wire contract — workspace manifest discovery, reference plugins, CLI/LSP hooks, subprocess workflow runner, UI views/commands/preferences/context actions, lifecycle (`depends_on` / `activation`), provider actions (see [Plugin authoring](guides/plugins.md)).
- **v0.33 targets:** curated plugin discovery and a maintained EBISPOT/owlmake adapter.

The engine is **not** a workflow engine; build, release, and QC automation should live in external tools and workflow plugins rather than becoming core engine dependencies.

### External workflow plugins (e.g. owlmake)

**SDK 1.0 wire ships today; production workflow integration is planned for v0.33.** [EBISPOT/owlmake](https://github.com/EBISPOT/owlmake) is the target for the maintained reference adapter—ROBOT/ODK-style pipelines without becoming a core Strixonomy dependency. Today, Strixonomy ships the ROBOT CLI wrapper and a generic subprocess workflow scaffold, not the owlmake adapter.

### Strixonomy IDE

Reference IDE on top of the Strixonomy engine. Presents editing, reasoning, and diagnostics in VS Code. Plugin views, commands, preferences, and context actions ship today (SDK 1.0); production workflow automation is planned for **v0.33**.

## Design Philosophy

Ontologos thinks.

Strixonomy engine understands.

Strixonomy IDE presents.

Workflow plugins automate.

## Further reading

| Document | When |
|----------|------|
| [Implementation architecture](design/ARCHITECTURE.md) | Crate layout (contributors) |
| [Platform overview (GitHub)](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md) | OntoUI / WorkspaceStore implementers |
| [Plugin authoring](guides/plugins.md) | Workspace manifests and reference plugins |
