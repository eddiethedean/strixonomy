# PROTEGE_FILE_FORMAT_AUDIT

# Protégé Desktop File Format Audit

**Status:** Living Reference Document\
**Purpose:** Define the ontology serialization formats that a standard
Protégé Desktop installation supports and establish the functional
parity expectations for Strixonomy.

> This audit focuses on user workflows (open, edit, save, round-trip),
> not parser implementation details.

------------------------------------------------------------------------

# Purpose

File format compatibility is a core requirement for replacing Protégé.
Users must be able to open existing ontologies, edit them semantically,
and save them without losing information.

This document identifies the required formats, expected workflows, and
audit criteria.

------------------------------------------------------------------------

# Audit Principles

-   Semantic fidelity over byte-for-byte reproduction.
-   Format-independent editing through a canonical ontology model.
-   Deterministic serialization where practical.
-   Round-trip verification for all required formats.
-   Preserve ontology semantics, metadata, and imports.

------------------------------------------------------------------------

# Required Formats (P0)

  -----------------------------------------------------------------------------------
  Format            Open          Edit          Save       Round-Trip   Notes
  ------------- ------------- ------------- ------------- ------------- -------------
  Turtle (.ttl)    REVIEW        REVIEW        REVIEW        REVIEW     Primary
                                                                        authoring
                                                                        format.

  RDF/XML          REVIEW        REVIEW        REVIEW        REVIEW     Common
  (.rdf/.owl)                                                           Protégé
                                                                        interchange
                                                                        format.

  OWL/XML          REVIEW        REVIEW        REVIEW        REVIEW     Frequently
  (.owl)                                                                used for OWL
                                                                        tooling.

  OBO (.obo)       REVIEW        REVIEW        REVIEW        REVIEW     Required for
                                                                        OBO ecosystem
                                                                        users.
  -----------------------------------------------------------------------------------

------------------------------------------------------------------------

# Secondary Formats (P1)

These formats are valuable but are not required to claim baseline
Protégé parity:

  Format              Expected Capability
  ------------------- --------------------------------------
  Functional Syntax   Parse / export where supported
  Manchester Syntax   Editing expressions and queries
  JSON-LD             Parse / serialize (optional for 1.0)
  N-Triples           Parse / serialize
  N-Quads             Parse / serialize
  TriG                Parse / serialize

------------------------------------------------------------------------

# Semantic Preservation Requirements

Every required format should preserve:

-   Ontology IRI
-   Version IRI
-   Prefixes (where applicable)
-   Imports
-   Ontology annotations
-   Entity annotations
-   OWL 2 axioms
-   Axiom annotations
-   Anonymous class expressions
-   Blank-node semantics

Formatting differences are acceptable if semantic equivalence is
maintained.

------------------------------------------------------------------------

# Round-Trip Validation

Each required format should support the following workflow:

1.  Open ontology.
2.  Parse into the canonical ontology model.
3.  Perform semantic edits.
4.  Serialize back to the original format.
5.  Reload the saved ontology.
6.  Verify semantic equivalence.

Automated regression tests should cover each stage.

------------------------------------------------------------------------

# Current Audit Focus

The repository audit identified the following high-priority engineering
work:

-   Complete RDF/XML write-back.
-   Complete OWL/XML write-back.
-   Unified format-independent edit pipeline.
-   Expanded semantic round-trip corpus.
-   Deterministic serialization policy.

These items are tracked separately in the parity backlog.

------------------------------------------------------------------------

# Acceptance Criteria

A required format is considered parity-complete only when:

-   Parsing is production ready.
-   Semantic editing is supported.
-   Saving is supported.
-   Round-trip tests pass.
-   Regression fixtures exist.
-   Known limitations are documented.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   CURRENT_FEATURE_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   FORMAT_SUPPORT.md
-   PARITY_MATRIX.md
-   PARITY_TEST_PLAN.md
-   PARITY_RELEASE_GATE.md
