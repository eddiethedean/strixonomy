# Ecosystem Architecture

> **Audience:** Evaluators, adopters, and new contributors — **canonical user-facing architecture**.
>
> **Which architecture doc?**
>
> | Read this | When |
> |-----------|------|
> | **This page** (`architecture.md`) | Product/ecosystem overview — Ontologos, Strixonomy, Strixonomy |
> | [Implementation architecture](design/ARCHITECTURE.md) | Contributor crate layout and internal modules |
> | [Product design / UI platform](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/PLATFORM_ARCHITECTURE.md) | Shared **OntoUI**, OntoStudio target, design system |
> | [Platform architecture (implementation)](https://github.com/eddiethedean/strixonomy/blob/main/docs/platform/OVERVIEW.md) | OntoUI, WorkspaceStore, plugin host — **shipped v0.13–v0.17** |
> | [Plugin authoring](guides/plugins.md) | Workspace manifests, reference plugins, subprocess workflows (v0.17) |
> | [Strixonomy architecture](strixonomy/architecture.md) | Short Strixonomy stack summary (links here for detail) |
>
> **Contributor crate layout:** [Implementation architecture](design/ARCHITECTURE.md) (internal modules only).
>
> **Latest tagged: v0.26.2** — v0.26 ships today under OntoCode/OntoCore names.
>
> **v0.27 in progress (unreleased):** renames the platform to Strixonomy.
>
> **Planned v1.0:** curated plugin marketplace, production owlmake integration, language SDKs, MCP server. Plugin **SDK 1.0** wire is frozen today — [Plugin policy](guides/plugin-policy.md). See [Platform roadmap](roadmap.md) ([full ROADMAP.md on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/ROADMAP.md)).
>
> **Planned post-1.0:** OntoStudio desktop, AI-native workflows — [UI roadmap mapping](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ROADMAP_MAPPING.md).
>
> **Implementers only:** `docs/design/`, `docs/platform/`, and `docs/ui/` architecture specs are engineering targets — not the product capability matrix. Use [What ships today](SHIPPED.md) for adoption decisions.

```
External Workflow Plugins (SDK 1.0)  ← TOML + subprocess plugins; api_version = "1"
├── owlmake (reference design; production hardening → product 1.0)
├── ROBOT / ODK workflow adapters
└── Future build, validation, doc plugins
          │
          ▼
Applications
├── Strixonomy (VS Code)             ← ships today
├── OntoStudio (desktop)           ← planned post v1.0 ([UI spec](https://github.com/eddiethedean/strixonomy/blob/main/docs/ui/ONTOSTUDIO_DESKTOP.md))
├── CLI                            ← ships today
├── GitHub Actions (via CLI)       ← ships today
├── Python / TypeScript SDKs       ← planned
├── MCP Server                     ← planned
└── Future Desktop/Web Apps
          │
          ▼
      Strixonomy (ships today)
────────────────────────
Workspace Engine
Parser
Semantic Index
Query Engine
SQL/SPARQL (SQL-like virtual tables)
Diagnostics
Navigation
Refactoring
Plugin host (SDK 1.0 wire frozen; marketplace → product 1.0)
Persistent Cache
LSP
          │
          ▼
      Ontologos
────────────────────────
Reasoning
Classification
Consistency
Inference
Explanations
          │
          ▼
OWL • RDF • Turtle • OBO
(SHACL: plugin scaffold)
```

!!! note "Plugin platform"
    Plugin **SDK 1.0** freezes the TOML + subprocess JSON wire (`api_version = "1"`) — safe to author against today. A curated marketplace and production owlmake integration remain **product 1.0** goals. See [Plugin policy](guides/plugin-policy.md) and [Plugin authoring](guides/plugins.md).

## Responsibilities

### Ontologos

Reasoning algorithms and semantic inference. Strixonomy delegates classification, consistency, and explanations to Ontologos — it does not embed a separate reasoner.

### Strixonomy

Reusable semantic workspace platform: index, query, diagnostics, refactoring, and semantic diff. Consumed by the VS Code extension, CLI, and Rust library.

**Plugin platform status:**

- **Shipped (SDK 1.0 / v0.26):** frozen wire contract — workspace manifest discovery, reference plugins, CLI/LSP hooks, subprocess workflow runner, UI views/commands/preferences/context actions, lifecycle (`depends_on` / `activation`), provider actions (see [Plugin authoring](guides/plugins.md)).
- **Product 1.0 targets:** curated marketplace/discovery and production owlmake hardening.

Strixonomy is **not** a workflow engine; build, release, and QC automation should live in external tools and workflow plugins rather than becoming core engine dependencies.

### External workflow plugins (e.g. owlmake)

**SDK 1.0 wire ships today; production owlmake integration is product 1.0.** [owlmake](https://github.com/INCATools/owlmake) is the reference workflow plugin design — ROBOT/ODK-style pipelines without becoming a core Strixonomy dependency. Today, ROBOT interop is the `strixonomy robot` CLI wrapper plus the subprocess workflow scaffold.

### Strixonomy

Reference IDE on top of Strixonomy. Presents editing, reasoning, and diagnostics in VS Code. Plugin views, commands, preferences, and context actions ship today (SDK 1.0); marketplace-scale workflow automation remains a product **1.0** target.

## Design Philosophy

Ontologos thinks.

Strixonomy understands.

Strixonomy presents.

Workflow plugins automate.

## Future Extensions

- Plugin marketplace and discovery
- owlmake and third-party workflow plugins
- AI assistants
- Enterprise governance
- Documentation generators (via plugin APIs)
- Visualization tools
- Collaborative editing
- JetBrains and web clients

For implementation-level crate layout and diagrams, see [Implementation architecture](https://strixonomy-vs.readthedocs.io/en/latest/design/ARCHITECTURE/) (also [docs/design/ARCHITECTURE.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/design/ARCHITECTURE.md) on GitHub).
