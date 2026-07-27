# PROTEGE_FEATURE_INVENTORY

# Protégé Desktop Feature Inventory

**Status:** Living Reference Document\
**Purpose:** Canonical inventory of the capabilities provided by a
standard Protégé Desktop installation that are relevant to Strixonomy's
1.0 parity goal.

------------------------------------------------------------------------

# Purpose

This inventory defines **what must be evaluated** during the parity
effort. It is intentionally feature-oriented rather than
implementation-oriented.

A feature appearing in this inventory **does not imply** Strixonomy is
missing it. Current implementation status is tracked separately in:

-   `CURRENT_FEATURE_MATRIX.md`
-   `IMPLEMENTATION_EVIDENCE.md`
-   `PARITY_MATRIX.md`

------------------------------------------------------------------------

# Inventory Categories

## 1. Project & Ontology Management

-   Create ontology
-   Open ontology
-   Save / Save As / Save All
-   Recent projects
-   Ontology IRI management
-   Version IRI management
-   Prefix management
-   Ontology annotations
-   Import management
-   Catalog support
-   Multiple open ontologies
-   Read-only imported ontologies

------------------------------------------------------------------------

## 2. Entity Authoring

-   Classes
-   Object properties
-   Data properties
-   Annotation properties
-   Individuals
-   Datatypes

For each entity type:

-   Create
-   Edit
-   Delete
-   Rename
-   Search
-   Navigate
-   Usage lookup

------------------------------------------------------------------------

## 3. OWL 2 Authoring

Support for creation and editing of:

-   SubClassOf
-   EquivalentClasses
-   DisjointClasses
-   DisjointUnion
-   ObjectPropertyDomain
-   ObjectPropertyRange
-   DataPropertyDomain
-   DataPropertyRange
-   Functional, Inverse Functional, Symmetric, Asymmetric, Reflexive,
    Irreflexive, Transitive properties
-   Inverse properties
-   Property chains
-   SameIndividual
-   DifferentIndividuals
-   Class assertions
-   Object property assertions
-   Data property assertions
-   Negative assertions
-   Keys
-   Datatype definitions
-   Axiom annotations

------------------------------------------------------------------------

## 4. Reasoning

-   Ontology classification
-   Consistency checking
-   Unsatisfiable classes
-   Realization
-   Instance checking
-   Asserted vs inferred hierarchy
-   Explanations
-   Multiple reasoner selection

------------------------------------------------------------------------

## 5. Querying

-   DL Query
-   SPARQL
-   Entity search
-   Usage search
-   Annotation search

------------------------------------------------------------------------

## 6. SWRL

-   Rule browser
-   Rule editor
-   Rule validation
-   Built-ins
-   Serialization
-   Rule search

------------------------------------------------------------------------

## 7. Refactoring

-   Rename entity
-   Merge entities
-   Replace references
-   Move entities
-   Module extraction
-   Ontology merge
-   Namespace migration
-   Usage analysis

------------------------------------------------------------------------

## 8. Visualization

-   Class hierarchy
-   Property hierarchy
-   Individual browser
-   Import graph
-   Ontology graph
-   Asserted view
-   Inferred view

------------------------------------------------------------------------

## 9. Formats

Required production workflows:

-   Turtle
-   RDF/XML
-   OWL/XML
-   OBO

Additional formats commonly encountered:

-   Functional Syntax
-   Manchester Syntax
-   JSON-LD
-   N-Triples
-   N-Quads
-   TriG

------------------------------------------------------------------------

## 10. Plugins

-   Plugin discovery
-   Plugin lifecycle
-   Views
-   Commands
-   Validators
-   Reasoner integrations
-   Import/export providers

------------------------------------------------------------------------

## 11. User Experience

-   Undo / Redo
-   Keyboard shortcuts
-   Context menus
-   Dockable views
-   Session restoration
-   Preferences
-   Accessibility

------------------------------------------------------------------------

# Inventory Usage

Each feature should eventually map to:

-   A requirement ID in `PARITY_MATRIX.md`
-   Implementation evidence
-   Automated tests
-   Acceptance criteria
-   GitHub issue(s), if applicable

------------------------------------------------------------------------

# Exclusions

This inventory intentionally excludes:

-   Swing-specific implementation details
-   Java APIs
-   Internal Protégé implementation classes
-   Third-party plugins that are not part of a standard installation

The objective is **functional equivalence**, not binary compatibility.

------------------------------------------------------------------------

# Related Documents

-   README.md
-   ONTOCODE_CURRENT_PROTEGE_PARITY_AUDIT.md
-   PARITY_SCOPE.md
-   PARITY_MATRIX.md
-   IMPLEMENTATION_EVIDENCE.md
-   PARITY_GAP_ANALYSIS.md
