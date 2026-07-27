# PARITY_RELEASE_GATE

# Protégé Desktop Parity Release Gate

**Status:** Normative Release Specification\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document defines the objective release criteria that must be
satisfied before Strixonomy 1.0.0 may be described as a functional
replacement for Protégé Desktop.

A release gate is passed only through demonstrable evidence, not
implementation intent.

------------------------------------------------------------------------

# Release Principles

-   Evidence over assertions
-   Functional parity over visual parity
-   Semantic correctness over serializer details
-   Automated verification wherever practical
-   No unresolved P0 parity gaps

------------------------------------------------------------------------

# Mandatory Gates

## Gate 1 --- Scope

-   [ ] Parity scope frozen
-   [ ] Scope changes documented
-   [ ] Scope approved

## Gate 2 --- Repository Audit

-   [ ] Current repository audit completed
-   [ ] Current feature matrix updated
-   [ ] Implementation evidence synchronized

## Gate 3 --- P0 Requirements

-   [ ] Every P0 requirement is VERIFIED in PARITY_MATRIX.md
-   [ ] No unresolved P0 implementation issues

Automation (v0.25 / EPIC-011):

-   `python3 scripts/check-parity-release-gate.py` prints Gate 3 readiness from `parity/protege-desktop-parity.yaml`
-   CI enforces **infrastructure** (paths + evidence completeness + docs sync), not full Gate 3
-   `python3 scripts/check-parity-release-gate.py --strict-release` fails when Gate 3 is incomplete (for 1.0.0-rc)

## Gate 4 --- File Formats

Required formats must support:

-   [ ] Parse
-   [ ] Semantic edit
-   [ ] Save
-   [ ] Semantic round-trip

Formats:

-   [ ] Turtle
-   [ ] RDF/XML
-   [ ] OWL/XML
-   [ ] OBO

## Gate 5 --- OWL 2 Authoring

-   [ ] Required OWL 2 constructs supported
-   [ ] Validation implemented
-   [ ] Undo/redo verified
-   [ ] Serialization verified

## Gate 6 --- Workspace

-   [ ] Multi-ontology workflows
-   [ ] Session restoration
-   [ ] Dirty-state handling
-   [ ] Active ontology management
-   [ ] Save/Save All workflows

## Gate 7 --- Reasoning

-   [ ] Classification
-   [ ] Consistency
-   [ ] ABox reasoning
-   [ ] Explanations
-   [ ] Inferred hierarchy

## Gate 8 --- SWRL

-   [ ] Rule authoring
-   [ ] Validation
-   [ ] Serialization
-   [ ] Search
-   [ ] Automated tests

## Gate 9 --- User Experience

-   [ ] Keyboard workflows
-   [ ] Accessibility verification
-   [ ] Cross-platform testing
-   [ ] End-to-end workflow validation

## Gate 10 --- Testing

-   [ ] Unit tests pass
-   [ ] Integration tests pass
-   [ ] End-to-end tests pass
-   [ ] Conformance suite passes
-   [ ] Regression suite passes
-   [ ] Performance targets met

## Gate 11 --- Documentation

-   [ ] User documentation complete
-   [ ] Developer documentation complete
-   [ ] Implementation evidence current
-   [ ] Gap analysis current
-   [ ] Release notes prepared

------------------------------------------------------------------------

# Exit Criteria

The release gate is satisfied only when:

1.  Every P0 requirement is VERIFIED.
2.  All mandatory gates are complete.
3.  Remaining P1/P2 gaps are documented and accepted.
4.  No known release-blocking defects remain.
5.  CI passes on supported platforms.

------------------------------------------------------------------------

# Required Evidence

Release approval should reference:

-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_TEST_PLAN.md
-   Final CI results
-   Performance benchmark report

------------------------------------------------------------------------

# Approval

  Role              Approval
  ----------------- ----------
  Technical Lead    ☐
  Maintainer        ☐
  QA                ☐
  Release Manager   ☐

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PARITY_SCOPE.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_GAP_ANALYSIS.md
