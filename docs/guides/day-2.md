# Your next steps (after First success)

You finished [First success](first-success.md). This page is the **day-2 IDE path** — edit → query → reason → save/CI — without diving into the full capability matrix yet.

!!! tip "Docs vs Marketplace"
    Read the Docs `latest` may describe work **after** the last tag. Install pins and Marketplace builds follow [`docs/TAGGED_RELEASE`](https://github.com/eddiethedean/strixonomy/blob/main/docs/TAGGED_RELEASE) (currently **0.26.2**). See [Versions & channels](versions-and-channels.md).

## 1. Edit more

| Task | Where |
|------|-------|
| Labels, parents, create/delete | Entity Inspector — [Authoring](../authoring.md) |
| Complex class expressions | Manchester editor — [Manchester](../ide/manchester-editor.md) |
| OBO terms | [OBO authoring](../ide/obo-authoring.md) |
| Imports | [Manage Imports](../ide/manage-imports.md) |

## 2. Query

| Mode | Try |
|------|-----|
| SQL (catalog subset) | `SELECT short_name FROM classes` — [Query Workbench](../ide/query-workbench.md) |
| SPARQL | Graph patterns — [SPARQL reference](../sparql-reference.md) |
| DL Query | Manchester class expression — [DL Query honesty](dl-query.md) |

## 3. Reason

1. **Strixonomy: Run Reasoner** (or Reasoner panel) with profile `el` / `dl` / `auto`.
2. Inspect inferred hierarchy / clashes.
3. Optional: [Realize cookbook](../examples/realize.md).

## 4. Save and CI (optional)

Most IDE users never install the CLI. For automation:

- Linux x64: [CI integration](../ci-integration.md) (release tarball)
- Pin: `cargo install strixonomy-cli --locked --version 0.26.2` — [Install](../install.md)

## Fit check before a larger pilot

1. [Known limitations](../known-limitations.md)
2. [What ships today](../SHIPPED.md) (capability matrix)
3. [Protégé vs Strixonomy](protege-decision.md) if you already use Protégé

## Related

- [Feature tour](../ide/feature-tour.md)
- [Documentation index](../documentation-index.md) (reading-order map)
- [Troubleshooting](../troubleshooting.md)
