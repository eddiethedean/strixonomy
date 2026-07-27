# FORMAT_SUPPORT

# Strixonomy Format Support Specification

**Status:** Normative Specification\
**Target Release:** Strixonomy 1.0.0

------------------------------------------------------------------------

# Purpose

This document defines the file formats that Strixonomy must support for
Protégé Desktop parity, the level of support required for each format,
and the acceptance criteria used to verify compatibility.

------------------------------------------------------------------------

# Design Principles

-   Semantic fidelity over textual fidelity
-   Serializer-independent editing
-   Deterministic round-trip behavior
-   Extensible serializer framework
-   Consistent user workflows across formats

------------------------------------------------------------------------

# Required 1.0 Formats

  Format                  Parse   Edit   Save   Round-trip  Priority
  ---------------------- ------- ------ ------ ------------ ----------
  Turtle (.ttl)            ✅      ✅     ✅        ✅      P0
  RDF/XML (.rdf, .owl)     ✅      ✅     ✅        ✅      P0
  OWL/XML                  ✅      ✅     ✅        ✅      P0
  OBO                      ✅      ✅     ✅        ✅      P0

------------------------------------------------------------------------

# Planned Post-1.0 Formats

-   Functional Syntax
-   Manchester Syntax export
-   JSON-LD
-   TriG
-   N-Triples
-   RDF/JSON

------------------------------------------------------------------------

# Capability Requirements

Every required format must support:

-   Open
-   Create
-   Edit
-   Save
-   Save As
-   Validation
-   Imports
-   Ontology metadata
-   Prefix management
-   Axiom annotations
-   Semantic round-trip

------------------------------------------------------------------------

# Serializer Architecture

``` text
Workspace
    │
    ▼
Canonical Ontology Model
    │
    ▼
Semantic Transactions
    │
 ┌──┴──────────────┐
 ▼                 ▼
Parser         Serializer
Adapter         Adapter
```

All serializers consume the canonical ontology model rather than
implementing independent editing logic.

------------------------------------------------------------------------

# Semantic Round-Trip

Round-trip validation should verify:

1.  Parse source ontology
2.  Apply semantic edits
3.  Serialize
4.  Reload
5.  Compare semantic model
6.  Run reasoning validation
7.  Execute regression fixtures

------------------------------------------------------------------------

# Compatibility Targets

-   Protégé-generated ontologies
-   OBO Foundry ontologies
-   Large production ontologies
-   Ontologies with imports
-   Annotated ontologies
-   Anonymous expression heavy ontologies

------------------------------------------------------------------------

# Acceptance Criteria

A required format is VERIFIED when:

-   Parsing succeeds.
-   Editing is serializer-independent.
-   Saving preserves semantics.
-   Round-trip tests pass.
-   Conformance corpus passes.
-   Performance targets are met.

------------------------------------------------------------------------

# Success Metrics

-   100% required formats VERIFIED
-   Zero semantic-loss defects
-   Shared serializer architecture
-   Passing format conformance suite

------------------------------------------------------------------------

# Related Documents

-   FORMAT_PARITY.md
-   PROTEGE_FILE_FORMAT_AUDIT.md
-   BLOCKER_01_FORMAT_INDEPENDENCE.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
