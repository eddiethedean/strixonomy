# Strixonomy LSP API (v0.27)

> **Status:** Documents behavior in **Strixonomy v0.27.0**. Pre-1.0 APIs may change.
> Canonical feature list: [What ships today](SHIPPED.md).

This document describes **what ships today** in `strixonomy-lsp`. Custom methods use the **`strixonomy/*`** prefix; servers also accept legacy **`ontocore/*`** through at least 1.0 ([migration/v0.27.md](migration/v0.27.md)). For the **v1.0 target** (extended plugin methods), see [LSP_SPEC.md](design/LSP_SPEC.md).

## Start here: index a workspace

Minimal custom-client flow after `initialize`:

1. Call `strixonomy/indexWorkspace` with `{ "rootUri": "file:///path/to/ontologies" }`.
2. Call `strixonomy/query` with `{ "sql": "SELECT short_name FROM classes" }` (or open [LSP hello world](guides/lsp-hello-world.md)).
3. Treat [`lsp-protocol.schema.json`](lsp-protocol.schema.json) as the machine-readable contract.

## Methods at a glance

| Method | Status | Since |
|--------|--------|-------|
| `textDocument/completion` | Shipped | v0.11 |
| `textDocument/codeAction` | Shipped | v0.11 |
| `textDocument/hover` | Shipped | early |
| `textDocument/definition` | Shipped | early |
| `textDocument/rename` / `prepareRename` | Shipped | rename path |
| `textDocument/semanticTokens/full` | Shipped | v0.13 |
| `strixonomy/indexWorkspace` | Shipped | early |
| `strixonomy/getCatalogSnapshot` | Shipped | early |
| `strixonomy/getEntity` | Shipped | early |
| `strixonomy/query` | Shipped | v0.5 |
| `strixonomy/sparql` | Shipped | v0.5 |
| `strixonomy/parseManchester` | Shipped | v0.5 |
| `strixonomy/applyAxiomPatch` | Shipped | early |
| `strixonomy/runReasoner` | Shipped (+ `$/cancelRequest`) | early |
| `strixonomy/getExplanation` | Shipped | v0.15 |
| `strixonomy/getGraph` | Shipped | v0.7 |
| `strixonomy/runRobot` | Shipped | v0.7 |
| `strixonomy/findUsages` | Shipped | v0.8 |
| `strixonomy/previewRefactor` / `applyRefactor` | Shipped | v0.8 |
| `strixonomy/semanticDiff` | Shipped | v0.10 |
| `strixonomy/listSqlSchema` | Shipped | v0.13 |
| `strixonomy/listPlugins` / `runPlugin` | Shipped | v0.14 |
| `strixonomy/listCommands` | Shipped | v0.17 |
| `strixonomy/getWorkspaceUiState` | Shipped | v0.17 |
| `strixonomy/getDialogSchema` | Shipped | v0.17 |
| `strixonomy/createOntology` / `exportOntology` / `setActiveOntology` / `deleteImpact` | Shipped | v0.17 |
| `strixonomy/dlQuery` / `search` | Shipped | v0.24 |
| `strixonomy/checkInstance` | Shipped | v0.23 |
| `strixonomy/listSwrlRules` / `validateSwrlRule` / `parseSwrlRule` | Shipped | v0.23 |
| `strixonomy/realize` | **Not an LSP method** — use CLI | — |

## Start with the schema (recommended)

If you are integrating Strixonomy outside VS Code (custom editor, scripts, automation), treat the JSON schema as the **canonical, machine-readable contract** for this release:

- **LSP JSON Schema:** [`lsp-protocol.schema.json`](lsp-protocol.schema.json) (ships with product **v0.27.0**)

### Schema vs product version

The schema file is the wire contract for the **current product release**. Until v1.0, minor product releases may add or change fields — always pin Strixonomy and consume the schema from the **same tagged release**. Historical labels such as “v0.17 schema” in older docs referred to the product release that last expanded the contract, not a separate schema versioning scheme.

### Versioning and pinning (pre-1.0)

Until v1.0, minor releases may change request/response fields.
For stable integrations:

- Pin Strixonomy to **0.27.0** in your tooling.
- Prefer consuming `lsp-protocol.schema.json` from the same tagged release you deploy.

## Wire format

LSP JSON uses **snake_case** for enums serialized from Rust (`EntityKind`, `ParseStatus`, `OntologyFormat`), e.g. `"kind": "class"`, `"parse_status": "ok"`. SQL virtual tables use the same snake_case strings via `as_str()` on core enums (e.g. `ParseStatus::as_str()` → `"ok"`, `EntityKind::as_str()` → `"class"`, `axiom_kind` → `"sub_class_of"`).

