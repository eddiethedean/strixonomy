# Strixonomy SQL views

Strixonomy exposes ontology data as **SQL virtual tables** over the indexed catalog. Queries run locally — no database server required.

## Quick example

```bash
strixonomy query ./ontology "SELECT short_name, labels FROM classes"
```

```rust
use strixonomy::workspace::Workspace;

let ws = Workspace::open("./ontology")?;
let result = ws.query("SELECT short_name FROM classes WHERE deprecated = false")?;
```

## Virtual tables

| Table | Description |
|-------|-------------|
| `ontologies` | Indexed ontology documents |
| `classes` | OWL/RDFS classes |
| `object_properties` | OWL object properties |
| `data_properties` | OWL datatype properties |
| `annotation_properties` | OWL annotation properties |
| `individuals` | OWL named individuals |
| `entities` | All extracted entities |
| `annotations` | Label/comment and other annotation triples |
| `axioms` | Extracted axioms (e.g. SubClassOf) |
| `restrictions` | OWL class restrictions (Horned-OWL catalog) |
| `equivalent_class_axioms` | Equivalent class expressions |
| `disjoint_class_axioms` | Disjoint class pairs |
| `domain_axioms` | Property domain axioms |
| `range_axioms` | Property range axioms |
| `namespaces` | Namespace prefixes |
| `imports` | Ontology imports |
| `diagnostics` | Lint and parse diagnostics |
| `properties` | Union of all property kinds |

Full column schemas, types, and examples: **[SQL reference](../sql-reference.md)**.

SPARQL over the triple store: **[SPARQL reference](../sparql-reference.md)**.
