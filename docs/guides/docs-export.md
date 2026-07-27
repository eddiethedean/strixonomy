# Documentation export

Generate navigable Markdown or HTML documentation from an indexed Strixonomy workspace.

## CLI

```bash
# All ontologies in workspace → Markdown
strixonomy docs ./fixtures --format markdown --output /tmp/onto-docs

# Single ontology by IRI or document id
strixonomy docs ./fixtures --format html --output /tmp/onto-docs \
  --ontology-id http://example.org/people

# Open index
open /tmp/onto-docs/index.html   # HTML
open /tmp/onto-docs/index.md     # Markdown
```

## Rust API

```rust
use strixonomy::{Workspace, docs::{export_workspace, ExportOptions}};

let ws = Workspace::open("./fixtures")?;
export_workspace(
    ws.catalog(),
    ExportOptions::html("/tmp/onto-docs").with_ontology_id("http://example.org/people"),
)?;
```

## Output layout

| File | Contents |
|------|----------|
| `index.md` / `index.html` | Links to per-ontology pages; **class hierarchy** and **property index** (v0.13+) |
| `{slug}.md` / `{slug}.html` | Entity tables, labels, comments, class parents, import list |

## CI / team docs

Run after `strixonomy index` or on a clean checkout:

```bash
strixonomy docs . --format markdown --output docs/generated/ontology
```

Commit or publish `docs/generated/` as part of your release pipeline.

## Related

- [SHIPPED.md](../SHIPPED.md) — capability matrix
- [migration/v0.13.md](../migration/v0.13.md) — hierarchy + property index sections
