# BLOCKER_06_REFACTORING

# Blocker 06 --- Semantic Refactoring Parity

**Status:** P0 Release Blocker\
**Priority:** Critical\
**Target Release:** Strixonomy v0.30.0

------------------------------------------------------------------------

# Purpose

This document defines the engineering work required for Strixonomy to
achieve semantic refactoring parity with a standard Protégé Desktop
installation.

Refactoring must preserve ontology semantics while safely transforming
entities, axioms, namespaces, and ontology structure.

------------------------------------------------------------------------

# Problem Statement

The repository audit found existing semantic refactoring capabilities,
but they are not yet comprehensive enough for Protégé parity.

Current limitations include incomplete coverage across OWL 2 constructs,
serializer-independent transformations, workspace-wide operations, and
verification.

------------------------------------------------------------------------

# Goals

Provide safe, semantic refactoring for every supported ontology format
through a common transaction pipeline.

All refactorings should be:

-   Semantic-aware
-   Atomic
-   Undoable
-   Previewable
-   Workspace-aware
-   Serializer-independent

------------------------------------------------------------------------

# Non-Goals

This blocker does **not** require:

-   Text-based search/replace
-   Java Protégé compatibility
-   IDE-specific rename semantics

The objective is ontology-preserving semantic transformation.

------------------------------------------------------------------------

# Required Refactorings

## Entity Operations

-   Rename entity
-   Delete entity
-   Merge entities
-   Split entities (where supported)
-   Move entities between ontologies
-   Replace references

## Ontology Operations

-   Merge ontologies
-   Extract module
-   Namespace migration
-   Import cleanup
-   Prefix migration
-   Ontology metadata updates

## Semantic Operations

-   Update class expressions
-   Rewrite property chains
-   Rewrite restrictions
-   Rewrite annotations
-   Update SWRL references
-   Update query references

------------------------------------------------------------------------

# Preview & Validation

Every refactoring should support:

-   Dry-run preview
-   Affected entity count
-   Affected axiom count
-   Conflict detection
-   Validation before commit
-   Rollback on failure

------------------------------------------------------------------------

# Workspace Integration

Refactorings must integrate with:

-   Workspace transactions
-   Undo/redo
-   Dirty-state tracking
-   Event bus
-   Reasoning refresh
-   Search indexes
-   Navigation history

------------------------------------------------------------------------

# Architecture

``` text
Workspace
    │
    ▼
Refactoring Manager
    │
    ├── Analysis Engine
    ├── Dependency Graph
    ├── Transformation Engine
    ├── Validation Engine
    ├── Preview Generator
    ├── Transaction Adapter
    └── Serializer Adapters
```

All transformations should operate on the canonical semantic model
introduced in `BLOCKER_01_FORMAT_INDEPENDENCE.md`.

------------------------------------------------------------------------

# Dependencies

Depends on:

-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   BLOCKER_03_WORKSPACE.md

Enables:

-   Reliable ontology evolution
-   Complete OWL 2 editing
-   SWRL-aware refactoring
-   Safe multi-format editing

------------------------------------------------------------------------

# Implementation Phases

## Phase 1

-   Audit existing refactorings
-   Define semantic transformation API
-   Create dependency analysis

## Phase 2

-   Complete rename/merge/reference replacement
-   Workspace-wide transactions
-   Preview engine

## Phase 3

-   Module extraction
-   Namespace migration
-   Ontology merge
-   SWRL-aware updates

## Phase 4

-   Conformance fixtures
-   Performance optimization
-   Documentation
-   Regression suite

------------------------------------------------------------------------

# Risks

-   Partial updates
-   Broken references
-   Anonymous expression handling
-   Cross-ontology consistency
-   Large ontology performance

Mitigate through atomic transactions, exhaustive validation, dependency
analysis, and regression testing.

------------------------------------------------------------------------

# Acceptance Criteria

This blocker is complete when:

-   All required semantic refactorings are implemented.
-   Refactorings are previewable and reversible.
-   Workspace transactions guarantee atomicity.
-   References remain semantically correct.
-   Reasoning remains consistent after transformations.
-   Regression and conformance suites pass.

------------------------------------------------------------------------

# Success Metrics

-   100% P0 refactoring workflows complete
-   Zero serializer-specific refactoring logic
-   Zero known release-blocking refactoring defects
-   All parity refactoring requirements VERIFIED

------------------------------------------------------------------------

# Related Documents

-   REFACTORING_PARITY.md
-   PROTEGE_WORKFLOW_AUDIT.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   BLOCKER_03_WORKSPACE.md
