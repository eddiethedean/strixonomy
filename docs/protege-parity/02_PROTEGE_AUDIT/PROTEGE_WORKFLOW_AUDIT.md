# PROTEGE_WORKFLOW_AUDIT

# Protégé Desktop Workflow Audit

**Status:** Living Reference Document\
**Purpose:** Define the end-to-end ontology engineering workflows that
Strixonomy must support to achieve functional parity with a standard
Protégé Desktop installation.

> This document audits complete user workflows rather than individual
> features or UI components.

------------------------------------------------------------------------

# Purpose

Users evaluate ontology editors by whether they can accomplish real
tasks efficiently. This audit captures those tasks and maps them to
Strixonomy's implementation and parity requirements.

Workflow parity---not visual similarity---is the objective.

------------------------------------------------------------------------

# Workflow Status Legend

  -----------------------------------------------------------------------
  Status                              Meaning
  ----------------------------------- -----------------------------------
  COMPLETE                            End-to-end workflow is functionally
                                      equivalent.

  PARTIAL                             Workflow largely exists but
                                      important steps are missing.

  MISSING                             No equivalent workflow currently
                                      exists.

  REVIEW                              Requires verification against the
                                      selected Protégé baseline.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Core Workflows

  ----------------------------------------------------------------------------------
  Workflow       Typical User Goal      Expected       Status         Priority
                                        Strixonomy                      
                                        Equivalent                    
  -------------- ---------------------- -------------- -------------- --------------
  Create         Start a new ontology   New ontology   REVIEW         P0
  ontology                              workflow                      

  Open existing  Continue work          Open/import    REVIEW         P0
  ontology                              workflow                      

  Save changes   Persist ontology       Save / Save    REVIEW         P0
                                        All                           

  Import         Reuse existing         Imports        REVIEW         P0
  ontology       vocabulary             manager                       

  Manage         Maintain namespaces    Prefix manager REVIEW         P0
  prefixes                                                            

  Browse class   Explore ontology       Class          REVIEW         P0
  hierarchy                             hierarchy                     
                                        panel                         

  Create and     Model ontology         Entity editors REVIEW         P0
  edit entities                                                       

  Create OWL     Define semantics       Structured     REVIEW         P0
  axioms                                authoring                     

  Run reasoner   Infer knowledge        Reasoning      REVIEW         P0
                                        panel                         

  Explain        Understand results     Explanation    REVIEW         P0
  inference                             panel                         

  Execute DL     Semantic queries       Query          REVIEW         P1
  Query                                 workbench                     

  Execute SPARQL Graph queries          Query          REVIEW         P1
                                        workbench                     

  Refactor       Rename/merge/migrate   Refactoring    REVIEW         P0
  ontology                              tools                         

  Visualize      Inspect relationships  Graph view     REVIEW         P1
  ontology                                                            

  Author SWRL    Rule-based reasoning   SWRL editor    REVIEW         P0
  rules                                                               

  Restore        Resume work            Workspace      REVIEW         P0
  previous                              restoration                   
  session                                                             
  ----------------------------------------------------------------------------------

------------------------------------------------------------------------

# Workflow Requirements

Every production workflow should:

-   Be discoverable through the UI or command palette.
-   Preserve semantic correctness.
-   Support undo/redo where appropriate.
-   Produce actionable diagnostics on failure.
-   Integrate with workspace state.
-   Be accessible using keyboard navigation.

------------------------------------------------------------------------

# Workflow Mapping

Each workflow should ultimately reference:

-   Parity requirement ID
-   Strixonomy command(s)
-   Responsible UI components
-   Rust crates/services
-   Automated tests
-   Acceptance criteria
-   Implementation evidence

------------------------------------------------------------------------

# High-Priority Gaps

Based on the current repository audit, the largest workflow gaps are:

1.  RDF/XML editing workflow
2.  OWL/XML editing workflow
3.  Complete OWL 2 authoring workflow
4.  Workspace restoration workflow
5.  Full reasoning workflow (including ABox)
6.  SWRL authoring workflow
7.  End-to-end parity verification workflow

------------------------------------------------------------------------

# Verification Strategy

Every workflow should be validated using:

-   Manual walkthroughs
-   Automated UI tests
-   Regression tests
-   Semantic round-trip tests (where applicable)
-   Cross-platform verification

------------------------------------------------------------------------

# Acceptance Criteria

A workflow may be marked COMPLETE only when:

1.  A user can accomplish the task without unsupported manual steps.
2.  Semantic correctness is preserved.
3.  Required tests pass.
4.  Documentation exists.
5.  Remaining limitations are documented.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PROTEGE_FEATURE_INVENTORY.md
-   PROTEGE_MENU_AUDIT.md
-   PROTEGE_VIEW_AUDIT.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
