# ROADMAP

# Strixonomy 1.x Engineering Roadmap

**Directory:** 07_BACKLOG\
**Status:** Living Backlog Roadmap

------------------------------------------------------------------------

# Purpose

This roadmap captures work that extends **beyond** the release-blocking
Protégé parity effort (1.1 and later). Items here should not delay the
1.0 release unless they are explicitly promoted to P0.

**Pre-1.0 parity releases** (v0.19–v0.25 → 1.0.0) are **not** defined
here. Use the canonical pre-1.0 plan instead:

-   [PRE_1_0_PHASES.md](PRE_1_0_PHASES.md) — versioned release phases
-   [EXECUTION_ORDER.md](../05_IMPLEMENTATION/EXECUTION_ORDER.md) — stage sequencing
-   [P0_IMPLEMENTATION_PLAN.md](../05_IMPLEMENTATION/P0_IMPLEMENTATION_PLAN.md) — P0 scope

------------------------------------------------------------------------

# Pre-1.0 (do not use this file)

| Release | Theme | Status | Plan |
|---------|-------|--------|------|
| v0.19 | Semantic foundation + program baseline | **Shipped** (v0.21.0) | [PRE_1_0_PHASES.md § v0.19](PRE_1_0_PHASES.md#v019-semantic-foundation-program-baseline) |
| v0.20 | Workspace runtime | Planned | [PRE_1_0_PHASES.md § v0.20](PRE_1_0_PHASES.md#v020-workspace-runtime) |
| v0.21 | Required format write-back | Shipped | [PRE_1_0_PHASES.md § v0.21](PRE_1_0_PHASES.md#v021-required-format-write-back) |
| v0.22 | Complete OWL 2 authoring | Planned | [PRE_1_0_PHASES.md § v0.22](PRE_1_0_PHASES.md#v022-complete-owl-2-authoring) |
| v0.23 | Reasoning parity + SWRL | Planned | [PRE_1_0_PHASES.md § v0.23](PRE_1_0_PHASES.md#v023-reasoning-parity-swrl) |
| v0.24 | Semantic services completion | Planned | [PRE_1_0_PHASES.md § v0.24](PRE_1_0_PHASES.md#v024-semantic-services-completion) |
| v0.25 | UX completion + executable verification | Planned | [PRE_1_0_PHASES.md § v0.25](PRE_1_0_PHASES.md#v025-ux-completion-executable-verification) |
| 1.0.0-rc | Stabilize | Planned | [PRE_1_0_PHASES.md § 1.0.0-rc](PRE_1_0_PHASES.md#100-rc-release-candidate) |
| 1.0.0 | Ship | Planned | [PRE_1_0_PHASES.md § 1.0.0](PRE_1_0_PHASES.md#100-protege-replacement-release) |

------------------------------------------------------------------------

# Guiding Principles

-   Protect 1.0 stability
-   Ship small, measurable increments
-   Prefer platform capabilities over one-off features
-   Validate ideas with prototypes before broad adoption

------------------------------------------------------------------------

# 1.0 (P0)

Complete via [PRE_1_0_PHASES.md](PRE_1_0_PHASES.md) (v0.19–v0.25 → 1.0.0-rc → 1.0.0):

-   Protégé parity
-   Executable parity verification
-   Stable plugin SDK
-   Migration guides
-   Cross-platform release

------------------------------------------------------------------------

# Post-1.0 only (1.1+)

The sections below are **post-1.0 backlog**. They must not delay 1.0
unless explicitly promoted to P0.

# 1.1

Focus:

-   Performance optimization
-   Workspace polish
-   Visualization enhancements
-   Additional serializer improvements
-   Better diagnostics

Deliverables:

-   Faster indexing
-   Improved graph layouts
-   Better startup time
-   Enhanced command palette

------------------------------------------------------------------------

# 1.2

Focus:

-   Collaboration
-   Enterprise readiness
-   Query enhancements

Deliverables:

-   Shared workspaces
-   Review workflows
-   Federated queries
-   Audit logging

------------------------------------------------------------------------

# 1.3

Focus:

-   AI-assisted ontology engineering

Deliverables:

-   Ontology generation
-   Semantic code actions
-   Automated repair suggestions
-   Documentation generation

------------------------------------------------------------------------

# 1.4

Focus:

-   Ecosystem expansion

Deliverables:

-   Plugin marketplace
-   SDK tooling
-   Reference extensions
-   Community templates

------------------------------------------------------------------------

# Future Research

-   Distributed reasoning
-   Knowledge graph analytics
-   Live collaboration
-   Cloud workspaces
-   Ontology notebooks
-   Advanced graph visualization
-   Agent-assisted ontology engineering

------------------------------------------------------------------------

# Backlog Management

Every backlog item should include:

-   Unique ID
-   Priority
-   Owner
-   Dependencies
-   Acceptance criteria
-   Linked GitHub issue
-   Estimated effort

------------------------------------------------------------------------

# Promotion Rules

A backlog item may move to active development when:

-   Dependencies are satisfied
-   Architecture impact is understood
-   Acceptance criteria are defined
-   Engineering capacity is available

------------------------------------------------------------------------

# Related Documents

-   PRE_1_0_PHASES.md — **pre-1.0 release phases (canonical)**
-   P0_IMPLEMENTATION_PLAN.md
-   P1_IMPLEMENTATION_PLAN.md
-   IMPLEMENTATION_PLAN.md
-   EXECUTION_ORDER.md
-   DEPENDENCY_GRAPH.md
