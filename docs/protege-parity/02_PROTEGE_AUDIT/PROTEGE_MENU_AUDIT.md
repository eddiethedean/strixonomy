# PROTEGE_MENU_AUDIT

# Protégé Desktop Menu Audit

**Status:** Living Reference Document\
**Purpose:** Audit the major menus and commands available in a standard
Protégé Desktop installation and map them to Strixonomy parity
requirements.

> This document inventories user-facing functionality, not Swing
> implementation details. It should be refined against the exact Protégé
> release selected as the parity baseline.

------------------------------------------------------------------------

# Goals

-   Enumerate major menu-driven workflows.
-   Ensure every meaningful workflow is represented in
    `PARITY_MATRIX.md`.
-   Identify gaps between Protégé and Strixonomy.
-   Provide a checklist for UI parity verification.

------------------------------------------------------------------------

# Audit Status Legend

  Status    Meaning
  --------- -----------------------------------------------------
  AUDITED   Workflow reviewed and mapped.
  PENDING   Requires verification against the baseline release.
  N/A       Not part of the Strixonomy parity scope.

------------------------------------------------------------------------

# File Menu

  Menu Item         Parity Required Audit     Notes
  --------------- ----------------- --------- -------------------------
  New Ontology                    ✓ PENDING   
  Open Ontology                   ✓ PENDING   
  Open Recent                     ✓ PENDING   
  Save                            ✓ PENDING   
  Save As                         ✓ PENDING   
  Save All                        ✓ PENDING   
  Export                          ✓ PENDING   
  Close                           ✓ PENDING   
  Preferences                     ✓ PENDING   
  Exit                          N/A AUDITED   IDE-managed in VS Code.

------------------------------------------------------------------------

# Edit Menu

-   Undo / Redo
-   Cut / Copy / Paste (where applicable)
-   Delete
-   Rename
-   Find
-   Select All (where appropriate)

All editing actions should integrate with semantic transactions rather
than text editing.

------------------------------------------------------------------------

# Entity & Ontology Workflows

Audit the availability of:

-   Create class
-   Create object property
-   Create data property
-   Create annotation property
-   Create individual
-   Create datatype
-   Ontology metadata editing
-   Prefix management
-   Import management

------------------------------------------------------------------------

# Reasoner Menu

Required workflows:

-   Select reasoner
-   Start classification
-   Stop reasoner (if supported)
-   Synchronize
-   Check consistency
-   View inferred hierarchy
-   View explanations

------------------------------------------------------------------------

# Query Menu

Required workflows:

-   DL Query
-   SPARQL
-   Entity search
-   Usage search

------------------------------------------------------------------------

# Refactor Menu

Required workflows:

-   Rename
-   Merge
-   Replace references
-   Module extraction
-   Namespace migration
-   Ontology merge
-   Usage analysis

------------------------------------------------------------------------

# Window / View Menu

Expected view categories include:

-   Class hierarchy
-   Property hierarchy
-   Individuals
-   Imports
-   Query
-   Reasoner
-   Graphs
-   Explanations
-   Plugin views

------------------------------------------------------------------------

# Tools Menu

Review workflows for:

-   Validation
-   Diagnostics
-   Plugins
-   Settings
-   Utility operations

------------------------------------------------------------------------

# Help Menu

Confirm availability of:

-   Documentation
-   Keyboard shortcuts
-   Version information
-   Diagnostic information

------------------------------------------------------------------------

# Deliverables

Each audited menu item should ultimately map to:

-   A parity requirement ID
-   An Strixonomy implementation
-   Automated UI tests
-   Acceptance criteria

------------------------------------------------------------------------

# Known Differences

The following differences are acceptable if functional parity is
preserved:

-   VS Code commands replacing Swing menus
-   Command palette replacing menu nesting
-   Modern dialogs replacing legacy Swing dialogs
-   IDE-native preferences instead of standalone settings windows

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PROTEGE_FEATURE_INVENTORY.md
-   PROTEGE_WORKFLOW_AUDIT.md
-   UI_WORKFLOWS.md
-   PARITY_MATRIX.md
-   PARITY_RELEASE_GATE.md
