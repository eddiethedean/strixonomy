# PARITY_SCOPE

# Protégé Desktop Parity Scope

**Status:** Normative Specification (frozen)\
**Frozen:** 2026-07-13 | Audit baseline: v0.18.2\
**Target Release:** Strixonomy v0.30.0

------------------------------------------------------------------------

# Purpose

This document defines exactly what **"Protégé Desktop parity"** means
for Strixonomy v0.30. It establishes the official scope of the parity
program so implementation, testing, documentation, and release decisions
are measured against a single, stable definition.

This scope is based on:

-   The audited capabilities of the current Strixonomy repository
-   A standard installation of Protégé Desktop
-   The project goal of replacing Protégé for day-to-day ontology
    engineering

------------------------------------------------------------------------

# Guiding Principles

The parity effort prioritizes:

1.  Functional parity over visual parity.
2.  Semantic correctness over serializer-specific behavior.
3.  Modern UX over Swing replication.
4.  Native Rust implementations rather than JVM dependencies.
5.  Objective evidence over marketing claims.
6.  Automated verification before release.

------------------------------------------------------------------------

# In Scope

Strixonomy v0.30 should provide equivalent workflows for:

## Ontology Lifecycle

-   Create, open, save, save all
-   Recent projects
-   Import/export
-   Ontology metadata
-   Prefix management

## Multi-Ontology Workspaces

-   Multiple loaded ontologies
-   Active ontology selection
-   Imports closure
-   Dirty-state tracking
-   Session persistence
-   Navigation history

## OWL 2 Authoring

Support authoring, editing, deletion, validation, serialization, and
semantic round-trip for required OWL 2 constructs.

## Required File Formats

Production-quality support for:

-   Turtle
-   RDF/XML
-   OWL/XML
-   OBO

Other formats may be supported but are not required for parity.

## Reasoning

Equivalent user workflows for:

-   Classification
-   Consistency checking
-   Unsatisfiable classes
-   Explanations
-   Inferred hierarchies

## Querying

-   SPARQL
-   DL Query or equivalent semantic querying
-   Entity search
-   Usage search

## SWRL

Support for:

-   Rule authoring
-   Editing
-   Validation
-   Serialization
-   Search

## Refactoring

-   Rename
-   Merge
-   Replace references
-   Module extraction
-   Namespace migration
-   Ontology merge where supported

## Visualization

-   Hierarchy browsers
-   Relationship graphs
-   Import graphs
-   Inferred vs asserted views

## Plugin Platform

A stable, documented SDK supporting:

-   Commands
-   Views
-   Validators
-   Import/export providers
-   Reasoner integrations

------------------------------------------------------------------------

# Explicitly Out of Scope

The following are not required for parity:

-   Binary compatibility with Java Protégé plugins
-   Swing UI replication
-   JVM execution
-   Cloud-hosted collaboration
-   AI-assisted ontology authoring
-   Marketplace infrastructure
-   Experimental research features

These may be implemented after v0.30.

------------------------------------------------------------------------

# Success Criteria

Parity is achieved only when:

-   Every P0 requirement in `PARITY_MATRIX.md` is COMPLETE.
-   Release gates are satisfied.
-   Required formats pass semantic round-trip testing.
-   Acceptance criteria are met.
-   Automated conformance tests pass.
-   Remaining limitations are documented.

------------------------------------------------------------------------

# Scope Governance

Scope changes require:

1.  Documentation update.
2.  Parity matrix update.
3.  Gap analysis review.
4.  Roadmap review.
5.  Acceptance criteria review.

------------------------------------------------------------------------

# Related Documents

-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   PARITY_MATRIX.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_RELEASE_GATE.md
