# ONTOGRAF.md

# OntoGraf Reverse Engineering Specification
## Plugin Analysis and Design Blueprint for Strixonomy

## Purpose

OntoGraf is a Protégé visualization plugin that provides interactive graph exploration of ontology entities and their relationships. Unlike OWLViz, which primarily focuses on class hierarchies, OntoGraf enables exploration of arbitrary relationships among classes, properties, individuals, and imports.

Strixonomy should preserve OntoGraf's exploratory capabilities while evolving them into a modern graph-native ontology engineering environment.

---

# Goals

An OntoGraf-style visualization should:

- Explore arbitrary ontology relationships
- Support interactive graph expansion
- Integrate with editors and reasoners
- Scale to large knowledge graphs
- Enable analysis and editing from the graph

---

# Core Concepts

Entities represented include:

- Classes
- Object properties
- Data properties
- Annotation properties
- Individuals
- Ontologies (imports)

Relationships include:

- Subclass
- Property assertions
- Domain
- Range
- Equivalent entities
- Imports
- Inferred relationships

---

# Architecture

```text
Ontology
    │
    ▼
Graph Builder
    │
    ▼
Relationship Filters
    │
    ▼
Layout Engine
    │
    ▼
Interactive Graph Canvas
```

---

# Workspace Layout

```text
+--------------------------------------------------------------------+
| Toolbar                                                            |
+--------------------------------------------------------------------+
| Filters | Graph Canvas                          | Inspector         |
|          |                                      |-------------------|
|          |                                      | Selected Entity   |
|          |                                      | Properties        |
|          |                                      | Annotations       |
|          |                                      | Usage             |
+--------------------------------------------------------------------+
```

---

# Graph Exploration

Users should be able to:

- Expand neighbors
- Collapse regions
- Follow relationships
- Focus on an entity
- Pin important nodes
- Hide nodes and edges
- Save graph state

Expansion should be incremental rather than loading the entire ontology.

---

# Filtering

Support filtering by:

- Entity type
- Relationship type
- Namespace
- Ontology
- Assertion vs. inference
- Search query

---

# Layout Algorithms

Recommended layouts:

- Hierarchical
- Force-directed
- Radial
- Circular
- Orthogonal

Layouts should be switchable without rebuilding the graph.

---

# Navigation

Every node should support:

- Open editor
- Show usages
- Rename
- Copy IRI
- Center graph
- Highlight neighborhood

---

# Synchronization

Refresh incrementally after:

- Ontology edits
- Refactorings
- Reasoning
- Import changes
- Selection changes

Selected entities should remain selected when practical.

---

# Performance

Support:

- Virtualized rendering
- Lazy graph expansion
- Cached layouts
- Background layout computation
- Level-of-detail rendering

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen-reader summaries
- High contrast mode
- Scalable fonts
- Non-color edge differentiation

---

# Plugin Extension Points

Plugins may contribute:

- Custom node renderers
- Custom edge renderers
- Layout algorithms
- Analysis overlays
- Export formats
- Graph analytics

---

# AI-Assisted Graph Analysis

AI may assist by:

- Explaining graph regions
- Finding modeling smells
- Suggesting missing relationships
- Identifying redundant structures
- Summarizing neighborhoods

---

# Strixonomy Modernization

Recommended improvements:

- React Flow (or equivalent) canvas
- Multi-tab graph workspaces
- Collaborative graph editing
- Git diff overlays
- Reasoning overlays
- Validation overlays
- Timeline playback
- Minimap
- Command palette integration
- Editable graph operations

---

# Feature Parity Checklist

Graph

- [ ] Interactive graph
- [ ] Incremental expansion
- [ ] Filtering
- [ ] Multiple layouts

Navigation

- [ ] Open editor
- [ ] Search
- [ ] Focus node
- [ ] Inspector

Platform

- [ ] Reasoner synchronization
- [ ] Accessibility
- [ ] Plugin extensions
- [ ] Background rendering

---

# Beyond OntoGraf

Strixonomy should transform graph visualization from a passive exploration tool into an active ontology engineering surface where users can create, edit, refactor, validate, reason over, and collaborate directly within the graph.

---

# Summary

OntoGraf expanded Protégé's visualization capabilities by enabling interactive exploration of ontology relationships beyond simple class hierarchies. Strixonomy should preserve these strengths while integrating graph editing, reasoning, AI assistance, collaboration, and high-performance visualization into a unified ontology engineering platform.
