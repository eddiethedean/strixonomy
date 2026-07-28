# PARITY_MATRIX

# Protégé Desktop Parity Matrix

**Status:** Living Specification\
**Target Release:** Strixonomy v0.30.0

------------------------------------------------------------------------

# Purpose

The parity matrix is the canonical source of truth for tracking
Strixonomy's progress toward full Protégé Desktop parity.

Unlike the feature inventory, which lists Protégé capabilities, this
document maps each requirement to its implementation status, evidence,
testing, and acceptance criteria.

Every parity claim must be traceable to this matrix.

------------------------------------------------------------------------

# Status Values

  -----------------------------------------------------------------------
  Status                              Meaning
  ----------------------------------- -----------------------------------
  COMPLETE                            Fully implemented and verified.

  PARTIAL                             Significant implementation exists
                                      but parity is incomplete.

  NOT_IMPLEMENTED                     No meaningful implementation
                                      exists.

  BLOCKED                             Waiting on prerequisite
                                      architectural work.

  VERIFIED                            COMPLETE and all acceptance tests
                                      have passed.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Priority Levels

  Priority   Description
  ---------- -------------------------------------
  P0         Required before v0.30 release.
  P1         Important but not release-blocking.
  P2         v0.31+ enhancement.

------------------------------------------------------------------------

# Matrix

  ----------------------------------------------------------------------------------------------------------------------------------------------
  ID             Area            Requirement         Priority   Current Status    Evidence                   Tests              Notes
  -------------- --------------- ------------------- ---------- ----------------- -------------------------- ------------------ ----------------
  PAR-LIFE-001   Lifecycle       Create/Open/Save on P0         PARTIAL           IMPLEMENTATION_EVIDENCE    dialogs/workflows  Multi-format save incomplete
  PAR-LIFE-002   Lifecycle       Multi-ontology work P0         COMPLETE          IMPLEMENTATION_EVIDENCE    workspace.runtime  v0.20 workspace runtime
  PAR-FMT-001    Formats         Turtle semantic rou P0         COMPLETE          IMPLEMENTATION_EVIDENCE    round-trip tests   Primary Turtle workflow
  PAR-FMT-002    Formats         OBO semantic round- P0         PARTIAL           IMPLEMENTATION_EVIDENCE    round-trip tests   Expand OBO corpus
  PAR-FMT-003    Formats         RDF/XML write-back  P0         COMPLETE          ADR-0021                   xml_writeback      Closed v0.21
  PAR-FMT-004    Formats         OWL/XML write-back  P0         COMPLETE          ADR-0021                   xml_writeback      Closed v0.21
  PAR-OWL-001    OWL2            Complete OWL2 autho P0         COMPLETE          OWL2_AUTHORING             owl2_authoring     Shipped v0.22
  PAR-WS-001     Workspace       Persistent workspac P0         COMPLETE          BLOCKER_03                 workspace tests    v0.20 workspace runtime
  PAR-RSN-001    Reasoning       Classification      P0         COMPLETE          IMPLEMENTATION_EVIDENCE    reasoner_el        Classification
  PAR-RSN-002    Reasoning       Full ABox reasoning P0         VERIFIED          BLOCKER_04                 reasoner_abox      Shipped v0.23
  PAR-RSN-003    Reasoning       Native DL explanati P0         VERIFIED          BLOCKER_04                 explain tests      Shipped v0.23
  PAR-QRY-001    Query           SPARQL              P0         VERIFIED          IMPLEMENTATION_EVIDENCE    query tests        SPARQL
  PAR-QRY-002    Query           DL Query workflow   P1         VERIFIED          BLOCKER_07                 dl_query + UI      Shipped v0.24
  PAR-SWRL-001   SWRL            Rule authoring/edit P0         VERIFIED          BLOCKER_05                 swrl tests         Shipped v0.23
  PAR-REF-001    Refactoring     Semantic refactorin P0         VERIFIED          IMPLEMENTATION_EVIDENCE    refactor tests     Multi-format rename/merge/replace
  PAR-VIS-001    Visualization   Graph parity        P1         VERIFIED          BLOCKER_08                 GraphPanel tests   Shipped v0.25 EPIC-008
  PAR-PLG-001    Plugins         Stable SDK          P1         VERIFIED          BLOCKER_09                 plugin_sdk_compat  Shipped v0.25 EPIC-009
  PAR-ACC-001    Accessibility   Keyboard and screen P1         VERIFIED          ACCESSIBILITY_REPORT       a11y Vitest        Shipped v0.25 EPIC-010
  PAR-TST-001    Verification    Executable parity c P0         VERIFIED          BLOCKER_11                 parity CI scripts  Shipped v0.25 EPIC-011
  ----------------------------------------------------------------------------------------------------------------------------------------------

# Traceability

Every row should ultimately link to:

-   Source implementation
-   Responsible crate(s)
-   GitHub issue
-   Acceptance criteria
-   Automated tests
-   Documentation

------------------------------------------------------------------------

# Release Rule

Strixonomy v0.30 may claim Protégé Desktop parity only when:

-   Every P0 requirement is VERIFIED.
-   Release gates are satisfied.
-   Remaining P1/P2 items are documented and accepted.

------------------------------------------------------------------------

# Maintenance

Whenever implementation changes:

1.  Update this matrix.
2.  Update IMPLEMENTATION_EVIDENCE.md.
3.  Update PARITY_GAP_ANALYSIS.md.
4.  Update automated tests.
5.  Re-run parity validation.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PARITY_SCOPE.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