**Reference links (implementation):**

- Types: [`protocol.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-lsp/src/protocol.rs)
- JSON Schema: [`lsp-protocol.schema.json`](lsp-protocol.schema.json) — query, DL Query, search, patch, reasoner, refactor, graph, semantic diff, schema browser, PR summary, plugin payloads, SWRL, explanation alternatives.
- Handlers: [`handlers.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-lsp/src/handlers.rs)
- Extension client: [`client.ts` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/extension/src/lsp/client.ts)

## Transport

- stdio (VS Code language client)

## Standard LSP capabilities

| Capability | Status |
|------------|--------|
| `textDocument/hover` | Implemented (basic entity info) |
| `textDocument/documentSymbol` | Implemented |
| `workspace/symbol` | Implemented |
| `textDocument/definition` | Implemented |
| Diagnostics | **Implemented** — server pushes `textDocument/publishDiagnostics` after each reindex |
| `textDocument/completion` | **Implemented (v0.11)** — Turtle prefix, QName, and IRI contexts |
| `textDocument/codeAction` | **Implemented (v0.11)** — diagnostic quick fixes |
| Rename | **Implemented** — `textDocument/rename` + `textDocument/prepareRename` |
| Find references | **Implemented** — `textDocument/references` |

### `textDocument/completion` (v0.11)

Advertised with trigger characters `:`, `<`, and `@`. Applies to **Turtle (`.ttl`)** files only.

| Context | When | Items |
|---------|------|-------|
| Prefix declaration | After `@prefix` / `@base` | Namespace IRIs from the indexed catalog |
| QName prefix | Before `:` in a QName | Declared `@prefix` names |
| QName local | After `prefix:` | Entity short names matching the prefix (classes, properties, individuals) |
| IRI bracket | Inside `<` … | Full IRIs from the catalog (capped at 100 items) |

Results are capped at **100 items** per request.

### `textDocument/codeAction` (v0.11)

Quick fixes are offered when a diagnostic includes encoded `QuickFix` data in `Diagnostic.data`. The client shows a lightbulb on supported codes:

| Diagnostic code | Quick fix behavior |
|-----------------|-------------------|
| `undefined_prefix` | Insert `@prefix` declaration for the missing prefix |
| `missing_label` | Apply patch to add a default label |
| `broken_import` | Remove the broken `owl:imports` line |

Other diagnostic codes (`parse_error`, `duplicate_label`, `orphan_class`, …) publish diagnostics but do not ship quick fixes in v0.11.

Code actions use kind `quickfix` and return workspace edits (insert text, remove line, or apply Turtle patch).

## Custom methods

All custom methods use the `strixonomy/` prefix.

### `strixonomy/indexWorkspace`

Rebuild the workspace catalog.

**Params:** `IndexWorkspaceParams`

```json
{ "workspace_uri": "file:///path/to/workspace", "disk_cache": true }
```

| Field | Type | Description |
|-------|------|-------------|
| `workspace_uri` | string? | Workspace folder URI; omitted uses initialized workspace |
| `disk_cache` | boolean | When `true`, persist parse snapshots under `.strixonomy/cache/` (v0.10+) |

`workspace_uri` is optional; the server uses the initialized workspace folder when omitted. Legacy clients may send `workspaceUri` (camelCase); prefer `workspace_uri` for new integrations.

**Result:** `IndexWorkspaceResult`

| Field | Type | Description |
|-------|------|-------------|
| `stats` | `CatalogStats` | Counts after indexing (see below) |
| `indexed_at` | number | Unix timestamp (seconds) |

**`CatalogStats` fields:**

| Field | Description |
|-------|-------------|
| `ontology_count` | Indexed ontology documents |
| `class_count` | Classes |
| `object_property_count` | Object properties |
| `data_property_count` | Data properties |
| `annotation_property_count` | Annotation properties |
| `individual_count` | Named individuals |
| `axiom_count` | Extracted axioms |
| `annotation_count` | Annotation triples |
| `triple_count` | Total RDF triples (Oxigraph) |
| `error_count` | Documents with parse errors |
| `diagnostic_error_count` | Lint errors |
| `diagnostic_warning_count` | Lint warnings |
| `diagnostic_info_count` | Lint info |

```json
{
  "stats": {
    "ontology_count": 1,
    "class_count": 2,
    "object_property_count": 1,
    "data_property_count": 0,
    "annotation_property_count": 0,
    "individual_count": 1,
    "axiom_count": 2,
    "annotation_count": 4,
    "triple_count": 12,
    "error_count": 0,
    "diagnostic_error_count": 0,
    "diagnostic_warning_count": 0,
    "diagnostic_info_count": 0
  },
  "indexed_at": 1718000000
}
```

