# strixonomy-docs

> Part of **Strixonomy** (semantic workspace engine). **New in v0.11.**

Export ontology workspace catalogs to Markdown and HTML documentation.

## CLI

```bash
strixonomy docs ./ontologies --format markdown --output ./docs-out
strixonomy docs ./ontologies --format html --output ./docs-out
```

## Library

```rust
use strixonomy_docs::{export_workspace, ExportFormat, ExportOptions};
use strixonomy_catalog::OntologyCatalog;

let catalog = /* from Workspace */;
export_workspace(&catalog, ExportOptions::markdown("./out"))?;
```

## Documentation

- [CLI reference](https://strixonomy.readthedocs.io/en/latest/cli-reference/)
- [What ships today](https://strixonomy.readthedocs.io/en/latest/SHIPPED/)

## License

MIT OR Apache-2.0
