# ADR-0011 — Use sqlparser for SQL Virtual Tables

## Status

Accepted (supersedes [ADR-0004](0004-use-datafusion-for-sql.md) for v0.1–v0.2)

## Context

Strixonomy exposes ontology catalog data as SQL-queryable virtual tables. [ADR-0004](0004-use-datafusion-for-sql.md) proposed evaluating DataFusion; the MVP needed a lightweight `SELECT`/`FROM`/`WHERE` path without pulling the full Arrow/DataFusion stack.

## Decision

Implement SQL-like queries with the [`sqlparser`](https://crates.io/crates/sqlparser) crate and hand-rolled virtual table projection in `strixonomy-query`.

## Consequences

Positive:

- Small dependency footprint
- Straightforward mapping from catalog structs to rows
- Easy to document the supported SQL subset

Negative:

- Limited SQL (single table, simple filters in v0.2)
- No Arrow/Parquet integration until v0.30 joins evaluation

## Amendment ([ADR-0016](0016-dependency-first-implementation.md) Appendix B)

**v0.30 joins / aggregations:** extend hand-rolled virtual tables in `strixonomy-query` using `sqlparser` AST first.

**Revisit [ADR-0004](0004-use-datafusion-for-sql.md) / DataFusion** only if: join logic exceeds maintainability, Parquet/Arrow export becomes P0, or aggregations need a query optimizer beyond hand-rolled plans.

Record final choice in [DEPENDENCY_MATRIX.md](../DEPENDENCY_MATRIX.md).

## Implementation

See [`sql.rs` on GitHub](https://github.com/eddiethedean/strixonomy/blob/main/crates/strixonomy-query/src/sql.rs) and [sql-reference.md](../../sql-reference.md).
