# strixonomy-query

> Part of **Strixonomy** (semantic workspace engine).

SQL-like virtual tables and SPARQL query engine for [Strixonomy](https://github.com/eddiethedean/strixonomy).

## Install

```toml
strixonomy-query = "0.28"
```

## Quick example

```rust
use strixonomy_catalog::IndexBuilder;
use strixonomy_query::query_catalog;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let result = query_catalog(&catalog, "SELECT short_name FROM classes")?;
```

## Documentation

- [Rust & CLI docs](https://strixonomy.readthedocs.io/en/latest/guides/rust-crates/)
- [SQL reference](https://strixonomy.readthedocs.io/en/latest/sql-reference/)
- [SPARQL reference](https://strixonomy.readthedocs.io/en/latest/sparql-reference/)
- [Query cookbook](https://strixonomy.readthedocs.io/en/latest/examples/queries/)
- [docs.rs](https://docs.rs/strixonomy-query)

## License

MIT OR Apache-2.0
