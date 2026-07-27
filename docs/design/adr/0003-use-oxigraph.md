# ADR-0003 — Use Oxigraph for RDF/SPARQL Infrastructure

## Status
Accepted

## Context
Strixonomy needs robust RDF parsing/storage/query capabilities.

## Decision
Use Oxigraph for RDF/SPARQL infrastructure where appropriate.

## Consequences
Positive:
- mature Rust RDF/SPARQL ecosystem
- avoids building triplestore internals
- enables SPARQL support

Negative:
- Strixonomy must differentiate above the triplestore layer
