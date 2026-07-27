# PARITY_GAP_ANALYSIS

# Protégé Desktop Parity Gap Analysis

**Status:** Living Engineering Document\
**Repository Baseline:** Strixonomy v0.18.2 (audit baseline)\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document analyzes the remaining gaps between the current Strixonomy
repository and the defined Protégé Desktop parity scope.

Unlike the feature matrix, which records implementation status, this
document explains **why** each gap exists, **what blocks completion**,
**how it should be resolved**, and **its impact on the 1.0 release**.

It is the primary engineering planning document for closing parity.

------------------------------------------------------------------------

# Executive Summary

The repository audit found that Strixonomy is already a mature ontology
engineering platform with strong support for:

-   Native Rust architecture
-   VS Code/Cursor integration
-   Turtle and OBO authoring
-   Language Server features
-   Semantic refactoring
-   Query workbench
-   Graph visualization
-   Plugin infrastructure
-   Automated testing foundation

The remaining work is concentrated in a relatively small number of deep
architectural gaps rather than hundreds of missing features.

Estimated parity:

  Scope                               Estimated Progress
  --------------------------------- --------------------
  Turtle/OBO engineering workflow                  \~90%
  Full Protégé Desktop parity                  \~65--72%

------------------------------------------------------------------------

# P0 Release-Blocking Gaps

## GAP-001 --- Format-Independent Semantic Editing

### Current State

Editing logic is still closely coupled to serializer-specific write-back
paths.

### Impact

This blocks consistent editing across RDF/XML, OWL/XML, and future
formats.

### Resolution

Implement a canonical semantic change model that all serializers
consume.

### Dependencies

None.

------------------------------------------------------------------------

## GAP-002 --- RDF/XML Write-Back

### Current State

Parsing exists.

Production write-back does not.

### Impact

Many existing Protégé ontologies cannot be edited and saved.

### Resolution

Implement serializer, patch pipeline, and semantic round-trip tests.

### Depends On

GAP-001

------------------------------------------------------------------------

## GAP-003 --- OWL/XML Write-Back

### Current State

Read/index support exists.

Editing does not.

### Resolution

Implement semantic write-back using the canonical change model.

### Depends On

GAP-001

------------------------------------------------------------------------

## GAP-004 --- Complete OWL 2 Authoring

### Current State

Broad authoring support exists.

Several OWL 2 structural constructs remain incomplete.

### Resolution

Inventory every OWL 2 construct and close remaining authoring gaps.

------------------------------------------------------------------------

## GAP-005 --- Workspace Semantics

### Current State

WorkspaceStore provides strong foundations.

Persistent ontology state and live restoration remain incomplete.

### Resolution

Introduce a formal workspace transaction model and full session
restoration.

------------------------------------------------------------------------

## GAP-006 --- Reasoning Parity

### Current State

Classification is strong.

ABox reasoning, realization, and native DL explanations remain
incomplete.

### Resolution

Expand reasoning capabilities and add conformance tests.

------------------------------------------------------------------------

## GAP-007 --- SWRL

### Current State

No production rule authoring subsystem.

### Resolution

Implement:

-   Rule model
-   Parser
-   Editor
-   Validation
-   Serialization
-   Search
-   Reasoner integration

------------------------------------------------------------------------

## GAP-008 --- Executable Parity Verification

### Current State

Parity tracking is primarily documentation-driven.

### Resolution

Create:

-   Machine-readable parity manifest
-   Automated conformance suite
-   Semantic round-trip corpus
-   CI validation

------------------------------------------------------------------------

# P1 Gaps

-   Stable Plugin SDK
-   Complete OntoGraf-style visualization
-   Accessibility verification
-   DL Query refinement
-   Advanced ontology merge workflows
-   Locality-based module extraction
-   Performance benchmarking

------------------------------------------------------------------------

# Root Causes

The remaining gaps are primarily architectural rather than UI-related.

Major themes include:

-   Serializer coupling
-   Missing canonical transaction model
-   Incomplete semantic coverage
-   Test corpus maturity
-   Release verification automation

------------------------------------------------------------------------

# Recommended Engineering Order

1.  Canonical semantic editing
2.  RDF/XML & OWL/XML write-back
3.  Complete OWL 2 authoring
4.  Workspace semantics
5.  Reasoning parity
6.  SWRL
7.  Executable parity verification
8.  Remaining P1 polish

------------------------------------------------------------------------

# Success Metrics

The parity program is complete when:

-   Every P0 gap is closed.
-   All P0 requirements are VERIFIED.
-   Semantic round-trip tests pass.
-   Release gates are satisfied.
-   Remaining differences are documented and intentional.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   CURRENT_LIMITATIONS.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
