# PROTEGE_VIEW_AUDIT

# Protégé Desktop View Audit

**Status:** Living Reference Document\
**Purpose:** Audit the primary views available in a standard Protégé
Desktop installation and map them to equivalent Strixonomy workflows.

> This audit evaluates functional workflows rather than visual
> similarity. Strixonomy is expected to provide equivalent capabilities
> through modern IDE-native interfaces.

------------------------------------------------------------------------

# Goals

-   Inventory every major Protégé view.
-   Identify the equivalent Strixonomy panel or workflow.
-   Record current parity status.
-   Drive UI implementation and end-to-end testing.

------------------------------------------------------------------------

# Audit Status

  Status     Meaning
  ---------- ------------------------------------------------------------
  COMPLETE   Functionally equivalent workflow exists.
  PARTIAL    Significant functionality exists but gaps remain.
  MISSING    No equivalent workflow currently exists.
  REVIEW     Requires validation against the selected Protégé baseline.

------------------------------------------------------------------------

# View Inventory

  --------------------------------------------------------------------------------
  Protégé View   Purpose          Expected Strixonomy  Audit Status   Notes
                                  Equivalent                        
  -------------- ---------------- ------------------ -------------- --------------
  Active         Ontology         Ontology           REVIEW         
  Ontology       metadata         inspector/editor                  

  Classes        Class hierarchy  Class hierarchy    REVIEW         
                 & editing        panel                             

  Object         Property         Property editor    REVIEW         
  Properties     hierarchy                                          

  Data           Property         Property editor    REVIEW         
  Properties     hierarchy                                          

  Annotation     Annotation       Annotation panel   REVIEW         
  Properties     editing                                            

  Individuals    Instance         Individuals        REVIEW         
                 management       browser                           

  Datatypes      Datatype         Datatype editor    REVIEW         
                 authoring                                          

  Prefixes       Prefix           Prefix manager     REVIEW         
                 management                                         

  Imports        Imports closure  Imports panel      REVIEW         

  Annotations    Ontology         Annotation editor  REVIEW         
                 annotations                                        

  DL Query       Description      Query workbench    REVIEW         
                 logic queries                                      

  SPARQL         SPARQL queries   Query workbench    REVIEW         

  Reasoner       Classification & Reasoning panel    REVIEW         
                 consistency                                        

  Explanations   Why is this      Explanation panel  REVIEW         
                 inferred?                                          

  OntoGraf       Graph            Graph view         REVIEW         
                 visualization                                      

  Usage          Entity           Usage search       REVIEW         
                 references                                         

  Search         Locate entities  Global/entity      REVIEW         
                                  search                            

  SWRL           Rule authoring   SWRL editor        REVIEW         Major parity
                                                                    blocker.
  --------------------------------------------------------------------------------

------------------------------------------------------------------------

# View Requirements

Every production view should support:

-   Keyboard navigation
-   Context menus
-   Deep-linking from diagnostics
-   Undo/redo integration
-   Consistent selection synchronization
-   Accessibility support
-   Persistence where appropriate

------------------------------------------------------------------------

# View Mapping

For each view, maintain links to:

-   Responsible UI component
-   Extension command(s)
-   Language Server support
-   Workspace integration
-   Automated UI tests
-   Acceptance criteria

------------------------------------------------------------------------

# Acceptable Differences

Functional parity does **not** require:

-   Swing docking behavior
-   Identical layouts
-   Identical widget hierarchy
-   Matching menu locations

Equivalent IDE-native workflows are acceptable when they provide the
same user outcome.

------------------------------------------------------------------------

# Remaining High-Priority Views

The repository audit indicates additional work is likely required for:

-   Complete SWRL view
-   Full DL Query experience
-   Workspace restoration
-   Advanced graph interactions
-   Comprehensive accessibility validation

------------------------------------------------------------------------

# Maintenance

Whenever a new view is added or substantially changed:

1.  Update this audit.
2.  Update the parity matrix.
3.  Link implementation evidence.
4.  Add end-to-end UI tests.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PROTEGE_FEATURE_INVENTORY.md
-   PROTEGE_MENU_AUDIT.md
-   PROTEGE_WORKFLOW_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
