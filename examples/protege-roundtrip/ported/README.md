# Protégé-ported fixtures (v0.26)

Behavioral **equivalents** of Protégé Desktop `protege-editor-owl` test
resources under
[protegeproject/protege](https://github.com/protegeproject/protege)
(`src/test/resources/ontologies/...`). These are **synthetic** Strixonomy
fixtures shaped to the same scenarios (loops, multi-parent trees,
versioned ontology IRIs, idranges, catalogs)—not byte copies of Protégé
files (except attributed Manchester idpolicy `*.owl` under `idpolicy/`).

License for this directory: MIT OR Apache-2.0 (Strixonomy). Behavioral
inspiration from Protégé test designs; Protégé itself is BSD-2-Clause.
Vendored `idpolicy/*.owl` retains Protégé BSD attribution via upstream copy.

| File | Scenario |
|------|----------|
| `simple_loop.ttl` | Cyclic `rdfs:subClassOf` (AssertedClassHierarchy loop case) |
| `two_parents.ttl` | Class with two asserted parents |
| `two_eq.ttl` | Equivalent classes + subclass (twoEq tree case) |
| `versioned_ontology.ttl` | Ontology IRI + version IRI |
| `versioned_ontology.owl` | Same identity in RDF/XML |
| `ambiguous_name.owl` | Ontology IRI ≠ xml:base / file name |
| `idranges_minimal.ttl` | Minimal GO-style idranges annotations |
| `idpolicy/` | Vendored GO idranges + validation-error fixtures |
| `tabbed_hierarchy.txt` | Indented hierarchy input for parser → SubClassOf |
| `merge_labels.ttl` | Merge with rdfs:label on source |
| `obofoundry_minimal.json` | Truncated OBO Foundry registry JSON (Wave 3) |
| `obofoundry_expanded.json` | Larger Foundry-shaped registry (stress) |
| `imports_home/` | Multi-file imports for axiom `ontology_id` location |
| `catalog_home/` | `catalog-v001.xml` IRI redirect + import pair |

Used by `cargo test -p strixonomy-workspace --test protege_port_*`.
Copy into a tempfile workspace before mutating; never edit committed files in place.
