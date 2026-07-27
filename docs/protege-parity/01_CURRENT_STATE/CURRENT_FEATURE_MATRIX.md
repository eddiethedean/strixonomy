# CURRENT_FEATURE_MATRIX

# Current Repository Feature Matrix

**Status:** Living Document\
**Repository Baseline:** Strixonomy v0.18.2 (audit baseline)\
**Purpose:** Summarize the current implementation status of major
features before additional parity work begins.

------------------------------------------------------------------------

# Overview

This matrix is a snapshot of the repository as it exists today. It is
**not** a roadmap. It records implemented capabilities, partial
implementations, and known gaps identified during the repository audit.

This document should be updated after every significant release.

------------------------------------------------------------------------

# Status Legend

  Status            Meaning
  ----------------- ----------------------------------------------------
  IMPLEMENTED       Production implementation exists.
  PARTIAL           Major functionality exists but parity gaps remain.
  PLANNED           Design exists but implementation is incomplete.
  NOT IMPLEMENTED   No meaningful implementation exists.

------------------------------------------------------------------------

# Feature Matrix

  ---------------------------------------------------------------------------------------
  Category        Capability          Status         Confidence     Notes
  --------------- ------------------- -------------- -------------- ---------------------
  Repository      Rust workspace      IMPLEMENTED    High           \~20 crates with
                                                                    modular architecture.

  Extension       VS Code/Cursor      IMPLEMENTED    High           Mature extension with
                  extension                                         dozens of commands.

  UI              React webviews      IMPLEMENTED    High           Inspector, graphs,
                                                                    imports, reasoning,
                                                                    query, diff.

  LSP             Language Server     IMPLEMENTED    High           Semantic diagnostics
                                                                    and navigation.

  Ontology        Create/Open/Save    PARTIAL        High           Strong workflow;
  Lifecycle                                                         multi-format
                                                                    persistence
                                                                    incomplete.

  Workspace       Multi-ontology      PARTIAL        Medium         WorkspaceStore
                  support                                           present; session
                                                                    semantics incomplete.

  Turtle          Read/Write          IMPLEMENTED    High           Primary authoring
                                                                    workflow.

  OBO             Read/Write          IMPLEMENTED    High           Dedicated patch
                                                                    pipeline.

  OWL/XML         Read                IMPLEMENTED    High           Index/browse
                                                                    supported.

  OWL/XML         Write               NOT            High           Major parity blocker.
                                      IMPLEMENTED                   

  RDF/XML         Read                IMPLEMENTED    High           Index/browse
                                                                    supported.

  RDF/XML         Write               NOT            High           Major parity blocker.
                                      IMPLEMENTED                   

  OWL 2 Authoring General support     PARTIAL        Medium         Broad coverage; not
                                                                    complete structural
                                                                    specification.

  Reasoning       Classification      IMPLEMENTED    High           Multiple
                                                                    profiles/adapters.

  Reasoning       ABox reasoning      PARTIAL        Medium         Full parity not yet
                                                                    achieved.

  Explanations    Unsat explanations  PARTIAL        Medium         Native DL
                                                                    explanations
                                                                    incomplete.

  Query           SPARQL              IMPLEMENTED    High           Query workbench
                                                                    present.

  Query           DL Query            PARTIAL        Medium         Equivalent workflow
                                                                    requires expansion.

  SWRL            Rule authoring      NOT            High           Planned parity
                                      IMPLEMENTED                   blocker.

  Refactoring     Semantic            PARTIAL        High           Rename/merge/module
                  refactoring                                       extraction;
                                                                    Turtle-focused.

  Visualization   Graphs              PARTIAL        Medium         Strong foundation;
                                                                    OntoGraf parity
                                                                    incomplete.

  Plugin Platform SDK                 PARTIAL        High           Functional but
                                                                    pre-stable.

  Accessibility   UI accessibility    PARTIAL        Medium         Good foundation;
                                                                    formal audit pending.

  Testing         Automated tests     IMPLEMENTED    High           Extensive suite;
                                                                    parity corpus
                                                                    incomplete.

  Documentation   Architecture/docs   IMPLEMENTED    High           Comprehensive
                                                                    documentation base.
  ---------------------------------------------------------------------------------------

------------------------------------------------------------------------

# Highest-Priority Gaps

The audit identified the following P0 engineering blockers:

1.  RDF/XML write-back
2.  OWL/XML write-back
3.  Complete OWL 2 authoring
4.  Workspace semantics
5.  Full reasoning parity
6.  SWRL support
7.  Executable parity verification

------------------------------------------------------------------------

# Usage

This matrix is intended to answer:

-   What is already implemented?
-   What remains incomplete?
-   Where should engineering effort be focused?

For detailed implementation evidence, see `IMPLEMENTATION_EVIDENCE.md`.

For the complete audit, see `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md`.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_SCOPE.md
-   PARITY_MATRIX.md
-   PARITY_GAP_ANALYSIS.md
