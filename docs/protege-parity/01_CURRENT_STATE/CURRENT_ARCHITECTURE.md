# CURRENT_ARCHITECTURE

# Current Repository Architecture

**Status:** Living Architecture Document\
**Repository Baseline:** Strixonomy v0.18.2 (audit baseline)

------------------------------------------------------------------------

# Purpose

This document describes the current high-level architecture of Strixonomy
as observed during the repository audit. It records the architecture
that exists today rather than a future-state design.

It is intended to help contributors understand how the major subsystems
fit together and where the remaining Protégé parity work should be
concentrated.

------------------------------------------------------------------------

# Architectural Principles

-   Native Rust implementation
-   IDE-first experience through the VS Code/Cursor extension
-   Canonical semantic model shared across tooling
-   Modular workspace composed of focused crates
-   Separation of engine, language server, UI, and extension logic
-   Test-driven evolution toward Protégé parity

------------------------------------------------------------------------

# High-Level Architecture

``` text
                 VS Code / Cursor
                        │
          ┌─────────────┴─────────────┐
          │                           │
   Extension Host               React Webviews
          │                           │
          ├─────────────┬─────────────┤
          │             │             │
     Command Layer   Workspace     Language Server
          │             │             │
          └─────────────┴─────────────┘
                        │
                  Strixonomy Crates
                        │
      ┌─────────────────┼─────────────────┐
      │                 │                 │
 Parsers          Semantic Model     Reasoning
      │                 │                 │
 Serializers      Refactoring       Query Engine
      │                 │                 │
      └─────────────────┴─────────────────┘
                        │
                Ontology Files
```

------------------------------------------------------------------------

# Major Layers

## Extension Layer

Responsible for:

-   Command registration
-   Workspace integration
-   Panel lifecycle
-   User interaction
-   IDE integration

------------------------------------------------------------------------

## React UI

Provides:

-   Entity inspector
-   Graphs
-   Query workbench
-   Imports browser
-   Reasoning views
-   Semantic diff
-   Dialogs

------------------------------------------------------------------------

## Language Server

Provides:

-   Diagnostics
-   Navigation
-   Semantic analysis
-   Code actions
-   Completion
-   Workspace indexing

------------------------------------------------------------------------

## Strixonomy

Core Rust implementation containing:

-   Parsers
-   Semantic model
-   Refactoring
-   Reasoning
-   Query processing
-   Plugin runtime
-   CLI support

------------------------------------------------------------------------

# Cross-Cutting Services

-   WorkspaceStore
-   Diagnostics
-   Plugin system
-   Test fixtures
-   Documentation
-   Serialization

------------------------------------------------------------------------

# Current Architectural Strengths

-   Clear modular decomposition
-   Native Rust implementation
-   Strong IDE integration
-   Existing reasoning framework
-   Mature extension architecture
-   Broad automated test foundation

------------------------------------------------------------------------

# Architectural Gaps

The repository audit identified the following architectural priorities:

1.  Canonical format-independent change model
2.  RDF/XML and OWL/XML write-back pipeline
3.  Complete OWL 2 authoring support
4.  Stronger workspace semantics
5.  Full reasoning parity
6.  SWRL subsystem
7.  Executable parity verification

These gaps represent architectural work rather than isolated UI
features.

------------------------------------------------------------------------

# Evolution Strategy

The existing architecture should be extended rather than replaced.

Priority should be given to:

-   Completing semantic infrastructure
-   Expanding existing subsystems
-   Preserving modularity
-   Avoiding duplication between parsers and serializers
-   Keeping UI workflows independent of ontology serialization formats

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_SCOPE.md
-   PARITY_GAP_ANALYSIS.md
-   PROTEGE_PARITY_ROADMAP.md
