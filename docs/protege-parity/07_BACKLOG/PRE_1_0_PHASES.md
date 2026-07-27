# PRE_1_0_PHASES

# Pre-1.0 Protégé Parity Release Phases

**Directory:** 07_BACKLOG\
**Status:** Canonical pre-1.0 release plan\
**Baseline:** Strixonomy v0.18.2\
**Target:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document maps the Protégé parity program onto **versioned pre-1.0
releases** (v0.19–v0.27, then 1.0.0-rc, then 1.0.0).

It is the single entry point for contributors implementing parity work
before 1.0.0. Engineering sequencing detail lives in
[EXECUTION_ORDER.md](../05_IMPLEMENTATION/EXECUTION_ORDER.md); P0 scope
lives in [P0_IMPLEMENTATION_PLAN.md](../05_IMPLEMENTATION/P0_IMPLEMENTATION_PLAN.md).

**v0.18 shipped a desktop UX shell gate** (menus, layouts, workflows,
migration readiness). **Full functional Protégé Desktop parity** is the
objective of the phases below.

------------------------------------------------------------------------

# Phase map

| Release | P0 phase | Stages | Primary epics | Primary blockers | Status |
|---------|----------|--------|---------------|------------------|--------|
| **v0.19** | A + baseline | 0–1 | EPIC-001, EPIC-011 (skeleton) | BLOCKER_01, BLOCKER_11 (manifest skeleton) | Complete |
| **v0.20** | B | 2 | EPIC-003 | BLOCKER_03 | Complete (branch) |
| **v0.21** | C (formats) | 3 | EPIC-001, EPIC-020 | BLOCKER_01, format audit | Shipped |
| **v0.22** | C (OWL 2) | 4 | EPIC-002 | BLOCKER_02 | Planned |
| **v0.23** | D (reason + SWRL) | 5–6 | EPIC-004, EPIC-005 | BLOCKER_04, BLOCKER_05 | Complete |
| **v0.24** | D (services) | 7 | EPIC-006, EPIC-007 | BLOCKER_06, BLOCKER_07 | Shipped |
| **v0.25** | E + F | 8–9 | EPIC-008–011 | BLOCKER_08–011 | Shipped |
| **v0.26** | F (test port) | — | Protégé JUnit behavioral port | BLOCKER_11 (corpus) | Shipped |
| **v0.27** | Product identity | — | Strixonomy Rust + VS Code rename | Package and extension migration | Planned |
| **1.0.0-rc** | Stabilize | 10 | — | — | Planned |
| **1.0.0** | Ship | — | — | [PARITY_RELEASE_GATE.md](../03_PARITY/PARITY_RELEASE_GATE.md) | Planned |

Stages are defined in [EXECUTION_ORDER.md](../05_IMPLEMENTATION/EXECUTION_ORDER.md).
Epics are indexed in [EPIC_INDEX.md](EPIC_INDEX.md).

------------------------------------------------------------------------

# v0.19 — Semantic foundation + program baseline

**Status:** Complete (v0.21.0, 2026-07-13)\
**P0 phase:** A (Semantic Foundation)\
**Stages:** 0–1\
**Theme:** Freeze parity scope and route all edits through semantic transactions.

## Objective

Establish a stable source of truth and the canonical semantic change model
required by every later phase.

## Primary documents

-   [BLOCKER_01_FORMAT_INDEPENDENCE.md](../04_BLOCKERS/BLOCKER_01_FORMAT_INDEPENDENCE.md)
-   [BLOCKER_11_PARITY_VERIFICATION.md](../04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md) (manifest skeleton)
-   [06_SUBSYSTEMS/FORMAT_SUPPORT.md](../06_SUBSYSTEMS/FORMAT_SUPPORT.md)
-   [PARITY_SCOPE.md](../PARITY_SCOPE.md)
-   [PARITY_MATRIX.md](../03_PARITY/PARITY_MATRIX.md)

