# CLASSES.md

# Protégé Classes Editor Reverse Engineering Specification

## Purpose

The Classes editor is the primary ontology modeling interface in Protégé. It allows users to create, organize, inspect, annotate, and define OWL classes using asserted axioms while viewing inferred classifications from a reasoner.

For Strixonomy, the Classes editor should achieve complete functional parity while modernizing the user experience.

---

# Responsibilities

The Classes editor is responsible for:

- Creating classes
- Editing class metadata
- Managing the class hierarchy
- Defining logical axioms
- Viewing inferred hierarchies
- Managing annotations
- Searching and navigating classes
- Displaying usages
- Supporting refactoring

---

# High-Level Layout

```text
+-----------------------------------------------------------------------+
| Toolbar                                                               |
+-----------------------------------------------------------------------+
| Class Hierarchy | Class Editor                                        |
|                 |------------------------------------------------------|
|                 | IRI                                                  |
|                 | Labels                                               |
|                 | Superclasses                                         |
|                 | Equivalent Classes                                   |
|                 | Disjoint Classes                                     |
|                 | Necessary & Sufficient Conditions                    |
|                 | Restrictions                                         |
|                 | Annotations                                          |
|                 | Usage                                                 |
+-----------------------------------------------------------------------+
```

---

# Class Hierarchy

Displays asserted subclass relationships.

Capabilities:

- Expand/collapse
- Search
- Filter
- Create child
- Create sibling
- Rename
- Delete
- Drag-and-drop (where semantically valid)

Context menu actions commonly include:

- New subclass
- New sibling
- Rename
- Delete
- Show usages
- Copy IRI

---

# Entity Header

Displays:

- Preferred label
- IRI
- Prefix rendering
- Entity type
- Imported/local status

Should support copying both label and IRI.

---

# Class Metadata

Editable fields:

- rdfs:label
- rdfs:comment
- Alternative labels
- Custom annotations
- Version information
- Deprecation status

---

# Logical Definition Sections

## Superclasses

Defines asserted superclass relationships.

Example:

```text
Dog
  SubClassOf Mammal
```

Users should be able to add, edit, and remove superclasses.

---

## Equivalent Classes

Defines semantic equivalence.

Example:

```text
Parent
EquivalentTo
Person and (hasChild some Person)
```

The reasoner uses these axioms for automatic classification.

---

## Disjoint Classes

Declares mutually exclusive classes.

Example:

```text
Male
DisjointWith
Female
```

---

## Restrictions

Supported OWL constructs include:

- someValuesFrom
- allValuesFrom
- hasValue
- minCardinality
- maxCardinality
- exactCardinality
- intersectionOf
- unionOf
- complementOf
- oneOf

The editor should provide structured builders rather than requiring raw syntax.

---

# Manchester Syntax

Protégé supports editing logical expressions using Manchester Syntax.

Example:

```text
Person and
(hasPet some Dog)
```

Strixonomy should additionally provide:

- syntax highlighting
- autocomplete
- inline validation
- quick fixes

---

# Inferred Hierarchy

After reasoning, display:

- inferred superclasses
- inferred subclasses
- equivalent classes
- unsatisfiable classes

Asserted and inferred relationships should be visually distinct.

---

# Usage View

Displays every axiom referencing the selected class.

Capabilities:

- Navigate to reference
- Filter references
- Copy reference
- Group by ontology

---

# Search

Supported search modes:

- Label
- IRI
- Prefix
- Annotation
- Full-text

Recommended Strixonomy improvements:

- fuzzy search
- symbol search
- command palette integration

---

# Refactoring

Supported operations:

- Rename class
- Move class
- Merge classes
- Safe delete
- Extract module
- Replace references

Every refactoring should preview semantic impact.

---

# Validation

The editor should highlight:

- duplicate labels
- orphan classes
- cyclic hierarchies (where unintended)
- inconsistent definitions
- unsatisfiable classes
- missing annotations
- deprecated references

---

# Events

Typical events:

- ClassCreated
- ClassDeleted
- ClassRenamed
- SelectionChanged
- HierarchyChanged
- AnnotationChanged
- ReasonerFinished

Views should update incrementally.

---

# Accessibility

Requirements:

- keyboard navigation
- screen reader labels
- accessible hierarchy tree
- scalable fonts
- high contrast support

---

# Plugin Extension Points

Plugins should contribute:

- editor panels
- validation sections
- visualization panes
- custom annotation editors
- AI assistants

---

# Strixonomy Modernization

Recommended enhancements:

- React component architecture
- Split editor (asserted vs inferred)
- Interactive graph synchronized with hierarchy
- AI-assisted class definition generation
- Git blame/history for classes
- Inline ontology linting
- Live collaboration
- Drag-and-drop ontology canvas
- Property and restriction builders
- Visual explanation of inferences

---

# Feature Parity Checklist

Hierarchy

- [ ] Class tree
- [ ] Search
- [ ] Create child
- [ ] Create sibling
- [ ] Rename
- [ ] Delete

Metadata

- [ ] Labels
- [ ] Comments
- [ ] Custom annotations

Logical Modeling

- [ ] Superclasses
- [ ] Equivalent classes
- [ ] Disjoint classes
- [ ] Restrictions
- [ ] Manchester syntax

Reasoning

- [ ] Inferred hierarchy
- [ ] Equivalent classes
- [ ] Unsatisfiable classes

Utilities

- [ ] Usage view
- [ ] Validation
- [ ] Refactoring

Platform

- [ ] Undo/redo
- [ ] Plugin panels
- [ ] Accessibility
- [ ] Event synchronization

---

# Summary

The Classes editor is the heart of Protégé. It combines hierarchy navigation, semantic modeling, reasoning, annotations, and validation into a unified workspace for OWL class engineering. Strixonomy should preserve these capabilities while delivering a modern IDE experience with richer visualization, AI-assisted modeling, collaborative editing, and a modular React/Rust architecture.
