# PROTEGE_PLUGIN_AUDIT

# Protégé Desktop Plugin Audit

**Status:** Living Reference Document\
**Purpose:** Audit the extension capabilities provided by a standard
Protégé Desktop installation and define the equivalent functionality
required from Strixonomy's native plugin platform.

> This document evaluates functional extensibility, not Java binary
> compatibility.

------------------------------------------------------------------------

# Purpose

Protégé derives much of its power from its extensibility. Strixonomy
should provide an equivalent capability through a modern, stable,
Rust-native plugin platform.

This audit identifies plugin categories, extension points, and workflows
that should be supported before claiming Protégé Desktop parity.

------------------------------------------------------------------------

# Audit Principles

-   Functional parity over Java compatibility.
-   Stable public APIs.
-   Versioned SDK.
-   Secure plugin execution.
-   Well-defined lifecycle.
-   Automated compatibility testing.

------------------------------------------------------------------------

# Plugin Categories

  -----------------------------------------------------------------------
  Category          Typical Purpose   Strixonomy          Status
                                      Equivalent        
  ----------------- ----------------- ----------------- -----------------
  Reasoner          Classification &  Reasoner adapter  REVIEW
                    inference         API               

  Visualization     Graphs and        Graph provider    REVIEW
                    diagrams          API               

  Import/Export     Additional        Import/export     REVIEW
                    formats           provider          

  Validation        Custom quality    Validator         REVIEW
                    rules             provider          

  Refactoring       Semantic          Refactoring       REVIEW
                    transformations   provider          

  Query             Custom query      Query provider    REVIEW
                    engines                             

  UI                Panels, dialogs,  View & command    REVIEW
                    commands          APIs              

  Utilities         Miscellaneous     General plugin    REVIEW
                    tooling           API               
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Required Extension Points

Plugins should be able to contribute:

-   Commands
-   Views
-   Editors
-   Diagnostics
-   Validators
-   Query providers
-   Reasoner adapters
-   Graph providers
-   Refactoring actions
-   Import/export handlers
-   Settings pages
-   Context menu actions

------------------------------------------------------------------------

# Plugin Lifecycle

Every plugin should support:

1.  Discovery
2.  Manifest validation
3.  Dependency resolution
4.  Registration
5.  Activation
6.  Runtime execution
7.  Graceful shutdown
8.  Cleanup

------------------------------------------------------------------------

# Manifest Requirements

Each plugin should declare:

-   Identifier
-   Name
-   Version
-   SDK compatibility
-   Description
-   Author
-   Extension points
-   Configuration schema
-   Optional permissions

------------------------------------------------------------------------

# Compatibility Goals

The plugin platform should provide:

-   Stable SDK across compatible releases
-   Semantic versioning
-   Migration guidance
-   Backward compatibility where practical
-   Isolation between plugins
-   Clear diagnostics for incompatible plugins

------------------------------------------------------------------------

# Known Audit Findings

Based on the repository audit:

-   A native plugin runtime already exists.
-   Manifest discovery is implemented.
-   Example plugins are present.
-   The SDK is functional but not yet considered stable.
-   Marketplace infrastructure is intentionally out of scope for v0.30.
-   Java Protégé plugin compatibility is not a project goal.

------------------------------------------------------------------------

# Acceptance Criteria

Plugin parity is achieved when:

1.  Required extension points are available.
2.  SDK documentation is complete.
3.  Example plugins demonstrate every extension point.
4.  Compatibility tests pass.
5.  Public APIs are versioned.
6.  Plugin lifecycle behavior is verified.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   PROTEGE_FEATURE_INVENTORY.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   IMPLEMENTATION_EVIDENCE.md
-   PLUGIN_PARITY.md
-   PARITY_MATRIX.md
-   PARITY_RELEASE_GATE.md
