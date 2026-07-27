# DATA_PROPERTIES.md

# Protégé Data Properties Editor Reverse Engineering Specification

## Purpose

The Data Properties editor manages OWL data properties, which relate individuals to literal values such as strings, numbers, dates, booleans, and other XML Schema datatypes. It provides tools to model datatype semantics, define domains, organize property hierarchies, and support ontology validation and reasoning.

For Strixonomy, this editor should provide full functional parity with Protégé while offering a modern, component-based user experience.

---

# Responsibilities

The Data Properties editor supports:

- Creating data properties
- Organizing the data property hierarchy
- Editing property metadata
- Defining domains
- Defining literal ranges
- Configuring property characteristics
- Managing equivalent and disjoint properties
- Viewing usages
- Validation and refactoring

---

# Workspace Layout

```text
+------------------------------------------------------------------------+
| Toolbar                                                                |
+------------------------------------------------------------------------+
| Data Property Tree | Property Inspector                                |
|                    |----------------------------------------------------|
|                    | IRI                                                |
|                    | Labels & Annotations                               |
|                    | Superproperties                                    |
|                    | Equivalent Properties                              |
|                    | Disjoint Properties                                |
|                    | Domain                                              |
|                    | Range (Datatype)                                    |
|                    | Characteristics                                     |
|                    | Usage                                                |
+------------------------------------------------------------------------+
```

---

# Data Property Hierarchy

Displays asserted subproperty relationships.

Common actions:

- Create child property
- Create sibling property
- Rename
- Delete
- Search
- Filter
- Expand/collapse

---

# Metadata

Editable metadata includes:

- IRI
- Preferred label
- Comments
- Alternative labels
- Version annotations
- Deprecation status
- Custom annotations

---

# Domains

Defines the class whose instances may use the property.

Example:

```text
hasAge
Domain Person
```

Multiple domains should be supported.

---

# Ranges

Ranges define the datatype of literal values.

Common XML Schema datatypes:

- xsd:string
- xsd:boolean
- xsd:integer
- xsd:decimal
- xsd:float
- xsd:double
- xsd:date
- xsd:dateTime
- xsd:time
- xsd:anyURI

Example:

```text
hasAge
Range xsd:integer
```

Support datatype restrictions and custom datatypes where applicable.

---

# Characteristics

Common characteristics include:

- Functional

Unlike object properties, data properties are not transitive or symmetric.

---

# Equivalent Properties

Allows semantically equivalent data properties to be declared.

---

# Disjoint Properties

Supports declaration of mutually exclusive data properties where appropriate.

---

# Usage View

Displays all ontology axioms referencing the selected property.

Capabilities:

- Navigate to references
- Filter
- Copy
- Group by ontology

---

# Search

Supported search modes:

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

Reasoners may infer:

- Subproperty relationships
- Equivalent properties
- Domain implications
- Range implications

Asserted and inferred information should be visually distinct.

---

# Validation

Highlight:

- Missing domains
- Missing ranges
- Invalid datatypes
- Duplicate labels
- Deprecated references
- Conflicting datatype restrictions

---

# Refactoring

Supported operations:

- Rename property
- Move property
- Merge properties
- Replace references
- Safe delete

Each operation should preview semantic impact.

---

# Events

Typical events:

- DataPropertyCreated
- DataPropertyDeleted
- DataPropertyRenamed
- DomainChanged
- RangeChanged
- AnnotationChanged
- ReasonerFinished

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen reader compatibility
- Accessible hierarchy tree
- High contrast mode
- Scalable fonts

---

# Plugin Extension Points

Plugins should contribute:

- Datatype editors
- Validation panels
- Custom annotation editors
- Visualization panes
- AI assistants

---

# Strixonomy Modernization

Recommended improvements:

- React-based inspector
- Datatype picker with documentation
- Constraint builder UI
- AI-assisted datatype suggestions
- Live validation
- Git history
- Collaboration support
- Split asserted/inferred views

---

# Feature Parity Checklist

Hierarchy

- [ ] Data property tree
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
- [ ] Datatype ranges
- [ ] Functional characteristic
- [ ] Equivalent properties
- [ ] Disjoint properties

Reasoning

- [ ] Inferred hierarchy
- [ ] Equivalent properties

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

The Data Properties editor enables ontology engineers to model literal-valued relationships with strong datatype semantics. Protégé provides robust editing, validation, and reasoning support for these properties. Strixonomy should preserve this capability while modernizing the experience with richer datatype tooling, AI-assisted modeling, collaborative workflows, and a command-driven React/Rust architecture.
