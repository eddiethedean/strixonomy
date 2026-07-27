# REFACTORING.md

# Ontology Refactoring Workflow
## Reverse Engineering Specification for Protégé and Design Blueprint for Strixonomy

## Purpose

Ontology refactoring is the process of improving the internal structure of an ontology while preserving its intended semantics. Protégé provides a collection of refactoring operations through menus, context actions, and plugins. Strixonomy should evolve these capabilities into an IDE-quality refactoring system comparable to modern software development environments.

---

# Goals

A refactoring workflow should:

- Preserve ontology semantics whenever possible
- Preview the impact before changes are applied
- Update all references safely
- Be fully undoable
- Scale to very large ontologies
- Integrate with reasoning and version control

---

# Refactoring Lifecycle

```text
Select Entity
      │
      ▼
Choose Refactoring
      │
      ▼
Analyze Dependencies
      │
      ▼
Preview Changes
      │
      ▼
Apply Refactoring
      │
      ▼
Run Validation
      │
      ▼
Reclassify Ontology
      │
      ▼
Review Results
```

---

# Core Refactorings

## Rename Entity

Supports:

- Classes
- Object properties
- Data properties
- Annotation properties
- Individuals

Expected behavior:

- Update all references
- Preserve annotations
- Validate new IRI
- Preview affected axioms

---

## Move Entity

Typical examples:

- Move class to a different superclass
- Move property under another superproperty
- Reorganize module structure

The UI should display before/after hierarchy previews.

---

## Merge Entities

Combine two or more entities into one.

Workflow:

1. Select primary entity.
2. Select entities to merge.
3. Resolve metadata conflicts.
4. Update references.
5. Remove obsolete entities.
6. Re-run reasoning.

---

## Safe Delete

Deletion should never occur without impact analysis.

Preview should include:

- Direct references
- Indirect references
- Imported ontology impact
- Number of affected axioms
- Suggested alternatives

---

## Replace References

Replace every use of one entity with another.

Useful during:

- Vocabulary migration
- Ontology consolidation
- Standardization

---

## Extract Module

Create a reusable ontology module from selected entities.

Options:

- Signature-based extraction
- Dependency-based extraction
- Import-aware extraction

---

## Change Namespace / IRI

Support bulk namespace migration.

Example:

Old:

```
http://example.org/ontology#
```

New:

```
https://company.com/ontology#
```

All affected IRIs should be updated consistently.

---

# Dependency Analysis

Before every refactoring, analyze:

- Class hierarchy
- Property hierarchy
- Individuals
- Logical axioms
- Imports
- Annotations
- Queries
- Graph references

Produce an impact report before execution.

---

# Validation Pipeline

After refactoring:

1. Syntax validation
2. IRI validation
3. Import validation
4. Reasoner synchronization
5. Consistency check
6. Quality rules

Failures should provide direct navigation to affected entities.

---

# Reasoning Integration

Every semantic refactoring should optionally:

- Synchronize the active reasoner
- Refresh inferred hierarchies
- Detect newly unsatisfiable classes
- Compare inferred hierarchies before and after

---

# Version Control

Strixonomy should integrate refactoring with Git.

Recommended workflow:

1. Create branch
2. Apply refactoring
3. Review semantic diff
4. Commit
5. Open pull request

Refactoring metadata should be included in commit messages.

---

# AI-Assisted Refactoring

AI may assist by:

- Detecting modeling smells
- Suggesting merges
- Recommending namespace cleanup
- Proposing missing annotations
- Identifying duplicate concepts

All changes require explicit user approval.

---

# Events

Representative events:

- RefactoringStarted
- DependencyAnalysisCompleted
- PreviewGenerated
- RefactoringApplied
- ValidationCompleted
- ReasonerCompleted
- RefactoringUndone

---

# Accessibility

Requirements:

- Keyboard-first workflow
- Accessible preview tables
- Screen-reader support
- High-contrast compatibility

---

# Strixonomy Modernization

Recommended enhancements:

- JetBrains-style refactoring previews
- Semantic diff viewer
- Graph-aware previews
- Batch refactoring recipes
- Workspace-wide rename
- Cross-project refactoring
- AI-generated migration plans
- Plugin-defined refactorings

---

# Feature Parity Checklist

Core

- [ ] Rename
- [ ] Move
- [ ] Merge
- [ ] Safe delete
- [ ] Replace references
- [ ] Extract module

Analysis

- [ ] Dependency analysis
- [ ] Impact preview
- [ ] Semantic diff

Validation

- [ ] Reasoner integration
- [ ] Consistency check
- [ ] Quality validation

Platform

- [ ] Undo/redo
- [ ] Git integration
- [ ] Plugin support
- [ ] Accessibility

---

# Summary

Protégé provides the foundations for ontology refactoring, but many operations remain relatively manual. Strixonomy should elevate refactoring to a first-class engineering capability with dependency analysis, semantic previews, Git integration, AI-assisted recommendations, and an extensible command-based architecture that makes ontology maintenance as safe and productive as modern software development.
