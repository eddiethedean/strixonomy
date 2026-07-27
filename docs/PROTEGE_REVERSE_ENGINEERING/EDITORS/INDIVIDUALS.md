# INDIVIDUALS.md

# Protégé Individuals Editor Reverse Engineering Specification

## Purpose

The Individuals editor manages OWL named individuals (instances). It allows ontology engineers to create instances of classes, assign object and data property values, define same/different individuals, annotate instances, inspect inferred types, and validate instance data.

For Strixonomy, this editor should preserve Protégé's capabilities while providing a modern, interactive experience for instance modeling and knowledge graph editing.

---

# Responsibilities

The Individuals editor supports:

- Creating named individuals
- Assigning asserted types
- Viewing inferred types
- Editing object property assertions
- Editing data property assertions
- Managing annotations
- Declaring SameIndividual and DifferentIndividuals axioms
- Searching and navigating individuals
- Validation and refactoring

---

# Workspace Layout

```text
+---------------------------------------------------------------------------+
| Toolbar                                                                   |
+---------------------------------------------------------------------------+
| Individual Browser | Individual Inspector                                 |
|                    |-------------------------------------------------------|
|                    | IRI                                                   |
|                    | Labels & Annotations                                  |
|                    | Asserted Types                                        |
|                    | Inferred Types                                        |
|                    | Object Property Assertions                            |
|                    | Data Property Assertions                              |
|                    | Same Individuals                                      |
|                    | Different Individuals                                 |
|                    | Usage                                                  |
+---------------------------------------------------------------------------+
```

---

# Individual Browser

Displays individuals grouped by class or through searchable lists.

Common actions:

- Create individual
- Rename
- Delete
- Search
- Filter
- Copy IRI
- Show usages

---

# Metadata

Editable metadata includes:

- IRI
- Preferred label
- Comments
- Version annotations
- Custom annotations

---

# Asserted Types

Assign one or more classes to the selected individual.

Example:

```text
Fido
Type Dog
```

Multiple asserted types should be supported.

---

# Inferred Types

After reasoning, display all inferred class memberships.

Asserted and inferred types should be visually distinct.

---

# Object Property Assertions

Create relationships between individuals.

Example:

```text
Fido
hasOwner Alice
```

Capabilities:

- Add assertion
- Edit assertion
- Remove assertion
- Navigate to related individual

---

# Data Property Assertions

Assign literal values.

Examples:

```text
hasAge 5
hasName "Fido"
hasVaccinated true
```

Support common XML Schema datatypes and datatype validation.

---

# Same Individuals

Declare equivalent identities.

Example:

```text
SameIndividual:
Bob
Robert
```

---

# Different Individuals

Declare individuals that are explicitly distinct.

Example:

```text
DifferentIndividuals:
Alice
Charlie
```

---

# Usage View

Displays every ontology axiom referencing the selected individual.

Capabilities:

- Navigate
- Filter
- Group by ontology
- Copy references

---

# Search

Supported search modes:

- Label
- IRI
- Annotation
- Full-text
- Type

Recommended Strixonomy improvements:

- Fuzzy search
- Semantic search
- Command palette integration

---

# Validation

Highlight:

- Missing asserted types
- Invalid property values
- Datatype violations
- Broken references
- Duplicate labels
- Deprecated entities
- Logical inconsistencies

---

# Refactoring

Supported operations:

- Rename individual
- Merge individuals
- Replace references
- Safe delete

All refactorings should preview ontology impact.

---

# Events

Typical events:

- IndividualCreated
- IndividualDeleted
- IndividualRenamed
- TypeChanged
- PropertyAssertionChanged
- AnnotationChanged
- ReasonerFinished

Views should update incrementally.

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen reader compatibility
- Accessible property tables
- High contrast support
- Scalable fonts

---

# Plugin Extension Points

Plugins should contribute:

- Custom property editors
- Validation panels
- Timeline/history views
- Graph visualizations
- AI assistants

---

# Strixonomy Modernization

Recommended enhancements:

- Graph-based instance editing
- Inline autocomplete for individuals and properties
- Spreadsheet-style assertion editing
- Bulk editing
- AI-generated sample individuals
- Git history and blame
- Live collaboration
- Knowledge graph canvas synchronized with forms

---

# Feature Parity Checklist

Individuals

- [ ] Create
- [ ] Rename
- [ ] Delete
- [ ] Search

Metadata

- [ ] Labels
- [ ] Comments
- [ ] Custom annotations

Modeling

- [ ] Asserted types
- [ ] Inferred types
- [ ] Object property assertions
- [ ] Data property assertions
- [ ] SameIndividual
- [ ] DifferentIndividuals

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

The Individuals editor is where ontology schemas become concrete knowledge. Protégé provides comprehensive support for instance creation, typing, property assertions, reasoning, and validation. Strixonomy should retain these capabilities while introducing graph-native editing, richer validation, collaborative workflows, AI-assisted instance authoring, and a modern React/Rust architecture.
