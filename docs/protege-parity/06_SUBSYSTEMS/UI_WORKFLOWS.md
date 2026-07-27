# UI_WORKFLOWS

# Strixonomy UI Workflow Specification

**Status:** Normative UX Specification\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document defines the canonical user workflows required for Strixonomy
to achieve functional Protégé Desktop parity while taking advantage of a
modern IDE-native interface.

The emphasis is on completing ontology engineering tasks efficiently,
consistently, and accessibly.

------------------------------------------------------------------------

# UX Principles

-   Semantic-first interactions
-   Keyboard-first design
-   Consistent workflows
-   Progressive disclosure
-   Undoable actions
-   Workspace awareness
-   Accessible by default

------------------------------------------------------------------------

# Primary Workflows

## Workspace

-   Create workspace
-   Open ontology
-   Open multiple ontologies
-   Save / Save All
-   Restore previous session
-   Manage imports

## Authoring

-   Create entity
-   Edit entity
-   Delete entity
-   Add axiom
-   Edit annotations
-   Create restrictions
-   Create individuals
-   Manage prefixes

## Reasoning

-   Select reasoner
-   Classify ontology
-   Check consistency
-   Inspect inferred hierarchy
-   View explanations

## SWRL

-   Create rule
-   Edit rule
-   Validate rule
-   Search rules
-   Navigate rule references

## Query

-   Execute SPARQL
-   Execute DL Query
-   Search entities
-   Find usages
-   Save queries
-   Export results

## Refactoring

-   Rename entity
-   Merge entities
-   Replace references
-   Extract module
-   Preview changes
-   Commit or rollback

## Visualization

-   Explore graph
-   Filter graph
-   Overlay inferred relationships
-   Navigate from graph to editor
-   Export visualization

## Plugins

-   Install plugin
-   Enable/disable plugin
-   Configure plugin
-   View diagnostics

------------------------------------------------------------------------

# Cross-Cutting UX

Every workflow should support:

-   Keyboard shortcuts
-   Context menus
-   Command palette
-   Search
-   Undo/redo
-   Progress reporting
-   Error recovery
-   Accessibility

------------------------------------------------------------------------

# Navigation Model

Provide:

-   Global command palette
-   Breadcrumbs
-   Back/forward history
-   Reveal in hierarchy
-   Jump to definition
-   Jump to usages

------------------------------------------------------------------------

# Acceptance Criteria

The UI workflow layer is complete when:

-   All P0 ontology engineering workflows are implemented.
-   Keyboard-only operation is supported.
-   Workflow parity with Protégé is achieved.
-   End-to-end workflow tests pass.
-   Accessibility requirements are satisfied.

------------------------------------------------------------------------

# Related Documents

-   UI_WORKFLOW_PARITY.md
-   PROTEGE_WORKFLOW_AUDIT.md
-   PROTEGE_MENU_AUDIT.md
-   PROTEGE_VIEW_AUDIT.md
-   WORKSPACE.md
-   IMPLEMENTATION_PLAN.md
-   PARITY_TEST_PLAN.md
