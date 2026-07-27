# Workspace resource limits

Strixonomy (`strixonomy-*`) enforces limits to keep local indexing predictable and to reduce DoS risk when
opening untrusted ontology repositories.

Constants live in [`limits.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-core/src/limits.rs).

## Recommended workspace size

| Guideline | Value |
|-----------|-------|
| Ontology files | Up to **10,000** per workspace (`MAX_SCAN_FILES`) |
| Filesystem walk entries | Up to **500,000** visited paths during scan (`MAX_SCAN_WALK_ENTRIES`) |
| Single file size | Up to **50 MB** on disk or in an LSP open buffer (`MAX_FILE_BYTES`) |
| Open editor buffers (LSP) | Up to **256** tracked paths (`MAX_OPEN_DOCUMENTS`) |
| Entities | Up to **1,000,000** per workspace (`MAX_ENTITIES`) |
| RDF triples | Up to **20,000,000** total (`MAX_TOTAL_TRIPLES`) |
| Triples per file | Up to **5,000,000** (`MAX_TRIPLES_PER_FILE`) |

Workspaces above these limits may fail indexing with a clear error. For very large
terminologies (e.g. full OBO), prefer CLI batch workflows and expect v0.4+ incremental
indexing improvements.

## Query limits

| Limit | Value |
|-------|-------|
| SQL/SPARQL query string | **1 MB** (`MAX_QUERY_BYTES`) |
| SQL result rows | **100,000** — **silently truncated** (no error) |
| SPARQL result rows | **100,000** — **silently truncated** (no error) |

LSP `strixonomy/query` and `strixonomy/sparql` set `truncated: true` on `TabularQueryResult` when the cap is hit.

## Graph visualization limits

| Limit | Value |
|-------|-------|
| Graph nodes per `getGraph` payload | **2,000** (`MAX_GRAPH_NODES`) |
| Graph edges per `getGraph` payload | **5,000** (`MAX_GRAPH_EDGES`) |

When either cap is hit, `GraphPayload.truncated` is `true` and the graph panel shows a truncation badge. Prefer neighborhood graphs rooted at a specific entity, lower `depth`, or apply ontology / kind / namespace filters.

## `strixonomy validate` exit codes

| Outcome | Exit code |
|---------|-----------|
| No diagnostic **errors** (warnings/info allowed) | **0** |
| One or more diagnostic **errors** | **non-zero** |

Warnings and info diagnostics are printed to stderr but do not fail CI.

## `strixonomy classify` exit codes

| Outcome | Exit code |
|---------|-----------|
| Consistent ontology (no unsatisfiable classes) | **0** |
| Inconsistent / unsatisfiable classes detected | **non-zero** |
| Reasoner error (unknown profile, classify failure, parse failure) | **non-zero** |

Use `--format json` in CI to inspect `consistent`, `unsatisfiable`, and `warnings`.
