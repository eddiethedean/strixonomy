# Automation and stability (pre-1.0)

This page answers one question: **what can you safely automate against Strixonomy today?**

## Stable enough for CI gates (recommended)

These are documented and intentionally stable for automation:

- **`strixonomy validate`** — exit code gates on diagnostic **errors**
- **`strixonomy classify`** — exit code gates on consistency/unsatisfiable classes
- **`strixonomy diff`** — documented CLI usage (pin version pre-1.0)

See:

- [CI integration](ci-integration.md)
- [Workspace limits](workspace-limits.md)

## Pinning guidance (strongly recommended)

Until v1.0, minor releases may change non-gate details. For reproducible pipelines:

- Pin the CLI in CI:
  - `cargo install ontocore-cli --locked --version 0.26.2`, or
  - Linux x64: pinned release binary + SHA256 verification

## Surfaces that may change pre-1.0

- SQL virtual table column names and supported SQL subset
- LSP `strixonomy/*` JSON payload shapes
- Plugin API surface beyond the shipped host MVP

For integrations, treat these as versioned contracts and pin your deployed version:

- [LSP API](lsp-api.md) + [schema](lsp-protocol.schema.json)
- [SQL reference](sql-reference.md)
- [Plugin authoring](guides/plugins.md)

