# Strixonomy React UI Integration Plan

> **Status: Historical.** React + Vite foundation and panel migration **shipped** (v0.7a–v0.13+). Do **not** treat this as a current implementation plan.
>
> **Shipped today:** React webviews (inspector, graphs, Query Workbench, Manchester, reasoner, explanation, refactor preview, semantic diff, imports) — see [What ships today](../SHIPPED.md) and [Webview protocol](../webview-protocol.md).
>
> **Related:** [ROADMAP.md](ROADMAP.md) · [ADR-0017](adr/0017-react-webview-ui.md) · [ARCHITECTURE.md](ARCHITECTURE.md) §5

## Purpose

This document defines the strategy for migrating Strixonomy's existing VS Code webviews to a modern React-based architecture while preserving the existing TypeScript extension host and Rust backend services.

---

## Background

Current state:

- VS Code extension under `extension/`
- Webviews implemented with hand-written HTML/CSS/JavaScript
- Rust workspace provides ontology indexing, parsing, querying, reasoning, and LSP functionality

Target state:

- React + TypeScript UI
- Vite build pipeline
- Secure VS Code webviews
- Typed message protocol
- Marketplace-ready architecture

---

## Architecture

```text
extension/
  src/
    extension.ts
    webviews/
      getWebviewHtml.ts
      panelHost.ts
      messages.ts

  webview-ui/
    package.json
    vite.config.ts

    src/
      main.tsx
      App.tsx
      vscodeApi.ts

      panels/
        EntityInspector.tsx
        QueryWorkbench.tsx
        ManchesterEditor.tsx
        ReasonerPanel.tsx
        ExplanationPanel.tsx

      components/
      styles/
```

### Responsibilities

#### VS Code Extension Host

Responsible for:

- VS Code API integration
- Commands
- Tree Views
- LSP integration
- Ontology service orchestration
- Webview lifecycle management

#### React Application

Responsible for:

- User interface
- State management
- Data visualization
- Forms and editors
- Query results
- Reasoning displays

---

## Webview Architecture

### Build Process

- Vite
- React
- TypeScript

Output:

```text
extension/webview-ui/dist
```

Assets loaded using:

```ts
webview.asWebviewUri(...)
```

### Security

Requirements:

- Strict Content Security Policy
- Nonce-based script execution
- No external CDNs
- No inline JavaScript
- All assets bundled locally

---

## Typed Message Protocol

### Shared Messages

#### Extension -> React

- loadEntity
- queryResult
- validationResult
- reasonerResult
- explanationResult
- error

#### React -> Extension

- ready
- runQuery
- validateManchester
- runReasoner
- explainAxiom

### TypeScript Contracts

Create:

```ts
interface Message {
    type: string;
    payload: unknown;
}
```

Expand into strongly typed unions as implementation progresses.

---

## UI Vision

### Entity Inspector

Features:

- Classes
- Properties
- Individuals
- Annotations
- Axiom navigation

### Query Workbench

Features:

- Query editor
- Saved queries
- Query history
- Results table
- CSV export

### Manchester Editor

Features:

- Syntax highlighting
- Validation
- Error display
- Quick fixes

### Reasoner Panel

Features:

- Consistency checking
- Classification
- Inferred relationships
- Incremental reasoning status

### Explanation Panel

Features:

- Why explanations
- Justifications
- Axiom traces
- Dependency visualization

---

## Migration Roadmap

Phases map to product milestones in [ROADMAP.md](ROADMAP.md). Ontology features (graphs, OBO, refactoring, semantic diff) ship on the React stack once the foundation is in place.

| Phase | Product milestone | Deliverables |
|-------|-------------------|--------------|
| 1–2 | **v0.7a** — React foundation | Vite + React + TypeScript; `build:webview` in VSIX pipeline; typed message protocol; panel host; CSP framework |
| 3 + graphs | **v0.7** — Visualization | React entity inspector; class/property/import/neighborhood graph panels |
| 4–5 | **v0.8** — Refactoring + full Manchester | React query workbench; React Manchester editor with validation UI |
| 6 + diff | **v0.9** — Workflow | React reasoner + explanation panels; semantic diff panel |
| 7 | **v0.30** — Release hardening | Accessibility review; webview integration tests; all production panels on React; legacy HTML panels removed |

### Phase 1 — React foundation (v0.7a)

Deliverables:

- Vite setup
- React setup
- Build integration (`extension/webview-ui/dist` → VSIX)

### Phase 2 — Shared infrastructure (v0.7a)

Deliverables:

- Message bus
- Panel host
- CSP framework (nonce scripts, no CDNs)

### Phase 3 — Entity Inspector migration (v0.7)

Deliverables:

- React-based entity browser
- Shared layout primitives for future panels

### Phase 4 — Query Workbench migration (v0.8)

Deliverables:

- React query interface (SQL + SPARQL tabs, history, export)

### Phase 5 — Manchester Editor migration (v0.8)

Deliverables:

- Validation UI, error display, quick-fix hooks

### Phase 6 — Reasoner + Explanation migration (v0.9)

Deliverables:

- Modern reasoning experience (classification status, inferred edges, justification traces)

### Phase 7 — Hardening (v0.30)

Deliverables:

- Unit + integration tests (extension ↔ React)
- Accessibility review (keyboard, screen reader, VS Code themes)
- Marketplace CSP compliance sign-off
- Remove legacy hand-written webview HTML

---

## UX Goals

- Professional ontology IDE experience
- Native VS Code theme support
- Keyboard-first navigation
- Accessibility compliance
- Responsive layouts
- Fast rendering on large ontologies

---

## Build Requirements

### NPM Scripts

```json
{
  "build:webview": "vite build",
  "build": "npm run build:webview && esbuild ..."
}
```

### Packaging

Requirements:

- React assets bundled automatically
- Included in VSIX packages
- No external dependencies at runtime

---

## Testing Strategy

### Unit Tests

- Message handling
- Component rendering
- State transitions

### Integration Tests

- Extension ↔ React communication
- Query execution
- Reasoner workflows

### Accessibility Tests

- Keyboard navigation
- Screen reader compatibility
- Theme validation

---

## Acceptance Criteria

- Existing extension functionality remains operational
- React build integrated successfully
- At least one production React panel migrated
- Typed message protocol documented
- CSP marketplace compliant
- Automated tests implemented
- Documentation updated

---

## Long-Term Vision

Transform Strixonomy into a modern ontology engineering platform that combines:

- Rust performance
- VS Code extensibility
- React user experience
- Enterprise-grade ontology tooling

while positioning Strixonomy as a credible long-term replacement for traditional ontology desktop tools.
