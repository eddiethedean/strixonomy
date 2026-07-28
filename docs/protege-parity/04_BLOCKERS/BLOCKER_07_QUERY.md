# BLOCKER_07_QUERY

# Blocker 07 --- Query & Semantic Search Parity

**Status:** P0 Release Blocker\
**Priority:** Critical\
**Target Release:** Strixonomy v0.30.0

------------------------------------------------------------------------

# Purpose

This document defines the engineering work required for Strixonomy to
achieve functional query parity with a standard Protégé Desktop
installation.

The objective is to provide comprehensive ontology querying capabilities
through a modern Rust-native architecture integrated with the workspace,
language server, reasoning engine, and semantic transaction model.

------------------------------------------------------------------------

# Problem Statement

The repository audit found a strong query foundation, including SPARQL
support and semantic search capabilities.

However, several workflows expected by Protégé users remain incomplete,
including full DL Query parity, semantic usage analysis, unified query
infrastructure, and complete workspace integration.

------------------------------------------------------------------------

# Goals

Provide complete support for:

-   SPARQL querying
-   DL Query workflows
-   Entity search
-   Usage search
-   Annotation search
-   Semantic navigation
-   Query history
-   Saved queries
-   Query-aware reasoning
-   Query-aware refactoring

------------------------------------------------------------------------

# Non-Goals

This blocker does **not** require:

-   Java Protégé plugin compatibility
-   Swing query interfaces
-   Proprietary query languages

The objective is functional query parity using modern IDE-native
workflows.

------------------------------------------------------------------------

# Functional Requirements

## SPARQL

Support:

-   SELECT
-   ASK
-   CONSTRUCT
-   DESCRIBE
-   Parameterized queries
-   Query history
-   Saved queries
-   Export results

## DL Query

Support:

-   Class expression queries
-   Equivalent class lookup
-   Subclass queries
-   Superclass queries
-   Instance retrieval
-   Property restrictions
-   Syntax validation
-   Query history

## Semantic Search

Support:

-   Entity search
-   Prefix search
-   Annotation search
-   Fuzzy search
-   Namespace search
-   Symbol lookup

## Usage Analysis

Support:

-   Find usages
-   Incoming references
-   Outgoing references
-   Dependency analysis
-   Cross-ontology references
-   Refactoring previews

## Query Results

Every query should support:

-   Sorting
-   Filtering
-   Navigation
-   Copy/export
-   Jump to entity
-   Reveal in hierarchy
-   Open inspector

## Workspace Integration

Queries should integrate with:

-   Active ontology
-   Multiple ontologies
-   Workspace transactions
-   Navigation history
-   Selection synchronization
-   Dirty-state awareness

## Reasoning Integration

Queries should optionally execute against:

-   Asserted hierarchy
-   Inferred hierarchy
-   Current reasoner
-   Current workspace state

------------------------------------------------------------------------

# Architecture

``` text
Workspace
    │
    ▼
Query Manager
    │
    ├── SPARQL Engine
    ├── DL Query Engine
    ├── Search Engine
    ├── Usage Engine
    ├── Result Model
    ├── History Manager
    └── UI Integration
```

All query engines should operate on the canonical ontology model and
consume the same semantic indexes used throughout Strixonomy.

------------------------------------------------------------------------

# Dependencies

Depends on:

-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   BLOCKER_03_WORKSPACE.md
-   BLOCKER_04_REASONING.md

Enables:

-   Complete semantic navigation
-   Advanced refactoring
-   Workspace-wide analysis
-   Explanation workflows
-   Productivity improvements

------------------------------------------------------------------------

# Implementation Phases

## Phase 1

-   Audit existing query capabilities
-   Define unified Query API
-   Define result model

## Phase 2

-   Complete DL Query implementation
-   Expand semantic search
-   Add usage analysis

## Phase 3

-   Workspace integration
-   Reasoner integration
-   Saved queries
-   Query history

## Phase 4

-   Performance optimization
-   Conformance fixtures
-   Documentation
-   Regression suite

------------------------------------------------------------------------

# Risks

-   Query performance on large ontologies
-   Incremental index synchronization
-   Cross-ontology query correctness
-   Reasoner synchronization

Mitigate through incremental indexing, benchmark-driven optimization,
deterministic query execution, and comprehensive regression testing.

------------------------------------------------------------------------

# Acceptance Criteria

This blocker is complete when:

-   All required SPARQL workflows function correctly.
-   DL Query workflows reach Protégé parity.
-   Semantic search covers all supported entity types.
-   Usage analysis is available across the workspace.
-   Query results integrate with navigation and inspectors.
-   Query regression and conformance suites pass.

------------------------------------------------------------------------

# Success Metrics

-   100% required query workflows implemented
-   Zero release-blocking query defects
-   Complete workspace-wide semantic search
-   All query parity requirements VERIFIED

------------------------------------------------------------------------

# Related Documents

-   PROTEGE_WORKFLOW_AUDIT.md
-   PROTEGE_REASONER_AUDIT.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   BLOCKER_03_WORKSPACE.md
-   BLOCKER_04_REASONING.md
