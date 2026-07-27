# OWL 2 authoring gaps (v0.22)

**Parity ID:** `PAR-OWL-001` · **EPIC:** EPIC-002 (#248) · **Blocker:** [BLOCKER_02](../04_BLOCKERS/BLOCKER_02_OWL2_AUTHORING.md)

This matrix is the v0.22 construct inventory. Status values: **Shipped** (pre-0.22), **v0.22** (target), **N/A** (wrong surface).

Formats: **TTL** = Turtle · **XML** = RDF/XML + OWL/XML · **OBO** = not OWL2 surface.

## Ontology metadata

| Construct | PatchOp | TTL | XML | UI | Test ID |
|-----------|---------|-----|-----|-----|---------|
| Ontology IRI | `set_ontology_iri` | Shipped | Shipped | Inspector | owl2_meta_ontology_iri |
| Version IRI | `set_version_iri` | Shipped | Shipped | Inspector | owl2_meta_version_iri |
| Imports | `add_import` / `remove_import` | Shipped | Shipped | Imports panel | owl2_meta_import |
| Prefixes | `add_prefix` / `remove_prefix` / `set_prefix` | Shipped | Error (Turtle-only) | Prefix manager | owl2_meta_prefix |
| Ontology annotations | `add_ontology_annotation` / `remove_ontology_annotation` | Shipped | Shipped | Inspector | owl2_meta_anno |

## Entities

| Construct | PatchOp / kind | TTL | XML | UI | Test ID |
|-----------|----------------|-----|-----|-----|---------|
| Class | `create_entity` `class` | Shipped | Shipped | Explorer | owl2_ent_class |
| Object property | `create_entity` `object_property` | Shipped | Shipped | Explorer | owl2_ent_op |
| Data property | `create_entity` `data_property` | Shipped | Shipped | Explorer | owl2_ent_dp |
| Annotation property | `create_entity` `annotation_property` | Shipped | Shipped | Explorer | owl2_ent_ap |
| Individual | `create_entity` `individual` | Shipped | Shipped | Explorer | owl2_ent_ind |
| Datatype | `create_entity` `datatype` | v0.22 | v0.22 | Explorer | owl2_ent_dt |
| Delete entity | `delete_entity` | Shipped | Shipped | Inspector | owl2_ent_delete |

## TBox / RBox

| Construct | PatchOp | TTL | XML | UI | Test ID |
|-----------|---------|-----|-----|-----|---------|
| SubClassOf (named) | `add_sub_class_of` / `remove_sub_class_of` | Shipped | Shipped | Inspector | owl2_tbox_subclass |
| SubClassOf (complex) | `add_complex_sub_class_of` / `remove_…` | Shipped | Shipped | Manchester | owl2_tbox_subclass_ce |
| EquivalentClasses | `add_equivalent_class` / `remove_…` / `set_…` | Shipped | Shipped | Manchester | owl2_tbox_equiv |
| DisjointClasses | `add_disjoint_class` / `remove_…` | Shipped | Shipped | Manchester / Inspector | owl2_tbox_disjoint |
| DisjointUnion | `add_disjoint_union` / `remove_disjoint_union` | v0.22 | Shipped | Inspector | owl2_tbox_disjoint_union |
| Domain / range | `add_domain` / `add_range` / removes | Shipped | Shipped | Inspector | owl2_rbox_domain_range |
| Characteristics | `set_functional` … `set_irreflexive` | Shipped | Shipped | Inspector | owl2_rbox_chars |
| Property chain | `add_property_chain` / `remove_…` | Shipped | Shipped | Chain editor | owl2_rbox_chain |
| InverseObjectProperties | `add_inverse_object_properties` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_inverse |
| EquivalentObjectProperties | `add_equivalent_object_properties` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_equiv_op |
| DisjointObjectProperties | `add_disjoint_object_properties` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_disjoint_op |
| EquivalentDataProperties | `add_equivalent_data_properties` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_equiv_dp |
| DisjointDataProperties | `add_disjoint_data_properties` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_disjoint_dp |
| SubObjectPropertyOf | `add_sub_object_property_of` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_sub_op |
| SubDataPropertyOf | `add_sub_data_property_of` / `remove_…` | v0.22 | Shipped | Inspector | owl2_rbox_sub_dp |
| HasKey | `add_has_key` / `remove_has_key` | v0.22 | Shipped | Inspector | owl2_tbox_haskey |

## ABox

| Construct | PatchOp | TTL | XML | UI | Test ID |
|-----------|---------|-----|-----|-----|---------|
| ClassAssertion | `add_class_assertion` / `remove_…` | Shipped | Shipped | Inspector | owl2_abox_type |
| ObjectPropertyAssertion | `add_object_property_assertion` / `remove_…` | Shipped | Shipped | Inspector | owl2_abox_opa |
| DataPropertyAssertion | `add_data_property_assertion` / `remove_…` | Shipped | Shipped | Inspector | owl2_abox_dpa |
| NegativeObjectPropertyAssertion | `add_negative_object_property_assertion` / `remove_…` | v0.22 | Shipped | Inspector | owl2_abox_nopa |
| NegativeDataPropertyAssertion | `add_negative_data_property_assertion` / `remove_…` | v0.22 | Shipped | Inspector | owl2_abox_ndpa |
| SameIndividual | `add_same_individual` / `remove_same_individual` | v0.22 | Shipped | Inspector | owl2_abox_same |
| DifferentIndividuals | `add_different_individuals` / `remove_different_individuals` | v0.22 | Shipped | Inspector | owl2_abox_different |

## Expressions (Manchester)

| Construct | Support | Test ID |
|-----------|---------|---------|
| Intersection / Union | Shipped (`and` / `or`) | owl2_ce_and_or |
| Complement (`not`) | v0.22 | owl2_ce_not |
| ObjectSome/AllValuesFrom | Shipped (`some` / `only`) | owl2_ce_some_only |
| ObjectMin/Max/ExactCardinality | Shipped | owl2_ce_card |
| ObjectHasValue (`value`) | v0.22 | owl2_ce_value |
| ObjectHasSelf (`Self`) | v0.22 | owl2_ce_self |
| ObjectOneOf (`{…}`) | v0.22 | owl2_ce_oneof |
| Data restrictions / facets | v0.22 | owl2_ce_data |
| DatatypeDefinition | `add_datatype_definition` / `remove_…` | owl2_dt_def |

## Annotations

| Construct | PatchOp | TTL | XML | UI | Test ID |
|-----------|---------|-----|-----|-----|---------|
| Entity annotations | `add_annotation` / `remove_annotation` (+ label/comment) | Shipped | Shipped | Inspector | owl2_anno_entity |
| Axiom annotations | `add_axiom_annotation` / `remove_axiom_annotation` | Shipped | Shipped | Inspector | owl2_anno_axiom |

**Axiom annotation identity:** `axiom_op` + `subject_iri` + optional `related_iri` + `predicate`/`value` match the annotated axiom (Turtle: `owl:Axiom` reification; XML: Horned `AnnotatedComponent.ann`). Matcher targets named entities; complex CE superclass/filler identity remains limited. Ambiguous multi-match requires `related_iri`.

**Catalog listing:** Horned bridge projects all v0.22 axiom families onto `EntityDetail.axioms` (including nested axiom annotations). Datatype entities use `EntityKind::Datatype`.

## Out of v0.22

| Item | Target |
|------|--------|
| SWRL | v0.23 |
| Reasoning parity / DL explanations depth | v0.23 |
| Full refactor for every new axiom | v0.24 |
| DL Query | v0.24 |
| OBO encoding of OWL2 | N/A |
| Byte-identical XML | Non-goal (ADR-0021) |

## Wire sync checklist

- [x] `crates/strixonomy-owl/src/bridge.rs` catalog projection (+ axiom annotations)
- [x] `crates/strixonomy-owl/src/patch.rs` `PatchOp`
- [x] `crates/strixonomy-edit/src/invert.rs`
- [x] `crates/strixonomy-owl/src/mutate.rs` / `apply_xml.rs`
- [x] `crates/strixonomy-owl/src/manchester.rs`
- [x] `docs/patch-reference.md`
- [x] `extension/src/lsp/protocol.ts` + webview messages
- [x] Inspector / Manchester UI
- [x] `parity/protege-desktop-parity.yaml` `PAR-OWL-001` `test_ids`
