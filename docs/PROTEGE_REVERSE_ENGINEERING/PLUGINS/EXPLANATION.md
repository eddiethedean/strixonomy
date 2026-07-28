# EXPLANATION.md

# Explanation Plugin Reverse Engineering Specification
## Protégé Explanation Framework and Strixonomy Design Blueprint

## Purpose

The Explanation plugin helps ontology engineers understand *why* a logical consequence exists or *why* an ontology is inconsistent. It bridges the gap between automated reasoning and human understanding by presenting one or more minimal sets of axioms (justifications) that entail a selected inference.

Strixonomy should preserve this capability while providing richer visualizations, AI-assisted explanations, and deeper integration with the editing workflow.

---

# Goals

The explanation system should:

- Explain inferred relationships
- Explain ontology inconsistencies
- Explain unsatisfiable classes
- Present minimal justifications
- Navigate directly to supporting axioms
- Support iterative debugging

---

# Typical Workflow

```text
Run Reasoner
      │
      ▼
Select Inference or Error
      │
      ▼
Generate Explanation
      │
      ▼
Display Justifications
      │
      ▼
Navigate to Source Axioms
      │
      ▼
Edit Ontology
      │
      ▼
Reclassify
```

---

# Supported Explanation Types

- Unsatisfiable class
- Equivalent class
- Inferred subclass
- Inferred property relationship
- Ontology inconsistency
- Individual realization
- Property assertion inference

---

# Justifications

A justification is a minimal set of axioms sufficient to produce a conclusion.

Requirements:

- Show one or more minimal justifications
- Allow switching between alternative justifications
- Highlight common axioms
- Preserve ordering for readability

---

# Workspace Layout

```text
+--------------------------------------------------------------------+
| Selected Inference                                                  |
+--------------------------------------------------------------------+
| Justification List | Explanation Details | Ontology Navigation      |
+--------------------------------------------------------------------+
| Diagnostics / Actions                                               |
+--------------------------------------------------------------------+
```

---

# Explanation Details

Display:

- Target inference
- Supporting axioms
- Entity links
- Ontology source
- Logical construct summaries

Every entity should be clickable.

---

# Navigation

Users should be able to:

- Open entity editor
- Open source ontology
- Show usages
- Highlight in graph
- Copy explanation
- Export explanation

---

# Visualization

Recommended visualizations:

- Dependency graph
- Justification graph
- Inference chain
- Axiom tree
- Before/after comparison

Asserted and inferred relationships should be visually distinct.

---

# Synchronization

Explanations become stale after ontology edits.

The system should:

- Detect stale explanations
- Offer regeneration
- Preserve user context when possible

---

# Performance

Support:

- Background generation
- Cancellation
- Progress reporting
- Explanation caching
- Incremental refresh

---

# AI-Assisted Explanations

AI may provide:

- Plain-language summaries
- Step-by-step logical walkthroughs
- Suggested repairs
- Educational explanations
- Modeling smell detection

AI output should complement—not replace—the formal justification.

---

# Events

Representative events:

- ExplanationRequested
- ExplanationStarted
- ExplanationGenerated
- ExplanationCancelled
- ExplanationInvalidated
- OntologyChanged

---

# Accessibility

Requirements:

- Keyboard navigation
- Screen-reader friendly justification lists
- High-contrast visualization
- Scalable fonts
- Accessible graph descriptions

---

# Plugin Extension Points

Plugins may contribute:

- Alternative explanation engines
- Graph renderers
- Export formats
- Explanation scoring
- Educational overlays

---

# Strixonomy Modernization

Recommended enhancements:

- Split textual/graph views
- Monaco-based explanation viewer
- Interactive dependency graphs
- AI copilot integration
- Saved explanation sessions
- Collaborative review comments
- Git-aware explanation history

---

# Feature Parity Checklist

Core

- [x] Explain inference (unsatisfiable class justifications)
- [ ] Explain inconsistency (dedicated inconsistency workspace — v0.30)
- [x] Multiple justifications (EL alternatives)
- [x] Minimal justifications (trace-based; not full HermiT hitting sets)

Navigation

- [x] Open entity
- [ ] Highlight graph (explanation overlay — v0.30)
- [ ] Export (file export — v0.30; copy shipped)
- [x] Copy

Platform

- [x] Background execution (progress UI + client cancel)
- [x] Caching
- [x] Accessibility (basic ARIA / focus on Explanation panel)
- [ ] Plugin support (stable marketplace API — v0.33)

---

# Beyond Protégé

Strixonomy should elevate explanations from a diagnostic tool to an interactive reasoning workspace where users can inspect, visualize, discuss, annotate, and repair logical consequences with assistance from AI and collaborative review workflows.

---

# Summary

Protégé's Explanation plugin is essential for understanding the results of automated reasoning. By presenting minimal justifications for inferences and inconsistencies, it enables ontology engineers to debug and refine their models. Strixonomy should preserve these capabilities while adding modern visualization, AI-assisted interpretation, collaboration, and richer developer APIs.
