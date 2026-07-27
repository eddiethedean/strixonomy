# Platform Architecture

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


> **Implementation architecture:** [platform/OVERVIEW.md](../platform/OVERVIEW.md) · **Terms:** [Glossary](../glossary.md)

## 1. Overview

The Ontologos platform is a layered architecture centered on Strixonomy. See [platform/OVERVIEW.md](../platform/OVERVIEW.md) for the canonical implementation diagram.

```text
Applications
├── Strixonomy (VS Code)
├── OntoStudio (planned)
└── Future Web Client

OntoUI (shared React platform — v0.13 foundation shipped)
├── Component library · Design tokens
├── Workspace runtime (WorkspaceStore, focus relay, event bus)
└── WorkspaceHost adapter

Strixonomy (implemented v0.13)
└── … see platform/OVERVIEW.md
```

## 2. Strixonomy Responsibilities

Strixonomy owns semantic truth.

- Parse ontology formats.
- Build workspace indexes.
- Resolve symbols and references.
- Provide semantic diagnostics.
- Execute queries.
- Coordinate reasoning.
- Support semantic refactoring.
- Produce semantic diffs.
- Generate documentation metadata.
- Expose stable APIs to applications and plugins.

## 3. Strixonomy Responsibilities

Strixonomy is the VS Code product surface.

- Extension activation.
- VS Code commands.
- LSP integration.
- Webview hosting.
- File watching.
- Workspace discovery.
- Command palette contribution.
- VS Code settings.
- Native tree views where appropriate.
- Bridge between VS Code and shared React UI.

## 4. OntoStudio Responsibilities

OntoStudio is the dedicated desktop product.

- Native app shell.
- Multi-window layouts.
- High-performance graph rendering.
- Plugin marketplace.
- Offline-first workflows.
- Local AI integration.
- Enterprise deployment support.

## 5. OntoUI (shared React platform)

**OntoUI** is host-agnostic React UI shared by Strixonomy and future OntoStudio. **WorkspaceHost** adapters provide shell integration.

> **Architecture:** [platform/ONTOUI.md](../platform/ONTOUI.md) · [adr/0001](../adr/0001-ontoui-shared-react-platform.md)

Host adapters provide: file operations, notifications, theme, clipboard, command execution, Capability Provider loading (planned), native shell integrations.

## 6. Capability Providers

Extensibility uses **Capability Provider** interfaces.

> **Architecture:** [platform/CAPABILITY_PROVIDERS.md](../platform/CAPABILITY_PROVIDERS.md) · [adr/0005](../adr/0005-capability-provider-plugin-model.md)

Examples:

```ts
interface ReasoningProvider {}
interface AIProvider {}
interface QueryLanguageProvider {}
interface RefactoringProvider {}
interface DiagnosticsProvider {}
interface DocumentationProvider {}
interface VisualizationProvider {}
interface ImportExportProvider {}
```

Capability Providers register with the platform; Strixonomy hosts runtime (planned v0.14).

## 7. API Boundaries

### Rust Boundary

Rust exposes typed APIs through:

- LSP
- JSON-RPC
- WASM where appropriate
- Native Tauri commands
- CLI
- Future bindings

### Frontend Boundary

Frontend communicates through a typed client:

```ts
const workspace = await api.workspace.open(path)
const entity = await api.entities.get(id)
const result = await api.query.run(query)
```

No component directly calls raw transport APIs.

## 8. Performance Architecture

- Incremental parsing.
- Incremental indexing.
- Virtualized UI lists.
- Background reasoning.
- Cached query plans.
- Progressive graph loading.
- Lazy plugin activation.
- Streaming AI responses.

## 9. Security Architecture

- Plugin sandboxing.
- Capability permissions.
- Signed plugin packages.
- Workspace trust model.
- AI data disclosure controls.
- Enterprise policy hooks.

## 10. Success Criteria

The platform succeeds when Strixonomy, OntoStudio, Capability Providers, AI tools, and automation systems share the same semantic foundation without duplicating domain logic.

## Evolution

Layer diagram and API boundaries consolidated into [platform/OVERVIEW.md](../platform/OVERVIEW.md). This page retains product-level responsibility split and UX context.
