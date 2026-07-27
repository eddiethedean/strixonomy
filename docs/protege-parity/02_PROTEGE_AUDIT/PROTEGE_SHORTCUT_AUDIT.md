# PROTEGE_SHORTCUT_AUDIT

# Protégé Desktop Keyboard Shortcut Audit

**Status:** Living Reference Document\
**Purpose:** Audit keyboard shortcuts and keyboard-driven workflows in a
standard Protégé Desktop installation and define equivalent expectations
for Strixonomy.

> Functional keyboard accessibility is the goal. Strixonomy is not
> required to duplicate Protégé's exact key bindings, especially when
> running inside VS Code/Cursor where IDE shortcuts take precedence.

------------------------------------------------------------------------

# Goals

-   Identify important keyboard workflows.
-   Ensure every critical action is keyboard accessible.
-   Respect host IDE shortcut conventions.
-   Support accessibility and power users.

------------------------------------------------------------------------

# Audit Principles

-   Workflow parity over identical shortcuts.
-   Prefer native VS Code/Cursor conventions.
-   Every frequent action should be reachable without a mouse.
-   Shortcut conflicts should be documented.

------------------------------------------------------------------------

# Shortcut Categories

## File Operations

  -----------------------------------------------------------------------
  Workflow          Protégé Shortcut  Strixonomy          Status
                                      Equivalent        
  ----------------- ----------------- ----------------- -----------------
  New ontology      Platform          VS Code command / REVIEW
                    dependent         keybinding        

  Open ontology     Platform          VS Code command / REVIEW
                    dependent         keybinding        

  Save              Ctrl/Cmd+S        Native IDE save   REVIEW

  Save All          IDE dependent     Native IDE save   REVIEW
                                      all               
  -----------------------------------------------------------------------

## Editing

  Workflow        Requirement           Status
  --------------- --------------------- --------
  Undo            Keyboard accessible   REVIEW
  Redo            Keyboard accessible   REVIEW
  Delete entity   Keyboard accessible   REVIEW
  Rename entity   Keyboard accessible   REVIEW
  Find/Search     Keyboard accessible   REVIEW

## Navigation

Required keyboard workflows:

-   Move through class hierarchy
-   Expand/collapse trees
-   Switch panels
-   Back/forward navigation
-   Jump to entity
-   Open references/usages

## Reasoning

Required keyboard access:

-   Run reasoner
-   Reclassify ontology
-   Open explanations
-   Refresh inferred hierarchy

## Querying

-   Open query workbench
-   Execute query
-   Navigate results
-   Return focus to editor

## Refactoring

-   Rename
-   Merge
-   Preview changes
-   Apply/cancel refactoring

------------------------------------------------------------------------

# Accessibility Requirements

Keyboard workflows should support:

-   Full tab navigation
-   Logical focus order
-   Visible focus indicators
-   Screen reader compatibility
-   No mouse-only critical workflows

------------------------------------------------------------------------

# Known Differences

Acceptable differences include:

-   Using the VS Code Command Palette instead of menu accelerators.
-   Reusing IDE shortcuts for save, find, navigation, and settings.
-   Avoiding conflicts with standard editor shortcuts.

------------------------------------------------------------------------

# Verification

Each shortcut workflow should be verified by:

-   Manual keyboard-only testing
-   Automated UI tests where practical
-   Accessibility review
-   Cross-platform verification (Windows, macOS, Linux)

------------------------------------------------------------------------

# Acceptance Criteria

Keyboard parity is achieved when:

1.  Every P0 workflow is keyboard accessible.
2.  No essential ontology engineering task requires a mouse.
3.  Shortcut conflicts are resolved or documented.
4.  Accessibility tests pass.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PROTEGE_MENU_AUDIT.md
-   PROTEGE_VIEW_AUDIT.md
-   PROTEGE_WORKFLOW_AUDIT.md
-   UI_WORKFLOW_PARITY.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
