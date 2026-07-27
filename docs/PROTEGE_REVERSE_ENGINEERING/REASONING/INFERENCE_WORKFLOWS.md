# INFERENCE_WORKFLOWS.md

# Ontology Inference Workflows
## Reverse Engineering Specification for Protégé and Design Blueprint for Strixonomy

## Purpose

Inference workflows describe the end-to-end lifecycle of logical reasoning within an ontology engineering environment. Rather than documenting a specific reasoner, this specification focuses on how users, the workspace, and one or more reasoners cooperate to transform asserted axioms into inferred knowledge.

Strixonomy should preserve Protégé's mature reasoning workflows while modernizing execution, visualization, diagnostics, and collaboration.

---

# Goals

Inference workflows should:

- Keep inferred knowledge synchronized with ontology edits
- Make reasoning understandable to users
- Support multiple interchangeable reasoners
- Scale to very large ontologies
- Minimize unnecessary recomputation
- Keep the UI responsive

---

# High-Level Lifecycle

```text
User edits ontology
        │
        ▼
Ontology becomes "dirty"
        │
        ▼
Reasoner synchronization requested
        │
        ▼
Profile validation
        │
        ▼
Classification
        │
        ▼
Inference generation
        │
        ▼
Workspace refresh
        │
        ▼
Explanation / validation / navigation
```

---

# Primary Workflow: Classification

1. User edits ontology.
2. Workspace marks reasoning state as stale.
3. User (or auto-sync) starts reasoning.
4. Selected reasoner loads current ontology.
5. Classification completes.
6. Inferred hierarchy is published.
7. Views refresh incrementally.
8. Diagnostics are updated.

Expected outputs:

- Inferred class hierarchy
- Property hierarchy
- Equivalent entities
- Unsatisfiable classes
- Updated instance realization (where supported)

---

# Consistency Checking

Purpose:

Determine whether the ontology contains logical contradictions.

Workflow:

1. Synchronize ontology.
2. Execute consistency check.
3. Display overall status.
4. Highlight problematic entities.
5. Offer explanation workflow.

Possible outcomes:

- Consistent
- Inconsistent
- Unknown (failure/cancelled)

---

# Unsatisfiable Class Investigation

Workflow:

1. User selects an unsatisfiable class.
2. Request explanation.
3. Display minimal supporting axioms.
4. Navigate directly to affected entities.
5. Edit ontology.
6. Re-run classification.

---

# Incremental Reasoning

Supported by engines such as ELK.

Workflow:

1. Detect ontology delta.
2. Identify affected regions.
3. Recompute only impacted inferences.
4. Refresh affected views.

Benefits:

- Faster feedback
- Lower CPU usage
- Better interactive editing

---

# Reasoner Switching

Users should be able to change reasoners without changing workflows.

Example:

```text
ELK
 ↓
HermiT
 ↓
Pellet
```

The workspace should preserve:

- Selected entity
- Open editors
- Query history
- Layout
- Diagnostics history

Only inferred results should change.

---

# Explanation Workflow

Workflow:

1. Select inferred relationship or error.
2. Request explanation.
3. Display supporting axioms.
4. Visualize dependency chain.
5. Navigate to source axioms.
6. Modify ontology.
7. Reclassify.

Strixonomy should provide both textual and graphical explanations.

---

# Background Reasoning

Reasoning should execute asynchronously.

Requirements:

- Progress indicator
- Cancellation
- Background worker
- Responsive UI
- Status notifications

---

# Validation Pipeline

Recommended execution order:

1. Syntax validation
2. Ontology profile validation
3. Import validation
4. Classification
5. Consistency checking
6. Quality rules
7. Explanation generation

Each stage should publish structured diagnostics.

---

# Workspace Synchronization

Views subscribing to reasoning events include:

- Class hierarchy
- Property hierarchies
- Individuals
- Usage
- Validation
- Graph view
- DL Query
- Metrics

Only affected regions should refresh.

---

# Event Model

Representative events:

- OntologyChanged
- ReasonerSelected
- SynchronizationStarted
- SynchronizationCompleted
- ClassificationStarted
- ClassificationCompleted
- ConsistencyChecked
- ExplanationsGenerated
- DiagnosticsUpdated
- WorkspaceRefreshed

---

# Diagnostics

Diagnostics should include:

- Severity
- Source
- Affected entity
- Supporting axioms
- Suggested remediation

Severity levels:

- Error
- Warning
- Information

---

# Performance Strategy

Recommended architecture:

- Background worker threads
- Incremental updates
- Cached inference graphs
- Lazy visualization
- Cancellation tokens
- Progress telemetry

---

# Collaboration Workflow

Future collaborative reasoning:

1. User A edits ontology.
2. Shared workspace marks reasoning stale.
3. Background reasoning executes.
4. All collaborators receive updated inferred results.
5. Explanation links remain stable.

---

# AI-Assisted Workflow

Strixonomy should support:

- Explain this inference
- Suggest ontology fixes
- Recommend missing axioms
- Detect modeling smells
- Generate competency questions
- Convert natural language into logical axioms

AI suggestions should never silently modify ontology content.

---

# Recommended Strixonomy Architecture

```text
Workspace
     │
     ▼
Reasoning Service
     │
 ┌───┼─────────────┐
 │   │             │
ELK HermiT      Pellet
 │   │             │
 └───┼─────────────┘
     ▼
Inference Store
     ▼
Workspace Event Bus
     ▼
Views / Editors / Graphs
```

---

# Feature Parity Checklist

Lifecycle

- [ ] Synchronize ontology
- [ ] Classify ontology
- [ ] Consistency check
- [ ] Cancel reasoning

Inference

- [ ] Class hierarchy
- [ ] Property hierarchy
- [ ] Realization
- [ ] Equivalent entities
- [ ] Unsatisfiable classes

Diagnostics

- [ ] Explanations
- [ ] Structured diagnostics
- [ ] Navigation
- [ ] Progress reporting

Performance

- [ ] Background execution
- [ ] Incremental reasoning
- [ ] Cached results

Platform

- [ ] Event bus integration
- [ ] Multi-reasoner support
- [ ] Accessibility
- [ ] Collaboration readiness

---

# Beyond Protégé

Strixonomy should treat inference as a platform capability rather than a menu command. Reasoning should be continuously available through a shared reasoning service that powers editors, queries, graphs, validation, AI assistants, automation, and collaboration.

---

# Summary

Protégé's reasoning workflows revolve around explicit synchronization, classification, explanation, and inspection of inferred knowledge. Strixonomy should retain these proven workflows while evolving them into a responsive, service-oriented architecture with incremental reasoning, shared diagnostics, AI-assisted explanations, collaborative updates, and a unified event-driven reasoning pipeline.
