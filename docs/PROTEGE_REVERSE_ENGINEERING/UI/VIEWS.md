
# VIEWS.md

# Protégé View System Reverse Engineering Specification

## Purpose

Views are the fundamental UI building blocks of the Protégé workspace. A view presents a synchronized perspective of the active ontology, selected entity, reasoning results, search results, or project metadata. Multiple views work together to create each workspace tab.

Unlike editors, which focus on modifying a selected entity, views are responsible for navigation, inspection, visualization, filtering, and contextual information.

---

# Design Goals

A view should:

- Present a focused representation of ontology data
- Stay synchronized with the current selection
- Support docking, floating, hiding, and resizing
- Refresh automatically after ontology changes
- Be independently extensible by plugins

---

# View Architecture

```text
Workspace
 ├── Tabs
 │    ├── Classes
 │    ├── Object Properties
 │    ├── Data Properties
 │    ├── Annotation Properties
 │    ├── Individuals
 │    └── DL Query
 │
 └── Views
      ├── Navigation
      ├── Editor
      ├── Usage
      ├── Annotations
      ├── Inferred Hierarchy
      ├── Metrics
      ├── Search
      └── Plugin Views
```

---

# Core View Categories

## Navigation Views

Purpose:

Provide fast movement through ontology structures.

Examples:

- Class hierarchy
- Object property hierarchy
- Data property hierarchy
- Annotation property hierarchy
- Individual hierarchy
- Imports tree

Common capabilities:

- Expand/collapse
- Filter
- Search
- Drag-and-drop (where supported)
- Context menus

---

## Editor Views

Display editable information for the selected entity.

Typical sections include:

- Labels
- IRI
- Superclasses
- Equivalent classes
- Restrictions
- Annotations
- Property assertions

Editor views are synchronized with navigation selections.

---

## Usage View

Shows where an entity is referenced.

Typical information:

- Referencing axioms
- Ontology imports
- Annotation usage
- Logical dependencies

Supports:

- Double-click navigation
- Filtering
- Copying references

---

## Annotation View

Displays ontology and entity annotations.

Examples:

- rdfs:label
- rdfs:comment
- dc:title
- owl:versionInfo

Supports add, edit, delete, and ordering.

---

## Inferred Views

Visible after reasoning.

Examples:

- Inferred class hierarchy
- Inferred property hierarchy
- Equivalent classes
- Unsatisfiable classes

Should distinguish asserted vs inferred relationships.

---

## Search View

Provides entity discovery.

Search modes may include:

- Label
- IRI
- Annotation
- Prefix
- Full-text

Results should synchronize with editors and navigation.

---

## Metrics View

Displays ontology statistics.

Common metrics:

- Classes
- Properties
- Individuals
- Axioms
- Imports
- Annotation counts

---

## Explanation View

Used alongside reasoning.

Displays:

- Unsatisfiable explanations
- Logical conflicts
- Supporting axioms

---

## Graph Views

Typically plugin-provided.

Examples:

- OntoGraf
- OWLViz
- VOWL-inspired visualizations

Capabilities:

- Pan
- Zoom
- Expand neighborhood
- Highlight selected entity

---

# View Lifecycle

Views generally follow this lifecycle:

1. Create
2. Initialize
3. Subscribe to workspace events
4. Render
5. Refresh after ontology changes
6. Refresh after selection changes
7. Dispose

---

# Synchronization Model

Views respond to events such as:

- Active ontology changed
- Entity selected
- Ontology modified
- Reasoner completed
- Search executed
- Imports changed

Synchronization should avoid unnecessary refreshes.

---

# Docking Behavior

Views should support:

- Dock
- Float
- Hide
- Restore
- Resize
- Reset layout

Layout should persist across sessions.

---

# Plugin-Contributed Views

Plugins should be able to register:

- New dockable panels
- New visualization views
- Specialized editors
- Validation dashboards
- Custom inspectors

Required metadata:

- View ID
- Title
- Icon
- Default location
- Persistence key

---

# Accessibility

Every view should provide:

- Keyboard navigation
- Focus management
- Screen reader labels
- High contrast compatibility
- Scalable text

---

# Strixonomy Modernization

Recommended improvements:

- React component-based view system
- Virtualized trees for very large ontologies
- Multiple synchronized graph canvases
- Split editor support
- Tabbed panels
- Command-palette integration
- AI insight panels
- Git-aware change indicators
- Collaborative cursors and presence
- Workspace profiles (Modeling, Reasoning, Review)

---

# Recommended View Types for Strixonomy

Navigation

- Class Explorer
- Property Explorer
- Individual Explorer
- Import Explorer

Inspection

- Entity Inspector
- Usage Explorer
- Annotation Inspector
- Metrics Dashboard

Visualization

- Graph Canvas
- Dependency Graph
- Import Graph

Reasoning

- Classification Results
- Explanations
- Validation Issues

Developer

- Event Log
- Plugin Diagnostics
- Workspace State

---

# Feature Parity Checklist

Navigation

- [ ] Hierarchy trees
- [ ] Search
- [ ] Filtering
- [ ] Context menus

Editing

- [ ] Entity editor
- [ ] Annotation editor
- [ ] Property editor

Reasoning

- [ ] Inferred hierarchy
- [ ] Explanation view
- [ ] Validation issues

Visualization

- [ ] Graph views
- [ ] Plugin visualizations

Workspace

- [ ] Docking
- [ ] Floating
- [ ] Layout persistence
- [ ] Plugin registration

---

# Summary

Protégé's view architecture enables multiple synchronized perspectives on the same ontology, allowing users to navigate, inspect, edit, reason over, and visualize complex semantic models. Strixonomy should preserve this flexible, dockable architecture while modernizing it with a component-based UI, richer visualizations, real-time collaboration, AI assistance, and a unified extension API.
