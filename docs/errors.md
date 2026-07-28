# Errors reference (Strixonomy v0.28)

Unified catalog of error codes, exit behavior, and failure modes for Strixonomy **v0.28.0** (latest tagged).

## CLI exit codes

| Command | Exit 0 | Exit non-zero |
|---------|--------|---------------|
| `strixonomy index` | Index succeeded | Index/parse/I/O failure |
| `strixonomy inspect` | Index succeeded | Index/parse/I/O failure |
| `strixonomy new` | Ontology created | Path exists (without `--force`), IRI validation failure, I/O |
| `strixonomy validate` | No diagnostic **errors** (warnings/info allowed; includes plugin validators) | One or more diagnostic **errors** (core or plugin) |
| `strixonomy query` | Query succeeded | Parse error, unsupported SQL, I/O failure |
| `strixonomy sparql` | Query succeeded (results may be truncated at row cap) | Parse error, I/O failure |
| `strixonomy dl-query` | Query succeeded | Manchester parse error, reasoner error, I/O |
| `strixonomy patch` | Patch applied or preview succeeded | Invalid patch, unsupported format, I/O failure |
| `strixonomy classify` | Consistent ontology (no unsatisfiable classes) | Unsatisfiable classes, reasoner error, I/O failure |
| `strixonomy explain` | Explanation produced | Class not found, explanation unavailable, reasoner error |
| `strixonomy realize` | Realization succeeded | Reasoner error, I/O |
| `strixonomy check-instance` | Instance **entailed** | **Not entailed**, or reasoner error |
| `strixonomy refactor` (subcommands) | Preview or apply succeeded | Invalid request, path outside workspace, I/O failure |
| `strixonomy diff` | Diff succeeded | Git/parse/I/O failure, invalid ref token |
| `strixonomy docs` | Export succeeded | Index, plugin export, or I/O failure |
| `strixonomy plugins list` | Discovery succeeded | Host/discovery failure |
| `strixonomy plugins run` | Plugin action succeeded | Host/action failure |
| `strixonomy workflow` | Workflow step succeeded | Plugin/subprocess failure |
| `strixonomy robot` | ROBOT subprocess exit 0 | ROBOT non-zero exit or spawn failure |

`validate` and `classify` exit semantics are stable for CI — see [workspace-limits.md](workspace-limits.md) and [ci-integration.md](ci-integration.md).

Other CLI commands return non-zero on failure with a human-readable message on stderr.

### `classify` (CLI) vs `runReasoner` (LSP)

| Surface | Unsatisfiable classes | Use in CI |
|---------|----------------------|-----------|
| `strixonomy classify` | **Non-zero exit** | Fail the job when the ontology is inconsistent |
| `strixonomy/runReasoner` | **Success** with `{ "consistent": false, ... }` | Inspect `consistent` in the JSON result; do not rely on JSON-RPC error |

IDE flows should use `runReasoner` and show unsatisfiability in the UI. Pipelines that gate merges should prefer `strixonomy classify`.

## LSP custom method errors (`LspErrorPayload`)

Custom `strixonomy/*` method failures return JSON-RPC errors with `data` containing:

| Field | Type | Description |
|-------|------|-------------|
| `code` | string | Machine-readable code |
| `message` | string | Human-readable message |
| `recoverable` | boolean | Whether the client can retry |
| `user_action` | string? | Suggested user action |

### LSP error codes

