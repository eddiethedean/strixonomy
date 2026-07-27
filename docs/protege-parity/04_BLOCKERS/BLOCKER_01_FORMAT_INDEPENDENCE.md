# BLOCKER_01_FORMAT_INDEPENDENCE

# Blocker 01 --- Format-Independent Semantic Editing

**Status:** P0 Release Blocker\
**Priority:** Critical\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document defines the architectural work required to make ontology
editing independent of any specific serialization format.

The repository audit identified serializer-coupled editing as the single
highest-impact blocker preventing full Protégé Desktop parity.

A canonical semantic editing pipeline is the foundation for RDF/XML
write-back, OWL/XML write-back, consistent undo/redo, and deterministic
semantic editing.

------------------------------------------------------------------------

# Problem Statement

Today, semantic edits are closely tied to format-specific patch
implementations (for example, Turtle and OBO).

This leads to:

-   Duplicate editing logic
-   Feature inconsistencies between formats
-   Higher maintenance cost
-   Difficulty adding new serializers
-   Incomplete Protégé compatibility

------------------------------------------------------------------------

# Goals

Create a canonical semantic transaction model that:

-   Represents ontology changes independently of syntax
-   Can be consumed by every serializer
-   Supports deterministic application
-   Produces reversible operations
-   Preserves ontology semantics

------------------------------------------------------------------------

# Non-Goals

This blocker does **not** require:

-   Identical file formatting
-   Byte-for-byte serializer output
-   Java compatibility
-   Swing workflow replication

Semantic equivalence is the objective.

------------------------------------------------------------------------

# Target Architecture

``` text
User Action
      │
      ▼
Semantic Transaction
      │
      ▼
Canonical Ontology Model
      │
 ┌────┴────┐
 │         │
 ▼         ▼
RDF/XML  OWL/XML
Serializer Serializer
 │         │
 ▼         ▼
Ontology Files
```

Future serializers (JSON-LD, Functional Syntax, TriG, etc.) should
attach to the same semantic layer.

------------------------------------------------------------------------

# Required Components

## Semantic Transaction Model

Introduce immutable operations such as:

-   Create entity
-   Delete entity
-   Rename entity
-   Add axiom
-   Remove axiom
-   Modify annotation
-   Add import
-   Remove import
-   Change ontology metadata

Each transaction should support:

-   Validation
-   Inverse generation
-   Undo/redo
-   Composition
-   Replay

------------------------------------------------------------------------

## Canonical Ontology Model

All edits should target a single semantic representation rather than
serializer-specific structures.

------------------------------------------------------------------------

## Serializer Adapters

Each supported format should:

1.  Read into the canonical model.
2.  Apply semantic transactions.
3.  Serialize back to its native format.

Required for:

-   Turtle
-   OBO
-   RDF/XML
-   OWL/XML

------------------------------------------------------------------------

# Dependencies

This blocker enables:

-   BLOCKER_02_OWL2_AUTHORING
-   RDF/XML write-back
-   OWL/XML write-back
-   Semantic round-trip testing
-   Consistent workspace transactions

------------------------------------------------------------------------

# Implementation Plan

## Phase 1

-   Define transaction types
-   Define validation rules
-   Define inverse operations

## Phase 2

-   Route Turtle editing through transactions
-   Route OBO editing through transactions

## Phase 3

-   Implement RDF/XML serializer adapter
-   Implement OWL/XML serializer adapter

## Phase 4

-   Add semantic round-trip conformance tests
-   Benchmark performance
-   Remove legacy edit paths where appropriate

------------------------------------------------------------------------

# Risks

-   Increased architectural complexity
-   Serializer edge cases
-   Anonymous node handling
-   Axiom annotation preservation
-   Performance regressions

Mitigate through incremental migration, feature flags where useful, and
comprehensive regression testing.

------------------------------------------------------------------------

# Acceptance Criteria

This blocker is complete when:

-   All semantic edits use the canonical transaction model.
-   Required serializers consume the same transaction pipeline.
-   Undo/redo is transaction-based.
-   RDF/XML and OWL/XML write-back operate through the shared model.
-   Semantic round-trip tests pass for all required formats.
-   Existing Turtle/OBO workflows remain regression-free.

------------------------------------------------------------------------

# Success Metrics

-   Zero serializer-specific business logic in UI workflows.
-   100% semantic round-trip for required formats.
-   No duplicated edit implementations for supported formats.
-   P0 format requirements marked VERIFIED.

------------------------------------------------------------------------

# Related Documents

-   CURRENT_ARCHITECTURE.md
-   TECHNICAL_DEBT.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_RELEASE_GATE.md
