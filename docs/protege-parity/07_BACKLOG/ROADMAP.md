# ROADMAP

# Strixonomy 1.x Engineering Roadmap

**Directory:** 07_BACKLOG\
**Status:** Living Backlog Roadmap

------------------------------------------------------------------------

# Purpose

This roadmap captures work that extends **beyond** the release-blocking
Protégé parity effort (1.1 and later). Items here should not delay the
v0.30 release unless they are explicitly promoted to P0.

**v0.29–v0.30 parity releases** (v0.19–v0.30) are **not** defined
here. Use the canonical v0.29–v0.30 plan instead:

-   [V0_30_PHASES.md](V0_30_PHASES.md) — versioned release phases
-   [EXECUTION_ORDER.md](../05_IMPLEMENTATION/EXECUTION_ORDER.md) — stage sequencing
-   [P0_IMPLEMENTATION_PLAN.md](../05_IMPLEMENTATION/P0_IMPLEMENTATION_PLAN.md) — P0 scope

------------------------------------------------------------------------

# v0.29–v0.30 (do not use this file)

| Release | Theme | Status | Plan |
|---------|-------|--------|------|
| v0.19 | Semantic foundation + program baseline | **Shipped** (v0.21.0) | [V0_30_PHASES.md § v0.19](V0_30_PHASES.md#v019-semantic-foundation-program-baseline) |
| v0.20 | Workspace runtime | Planned | [V0_30_PHASES.md § v0.20](V0_30_PHASES.md#v020-workspace-runtime) |
| v0.21 | Required format write-back | Shipped | [V0_30_PHASES.md § v0.21](V0_30_PHASES.md#v021-required-format-write-back) |
| v0.22 | Complete OWL 2 authoring | Planned | [V0_30_PHASES.md § v0.22](V0_30_PHASES.md#v022-complete-owl-2-authoring) |
| v0.23 | Reasoning parity + SWRL | Planned | [V0_30_PHASES.md § v0.23](V0_30_PHASES.md#v023-reasoning-parity-swrl) |
| v0.24 | Semantic services completion | Planned | [V0_30_PHASES.md § v0.24](V0_30_PHASES.md#v024-semantic-services-completion) |
| v0.25 | UX completion + executable verification | Planned | [V0_30_PHASES.md § v0.25](V0_30_PHASES.md#v025-ux-completion-executable-verification) |
| v0.29 | Stabilize | Planned | [V0_30_PHASES.md § v0.29](V0_30_PHASES.md#100-rc-release-candidate) |
| v0.30 | Ship | Planned | [V0_30_PHASES.md § v0.30](V0_30_PHASES.md#v030-protege-replacement-release) |

------------------------------------------------------------------------

# Guiding Principles

-   Protect v0.30 stability
-   Ship small, measurable increments
-   Prefer platform capabilities over one-off features
-   Validate ideas with prototypes before broad adoption

------------------------------------------------------------------------

# v0.30 (P0)

Complete via [V0_30_PHASES.md](V0_30_PHASES.md) (v0.19–v0.25 → v0.29 → v0.30):

-   Protégé parity
-   Executable parity verification
-   Stable plugin SDK
-   Migration guides
-   Cross-platform release

------------------------------------------------------------------------

# v0.31+ only (1.1+)

The sections below are **v0.31+ backlog**. They must not delay v0.30
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

-   V0_30_PHASES.md — **v0.29–v0.30 release phases (canonical)**
-   P0_IMPLEMENTATION_PLAN.md
-   P1_IMPLEMENTATION_PLAN.md
-   IMPLEMENTATION_PLAN.md
-   EXECUTION_ORDER.md
-   DEPENDENCY_GRAPH.md
