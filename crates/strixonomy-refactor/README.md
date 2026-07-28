# strixonomy-refactor

> Part of **Strixonomy** (semantic workspace engine).

Workspace refactoring for Strixonomy — find usages, safe IRI rename, namespace migration, move entity, and extract module.

## Usage

```rust
use strixonomy_catalog::IndexBuilder;
use strixonomy_refactor::{find_usages, preview_rename_iri, apply_refactor_plan};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let usages = find_usages(&catalog, "http://example.org/people#Person");
let plan = preview_rename_iri(&catalog, "http://example.org/people#Person", "http://example.org/people#Human")?;
apply_refactor_plan(&plan, false, workspace_root)?;
```

CLI: `strixonomy refactor usages|rename|migrate-namespace|move|extract`.

## Install

```toml
strixonomy-refactor = "0.28"
```

## Documentation

- [Refactoring guide](https://strixonomy.readthedocs.io/en/latest/guides/refactoring/)
- [Rust & CLI docs](https://strixonomy.readthedocs.io/en/latest/guides/rust-crates/)

## License

MIT OR Apache-2.0