### `strixonomy/getCatalogSnapshot`

Return documents, entities, and class hierarchy for the explorer UI.

**Params:** `null`

**Result:** `CatalogSnapshot`

| Field | Type | Description |
|-------|------|-------------|
| `documents` | `OntologyDocument[]` | Indexed files (`id`, `path`, `format`, `parse_status`, …) |
| `entities` | `Entity[]` | All extracted entities |
| `hierarchy` | `ClassHierarchy` | `edges`, `parents`, `children` maps (asserted) |
| `stats` | `CatalogStats`? | **(v0.17+)** Same counts as `indexWorkspace` (when catalog is indexed) |
| `active_ontology_id` | string? | **(v0.17+)** Active ontology id when set via `setActiveOntology` |
| `diagnostics` | `DiagnosticSummary[]` | Lint summaries for explorer |
| `reasoner` | `ReasonerSnapshot` (optional) | Last reasoner run — inferred hierarchy, unsatisfiable classes |

**`Entity` fields:** `iri`, `short_name`, `kind`, `ontology_id`, `labels[]`, `comments[]`, `deprecated`, optional `source_location`.

**Errors:** `NOT_INDEXED` if the workspace has not been indexed.

### `strixonomy/getEntity`

Return detailed entity information for the inspector.

**Params:** `GetEntityParams`

```json
{ "iri": "http://example.org/people#Person" }
```

**Result:** `GetEntityResult` with `detail` (`EntityDetail`):

| Field | Description |
|-------|-------------|
| `entity` | Core `Entity` record |
| `parents` | Parent class IRIs |
| `children` | Child class IRIs |
| `axioms` | `EntityAxiomSummary[]` — structured axiom rows for inspector and Manchester editor |
| `source` | Optional `{ path, line, column }` |
| `editable` | `true` when the entity's declaring file supports patch write-back (`.ttl`, `.obo`, `.owl`/`.rdf`, `.owx` as of v0.21); see [Patch reference](patch-reference.md) |
| `document_path` | Filesystem path to declaring file |

**`EntityAxiomSummary` fields:**

| Field | Description |
|-------|-------------|
| `kind` | `sub_class_of` or `equivalent_class` |
| `display` | Human-readable label (e.g. `SubClassOf ex:Person` or Manchester string) |
| `manchester` | Manchester expression when the axiom is complex (optional) |
| `parent_iri` | Named parent class IRI for simple `SubClassOf` (optional) |
| `editable` | Whether the axiom can be edited via patch write-back |

**Errors:** `NOT_INDEXED`, `ENTITY_NOT_FOUND`

### `strixonomy/query`

Run a SQL-like query against the indexed workspace catalog.

**Params:** `QueryParams`

```json
{ "sql": "SELECT short_name, labels FROM classes" }
```

**Result:** `TabularQueryResult`

| Field | Description |
|-------|-------------|
| `columns` | Column names |
| `rows` | Array of row objects (`column` → string value) |
| `truncated` | `true` when the server row cap was hit (optional) |

**Errors:** `NOT_INDEXED`, `QUERY_FAILED`

### `strixonomy/sparql`

Run SPARQL against the indexed catalog store.

**Params:** `SparqlParams`

```json
{ "query": "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10" }
```

**Result:** `TabularQueryResult` (same shape as `strixonomy/query`).

**Errors:** `NOT_INDEXED`, `QUERY_FAILED`

### `strixonomy/parseManchester`

Parse and validate a Manchester class expression; return normalized text, Turtle fragment, expression tree, diagnostics, and catalog-based completion lists.

**Params:** `ParseManchesterParams`

```json
{
  "expression": "ex:hasRecord some ex:MedicalRecord",
  "axiom_kind": "sub_class_of",
  "entity_iri": "http://example.org/clinic#Patient",
  "document_uri": "file:///path/to/ontology.ttl"
}
```

**Result:** `ParseManchesterResult`

| Field | Description |
|-------|-------------|
| `normalized` | Canonical Manchester string |
| `turtle_fragment` | Turtle axiom fragment for preview |
| `tree` | JSON expression tree for UI |
| `diagnostics` | Parse errors (`PatchDiagnostic[]`) |
| `completions` | `ManchesterCompletions` — classes, object/data properties, XSD datatypes from catalog |

**Errors:** `NOT_INDEXED`, `MANCHESTER_INVALID`

### `strixonomy/applyAxiomPatch`

