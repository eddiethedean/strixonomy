# strixonomy-cli

> Part of **Strixonomy** (semantic workspace engine).

Command-line interface for [Strixonomy](https://github.com/eddiethedean/strixonomy) — index ontology workspaces, run SQL/SPARQL queries, validate, patch Turtle, OBO, RDF/XML, and OWL/XML files, and classify with OntoLogos.

## Prerequisites

- Rust **1.88+** (`rustup update stable`; check with `rustc --version`)
- `~/.cargo/bin` on your `PATH` after `cargo install`

## Install (pinned)

```bash
cargo install strixonomy-cli --locked --version 0.27.0
```

## Linux x64 without Rust

Download `strixonomy-v*-x86_64-unknown-linux-gnu.tar.gz` from [GitHub Releases](https://github.com/eddiethedean/strixonomy/releases) and verify `SHA256SUMS` — [release integrity](https://strixonomy-vs.readthedocs.io/en/latest/release-integrity/).

## Quick example

```bash
strixonomy inspect /path/to/ontologies
strixonomy query /path/to/ontologies "SELECT short_name FROM classes"
strixonomy validate /path/to/ontologies
strixonomy classify /path/to/ontologies --profile el --format json

# Requires a git repository at /path/to/repo
strixonomy diff /path/to/repo HEAD..WORKTREE
strixonomy diff --left-ref main --right-ref feature --format markdown --breaking-only
strixonomy docs /path/to/ontologies --format markdown --output ./docs-out
```

Semantic diff compares catalogs from git refs, directories, or workspace snapshots. See [semantic diff guide](https://strixonomy-vs.readthedocs.io/en/latest/strixonomy/semantic-diff/).

## Documentation

- [Rust & CLI docs](https://strixonomy-vs.readthedocs.io/en/latest/guides/rust-crates/)
- [CLI reference](https://strixonomy-vs.readthedocs.io/en/latest/cli-reference/)
- [Install CLI & CI (detail)](https://strixonomy-vs.readthedocs.io/en/latest/install-cli-ci/)
- [What ships today](https://strixonomy-vs.readthedocs.io/en/latest/SHIPPED/)

See [crates.io](https://crates.io/crates/strixonomy-cli) for the latest published version.

## License

MIT OR Apache-2.0
