# strixonomy-robot

> Part of **Strixonomy** (semantic workspace engine).

Thin wrappers around the [ROBOT](https://github.com/ontodev/robot) CLI for Strixonomy.

## Install

```toml
strixonomy-robot = "0.26"
```

## CLI

```bash
strixonomy robot validate ./ontology.obo
strixonomy robot merge --inputs a.owl b.owl --output merged.owl
strixonomy robot report ./ontology --report report.tsv
```

## Library

```rust
use strixonomy_robot::{robot_validate, run_robot};

let output = robot_validate(None, path.as_path())?;
```

## License

MIT OR Apache-2.0
