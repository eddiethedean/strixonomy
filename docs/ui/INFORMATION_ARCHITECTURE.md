# INFORMATION_ARCHITECTURE.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Information Architecture

## Purpose

This document defines how information is organized, discovered,
navigated, and presented throughout Strixonomy. The architecture is
designed around **semantic workflows**, not traditional ontology editor
panels.

------------------------------------------------------------------------

# Design Goals

The information architecture should be:

-   Intuitive
-   Predictable
-   Discoverable
-   Scalable
-   Context-aware
-   Workspace-centric

Users should always know: - Where they are - What they are looking at -
How it relates to the rest of the ontology - What actions are available
next

------------------------------------------------------------------------

# Mental Model

Strixonomy is **not** a file explorer.

It is a **Semantic Workspace**.

The primary object is not a file---it is a semantic entity.

Examples:

-   Class
-   Individual
-   Object Property
-   Data Property
-   Annotation Property
-   Ontology
-   Import
-   Query
-   Diagnostic
-   Documentation
-   Refactoring
-   AI Suggestion

Everything revolves around these objects.

------------------------------------------------------------------------

# Information Hierarchy

## Level 1 --- Workspace

The entire ontology project.

Contains:

-   Ontologies
-   Modules
-   Packages
-   Queries
-   Documentation
-   Git History
-   Diagnostics

------------------------------------------------------------------------

## Level 2 --- Collections

Logical groupings.

Examples:

-   Classes
-   Individuals
-   Properties
-   Imports
-   Rules
-   Shapes
-   Documentation
-   Saved Queries

------------------------------------------------------------------------

## Level 3 --- Semantic Objects

Individual entities.

Examples:

Patient

Doctor

Disease

hasDiagnosis

Medication

------------------------------------------------------------------------

## Level 4 --- Views

Every object exposes multiple synchronized views.

Examples:

-   Overview
-   Hierarchy
-   Relationships
-   Constraints
-   Documentation
-   Graph
-   History
-   References
-   Reasoning
-   AI Insights

These are views---not separate objects.

------------------------------------------------------------------------

# Global Layout

  -----------------------------------------------------------
  Command Palette / Universal Search

  -----------------------------------------------------------

| Explorer \| Workspace \| Inspector \|
|          \| \| \|
|          \| \| \|
|          \| \| \|

+-----------------------------------------------------------+

| Problems \| Graph \| Query \| AI \| Git \| Output \| Terminal \|

+-----------------------------------------------------------+

------------------------------------------------------------------------

# Navigation Model

Navigation occurs at four levels.

## Workspace Navigation

Switch between:

-   Ontologies
-   Modules
-   Projects

------------------------------------------------------------------------

## Collection Navigation

Browse:

-   Classes
-   Properties
-   Individuals

------------------------------------------------------------------------

## Object Navigation

Move directly between semantic entities.

Supports:

-   Back
-   Forward
-   Recent
-   Favorites

------------------------------------------------------------------------

## Context Navigation

Jump between:

-   References
-   Parents
-   Children
-   Graph neighbors
-   Diagnostics
-   Documentation

------------------------------------------------------------------------

# Universal Search

Universal search indexes:

-   Entity names
-   Labels
-   IRIs
-   Relationships
-   Annotations
-   Documentation
-   Queries
-   Diagnostics
-   Git commits
-   AI suggestions

Results are grouped by category and ranked by relevance.

------------------------------------------------------------------------

# Explorer

The Explorer provides structural navigation only.

It should never duplicate information shown elsewhere.

Responsibilities:

-   Browse
-   Filter
-   Organize
-   Favorite
-   Reveal current focus

------------------------------------------------------------------------

# Workspace

The center workspace is task-oriented.

Possible workspace content:

-   Entity editor
-   Query workbench
-   Graph
-   Documentation
-   Diff
-   Review
-   AI workflow

Workspace tabs persist across sessions.

------------------------------------------------------------------------

# Inspector

The Inspector answers:

"What do I need to know about this object?"

It contains:

-   Summary
-   Relationships
-   Metadata
-   Diagnostics
-   History
-   AI suggestions

------------------------------------------------------------------------

# Bottom Dock

Transient information belongs here.

Examples:

-   Problems
-   Build
-   Reasoner
-   Query Results
-   Graph Details
-   AI Chat
-   Git Output

The dock is optional and collapsible.

------------------------------------------------------------------------

# Context Synchronization

Selecting any semantic object updates:

Explorer

Inspector

Workspace

Graph

Breadcrumbs

Documentation

Reasoner

History

AI

No manual synchronization is required.

------------------------------------------------------------------------

# Information Density

The interface prioritizes:

1.  Human-readable information
2.  Semantic meaning
3.  Relationships
4.  Diagnostics
5.  Technical metadata

Internal identifiers should remain available without dominating the UI.

------------------------------------------------------------------------

# Persistence

Remember:

-   Panel sizes
-   Dock state
-   Open tabs
-   Recent entities
-   Favorites
-   Search history
-   Workspace layout

Users should feel like they are returning to the same workspace every
session.

------------------------------------------------------------------------

# Extensibility

Plugins integrate into the architecture through:

-   Explorer nodes
-   Inspector cards
-   Workspace tabs
-   Bottom dock tools
-   Command palette
-   Context menus

Plugins should behave as first-class citizens rather than isolated
extensions.

------------------------------------------------------------------------

# Success Criteria

The architecture is successful when:

-   Navigation feels effortless.
-   Users rarely become lost.
-   Information appears where it is expected.
-   Every view reinforces the current semantic context.
-   The workspace scales from small ontologies to enterprise knowledge
    graphs without becoming overwhelming.
