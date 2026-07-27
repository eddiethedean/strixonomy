# ADR-0001 — Use Rust for Strixonomy

## Status
Accepted

## Context
Strixonomy needs high performance, safe concurrency, native binaries, good CLI ergonomics, and integration with VS Code through a language server.

## Decision
Use Rust as the implementation language for Strixonomy.

## Consequences
Positive:
- strong performance
- memory safety
- native CLI
- cross-platform binaries
- good LSP implementation options

Negative:
- smaller ontology ecosystem than Java/Python
- some OWL/reasoner integrations may require adapters
