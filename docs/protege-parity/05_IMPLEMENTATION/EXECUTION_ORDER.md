# EXECUTION_ORDER

# Strixonomy 1.0 Protégé Parity Execution Order

**Status:** Master Sequencing Document\
**Target Release:** Strixonomy 1.0.0\
**Versioned releases:** [PRE_1_0_PHASES.md](../07_BACKLOG/PRE_1_0_PHASES.md) (v0.19–v0.25 → 1.0.0-rc → 1.0.0)

------------------------------------------------------------------------

# Stage-to-release map

| Stage | Release | Theme |
|-------|---------|-------|
| 0–1 | v0.19 | Program baseline + semantic foundation |
| 2 | v0.20 | Workspace runtime |
| 3 | v0.21 | Required format write-back |
| 4 | v0.22 | Complete OWL 2 authoring |
| 5–6 | v0.23 | Reasoning parity + SWRL |
| 7 | v0.24 | Semantic services (refactor + query) |
| 8–9 | v0.25 | Viz, plugins, a11y + parity verification |
| 10 | 1.0.0-rc | Release candidate stabilization |

------------------------------------------------------------------------

# Purpose

This document defines the recommended order for implementing the
remaining Protégé parity work.

It converts the dependency graph into a concrete engineering sequence
and identifies which work must happen serially, which work can proceed
in parallel, and which milestones must be completed before dependent
work begins.

------------------------------------------------------------------------

# Sequencing Principles

1.  Build shared semantic foundations before adding more UI-specific
    behavior.
2.  Avoid parallel implementations of the same semantic logic.
3.  Complete enabling architecture before dependent features.
4.  Keep testing and documentation active throughout every phase.
5.  Prefer small, verifiable slices over large rewrites.
6.  Do not mark a phase complete until its exit criteria are satisfied.

------------------------------------------------------------------------

# Stage 0 --- Freeze the Program Baseline

## Objective

Establish a stable source of truth before implementation begins.

## Required Work

-   Finalize `PARITY_SCOPE.md`
-   Finalize `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md`
-   Assign stable parity requirement IDs
-   Synchronize:
    -   `PARITY_MATRIX.md`
    -   `IMPLEMENTATION_EVIDENCE.md`
    -   `PARITY_GAP_ANALYSIS.md`
-   Create the initial machine-readable parity manifest
-   Create GitHub epics for every P0 blocker

## Exit Criteria

-   Scope is frozen
-   Every P0 requirement has an ID
-   Every blocker has an owner and milestone
-   Parity status is reproducible from repository evidence

------------------------------------------------------------------------

# Stage 1 --- Canonical Semantic Change Model

## Primary Blocker

`BLOCKER_01_FORMAT_INDEPENDENCE.md`

## Objective

Create the shared semantic editing foundation required by all later
phases.

## Execution Order

1.  Define canonical ontology change types
2.  Define transaction composition
3.  Define inverse operations
4.  Define validation hooks
5.  Define change-set serialization for debugging and tests
6.  Route existing Turtle edits through the new model
7.  Route existing OBO edits through the new model
8.  Add transaction regression tests
9.  Deprecate serializer-specific business logic

## Parallel Work

-   Build semantic round-trip fixtures
-   Expand OWL 2 requirement inventory
-   Design workspace transaction API
-   Draft serializer architecture decisions

## Exit Criteria

-   All supported edits flow through semantic transactions
-   Turtle and OBO behavior remains regression-free
-   Undo/redo can consume inverse semantic changes
-   No new UI feature writes serializer-specific patches directly

------------------------------------------------------------------------

# Stage 2 --- Workspace Runtime

## Primary Blocker

`BLOCKER_03_WORKSPACE.md`

## Objective

Make the workspace the central runtime for ontology state and semantic
transactions.

## Execution Order

1.  Formalize loaded ontology registry
2.  Add active ontology state
3.  Add editability/read-only state
4.  Add per-ontology dirty state
5.  Implement transaction manager
6.  Implement deterministic event publication
7.  Implement selection manager
8.  Implement navigation manager
9.  Implement save and Save All orchestration
10. Implement session persistence
11. Restore live panels and semantic state
12. Add recovery and external-change handling

## Parallel Work

-   Create workspace integration fixtures
-   Expand end-to-end UI tests
-   Design reasoner synchronization contract
-   Design plugin workspace APIs

## Exit Criteria

-   Multiple ontologies are managed reliably
-   Transactions are atomic and workspace-aware
-   Panel selection is synchronized
-   Workspace state survives restart
-   Save, Save All, and dirty-state workflows pass end-to-end tests

------------------------------------------------------------------------

# Stage 3 --- Required Format Write-Back

## Primary Blockers

-   `BLOCKER_01_FORMAT_INDEPENDENCE.md`
-   `PROTEGE_FILE_FORMAT_AUDIT.md`

## Objective

