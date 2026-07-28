# Query Workbench

The **Query Workbench** is an Strixonomy React panel for **Catalog SQL (subset)**, **SPARQL**, and **DL Query** (Manchester class expressions) against your indexed workspace. Queries execute in **Strixonomy** via LSP (`strixonomy/query`, `strixonomy/sparql`, `strixonomy/dlQuery`).

| Mode | Engine | Best for |
|------|--------|----------|
| **Catalog SQL (subset)** | Virtual tables | Tabular listing (`classes`, `properties`, …) — **not full SQL** |
| **SPARQL** | Oxigraph | Graph patterns, filters, `LIMIT` |
| **DL Query** | Reasoner + Manchester | Class expression queries (Instances / Subclasses / …) |

!!! note "Catalog SQL is not full SQL"
    Supported: single-table `SELECT`, simple `WHERE` (`=`, `!=`, `AND`/`OR`). No `JOIN`, `ORDER BY`, `GROUP BY`, or `LIMIT`. Use **SPARQL** for graph patterns or **DL** for Manchester expressions.

!!! tip "DL Query (v0.24+)"
    **DL** mode provides Protégé-style Manchester class expressions with Instances / Subclasses / Superclasses / Equivalents tabs (asserted or inferred). Details: [DL Query](../guides/dl-query.md).

## Open the workbench

1. **Command Palette** (`Ctrl+Shift+P` / `Cmd+Shift+P`) → **Strixonomy: Open Query Workbench**
2. Or open Query Workbench from the Command Palette after [First success](../guides/first-success.md) and run `SELECT short_name FROM classes`

## Modes

| Mode | Engine | Use for |
|------|--------|---------|
| **SQL** | Strixonomy virtual tables | Catalog queries (`classes`, `properties`, `diagnostics`, …) |
| **SPARQL** | Oxigraph over indexed triples | RDF graph patterns |
| **DL Query** | Reasoner (`strixonomy/dlQuery`) | Manchester class expressions → Instances / Subclasses / Superclasses / Equivalents |

Toggle **Mode** at the top of the panel. Starter templates load when you switch modes.

### SQL quick start

```sql
SELECT short_name, labels FROM classes
```

Virtual table schemas: [Strixonomy SQL views](../strixonomy/sql-views.md) · [SQL reference](../sql-reference.md).

### SPARQL quick start

```sparql
SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10
```

More examples: [SPARQL reference](../sparql-reference.md) · [Query cookbook](../examples/queries.md).

### DL Query quick start (v0.24+)

Enter a Manchester expression such as `Person and hasPet some Dog`, choose asserted or inferred, and run. Results appear in the four tabs. More: [DL Query](../guides/dl-query.md).

## Schema browser (v0.13)

When the workspace is indexed, the Query Workbench shows a collapsible **Schema** sidebar (SQL mode only):

1. Expand a table name to see columns and types.
2. Click a **column** to insert its name into the query editor.
3. Click **Insert table query** for `SELECT * FROM <table>`.

Schema metadata comes from LSP `strixonomy/listSqlSchema`, including Horned-OWL axiom tables (`domain_axioms`, `range_axioms`, `restrictions`, …). See [SQL reference](../sql-reference.md#horned-owl-axiom-projections-v013).

## Cross-panel focus (v0.13)

Selecting an entity in the explorer updates **Current Focus** across open React panels (Inspector, Graph) via the extension-host focus relay. You do not need to re-select the same entity in each panel.

## Results

- Results appear in a table below the query editor.
- If the server row cap (100,000 rows) is hit, a banner shows **Results truncated at server row limit.**
- **Export CSV** or **Export JSON** copies the current result to the clipboard.

## Saved queries and history

- **Save Query** — name and store the current query in workspace state.
- **Saved** / **History** dropdowns reload prior queries.
- History length is controlled by the `strixonomy.queryHistoryLimit` setting (default 20).

## CLI equivalent

```bash
strixonomy query . "SELECT short_name, labels FROM classes"
strixonomy sparql . "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
strixonomy dl-query "Person and hasPet some Dog" --profile dl
```

See [Rust & CLI guide](../guides/rust-crates.md).
