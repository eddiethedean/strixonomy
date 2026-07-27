# TESTING

# Strixonomy Testing Strategy

**Status:** Normative Engineering Specification\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document defines the testing strategy required to achieve and
maintain Protégé Desktop parity for Strixonomy.

Testing is organized around objective verification of semantic
correctness, user workflows, performance, and platform stability.

------------------------------------------------------------------------

# Testing Principles

-   Test semantics, not implementation details
-   Automate everything practical
-   Every bug becomes a regression test
-   Cross-platform verification
-   Deterministic test fixtures
-   Trace every test to a parity requirement

------------------------------------------------------------------------

# Test Pyramid

``` text
        End-to-End
      Integration Tests
        Component Tests
          Unit Tests
```

------------------------------------------------------------------------

# Test Categories

## Unit Tests

Cover:

-   Parsers
-   Serializers
-   Semantic transactions
-   Validators
-   Reasoning primitives
-   Query engine
-   Refactoring engine

## Integration Tests

Cover:

-   Workspace interactions
-   Multi-ontology workflows
-   Reasoning integration
-   Query integration
-   Plugin lifecycle
-   Import/export

## End-to-End Tests

Cover complete workflows:

-   Create ontology
-   Edit OWL entities
-   Save and reload
-   Run reasoner
-   Execute queries
-   Perform refactoring
-   Visualize ontology
-   Install plugins

## Conformance Tests

Verify:

-   OWL 2 support
-   File formats
-   Semantic round-trip
-   Protégé parity
-   Accessibility
-   Performance targets

## Regression Tests

Every fixed defect must include:

-   Reproduction fixture
-   Automated test
-   Linked GitHub issue
-   Linked parity requirement

------------------------------------------------------------------------

# Test Data

Maintain curated fixtures for:

-   Small ontologies
-   Large ontologies
-   OBO Foundry samples
-   Protégé-generated ontologies
-   Complex OWL 2 expressions
-   SWRL rules

------------------------------------------------------------------------

# Continuous Integration

Every pull request should execute:

-   Formatting
-   Linting
-   Unit tests
-   Integration tests
-   End-to-end tests
-   Conformance suites
-   Accessibility checks
-   Performance smoke tests

------------------------------------------------------------------------

# Quality Gates

A change may merge only when:

-   Tests pass
-   Coverage does not regress
-   No P0 failures exist
-   Parity metrics remain green

------------------------------------------------------------------------

# Success Metrics

-   High unit and integration coverage
-   100% P0 requirements linked to tests
-   Zero known regression failures
-   Stable cross-platform CI

------------------------------------------------------------------------

# Related Documents

-   PARITY_TEST_PLAN.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_RELEASE_GATE.md
-   IMPLEMENTATION_EVIDENCE.md
-   IMPLEMENTATION_PLAN.md
