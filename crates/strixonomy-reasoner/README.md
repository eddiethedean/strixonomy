# strixonomy-reasoner

> Part of **Strixonomy** (semantic workspace engine).

Thin [OntoLogos](https://github.com/eddiethedean/ontologos) **1.0.0** facade for Strixonomy — EL, RL, RDFS, DL, and auto-routed classification, hierarchy merge, and EL-first explanations.

## Usage

```rust
use strixonomy_catalog::IndexBuilder;
use strixonomy_reasoner::{classify, ReasonerId, WorkspaceInputLoader};

let catalog = IndexBuilder::new().workspace("fixtures").build()?;
let input = WorkspaceInputLoader::new("fixtures").load()?;
let result = classify(ReasonerId::Dl, &input, false)?;
// `consistent` is true when no *named class* is unsatisfiable (not full ABox consistency).
println!("consistent: {}", result.consistent);
```

CLI equivalent: `strixonomy classify <workspace> --profile dl`.

## Install

```toml
strixonomy-reasoner = "0.27"
```

## Profiles

| Profile | Status |
|---------|--------|
| `el`, `rl`, `rdfs`, `dl`, `auto` | Shipped (OntoLogos 1.0) |

## Documentation

- [Rust & CLI docs](https://strixonomy.readthedocs.io/en/latest/guides/rust-crates/)
- [Reasoner guide](https://strixonomy.readthedocs.io/en/latest/guides/reasoner/)
- [REASONER_SPEC](https://strixonomy.readthedocs.io/en/latest/design/REASONER_SPEC/)

## License

MIT OR Apache-2.0
