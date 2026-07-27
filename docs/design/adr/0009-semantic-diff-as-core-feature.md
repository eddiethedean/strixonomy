# ADR-0009 — Treat Semantic Diff as a Core Feature

## Status
Accepted

## Context
Git text diffs are not enough for ontology review.

## Decision
Implement semantic diff in Strixonomy as a core feature, not only a VS Code UI feature.

## Consequences
Positive:
- usable in CLI and CI
- strong differentiator from Protégé
- improves PR review workflows

Negative:
- complex ontology normalization required
