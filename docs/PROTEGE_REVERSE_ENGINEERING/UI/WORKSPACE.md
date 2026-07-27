# WORKSPACE.md

# Protégé Workspace Reverse Engineering

## Purpose

The Protégé workspace is the primary environment in which ontology
engineers create, edit, reason over, and navigate OWL ontologies. The
workspace is organized around dockable "views" grouped into
task-oriented tabs. Although layouts can be customized, the default
workspace establishes the mental model for nearly all Protégé workflows.

------------------------------------------------------------------------

# High-Level Layout

    +-------------------------------------------------------------+
    | Menu Bar                                                    |
    +-------------------------------------------------------------+
    | Toolbar                                                     |
    +-------------------------------------------------------------+
    | Tabs: Classes | Object Properties | Data Properties | ...   |
    +---------------------------+---------------------------------+
    | Navigation View           | Editor / Main View              |
    | (Tree, Lists, Hierarchy)  |                                 |
    |                           | Details / Forms / Graphs        |
    +---------------------------+---------------------------------+
    | Optional Docked Views / Search / Explanation / Metrics      |
    +-------------------------------------------------------------+
    | Status Bar                                                   |
    +-------------------------------------------------------------+

------------------------------------------------------------------------

# Workspace Goals

-   Provide a task-focused editing experience.
-   Allow multiple synchronized views of the same ontology.
-   Support docking, resizing, and persistence of layouts.
-   Keep reasoning results synchronized with edits.
-   Minimize context switching while editing.

------------------------------------------------------------------------

# Core Workspace Regions

## Menu Bar

Contains global application actions including:

-   File
-   Edit
-   Active Ontology
-   Refactor
-   Reasoner
-   Tools
-   Window
-   Help

These actions generally operate on the current project rather than an
individual entity.

------------------------------------------------------------------------

## Toolbar

Provides one-click access to common actions such as:

-   New ontology
-   Open
-   Save
-   Undo / Redo
-   Synchronize reasoner
-   Search
-   Preferences

Toolbars are intentionally lightweight and avoid exposing every command.

------------------------------------------------------------------------

## Workspace Tabs

The central navigation model is task-based. Common tabs include:

-   Classes
-   Object Properties
-   Data Properties
-   Annotation Properties
-   Individuals
-   DL Query

Plugins may introduce additional tabs.

Each tab owns its own collection of synchronized views.

------------------------------------------------------------------------

## Navigation Views

Typically shown on the left side.

Examples:

-   Class hierarchy
-   Property hierarchy
-   Individual tree
-   Imported ontologies
-   Search results

Responsibilities:

-   Rapid navigation
-   Tree expansion/collapse
-   Filtering
-   Selection synchronization

------------------------------------------------------------------------

## Main Editor

Displays the selected ontology entity.

Depending on the active tab, editors expose:

-   Labels
-   IRIs
-   Superclasses
-   Equivalent classes
-   Restrictions
-   Property assertions
-   Annotations
-   Domain and range definitions

Editing is generally form-based.

------------------------------------------------------------------------

## Auxiliary Views

Optional dockable panels may include:

-   Entity description
-   Usage
-   Inferred hierarchy
-   Metrics
-   Explanation
-   Search
-   Graph visualization (plugins)

These panels stay synchronized with the active selection.

------------------------------------------------------------------------

## Status Bar

Typically displays:

-   Loaded ontology status
-   Active reasoner
-   Classification state
-   Background task progress
-   Warnings or errors

------------------------------------------------------------------------

# Workspace Behavior

## Selection Synchronization

Selecting an entity updates:

-   hierarchy
-   editor
-   annotations
-   usage
-   explanation
-   inferred views

This synchronized behavior is a defining characteristic of the Protégé
workspace.

## Docking

Views may be:

-   moved
-   docked
-   floated
-   hidden
-   restored

Layouts persist between sessions.

## Persistence

The workspace stores user preferences such as:

-   window layout
-   open tabs
-   splitter positions
-   selected reasoner
-   visible panels

------------------------------------------------------------------------

# Typical Workflow

1.  Open ontology.
2.  Navigate using the hierarchy tree.
3.  Select an entity.
4.  Edit metadata and logical axioms.
5.  Run a reasoner.
6.  Inspect inferred hierarchy.
7.  Resolve issues.
8.  Save changes.

------------------------------------------------------------------------

# Pain Points

-   Dense, expert-oriented interface.
-   Aging Swing docking framework.
-   Limited keyboard-centric workflows.
-   Minimal contextual guidance.
-   Static graph integration.

------------------------------------------------------------------------

# Modernization Opportunities for Strixonomy

A modern successor could improve the workspace by introducing:

-   VS Code--style activity bar and sidebars.
-   React/Tauri interface with responsive layouts.
-   Command palette for all actions.
-   Multi-document editing.
-   Native graph canvas synchronized with forms.
-   AI-assisted property and class editing.
-   Workspace profiles (modeling, reasoning, review).
-   Git-aware change indicators.
-   Real-time collaboration presence.
-   Pluggable panels using a stable extension API.

------------------------------------------------------------------------

# Feature Parity Checklist

-   [ ] Menu system
-   [ ] Toolbar
-   [ ] Dockable views
-   [ ] Task-based tabs
-   [ ] Hierarchy navigation
-   [ ] Property editors
-   [ ] Selection synchronization
-   [ ] Layout persistence
-   [ ] Reasoner integration
-   [ ] Search panels
-   [ ] Status reporting
-   [ ] Plugin-contributed views
