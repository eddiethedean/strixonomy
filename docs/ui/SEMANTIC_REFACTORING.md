# SEMANTIC_REFACTORING.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Semantic Refactoring Specification

## Purpose

Semantic Refactoring transforms ontologies while preserving their
intended meaning. Like refactoring in modern IDEs, these operations
improve maintainability, consistency, and quality without changing
expected behavior.

Refactoring should become a routine, low-risk activity rather than a
dangerous manual process.

------------------------------------------------------------------------

# Vision

Strixonomy should provide the world's most comprehensive semantic
refactoring engine.

Every refactoring should be:

-   Safe
-   Previewable
-   Undoable
-   Workspace-aware
-   AI-assisted
-   Reasoning-aware

------------------------------------------------------------------------

# Design Principles

## Semantic Safety

Refactorings operate on semantic objects---not text.

Renaming a class updates:

-   References
-   Restrictions
-   Queries
-   Documentation
-   Imports
-   AI context

No broken references should remain.

## Preview Before Apply

Every refactoring displays:

-   Changed entities
-   Impact summary
-   Reasoning changes
-   Validation results
-   Potential risks

Nothing is applied automatically.

## Atomic Operations

Each refactoring is a single undoable transaction.

------------------------------------------------------------------------

# Refactoring Categories

## Naming

-   Rename Class
-   Rename Property
-   Rename Individual
-   Rename Namespace
-   Change IRI
-   Update Labels

## Structural

-   Move Class
-   Extract Module
-   Merge Modules
-   Split Module
-   Reorganize Hierarchy

## Entity

-   Merge Equivalent Classes
-   Split Overloaded Class
-   Safe Delete
-   Introduce Intermediate Class
-   Replace Deprecated Entity

## Relationships

-   Convert Relationship
-   Normalize Restrictions
-   Update Domains
-   Update Ranges
-   Remove Redundant Relationships

## Documentation

-   Generate Missing Docs
-   Normalize Terminology
-   Synchronize Labels
-   Update Examples

------------------------------------------------------------------------

# Refactoring Workflow

1.  Select entity
2.  Choose refactoring
3.  Analyze workspace
4.  Display preview
5.  Run reasoning
6.  Show impact report
7.  User approves
8.  Apply transaction
9.  Refresh workspace

------------------------------------------------------------------------

# Preview Window

The preview displays:

-   Changed entities
-   Added axioms
-   Removed axioms
-   Updated references
-   Documentation changes
-   Reasoning impact
-   Diagnostics before/after

Support filtering and export.

------------------------------------------------------------------------

# Semantic Impact Analysis

Every operation analyzes:

-   Direct references
-   Transitive references
-   Imports
-   Queries
-   SHACL
-   Rules
-   Documentation
-   Git history

------------------------------------------------------------------------

# AI-Assisted Refactoring

AI may recommend:

-   Duplicate concepts
-   Better hierarchy
-   Naming improvements
-   Modularization
-   Simpler restrictions
-   Documentation cleanup

AI must explain every recommendation.

------------------------------------------------------------------------

# Batch Refactoring

Support applying operations to multiple entities.

Examples:

-   Rename namespace
-   Normalize labels
-   Convert annotations
-   Add missing documentation
-   Replace deprecated IRIs

Preview remains mandatory.

------------------------------------------------------------------------

# Reasoning Integration

Run reasoning before and after refactoring.

Highlight:

-   New inconsistencies
-   Resolved inconsistencies
-   Inferred changes
-   Unsatisfied classes

------------------------------------------------------------------------

# Undo / Redo

Every refactoring is reversible.

Undo restores:

-   Semantic state
-   Documentation
-   References
-   Graph layouts where appropriate

------------------------------------------------------------------------

# Collaboration

Future support:

-   Refactoring reviews
-   Shared refactoring plans
-   Pull request previews
-   Team approval workflows

------------------------------------------------------------------------

# Plugin Extension Points

Plugins may contribute:

-   New refactorings
-   Validation rules
-   Preview renderers
-   AI advisors
-   Impact analyzers

------------------------------------------------------------------------

# Performance Targets

Small refactoring preview: \<250 ms

Large workspace analysis: progressive

Apply transaction: atomic

Reasoning: asynchronous with progress

------------------------------------------------------------------------

# Example Refactorings

## Rename Class

Updates every semantic reference.

## Merge Equivalent Classes

Combines duplicate concepts while preserving history.

## Extract Module

Creates a reusable ontology module.

## Normalize Hierarchy

Removes unnecessary intermediate levels.

## Safe Delete

Verifies no remaining semantic dependencies before removal.

------------------------------------------------------------------------

# Success Criteria

Semantic refactoring succeeds when ontology engineers trust it as much
as software developers trust refactoring tools in modern IDEs. Users
should confidently improve ontology quality through automated,
reasoning-aware transformations that are previewable, reversible, and
deeply integrated with the Strixonomy workspace.
