# DEPENDENCY_MATRIX.md

> **Canonical inventory** of external Rust crates and CLIs used by Strixonomy/Strixonomy.
> Policy: [ADR-0016](adr/0016-dependency-first-implementation.md). Do not add dependencies without updating this file.

**Last updated:** 2026-06-24

## How to read this table

| Column | Meaning |
|--------|---------|
| **Capability** | Product feature area |
| **Chosen dependency** | Crate or external tool Strixonomy delegates to |
| **Version pin** | Target for new workspace deps (workspace `Cargo.toml` may lag until integration) |
| **Strixonomy facade** | Thin `strixonomy-*` crate or module |
| **Strixonomy owns** | Logic that stays in-tree |
| **ADR** | Decision record |
| **Phase** | Roadmap milestone |

---

## Master matrix

| Capability | Chosen dependency | Version pin | Strixonomy facade | Strixonomy owns | ADR | Phase |
|------------|-------------------|-------------|------------------|----------------|-----|-------|
| RDF parse + triple store + SPARQL | [`oxigraph`](https://crates.io/crates/oxigraph) | `0.4` (workspace) | `strixonomy-parser`, `strixonomy-query` | Entity extraction, catalog mapping, limits | [0003](adr/0003-use-oxigraph.md) | v0.2 |
| RDF I/O ecosystem peer | [`oxrdf`](https://crates.io/crates/oxrdf), [`oxrdfio`](https://crates.io/crates/oxrdfio) | via Oxigraph / OntoLogos | — | — | — | — |
| SQL-like query parse | [`sqlparser`](https://crates.io/crates/sqlparser) | `0.53` (workspace) | `strixonomy-query` | Virtual tables, projection, `WHERE` | [0011](adr/0011-use-sqlparser-for-sql.md) | v0.2 |
| SQL joins / aggregations (v1.0) | Extend `sqlparser` virtual tables first; [`datafusion`](https://crates.io/crates/datafusion) if scope exceeds hand-rolled | TBD at v1.0 | `strixonomy-query` | Table join logic or DataFusion adapter | [0011](adr/0011-use-sqlparser-for-sql.md) | v1.0 |
| Workspace scan + gitignore | [`ignore`](https://crates.io/crates/ignore) | `0.4` (workspace) | `strixonomy-core` | Path jail, hashing, format detection | [0005](adr/0005-local-first-by-default.md) | v0.2 |
| LSP protocol | [`lsp-server`](https://crates.io/crates/lsp-server), [`lsp-types`](https://crates.io/crates/lsp-types) | `0.7` / `0.97` | `strixonomy-lsp` | Custom methods, catalog JSON, path sandbox | [0007](adr/0007-language-server-boundary.md) | v0.2 |
| Parse / import / prefix diagnostics | [`oxigraph`](https://crates.io/crates/oxigraph) parse errors + catalog rules | `0.4` | `strixonomy-diagnostics` | Lint rules: duplicate/missing labels, orphans, broken imports | [0016](adr/0016-dependency-first-implementation.md) | v0.3 |
| OBO validation (future) | [`fastobo-validator`](https://crates.io/crates/fastobo-validator) | `0.4` | `strixonomy-diagnostics` | Map violations to LSP diagnostics | [0016](adr/0016-dependency-first-implementation.md) | v0.7b |
| OWL axiom model + round-trip | [`horned-owl`](https://crates.io/crates/horned-owl) | `1.4` | `strixonomy-owl` | Catalog bridge, patch write-back, consistency tests | [0013](adr/0013-dual-stack-oxigraph-horned-owl.md) | v0.4.0 |
| Manchester functional syntax | in-house (`strixonomy-owl` Manchester parser) | — | `strixonomy-owl` | LSP range mapping, webview wire format | [0016](adr/0016-dependency-first-implementation.md) | v0.5 |
| Manchester editor assist (optional) | [`owl-ms-language-server`](https://crates.io/crates/owl-ms-language-server) | evaluate at v0.5 | extension / LSP | Embed vs subprocess decision at implementation | [0016](adr/0016-dependency-first-implementation.md) | v0.5 |
| Reasoner orchestration | [OntoLogos](https://github.com/eddiethedean/ontologos) (`ontologos-*`) | `0.9` → `1.0` | `strixonomy-reasoner` | `ReasonerAdapter` trait, cache, LSP JSON | [0015](adr/0015-adopt-ontologos-reasoner.md) | v0.6 / v1.0 |
| Reasoner file load | [`ontologos-parser`](https://crates.io/crates/ontologos-parser) | `0.9` → `1.0` | `strixonomy-reasoner` | Workspace path → ontology input bridge | [0015](adr/0015-adopt-ontologos-reasoner.md) | v0.6 |
| Reasoning transitive (via OntoLogos) | [`reasonable`](https://crates.io/crates/reasonable), [`horned-owl`](https://crates.io/crates/horned-owl), [`petgraph`](https://crates.io/crates/petgraph) | via OntoLogos | — | Do not depend directly | [0015](adr/0015-adopt-ontologos-reasoner.md) | v0.6 |
| Graph structure for viz | [`petgraph`](https://crates.io/crates/petgraph) | `0.8` | `strixonomy-lsp` / export API | JSON graph for React webview; layout in `webview-ui` | [0016](adr/0016-dependency-first-implementation.md) | v0.7 |
| Webview panel UI | [`react`](https://www.npmjs.com/package/react), [`vite`](https://www.npmjs.com/package/vite) | evaluate at v0.7a | `extension/webview-ui` | Typed `postMessage` protocol, panel host, CSP | [0017](adr/0017-react-webview-ui.md) | v0.7a+ |
| OBO read/write | [`fastobo`](https://crates.io/crates/fastobo), [`fastobo-owl`](https://crates.io/crates/fastobo-owl) | `0.15` / `0.3` | `strixonomy-parser` or `strixonomy-owl` | Catalog mapping, OBO id ↔ IRI | [0016](adr/0016-dependency-first-implementation.md) | v0.7b |
| ROBOT release pipelines | [ROBOT](https://github.com/ontodev/robot) CLI | external | `strixonomy-robot` | Subprocess wrapper, exit codes, settings | [OBO_ROBOT_SPEC](OBO_ROBOT_SPEC.md) | v0.7b |
| File-watch reindex | [`notify`](https://crates.io/crates/notify) or `ontologos-watch` | `9` / via OntoLogos | `strixonomy-lsp` | Debounce, catalog invalidation | [0015](adr/0015-adopt-ontologos-reasoner.md) | v0.9 |
| Git branch / commit inputs | [`git2`](https://crates.io/crates/git2) | `0.21` | `strixonomy-diff` | Checkout snapshots, diff presentation | [0016](adr/0016-dependency-first-implementation.md) | v0.9 |
| Semantic axiom diff | [`horned-owl`](https://crates.io/crates/horned-owl) (in-house diff logic) | `1.4` | `strixonomy-diff` | Breaking-change heuristics, PR markdown | [0009](adr/0009-semantic-diff-as-core-feature.md) | v0.9 |
| Docs export Markdown | [`pulldown-cmark`](https://crates.io/crates/pulldown-cmark) | `0.13` | `strixonomy-docs` | Entity page templates, TOC | [0016](adr/0016-dependency-first-implementation.md) | v0.9 |
| Docs export HTML | [`minijinja`](https://crates.io/crates/minijinja) | `2` | `strixonomy-docs` | Template files, asset bundling | [0016](adr/0016-dependency-first-implementation.md) | v0.9 |
| SHACL validation (P1) | [`rudof`](https://crates.io/crates/rudof) (shapes-rs) | `0.1` | plugin / `strixonomy-diagnostics` | Shape path config, LSP diagnostic mapping | [SHACL_SPEC](SHACL_SPEC.md) | v1.0 P1 |

---

## Explicitly excluded dependencies

| Crate / tool | Reason |
|--------------|--------|
| ELK, HermiT, Pellet (JVM) | [ADR-0014](adr/0014-rust-native-reasoners-only.md) |
| `whelk-rs` direct | Use `ontologos-el` via [ADR-0015](adr/0015-adopt-ontologos-reasoner.md) |
| `reasonable` direct | Use `ontologos-rl` / `ontologos-rdfs` via ADR-0015 |
| `owl-dl-core` / rustdl | Single DL stack = OntoLogos `ontologos-dl` |
| Reimplementing ROBOT merge/report | [OBO_ROBOT_SPEC](OBO_ROBOT_SPEC.md) |
| Reimplementing triple store | Oxigraph covers RDF/SPARQL ([ADR-0003](adr/0003-use-oxigraph.md)) |

---

## Intentionally in-house (no crate substitute)

| Area | Strixonomy owns | Why |
|------|----------------|-----|
| Workspace scanner security | Path jail, resource limits | Product-specific ([`path_jail.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-core/src/path_jail.rs)) |
| Catalog schema + SQL virtual tables | Table design, entity API | Product-specific |
| Ontology quality lints | Duplicate labels, orphans, missing labels | Rules on catalog; no ontology-linter crate |
| LSP custom protocol | `strixonomy/*` methods, snake_case enums | [ADR-0007](adr/0007-language-server-boundary.md), [ADR-0012](adr/0012-lsp-json-snake-case-enums.md) |
| Semantic diff UX + breaking-change report | PR summaries, Git compare UI | No ontology PR-diff library |
| VS Code extension host | Tree views, commands, LSP client, webview lifecycle | TypeScript orchestration only ([ADR-0007](adr/0007-language-server-boundary.md)) |
| VS Code webview UI | Panel UI (inspector, query, Manchester, reasoner, graphs, diff) | React + Vite in `extension/webview-ui` ([ADR-0017](adr/0017-react-webview-ui.md)) |

---

## License compatibility

See [LICENSES.md](LICENSES.md). Summary:

| Dependency | License | Notes |
|------------|---------|-------|
| Strixonomy / Strixonomy | MIT OR Apache-2.0 | Dual-licensed |
| `horned-owl` | LGPL-3.0 | Dynamic linking; document in distributions |
| `reasonable` (transitive via OntoLogos) | BSD-3-Clause | NOTICES file |
| `oxigraph`, `sqlparser`, OntoLogos crates | MIT OR Apache-2.0 | Compatible |
| `fastobo` family | MIT | Compatible |
| `rudof` | MIT OR Apache-2.0 | Compatible |

---

## MSRV alignment

| Source | MSRV |
|--------|------|
| Strixonomy workspace (`Cargo.toml`) | **1.86** |
| `horned-owl` 1.4 | **1.88** |
| OntoLogos 0.9+ | **1.88** |

**Action:** Bump workspace `rust-version` to **1.88** when integrating `strixonomy-reasoner` (v0.6). (`strixonomy-owl` shipped in v0.4.0 at MSRV 1.88.)

---

## OntoLogos crate map (reasoner stack)

| OntoLogos crate | Role | Strixonomy uses via |
|-----------------|------|-------------------|
| `ontologos-core` | OWL ontology model | `strixonomy-reasoner` |
| `ontologos-parser` | Load `.owl`/`.ttl` | `strixonomy-reasoner` |
| `ontologos-profile` | Profile detection | `strixonomy-reasoner`, LSP warnings |
| `ontologos-el` / `rl` / `rdfs` | Profile engines | `strixonomy-reasoner` adapters |
| `ontologos-dl` | OWL 2 DL (1.0.0) | `strixonomy-reasoner` `dl` adapter |
| `ontologos-facade` | Auto routing (1.0.0) | `strixonomy-reasoner` `auto` adapter |
| `ontologos-explain` | Proof graphs | LSP `getExplanation` |
| `ontologos-watch` | File-watch reload | `strixonomy-lsp` (v0.9 eval) |

---

## Related

- [ADR-0016](adr/0016-dependency-first-implementation.md) — adoption rules
- [OntoLogos dependency-first ADR](https://github.com/eddiethedean/ontologos/blob/main/docs/internal/design/dependency-first.md)
- [OntoLogos rust ecosystem study](https://github.com/eddiethedean/ontologos/blob/main/docs/internal/research/rust-ecosystem.md)
