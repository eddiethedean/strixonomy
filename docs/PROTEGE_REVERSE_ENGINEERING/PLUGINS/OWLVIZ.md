# OWLVIZ.md

# OWLViz Reverse Engineering Specification
## Plugin Analysis and Design Blueprint for Strixonomy

## Purpose

OWLViz is one of the classic visualization plugins for Protégé. It provides a graph-based visualization of OWL class hierarchies, allowing ontology engineers to inspect asserted and inferred subclass relationships that are difficult to understand from tree views alone.

Strixonomy should preserve the useful concepts introduced by OWLViz while dramatically expanding visualization capabilities into a first-class graph editing experience.

---

# Goals

An OWLViz-style visualization should:

- Display class hierarchies clearly
- Differentiate asserted and inferred relationships
- Synchronize with ontology edits
- Scale to large ontologies
- Integrate with reasoning
- Support rapid navigation

---

# Architecture

```text
Ontology
    │
    ▼
Reasoner
    │
    ▼
Hierarchy Model
    │
    ▼
Graph Layout
    │
    ▼
Interactive Visualization
```

---

# Primary Features

OWLViz focuses on:

- Asserted class hierarchy
- Inferred class hierarchy
- Expand/collapse hierarchy
- Node selection
- Zoom and pan
- Automatic layout
- Navigation back to ontology editors

---

# Visualization Modes

## Asserted View

Displays only axioms explicitly authored by the user.

Use cases:

- Reviewing manual modeling
- Understanding authored structure
- Editing hierarchy

## Inferred View

Displays hierarchy after reasoning.

Use cases:

- Validating ontology semantics
- Detecting unexpected inferences
- Finding modeling errors

---

# Graph Elements

## Nodes

Represent OWL classes.

Display:

- Preferred label
- IRI (optional)
- Status indicators
- Selection state

## Edges

Represent subclass relationships.

Recommended edge types:

- Asserted
- Inferred

These should be visually distinguishable.

---

# Interaction

Users should be able to:

- Select nodes
- Center selected node
- Zoom
- Pan
- Expand/collapse branches
- Open class editor
- Show usages
- Copy IRI

---

# Layout

Useful layout algorithms include:

- Hierarchical
- Layered DAG
- Force-directed (optional)
- Radial (optional)

The layout engine should support incremental updates.

---

# Synchronization

Refresh graph after:

- Ontology edits
- Reasoner completion
- Refactoring
- Import changes

Avoid full redraws whenever possible.

---

# Search

Graph search should support:

- Labels
- IRIs
- Prefixes
- Fuzzy search

Results should animate and center automatically.

---

# Performance

Support:

- Virtualized rendering
- Progressive expansion
- Lazy node creation
- Cached layouts
- Background layout computation

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen-reader summaries
- High-contrast themes
- Zoom support
- Non-color edge differentiation

---

# Plugin Extension Points

Allow plugins to contribute:

- Layout algorithms
- Node renderers
- Edge renderers
- Analysis overlays
- Export formats

---

# Strixonomy Modernization

Recommended improvements:

- React Flow-based canvas
- Live synchronization with editors
- Editable graph nodes
- Drag-and-drop hierarchy editing
- Inference overlays
- AI graph summaries
- Collaboration presence
- Git diff overlays
- Minimap
- Timeline replay

---

# Feature Parity Checklist

Visualization

- [ ] Asserted hierarchy
- [ ] Inferred hierarchy
- [ ] Zoom
- [ ] Pan
- [ ] Selection

Navigation

- [ ] Open editor
- [ ] Search
- [ ] Center node
- [ ] Expand/collapse

Platform

- [ ] Reasoner synchronization
- [ ] Background layout
- [ ] Accessibility
- [ ] Plugin layouts

---

# Beyond OWLViz

Rather than treating visualization as a standalone plugin, Strixonomy should make graph visualization a core workspace surface shared by modeling, reasoning, debugging, refactoring, collaboration, and AI assistance.

---

# Summary

OWLViz pioneered graphical ontology visualization inside Protégé by exposing asserted and inferred class hierarchies through an interactive graph. Strixonomy should preserve these capabilities while evolving them into a scalable, graph-native ontology engineering experience tightly integrated with the rest of the platform.
