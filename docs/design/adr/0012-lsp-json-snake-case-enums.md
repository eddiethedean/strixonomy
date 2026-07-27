# ADR-0012: LSP JSON uses snake_case enum strings

## Status

Accepted (v0.2.0)

## Context

`EntityKind`, `ParseStatus`, and `OntologyFormat` are Rust enums with serde's default PascalCase JSON (`"Class"`, `"Ok"`). The VS Code extension and `docs/lsp-api.md` expect snake_case (`"class"`, `"ok"`). SQL CLI output already used `EntityKind::as_str()` (snake_case), so LSP JSON disagreed with both the extension and the CLI.

## Decision

- Add `#[serde(rename_all = "snake_case")]` on wire enums in `strixonomy-core`.
- Treat snake_case as the normative LSP JSON contract; validate in Rust wire-format tests and TypeScript `protocolGuards`.
- Keep `EntityKind::as_str()` aligned with serde output for SQL and LSP.

## Consequences

- Breaking change for any external client that relied on PascalCase JSON (unlikely in v0.2).
- Single string form per kind across LSP, SQL, and extension filters.
