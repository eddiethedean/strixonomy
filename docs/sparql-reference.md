# SPARQL reference (Strixonomy v0.26)

> **Status:** Documents behavior in **Strixonomy v0.26.2**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

Run SPARQL queries over the **indexed triple store** built from workspace ontology files.

```bash
strixonomy sparql /path/to/workspace "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
```

From a git clone:

```bash
cargo run -- sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"
```

**Source of truth:** [`sparql.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-query/src/sparql.rs) (Oxigraph SPARQL engine).

## Query forms

| Form | Supported? | Result shape |
|------|------------|--------------|
| `SELECT` | **Yes** | Tabular `columns` + `rows` |
| `ASK` | **Yes** | Single row column `boolean` (`true`/`false`) |
| `CONSTRUCT` | **No** | Error: graph results not supported |
| `DESCRIBE` | **No** | Error: graph results not supported |
| SPARQL Update (`INSERT`/`DELETE`/…) | **No** | Error: updates not supported — use patches |

Engine: [Oxigraph](https://github.com/oxigraph/oxigraph) via [`sparql.rs`](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-query/src/sparql.rs).

## Behavior

- Queries run against triples parsed from all indexed files in the workspace
- The workspace is re-indexed on each CLI invocation (same as `query` and `validate`)
- Result formats: text (default), JSON (`--format json`)
- SQL virtual tables are separate — use `strixonomy query` for catalog tables like `classes`

## Prefixes and IRIs

- Use **full IRIs** in queries, or declare prefixes in the SPARQL query:

```sparql
PREFIX ex: <http://example.org/people#>
SELECT ?label WHERE { ex:Person rdfs:label ?label }
```

- Prefixes from Turtle `@prefix` declarations in source files are **not** automatically injected into SPARQL — declare them in the query string.

## Examples

```bash
# All triples (limited)
strixonomy sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 5"

# ASK
strixonomy sparql fixtures "ASK { <http://example.org/people#Person> a owl:Class }"

# Labels for a known IRI
strixonomy sparql fixtures "SELECT ?label WHERE { <http://example.org/people#Person> rdfs:label ?label }"

# Count classes (approximate via rdf:type)
strixonomy sparql fixtures "SELECT (COUNT(?c) AS ?count) WHERE { ?c a owl:Class }"

# JSON output
strixonomy sparql fixtures "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 3" --format json
```

More examples: [query cookbook](examples/queries.md).

## Limits and failure modes

| Limit / failure | Behavior |
|-----------------|----------|
| Query string size | 1 MiB (`MAX_QUERY_BYTES`) — larger queries error |
| Result rows | 100,000 (silently truncated; LSP sets `truncated: true`) |
| `CONSTRUCT` / `DESCRIBE` | Hard error (graph results unsupported) |
| SPARQL Update | Hard error (read-only queries only) |
| Parse / engine errors | Surfaced as `QUERY_FAILED` / SPARQL error strings |

See [workspace-limits.md](workspace-limits.md).

## LSP

SPARQL is available via the CLI, Rust API, LSP (`strixonomy/sparql`), and the VS Code **Query Workbench** (v0.5+). See [lsp-api.md](lsp-api.md).

## Related

- SQL-like catalog queries: [sql-reference.md](sql-reference.md)
- Query cookbook: [query cookbook](examples/queries.md)
