# WORKSPACE

# Strixonomy Workspace Subsystem Specification

**Subsystem:** Workspace Runtime\
**Status:** Normative Architecture Specification\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

The Workspace subsystem is the central runtime of Strixonomy. It manages
the lifecycle of ontologies, coordinates semantic transactions,
synchronizes application state, and provides the shared execution
environment used by every major subsystem.

Unlike Protégé's document-centric architecture, Strixonomy's Workspace
acts as the orchestration layer for the entire application.

------------------------------------------------------------------------

# Responsibilities

The Workspace is responsible for:

-   Managing loaded ontologies
-   Tracking the active ontology
-   Coordinating semantic transactions
-   Undo/redo orchestration
-   Dirty-state management
-   Save/Save All workflows
-   Session persistence
-   Selection synchronization
-   Navigation history
-   Event distribution
-   Plugin integration
-   Reasoner coordination

------------------------------------------------------------------------

# Design Principles

-   Workspace-first architecture
-   Immutable semantic transactions
-   Event-driven synchronization
-   Serializer independence
-   Multi-ontology support
-   Deterministic state transitions
-   High scalability

------------------------------------------------------------------------

# Core Components

``` text
Workspace Runtime
      │
      ├── Ontology Registry
      ├── Active Workspace
      ├── Transaction Manager
      ├── Undo/Redo Manager
      ├── Event Bus
      ├── Selection Manager
      ├── Navigation Manager
      ├── Persistence Manager
      ├── Save Coordinator
      ├── Plugin Bridge
      └── Reasoner Coordinator
```

------------------------------------------------------------------------

# Ontology Lifecycle

Every ontology supports:

1.  Create
2.  Open
3.  Import
4.  Activate
5.  Edit
6.  Save
7.  Save As
8.  Reload
9.  Close

Lifecycle transitions must be atomic and observable.

------------------------------------------------------------------------

# Semantic Transactions

Every modification is represented as a semantic transaction.

Transactions support:

-   Validation
-   Atomic commit
-   Rollback
-   Undo
-   Redo
-   Event publication
-   Audit trail generation

No subsystem should modify ontology state outside the transaction
manager.

------------------------------------------------------------------------

# Multi-Ontology Support

The Workspace maintains:

-   Loaded ontology registry
-   Active ontology
-   Import closure
-   Dependency graph
-   Dirty-state tracking
-   Read-only imported ontologies

------------------------------------------------------------------------

# Session Persistence

Persist and restore:

-   Open ontologies
-   Active ontology
-   Layout
-   Selection
-   Navigation history
-   Window state
-   Workspace preferences

------------------------------------------------------------------------

# Event Bus

Events include:

-   Ontology opened
-   Ontology closed
-   Transaction committed
-   Selection changed
-   Reasoner completed
-   Query executed
-   Refactoring completed
-   Plugin loaded

Subscribers should react without creating cyclic dependencies.

------------------------------------------------------------------------

# Integration

The Workspace coordinates:

-   OWL2 Authoring
-   Reasoning
-   Refactoring
-   Query
-   Visualization
-   Plugin Platform
-   Accessibility
-   Diagnostics

------------------------------------------------------------------------

# Performance Requirements

The Workspace should:

-   Scale to large ontology projects
-   Minimize unnecessary state updates
-   Batch event notifications where appropriate
-   Recover gracefully from failures

------------------------------------------------------------------------

# Acceptance Criteria

The subsystem is complete when:

-   Multi-ontology workflows are reliable.
-   Transactions are atomic.
-   Undo/redo is deterministic.
-   Session restoration succeeds.
-   Event synchronization is consistent.
-   Integration tests pass across supported platforms.

------------------------------------------------------------------------

# Related Documents

-   BLOCKER_03_WORKSPACE.md
-   WORKSPACE_PARITY.md
-   IMPLEMENTATION_PLAN.md
-   DEPENDENCY_GRAPH.md
-   CURRENT_ARCHITECTURE.md
