# ADR-0017 — Adopt React for VS Code Webview Panels

## Status

Accepted (shipped — React webview panels in production since mid-0.x)

## Context

Strixonomy v0.5–v0.6 ships webview panels (entity inspector, query workbench, Manchester editor, reasoner, explanation) as hand-written HTML/CSS/JavaScript in the TypeScript extension host. This works for MVP delivery but becomes harder to maintain as panels grow in complexity (graphs, semantic diff, refactoring previews, large result tables).

The extension host must remain a thin orchestration layer per [ADR-0007](0007-language-server-boundary.md): ontology intelligence stays in Rust (`strixonomy-lsp`); TypeScript owns VS Code API integration only.

## Decision

Migrate Strixonomy webview panels to a **React + TypeScript** application built with **Vite**, loaded inside VS Code webviews via a typed message protocol between the extension host and the React app.

- **Extension host** (`extension/src/`): commands, tree views, LSP client, webview lifecycle, CSP nonces, `postMessage` bridge.
- **React app** (`extension/webview-ui/`): panel UI, state, forms, tables, graph rendering, theme-aware styling.
- **Rust backend**: unchanged — panels call existing LSP methods; no ontology logic in React.

New panels (graphs v0.7, semantic diff v0.9) ship on the React stack from the start. Existing panels migrate incrementally per [OntoCode_React_UI_Integration_Plan.md](../OntoCode_React_UI_Integration_Plan.md).

## Consequences

Positive:

- Maintainable component model for complex IDE panels
- Shared UI primitives across inspector, query workbench, Manchester editor, reasoner, graphs, diff
- Marketplace-compliant CSP (bundled assets, no CDNs) with a standard build pipeline
- Easier accessibility and keyboard-navigation testing

Negative:

- Additional build step (`build:webview`) and `webview-ui` package to maintain
- Two-process UI debugging (extension host ↔ webview iframe)
- Migration period where legacy and React panels may coexist briefly

## References

- [OntoCode_React_UI_Integration_Plan.md](../OntoCode_React_UI_Integration_Plan.md)
- [ROADMAP.md](../ROADMAP.md) — v0.7a through v0.30 React milestones
- [ARCHITECTURE.md](../ARCHITECTURE.md) §5 Strixonomy internal modules