Enable full editing and semantic round-trip for the required formats.

## Execution Order

1.  Define deterministic serializer policy
2.  Implement RDF/XML serializer adapter
3.  Add RDF/XML ontology metadata support
4.  Add RDF/XML axiom annotation support
5.  Add RDF/XML anonymous-node support
6.  Implement OWL/XML serializer adapter
7.  Add OWL/XML metadata and import support
8.  Add OWL/XML full axiom serialization
9.  Build cross-format semantic comparator
10. Add Protégé-generated fixture corpus
11. Add save-conflict and malformed-input tests

## Parallel Work

-   Expand format documentation
-   Benchmark serializer performance
-   Build conversion workflow tests
-   Validate real-world ontology samples

## Exit Criteria

-   Turtle, OBO, RDF/XML, and OWL/XML support open-edit-save-reload
-   Required metadata and annotations survive round-trip
-   Semantic comparison passes for the required fixture corpus
-   No required format remains browse-only

------------------------------------------------------------------------

# Stage 4 --- Complete OWL 2 Authoring

## Primary Blocker

`BLOCKER_02_OWL2_AUTHORING.md`

## Objective

Close all remaining OWL 2 structural and UI authoring gaps.

## Execution Order

1.  Finalize atomic OWL 2 construct inventory
2.  Map every construct to:
    -   Model
    -   Transaction
    -   Parser
    -   Serializer
    -   UI
    -   Test
3.  Implement missing TBox axioms
4.  Implement missing RBox/property axioms
5.  Implement missing ABox assertions
6.  Implement keys and datatype definitions
7.  Implement datatype restrictions and facets
8.  Implement axiom annotations
9.  Add structured editors
10. Add validation and diagnostics
11. Add undo/redo coverage
12. Add complete OWL 2 conformance fixtures

## Parallel Work

-   Documentation for completed construct families
-   Refactoring compatibility tests
-   Reasoning fixtures for newly supported axioms
-   Accessibility testing for new editors

## Exit Criteria

-   Every P0 OWL 2 construct can be created, edited, deleted, and
    serialized
-   All required formats preserve every supported construct
-   All OWL 2 authoring requirements are VERIFIED

------------------------------------------------------------------------

# Stage 5 --- Reasoning Parity

## Primary Blocker

`BLOCKER_04_REASONING.md`

## Objective

Complete TBox and ABox reasoning workflows and explanation quality.

## Execution Order

1.  Freeze reasoner interface contract
2.  Add full consistency semantics
3.  Implement realization
4.  Implement instance checking
5.  Add inferred class assertions
6.  Add inferred object property assertions
7.  Add same-individual inference where applicable
8.  Implement native DL explanation traces
9.  Add engine-level cancellation
10. Add incremental synchronization
11. Add cross-reasoner conformance corpus
12. Add benchmark suite

## Parallel Work

-   UI improvements for inferred/asserted views
-   Explanation panel enhancements
-   Performance profiling
-   Query integration tests

## Exit Criteria

-   Required TBox and ABox workflows pass conformance tests
-   Native explanations are available for P0 workflows
-   Workspace and reasoner state remain synchronized
-   No reasoning-related P0 gap remains open

------------------------------------------------------------------------

# Stage 6 --- SWRL

## Primary Blocker

`BLOCKER_05_SWRL.md`

## Objective

Implement complete SWRL authoring, validation, serialization, and
supported execution workflows.

## Execution Order

1.  Define SWRL semantic model
2.  Implement parser
3.  Implement serializer support
4.  Implement rule browser
5.  Implement rule editor
6.  Implement variables and entity completion
7.  Implement built-in validation
8.  Add search and usage analysis
9.  Integrate workspace transactions and undo/redo
10. Integrate reasoning behavior
11. Add SWRL fixture corpus
12. Add end-to-end workflow tests

## Parallel Work

-   SWRL user documentation
-   Built-in reference documentation
-   Accessibility testing for rule editor
-   Refactoring support for rule references

## Exit Criteria

-   Standard SWRL rules can be created, edited, validated, saved, and
    reopened
-   Required execution behavior is documented and tested
-   SWRL P0 requirements are VERIFIED

------------------------------------------------------------------------

# Stage 7 --- Semantic Services Completion

## Primary Blockers

-   `BLOCKER_06_REFACTORING.md`
-   `BLOCKER_07_QUERY.md`

## Objective

Complete advanced ontology engineering workflows after the semantic core
stabilizes.

## Refactoring Order

1.  Complete workspace-wide rename
2.  Complete merge and replace-reference workflows
3.  Implement move selected axioms
4.  Implement ontology merge
5.  Implement flatten imports
6.  Implement locality-based module extraction
7.  Add SWRL-aware refactoring
8.  Add full multi-format regression coverage

## Query Order

