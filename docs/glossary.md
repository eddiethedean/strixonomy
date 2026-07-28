# Glossary

Canonical terminology for Strixonomy IDE, Strixonomy engine, OntoUI, and the Ontologos platform. Use these terms consistently in planning, architecture, and implementation docs.

| Term | Definition | Status |
|------|------------|--------|
| **Strixonomy engine** | Rust semantic workspace engine — indexing, queries, diagnostics, reasoning integration, refactoring, CLI, LSP, plugin host | **Implemented** (v0.28) |
| **Strixonomy IDE** | VS Code extension — host shell, commands, LSP client, webview hosting | **Implemented** (v0.28) |
| **OntoUI** | Shared React UI platform — component library, design tokens, workspace runtime; lives in `extension/webview-ui/` | **Partial** — WorkspaceStore + focus relay **shipped v0.13**; plugin inspector cards **shipped v0.14** |
| **OntoStudio** | Future standalone desktop application reusing OntoUI + Strixonomy engine | **Planned** (post v1.0) |
| **Workspace** | Task-oriented product surface (Entity, Graph, Query, Reasoning, Refactoring, …) — **not** a VS Code workspace folder | **Partial** — cross-panel focus sync shipped v0.13; surfaces remain webview panels |
| **Current Focus** | Active semantic object (entity, axiom, query, diagnostic, graph node) that drives UI synchronization | **Shipped** (v0.13) |
| **WorkspaceStore** | Single source of truth for OntoUI global state | **Shipped** (v0.28) |
| **WorkspaceRegistry** | Registry mapping workspace type → factory / component | **Partial** — capability registry shipped v0.14 |
| **WorkspaceHost** | Host adapter bridging OntoUI to a shell (VS Code webview, future Electron/Tauri) | **Partial** — VS Code extension host only |
| **Capability Provider** | Plugin-provided implementation of a platform capability (reasoning, querying, AI, refactoring, diagnostics, import/export) | **Shipped** (v0.14) — MVP registry + inspector cards |
| **Plugin host** | Strixonomy engine runtime that discovers workspace manifests and runs in-process or subprocess plugins | **Shipped** (v0.14) |
| **Ontologos** | External OWL reasoner (EL/RL/RDFS/DL) integrated via `strixonomy-reasoner` | **Implemented** (v0.9+) |

## Disambiguation

| Avoid | Use instead |
|-------|-------------|
| "VS Code workspace" when meaning product UI | **Workspace** (product) or "VS Code workspace folder" |
| "Shared React UI" / "Frontend platform" | **OntoUI** (implementers); say "Strixonomy panels" for end users |
| "SQL" without qualification | **catalog SQL (subset)** |
| "Plugin" (generic) | **Capability Provider** when referring to extensibility interfaces |
| "Panel" (long-term) | **Workspace** (product surface); "panel" OK for current webview implementation |
| Bare "Strixonomy" when the audience might confuse IDE vs engine | **Strixonomy IDE** or **Strixonomy engine** — [Product identity](guides/product-identity.md) |
| `cargo install strixonomy` | `cargo install strixonomy-cli` (repo is `strixonomy`; CLI crate is `strixonomy-cli`) |
| OntoIndex / `ontoindex` | **Strixonomy engine** / `strixonomy` (renamed in v0.9) |
| OntoCore / OntoCode | **Strixonomy engine** / **Strixonomy IDE** (renamed in v0.27) |
| OntoLogos (alternate casing) | **Ontologos** |
| "Byte-identical XML" | **Semantic re-serialize** — meaning preserved, Protégé layout not — [OWL/XML workflow](guides/owl-xml-workflow.md) |

## See also

- [Product identity](guides/product-identity.md)
- [Known limitations](known-limitations.md)
- [What ships today](SHIPPED.md)
- [Engineering docs (GitHub)](engineering.md)
- [Documentation index](documentation-index.md)
