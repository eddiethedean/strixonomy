# SPEC.md — Strixonomy and Strixonomy Technical Specification

> **Related specs:** [PROTEGE_PARITY.md](PROTEGE_PARITY.md) (v1.0 exit bar),
> [DEPENDENCY_MATRIX.md](DEPENDENCY_MATRIX.md) (external crates),
> [OWL_AUTHORING_SPEC.md](OWL_AUTHORING_SPEC.md), [OBO_ROBOT_SPEC.md](OBO_ROBOT_SPEC.md),
> [REASONER_SPEC.md](REASONER_SPEC.md), [SHACL_SPEC.md](SHACL_SPEC.md)

## 1. System Overview

The system consists of four major layers:

1. **File layer** — ontology files in a local workspace.
2. **Strixonomy layer** — Rust indexing, parsing, cataloging, diagnostics, query, diff, docs.
3. **Language server layer** — editor protocol services.
4. **VS Code extension layer** — UI panels, commands, graph views, editing workflows.

## 2. Strixonomy Crate Layout

```text
strixonomy/
├── crates/
│   ├── strixonomy-core          # v0.2 — ignore
│   ├── strixonomy-parser        # v0.2 — oxigraph (+ fastobo v0.7b)
│   ├── strixonomy-owl           # v0.4b — horned-owl, horned-functional
│   ├── strixonomy-catalog       # v0.2
│   ├── strixonomy-query         # v0.2 — sqlparser, oxigraph SPARQL
│   ├── strixonomy-diagnostics   # v0.3 — oxigraph + catalog lints
│   ├── strixonomy-diff          # v0.9 — horned-owl, git2
│   ├── strixonomy-docs          # v0.9 — pulldown-cmark, minijinja
│   ├── strixonomy-reasoner      # v0.6 — OntoLogos 0.9→1.0
│   ├── strixonomy-robot         # v0.7b — ROBOT CLI (external)
│   ├── strixonomy-lsp           # v0.3 — lsp-server, lsp-types, diagnostics
│   └── strixonomy-cli           # v0.3
├── examples/
│   ├── protege-roundtrip/      # v1.0 — OWL round-trip fixtures
│   └── obo-workflow/           # v0.7b — OBO + ROBOT demo
├── benches/
├── tests/
└── docs/
```

## 3. Supported Ontology Formats

Required:

- Turtle `.ttl`
- RDF/XML `.rdf`
- OWL `.owl`
- JSON-LD `.jsonld`
- N-Triples `.nt`

Desired:

- N-Quads `.nq`
- TriG `.trig`
- OBO `.obo`

## 4. Core Data Model

### OntologyDocument

Fields:

- id
- path
- format
- base_iri
- imports
- namespaces
- parse_status
- content_hash
- modified_time

### Entity

Fields:

- iri
- short_name
- kind
- ontology_id
- source_location
- labels
- comments
- annotations
- deprecated
- usages

### Axiom

Fields:

- id
- ontology_id
- subject
- predicate
- object
- axiom_kind
- source_location
- annotations

### Diagnostic

Fields:

- code
- severity
- message
- file
- range
- entity_iri
- quick_fix

## 5. Virtual Tables

Required virtual tables:

- `ontologies`
- `namespaces`
- `imports`
- `entities`
- `classes`
- `object_properties`
- `data_properties`
- `annotation_properties`
- `individuals`
- `annotations`
- `axioms`
- `subclass_axioms`
- `equivalent_class_axioms`
- `disjoint_class_axioms`
- `domain_axioms`
- `range_axioms`
- `restrictions`
- `usages`
- `diagnostics`
- `broken_imports`
- `duplicate_labels`
- `orphan_classes`
- `deprecated_usages`

**v1.0:** Tables `restrictions`, `equivalent_class_axioms`, `disjoint_class_axioms`, `domain_axioms`, `range_axioms` are populated from **Horned-OWL** ([ADR-0013](adr/0013-dual-stack-oxigraph-horned-owl.md)), not triple pattern matching.

## 6. Query Interfaces

### CLI

```bash
strixonomy index .
strixonomy query . "SELECT * FROM classes"
strixonomy sparql . "SELECT ?s ?p ?o WHERE { ?s ?p ?o } LIMIT 10"
strixonomy validate .
strixonomy diff main..feature
strixonomy docs . --format markdown --out docs/ontology
strixonomy robot validate ./ontology
strixonomy robot merge --inputs a.owl b.owl --output merged.owl
```

### Rust API

```rust
let catalog = IndexBuilder::new()
    .workspace("./ontology")
    .build()?;

let rows = strixonomy_query::query_catalog(
    &catalog,
    "SELECT * FROM classes WHERE deprecated = true",
)?;
```

## 7. Strixonomy Extension Components

Per [ADR-0017](adr/0017-react-webview-ui.md), webview panels migrate to a React app; the extension host stays a thin orchestration layer.

```text
extension/
├── src/                      # extension host (TypeScript)
│   ├── extension.ts
│   ├── commands/
│   ├── treeviews/
│   ├── webviews/             # panelHost, messages, getWebviewHtml
│   ├── lsp/
│   └── utils/
├── webview-ui/               # React app (v0.7a+)
│   ├── vite.config.ts
│   └── src/
│       ├── panels/
│       └── components/
├── media/
├── package.json
└── README.md
```

## 8. VS Code Commands

Required commands:

- `Strixonomy: Index Workspace`
- `Strixonomy: Validate Workspace`
- `Strixonomy: Run Ontology Query`
- `Strixonomy: Run SPARQL Query`
- `Strixonomy: Open Ontology Explorer`
- `Strixonomy: Create Class`
- `Strixonomy: Create Property`
- `Strixonomy: Create Individual`
- `Strixonomy: Find Entity Usages`
- `Strixonomy: Rename Entity IRI`
- `Strixonomy: Generate Documentation`
- `Strixonomy: Show Semantic Diff`
- `Strixonomy: Run Reasoner`

## 9. LSP Features

Required:

- diagnostics
- hover
- completion
- go to definition
- find references
- document symbols
- workspace symbols
- rename
- code actions
- semantic tokens

## 10. Reasoner Adapter Interface

Reasoners are external integrations accessed through a stable adapter layer.

Required operations:

- classify ontology
- get inferred hierarchy
- find unsatisfiable classes
- explain inference
- validate consistency

## 11. Testing Strategy

- Rust unit tests
- Rust integration tests with ontology fixtures
- golden snapshot tests
- LSP protocol tests
- VS Code extension integration tests
- semantic diff regression tests
- parser fuzz tests
- performance benchmarks
- **Protégé round-trip tests** (`examples/protege-roundtrip/`)
- **Manchester parse corpus** tests
- **ROBOT interop smoke** tests (when `robot` on PATH)
- **Oxigraph ↔ Horned-OWL consistency** tests
