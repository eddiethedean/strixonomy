# IMPLEMENTATION_EVIDENCE

# Implementation Evidence Registry

**Status:** Living Specification\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document is the authoritative mapping between Protégé parity
requirements and the actual Strixonomy implementation.

A feature is **not considered implemented** because it appears in
documentation or a roadmap. Every parity claim must be backed by
concrete evidence from the repository.

This registry should be updated whenever a feature is added, removed, or
substantially changed.

------------------------------------------------------------------------

# Evidence Requirements

Every parity feature should reference:

-   Source crate(s)
-   Primary source files
-   Public APIs
-   UI implementation (if applicable)
-   Tests
-   Documentation
-   Related parity requirement IDs
-   Current implementation status

------------------------------------------------------------------------

# Status Values

  -----------------------------------------------------------------------
  Status                              Meaning
  ----------------------------------- -----------------------------------
  IMPLEMENTED                         Functional implementation exists
                                      and is verified.

  PARTIAL                             Significant implementation exists
                                      but parity is incomplete.

  PLANNED                             Planned but not yet implemented.

  NOT_IMPLEMENTED                     No implementation currently exists.

  VERIFIED                            Implementation has passed parity
                                      acceptance criteria and test gates.
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# Evidence Matrix

  -------------------------------------------------------------------------------------------------------------------------
  Requirement    Capability   Status            Crates              Primary Files    UI      Tests   Docs    Notes
  -------------- ------------ ----------------- ------------------- ---------------- ------- ------- ------- --------------
  PAR-LIFE-001   Ontology     PARTIAL           strixonomy-cli,       TBD              ✓       TBD     ✓       Verify
                 lifecycle                      extension                                                    multi-format
                                                                                                             persistence.

  PAR-LIFE-002   Multi-       IMPLEMENTED       extension           ontologyRegistry  ✓       ✓       ✓       v0.20
                 ontology                       workspace/          sessionPersistence                 registry +
                                                                                                             session restore

  PAR-FMT-001    Turtle       IMPLEMENTED       strixonomy-edit,      transaction.rs,  ✓       ✓       ✓       v0.19:
                 write-back                     strixonomy-owl        adapter.rs,                         transaction
                                                                                        patch.rs               apply path.

  PAR-FMT-002    OBO          IMPLEMENTED       strixonomy-edit,      transaction.rs,  ✓       ✓       ✓       v0.19:
                 write-back                     strixonomy-obo        adapter.rs,                         transaction
                                                                                        patch.rs               apply path.

  PAR-FMT-003    RDF/XML      COMPLETE          strixonomy-owl        serialize.rs,    ✓       ✓       ✓       Horned
                 write-back                     strixonomy-edit       mutate.rs,                         RDF/XML
                                                                    apply_xml.rs,                      re-serialize
                                                                    adapter.rs                         (v0.21).

  PAR-FMT-004    OWL/XML      COMPLETE          strixonomy-owl        serialize.rs,    ✓       ✓       ✓       Horned
                 write-back                     strixonomy-edit       mutate.rs,                         OWL/XML
                                                                    apply_xml.rs,                      re-serialize
                                                                    adapter.rs                         (v0.21).

  PAR-OWL-001    OWL 2        PARTIAL           strixonomy-owl        patch.rs,        ✓       TBD     ✓       Target
                 authoring                                          manchester.rs                       v0.22.

  PAR-WS-001     Workspace    IMPLEMENTED       extension           workspaceRuntime ✓       ✓       ✓       v0.20
                 model                          workspace/          saveCoordinator                  workspace
                                                                                                             runtime

  PAR-RSN-001    Reasoning    IMPLEMENTED       strixonomy-reasoner   lib.rs           ✓       ✓       ✓       EL
                 classify                                                                                    classification.

  PAR-RSN-002    ABox         VERIFIED          strixonomy-reasoner   abox.rs          ✓       ✓       ✓       v0.23.0
                 reasoning                      lsp                                                 realization,
                                                                                                    instance check.

  PAR-RSN-003    DL           VERIFIED          strixonomy-reasoner   explain.rs       ✓       ✓       ✓       v0.23.0
                 explanations                                                                                DL-first.

  PAR-QRY-001    SPARQL       IMPLEMENTED       strixonomy-query      lib.rs           ✓       TBD     ✓       Query
                 query                                                                                       workbench.

  PAR-QRY-002    DL Query     VERIFIED          strixonomy-reasoner   dl_query.rs      ✓       ✓       ✓       v0.24.0
                 workflow                       lsp / extension                              Workbench DL;
                                                                                                             CLI/LSP.

  PAR-SWRL-001   SWRL         VERIFIED          strixonomy-swrl       lib.rs           ✓       ✓       ✓       v0.23.0
                                                                                                             Rule Browser.

  PAR-REF-001    Semantic     VERIFIED          strixonomy-refactor   lib.rs,          ✓       ✓       ✓       v0.24
                 refactoring                                        rename.rs,                               rename/merge/replace
                                                                    ontology.rs                              multi-format;
                                                                                                             move/extract Turtle-first.

  PAR-VIS-001    Graph        VERIFIED          strixonomy-catalog    graph.rs         ✓       ✓       ✓       v0.25
                 visualization                  extension           GraphPanel.tsx                           EPIC-008 kinds,
                                                                    GraphPanel.test                          filters, overlays,
                                                                                                             virt, list view.

  PAR-PLG-001    Plugin SDK   VERIFIED          strixonomy-plugin     host.rs,         ✓       ✓       ✓       v0.25
                                                                    lifecycle.rs,                            EPIC-009 SDK 1.0;
                                                                    manifest.rs                              providers +
                                                                    plugin_sdk_compat                        compat harness.

  PAR-ACC-001    Accessibility VERIFIED         webview-ui          a11y/*,          ✓       ✓       ✓       v0.25
                                                                    DialogShell,                             EPIC-010 focus/
                                                                    panels + axe                             SR + axe harness.

  PAR-TST-001    Parity       VERIFIED          parity/scripts      validate +       ✓       ✓       ✓       v0.25
                 verification                                       release-gate +                           EPIC-011 path
                                                                    generate-docs                            checks, metrics,
                                                                                                             Gate 3 report.
  -------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------

# v0.19 Deliverables (EPIC-001 / EPIC-011)

| Deliverable | Evidence |
|-------------|----------|
| `strixonomy-edit` crate with `Transaction`, `SemanticChange`, invert/compose/validate | `crates/strixonomy-edit/` |
| Turtle/OBO LSP + CLI apply routed through transactions | `crates/strixonomy-lsp/src/handlers.rs`, `crates/strixonomy-cli/src/main.rs` |
| ADR for semantic transaction model | `docs/design/adr/0020-semantic-transaction-edit-model.md` |
| Parity manifest + validator in CI | `parity/protege-desktop-parity.yaml`, `scripts/validate-parity-manifest.py` |
| GitHub epics EPIC-001…011 | [EPIC_INDEX.md](07_BACKLOG/EPIC_INDEX.md) |

------------------------------------------------------------------------

# Evidence Standards

Each VERIFIED feature should include:

1.  Source implementation reference.
2.  Relevant crate(s).
3.  Primary source files.
4.  Automated test references.
5.  Documentation references.
6.  Acceptance criteria satisfied.
7.  Related parity matrix entry.
8.  Date verified.

------------------------------------------------------------------------

# Repository Audit Integration

The initial entries in this registry should be derived from:

-   `ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md`
-   Static source inspection
-   Existing automated tests
-   Existing documentation

No capability should be marked IMPLEMENTED or VERIFIED solely because a
command exists or a roadmap mentions it.

------------------------------------------------------------------------

# Maintenance Workflow

Whenever a feature changes:

1.  Update implementation status.
2.  Add or update source references.
3.  Update associated tests.
4.  Update parity matrix status.
5.  Re-run parity validation.

------------------------------------------------------------------------

# Future Automation

This document is intended to become machine-readable.

Future enhancements may include:

-   YAML or JSON parity manifests
-   Automatic source validation
-   CI verification that evidence paths exist
-   Coverage reports linked to parity requirements

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   PARITY_SCOPE.md
-   PARITY_MATRIX.md
-   PARITY_GAP_ANALYSIS.md
-   PARITY_ACCEPTANCE_CRITERIA.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
