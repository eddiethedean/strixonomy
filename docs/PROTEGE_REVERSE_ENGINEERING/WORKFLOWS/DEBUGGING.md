# DEBUGGING.md

# Ontology Debugging Workflow
## Reverse Engineering Specification for Protégé and Design Blueprint for Strixonomy

## Purpose

Ontology debugging is the process of identifying, understanding, and resolving logical, structural, and modeling problems in an ontology. Protégé combines reasoners, explanation tools, validation views, and manual inspection to support debugging. Strixonomy should unify these capabilities into an integrated debugging experience similar to modern software IDEs.

---

# Goals

A debugging workflow should:

- Detect logical inconsistencies quickly
- Explain why problems occur
- Navigate directly to affected axioms
- Support iterative fixes
- Integrate tightly with reasoning
- Scale to large ontologies

---

# High-Level Workflow

```text
Edit Ontology
      │
      ▼
Run Validation
      │
      ▼
Run Reasoner
      │
      ▼
Collect Diagnostics
      │
      ▼
Explain Problems
      │
      ▼
Navigate to Source
      │
      ▼
Apply Fixes
      │
      ▼
Reclassify
```

---

# Debugging Categories

## Syntax Errors

Examples:

- Invalid Manchester syntax
- Malformed IRIs
- Invalid prefixes
- Unsupported serialization

Should be detected immediately during editing.

---

## Modeling Errors

Examples:

- Missing domains
- Missing ranges
- Duplicate labels
- Orphan classes
- Deprecated references
- Missing documentation

These should be reported as quality diagnostics.

---

## Logical Errors

Typical issues include:

- Inconsistent ontology
- Unsatisfiable classes
- Conflicting restrictions
- Impossible cardinalities
- Invalid disjointness

Require reasoner support.

---

## Import Errors

Detect:

- Missing imports
- Broken URLs
- Version conflicts
- Circular dependencies

---

## Instance Errors

Examples:

- Invalid datatype values
- Missing required assertions
- Violated property expectations
- Unexpected inferred types

---

# Diagnostics Panel

Each diagnostic should include:

- Severity
- Category
- Message
- Affected entity
- Supporting axioms
- Suggested resolution

Severity levels:

- Error
- Warning
- Information

---

# Explanation Workflow

1. Select a diagnostic.
2. Request explanation.
3. Display supporting axioms.
4. Visualize dependency chain.
5. Navigate to editors.
6. Apply fix.
7. Re-run reasoning.

---

# Navigation

Diagnostics should support:

- Open entity
- Open axiom
- Highlight in hierarchy
- Highlight in graph
- Show usages

---

# Validation Pipeline

Recommended order:

1. Syntax
2. Namespace
3. Imports
4. Ontology quality rules
5. Reasoning
6. Explanations

Each stage publishes structured diagnostics.

---

# Continuous Validation

While editing:

- Validate IRIs
- Validate Manchester syntax
- Detect duplicate labels
- Detect missing metadata

Heavy logical reasoning should execute asynchronously.

---

# Reasoning Integration

After reasoning:

Display:

- Inferred hierarchy
- Unsatisfiable classes
- Equivalent classes
- Consistency status

Users should compare asserted and inferred models.

---

# AI-Assisted Debugging

AI may assist by:

- Explaining diagnostics
- Suggesting ontology repairs
- Detecting modeling smells
- Recommending better restrictions
- Drafting annotations

Suggestions should never be applied automatically.

---

# Events

Representative events:

- ValidationStarted
- ValidationCompleted
- DiagnosticCreated
- DiagnosticResolved
- ExplanationGenerated
- ReasonerCompleted
- WorkspaceUpdated

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen-reader friendly diagnostics
- Accessible explanation panels
- High-contrast support

---

# Strixonomy Modernization

Recommended enhancements:

- IDE-style Problems panel
- Live linting
- Semantic diff viewer
- Visual explanation graphs
- One-click navigation
- Quick fixes
- Background validation workers
- Git blame integration
- AI debugging assistant

---

# Feature Parity Checklist

Validation

- [ ] Syntax validation
- [ ] Ontology validation
- [ ] Import validation
- [ ] Metadata validation

Reasoning

- [ ] Consistency checking
- [ ] Unsatisfiable classes
- [ ] Explanation generation

Diagnostics

- [ ] Problems panel
- [ ] Navigation
- [ ] Severity levels
- [ ] Quick fixes

Platform

- [ ] Background execution
- [ ] Event integration
- [ ] Accessibility
- [ ] Plugin diagnostics

---

# Summary

Protégé's debugging workflow combines validation, reasoning, and explanation to help ontology engineers resolve semantic problems. Strixonomy should preserve these strengths while introducing continuous validation, IDE-style diagnostics, visual explanations, AI-assisted repair suggestions, and a unified debugging experience integrated throughout the workspace.
