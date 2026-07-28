# Architecture Decision Records

Canonical ADRs live in this directory. The former `adrs/` folder was merged here; do not add duplicate ADR trees.

## Index

| ADR | Title | Status |
|-----|-------|--------|
| [0001](0001-use-rust-for-ontocore.md) | Use Rust for Strixonomy | Accepted |
| [0002](0002-use-horned-owl.md) | Use Horned-OWL for OWL modeling | **Superseded by ADR-0013** (v0.4b+) |
| [0003](0003-use-oxigraph.md) | Use Oxigraph | Accepted |
| [0004](0004-use-datafusion-for-sql.md) | Use DataFusion for SQL | **Superseded** (v0.2 uses sqlparser virtual tables) |
| [0005](0005-local-first-by-default.md) | Local-first by default | Accepted |
| [0006](0006-patch-based-write-back.md) | Patch-based write-back | Accepted (v0.4.0) |
| [0007](0007-language-server-boundary.md) | Language server boundary | Accepted |
| [0008](0008-reasoner-adapters-not-built-in-reasoner.md) | Reasoner adapters | Accepted (v0.6) |
| [0009](0009-semantic-diff-as-core-feature.md) | Semantic diff as core feature | Accepted (v0.9) |
| [0010](0010-ai-features-opt-in.md) | AI features opt-in | Accepted (planned) |
| [0011](0011-use-sqlparser-for-sql.md) | Use sqlparser for SQL virtual tables | Accepted |
| [0012](0012-lsp-json-snake-case-enums.md) | LSP JSON snake_case enums | Accepted |
| [0013](0013-dual-stack-oxigraph-horned-owl.md) | Dual stack Oxigraph + Horned-OWL | Accepted (v0.4.0) |
| [0014](0014-rust-native-reasoners-only.md) | Rust-native reasoners only (no JVM) | Accepted |
| [0015](0015-adopt-ontologos-reasoner.md) | Adopt OntoLogos as reasoner backend | Accepted |
| [0016](0016-dependency-first-implementation.md) | Dependency-first implementation | Accepted |
| [0017](0017-react-webview-ui.md) | Adopt React for VS Code webview panels | Accepted (shipped) |
| [0018](0018-ontocore-platform-identity.md) | Strixonomy platform identity | Accepted (v0.9); **brand superseded by ADR-0022** |
| [0019](0019-obo-write-back.md) | OBO write-back via fastobo | Accepted — **implemented v0.12** (read v0.11) |
| [0020](0020-semantic-transaction-edit-model.md) | Semantic transaction edit model | Accepted — **implemented v0.19** |
| [0021](0021-deterministic-xml-serializers.md) | Deterministic RDF/XML and OWL/XML serializers | Accepted — **implemented v0.21** |
| [0022](0022-strixonomy-identity.md) | Strixonomy product identity | Accepted (v0.27) |

## Current stack

### v0.2 (shipped)

- **Parsing / triple store:** Oxigraph ([ADR-0003](0003-use-oxigraph.md))
- **SQL-like queries:** sqlparser + virtual tables ([ADR-0011](0011-use-sqlparser-for-sql.md))
- **Editor integration:** LSP stdio ([ADR-0007](0007-language-server-boundary.md))
- **LSP wire format:** snake_case enums ([ADR-0012](0012-lsp-json-snake-case-enums.md))

### v0.3 (shipped)

- **Diagnostics:** in-house `strixonomy-diagnostics` + Oxigraph parse errors ([ADR-0016](0016-dependency-first-implementation.md), [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md))
- **LSP:** `textDocument/publishDiagnostics` after reindex; `CatalogSnapshot.diagnostics`

### v0.30 target

- **Dependency policy:** [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md), [ADR-0016](0016-dependency-first-implementation.md)
- **OWL modeling / write-back:** `horned-owl` + `horned-functional` via `strixonomy-owl` ([ADR-0013](0013-dual-stack-oxigraph-horned-owl.md))
- **Reasoning:** [OntoLogos](https://github.com/eddiethedean/ontologos) via `strixonomy-reasoner` — 0.9.0 at v0.6, 1.0.0 at v0.30 ([ADR-0014](0014-rust-native-reasoners-only.md), [ADR-0015](0015-adopt-ontologos-reasoner.md))
- **Webview UI:** React + Vite in `extension/webview-ui` ([ADR-0017](0017-react-webview-ui.md))
- **OBO:** `fastobo` / `fastobo-owl` ([DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md))
- **SHACL (P1):** `rudof` ([SHACL_SPEC.md](../SHACL_SPEC.md))
- **Exit bar:** [PROTEGE_PARITY.md](../PROTEGE_PARITY.md)

Horned-OWL and DataFusion ADRs remain for historical context; v0.30 uses Oxigraph + Horned-OWL + sqlparser per ADR-0013 and ADR-0011.
