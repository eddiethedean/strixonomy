# COMPONENT_LIBRARY.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Component Library

## Purpose

This document defines the canonical UI component library for Strixonomy.
Every screen should be composed from these reusable components to ensure
a consistent, accessible, and maintainable experience.

------------------------------------------------------------------------

# Design Goals

The component library should be:

-   Consistent
-   Composable
-   Accessible
-   Themeable
-   Keyboard-first
-   React-first

Each component should solve one problem well and be reusable across the
IDE.

------------------------------------------------------------------------

# Component Hierarchy

    Application
     ├── Layout
     │    ├── Top Bar
     │    ├── Explorer
     │    ├── Workspace
     │    ├── Inspector
     │    └── Bottom Dock
     ├── Navigation
     ├── Data Display
     ├── Forms
     ├── Feedback
     ├── Graph
     ├── AI
     └── Utilities

------------------------------------------------------------------------

# Layout Components

## WorkspaceLayout

Responsibilities

-   Three-pane layout
-   Resizable panels
-   Persistent sizes
-   Responsive collapse
-   Dock integration

Props

-   currentWorkspace
-   currentFocus
-   dockState

------------------------------------------------------------------------

## DockPanel

Supports:

-   Problems
-   Query
-   Graph
-   Git
-   AI
-   Output

States

-   Hidden
-   Collapsed
-   Expanded
-   Detached (future)

------------------------------------------------------------------------

# Navigation Components

## ExplorerTree

Features

-   Virtualized
-   Search
-   Favorites
-   Recent
-   Multi-select
-   Drag-and-drop

Events

-   onSelect
-   onExpand
-   onRename
-   onContextMenu

------------------------------------------------------------------------

## Breadcrumbs

Displays semantic navigation path.

Supports:

-   Click to navigate
-   Overflow handling
-   Keyboard navigation

------------------------------------------------------------------------

## CommandPalette

Universal action launcher.

Capabilities

-   Navigate entities
-   Run commands
-   Execute queries
-   AI actions
-   Open documentation

------------------------------------------------------------------------

# Workspace Components

## EntityEditor

Primary editing experience.

Sections

-   Overview
-   Hierarchy
-   Relationships
-   Constraints
-   Annotations
-   Documentation
-   History

Supports inline editing throughout.

------------------------------------------------------------------------

## WorkspaceTabs

Persistent semantic tabs.

Features

-   Pin
-   Reorder
-   Close
-   Restore
-   Dirty indicators

------------------------------------------------------------------------

# Inspector Components

## EntityCard

Displays:

-   Name
-   Type
-   Description
-   Status
-   Diagnostics
-   References

------------------------------------------------------------------------

## RelationshipList

Interactive list of semantic relationships.

Supports

-   Filtering
-   Sorting
-   Jump to entity
-   Inline editing

------------------------------------------------------------------------

## MetadataPanel

Displays technical metadata.

Examples

-   IRI
-   Namespace
-   Version
-   Source ontology

------------------------------------------------------------------------

# Search Components

## UniversalSearch

Indexes

-   Entities
-   Queries
-   Documentation
-   Diagnostics
-   Git
-   AI

Grouped results.

Keyboard-first.

------------------------------------------------------------------------

## SearchResultCard

Displays:

-   Title
-   Category
-   Description
-   Highlighted matches
-   Quick actions

------------------------------------------------------------------------

# Graph Components

## SemanticGraph

Interactive graph workspace.

Features

-   Zoom
-   Pan
-   Pin
-   Expand
-   Collapse
-   Saved layouts
-   Semantic coloring

------------------------------------------------------------------------

## GraphMiniMap

Overview of current graph viewport.

------------------------------------------------------------------------

## GraphToolbar

Actions

-   Layout
-   Filter
-   Export
-   Focus
-   Search

------------------------------------------------------------------------

# Query Components

## QueryEditor

Features

-   SQL editor
-   Syntax highlighting
-   Autocomplete
-   History
-   Saved queries

------------------------------------------------------------------------

## QueryResults

Views

-   Table
-   Graph
-   JSON
-   CSV Preview

------------------------------------------------------------------------

# AI Components

## AIActionBar

Contextual actions

-   Explain
-   Improve
-   Refactor
-   Document
-   Repair

------------------------------------------------------------------------

## SuggestionCard

Contains

-   Recommendation
-   Reasoning
-   Confidence
-   Preview
-   Apply

------------------------------------------------------------------------

# Feedback Components

## ProblemsPanel

Displays

-   Errors
-   Warnings
-   Suggestions

Supports click-to-navigate.

------------------------------------------------------------------------

## StatusChip

States

-   Success
-   Warning
-   Error
-   Busy
-   AI
-   Draft

------------------------------------------------------------------------

## Toast

Short-lived notifications.

Never blocks workflow.

------------------------------------------------------------------------

# Forms

## InlineEditor

Editable text with validation.

Supports:

-   keyboard commit
-   escape cancel
-   undo

------------------------------------------------------------------------

## PropertyField

Common property editor.

Supports

-   strings
-   enums
-   booleans
-   entity references
-   collections

------------------------------------------------------------------------

# Utility Components

## EmptyState

Always explains:

-   Why empty
-   Next action

------------------------------------------------------------------------

## LoadingSkeleton

Preferred over spinners.

------------------------------------------------------------------------

## ContextMenu

Generated dynamically from current selection.

------------------------------------------------------------------------

# Accessibility Requirements

Every component must support:

-   keyboard navigation
-   screen readers
-   focus indicators
-   reduced motion
-   high contrast

------------------------------------------------------------------------

# Theming

All styling uses design tokens.

No hard-coded:

-   colors
-   spacing
-   typography
-   border radius

------------------------------------------------------------------------

# Extension Points

Every major component exposes extension slots.

Examples

-   Explorer node badges
-   Inspector cards
-   Workspace tabs
-   AI providers
-   Context menu actions
-   Graph overlays

Plugins should compose existing components rather than replace them.

------------------------------------------------------------------------

# Component Lifecycle

Each component should define:

-   Inputs
-   Outputs
-   Loading state
-   Empty state
-   Error state
-   Keyboard behavior
-   Accessibility behavior
-   Performance expectations

------------------------------------------------------------------------

# Success Criteria

The component library should allow developers to construct any Strixonomy
screen by composing standardized, reusable components while ensuring
visual consistency, accessibility, and predictable interactions across
the entire application.
