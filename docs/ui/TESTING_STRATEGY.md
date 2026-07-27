# Testing Strategy

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


## 1. Purpose

Strixonomy requires confidence across Rust core logic, frontend UX, plugins, AI workflows, and integration boundaries.

## 2. Test Pyramid

- Rust unit tests
- Rust integration tests
- TypeScript unit tests
- React component tests
- Playwright E2E tests
- Plugin API contract tests
- AI workflow regression tests
- Accessibility tests
- Performance tests

## 3. Rust Core Tests

Cover:

- parsers
- indexes
- symbol resolution
- diagnostics
- query engine
- reasoning adapters
- refactoring engine
- semantic diffs

## 4. Frontend Tests

Cover:

- WorkspaceStore
- event bus
- component rendering
- selection behavior
- keyboard navigation
- empty/loading/error states

## 5. E2E Workflows

Required scenarios:

- open ontology workspace
- select entity
- edit label
- run reasoning
- run query
- graph exploration
- semantic refactor preview/apply/undo
- AI suggestion preview/apply
- review semantic diff

## 6. Plugin Tests

- manifest validation
- permission enforcement
- activation lifecycle
- command registration
- UI contribution rendering
- sandbox violation tests

## 7. AI Tests

- prompt snapshot tests
- structured context tests
- tool-call tests
- semantic patch validation
- hallucination guard tests

## 8. Accessibility Tests

- axe checks
- keyboard-only flows
- high contrast snapshots
- reduced motion tests

## 9. Performance Budgets

- entity selection <50ms
- search <100ms
- inspector update <50ms
- graph pan/zoom 60 FPS
- large ontology progressive loading

## 10. CI

All PRs run:

- formatting
- linting
- unit tests
- integration tests
- accessibility smoke tests
- selected E2E tests
