# OWL Authoring Specification (v0.30)

> **Status: Historical / target UX for remaining v0.30 polish.** Do **not** implement from this page as if authoring were still an MVP.
>
> **Shipped today:** complete OWL 2 authoring (`PAR-OWL-001`, v0.22+), Turtle/OBO/RDF/XML/OWL/XML write-back, Manchester editor, Entity Inspector — see [What ships today](../SHIPPED.md), [authoring.md](../authoring.md), and [Supported formats](../supported-formats.md).
>
> This document remains useful for target UX gaps vs Protégé. Depends on [ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md), [ADR-0006](adr/0006-patch-based-write-back.md), and [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md).

## 1. Purpose

Specify **hybrid authoring** for Protégé-competitive v0.30:

- **Quick forms** for common axioms and annotations
- **Manchester OWL Syntax editor** for complex class expressions
- **Text buffer** with LSP assist for power users

## 2. Authoring surfaces

| Surface | Use for |
|---------|---------|
| **Quick forms** (Entity Inspector) | Labels, comments, `SubClassOf`, domain/range, deprecated, individual type assertions |
| **Axiom editor panel** | Pick axiom type → form fields → preview Manchester + Turtle patch |
| **Manchester editor** (webview) | `ObjectIntersectionOf`, `ObjectUnionOf`, `ObjectSomeValuesFrom`, cardinality (`min`/`max`/`exact`), nested restrictions |
| **Text buffer** | Direct Turtle/Manchester edit; LSP completion and diagnostics |

## 3. P0 axiom types (v0.30)

| Axiom | Quick form | Manchester editor |
|-------|------------|-------------------|
| `SubClassOf` | Yes (parent picker) | Yes (complex RHS) |
| `EquivalentClasses` | Yes (single equivalent) | Yes |
| `DisjointClasses` | Yes (multi-select) | Yes |
| Object property domain / range | Yes | Partial |
| Object property characteristics (functional, transitive, …) | Yes (checkboxes) | N/A |
| Data property domain / range | Yes | Partial |
| `ClassAssertion` | Yes | Partial |
| `ObjectPropertyAssertion` | Yes | Partial |
| `DataPropertyAssertion` | Yes | Partial |
| Annotation assertions | Yes | N/A |

## 4. Manchester requirements (P0)

- Parse and serialize Manchester OWL Syntax via [`horned-functional`](https://crates.io/crates/horned-functional) + [`horned-owl`](https://crates.io/crates/horned-owl) in `strixonomy-owl` ([ADR-0016](adr/0016-dependency-first-implementation.md))
- Optional: evaluate [`owl-ms-language-server`](https://crates.io/crates/owl-ms-language-server) for embedded Manchester assist in webview (v0.5)
- Autocomplete: classes, properties, individuals in scope, XSD datatypes, OBO ids ([OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md))
- Parse errors from Horned-OWL → LSP diagnostics with file range
- **Open in Manchester** action from Entity Inspector for any editable axiom
- Live Turtle patch preview before apply

## 5. Write-back rules

Patches are generated from **Horned-OWL axiom objects**, not string templates.

1. Apply minimal source-range patch per [ADR-0006](adr/0006-patch-based-write-back.md)
2. Preserve unrelated triples, comments, and `@prefix` blocks
3. Show diff preview in VS Code before multi-file changes
4. All edits undoable via VS Code

### Round-trip test suite (required for v0.30)

- Fixtures: Protégé-exported Turtle ontologies in `examples/protege-roundtrip/`
- Flow: load → index → edit axiom via API → save → semantic equivalence (formatting variance allowed)
- CI gate: `cargo test protege_roundtrip`

## 6. LSP / custom methods

See [LSP_SPEC.md](LSP_SPEC.md):

- `strixonomy/applyAxiomPatch`
- `strixonomy/parseManchester`

## 7. UI wireframes

See [UI_WIREFRAMES.md](UI_WIREFRAMES.md) §8–§9.

## 8. Milestone mapping

| Milestone | Deliverable |
|-----------|-------------|
| v0.4a | Simple patches: labels, comments, `SubClassOf` |
| v0.4b | `strixonomy-owl` + Horned-OWL catalog integration |
| v0.5 | Manchester MVP (subclass + equivalent) |
| v0.8 | Full Manchester catalog (restrictions, disjoint, property chains view) |
