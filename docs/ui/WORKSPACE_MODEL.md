# WORKSPACE_MODEL.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Workspace Model

## Purpose

The **Workspace** (product surface) defines the runtime UX architecture — not a VS Code workspace folder. See [Glossary](../glossary.md).

> **Implementation architecture:** [platform/WORKSPACE_RUNTIME.md](../platform/WORKSPACE_RUNTIME.md) · **Current Focus** and **WorkspaceStore** **shipped v0.13**

Strixonomy operates as a single synchronized semantic workspace centered on shared application state (target). Today: isolated webview panels ([platform/ONTOUI.md](../platform/ONTOUI.md)).

## Current Focus

Everything in the IDE revolves around a single object:

Current Focus

Examples

-   Class
-   Individual
-   Property
-   Query
-   Diagnostic
-   Graph Node
-   Documentation Page

Selecting a new focus automatically updates the rest of the application.

------------------------------------------------------------------------

# Workspace State

The Workspace maintains a single source of truth.

``` text
WorkspaceState

├── Current Focus
├── Current Workspace
├── Selection
├── Open Tabs
├── Explorer State
├── Inspector State
├── Dock State
├── Graph State
├── Query State
├── Diagnostics
├── Navigation History
├── Favorites
├── Recent Items
├── Search State
├── Theme
├── AI Context
└── Plugin State
```

No component owns global application state.

------------------------------------------------------------------------

# State Ownership

Each feature owns only its local state.

Global state is coordinated through the WorkspaceStore.

Example

Explorer owns:

-   expansion
-   filtering

Workspace owns:

-   current entity
-   current tab

Inspector owns:

-   active inspector tab

Graph owns:

-   layout
-   zoom

Everything else is derived.

------------------------------------------------------------------------

# Synchronization

Selecting an entity updates:

-   Explorer highlight
-   Workspace editor
-   Inspector
-   Graph
-   Documentation
-   Diagnostics
-   AI context
-   Breadcrumbs
-   Navigation history

No manual synchronization is permitted.

------------------------------------------------------------------------

# Workspace Layout

    +-------------------------------------------------------------+
    | Command Palette / Universal Search                          |
    +-------------------------------------------------------------+

    | Explorer |             Workspace             | Inspector     |
    |          |                                   |               |
    |          |                                   |               |
    |          |                                   |               |
    +-------------------------------------------------------------+

    | Problems | Query | Graph | AI | Git | Output | Terminal      |
    +-------------------------------------------------------------+

The layout manager owns:

-   resizing
-   persistence
-   responsive behavior
-   docking
-   restoration

------------------------------------------------------------------------

# Tabs

Tabs represent semantic workspaces.

Examples

Patient

Disease

Healthcare Ontology

Query

Semantic Diff

Documentation

Each tab remembers:

-   scroll position
-   cursor
-   graph viewport
-   inspector tab
-   local UI state

------------------------------------------------------------------------

# Navigation

Navigation history is semantic.

Users can move:

Back

Forward

Recent

Favorites

Unlike file history, navigation follows semantic objects.

------------------------------------------------------------------------

# Command Palette

The command palette has full access to WorkspaceState.

It can:

-   navigate
-   refactor
-   run queries
-   invoke AI
-   open graphs
-   switch tabs
-   execute plugins

------------------------------------------------------------------------

# Explorer

Explorer is a navigation view.

Responsibilities:

-   browse
-   search
-   organize
-   reveal current focus

Explorer never stores business logic.

------------------------------------------------------------------------

# Inspector

Inspector answers:

"What do I need to know about this object?"

Inspector never becomes the primary editor.

------------------------------------------------------------------------

# Workspace

The central workspace hosts task-specific experiences.

Examples

-   Entity Editor
-   Graph Workspace
-   Query Workbench
-   Documentation
-   Review
-   Diff

Only one workspace is active per tab.

------------------------------------------------------------------------

# Bottom Dock

Transient information belongs here.

Examples

-   Problems
-   Query Results
-   Build
-   AI Conversations
-   Git Output
-   Logs

Dock panels are independent and optional.

------------------------------------------------------------------------

# Event Model

Workspace events drive synchronization.

Examples

EntitySelected

TabOpened

WorkspaceChanged

QueryExecuted

ReasoningFinished

DiagnosticsUpdated

PluginLoaded

AIRecommendationCreated

Components subscribe to events instead of polling.

------------------------------------------------------------------------

# Undo / Redo

Workspace supports semantic undo.

Examples

Rename Class

Merge Entities

Move Property

Generate Documentation

Apply AI Refactor

Undo should restore semantic state---not merely text.

------------------------------------------------------------------------

# Persistence

Persist between sessions:

-   layout
-   open tabs
-   panel sizes
-   favorites
-   recent items
-   navigation history
-   graph layouts
-   search history

Users should feel they are returning to the same workspace.

------------------------------------------------------------------------

# Performance

Target interaction latency

Selection

\<50 ms

Navigation

\<100 ms

Workspace switch

\<150 ms

Inspector update

\<50 ms

Search

\<100 ms

Graph update

\<150 ms

------------------------------------------------------------------------

# Plugin Integration

Plugins integrate through Workspace APIs.

Examples

Register:

-   Inspector cards
-   Workspace tabs
-   Explorer nodes
-   Dock panels
-   Commands
-   AI providers

Plugins consume WorkspaceState instead of duplicating it.

------------------------------------------------------------------------

# Multi-Window (Future)

Future releases may support:

-   multiple workspaces
-   detached graph windows
-   secondary inspectors
-   presentation mode

The WorkspaceStore remains authoritative.

------------------------------------------------------------------------

# Architectural Rules

1.  One source of truth.
2.  Current Focus drives the UI.
3.  Views never manually synchronize.
4.  State is predictable.
5.  Plugins are first-class citizens.
6.  Every action is undoable.
7.  Navigation is semantic.

------------------------------------------------------------------------

# Success Criteria

The Workspace Model is successful when users stop thinking about
windows, panels, and files, and instead feel they are directly
interacting with a coherent semantic environment. Every part of the
interface should appear synchronized, responsive, and aware of the
user's current context, making Strixonomy feel like a true modern IDE
rather than a collection of tools.
