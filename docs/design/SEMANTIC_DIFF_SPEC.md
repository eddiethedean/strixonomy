# SEMANTIC_DIFF_SPEC.md

> **Dependencies:** [`horned-owl`](https://crates.io/crates/horned-owl) (axiom diff), [`git2`](https://crates.io/crates/git2) (branch/commit inputs). See [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

## 1. Purpose

Text diffs are insufficient for ontology review. Semantic diffs should explain what changed in ontology meaning.

## 2. Diff Inputs

Supported inputs:

- two files
- two directories
- two Git branches (`git2` checkout / tree walk)
- two Git commits
- working tree vs HEAD
- cached catalog snapshots

**Strixonomy owns:** axiom-level diff logic on Horned-OWL models, breaking-change heuristics, PR markdown, VS Code panel. No ontology semantic-diff crate exists — intentionally in-house per [ADR-0016](adr/0016-dependency-first-implementation.md).

## 3. Diff Categories

### Entity Changes

- class added
- class removed
- property added
- property removed
- individual added
- individual removed
- entity renamed
- entity deprecated

### Axiom Changes

- subclass axiom added/removed
- equivalent class axiom added/removed
- disjointness changed
- domain changed
- range changed
- restriction added/removed

### Annotation Changes

- label changed
- comment changed
- definition changed
- synonym changed
- metadata changed

### Import Changes

- import added
- import removed
- import target changed
- broken import introduced

### Inference Changes

- inferred parent changed
- class became unsatisfiable
- class became satisfiable
- new inferred equivalence

## 4. Breaking Change Detection

Potential breaking changes:

- removed public entity
- renamed IRI
- changed domain/range
- removed superclass
- removed required restriction
- removed import
- deprecated widely-used entity

## 5. CLI

```bash
strixonomy diff main..feature
strixonomy diff --format markdown main..feature
strixonomy diff --breaking-only main..feature
```

## 6. VS Code UI

Semantic diff panel sections:

- Summary
- Breaking changes
- Entity changes
- Axiom changes
- Annotation changes
- Import changes
- Inference changes

## 7. Pull Request Summary

Generate Markdown:

- changed entity count
- breaking changes
- major hierarchy changes
- validation status
- docs impact

## 8. v0.30 Requirements

- Git branch comparison
- entity-level diff
- axiom-level diff
- breaking change report
- Markdown export
