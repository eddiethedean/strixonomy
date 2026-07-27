# GRAPH_WORKSPACE.md

> **Document type:** Product design specification (target state). **Not a shipped feature list.** See [ROADMAP_MAPPING.md](ROADMAP_MAPPING.md) for release mapping and [SHIPPED.md](../SHIPPED.md) for what works today.


# Strixonomy Graph Workspace Specification

## Purpose

The Graph Workspace is not a visualization panel---it is a primary
editing and exploration environment for semantic knowledge. It should
feel closer to Figma, Miro, and Obsidian Canvas than to the static graph
viewers found in traditional ontology tools.

## Vision

Users should be able to understand, navigate, edit, reason about, and
communicate ontology structure directly from an interactive semantic
canvas.

## Design Principles

-   The graph is a workspace, not a report.
-   Every node is editable.
-   Every edge is meaningful.
-   Layout is persistent.
-   Everything stays synchronized with the Workspace Model.
-   AI augments exploration rather than replacing it.

## Primary Layout

    +---------------------------------------------------------------+
    | Toolbar | Search | Layout | Filters | AI | Saved Views        |
    +---------------------------------------------------------------+
    |                                                               |
    |                    Semantic Canvas                            |
    |                                                               |
    |                                                               |
    +---------------------------------------------------------------+
    | Details | Mini-map | Selection | Problems | History           |
    +---------------------------------------------------------------+

## Semantic Canvas

The canvas supports:

-   Infinite pan and zoom
-   Mouse, touchpad, and keyboard navigation
-   Smooth zoom-to-selection
-   Box selection
-   Multi-selection
-   Drag-and-drop repositioning
-   Undo/redo

## Nodes

Supported node types include:

-   Class
-   Individual
-   Object Property
-   Data Property
-   Annotation Property
-   SHACL Shape
-   Rule
-   Ontology
-   Imported Ontology

Each node displays:

-   Name
-   Type
-   Status badges
-   Diagnostics
-   Optional summary

Double-click opens the Entity Editor.

## Edges

Edges represent semantic relationships.

Examples:

-   Subclass
-   Equivalent class
-   Disjoint
-   Object property
-   Data property
-   Domain
-   Range
-   Import

Users can filter edge categories independently.

## Interaction Model

Single click: - Select node

Double click: - Open Entity Editor

Right click: - Context menu

Drag: - Move nodes

Shift+Drag: - Multi-select

Mouse wheel / trackpad: - Zoom

Space+Drag: - Pan

## Current Focus

Selecting any node updates:

-   Explorer
-   Inspector
-   Entity Editor
-   Documentation
-   Reasoner
-   AI context
-   Breadcrumbs

The graph never owns independent selection state.

## Saved Views

Users may create named graph layouts.

Examples:

-   Clinical Domain
-   Security Model
-   Imports
-   Core Classes
-   Review Layout

Saved views persist with the workspace.

## Semantic Grouping

Nodes may be grouped by:

-   Namespace
-   Module
-   Ontology
-   Package
-   User-defined collections

Groups can collapse and expand.

## Layout Algorithms

Support multiple layouts:

-   Force-directed
-   Hierarchical
-   Radial
-   Orthogonal
-   Circular
-   Manual

Manual edits remain persistent.

## Search

Graph search supports:

-   Entity names
-   Labels
-   IRIs
-   Relationships
-   Diagnostics

Selecting a result smoothly centers the graph.

## AI Features

AI may:

-   Explain neighborhoods
-   Detect modeling patterns
-   Recommend refactors
-   Highlight anomalies
-   Suggest missing relationships
-   Generate summaries

AI actions are contextual and previewable.

## Reasoning Overlay

Optional overlays visualize:

-   Inferred relationships
-   Inconsistencies
-   Unsatisfiable classes
-   Equivalent classes
-   Cycles

Users can toggle overlays independently.

## Collaboration

Future capabilities:

-   Shared cursors
-   Comments
-   Review pins
-   Presentation mode
-   Live collaboration

## Accessibility

Support:

-   Keyboard navigation
-   Screen readers where practical
-   High contrast
-   Reduced motion
-   Zoom without information loss

## Performance Targets

-   Open graph: \<150 ms
-   Zoom: 60 FPS target
-   Pan: 60 FPS target
-   Selection update: \<50 ms
-   Layout switch: \<500 ms
-   Incremental graph updates without full redraw

## Plugin Extension Points

Plugins may contribute:

-   Custom node renderers
-   Edge decorators
-   Graph overlays
-   Toolbar actions
-   Layout algorithms
-   AI providers
-   Context menu actions

## Success Criteria

The Graph Workspace succeeds when users choose it as their primary way
to understand and evolve an ontology---not because they need a picture,
but because it becomes a first-class semantic engineering environment.
The graph should feel alive, continuously synchronized with the rest of
the IDE, and capable of scaling from small ontologies to enterprise
knowledge graphs without sacrificing clarity or performance.
