# ADR-0004 — Use DataFusion for SQL-Style Query Execution

## Status

**Superseded** — v0.1–v0.2 implemented SQL virtual tables with sqlparser ([ADR-0011](0011-use-sqlparser-for-sql.md)). DataFusion may be revisited for analytics or Parquet export later.

## Context

Strixonomy should expose ontology concepts as SQL-queryable virtual tables.

## Decision

Evaluate DataFusion as the SQL execution layer.

## Consequences

Positive:

- mature query execution
- Arrow ecosystem
- future Parquet/CSV integrations

Negative:

- virtual table integration complexity
- may be heavier than needed for early MVP

## Superseded by

[ADR-0011 — Use sqlparser for SQL virtual tables](0011-use-sqlparser-for-sql.md).
