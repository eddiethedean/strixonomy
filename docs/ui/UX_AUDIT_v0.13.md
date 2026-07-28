# UX audit — v0.13

Audit date: 2026-07-07. Scope: core flows after OntoUI platform migration (WorkspaceStore, focus relay, schema browser).

## Flows exercised

| Flow | Result | Notes |
|------|--------|-------|
| Open repo → index | Pass | Auto-index on activate unchanged |
| Explorer → Entity Inspector | Pass | `openEntity` sets focus relay + reuses panel |
| Inspector → Graph neighborhood | Pass | Graph panel hydrates from `focusState` |
| Query Workbench → schema insert | Pass | LSP `listSqlSchema` + sidebar insert |
| Reasoner run → store bridge | Pass | `reasoningState` relay updates store |
| Refactor preview | Pass | `pendingRefactor` slice drives preview panel |

## Top issues found (v0.13 fixes)

1. **Focus desync on rapid panel open** — mitigated by extension-host `FocusRelayService` as canonical source; webviews hydrate on `ready`.
2. **Query Workbench layout** — schema browser collapsible aside added; query text owned by store.
3. **Inspector navigation context** — store `focus` slice drives header; explorer sets relay before open.

## Accessibility checklist (P0 panels)

| Panel | Keyboard | aria-labels | Focus ring (tokens) |
|-------|----------|-------------|---------------------|
| Entity Inspector | Partial (forms) | Icon buttons labeled where added | `--oc-focus-ring` via primitives |
| Query Workbench | Schema browser tab order | Schema toggle/columns labeled | Yes |
| Graph Panel | Node selection via click | Toolbar buttons need follow-up | Partial |

## Deferred (v0.30)

- Full keyboard graph navigation
- Persistent tab/layout store slices
- PR summary UI panel (CLI/LSP only in v0.13)

## Sign-off

P0 panels meet v0.13 accessibility minimum per [ACCESSIBILITY_SPEC.md](ACCESSIBILITY_SPEC.md) for schema browser and shared primitives.
