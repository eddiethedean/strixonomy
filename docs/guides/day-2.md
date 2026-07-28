# Your next steps (after First success)

You finished [First success](first-success.md). This page is a **~15 minute day-2 path** with three copy-paste wins: query → reason → validate. Keep the same tutorial folder from First success (or re-download the samples).

!!! tip "Docs vs Marketplace"
    Read the Docs `latest` may describe work **after** the last tag. Install pins follow [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE) (currently **0.27.0**). See [Versions & channels](versions-and-channels.md).

## Prerequisites

- Strixonomy IDE installed ([First success](first-success.md) step 1)
- A folder open with at least `example.ttl` (First success samples)

## 1. Query (~5 min)

1. Open the **Strixonomy** activity bar → **Query Workbench** (or Command Palette → **Strixonomy: Open Query Workbench**).
2. Choose **SQL** mode.
3. Paste:

```sql
SELECT short_name FROM classes WHERE deprecated = 'false'
```

4. Run the query.

**Success looks like:** a result table that includes `Person` (and other classes from `example.ttl`). Boolean filters use **string** literals `'true'` / `'false'` — see [SQL reference](../sql-reference.md).

Optional SPARQL: switch to SPARQL mode and try a simple `SELECT ?s WHERE { ?s a <http://www.w3.org/2002/07/owl#Class> } LIMIT 10` pattern — [SPARQL reference](../sparql-reference.md). Catalog SQL has no `LIMIT`; use SPARQL for graph patterns.

## 2. Reason (~5 min)

1. Command Palette → **Strixonomy: Run Reasoner** (or open the Reasoner panel).
2. Pick profile **`el`** (fast) or **`auto`**.
3. Run classification.

**Success looks like:** a completed classification with no hard failure for the tutorial samples; you can toggle asserted/inferred hierarchy in the explorer. Details: [Reasoner guide](reasoner.md).

## 3. Validate outside the editor (optional, ~5 min)

Most IDE users stop after steps 1–2. For CI-like validation:

**Linux x64 (preferred):** [CI integration](../ci-integration.md) — download the release tarball, then:

```bash
./strixonomy validate /path/to/strixonomy-tutorial
```

**macOS / Windows:** [Install CLI](install-cli.md) (15–30+ min cold compile), then:

```bash
cargo install strixonomy-cli --locked --version 0.27.0
strixonomy validate /path/to/strixonomy-tutorial
```

**Success looks like:** exit code `0` and a short OK / summary line (no fatal parse errors).

## Edit more (when you have time)

| Task | Where |
|------|-------|
| Labels, parents, create/delete | Entity Inspector — [Authoring](../authoring.md) |
| Complex class expressions | Manchester editor — [Manchester](../ide/manchester-editor.md) |
| OBO terms | [OBO authoring](../ide/obo-authoring.md) |
| Imports | [Manage Imports](../ide/manage-imports.md) |

## Fit check before a larger pilot

1. [Known limitations](../known-limitations.md)
2. [What ships today](../SHIPPED.md)
3. [Protégé vs Strixonomy](protege-decision.md) if you already use Protégé
4. [Product identity](product-identity.md) — IDE vs engine naming

## Related

- [Feature tour](../ide/feature-tour.md)
- [Documentation index](../documentation-index.md)
- [Troubleshooting](../troubleshooting.md)
- [Examples](../examples/index.md)
