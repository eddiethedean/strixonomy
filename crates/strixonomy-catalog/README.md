# strixonomy-catalog

> Part of **Strixonomy** (semantic workspace engine).

Semantic catalog and index builder for [Strixonomy](https://github.com/eddiethedean/strixonomy).

## Install

```toml
strixonomy-catalog = "0.27"
```

Supports incremental rebuilds (content-hash reuse), optional disk cache (`.strixonomy/cache/`), and config fingerprinting for CI.

## Quick example

```rust
use strixonomy_catalog::IndexBuilder;

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
println!("{:?}", catalog.data().stats());
```

## Documentation

- [Rust & CLI docs](https://strixonomy.readthedocs.io/en/latest/guides/rust-crates/)
- [Rust library guide](https://strixonomy.readthedocs.io/en/latest/guides/rust-library/)
- [SQL reference](https://strixonomy.readthedocs.io/en/latest/sql-reference/)
- [docs.rs](https://docs.rs/strixonomy-catalog)

## License

MIT OR Apache-2.0