| Code | When | Typical `user_action` |
|------|------|------------------------|
| `INVALID_PARAMS` | Malformed or unknown method parameters | Fix request JSON; see [lsp-api.md](lsp-api.md) |
| `NOT_INDEXED` | Catalog methods called before first index | Run Strixonomy: Index Workspace |
| `ENTITY_NOT_FOUND` | `getEntity` / `setActiveOntology` id or IRI not in catalog | Check IRI/id spelling / re-index |
| `PATCH_INVALID` | Patch JSON invalid or entity missing | Check patch parameters and entity IRIs |
| `UNSUPPORTED_FORMAT` | Write-back on a read-only format | Write-back is supported for **Turtle**, **OBO**, **RDF/XML**, and **OWL/XML**. Convert JSON-LD / N-Triples / TriG before editing — see [supported-formats](supported-formats.md) |
| `INDEX_FAILED` | Indexing failed (parse, limits, I/O) **or plugin host failure** (`listPlugins`, `runPlugin`) | Check ontology files; verify plugin manifest and subprocess entry |
| `QUERY_FAILED` | SQL or SPARQL query failed | Check query syntax and [sql-reference](sql-reference.md) |
| `MANCHESTER_INVALID` | Manchester expression parse failed | Fix expression; see [Manchester editor](ide/manchester-editor.md) |
| `APPLIED_NOT_INDEXED` | Patch written to buffer/disk but reindex failed | Run Index Workspace; file may already be updated (`recoverable: true`) |
| `REASONER_FAILED` | `runReasoner` failed (profile, parse, OntoLogos error) | Try another profile or fix ontology axioms |
| `EXPLANATION_FAILED` | `getExplanation` failed | Run reasoner first or choose another class |
| `REFACTOR_FAILED` | Refactor preview/apply failed | Check IRIs, format coverage (rename/merge/replace vs Turtle-first ops), and preview plan |
| `GRAPH_FAILED` | `getGraph` failed | Re-index workspace or reduce neighborhood depth |
| `ROBOT_FAILED` | `runRobot` external process failed | Check `strixonomy.robotPath` and ROBOT install |

Plugin host errors (`listPlugins`, `runPlugin`) return `INDEX_FAILED` with a message describing the plugin failure (not `INVALID_PARAMS`).

Successful patch/refactor apply may include `reindex_warning` in the result when disk write succeeded but reindex failed.

```json
{
  "jsonrpc": "2.0",
  "id": 3,
  "error": {
    "code": -32000,
    "message": "Workspace has not been indexed yet",
    "data": {
      "code": "NOT_INDEXED",
      "message": "Workspace has not been indexed yet",
      "recoverable": true,
      "user_action": "Run Strixonomy: Index Workspace"
    }
  }
}
```

## Patch diagnostics (`PatchDiagnostic`)

When `strixonomy patch` or `strixonomy/applyAxiomPatch` fails validation, the result may include:

```json
{
  "applied": false,
  "diagnostics": [
    { "severity": "error", "message": "entity not found: http://example.org/unknown" }
  ]
}
```

| Field | Description |
|-------|-------------|
| `severity` | `error` or `warning` |
| `message` | Human-readable description |

LSP `applyAxiomPatch` returns the same `ApplyPatchResult` fields (`applied`, `preview_text`, `diagnostics`, `document_path`) plus optional `entity_detail` on success.

**Buffer-first apply (v0.5):** When called from VS Code, patches apply to the **open document buffer** first, then disk, then trigger reindex. If reindex fails after a successful write, the server returns `APPLIED_NOT_INDEXED` — the buffer/disk may already contain the patch.

## Lint diagnostic codes (`diagnostics` table)

| Code | Severity | Meaning |
|------|----------|---------|
| `parse_error` | error | File failed to parse |
| `broken_import` | error/warning | Import IRI could not be resolved |
| `undefined_prefix` | warning | Unknown prefix in file |
| `duplicate_label` | warning | Same label on multiple entities |
| `missing_label` | info/warning | Entity has no rdfs:label |
| `orphan_class` | info | Class with no declared parents |
| `owl_bridge_failed` | warning | Horned-OWL bridge failed; catalog uses parser-only semantics for that file |
| `io_read_error` | error | Diagnostic engine could not read file from disk |

Query diagnostics: `SELECT code, severity, message, file FROM diagnostics WHERE severity = 'error'`

## Plugin diagnostic codes (v0.14+)

Plugin validators contribute diagnostics with wire codes `plugin:<plugin_id>:<code>` and LSP `source` `strixonomy-plugin:<plugin_id>`.