## Deliverables

-   Frozen parity scope with stable requirement IDs
-   Synchronized parity matrix, gap analysis, and implementation evidence
-   Machine-readable parity manifest skeleton
-   Canonical ontology change types and transaction composition
-   Turtle and OBO edits routed through semantic transactions
-   Transaction regression tests; inverse operations for undo/redo

## Exit criteria

-   Scope is frozen; every P0 requirement has an ID
-   All supported edits flow through semantic transactions
-   Turtle and OBO behavior remains regression-free
-   No new UI feature writes serializer-specific patches directly
-   Parity status is reproducible from repository evidence

## Cursor session guidance

Target **one atomic issue per session** (e.g. define change types, route
one Turtle patch op, add one transaction test). Do not assign an entire
stage in one prompt. See [EXECUTION_ORDER.md § Cursor Session Strategy](../05_IMPLEMENTATION/EXECUTION_ORDER.md#cursor-session-strategy).

------------------------------------------------------------------------

# v0.20 — Workspace runtime

**Status:** Complete on `v0.20` branch (awaiting tag)\
**P0 phase:** B (Workspace Runtime)\
**Stage:** 2\
**Theme:** Make the workspace the central runtime for ontology state.

## Objective

Multi-ontology workspace orchestration with atomic transactions, dirty
state, save workflows, and session persistence.

## Primary documents

-   [BLOCKER_03_WORKSPACE.md](../04_BLOCKERS/BLOCKER_03_WORKSPACE.md)
-   [06_SUBSYSTEMS/WORKSPACE.md](../06_SUBSYSTEMS/WORKSPACE.md)

## Deliverables

-   Loaded ontology registry and active ontology state
-   Editability/read-only and per-ontology dirty state
-   Transaction manager and deterministic event publication
-   Selection and navigation managers
-   Save / Save All orchestration and session persistence
-   Panel restore and external-change recovery

## Exit criteria

-   Multiple ontologies managed reliably
-   Transactions are atomic and workspace-aware
-   Panel selection is synchronized across views
-   Workspace state survives restart
-   Save, Save All, and dirty-state workflows pass end-to-end tests

------------------------------------------------------------------------

# v0.21 — Required format write-back

**P0 phase:** C (Authoring & Formats — formats slice)\
**Stage:** 3\
**Theme:** Semantic round-trip for RDF/XML and OWL/XML.

## Objective

Enable full editing and semantic round-trip for every required format
beyond Turtle and OBO.

## Primary documents

-   [BLOCKER_01_FORMAT_INDEPENDENCE.md](../04_BLOCKERS/BLOCKER_01_FORMAT_INDEPENDENCE.md)
-   [02_PROTEGE_AUDIT/PROTEGE_FILE_FORMAT_AUDIT.md](../02_PROTEGE_AUDIT/PROTEGE_FILE_FORMAT_AUDIT.md)
-   [06_SUBSYSTEMS/FORMAT_SUPPORT.md](../06_SUBSYSTEMS/FORMAT_SUPPORT.md)

## Deliverables

-   Deterministic serializer policy
-   RDF/XML serializer adapter (metadata, annotations, anonymous nodes)
-   OWL/XML serializer adapter (metadata, imports, full axiom serialization)
-   Cross-format semantic comparator
-   Protégé-generated fixture corpus and save-conflict tests

## Exit criteria

-   Turtle, OBO, RDF/XML, and OWL/XML support open → edit → save → reload
-   Required metadata and annotations survive round-trip
-   Semantic comparison passes for the required fixture corpus
-   No required format remains browse-only

------------------------------------------------------------------------

# v0.22 — Complete OWL 2 authoring

**P0 phase:** C (Authoring & Formats — OWL 2 slice)\
**Stage:** 4\
**Theme:** Close all remaining OWL 2 structural and UI authoring gaps.

## Objective

Every P0 OWL 2 construct can be created, edited, deleted, and serialized
across required formats.

## Primary documents

-   [BLOCKER_02_OWL2_AUTHORING.md](../04_BLOCKERS/BLOCKER_02_OWL2_AUTHORING.md)
-   [06_SUBSYSTEMS/OWL2_AUTHORING.md](../06_SUBSYSTEMS/OWL2_AUTHORING.md)

## Deliverables

-   Atomic OWL 2 construct inventory mapped to model, transaction, parser,
    serializer, UI, and test
-   Missing TBox, RBox, and ABox axioms
-   Keys, datatype definitions, restrictions, facets, axiom annotations
-   Structured editors, validation, diagnostics, undo/redo coverage
-   Complete OWL 2 conformance fixtures

## Exit criteria

-   Every P0 OWL 2 construct is authorable end-to-end
-   All required formats preserve every supported construct
-   All OWL 2 authoring P0 requirements are VERIFIED

------------------------------------------------------------------------

# v0.23 — Reasoning parity + SWRL

**P0 phase:** D (Semantic Services — reason + SWRL slice)\
**Stages:** 5–6\
**Theme:** Complete TBox/ABox reasoning workflows and SWRL subsystem.

## Objective

Reasoning parity (classification, consistency, explanations, ABox) and
complete SWRL authoring, validation, serialization, and supported
execution workflows.

## Primary documents

-   [BLOCKER_04_REASONING.md](../04_BLOCKERS/BLOCKER_04_REASONING.md)
-   [BLOCKER_05_SWRL.md](../04_BLOCKERS/BLOCKER_05_SWRL.md)
-   [06_SUBSYSTEMS/REASONING.md](../06_SUBSYSTEMS/REASONING.md)
-   [06_SUBSYSTEMS/SWRL.md](../06_SUBSYSTEMS/SWRL.md)

## Deliverables

-   Full consistency semantics, realization, instance checking
-   Inferred class and property assertions; native DL explanation traces
-   Engine-level cancellation and incremental synchronization
-   SWRL semantic model, parser, serializer, rule browser/editor
-   SWRL fixture corpus and end-to-end workflow tests

## Exit criteria

-   Required TBox and ABox workflows pass conformance tests
-   Native explanations available for P0 workflows
-   Workspace and reasoner state remain synchronized
-   Standard SWRL rules can be created, edited, validated, saved, reopened
-   No reasoning- or SWRL-related P0 gap remains open

------------------------------------------------------------------------

# v0.24 — Semantic services completion

**Status:** Shipped (v0.24.0)\
**P0 phase:** D (Semantic Services — refactor + query slice)\
**Stage:** 7\
**Theme:** Advanced ontology engineering workflows on a stable semantic core.

## Objective

P0 refactoring and query/search parity after the semantic core stabilizes.

## Primary documents

-   [BLOCKER_06_REFACTORING.md](../04_BLOCKERS/BLOCKER_06_REFACTORING.md)
-   [BLOCKER_07_QUERY.md](../04_BLOCKERS/BLOCKER_07_QUERY.md)
-   [06_SUBSYSTEMS/REFACTORING.md](../06_SUBSYSTEMS/REFACTORING.md)
-   [06_SUBSYSTEMS/QUERY.md](../06_SUBSYSTEMS/QUERY.md)

## Deliverables

-   Workspace-wide rename, merge, replace-reference, move axioms
-   Ontology merge, flatten imports, locality-based module extraction
-   SWRL-aware refactoring with multi-format regression coverage
-   Unified query result model; DL Query; semantic search and usage analysis
-   Asserted/inferred query modes; saved queries and export workflows

## Exit criteria

-   All P0 refactoring workflows are atomic, previewable, and reversible
-   DL Query and workspace-wide search reach parity
-   Query and refactoring P0 requirements are VERIFIED

------------------------------------------------------------------------

# v0.25 — UX completion + executable verification

**Status:** Shipped\
**P0 phase:** E (User Experience) + F (Verification)\
**Stages:** 8–9\
**Theme:** Visualization, plugin SDK freeze, accessibility, and automated parity gates.

## Objective

Complete user-facing parity, stabilize extensibility, and make parity
status automatically verifiable in CI.

## Primary documents

-   [BLOCKER_08_VISUALIZATION.md](../04_BLOCKERS/BLOCKER_08_VISUALIZATION.md)
-   [BLOCKER_09_PLUGIN_PLATFORM.md](../04_BLOCKERS/BLOCKER_09_PLUGIN_PLATFORM.md)
-   [BLOCKER_10_ACCESSIBILITY.md](../04_BLOCKERS/BLOCKER_10_ACCESSIBILITY.md)
-   [BLOCKER_11_PARITY_VERIFICATION.md](../04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md)
-   [06_SUBSYSTEMS/VISUALIZATION.md](../06_SUBSYSTEMS/VISUALIZATION.md)
-   [06_SUBSYSTEMS/PLUGINS.md](../06_SUBSYSTEMS/PLUGINS.md)
-   [PARITY_RELEASE_GATE.md](../03_PARITY/PARITY_RELEASE_GATE.md)

## Deliverables

-   Standardized graph model with filtering, inferred overlays, virtualization
-   Public extension points frozen; plugin SDK 1.0 documented
-   Accessibility audit closed; keyboard and screen-reader coverage
-   Parity manifest populated; CI validator and release-gate automation
-   Conformance suite aggregation and parity metric generation

## Exit criteria

-   Required visualization workflows are complete
-   Plugin SDK 1.0 is stable and documented
-   All P0 workflows are keyboard accessible
-   Every P0 requirement has automated evidence in CI
-   Release readiness is objective and reproducible

------------------------------------------------------------------------

# v0.26 — Protégé Desktop test port

**Status:** Shipped\
**Theme:** Port portable Protégé Desktop JUnit behaviors into Strixonomy
Rust semantic oracles (rewrite specs — do not run the JVM suite).

## Primary documents

-   [PROTEGE_TEST_PORT.md](../03_PARITY/PROTEGE_TEST_PORT.md)
-   [`parity/protege-test-port.yaml`](../../../parity/protege-test-port.yaml)
-   [BLOCKER_11_PARITY_VERIFICATION.md](../04_BLOCKERS/BLOCKER_11_PARITY_VERIFICATION.md)

## Deliverables

-   Full upstream test-class inventory tagged `PORT_W1` / `PORT_W2` /
    `PORT_W3` / `PORT_W4` / `SKIP` / `COVERED`
-   Wave 1 oracle suites: hierarchy, merge, deprecation, history,
    axiom location, refs/defined-class, parsers/IDs
-   Wave 2 presentation suites: render/escape/prefix/IRI + annotation
    link extractors (LSP hover + Entity Inspector)
-   Wave 3 util suites: abbreviate, ISO8601, lexical replace, markdown
    entity links, annotation-property order; OBO Foundry registry JSON
    (vendored fixture)
-   Follow-ons: IdPolicy semantic parse; `catalog-v001.xml` redirects
    (`PORT_W4`); fixture hardening; expanded Foundry stress
-   Synthetic fixtures under `examples/protege-roundtrip/ported/`
-   `scripts/validate-protege-test-port.py` + CI wiring

## Exit criteria

-   Every `PORT_W1` / `PORT_W2` / `PORT_W3` / `PORT_W4` row has
    `strixonomy_tests` paths that exist (or an explicit `gap`)
-   `cargo test -p strixonomy --test protege_port_*` green
-   Webview annotation link + annotation-order Vitest green

------------------------------------------------------------------------

# v0.27 — Strixonomy rename

**Status:** Planned\
**Theme:** Rename Strixonomy and Strixonomy to Strixonomy before the 1.0 release freeze.

## Objective

Publish the Rust engine, CLI, language server, and VS Code extension under
the Strixonomy identity while giving v0.26 users an explicit, tested
migration path.

## Deliverables

-   `strixonomy` and `strixonomy-*` Rust packages
-   `strixonomy` CLI and `strixonomy-lsp` language-server binaries
-   Strixonomy for VS Code publisher, listing, commands, settings, views,
    bundled assets, Marketplace metadata, and Open VSX metadata
-   Compatibility and deprecation policy for Strixonomy/Strixonomy package,
    binary, configuration, extension-state, plugin, and automation names
-   v0.27 migration guide and CI coverage for clean installs and v0.26
    upgrades

## Exit criteria

-   New installs use Strixonomy names end to end
-   Existing users do not silently lose configuration, workspace state,
    plugins, or CI compatibility
-   Old names remain only in time-bounded compatibility paths, migration
    documentation, and historical release material

------------------------------------------------------------------------

# 1.0.0-rc — Release candidate

**Stage:** 10\
**Theme:** Stabilize without expanding scope.

## Allowed work

-   Bug fixes, performance improvements, documentation corrections
-   Test stabilization, packaging fixes, accessibility fixes
-   Migration guidance

## Prohibited work

-   New major features or architectural subsystems
-   Unapproved parity scope changes or breaking SDK changes

## Required validation

-   Full conformance and regression suites
-   Cross-platform tests and large-ontology benchmarks
-   Accessibility audit and Protégé migration trials
-   Final documentation review

## Exit criteria

-   All P0 requirements VERIFIED
-   All release gates pass; zero open P0 defects
-   Public APIs frozen; release sign-off complete

------------------------------------------------------------------------

# 1.0.0 — Protégé replacement release

**Theme:** Production-ready Protégé Desktop replacement in VS Code.

## Deliverables

-   Cross-platform VS Code extension and CLI release
-   Published `strixonomy` + `strixonomy-*` 1.0.0 on crates.io
-   Migration guides with honest parity table
-   Stable CLI/API/LSP semver 1.0

## Exit criteria

> Daily ontology engineering (OWL 2 DL + OBO maintenance) is completable
> in VS Code. Protégé is required only for **P2** features defined in
> [PARITY_SCOPE.md](../PARITY_SCOPE.md).

See [PARITY_RELEASE_GATE.md](../03_PARITY/PARITY_RELEASE_GATE.md) and
[08_RELEASE/RELEASE_CHECKLIST.md](../08_RELEASE/RELEASE_CHECKLIST.md).

------------------------------------------------------------------------

# Recommended immediate sequence

The next five implementation sessions (from
[EXECUTION_ORDER.md](../05_IMPLEMENTATION/EXECUTION_ORDER.md)):

1.  Create the machine-readable parity manifest skeleton (v0.20)
2.  Define the canonical semantic change API (v0.20)
3.  Route one existing Turtle edit through the new API (v0.20)
4.  Add workspace ontology registry and dirty state (v0.20)
5.  Design and test the first RDF/XML semantic write-back slice (v0.21)

------------------------------------------------------------------------

# Related documents

-   [README.md](../README.md) — parity program overview
-   [ROADMAP.md](ROADMAP.md) — post-1.0 backlog (1.1+)
-   [EXECUTION_ORDER.md](../05_IMPLEMENTATION/EXECUTION_ORDER.md) — stage sequencing
-   [P0_IMPLEMENTATION_PLAN.md](../05_IMPLEMENTATION/P0_IMPLEMENTATION_PLAN.md) — P0 scope
-   [EPIC_INDEX.md](EPIC_INDEX.md) — epic registry
-   [PARITY_STATUS.md](../03_PARITY/PARITY_STATUS.md) — progress dashboard
-   [Platform roadmap](../../roadmap.md) — release timeline integration
