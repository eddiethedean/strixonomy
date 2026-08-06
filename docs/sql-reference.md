# Catalog SQL query reference (Strixonomy v0.28)

> **Status:** Documents behavior in **Strixonomy v0.28.1**. v0.29–v0.30 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md) · [Known limitations](known-limitations.md).

!!! warning "Not full SQL"
    Strixonomy exposes indexed ontology data as **virtual tables** queried with a **catalog SQL (subset)**. There is **no** `JOIN`, `GROUP BY`, `ORDER BY`, `LIMIT`, or subqueries. Prefer [SPARQL](sparql-reference.md) for graph patterns.

Strixonomy exposes indexed ontology data as **virtual tables** queried with a SQL-like `SELECT` syntax. The CLI (`strixonomy query`) and Rust API (`query_catalog`) use the same engine.

**Source of truth:** [`sql.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-query/src/sql.rs)

## Supported SQL subset

- `SELECT *` or `SELECT col1, col2, ...` from a single virtual table
- `FROM table_name` (one table only)
- `WHERE` with comparisons and boolean combinations:
  - `column = 'value'` or `column != 'value'` (string literals)
  - **Boolean column:** `deprecated = 'true'` or `deprecated = 'false'` (values are strings in the virtual table)
  - **Boolean shorthand:** bare `deprecated` is true when the column value is the string `"true"`
  - Combine with `AND` / `OR` only — **no parentheses**, no `NOT`, no `LIKE`, no `IN`
- Output formats: text (default), JSON (`--format json`), CSV (`--format csv`)

Not supported: `JOIN`, subqueries, `GROUP BY`, `ORDER BY`, SQL functions, `NOT`, parentheses, `LIKE`, `IN`, or multiple tables.

```bash
strixonomy query fixtures "SELECT short_name FROM classes WHERE short_name = 'Person'"
strixonomy query fixtures "SELECT short_name FROM classes WHERE deprecated = 'false' AND short_name = 'Person'"

# Fails at parse/eval time
strixonomy query fixtures "SELECT short_name FROM classes WHERE NOT deprecated"
```

**Expected output (text):** tab-separated header plus one row for `Person` when run on `fixtures/`.

SPARQL over indexed triples: [sparql-reference.md](sparql-reference.md).

**v0.30 plan:** extend [`sqlparser`](https://crates.io/crates/sqlparser) virtual-table joins/aggregations first ([ADR-0011](design/adr/0011-use-sqlparser-for-sql.md) amendment). Revisit [DataFusion](https://crates.io/crates/datafusion) only if scope exceeds hand-rolled implementation — see [DEPENDENCY_MATRIX.md](design/DEPENDENCY_MATRIX.md).

**Limits:** query strings up to 1 MiB; result sets capped at 100,000 rows (see [workspace-limits.md](workspace-limits.md)).

> **Warning:** Both SQL and SPARQL silently truncate at 100,000 rows. The CLI does not exit non-zero for truncation. LSP responses set `truncated: true`. Do not use row counts as strict CI gates without checking [workspace-limits.md](workspace-limits.md).

## Failure modes

| Situation | Behavior |
|-----------|----------|
| Unsupported SQL (`JOIN`, `ORDER BY`, `LIKE`, `NOT`, parentheses, …) | CLI exit non-zero; LSP `QUERY_FAILED` |
| Unknown table or column | Parse/eval error (non-zero / `QUERY_FAILED`) |
| `OR` without parentheses | Left-to-right boolean combination only — write simpler `WHERE` or use SPARQL |
| `SELECT *` with **zero rows** | Still returns column headers (schema) — empty body |
| Empty workspace / not indexed | Index first (`strixonomy index` / Index Workspace) |

See [Errors reference](errors.md) (`query` exit row) and [Query cookbook](examples/queries.md).

## Virtual tables and columns

### `ontologies`

| Column | Description |
|--------|-------------|
| `id` | Document id (`doc-1`, …) |
| `path` | Filesystem path |
| `format` | `turtle`, `rdf_xml`, `owl`, … |
| `base_iri` | Declared base IRI |
| `parse_status` | `ok`, `warning`, or `error` |
| `content_hash` | SHA-256 content hash |
| `modified_time` | File mtime (seconds) |

### `classes`, `object_properties`, `data_properties`, `annotation_properties`, `individuals`, `entities`, `properties`

Entity tables share these columns (`properties` is the union of all property kinds):

| Column | Description |
|--------|-------------|
| `iri` | Full IRI |
| `short_name` | Local name |
| `kind` | `class`, `object_property`, … |
| `ontology_id` | Owning ontology id |
| `labels` | Semicolon-separated labels |
| `comments` | Semicolon-separated comments |
| `deprecated` | `true` or `false` |
| `obo_id` | OBO id when indexed from `.obo` (empty for RDF-only entities) |

### `annotations`

| Column | Description |
|--------|-------------|
| `subject` | Annotation subject IRI |
| `predicate` | Predicate IRI |
| `object` | Object value |
| `ontology_id` | Document id |

### `axioms`

| Column | Description |
|--------|-------------|
| `id` | Axiom id |
| `ontology_id` | Document id |
| `subject` | Subject IRI |
| `predicate` | Predicate IRI |
| `object` | Object IRI or value |
| `axiom_kind` | e.g. `sub_class_of` |

### Horned-OWL axiom projections (v0.13)

These tables project structured axioms from the Horned-OWL catalog (Turtle, OWL/XML, `.owx`):

| Table | Columns |
|-------|---------|
| `restrictions` | `class_iri`, `property_iri`, `restriction_kind`, `filler` |
| `equivalent_class_axioms` | `class_iri`, `expression` |
| `disjoint_class_axioms` | `class_iri`, `disjoint_with` |
| `domain_axioms` | `property_iri`, `domain` |
| `range_axioms` | `property_iri`, `range` |

Browse live schema in the Query Workbench **Schema** sidebar (`strixonomy/listSqlSchema`).

### `namespaces`

| Column | Description |
|--------|-------------|
| `prefix` | Prefix name |
| `iri` | Namespace IRI |
| `ontology_id` | Document id |

### `imports`

| Column | Description |
|--------|-------------|
| `ontology_id` | Importing document id |
| `import_iri` | Imported ontology IRI |

### `diagnostics`

| Column | Description |
|--------|-------------|
| `code` | `parse_error`, `broken_import`, `undefined_prefix`, `duplicate_label`, `missing_label`, `orphan_class` |
| `severity` | `error`, `warning`, or `info` |
| `message` | Human-readable description |
| `file` | Filesystem path |
| `line` | 1-based line number (empty if unknown) |
| `column` | 0-based column (empty if unknown) |
| `entity_iri` | Related entity IRI, if any |

## Examples

See [query cookbook](examples/queries.md) for a copy-paste cookbook.

```bash
strixonomy query ./fixtures "SELECT * FROM classes"
strixonomy query ./fixtures "SELECT short_name, labels FROM classes WHERE short_name = 'Person'"
strixonomy query ./fixtures "SELECT * FROM annotations" --format json
strixonomy query ./fixtures "SELECT code, message FROM diagnostics WHERE severity = 'warning'"
```
