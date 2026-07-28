# Cursor prompt: Improve Entity Workspace (inspector)

## Prerequisites

Read:

- [ ] `ui/ENTITY_EDITOR_SPEC.md`
- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md)`

## Non-goals

- Full relationship cards v0.30
- OBO forms rewrite

## Current state

- extension/webview-ui/src/panels/EntityInspector.tsx
- LSP getEntity via messages.ts

## Tasks

1. Read entity detail from WorkspaceStore focus when set
2. Reduce duplicate getEntity calls when focus unchanged
3. Wire PreviewApplyBar to store pending patch state (stub slice ok)
4. Add/update EntityInspector.test.tsx for focus-driven load

## Acceptance criteria

- [ ] Inspector uses store focus IRI
- [ ] Tests pass
- [ ] Preview flow unchanged functionally

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not add new patch ops in TS

## References

- [Cursor prompts index](README.md)
