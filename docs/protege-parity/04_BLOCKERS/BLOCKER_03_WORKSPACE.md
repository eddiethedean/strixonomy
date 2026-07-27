# BLOCKER_03_WORKSPACE

# Blocker 03 --- Workspace Architecture & Multi-Ontology Management

**Status:** P0 Release Blocker\
**Priority:** Critical\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document defines the engineering work required to achieve
Protégé-class workspace behavior for Strixonomy 1.0.

The workspace is responsible for coordinating multiple ontologies,
editor state, reasoning state, UI synchronization, persistence, and
semantic transactions.

------------------------------------------------------------------------

# Problem Statement

The repository audit found a solid WorkspaceStore foundation, but
several behaviors required for full Protégé parity remain incomplete.

Current limitations include:

-   Incomplete session restoration
-   Incomplete workspace persistence
-   Limited multi-ontology lifecycle management
-   Partial synchronization across panels
-   Missing formal transaction semantics

------------------------------------------------------------------------

# Goals

Provide a workspace that supports:

-   Multiple simultaneously open ontologies
-   Persistent user sessions
-   Consistent selection state
-   Shared semantic transactions
-   Reliable undo/redo
-   Cross-panel synchronization
-   Large ontology projects

------------------------------------------------------------------------

# Non-Goals

This blocker does **not** require:

-   Cloud collaboration
-   Multi-user editing
-   Distributed workspaces
-   Git integration as a core workspace feature

Those are post-1.0 enhancements.

------------------------------------------------------------------------

# Functional Requirements

## Ontology Lifecycle

Support:

-   Create
-   Open
-   Close
-   Save
-   Save As
-   Save All
-   Reload
-   Import management

------------------------------------------------------------------------

## Multi-Ontology Support

The workspace should maintain:

-   Loaded ontology registry
-   Active ontology
-   Imports closure
-   Dependency graph
-   Read-only imported ontologies
-   Dirty-state tracking

------------------------------------------------------------------------

## Session Management

Persist and restore:

-   Open ontologies
-   Active ontology
-   Panel layout
-   Expanded trees
-   Selected entities
-   Open editors
-   Navigation history
-   Query history (where appropriate)

------------------------------------------------------------------------

## Workspace Transactions

Every semantic edit should execute as a workspace transaction
supporting:

-   Validation
-   Atomic commit
-   Rollback
-   Undo
-   Redo
-   Event publication

Workspace transactions should consume the canonical semantic transaction
model defined in:

`BLOCKER_01_FORMAT_INDEPENDENCE.md`

------------------------------------------------------------------------

## UI Synchronization

Changes in one panel should update all dependent views:

-   Hierarchies
-   Inspectors
-   Graphs
-   Query results
-   Diagnostics
-   Reasoning
-   Breadcrumbs

Synchronization should be deterministic and event-driven.

------------------------------------------------------------------------

## Navigation

Provide:

-   Back/forward history
-   Jump to entity
-   Reveal in hierarchy
-   Cross-reference navigation
-   Usage navigation

------------------------------------------------------------------------

## Persistence

Persist:

-   Workspace metadata
-   User preferences
-   Recent projects
-   Window state
-   View state
-   Cached indexes (where beneficial)

------------------------------------------------------------------------

# Architecture

Recommended layers:

``` text
Workspace
    │
    ├── Ontology Registry
    ├── Transaction Manager
    ├── Event Bus
    ├── Selection Manager
    ├── Navigation Manager
    ├── Persistence Manager
    └── UI Synchronization
```

Each subsystem should have clear ownership and minimal coupling.

------------------------------------------------------------------------

# Dependencies

Depends on:

-   BLOCKER_01_FORMAT_INDEPENDENCE

Enables:

-   Reasoning synchronization
-   UI parity
-   Refactoring workflows
-   Reliable undo/redo
-   Session restoration
-   Plugin integration

------------------------------------------------------------------------

# Implementation Phases

## Phase 1

-   Formalize WorkspaceStore API
-   Introduce ontology registry
-   Define transaction lifecycle

## Phase 2

-   Implement event bus
-   Add selection synchronization
-   Add navigation manager

## Phase 3

-   Session persistence
-   Layout restoration
-   Dirty-state improvements
-   Save/Save All workflow

## Phase 4

-   Performance tuning
-   Large workspace testing
-   Regression suite
-   Documentation

------------------------------------------------------------------------

# Risks

-   Event ordering bugs
-   State synchronization issues
-   Performance regressions on large ontologies
-   Undo/redo edge cases
-   Plugin interaction complexity

Mitigate through immutable state where practical, exhaustive integration
tests, and deterministic event sequencing.

------------------------------------------------------------------------

# Acceptance Criteria

This blocker is complete when:

-   Multiple ontologies can be managed reliably.
-   Workspace state survives restart.
-   All panels remain synchronized.
-   Workspace transactions are atomic.
-   Undo/redo is workspace-aware.
-   Navigation history functions correctly.
-   Integration tests pass across supported platforms.

------------------------------------------------------------------------

# Success Metrics

-   100% P0 workspace workflows complete
-   Zero known workspace synchronization defects
-   Successful session restoration for representative projects
-   Cross-platform integration tests passing

------------------------------------------------------------------------

# Related Documents

-   CURRENT_ARCHITECTURE.md
-   CURRENT_LIMITATIONS.md
-   WORKSPACE_PARITY.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
-   BLOCKER_01_FORMAT_INDEPENDENCE.md
