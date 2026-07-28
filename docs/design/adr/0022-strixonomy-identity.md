# ADR-0022: Strixonomy product identity

## Status

Accepted (v0.27.0)

## Context

ADR-0018 established **OntoCore** (Rust engine) and **OntoCode** (VS Code IDE) as distinct brands. Before freezing v0.30 APIs and Marketplace identity, the products consolidate under a single public name: **Strixonomy**.

v0.9 renamed OntoIndex → OntoCore with **no** compatibility aliases. v0.27 must give v0.26 users a tested upgrade path (settings, workspace state, plugins, CI scripts).

## Decision

1. **Strixonomy** is the primary brand for the Rust semantic workspace engine and the VS Code IDE.
2. Publish crates as **`strixonomy`** / **`strixonomy-*`**; binaries **`strixonomy`** and **`strixonomy-lsp`**; LSP methods **`strixonomy/*`**; extension id **`strixonomy.strixonomy`**.
3. Through at least **v0.30**, dual-read legacy surfaces: `ontocore/*` LSP methods, migrated `ontocode.*` settings, forwarding `ontocode.*` command aliases, `.ontocore/` / `.ontocode/` paths, and deprecated CLI/LSP binary aliases (`ontocore`, `ontocore-lsp`). Emit actionable deprecation warnings. VS Code Memento state is private to an extension ID, so the new listing cannot directly read Memento-only state owned by `strixonomy.strixonomy`; the migration guide documents that limitation.
4. Publish thin crates.io compat packages only for **`ontocore`** (façade re-export), with legacy binary aliases shipped from the primary CLI/LSP packages. Direct `ontocore-*` implementation dependencies migrate via the rename table — no 22-crate shim farm.
5. **Not renamed:** Ontologos (separate reasoner product); stored ontology IRIs such as `http://ontocode.dev/ns#swrlRule` (data compatibility).
6. ADR-0018 remains historical for the OntoIndex → OntoCore cut; brand identity going forward is this ADR.

## Consequences

### Positive

- Single product name before v0.30 freeze.
- Upgrade path for v0.26 pilots without silent loss of configuration.

### Negative / mitigations

- Dual names increase maintenance until shims are removed v0.31+.
- Marketplace requires a new extension listing; document uninstall of OntoCode.
- See [migration/v0.27.md](../../migration/v0.27.md) and [v0.27_SCOPE.md](../v0.27_SCOPE.md).

## References

- [ADR-0018](0018-ontocore-platform-identity.md) (superseded for brand; historical for v0.9)
- [Migration v0.27](../../migration/v0.27.md)
- [ROADMAP.md](../../roadmap.md) § v0.27
- [V0_30_PHASES.md](https://github.com/eddiethedean/strixonomy/blob/main/docs/protege-parity/07_BACKLOG/V0_30_PHASES.md)
