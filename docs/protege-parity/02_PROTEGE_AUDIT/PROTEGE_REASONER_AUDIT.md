# PROTEGE_REASONER_AUDIT

# Protégé Desktop Reasoner Audit

**Status:** Living Reference Document\
**Purpose:** Audit the reasoning capabilities exposed by a standard
Protégé Desktop installation and define the equivalent functionality
required for Strixonomy v0.30.

> This audit evaluates reasoning workflows and observable behavior
> rather than matching any specific Java reasoner implementation.

------------------------------------------------------------------------

# Purpose

Protégé supports multiple OWL reasoners through a common interface.
Strixonomy achieves parity by providing equivalent reasoning workflows
through its native Rust-based reasoning architecture and adapters.

The goal is functional equivalence for ontology engineers, not
implementation compatibility.

------------------------------------------------------------------------

# Audit Principles

-   Functional parity over engine parity.
-   Native Rust implementation preferred.
-   Deterministic, reproducible reasoning.
-   Clear distinction between asserted and inferred knowledge.
-   Explicit diagnostics for unsupported constructs.

------------------------------------------------------------------------

# Core Reasoning Workflows

  -----------------------------------------------------------------------------------
  Workflow          Protégé Capability          Strixonomy            Status
                                                Expectation         
  ----------------- --------------------------- ------------------- -----------------
  Select reasoner   Choose installed reasoner   Native              REVIEW
                                                reasoner/adapters   

  Classify ontology Build inferred hierarchy    Classification      REVIEW
                                                workflow            

  Check consistency Detect inconsistencies      Consistency         REVIEW
                                                analysis            

  Synchronize       Refresh after edits         Incremental         REVIEW
  reasoner                                      synchronization     

  Browse inferred   Asserted vs inferred        Hierarchy modes     REVIEW
  hierarchy                                                         

  Explain inference Justification/explanation   Explanation panel   REVIEW

  Stop long-running Cancellation                Graceful            REVIEW
  reasoning                                     cancellation        
  -----------------------------------------------------------------------------------

------------------------------------------------------------------------

# Required Inference Capabilities

## TBox

-   Subclass inference
-   Equivalent class inference
-   Unsatisfiable classes
-   Property hierarchy inference
-   Domain and range implications

## ABox

-   Instance realization
-   Instance checking
-   Inferred class assertions
-   Object property assertions
-   Data property assertions
-   SameIndividual / DifferentIndividuals support where applicable

------------------------------------------------------------------------

# Explanation Requirements

The platform should support workflows for:

-   Unsatisfiable classes
-   Inconsistent ontologies
-   Inferred subclass relationships
-   Missing entailments (where feasible)

Explanations should identify the axioms contributing to the conclusion.

------------------------------------------------------------------------

# Reasoner Lifecycle

Every reasoner should support:

1.  Initialization
2.  Ontology loading
3.  Synchronization after edits
4.  Classification
5.  Query execution
6.  Disposal
7.  Recovery after failures

------------------------------------------------------------------------

# Performance Expectations

-   Responsive interaction on typical ontologies.
-   Incremental updates where practical.
-   Predictable memory usage.
-   Graceful handling of large ontologies.

Performance benchmarks should become part of CI before v0.30.

------------------------------------------------------------------------

# Current Audit Findings

The repository audit identified:

## Strengths

-   Native Rust reasoning infrastructure.
-   Multiple reasoning profiles/adapters.
-   Classification workflows.
-   Explanation framework.
-   Asserted/inferred hierarchy support.

## Remaining Gaps

-   Full ABox reasoning parity.
-   Native DL proof explanations.
-   Complete realization workflows.
-   Richer instance checking.
-   Engine-level cancellation semantics.
-   Expanded conformance corpus.

------------------------------------------------------------------------

# Verification

Reasoning parity should be demonstrated through:

-   OWL conformance fixtures
-   Automated regression tests
-   Semantic equivalence tests
-   Performance benchmarks
-   Cross-version compatibility tests

------------------------------------------------------------------------

# Acceptance Criteria

Reasoning parity is complete only when:

-   P0 reasoning workflows are implemented.
-   Asserted and inferred views remain synchronized.
-   Required explanation workflows are available.
-   Automated reasoning tests pass.
-   Remaining limitations are documented.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   IMPLEMENTATION_EVIDENCE.md
-   REASONING.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