1.  Define unified query result model
2.  Complete DL Query
3.  Expand semantic search
4.  Expand usage analysis
5.  Add asserted/inferred query modes
6.  Add saved queries and history persistence
7.  Add export and navigation workflows
8.  Add performance tests

## Exit Criteria

-   All P0 refactoring workflows are atomic, previewable, and reversible
-   DL Query and workspace-wide search reach parity
-   Query and refactoring P0 requirements are VERIFIED

------------------------------------------------------------------------

# Stage 8 --- Visualization, Plugins, and Accessibility

## Primary Blockers

-   `BLOCKER_08_VISUALIZATION.md`
-   `BLOCKER_09_PLUGIN_PLATFORM.md`
-   `BLOCKER_10_ACCESSIBILITY.md`

## Objective

Complete user-facing parity and stabilize extensibility.

## Visualization Order

1.  Standardize graph model
2.  Complete filtering and navigation
3.  Add inferred and query overlays
4.  Add refactoring previews
5.  Add large-graph virtualization
6.  Add accessible alternative views

## Plugin Order

1.  Freeze public extension points
2.  Define versioning and compatibility policy
3.  Complete lifecycle management
4.  Add reasoner/query/refactor/visualization providers
5.  Add compatibility harness
6.  Ship reference plugins
7.  Freeze SDK 1.0

## Accessibility Order

1.  Run complete accessibility audit
2.  Close keyboard navigation gaps
3.  Fix focus restoration
4.  Add screen-reader announcements
5.  Add graph alternative views
6.  Add automated accessibility tests
7.  Complete external/manual verification

## Exit Criteria

-   Required visualization workflows are complete
-   Plugin SDK 1.0 is stable and documented
-   All P0 workflows are keyboard accessible
-   No release-blocking accessibility issue remains

------------------------------------------------------------------------

# Stage 9 --- Executable Parity Verification

## Primary Blocker

`BLOCKER_11_PARITY_VERIFICATION.md`

## Objective

Make parity status automatically verifiable.

## Execution Order

1.  Finalize parity manifest schema
2.  Populate every P0 requirement
3.  Link source, tests, documentation, and issues
4.  Add manifest validator
5.  Add CI path validation
6.  Add conformance suite aggregation
7.  Add parity metric generation
8.  Add release-gate automation
9.  Generate status documentation
10. Block release on incomplete P0 evidence

## Exit Criteria

-   Every P0 requirement has automated evidence
-   CI calculates parity metrics
-   Documentation is generated or validated from the manifest
-   Release readiness is objective and reproducible

------------------------------------------------------------------------

# Stage 10 --- Release Candidate

## Objective

Stabilize without expanding scope.

## Allowed Work

-   Bug fixes
-   Performance improvements
-   Documentation corrections
-   Test stabilization
-   Packaging fixes
-   Accessibility fixes
-   Migration guidance

## Prohibited Work

-   New major features
-   New architectural subsystems
-   Unapproved parity scope changes
-   Breaking SDK changes

## Required Validation

-   Full conformance suite
-   Full regression suite
-   Cross-platform tests
-   Large ontology benchmarks
-   Accessibility audit
-   Real Protégé migration trials
-   Final documentation review

## Exit Criteria

-   All P0 requirements VERIFIED
-   All release gates pass
-   Zero open P0 defects
-   Public APIs frozen
-   Release sign-off complete

------------------------------------------------------------------------

# Cursor Session Strategy

Each Cursor session should:

1.  Target one atomic issue or tightly related issue group
2.  Inspect existing implementation before editing
3.  Preserve backward compatibility unless explicitly approved
4.  Add tests in the same session
5.  Update implementation evidence
6.  Update parity status only after tests pass
7.  Report exact commands and results

Avoid assigning Cursor an entire stage in one prompt.

------------------------------------------------------------------------

# Recommended Immediate Sequence

The next five implementation sessions should be:

1.  Create the machine-readable parity manifest skeleton
2.  Define the canonical semantic change API
3.  Route one existing Turtle edit through the new API
4.  Add workspace ontology registry and dirty state
5.  Design and test the first RDF/XML semantic write-back slice

------------------------------------------------------------------------

# Related Documents

-   PRE_1_0_PHASES.md — **versioned release phases (canonical)**
-   IMPLEMENTATION_PLAN.md
-   DEPENDENCY_GRAPH.md
-   PARITY_MATRIX.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   BLOCKER_02_OWL2_AUTHORING.md
-   BLOCKER_03_WORKSPACE.md
-   BLOCKER_04_REASONING.md
-   BLOCKER_05_SWRL.md
-   BLOCKER_06_REFACTORING.md
-   BLOCKER_07_QUERY.md
-   BLOCKER_08_VISUALIZATION.md
-   BLOCKER_09_PLUGIN_PLATFORM.md
-   BLOCKER_10_ACCESSIBILITY.md
-   BLOCKER_11_PARITY_VERIFICATION.md
