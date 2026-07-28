# Your next steps (after First success)

You finished [First success](first-success.md). This page is a **~15 minute day-2 path** with three copy-paste wins: query → reason → validate. Keep the same tutorial folder from First success (or re-download the samples).

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

## 3. Validate (~5 min)

Most IDE users validate in the editor:

1. Open **View → Problems** (or the Strixonomy diagnostics tree).
2. Confirm tutorial samples show no fatal parse errors.

**Success looks like:** no blocking ontology errors for `example.ttl` / `demo.obo`.

### Optional: CLI validate (CI teams)

Skip this on a laptop unless you need CI parity. Prefer the **Linux x64** release tarball — [CI integration](../ci-integration.md):

```bash
VERSION=0.27.0
BIN="strixonomy-v${VERSION}-x86_64-unknown-linux-gnu"
./${BIN} validate /path/to/strixonomy-tutorial
```

**macOS / Windows:** cold `cargo install` is **15–30+ minutes** — only for CI authors who need the CLI locally: [Install CLI](install-cli.md).

```bash
cargo install strixonomy-cli --locked --version 0.27.0
strixonomy validate /path/to/strixonomy-tutorial
```

**Success looks like:** exit code `0` and a short OK / summary line.

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

**Next:** [Feature tour](../ide/feature-tour.md) (week-1 IDE depth).

- [Documentation index](../documentation-index.md)
- [Troubleshooting](../troubleshooting.md)
- [Examples](../examples/index.md)
- [Uninstall](uninstall.md)
