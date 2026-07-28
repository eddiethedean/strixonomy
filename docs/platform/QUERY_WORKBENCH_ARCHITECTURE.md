# Query workbench architecture

> **Status:** SQL/SPARQL editor + **schema browser shipped v0.13**

## Scope

Query Workspace: editor → LSP execution → results table → history/saved queries → schema browser.

## Flow

```text
User edits SQL or SPARQL
    ↓
strixonomy/query or strixonomy/sparql (LSP)
    ↓
Result rows + truncated flag
    ↓
Results table + export
    ↓
QueryExecuted event → WorkspaceStore
```

## Current implementation

- `extension/webview-ui/src/panels/QueryWorkbench.tsx`
- `extension/webview-ui/src/components/SchemaBrowser.tsx`
- LSP `strixonomy/listSqlSchema` — table/column metadata
- LSP methods in [lsp-api.md](../lsp-api.md)
- SQL subset: [sql-reference.md](../sql-reference.md)

## Store shape (v0.13)

```ts
interface QueryState {
  language: "sql" | "sparql"
  text: string
  lastResult: QueryResult | null
  history: QueryHistoryEntry[]
  saved: SavedQuery[]
  schemaBrowserExpanded: boolean
}
```

## Schema browser (v0.13)

- Virtual table list from `strixonomy/listSqlSchema` (includes Horned-OWL axiom tables)
- Column names and types per table
- **Insert column** — appends column name to editor
- **Insert table query** — `SELECT * FROM <table>`
- Collapsible sidebar; state in `query.schemaBrowserExpanded`

Example axiom tables: `domain_axioms`, `range_axioms`, `restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`. See [sql-reference.md](../sql-reference.md#horned-owl-axiom-projections-v013).

## Links

- [ui/QUERY_WORKBENCH.md](../ui/QUERY_WORKBENCH.md)
- [ide/query-workbench.md](../ide/query-workbench.md)
- [strixonomy/sql-views.md](../strixonomy/sql-views.md)

## Evolution

UX spec: [ui/QUERY_WORKBENCH.md](../ui/QUERY_WORKBENCH.md). SQL `JOIN` / `GROUP BY` deferred to v0.30.
