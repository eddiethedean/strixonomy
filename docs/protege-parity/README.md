# Protégé Parity Program

> **NOT PRODUCT CLAIMS — engineering notes only.** This tree is **excluded from the public Read the Docs site**. Do **not** use it to evaluate what Strixonomy ships.
>
> **For adoption decisions use only:** [What ships today](../SHIPPED.md) and [Known limitations](../known-limitations.md).

This directory contains the engineering specifications, audits, and
implementation plans that drive **Strixonomy v0.30.0** toward **full
functional parity with Protégé Desktop**.

Unlike earlier planning documents, this documentation set is grounded in
an audit of the current repository. Its purpose is not to imagine a
future architecture, but to identify the remaining engineering work
required to replace Protégé for everyday ontology engineering.

------------------------------------------------------------------------

# Goals

The parity program has four primary goals:

1.  Define the exact scope of Protégé parity.
2.  Measure Strixonomy's current capabilities against that scope.
3.  Plan and prioritize the remaining implementation work.
4.  Provide objective evidence that Strixonomy v0.30 satisfies its parity
    claims.

------------------------------------------------------------------------

# Documentation Structure

Paths below are relative to this directory (`docs/protege-parity/`).

## Foundation

-   `README.md` — This document.
-   `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md` — Evidence-based audit of the
    current repository.
-   `PARITY_SCOPE.md` — Defines what "Protégé parity" means.
-   `IMPLEMENTATION_EVIDENCE.md` — Maps parity claims to source code
    and tests (canonical registry — do not duplicate under `05_IMPLEMENTATION/`).

## Current State (`01_CURRENT_STATE/`)

-   `01_CURRENT_STATE/CURRENT_FEATURE_MATRIX.md`
-   `01_CURRENT_STATE/CURRENT_ARCHITECTURE.md`
-   `01_CURRENT_STATE/CURRENT_LIMITATIONS.md`
-   `01_CURRENT_STATE/SHIPPED_CAPABILITIES.md`

## Protégé Audit (`02_PROTEGE_AUDIT/`)

-   `02_PROTEGE_AUDIT/PROTEGE_FEATURE_INVENTORY.md`
-   `02_PROTEGE_AUDIT/PROTEGE_MENU_AUDIT.md`
-   `02_PROTEGE_AUDIT/PROTEGE_VIEW_AUDIT.md`
-   `02_PROTEGE_AUDIT/PROTEGE_WORKFLOW_AUDIT.md`
-   `02_PROTEGE_AUDIT/PROTEGE_PLUGIN_AUDIT.md`
-   `02_PROTEGE_AUDIT/PROTEGE_REASONER_AUDIT.md`
-   `02_PROTEGE_AUDIT/PROTEGE_FILE_FORMAT_AUDIT.md`
-   `02_PROTEGE_AUDIT/PROTEGE_UI_AUDIT.md`

## Parity Tracking (`03_PARITY/`)

-   `03_PARITY/PARITY_MATRIX.md`
-   `03_PARITY/PARITY_GAP_ANALYSIS.md`
-   `03_PARITY/PARITY_STATUS.md`
-   `03_PARITY/PARITY_ACCEPTANCE_CRITERIA.md`
-   `03_PARITY/PARITY_TEST_PLAN.md`
-   `03_PARITY/PROTEGE_TEST_PORT.md` — Protégé Desktop JUnit → Strixonomy oracle port (v0.26)
-   `03_PARITY/PARITY_RELEASE_GATE.md`

## Engineering Blockers (`04_BLOCKERS/`)

The implementation roadmap is organized around blockers rather than
subsystems:

1.  Format-independent editing
2.  Complete OWL 2 authoring
3.  Workspace semantics
4.  Reasoning parity
5.  SWRL support
6.  Advanced ontology operations
7.  Parity verification

## Implementation Planning (`05_IMPLEMENTATION/`)

-   `05_IMPLEMENTATION/IMPLEMENTATION_PLAN.md`
-   `05_IMPLEMENTATION/EXECUTION_ORDER.md`
-   `05_IMPLEMENTATION/DEPENDENCY_GRAPH.md`
-   `05_IMPLEMENTATION/P0_IMPLEMENTATION_PLAN.md`
-   `05_IMPLEMENTATION/P1_IMPLEMENTATION_PLAN.md`
-   `05_IMPLEMENTATION/RISK_REGISTER.md`

## Subsystem Specifications (`06_SUBSYSTEMS/`)

Detailed implementation specifications for authoring, formats,
workspace, reasoning, SWRL, refactoring, querying, visualization,
plugins, UI workflows, and testing.

------------------------------------------------------------------------

# Recommended Reading Order

1.  Read [V0_30_PHASES.md](07_BACKLOG/V0_30_PHASES.md) for the versioned release plan (v0.19–v0.30).
2.  Read `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md` or `01_CURRENT_STATE/` audit docs.
3.  Read `PARITY_SCOPE.md`.
4.  Review `03_PARITY/PARITY_MATRIX.md` and `03_PARITY/PARITY_GAP_ANALYSIS.md`.
5.  Read the blocker documents in `04_BLOCKERS/` in order.
6.  Execute work from [EXECUTION_ORDER.md](05_IMPLEMENTATION/EXECUTION_ORDER.md) and backlog.
7.  Validate progress using the acceptance criteria and test plan.
8.  Ship only after every release gate has been satisfied.

------------------------------------------------------------------------

# Guiding Principles

-   Functional parity over visual similarity.
-   Semantic correctness over serialization details.
-   Native Rust implementation (no JVM dependency).
-   Evidence-backed engineering decisions.
-   Automated testing before parity claims.
-   Modern UX rather than reproducing Swing.

------------------------------------------------------------------------

# Definition of Success

Strixonomy v0.30.0 may be described as a Protégé Desktop replacement only
when:

-   Every P0 parity requirement is complete.
-   All required ontology formats support semantic round-trip.
-   Complete OWL 2 authoring workflows are implemented.
-   Workspace, reasoning, and refactoring parity are achieved.
-   Required automated tests pass.
-   Release gates are satisfied.

------------------------------------------------------------------------

# Maintaining This Directory

Whenever implementation changes:

1.  Update [IMPLEMENTATION_EVIDENCE.md](IMPLEMENTATION_EVIDENCE.md).
2.  Update the parity matrix.
3.  Reassess the gap analysis.
4.  Update the roadmap and backlog.
5.  Expand automated test coverage.
6.  Record any scope changes.

These documents should remain the authoritative engineering source of
truth for the Protégé parity effort.
