# HERMIT.md

# HermiT Reasoner Reverse Engineering Specification

## Purpose

HermiT is one of the primary OWL 2 DL reasoners integrated with Protégé. It performs automated logical inference over ontologies, allowing ontology engineers to classify classes, detect inconsistencies, infer property hierarchies, identify equivalent classes, and validate complex OWL models.

This document describes how Protégé integrates HermiT and provides implementation guidance for Strixonomy.

---

# Goals

The reasoner integration should:

- Provide sound and complete OWL 2 DL reasoning
- Synchronize with ontology edits
- Present inferred knowledge separately from asserted knowledge
- Support explanation workflows
- Run safely on large ontologies

---

# Core Capabilities

HermiT supports:

- Ontology consistency checking
- Class hierarchy classification
- Object property hierarchy classification
- Data property hierarchy classification
- Instance realization
- Equivalent class detection
- Unsatisfiable class detection
- Subsumption testing
- Satisfiability testing
- Inference precomputation

---

# Typical Workflow

1. Edit ontology.
2. Select HermiT as the active reasoner.
3. Start or synchronize the reasoner.
4. Compute inferences.
5. Display inferred hierarchies.
6. Investigate warnings or unsatisfiable classes.
7. Continue editing and reclassify as needed.

---

# Protégé Integration

Typical user actions include:

- Select HermiT from the Reasoner menu
- Start reasoner
- Stop reasoner
- Synchronize reasoner
- Classify ontology
- Precompute inferences
- Configure options

The UI should clearly indicate:

- Active reasoner
- Running state
- Classification progress
- Errors
- Last successful classification

---

# Inference Types

## Class Hierarchy

Example:

```text
Dog
 SubClassOf Mammal

Mammal
 SubClassOf Animal
```

The reasoner infers:

```text
Dog
 SubClassOf Animal
```

## Equivalent Classes

Automatically identifies logically equivalent definitions.

## Unsatisfiable Classes

Highlights classes that cannot possibly contain individuals due to contradictory axioms.

## Realization

Computes inferred class memberships for individuals.

## Property Reasoning

Infers:

- subproperties
- equivalent properties
- inverse relationships
- transitive implications

---

# Synchronization Model

Reasoning should be invalidated when:

- classes change
- properties change
- individuals change
- imports change
- ontology annotations affecting logic change

A synchronization command should recompute only what is necessary.

---

# Performance

Large ontologies may require:

- incremental reasoning
- cancellation support
- progress reporting
- background execution
- memory monitoring

The UI must remain responsive during reasoning.

---

# Explanation Integration

Users should be able to inspect:

- why a class is unsatisfiable
- why two classes are equivalent
- why a subclass relationship was inferred

Explanations should reference supporting axioms and provide navigation back to the editor.

---

# Error Handling

Detect and report:

- inconsistent ontologies
- unsupported constructs
- timeout conditions
- internal reasoner failures

Diagnostics should distinguish ontology errors from runtime failures.

---

# Events

Typical events:

- ReasonerSelected
- ReasonerStarted
- ReasonerStopped
- ReasonerSynchronized
- ClassificationCompleted
- ConsistencyCheckCompleted
- ExplanationGenerated

Views should subscribe to these events and refresh incrementally.

---

# Accessibility

Requirements:

- Keyboard accessible controls
- Screen-reader friendly progress updates
- Non-color indicators for status
- Accessible diagnostics

---

# Strixonomy Modernization

Recommended enhancements:

- Rust-native reasoner abstraction layer
- Background incremental reasoning
- Parallel inference scheduling
- Live reasoning as users edit
- Graph visualization of inferred relationships
- AI-generated explanations of logical results
- Reasoning performance profiler
- Multiple reasoner comparison mode
- Workspace-wide reasoning dashboard

---

# Reasoner Abstraction

Strixonomy should define a stable interface implemented by HermiT adapters and future engines.

Suggested capabilities:

- initialize()
- classify()
- check_consistency()
- realize()
- precompute()
- explain()
- cancel()
- dispose()

---

# Feature Parity Checklist

Lifecycle

- [ ] Select reasoner
- [ ] Start
- [ ] Stop
- [ ] Synchronize
- [ ] Dispose

Inference

- [ ] Classification
- [ ] Consistency checking
- [ ] Realization
- [ ] Equivalent classes
- [ ] Unsatisfiable classes
- [ ] Property inference

Diagnostics

- [ ] Explanations
- [ ] Error reporting
- [ ] Progress reporting
- [ ] Cancellation

Platform

- [ ] Background execution
- [ ] Event integration
- [ ] Accessibility
- [ ] Plugin compatibility

---

# Beyond Protégé

Strixonomy should not hard-code HermiT into the UI. Instead, HermiT should be one implementation behind a reasoner service that supports interchangeable engines (such as ELK, Pellet, Rust-native reasoners, or remote reasoning services) while presenting a unified user experience.

---

# Summary

HermiT is a cornerstone of Protégé's ontology engineering workflow, providing complete OWL 2 DL reasoning and automated inference. Strixonomy should preserve compatibility with HermiT while modernizing execution, visualization, diagnostics, extensibility, and developer APIs through a language-agnostic reasoner abstraction.
