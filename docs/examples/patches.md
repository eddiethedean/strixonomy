# Sample patch JSON

Copy-paste example for `strixonomy patch`. Full reference: [patch-reference.md](../patch-reference.md).

```json
[
  {
    "op": "create_entity",
    "entity_iri": "http://example.org/people#Student",
    "kind": "class"
  },
  {
    "op": "add_label",
    "entity_iri": "http://example.org/people#Student",
    "value": "Student"
  },
  {
    "op": "add_sub_class_of",
    "entity_iri": "http://example.org/people#Student",
    "parent_iri": "http://example.org/people#Person"
  }
]
```

```bash
# From a folder containing example.ttl (e.g. strixonomy-tutorial from first-success)
strixonomy patch ./example.ttl patches.json --preview
strixonomy patch ./example.ttl patches.json
```

Download: [patches.json](patches.json)

## OBO patches (v0.12)

Write synonym `scope` as OBO uppercase (`EXACT`, `RELATED`, …). See [patch reference](../patch-reference.md).

```json
[
  {
    "op": "set_name",
    "term_id": "EX:001",
    "value": "example class"
  },
  {
    "op": "add_synonym",
    "term_id": "EX:001",
    "value": "example",
    "scope": "EXACT"
  },
  {
    "op": "add_is_a",
    "term_id": "EX:001",
    "parent_id": "EX:000"
  }
]
```

```bash
strixonomy patch ./terms.obo patches-obo.json --preview
strixonomy patch ./terms.obo patches-obo.json
```

Download: [patches-obo.json](patches-obo.json)

## Complex axioms (v0.5)

```json
[
  {
    "op": "add_complex_sub_class_of",
    "entity_iri": "http://example.org/clinic#Patient",
    "manchester": "ex:hasRecord some ex:MedicalRecord"
  },
  {
    "op": "set_equivalent_class",
    "entity_iri": "http://example.org/clinic#Staff",
    "manchester": "ex:Employee"
  }
]
```

## RDF/XML and OWL/XML (v0.21)

The same Turtle-shaped patch JSON applies to `.owl` / `.rdf` / `.owx` via Horned re-serialize. Prefer core ops (labels, SubClassOf, imports, create/delete). See [OWL/XML write-back](../guides/owl-xml-workflow.md) and [patch reference](../patch-reference.md).

```bash
strixonomy patch ./ontology.owl patches.json --preview
strixonomy patch ./ontology.owl patches.json
```