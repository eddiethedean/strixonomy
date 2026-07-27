# OBJECT_PROPERTIES.md

# Protégé Object Properties Editor Reverse Engineering Specification

## Purpose

The Object Properties editor is responsible for defining relationships between individuals in an OWL ontology. Object properties connect one individual to another and form the backbone of semantic relationships used by reasoners.

For Strixonomy, this editor should achieve feature parity with Protégé while providing a modern, IDE-like experience.

---

# Responsibilities

The Object Properties editor supports:

- Creating object properties
- Managing the property hierarchy
- Editing property metadata
- Defining domains and ranges
- Configuring OWL property characteristics
- Defining inverse, equivalent, and disjoint properties
- Creating property chains
- Viewing inferred relationships
- Refactoring and validation

---

# High-Level Layout

```text
+------------------------------------------------------------------------+
| Toolbar                                                                |
+------------------------------------------------------------------------+
| Property Hierarchy | Property Inspector                                |
|                    |----------------------------------------------------|
|                    | IRI                                                |
|                    | Labels & Annotations                               |
|                    | Superproperties                                    |
|                    | Equivalent Properties                              |
|                    | Disjoint Properties                                |
|                    | Domain                                              |
|                    | Range                                               |
|                    | Characteristics                                     |
|                    | Inverse Property                                    |
|                    | Property Chains                                     |
|                    | Usage                                                |
+------------------------------------------------------------------------+
```

---

# Property Hierarchy

Displays asserted subproperty relationships.

Common actions:

- Create child property
- Create sibling property
- Rename
- Delete
- Search
- Filter
- Expand/collapse
- Copy IRI

---

# Property Metadata

Editable fields include:

- IRI
- Preferred label
- Comment
- Alternative labels
- Version annotations
- Deprecation status
- Custom annotations

---

# Domains

Defines the class(es) for which the property applies.

Example:

```text
hasParent
Domain Person
```

Multiple domain axioms should be supported.

---

# Ranges

Defines the class of values that may appear as the target.

Example:

```text
hasParent
Range Person
```

Multiple range axioms should be supported.

---

# Property Characteristics

Supported OWL characteristics include:

- Functional
- Inverse Functional
- Transitive
- Symmetric
- Asymmetric
- Reflexive
- Irreflexive

Each characteristic should be independently configurable.

---

# Inverse Properties

Allows users to associate inverse relationships.

Example:

```text
hasParent
InverseOf hasChild
```

Navigation between inverse properties should be bidirectional.

---

# Equivalent Properties

Declares semantically equivalent properties.

Example:

```text
owns
EquivalentTo possesses
```

---

# Disjoint Properties

Specifies properties that cannot both apply to the same pair of individuals.

---

# Subproperty Relationships

Supports:

- Superproperties
- Subproperties
- Property inheritance

The hierarchy should synchronize with reasoning results.

---

# Property Chains

OWL 2 supports property chain axioms.

Example:

```text
hasParent o hasSibling -> hasUncleOrAunt
```

The editor should provide a visual builder in addition to textual editing.

---

# Usage View

Displays every ontology axiom referencing the selected property.

Capabilities:

- Navigate to referencing axiom
- Group by ontology
- Filter results
- Copy references

---

# Search

Search modes:

- Label
- IRI
- Annotation
- Prefix
- Full-text

Recommended Strixonomy enhancements:

- Fuzzy search
- Symbol search
- Command palette integration

---

# Reasoning

After classification, users should be able to inspect:

- Inferred subproperties
- Equivalent properties
- Property hierarchy changes
- Logical inconsistencies

Asserted and inferred relationships should be visually distinct.

---

# Validation

Highlight:

- Missing domain/range
- Conflicting characteristics
- Invalid property chains
- Duplicate labels
- Deprecated references
- Unsatisfiable logical definitions

---

# Refactoring

Supported operations:

- Rename property
- Move property
- Merge properties
- Replace references
- Safe delete

Every refactoring should provide an impact preview.

---

# Events

Typical events include:

- ObjectPropertyCreated
- ObjectPropertyDeleted
- ObjectPropertyRenamed
- PropertyHierarchyChanged
- DomainChanged
- RangeChanged
- ReasonerFinished

Views should refresh incrementally.

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen reader support
- Accessible hierarchy tree
- High-contrast mode
- Scalable fonts

---

# Plugin Extension Points

Plugins should contribute:

- Property inspectors
- Validation panels
- Visualization panes
- Custom annotation editors
- AI assistants

---

# Strixonomy Modernization

Recommended improvements:

- React component architecture
- Graph-based relationship visualization
- Drag-and-drop domain/range editor
- Visual property chain builder
- AI-assisted property generation
- Git history for ontology entities
- Live collaboration
- Inline ontology linting
- Split asserted/inferred views

---

# Feature Parity Checklist

Hierarchy

- [ ] Property hierarchy
- [ ] Search
- [ ] Create child
- [ ] Rename
- [ ] Delete

Metadata

- [ ] Labels
- [ ] Comments
- [ ] Custom annotations

Modeling

- [ ] Domains
- [ ] Ranges
- [ ] Characteristics
- [ ] Inverse properties
- [ ] Equivalent properties
- [ ] Disjoint properties
- [ ] Property chains

Reasoning

- [ ] Inferred hierarchy
- [ ] Equivalent properties
- [ ] Consistency support

Utilities

- [ ] Usage view
- [ ] Validation
- [ ] Refactoring

Platform

- [ ] Undo/redo
- [ ] Plugin extensions
- [ ] Accessibility
- [ ] Event synchronization

---

# Summary

The Object Properties editor is central to modeling semantic relationships in OWL. Protégé provides mature support for hierarchy management, logical property definitions, reasoning, and annotations. Strixonomy should preserve these capabilities while introducing modern visualization, AI-assisted modeling, collaborative editing, and a command-driven architecture built on a Rust backend with a React-based UI.
