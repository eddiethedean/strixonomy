# ANNOTATIONS.md

# Protégé Annotation Properties Editor Reverse Engineering Specification

## Purpose

The Annotation Properties editor manages OWL annotation properties and the metadata they attach to ontologies, classes, properties, individuals, and axioms. Unlike logical object and data properties, annotation properties do not affect logical reasoning; they provide documentation, provenance, versioning, labels, and other descriptive metadata.

For Strixonomy, this editor should provide feature parity with Protégé while offering richer editing, validation, and collaboration capabilities.

---

# Responsibilities

The Annotation Properties editor supports:

- Creating annotation properties
- Organizing annotation property hierarchies
- Editing metadata
- Defining superproperties
- Managing domains and ranges (when applicable)
- Viewing usages
- Refactoring annotation properties
- Validation

---

# Workspace Layout

```text
+------------------------------------------------------------------------+
| Toolbar                                                                |
+------------------------------------------------------------------------+
| Annotation Property Tree | Property Inspector                          |
|                          |----------------------------------------------|
|                          | IRI                                          |
|                          | Labels & Comments                            |
|                          | Superproperties                              |
|                          | Domain                                       |
|                          | Range                                        |
|                          | Usage                                        |
|                          | Custom Annotations                           |
+------------------------------------------------------------------------+
```

---

# Annotation Property Hierarchy

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

# Metadata

Editable fields include:

- IRI
- Preferred label
- Comment
- Alternative labels
- Version information
- Deprecation status
- Custom annotations

Common built-in annotation properties include:

- rdfs:label
- rdfs:comment
- owl:versionInfo
- dc:title
- dc:creator
- skos:prefLabel
- skos:altLabel

---

# Domains and Ranges

OWL permits optional domain and range declarations for annotation properties.

Example:

```text
createdBy
Domain owl:Thing
Range xsd:string
```

Editors should support multiple domain and range axioms where appropriate.

---

# Usage View

Displays every location where the annotation property is used.

Examples:

- Ontology annotations
- Class annotations
- Property annotations
- Individual annotations
- Axiom annotations

Capabilities:

- Navigate to usage
- Filter results
- Group by ontology
- Copy references

---

# Search

Supported search modes:

- Label
- IRI
- Annotation value
- Prefix
- Full-text

Recommended Strixonomy improvements:

- Fuzzy search
- Symbol search
- Semantic search
- Command palette integration

---

# Validation

Highlight:

- Duplicate labels
- Missing labels
- Invalid IRIs
- Deprecated annotation properties
- Invalid literal datatypes
- Broken namespace references

Annotation quality rules should be configurable.

---

# Refactoring

Supported operations:

- Rename annotation property
- Move within hierarchy
- Merge annotation properties
- Replace references
- Safe delete

Every refactoring should include an impact preview.

---

# Events

Typical events include:

- AnnotationPropertyCreated
- AnnotationPropertyDeleted
- AnnotationPropertyRenamed
- AnnotationChanged
- HierarchyChanged
- UsageUpdated

Views should update incrementally.

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen reader compatibility
- Accessible hierarchy tree
- High contrast support
- Scalable fonts

---

# Plugin Extension Points

Plugins should contribute:

- Custom metadata editors
- Controlled vocabulary pickers
- Provenance panels
- Validation rules
- AI-assisted annotation generators

---

# Strixonomy Modernization

Recommended improvements:

- Rich-text and Markdown annotation editors
- Provenance templates
- Citation management
- AI-generated documentation
- Ontology documentation preview
- Bulk annotation editing
- Git history for metadata
- Live collaboration
- Metadata quality dashboard

---

# Feature Parity Checklist

Hierarchy

- [ ] Annotation property tree
- [ ] Search
- [ ] Create child
- [ ] Rename
- [ ] Delete

Metadata

- [ ] Labels
- [ ] Comments
- [ ] Version info
- [ ] Custom annotations

Modeling

- [ ] Superproperties
- [ ] Domain
- [ ] Range

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

The Annotation Properties editor provides the metadata foundation for ontology engineering. While annotation properties are not part of logical reasoning, they are essential for documentation, provenance, interoperability, and ontology maintenance. Strixonomy should preserve Protégé's capabilities while extending them with rich editing, metadata quality tooling, AI-assisted documentation, collaborative workflows, and a modern React/Rust architecture.
