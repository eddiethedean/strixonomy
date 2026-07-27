# ADR-0007 — Put Ontology Intelligence Behind an LSP Boundary

## Status
Accepted

## Context
Strixonomy needs responsive UI while heavy ontology logic should remain in Rust.

## Decision
Expose ontology intelligence through a Rust language server.

## Consequences
Positive:
- editor-agnostic potential
- clean TypeScript/Rust boundary
- scalable architecture

Negative:
- requires custom LSP methods
- debugging cross-process behavior is harder
