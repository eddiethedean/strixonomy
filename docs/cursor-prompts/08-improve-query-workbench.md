# Cursor prompt: Improve Query Workbench + schema browser stub

## Prerequisites

Read:

- [ ] [[ONTOUI.md](../platform/ONTOUI.md)](../[QUERY_WORKBENCH_ARCHITECTURE.md](../platform/QUERY_WORKBENCH_ARCHITECTURE.md)`
- [ ] `ui/QUERY_WORKBENCH.md`

## Non-goals

- Full schema browser v0.30
- AI query generation

## Current state

- extension/webview-ui/src/panels/QueryWorkbench.tsx

## Tasks

1. Add query slice to WorkspaceStore (text, lastResult, language)
2. Persist last query in store on execute
3. Add collapsible SchemaBrowser stub component listing static SQL table names from docs
4. Wire QueryExecuted event on successful run
5. Update QueryWorkbench.test.tsx

## Acceptance criteria

- [ ] Query text survives re-render via store
- [ ] Schema stub renders
- [ ] Tests pass

## Tests

- `cd extension/webview-ui && npm test`

## Do not

- Do not fetch live catalog until LSP snapshot method wired

## References

- [Cursor prompts index](README.md)