Apply patch operations to Turtle (`.ttl`), OBO (`.obo`), RDF/XML (`.owl`/`.rdf`), or OWL/XML (`.owx`) documents. See [authoring.md](authoring.md), [OBO authoring](ide/obo-authoring.md), and [OWL/XML write-back](guides/owl-xml-workflow.md).

**Buffer-first (VS Code):** Reads the open document buffer when available, applies patches in memory, updates the buffer, writes disk, then reindexes. See [errors.md](errors.md) for `APPLIED_NOT_INDEXED`.

**Params:** `ApplyAxiomPatchParams`

Legacy patch array (unchanged since v0.11):

```json
{
  "document_uri": "file:///path/to/ontology.ttl",
  "patches": [
    { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
  ],
  "preview_only": false
}
```

**v0.19+ transaction envelope (optional):** same operations wrapped for `strixonomy-edit` apply path. Legacy `patches` arrays remain accepted.

```json
{
  "document_uri": "file:///path/to/ontology.obo",
  "transaction": {
    "changes": [
      { "op": "add_label", "entity_iri": "http://ex#Person", "value": "Human" }
    ]
  },
  "preview_only": false
}
```

Either `patches` or `transaction.changes` may be supplied (not both required). See [migration v0.19](migration/v0.19.md).

**Result:** `ApplyAxiomPatchResult` (flattened patch result + optional entity):

| Field | Description |
|-------|-------------|
| `applied` | `true` if patches were written (false for preview-only or validation failure) |
| `preview_text` | Updated file preview when `preview_only: true` (Turtle or OBO text) |
| `diagnostics` | `PatchDiagnostic[]` on failure (`severity`, `message`) |
| `document_path` | Path to modified file |
| `entity_detail` | Updated `EntityDetail` after successful apply (LSP only) |
| `workspace_edit` | Optional LSP `WorkspaceEdit` for buffer sync (VS Code client applies this) |
| `reindex_warning` | Present when apply succeeded but reindex failed |

**`EntityAxiomSummary` kinds:** `sub_class_of`, `equivalent_class`, `disjoint_class`, `domain`, `range`, `property_chain` (property chains editable via patch ops since v0.12).

**Import ops (v0.11):** `add_import` and `remove_import` — see [patch-reference.md](patch-reference.md) and [Manage Imports](ide/manage-imports.md).

See [patch-reference.md](patch-reference.md) for CLI `ApplyPatchResult` examples and [errors.md](errors.md) for failure codes.

