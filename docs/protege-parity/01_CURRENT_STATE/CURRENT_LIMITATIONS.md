# CURRENT_LIMITATIONS

# Current Repository Limitations

**Status:** Living Document\
**Repository Baseline:** Strixonomy v0.18.2 (audit baseline)

------------------------------------------------------------------------

# Purpose

This document records the significant limitations identified during the
repository audit. It is intentionally descriptive rather than
aspirational: each limitation reflects the current implementation, not
planned functionality.

The purpose is to provide a realistic assessment of where Strixonomy still
differs from Protégé Desktop and to guide engineering priorities for the
v0.30 release.

------------------------------------------------------------------------

# Classification

Limitations are grouped into three categories:

-   **P0** -- Blocks a credible claim of Protégé Desktop parity.
-   **P1** -- Important but does not independently block the v0.30
    release.
-   **P2** -- Quality improvements or v0.31+ enhancements.

------------------------------------------------------------------------

# P0 Limitations

## RDF/XML Write-Back

**Status:** Not implemented

Current support allows parsing and inspection, but production-quality
editing and serialization are not available.

Impact:

-   Cannot edit many existing Protégé projects in place.
-   Prevents full semantic round-trip support.

------------------------------------------------------------------------

## OWL/XML Write-Back

**Status:** Not implemented

Current support is limited to reading and indexing.

Impact:

-   Major compatibility gap.
-   Blocks complete format parity.

------------------------------------------------------------------------

## Complete OWL 2 Authoring

Current implementation provides broad authoring support but does not yet
cover the complete OWL 2 Structural Specification.

Examples include:

-   Remaining axiom types
-   Complete datatype workflows
-   Full annotation workflows
-   Some advanced assertion types

------------------------------------------------------------------------

## Workspace Semantics

Current WorkspaceStore is a strong foundation but does not yet provide
complete parity for:

-   Session restoration
-   Live panel restoration
-   Per-ontology persistence semantics
-   Advanced multi-ontology workflows

------------------------------------------------------------------------

## Full Reasoning Parity

Reasoning currently emphasizes classification and class-level
consistency.

Remaining work includes:

-   ABox reasoning
-   Realization
-   Complete instance checking
-   Native DL explanations

------------------------------------------------------------------------

## SWRL

A production SWRL subsystem is not currently present.

Required capabilities include:

-   Rule authoring
-   Validation
-   Serialization
-   Search
-   Reasoner integration

------------------------------------------------------------------------

## Executable Parity Verification

Current testing is extensive but does not yet provide a complete Protégé
conformance suite.

Required additions include:

-   Semantic round-trip corpus
-   Automated parity validation
-   Machine-readable parity manifest

------------------------------------------------------------------------

# P1 Limitations

-   Complete OntoGraf-equivalent visualization
-   Stable public Plugin SDK
-   Expanded accessibility verification
-   Advanced ontology merge workflows
-   Locality-based module extraction
-   Complete DL Query workflow

------------------------------------------------------------------------

# P2 Limitations

Potential v0.31+ improvements include:

-   Cloud collaboration
-   AI-assisted ontology authoring
-   Marketplace infrastructure
-   Additional visualization modes
-   Advanced analytics

------------------------------------------------------------------------

# Design Philosophy

These limitations should be addressed by extending the existing
architecture rather than replacing it.

Priority should be given to:

1.  Reusing the current semantic infrastructure.
2.  Avoiding duplicate implementations.
3.  Preserving existing workflows.
4.  Expanding automated testing as features mature.

------------------------------------------------------------------------

# Relationship to Other Documents

-   `CURRENT_FEATURE_MATRIX.md` describes what exists.
-   This document describes what remains incomplete.
-   `PARITY_GAP_ANALYSIS.md` explains why these gaps matter.
-   `IMPLEMENTATION_EVIDENCE.md` records implementation progress.
-   `PARITY_RELEASE_GATE.md` determines when these limitations no longer
    block the v0.30 release.

------------------------------------------------------------------------

# Maintenance

This document should be updated whenever:

-   A limitation is resolved.
-   A new release-blocking issue is discovered.
-   Scope changes affect parity requirements.

Resolved limitations should be moved to release notes or historical
documentation rather than deleted, preserving an audit trail of project
progress.
