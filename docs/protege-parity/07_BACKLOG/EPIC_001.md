# EPIC_001

# EPIC-001 --- Format Independence

**Epic ID:** EPIC-001\
**GitHub:** https://github.com/eddiethedean/strixonomy/issues/247\
**Status:** In progress (v0.20)\
**Priority:** P0 (Release Blocking)\
**Target Release:** Strixonomy v0.30.0

------------------------------------------------------------------------

# Objective

Establish a canonical, serializer-independent semantic editing
architecture so every ontology modification is represented as a semantic
transaction instead of format-specific text manipulation.

This epic is the architectural foundation for the entire Protégé parity
effort.

------------------------------------------------------------------------

# Business Value

-   Eliminates duplicated editing logic
-   Enables consistent behavior across all ontology formats
-   Simplifies new format support
-   Enables reliable undo/redo
-   Provides the foundation for reasoning, refactoring, queries, and
    plugins

------------------------------------------------------------------------

# Scope

## In Scope

-   Canonical ontology change model
-   Semantic transaction API
-   Transaction validation
-   Undo/redo integration
-   Serializer adapters
-   Round-trip semantic verification
-   Migration of existing Turtle and OBO editing

## Out of Scope

-   New ontology formats beyond the P0 roadmap
-   UI redesign
-   Collaboration features

------------------------------------------------------------------------

# Major Deliverables

1.  Canonical semantic transaction model
2.  Shared ontology change API
3.  Serializer adapter interfaces
4.  Transaction manager integration
5.  Round-trip conformance suite
6.  Developer documentation

------------------------------------------------------------------------

# Dependencies

None (foundational epic).

Blocks:

-   EPIC-002 OWL 2 Authoring
-   EPIC-003 Workspace Runtime
-   EPIC-004 Reasoning
-   EPIC-006 Semantic Refactoring
-   EPIC-007 Query Engine

------------------------------------------------------------------------

# Milestones

## M1 --- Semantic Model

-   Define change primitives
-   Define inverse operations
-   Define validation lifecycle

## M2 --- Transaction Engine

-   Atomic commit
-   Rollback
-   Undo/redo

## M3 --- Serializer Integration

-   Turtle adapter
-   OBO adapter
-   RDF/XML adapter
-   OWL/XML adapter

## M4 --- Verification

-   Semantic round-trip corpus
-   Regression suite
-   Performance benchmarks

------------------------------------------------------------------------

# Acceptance Criteria

-   All edits flow through semantic transactions.
-   Serializer-specific business logic is removed from editing
    workflows.
-   Supported formats preserve ontology semantics after round-trip.
-   Regression and conformance suites pass.

------------------------------------------------------------------------

# Success Metrics

-   100% editing APIs use semantic transactions
-   Zero serializer-specific editing paths for supported formats
-   100% P0 format conformance
-   Zero semantic-loss defects in regression testing

------------------------------------------------------------------------

# Related Documents

-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   IMPLEMENTATION_PLAN.md
-   DEPENDENCY_GRAPH.md
-   EXECUTION_ORDER.md
-   FORMAT_SUPPORT.md
-   IMPLEMENTATION_EVIDENCE.md