**Errors:** `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `NOT_INDEXED`, `APPLIED_NOT_INDEXED`

### `strixonomy/runReasoner`

Run OWL classification via OntoLogos 1.0.0 (`el`, `rl`, `rdfs`, `dl`, `auto`).

**Params:** `RunReasonerParams`

```json
{ "profile": "el", "auto_detect": true }
```

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `profile` | string | `"el"` | `el`, `rl`, `rdfs`, `dl`, or `auto` |
| `auto_detect` | boolean | `true` | Run profile detection warnings via `ontologos-profile` |

**Result:** `RunReasonerResult`

| Field | Description |
|-------|-------------|
| `profile_used` | Profile that ran |
| `consistent` | `false` when unsatisfiable classes exist |
| `unsatisfiable` | Class IRIs |
| `inferred_edge_count` | Count of inferred-only subsumption edges |
| `new_inferences` | Inferred `SubclassEdge[]` not present in asserted hierarchy |
| `warnings` | Profile / construct warnings |
| `duration_ms` | Wall-clock classification time |
| `snapshot` | Full `ReasonerSnapshot` (also attached to subsequent `getCatalogSnapshot`) |

**Errors:** `NOT_INDEXED`, `REASONER_FAILED`

#### Cancelling a reasoner run (`$/cancelRequest`)

Long `strixonomy/runReasoner` calls honor the standard LSP notification [`$/cancelRequest`](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#cancelRequest) while classification (and enrich steps) run.

**Client:** send a notification (not a request):

```json
{ "jsonrpc": "2.0", "method": "$/cancelRequest", "params": { "id": <request-id-of-runReasoner> } }
```

**Server behavior:**

- Marks the matching in-flight reasoner request cancelled
- Returns a JSON-RPC error for that `runReasoner` call with `data.code` = `REASONER_FAILED` and message `Reasoner run cancelled`
- Does **not** update the reasoner snapshot on cancel (a cancelled run leaves the previous snapshot unchanged)

Cancel only applies to the active reasoner request id. Unrelated inbox messages received during classify are deferred and processed after the wait loop resumes.

### `strixonomy/getExplanation` (v0.15+)

Return an EL/RL/RDFS/DL explanation for an unsatisfiable class. Responses may include **multiple alternative justifications** and **staleness metadata** for cache invalidation.

**Params:** `GetExplanationParams`

```json
{ "class_iri": "http://example.org#Invalid", "profile": "el" }
```

**Result:** `GetExplanationResult`

| Field | Description |
|-------|-------------|
| `class_iri` | Requested class |
| `steps` | Primary justification — ordered `ExplanationStep[]` (`index`, `rule`, `display`, optional IRIs) |
| `text` | Rendered proof text for the primary justification |
| `alternatives` | **(v0.15+)** Additional `ExplanationResult` objects (each with `steps` and `text`) when the reasoner provides multiple justifications |
| `indexed_at` | **(v0.15+)** Unix timestamp (seconds) of the workspace index used when the explanation was generated |
| `content_hash` | **(v0.15+)** Workspace content hash at explanation time — compare on later calls to detect stale explanations after edits |

Clients should treat explanations as **stale** when `indexed_at` or `content_hash` no longer matches the current workspace snapshot. The Strixonomy explanation panel shows a stale warning and offers re-run.

**Errors:** `NOT_INDEXED`, `EXPLANATION_FAILED`

### `strixonomy/getGraph` (v0.7)

Returns graph nodes and edges for visualization webviews.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `graph_kind` | string | `class`, `property`, `object_property`, `data_property`, `individual`, `import`, `dependency`, `neighborhood`, `query_result`, or `refactor_preview` |
| `root_iri` | string? | Required for `neighborhood` / `individual` (single seed); optional elsewhere |
| `root_iris` | string[]? | Multi-root seeds for `query_result` / `refactor_preview` (and optional individual) |
| `depth` | number? | BFS depth for neighborhood/individual/result graphs (default 2, max 5) |
| `include_inferred` | boolean? | Include reasoner subclass edges when snapshot exists |
| `filters` | object? | `ontology_iri`, `hide_deprecated`, `entity_kinds[]`, `namespaces[]`, `relationship_kinds[]`, `search_text` |

**Graph panel (v0.26):** React graph webview supports asserted / inferred-only / combined modes, filtering, search dimming, inferred unsatisfiable overlays, Graph\|List alternate, keyboard navigation, and virtualized render (`onlyRenderVisibleElements`). Query Workbench and Refactor Preview can open result graphs via `query_result` / `refactor_preview`.

**Result:** `{ graph: { nodes, edges, truncated, graph_kind } }` — nodes may include `namespace`, `ontology_iri`, `deprecated`, `unsatisfiable`, `equivalent`.

**Errors:** `NOT_INDEXED`, `GRAPH_FAILED`, `INVALID_PARAMS` (e.g. missing `root_iri` for `neighborhood`)

### `strixonomy/runRobot` (v0.7)

Runs a ROBOT CLI subcommand.

**Params:** `{ subcommand, args?, robot_path? }`

**Result:** `{ exit_code, stdout, stderr }`

**Errors:** `ROBOT_FAILED` (ROBOT missing, non-zero exit, or I/O failure), `INVALID_PARAMS`

### `strixonomy/findUsages` (v0.8)

Return structured IRI usages across the workspace catalog and Turtle text spans.

**Params:** `{ "iri": "http://example.org/people#Person" }`

**Result:** `{ "usages": UsageSummary[] }` — each usage has `iri`, `referenced_iri`, `file`, `line`, `column`, `kind`, `context` (byte ranges are not serialized on the wire).

**Errors:** `NOT_INDEXED`, `INVALID_PARAMS`

### `strixonomy/previewRefactor` (v0.8)

Build a refactor plan without writing files.

**Params:** flattened refactor request — one of:

| `kind` | Fields |
|--------|--------|
| `rename_iri` | `from_iri`, `to_iri` |
| `merge_entities` | `keep_iri`, `merge_iri` |
| `replace_entity` | `from_iri`, `to_iri` |
| `migrate_namespace` | `from_base`, `to_base` |
| `move_entity` | `entity_iri`, `target_file` |
| `extract_module` | `entity_iris[]`, `output_file`, `leave_stub?`, `locality?` |
| `move_axioms` | `entity_iri`, `target_file`, `statement_indexes?`, `exclude_primary?` |
| `merge_ontologies` | `source_paths[]`, `target_file` |
| `flatten_imports` | `ontology_file` |
| `cleanup_imports` | `ontology_file` |

**Result:** `RefactorPlan` — `{ changes: FileChange[], warnings: string[] }` where each change has `path`, `preview_text`, `original_text`, and `hunks`.

**Errors:** `NOT_INDEXED`, `REFACTOR_FAILED`, `INVALID_PARAMS`

### `strixonomy/applyRefactor` (v0.8)

Apply a previously previewed refactor plan. The server re-previews from `request`, verifies the submitted `plan` matches, validates paths against the workspace jail, writes atomically, syncs open buffers, and reindexes.

**Params:** `{ "plan": RefactorPlan, "request": RefactorRequest, "preview_only"?: boolean }`

**Result:** `{ "files_written": number, "reindex_warning"?: string }`

**Errors:** `NOT_INDEXED`, `REFACTOR_FAILED`, `INVALID_PARAMS`

### `strixonomy/semanticDiff` (v0.10+)

Compare semantic catalogs between git refs, directories, or indexed workspace snapshots. Alias: `strixonomy/getSemanticDiff`.

**Params:** `SemanticDiffParams`

| Field | Type | Description |
|-------|------|-------------|
| `left_ref` | string? | Git left ref (default `HEAD`) or `INDEXED` / `CATALOG` for indexed catalog (legacy alias: `WORKSPACE`) |
| `right_ref` | string? | Git right ref (default `WORKTREE`) or `INDEXED` / `CATALOG` for indexed catalog (legacy alias: `WORKSPACE`) |
| `left_path` | string? | Left directory when comparing two paths on disk |
| `right_path` | string? | Right directory |
| `reasoner` | boolean? | Enrich diff with reasoner unsatisfiability changes |
| `format` | string? | `pr-summary` returns `{ "formatted": string }` PR-ready Markdown (v0.13+) |

When both `left_path` and `right_path` are set, git refs are ignored and directories are compared directly (paths must resolve within workspace roots).

**Result:** `{ "diff": DiffResult }` — axiom-level changes, entity additions/removals, breaking-change flags. With `format: "pr-summary"`, also includes `formatted` Markdown string. See [Semantic diff guide](ide/semantic-diff.md).

**Errors:** `INVALID_PARAMS` (bad refs, paths outside workspace, git errors), `NOT_INDEXED` (indexed-catalog ref before first index), `REASONER_FAILED` (when `reasoner: true` enrichment fails)

### `strixonomy/listSqlSchema` (v0.13+)

Returns SQL virtual table metadata for the Query Workbench schema browser.

**Params:** none (uses indexed workspace catalog)

**Result:** `{ "tables": [{ "name": string, "columns": [{ "name": string, "type": string }] }] }`

Includes core tables (`classes`, `properties`, …) and Horned-OWL axiom projections. See [sql-reference.md](sql-reference.md).

**Errors:** `NOT_INDEXED`

### `textDocument/semanticTokens/full` (v0.13+)

Standard LSP semantic tokens for **Turtle** (`.ttl`) and **OBO** (`.obo`) open documents.

**Token types:** `namespace`, `iri`, `keyword`, `comment`, `string`

Requires document text in the LSP open-document buffer (unsaved buffers supported).

### `strixonomy/listPlugins` (v0.14+)

Returns discovered workspace plugins from `.strixonomy/plugins/*.toml` plus built-in registration metadata.

**Params:** none (uses indexed workspace root)

**Result:** `{ "plugins": PluginDescriptor[] }`

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Plugin id |
| `name` | string | Display name |
| `version` | string | Manifest version |
| `kind` | string | `validator`, `exporter`, `workflow`, `reasoner`, `query`, `refactor`, `graph`, … |
| `manifest_path` | string | Absolute path to manifest TOML |
| `capabilities` | object | Legacy capability flags (`build`, `validate`, `release`, `diagnostics`, `export`) where declared |
| `permissions` | array | Declared permission strings (`workspace.read`, `workspace.write`, `external_process`, …) |
| `api_version` | string? | Manifest API version (e.g. `"1"` for SDK 1.0) |
| `state` | string | Lifecycle state (**SDK 1.0**) |
| `enabled` | boolean | Whether the plugin is enabled (**SDK 1.0**) |
| `depends_on` | array | Plugin ids this plugin depends on (**SDK 1.0**) |
| `activation` | string | Activation policy (**SDK 1.0**) |
| `ui.commands` | array | `{ id, title, scope? }` palette contributions |
| `ui.views` | array | `{ id, title }` dockable view contributions |
| `ui.inspector_cards` | array | `{ id, title, applies_to, command? }` inspector slots |
| `in_process` | boolean | `true` for built-in reference plugins |

**Errors:** `NOT_INDEXED`, `INDEX_FAILED` (discovery/host failure)

!!! note "Error codes"
    Per-method error codes are documented in prose and [errors.md](errors.md). A machine-checkable enum in `lsp-protocol.schema.json` is a follow-up (not yet shipped).

### `strixonomy/listCommands` (v0.17+)

Return stable command metadata for menus, toolbars, and enablement.

**Params:** none

**Result:** `{ "commands": CommandDescriptor[] }`

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Command id (e.g. `strixonomy.newOntology`) |
| `title` | string | Display title |
| `category` | string | Menu category (`File`, `Edit`, …) |
| `enablement` | string[] | Predicates: `always`, `has_ontology`, `is_dirty`, `has_selection`, `reasoner_running`, `reasoner_idle`, `can_edit_selection` |
| `undo_label` | string? | Semantic undo label for edits |
| `dialog_id` | string? | Associated dialog schema id |

**Errors:** none (empty `commands` if the server has no registered descriptors)

### `strixonomy/getWorkspaceUiState` (v0.17+)

Return enablement inputs for the command registry.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `selection_iri` | string? | Focused entity IRI from the client |
| `dirty_document_count` | number | Client-reported dirty document count |
| `active_ontology_id` | string? | Preferred active ontology id |

**Result:** `WorkspaceUiState` (`has_ontology`, `is_dirty`, `has_selection`, `selection_editable`, `reasoner_*`, `active_ontology_id`, optional `stats`)

**Errors:** none

### `strixonomy/getDialogSchema` (v0.17+)

**Params:** `{ "dialog_id": string }`

**Result:** `{ "schema": DialogSchema }` with `id`, `title`, `fields[]`, `primary_action`.

Known dialog ids include `new_ontology`, `export_ontology`, `save_as`, `prefix_manager`, `ontology_metadata`, `search`, `metrics`, `delete_entity`, `reasoner_settings`, `preferences`, `import`, `rename`.

**Errors:** `INVALID_PARAMS` (unknown `dialog_id`)

### `strixonomy/createOntology` (v0.17+)

Scaffold a new Turtle or OBO ontology file under the workspace.

**Params:** `{ "path", "ontology_iri", "version_iri"?, "format"?, "prefixes"? }`

**Result:** `{ "path", "ontology_iri" }`

**Errors:** `INVALID_PARAMS` (workspace not initialized, path outside roots, file already exists, unsafe path), `INDEX_FAILED` (I/O failure creating the file)

### `strixonomy/exportOntology` (v0.17+)

Export/convert an ontology via ROBOT `convert`, with same-format copy fallback when ROBOT is unavailable.

**Params:** `{ "source_path", "output_path", "format"? }`

**Result:** `{ "output_path", "success", "logs"? }` — `success` may be `false` when ROBOT exits non-zero (still an LSP success response).

**Errors:** `INVALID_PARAMS` (path outside workspace), `ROBOT_FAILED` (ROBOT unavailable and formats differ, or copy failure)

### `strixonomy/setActiveOntology` (v0.17+)

Set the active ontology id used for new-axiom targeting.

**Params:** `{ "ontology_id": string }` (document id, path, or base IRI)

**Result:** `{ "active_ontology_id": string }`

**Errors:** `ENTITY_NOT_FOUND` (id not in the indexed catalog), `NOT_INDEXED`

### `strixonomy/deleteImpact` (v0.17+)

Preview delete impact for an entity.

**Params:** `{ "entity_iri": string }`

**Result:** `{ "entity_iri", "usage_count", "axiom_count", "referencing_entities", "warnings" }`

**Errors:** `NOT_INDEXED`

### `strixonomy/runPlugin` (v0.14+, views v0.15)

Run a plugin validate/export/workflow/**ui_view** action.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `plugin_id` | string | Plugin id from manifest |
| `action` | string? | `validate` (default), `export`, `workflow`, or **`ui_view`** (v0.15+) |
| `step` | string? | Workflow step when `action` is `workflow` |
| `view_id` | string? | **(v0.15+)** View id from `ui.views` when `action` is `ui_view` |

**Result:** `{ "diagnostics": DiagnosticSummary[], "output_paths": string[], "logs": string?, "view_html": string?, "success": boolean }`

`view_html` is populated for `ui_view` actions — HTML rendered in the plugin view panel.

Plugin diagnostics use `code` values like `plugin:<id>:<code>` and LSP `source` `strixonomy-plugin:<id>`. See [errors.md](errors.md#plugin-diagnostic-codes-v014).

**Errors:** `NOT_INDEXED`, `INDEX_FAILED` (plugin not found, missing permission, unsupported action, subprocess failure, or export error)

`getCatalogSnapshot` includes plugin diagnostics merged after index (same wire codes).

### `strixonomy/dlQuery` (v0.24+)

Run a Manchester **class expression** DL Query (instances / subclasses / superclasses / equivalents). Named classes use hierarchy + realization; anonymous expressions inject a temporary equivalent class (never written to disk).

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `expression` | string | Manchester class expression |
| `profile` | string? | Reasoner profile (default `dl`) |
| `mode` | string? | `inferred` (default) or `asserted` |
| `document_uri` | string? | Optional document for prefix context |

**Result:**

| Field | Type | Description |
|-------|------|-------------|
| `expression` | string | Input expression |
| `normalized` | string | Normalized form |
| `query_class_iri` | string | Named class or temporary query class IRI |
| `subclasses` / `superclasses` / `equivalents` / `instances` | string[] | Hit IRIs |
| `profile` / `mode` | string | Profile and query mode used |
| `duration_ms` | integer | Elapsed time |
| `warnings` / `diagnostics` | string[] | Optional messages |

**Errors:** `NOT_INDEXED`, `REASONER_FAILED`, `INVALID_PARAMS`, `MANCHESTER_INVALID`

Honesty notes: [DL Query guide](guides/dl-query.md) · cookbook: [examples/dl-query.md](examples/dl-query.md).

### `strixonomy/search` (v0.24+)

Workspace entity search for QuickPick / live search UIs.

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `query` | string | Search text (short name / IRI substring) |
| `limit` | integer? | Max hits (default 100, capped at 500) |

**Result:** `{ entities: EntityDetail[] }`

**Errors:** `NOT_INDEXED`

### `strixonomy/checkInstance` (v0.23+)

Check whether an individual is an instance of a class (ABox).

**Params:**

| Field | Type | Description |
|-------|------|-------------|
| `individual_iri` | string | Individual IRI |
| `class_iri` | string | Class IRI |
| `profile` | string? | Reasoner profile (default `rl`) |

**Result:** `{ individual_iri, class_iri, entailed, profile_used, duration_ms }`

**Errors:** `NOT_INDEXED`, `REASONER_FAILED`, `INVALID_PARAMS`

!!! note "Realization is CLI-only"
    There is no LSP `strixonomy/realize` method. Use `strixonomy realize` for bulk realization in CI — [CLI reference](cli-reference.md) · [realize cookbook](examples/realize.md). Instance checks use this method or `strixonomy check-instance`.

### `strixonomy/listSwrlRules` (v0.23+)

List SWRL rules found in open Turtle buffers and indexed `.ttl` documents (`strixonomy:swrlRule` JSON).

**Params:** none

**Result:** `{ rules: SwrlRuleListItem[] }` — each item includes id/label, body/head counts, enabled flag, optional document URI

**Errors:** none (returns an empty list when no rules are present; indexing is recommended but not required for open buffers)

### `strixonomy/validateSwrlRule` (v0.23+)

Validate a SWRL rule JSON string (builtins / DLSafe diagnostics). Does not write the ontology.

**Params:** `{ rule_json: string }`

**Result:** `{ diagnostics: SwrlDiagnostic[] }`

**Errors:** `INVALID_PARAMS` (unparseable rule JSON)

### `strixonomy/parseSwrlRule` (v0.23+)

Parse SWRL rule JSON into the Strixonomy SWRL IR.

**Params:** `{ rule_json: string }`

**Result:** `{ rule: SwrlRule }`

**Errors:** `INVALID_PARAMS` (unparseable rule JSON)

## Structured errors

Custom method failures return `LspErrorPayload` in the JSON-RPC error `data` field. Full catalog: [errors.md](errors.md).

| Field | Description |
|-------|-------------|
| `code` | Machine-readable code (`NOT_INDEXED`, `INVALID_PARAMS`, `ENTITY_NOT_FOUND`, `PATCH_INVALID`, `UNSUPPORTED_FORMAT`, `INDEX_FAILED`, `QUERY_FAILED`, `MANCHESTER_INVALID`, `APPLIED_NOT_INDEXED`, `REASONER_FAILED`, `EXPLANATION_FAILED`, …) |
| `message` | Human-readable message |
| `recoverable` | Whether the client can retry |
| `user_action` | Suggested user action (optional) |

## Still planned (see LSP_SPEC)

These remain **target** items in [LSP_SPEC.md](design/LSP_SPEC.md) — they are **not** shipped as first-class LSP methods today:

- Dedicated LSP `strixonomy/realize` (use CLI `strixonomy realize` / `check-instance` instead)
- Additional standard LSP surfaces called out as future in LSP_SPEC (beyond completion, codeAction, semantic tokens, rename/prepareRename, hover, definition)

**Already shipped** (do not treat as gaps): `textDocument/completion` (v0.11), semantic tokens + `listSqlSchema` (v0.13), `prepareRename` / rename for IRI/QName, semantic diff (`strixonomy/semanticDiff` + [Semantic diff guide](ide/semantic-diff.md)).
