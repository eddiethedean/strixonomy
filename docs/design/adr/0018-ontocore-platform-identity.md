# ADR-0018: OntoCore platform identity

## Status

Accepted (v0.9.0) — **brand identity superseded by [ADR-0022](0022-strixonomy-identity.md)** (v0.27 Strixonomy). This ADR remains the historical record of the OntoIndex → OntoCore rename.

## Context

The Rust semantic workspace engine was introduced as **OntoIndex** (`ontoindex` CLI, `ontoindex-*` crates). As the platform matures, it needs a distinct identity separate from the VS Code product (**OntoCode**) and the reasoning stack (**OntoLogos**).

Splitting into a separate repository prematurely would create churn while APIs are still evolving. The monorepo already has a clean architectural split between `crates/` (engine) and `extension/` (IDE).

## Decision

1. **OntoCore** is the platform brand for the Rust semantic workspace engine.
2. **OntoCode** is the VS Code IDE powered by OntoCore.
3. In **v0.9.0**, rename all implementation crates to **`ontocore-*`** with no `ontoindex` compatibility aliases.
4. Publish a public façade crate **`ontocore`** that re-exports implementation crates and provides `Workspace` as the ergonomic entry point.
5. Standard binaries and wire protocol:
   - CLI: `ontocore` (`ontocore-cli` crate)
   - LSP: `ontocore-lsp` (`ontocore-lsp` crate)
   - LSP custom methods: `ontocore/*`
6. Repository remains a monorepo until API stability and release process justify a split.

## Consequences

### Positive

- Clear branding for users, contributors, and future bindings (Python, TypeScript, MCP).
- Single naming scheme across crates, binaries, docs, and LSP protocol.
- `ontocore` crate gives embedders one dependency for the high-level API.

### Negative / mitigations

- v0.9 is a **breaking release** for Rust, CLI, and LSP integrators — see [migration/v0.9.md](../../migration/v0.9.md).
- Historical ADRs and design docs may mention OntoIndex; user-facing docs use OntoCore.

## References

- [OntoCore roadmap](../../strixonomy/roadmap.md)
- [Migration v0.9](../../migration/v0.9.md)
- [ARCHITECTURE.md](../ARCHITECTURE.md)
- [ROADMAP.md](../ROADMAP.md) v0.9 section
