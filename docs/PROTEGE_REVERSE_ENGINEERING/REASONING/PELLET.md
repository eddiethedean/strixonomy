# PELLET.md

# Pellet Reasoner Reverse Engineering Specification

## Purpose

Pellet is an OWL 2 reasoner that integrates with Protégé to perform logical inference, ontology validation, consistency checking, realization, and query answering. Historically, Pellet has also been known for support of SWRL rules and explanation facilities, making it a valuable complementary reasoner alongside HermiT.

This document describes Pellet's role within Protégé and outlines how Strixonomy should support Pellet through a modern, pluggable reasoner architecture.

---

# Design Goals

Pellet integration should:

- Provide standards-compliant OWL 2 reasoning
- Support explanation workflows
- Expose SWRL-aware reasoning when available
- Integrate seamlessly with the workspace
- Operate interchangeably with other supported reasoners

---

# Core Capabilities

Pellet commonly supports:

- Ontology consistency checking
- Class classification
- Object property classification
- Data property classification
- Instance realization
- Subsumption testing
- Equivalent class detection
- Unsatisfiable class detection
- SWRL rule reasoning (where supported)
- Explanation generation

---

# Typical Workflow

1. Open an ontology.
2. Select Pellet as the active reasoner.
3. Synchronize ontology changes.
4. Run classification.
5. Inspect inferred hierarchies.
6. Review inconsistencies or explanations.
7. Continue editing and repeat.

---

# Protégé Integration

Typical commands:

- Select Pellet
- Start reasoner
- Stop reasoner
- Synchronize
- Classify ontology
- Check consistency
- Generate explanations

Status information should include:

- Active reasoner
- Busy/idle state
- Classification progress
- Last successful execution
- Error conditions

---

# Inference Support

## Class Classification

Infers superclass and subclass relationships.

## Property Classification

Infers property hierarchies and equivalent properties.

## Instance Realization

Computes inferred types for individuals.

## Consistency

Detects logical contradictions within the ontology.

## Unsatisfiable Classes

Highlights classes that cannot have instances.

## SWRL Rules

Where enabled, evaluates SWRL rules together with ontology axioms.

Strixonomy should detect and clearly indicate when rule reasoning is available.

---

# Synchronization

Reasoning should be invalidated after changes to:

- Classes
- Object properties
- Data properties
- Individuals
- Imports
- SWRL rules
- Logical axioms

Incremental synchronization should be preferred where supported.

---

# Explanation Support

Pellet is frequently used for ontology explanations.

Users should be able to inspect:

- Why an ontology is inconsistent
- Why a class is unsatisfiable
- Why an inference was produced

Each explanation should reference supporting axioms and allow navigation back to source editors.

---

# Performance Considerations

Large ontologies require:

- Background execution
- Progress reporting
- Cancellation
- Resource monitoring
- Incremental updates where possible

The UI must remain responsive throughout reasoning.

---

# Error Handling

Report:

- Ontology inconsistencies
- Unsupported constructs
- SWRL processing failures
- Timeouts
- Internal engine errors

Diagnostics should distinguish ontology problems from runtime failures.

---

# Events

Representative events:

- ReasonerSelected
- ReasonerStarted
- ReasonerStopped
- ReasonerSynchronized
- ClassificationCompleted
- ConsistencyChecked
- ExplanationGenerated

---

# Accessibility

Requirements:

- Keyboard-accessible controls
- Accessible progress updates
- Screen-reader friendly diagnostics
- Status indicators not dependent solely on color

---

# Strixonomy Modernization

Recommended improvements:

- Unified reasoner service API
- Background worker execution
- Live incremental reasoning
- Visual explanation graphs
- AI-generated summaries of explanations
- Side-by-side comparison with HermiT and ELK
- Performance telemetry dashboard
- Workspace-wide reasoning status

---

# Reasoner Service Interface

Pellet should implement the same interface as every supported engine.

Suggested operations:

- initialize()
- classify()
- check_consistency()
- realize()
- explain()
- synchronize()
- cancel()
- dispose()

---

# Feature Parity Checklist

Lifecycle

- [ ] Select Pellet
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
- [ ] SWRL reasoning (when supported)

Diagnostics

- [ ] Explanations
- [ ] Progress reporting
- [ ] Cancellation
- [ ] Error reporting

Platform

- [ ] Background execution
- [ ] Event integration
- [ ] Accessibility
- [ ] Plugin compatibility

---

# Beyond Protégé

Strixonomy should expose Pellet through a provider model rather than coupling UI logic to a specific implementation. This allows users to switch between Pellet, HermiT, ELK, future Rust-native engines, or remote reasoning services without changing workflows.

---

# Summary

Pellet complements Protégé with mature OWL reasoning, explanation capabilities, and support for rule-enhanced workflows. Strixonomy should preserve compatibility while modernizing execution, diagnostics, extensibility, and user experience through a unified reasoner abstraction and responsive interface.
