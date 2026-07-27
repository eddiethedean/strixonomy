# IMPORTS.md

# Protégé Imports Editor Reverse Engineering Specification

## Purpose

The Imports editor manages ontology dependencies through OWL import declarations. It allows ontology engineers to add, remove, inspect, resolve, and synchronize imported ontologies while maintaining logical consistency across an ontology project.

For Strixonomy, the Imports editor should provide full feature parity with Protégé while modernizing dependency management with package-style workflows, visualization, and validation.

---

# Responsibilities

The Imports editor supports:

- Creating import declarations
- Removing imports
- Viewing the import hierarchy
- Resolving logical and physical IRIs
- Reloading imported ontologies
- Detecting missing imports
- Inspecting the import closure
- Managing editable and read-only imports
- Validation and diagnostics

---

# Workspace Layout

```text
+----------------------------------------------------------------------------+
| Toolbar                                                                    |
+----------------------------------------------------------------------------+
| Import Tree        | Import Inspector                                      |
|                    |--------------------------------------------------------|
|                    | Logical IRI                                            |
|                    | Physical Location                                      |
|                    | Status                                                 |
|                    | Version IRI                                            |
|                    | Import Closure                                         |
|                    | Dependency Graph                                       |
|                    | Diagnostics                                            |
+----------------------------------------------------------------------------+
```

---

# Import Tree

Displays direct and transitive imports.

Capabilities:

- Expand/collapse
- Search
- Filter
- Select active import
- Copy IRI
- Open imported ontology

Visual indicators should distinguish:

- Loaded imports
- Missing imports
- Read-only imports
- Editable imports
- Version mismatches

---

# Creating Imports

Users should be able to import an ontology by:

- Logical IRI
- Physical file
- URL
- Local workspace reference

The editor should validate:

- Duplicate imports
- Circular dependencies
- Invalid IRIs
- Unreachable resources

---

# Import Inspector

Displays metadata including:

- Logical IRI
- Physical document location
- Version IRI
- Serialization format
- Last loaded time
- Editable status
- Import depth

---

# Import Closure

Shows all transitively imported ontologies.

Capabilities:

- Tree view
- Graph view
- Dependency count
- Cycle detection

---

# Resolution

Support mapping between:

- Logical IRIs
- Physical IRIs
- Local workspace paths
- Remote URLs

Modern implementations should allow configurable resolution strategies.

---

# Reloading Imports

Users should be able to:

- Reload selected import
- Reload all imports
- Refresh dependency graph
- Re-run validation

---

# Diagnostics

Detect:

- Missing imports
- Broken URLs
- Duplicate imports
- Circular imports
- Version conflicts
- Unused imports

Each diagnostic should include:

- Severity
- Description
- Suggested fix

---

# Search

Supported search modes:

- Logical IRI
- Physical path
- Version IRI
- Label
- Full-text

Recommended Strixonomy improvements:

- Fuzzy search
- Dependency search
- Package search

---

# Validation

Highlight:

- Invalid IRIs
- Missing resources
- Incompatible versions
- Circular references
- Duplicate declarations
- Stale cache entries

---

# Refactoring

Supported operations:

- Change logical IRI
- Change physical mapping
- Replace import
- Remove unused imports
- Bulk update versions

All operations should preview dependency impact.

---

# Events

Typical events:

- ImportAdded
- ImportRemoved
- ImportReloaded
- ImportResolved
- ImportValidationUpdated
- ActiveOntologyChanged

Views should update incrementally.

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen reader compatibility
- Accessible dependency tree
- High contrast support
- Scalable fonts

---

# Plugin Extension Points

Plugins should contribute:

- Repository providers
- Resolution strategies
- Validation rules
- Dependency visualizations
- Package sources

---

# Strixonomy Modernization

Recommended improvements:

- Package-manager style ontology dependencies
- Dependency lockfile
- Visual dependency graph
- Version pinning
- Workspace-local packages
- Git-backed imports
- Health dashboard
- AI-assisted conflict resolution
- Automatic update suggestions

---

# Recommended Dependency Model

```text
Workspace
 ├── Local Ontologies
 ├── External Packages
 ├── Remote Repositories
 └── Cached Dependencies
```

Support semantic versioning where appropriate and reproducible dependency resolution.

---

# Feature Parity Checklist

Import Management

- [ ] Add import
- [ ] Remove import
- [ ] Reload import
- [ ] Reload all

Inspection

- [ ] Logical IRI
- [ ] Physical location
- [ ] Version IRI
- [ ] Import closure

Validation

- [ ] Missing imports
- [ ] Circular imports
- [ ] Duplicate imports
- [ ] Version conflicts

Utilities

- [ ] Search
- [ ] Diagnostics
- [ ] Dependency graph
- [ ] Refactoring

Platform

- [ ] Undo/redo
- [ ] Plugin extensions
- [ ] Accessibility
- [ ] Event synchronization

---

# Beyond Protégé

Strixonomy should evolve imports into a first-class dependency system similar to modern package managers. Ontologies should be installable, versioned, cached, validated, and visualized through a unified dependency manager with Git integration, reproducible lockfiles, workspace-aware resolution, and extensible repository providers.

---

# Summary

The Imports editor is responsible for managing ontology dependencies and ensuring imported knowledge is available, consistent, and correctly resolved. Protégé provides robust import management centered on OWL import declarations. Strixonomy should preserve that compatibility while introducing modern dependency management, visualization, validation, and collaboration capabilities.
