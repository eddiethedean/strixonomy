# BLOCKER_09_PLUGIN_PLATFORM

# Blocker 09 --- Plugin Platform & SDK Parity

**Status:** Resolved for v0.25 (EPIC-009) — SDK 1.0 freeze on TOML + subprocess host\
**Priority:** Critical\
**Target Release:** Strixonomy v0.30.0 / delivered functional baseline in **v0.25**

------------------------------------------------------------------------

# Purpose

This document defines the engineering work required for Strixonomy to
deliver a stable, production-ready plugin platform comparable to the
extensibility expected by Protégé Desktop while embracing a modern
Rust-native architecture.

The objective is functional extensibility---not Java binary
compatibility.

------------------------------------------------------------------------

# Problem Statement

The repository audit identified an existing plugin runtime, manifest
discovery, and example plugins. However, the SDK, extension points,
lifecycle guarantees, compatibility policy, and verification
infrastructure must mature before v0.30.

Without a stable plugin platform, Strixonomy cannot provide a reliable
ecosystem for third-party extensions.

------------------------------------------------------------------------

# Goals

Provide:

-   Stable versioned SDK
-   Public extension APIs
-   Safe plugin lifecycle
-   Dependency management
-   Workspace integration
-   Sandboxed execution where appropriate
-   Comprehensive developer documentation
-   Automated compatibility testing

------------------------------------------------------------------------

# Non-Goals

This blocker does **not** require:

-   Java Protégé plugin compatibility
-   A centralized online marketplace
-   Remote code execution
-   Binary compatibility across major SDK versions

------------------------------------------------------------------------

# Required Extension Points

Plugins should be able to contribute:

-   Commands
-   Views
-   Editors
-   Validators
-   Diagnostics
-   Query providers
-   Reasoner adapters
-   Graph providers
-   Refactoring providers
-   Import/export providers
-   Language services
-   Settings pages
-   Context menu actions
-   Tool windows

------------------------------------------------------------------------

# Plugin Lifecycle

Every plugin must support:

1.  Discovery
2.  Manifest validation
3.  Dependency resolution
4.  Registration
5.  Activation
6.  Runtime execution
7.  Deactivation
8.  Cleanup

Lifecycle events should be deterministic and observable.

------------------------------------------------------------------------

# SDK Requirements

Provide:

-   Stable public APIs
-   Semantic versioning
-   Backward compatibility policy
-   Typed interfaces
-   Comprehensive examples
-   Migration guides
-   API reference documentation

------------------------------------------------------------------------

# Workspace Integration

Plugins should integrate with:

-   Workspace transactions
-   Event bus
-   Selection manager
-   Navigation manager
-   Reasoning manager
-   Query manager
-   Visualization manager
-   Settings persistence

------------------------------------------------------------------------

# Security

The platform should support:

-   Manifest validation
-   Capability declarations
-   Version compatibility checks
-   Graceful failure isolation
-   Diagnostic reporting

------------------------------------------------------------------------

# Architecture

``` text
Workspace
    │
    ▼
Plugin Manager
    │
    ├── Registry
    ├── Manifest Loader
    ├── Dependency Resolver
    ├── Lifecycle Manager
    ├── Extension Registry
    ├── Event Bridge
    └── SDK Interfaces
```

All plugins should communicate through stable public interfaces rather
than internal implementation details.

------------------------------------------------------------------------

# Dependencies

Depends on:

-   BLOCKER_03_WORKSPACE.md

Enables:

-   Community ecosystem
-   Third-party integrations
-   Specialized ontology tooling
-   Long-term platform growth

------------------------------------------------------------------------

# Implementation Phases

## Phase 1

-   Audit existing plugin infrastructure
-   Freeze public extension points
-   Define SDK policy

## Phase 2

-   Complete lifecycle management
-   Expand extension APIs
-   Improve diagnostics

## Phase 3

-   Produce sample plugins
-   Complete documentation
-   Compatibility testing

## Phase 4

-   Performance tuning
-   Regression suite
-   SDK stabilization
-   v0.30 API freeze

------------------------------------------------------------------------

# Risks

-   API churn
-   Dependency conflicts
-   Plugin isolation failures
-   Workspace synchronization bugs

Mitigate through semantic versioning, automated compatibility testing,
API reviews, and clear deprecation policies.

------------------------------------------------------------------------

# Acceptance Criteria

This blocker is complete when:

-   Public SDK is versioned and documented.
-   Required extension points are available.
-   Lifecycle behavior is deterministic.
-   Compatibility tests pass.
-   Example plugins demonstrate every extension point.
-   Plugin regression suite passes.

------------------------------------------------------------------------

# Success Metrics

-   100% required extension points implemented
-   Stable 1.0 SDK
-   Zero release-blocking plugin defects
-   All plugin parity requirements VERIFIED

------------------------------------------------------------------------

# Related Documents

-   PLUGIN_PARITY.md
-   PROTEGE_PLUGIN_AUDIT.md
-   CURRENT_ARCHITECTURE.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
-   BLOCKER_03_WORKSPACE.md
