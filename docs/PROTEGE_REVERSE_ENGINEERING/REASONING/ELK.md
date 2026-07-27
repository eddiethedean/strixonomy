# ELK.md

# ELK Reasoner Reverse Engineering Specification

## Purpose

ELK is a high-performance OWL reasoner designed primarily for the OWL 2 EL profile. Within Protégé it is commonly used for very large ontologies—especially biomedical ontologies—where fast incremental classification is more important than supporting the full OWL 2 DL language.

For Strixonomy, ELK should be supported as a first-class pluggable reasoner optimized for speed, scalability, and interactive modeling.

---

# Design Goals

ELK integration should:

- Provide extremely fast classification
- Support incremental reasoning
- Keep the UI responsive
- Clearly communicate OWL 2 EL profile limitations
- Be interchangeable with other supported reasoners

---

# OWL 2 EL Profile

ELK is optimized for the OWL 2 EL profile.

Typical supported constructs include:

- Classes
- Subclass axioms
- Equivalent classes
- Existential restrictions (someValuesFrom)
- Property hierarchies
- Transitive properties

Some expressive OWL 2 DL constructs are intentionally outside the EL profile.

The UI should clearly indicate when an ontology contains constructs that cannot be fully reasoned over by ELK.

---

# Core Capabilities

ELK commonly provides:

- Class classification
- Class hierarchy inference
- Equivalent class detection
- Unsatisfiable class detection (within profile)
- Incremental reclassification
- Fast hierarchy updates

---

# Typical Workflow

1. Load ontology.
2. Select ELK.
3. Synchronize changes.
4. Classify ontology.
5. Inspect inferred hierarchy.
6. Continue editing with rapid reclassification.

---

# Protégé Integration

Common actions include:

- Select ELK
- Start reasoner
- Stop reasoner
- Synchronize
- Classify ontology

The workspace should display:

- Active reasoner
- EL profile compatibility
- Classification progress
- Completion time

---

# Inference Support

## Class Classification

Efficient computation of inferred subclass relationships.

## Equivalent Classes

Detects logically equivalent classes supported by the EL profile.

## Unsatisfiable Classes

Highlights contradictions within supported constructs.

## Incremental Classification

One of ELK's defining strengths.

Small ontology changes should trigger fast incremental updates rather than complete recomputation whenever possible.

---

# Profile Awareness

Strixonomy should provide:

- EL profile validation
- Unsupported construct highlighting
- Suggestions to switch to a more expressive reasoner when necessary

---

# Synchronization

Reasoning should be invalidated when:

- Class axioms change
- Property hierarchies change
- Imports change
- EL-compatible logical constructs are modified

Incremental synchronization should be preferred.

---

# Performance

ELK is optimized for:

- Large ontologies
- Low latency
- Frequent edits
- Interactive development

Recommended UI features:

- Progress indicator
- Timing metrics
- Cancellation
- Background execution

---

# Error Handling

Report:

- Unsupported OWL constructs
- Classification failures
- Internal engine errors
- Timeout or cancellation

Diagnostics should distinguish ontology profile issues from runtime failures.

---

# Events

Representative events:

- ReasonerSelected
- ReasonerStarted
- ReasonerStopped
- ClassificationCompleted
- ProfileValidationCompleted
- SynchronizationCompleted

---

# Accessibility

Requirements:

- Keyboard-accessible controls
- Accessible progress reporting
- Screen-reader friendly diagnostics
- Non-color status indicators

---

# Strixonomy Modernization

Recommended enhancements:

- Automatic profile detection
- Real-time EL compatibility indicators
- Incremental background reasoning
- Performance dashboards
- Side-by-side comparison with HermiT and Pellet
- AI explanations for unsupported constructs
- Visual diff of inferred hierarchy changes

---

# Reasoner Service Interface

ELK should implement the common Strixonomy reasoner interface:

- initialize()
- classify()
- synchronize()
- validate_profile()
- cancel()
- dispose()

Optional capabilities should be discoverable through feature flags rather than hard-coded assumptions.

---

# Feature Parity Checklist

Lifecycle

- [ ] Select ELK
- [ ] Start
- [ ] Stop
- [ ] Synchronize
- [ ] Dispose

Inference

- [ ] Classification
- [ ] Equivalent classes
- [ ] Unsatisfiable classes
- [ ] Incremental reasoning

Diagnostics

- [ ] EL profile validation
- [ ] Unsupported construct reporting
- [ ] Progress reporting
- [ ] Cancellation

Platform

- [ ] Background execution
- [ ] Event integration
- [ ] Accessibility
- [ ] Plugin compatibility

---

# Beyond Protégé

Strixonomy should automatically recommend ELK for large EL-profile ontologies and seamlessly switch to more expressive reasoners when users require full OWL 2 DL support. A unified reasoner service should make these transitions transparent while preserving a consistent user experience.

---

# Summary

ELK is the performance-focused reasoner in the Protégé ecosystem, providing exceptionally fast classification for OWL 2 EL ontologies. Strixonomy should preserve ELK compatibility while enhancing profile awareness, diagnostics, visualization, and incremental reasoning through a modern Rust-based reasoner abstraction and responsive React interface.