| Wire code suffix | Plugin | Severity | Meaning |
|------------------|--------|----------|---------|
| `missing_label` | `strixonomy.naming-validator` | warning | Class/property missing `rdfs:label` |
| `iri_prefix` | `strixonomy.naming-validator` | warning | IRI does not match configured prefix |
| `shapes_missing` | `strixonomy.shacl-validator` | info | SHACL shapes directory not found |
| `shapes_empty` | `strixonomy.shacl-validator` | info | No `.ttl`/`.rdf`/`.shacl` files in shapes dir |
| `shacl_pending` | `strixonomy.shacl-validator` | info | Shapes found; full rudof validation not yet integrated |
| `plugin_error` | any | error | Plugin host run failure |

Example: `plugin:strixonomy.naming-validator:missing_label`

## Workspace limit failures

| Limit | Constant | Typical failure |
|-------|----------|-----------------|
| Files scanned | `MAX_SCAN_FILES` (10,000) | Scanner error during index |
| File size | `MAX_FILE_BYTES` (50 MB) | File skipped or index error |
| Open LSP buffers | `MAX_OPEN_DOCUMENTS` (256) | Document not tracked |
| Entities | `MAX_ENTITIES` (1,000,000) | Catalog build error |
| Triples total | `MAX_TOTAL_TRIPLES` (20M) | Index error |
| Triples per file | `MAX_TRIPLES_PER_FILE` (5M) | Per-file index error |
| Query size | `MAX_QUERY_BYTES` (1 MiB) | Query rejected |
| SQL rows | `MAX_SQL_RESULT_ROWS` (100k) | **Silent truncation** (`truncated: true` in LSP) |
| SPARQL rows | `MAX_SPARQL_RESULT_ROWS` (100k) | **Silent truncation** (`truncated: true` in LSP) |

Full limits: [workspace-limits.md](workspace-limits.md).

## Rust library errors

Integrators using crates.io crates should match **method → error type** (not every API returns the unified enum):

| API | Returns | Typical causes |
|-----|---------|----------------|
| `Workspace::open` / `open_with_options` / `reindex*` / `diff_against_path` | `CatalogError` | Path missing, scan/parse limits, I/O |
| `Workspace::query` / `sparql` | `QueryError` | Unsupported SQL, SPARQL parse, empty catalog edge cases |
| `Workspace::classify` / `explain` / `reasoner_input` | `ReasonerError` | Profile unsupported, axiom load failure |
| `import_graph*` | `GraphError` | Invalid depth / request |
| Owl / OBO patch helpers | `OwlError` / OBO errors | Patch mismatch, write-back format |
| Façade helpers that `?`-convert | `strixonomy::Error` | Unified: `Catalog` / `Query` / `Graph` / `Reasoner` / export / owl / obo variants |

| Crate | Error type | docs.rs |
|-------|------------|---------|
| `strixonomy` (façade) | `strixonomy::Error` | [Error](https://docs.rs/strixonomy/latest/strixonomy/enum.Error.html) |
| `strixonomy-core` | `StrixonomyError` | Shared diagnostic/core failures |
| `strixonomy-catalog` | `CatalogError`, `GraphError` | Index / graph |
| `strixonomy-query` | `QueryError` | SQL/SPARQL |
| `strixonomy-owl` | `OwlError` | Turtle patches |
| `strixonomy-reasoner` | `ReasonerError` | Classify/explain |
| `strixonomy-parser` | `ParseError` | RDF syntax |

**LSP mapping:** library failures often surface as `INDEX_FAILED`, `QUERY_FAILED`, `REASONER_FAILED`, or `UNSUPPORTED_FORMAT` in the table above.

crates.io tutorial: [Rust library guide](guides/rust-library.md). Clone-only example: [`examples/error_handling.rs`](https://github.com/eddiethedean/strixonomy/blob/main/examples/error_handling.rs).

## Related docs

- [LSP API](lsp-api.md) — custom methods and result types
- [patch-reference.md](patch-reference.md) — patch operations
- [sql-reference.md](sql-reference.md) — virtual tables and SQL subset
- [faq.md](faq.md) — common troubleshooting
- [troubleshooting.md](troubleshooting.md) — step-by-step fixes
